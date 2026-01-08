1) 📄 Contrato Rust completo de /api/v1/webrtc/offer (webrtc-rs 0.11)
1.1 Request/Response (estável e “compatível com migração”)

vms_core/src/webrtc_contract.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebRtcOfferRequest {
    pub camera_id: Uuid,
    pub sdp: String,
    #[serde(default = "default_offer_type")]
    pub sdp_type: String, // "offer"
    pub session_token: String,
}

fn default_offer_type() -> String { "offer".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebRtcAnswerResponse {
    pub sdp: String,
    #[serde(default = "default_answer_type")]
    pub sdp_type: String, // "answer"
    pub peer_id: Uuid,
    pub expires_at: i64,
    /// Porta UDP local onde o FFmpeg vai enviar RTP (útil pra debug e migração)
    pub rtp_port: u16,
}

fn default_answer_type() -> String { "answer".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorBody {
    pub code: String,
    pub message: String,
}

1.2 Estado de runtime necessário (peer + track + tarefa de streaming)

Requisito profissional: manter tabela em memória de peers e tasks, para stop/cleanup.

vms_server/src/webrtc/state.rs

use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

use webrtc::{
    peer_connection::RTCPeerConnection,
    track::track_local::track_local_static_rtp::TrackLocalStaticRTP,
};

pub struct PeerRuntime {
    pub peer: Arc<RTCPeerConnection>,
    pub track: Arc<TrackLocalStaticRTP>,
    pub rtp_port: u16,
    pub task_handle: tokio::task::JoinHandle<()>,
}

#[derive(Clone, Default)]
pub struct WebRtcRuntime {
    peers: Arc<Mutex<HashMap<Uuid, PeerRuntime>>>,
}

impl WebRtcRuntime {
    pub async fn insert(&self, peer_id: Uuid, rt: PeerRuntime) {
        self.peers.lock().await.insert(peer_id, rt);
    }

    pub async fn remove(&self, peer_id: Uuid) -> Option<PeerRuntime> {
        self.peers.lock().await.remove(&peer_id)
    }

    pub async fn count(&self) -> usize {
        self.peers.lock().await.len()
    }
}

1.3 Handler Axum “real” (cria track, cria peer, set_remote, create_answer, spawn FFmpeg e loop UDP)

Assume:

você já tem create_webrtc_api() montando MediaEngine com H264

você tem stream_rtsp_to_webrtc() com UDP recv → Packet::unmarshal → track.write_rtp()

Vou te dar uma versão pronta e robusta:

porta UDP alocada via bind 127.0.0.1:0

spawn de FFmpeg apontando pro rtp://127.0.0.1:<porta>

task separada para o loop RTP

cleanup se falhar SDP

grava peer_id/rtp_port em live_sessions

vms_server/src/api/webrtc.rs

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use uuid::Uuid;

use webrtc::{
    api::API,
    peer_connection::configuration::RTCConfiguration,
    peer_connection::sdp::session_description::RTCSessionDescription,
    rtp::packet::Packet,
    track::track_local::track_local_static_rtp::TrackLocalStaticRTP,
    rtp_transceiver::rtp_codec::RTCRtpCodecCapability,
};

use vms_core::webrtc_contract::{ApiErrorBody, WebRtcAnswerResponse, WebRtcOfferRequest};
use crate::{
    app_state::AppState,
    auth::session_token::{verify_session_token, SessionTokenClaims},
    db::live_sessions,
    webrtc::{engine::create_webrtc_api, stream::spawn_rtsp_rtp_task},
    webrtc::state::{PeerRuntime},
};

pub async fn webrtc_offer(
    State(state): State<AppState>,
    Json(req): Json<WebRtcOfferRequest>,
) -> impl IntoResponse {
    // 1) validar token curto
    let claims: SessionTokenClaims = match verify_session_token(&state.session_signer, &req.session_token) {
        Ok(c) => c,
        Err(e) => return (
            StatusCode::UNAUTHORIZED,
            Json(ApiErrorBody{ code:"SESSION_TOKEN_INVALID".into(), message:e })
        ).into_response(),
    };

    // 2) camera_id tem que bater
    if claims.camera_id != req.camera_id {
        return (
            StatusCode::FORBIDDEN,
            Json(ApiErrorBody{ code:"CAMERA_MISMATCH".into(), message:"token não pertence à câmera".into() })
        ).into_response();
    }

    // 3) carregar sessão e validar status/ttl
    let sess = match live_sessions::get_by_id(&state.db, claims.live_session_id).await {
        Ok(Some(s)) => s,
        Ok(None) => return (
            StatusCode::NOT_FOUND,
            Json(ApiErrorBody{ code:"SESSION_NOT_FOUND".into(), message:"sessão não existe".into() })
        ).into_response(),
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiErrorBody{ code:"DB_ERROR".into(), message: format!("{}", e) })
        ).into_response(),
    };

    if sess.expires_at <= chrono::Utc::now().timestamp() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ApiErrorBody{ code:"SESSION_EXPIRED".into(), message:"sessão expirada".into() })
        ).into_response();
    }

    // 4) criar API webrtc + peer
    let api: Arc<API> = state.webrtc_api.clone(); // criado no boot via create_webrtc_api()
    let peer_id = Uuid::new_v4();

    let peer = match api.new_peer_connection(RTCConfiguration::default()).await {
        Ok(p) => Arc::new(p),
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiErrorBody{ code:"PEER_CREATE_FAILED".into(), message: format!("{}", e) })
        ).into_response(),
    };

    // 5) criar track H264 e add_track
    let track = Arc::new(TrackLocalStaticRTP::new(
        RTCRtpCodecCapability { mime_type: webrtc::api::media_engine::MIME_TYPE_H264.to_string(), ..Default::default() },
        "video".to_string(),
        format!("vms-camera-{}", req.camera_id),
    ));

    if let Err(e) = peer.add_track(track.clone()).await {
        let _ = peer.close().await;
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiErrorBody{ code:"ADD_TRACK_FAILED".into(), message: format!("{}", e) })
        ).into_response();
    }

    // 6) aplicar offer -> create answer
    let remote = match RTCSessionDescription::offer(req.sdp.clone()) {
        Ok(s) => s,
        Err(e) => {
            let _ = peer.close().await;
            return (StatusCode::BAD_REQUEST, Json(ApiErrorBody{ code:"SDP_OFFER_INVALID".into(), message: format!("{}", e) })).into_response();
        }
    };

    if let Err(e) = peer.set_remote_description(remote).await {
        let _ = peer.close().await;
        return (StatusCode::BAD_REQUEST, Json(ApiErrorBody{ code:"SET_REMOTE_FAILED".into(), message: format!("{}", e) })).into_response();
    }

    let answer = match peer.create_answer(None).await {
        Ok(a) => a,
        Err(e) => {
            let _ = peer.close().await;
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiErrorBody{ code:"CREATE_ANSWER_FAILED".into(), message: format!("{}", e) })).into_response();
        }
    };

    if let Err(e) = peer.set_local_description(answer.clone()).await {
        let _ = peer.close().await;
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiErrorBody{ code:"SET_LOCAL_FAILED".into(), message: format!("{}", e) })).into_response();
    }

    // 7) iniciar streaming RTSP -> FFmpeg -> RTP/UDP -> track.write_rtp()
    //    spawn_rtsp_rtp_task retorna (task_handle, rtp_port)
    let (task_handle, rtp_port) = match spawn_rtsp_rtp_task(
        req.camera_id,
        sess.rtsp_url.clone(),          // recomendo: rtsp_url resolvida no server (não no viewer)
        track.clone(),
        state.ffmpeg_path.clone(),      // opcional
    ).await {
        Ok(v) => v,
        Err(e) => {
            let _ = peer.close().await;
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiErrorBody{ code:"STREAM_START_FAILED".into(), message: e })).into_response();
        }
    };

    // 8) persistir peer_id e rtp_port na live_session, ativar
    if let Err(e) = live_sessions::activate_with_peer(&state.db, sess.id, peer_id, rtp_port as i32).await {
        let _ = peer.close().await;
        task_handle.abort();
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiErrorBody{ code:"SESSION_UPDATE_FAILED".into(), message: format!("{}", e) })).into_response();
    }

    // 9) registrar runtime (pra stop/cleanup)
    state.webrtc_runtime.insert(peer_id, PeerRuntime {
        peer: peer.clone(),
        track: track.clone(),
        rtp_port,
        task_handle,
    }).await;

    // 10) responder SDP answer
    let local = peer.local_description().await;
    let sdp_answer = local.map(|d| d.sdp).unwrap_or(answer.sdp);

    (
        StatusCode::OK,
        Json(WebRtcAnswerResponse {
            sdp: sdp_answer,
            sdp_type: "answer".into(),
            peer_id,
            expires_at: sess.expires_at,
            rtp_port,
        })
    ).into_response()
}

1.4 Implementação do streaming “RTP task” (o teu loop, só mais robusto)

vms_server/src/webrtc/stream.rs

use std::{process::Stdio, sync::Arc};
use tokio::net::UdpSocket;
use tokio::process::Command;
use uuid::Uuid;

use webrtc::rtp::packet::Packet;
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;

pub async fn spawn_rtsp_rtp_task(
    camera_id: Uuid,
    rtsp_url: String,
    track: Arc<TrackLocalStaticRTP>,
    ffmpeg_path: String,
) -> Result<(tokio::task::JoinHandle<()>, u16), String> {
    // Bind porta dinâmica local
    let socket = UdpSocket::bind("127.0.0.1:0").await.map_err(|e| e.to_string())?;
    let rtp_port = socket.local_addr().map_err(|e| e.to_string())?.port();

    // Spawn ffmpeg enviando RTP p/ a porta local
    // OBS: "-payload_type 96" e SDP/rtpmap podem ajudar; mas mantendo teu modelo atual.
    let mut child = Command::new(if ffmpeg_path.is_empty() { "ffmpeg" } else { ffmpeg_path.as_str() })
        .args([
            "-rtsp_transport", "tcp",
            "-i", &rtsp_url,
            "-an",
            "-c:v", "libx264",
            "-preset", "ultrafast",
            "-tune", "zerolatency",
            "-profile:v", "baseline",
            "-b:v", "8M",
            "-g", "10",
            "-bf", "0",
            "-f", "rtp",
            &format!("rtp://127.0.0.1:{rtp_port}"),
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped()) // útil para logs
        .spawn()
        .map_err(|e| format!("ffmpeg spawn failed: {e}"))?;

    // Task: recv UDP -> unmarshal -> write_rtp
    let handle = tokio::spawn(async move {
        let mut buf = vec![0u8; 2048];

        loop {
            tokio::select! {
                res = socket.recv(&mut buf) => {
                    let n = match res {
                        Ok(n) => n,
                        Err(_) => break,
                    };
                    let pkt = match Packet::unmarshal(&mut &buf[..n]) {
                        Ok(p) => p,
                        Err(_) => continue,
                    };
                    if track.write_rtp(&pkt).await.is_err() {
                        break;
                    }
                }
                _ = child.wait() => {
                    // FFmpeg encerrou
                    break;
                }
            }
        }

        // best-effort: se ainda estiver rodando, mata
        let _ = child.kill().await;
        // aqui você pode registrar audit/log: camera_id stop, etc.
        let _ = camera_id; // evita warning se não usar
    });

    Ok((handle, rtp_port))
}

2) 🧩 LiveSession “real” no DB (com peer_id, rtp_port, ffmpeg_pid)

Você precisa desses campos para operar como VMS profissional:

status + ttl

peer_id (WebRTC)

rtp_port (debug e migração)

ffmpeg_pid (kill seguro se precisar)

node_id (hoje pode ser null; amanhã é chave da migração)

2.1 Postgres
CREATE TABLE live_sessions (
  id UUID PRIMARY KEY,
  user_id UUID NOT NULL,
  camera_id UUID NOT NULL,
  node_id UUID NULL,

  status TEXT NOT NULL,                  -- starting|active|stopping|stopped|failed
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  started_at TIMESTAMPTZ NULL,
  ended_at TIMESTAMPTZ NULL,
  expires_at TIMESTAMPTZ NOT NULL,

  peer_id UUID NULL,
  rtp_port INTEGER NULL,
  ffmpeg_pid INTEGER NULL,

  profile TEXT NULL,
  client_ip TEXT NULL,
  user_agent TEXT NULL,

  error_code TEXT NULL,
  error_message TEXT NULL
);

CREATE INDEX idx_live_sessions_camera ON live_sessions(camera_id);
CREATE INDEX idx_live_sessions_user ON live_sessions(user_id);
CREATE INDEX idx_live_sessions_status ON live_sessions(status);
CREATE INDEX idx_live_sessions_expires ON live_sessions(expires_at);

2.2 Função DB: “activate_with_peer”
pub async fn activate_with_peer(
    db: &sqlx::PgPool,
    session_id: uuid::Uuid,
    peer_id: uuid::Uuid,
    rtp_port: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(r#"
        UPDATE live_sessions
        SET status='active',
            started_at = COALESCE(started_at, NOW()),
            peer_id = $2,
            rtp_port = $3
        WHERE id = $1
    "#)
    .bind(session_id)
    .bind(peer_id)
    .bind(rtp_port)
    .execute(db)
    .await?;
    Ok(())
}

3) 🔐 Token de sessão (curto, assinado) — modelo final
3.1 Claims “mínimos e corretos”

live_session_id

camera_id

user_id

scope = "webrtc_offer"

exp (30–60s)

3.2 Por que TTL tão curto?

Porque o /webrtc/offer é uma porta crítica: se alguém roubar a URL, em 60s já morre.

Exemplo emissão (no POST /cameras/{id}/live)
let exp = chrono::Utc::now().timestamp() + 60;
let claims = SessionTokenClaims {
  live_session_id,
  camera_id,
  user_id,
  scope: "webrtc_offer".into(),
  exp,
};
let token = state.session_signer.sign(&claims)?;

Validação (no /webrtc/offer)

assinatura ok

exp ok

scope ok

camera_id bate

live_session existe no DB e está válida

Isso é “padrão VMS”.

4) 🎯 Roadmap exato: mover WebRTC do Server → Node (sem quebrar)

Você vai fazer em 2 passos “sem risco”:

Fase 1 — Node compatível + Server proxy (zero mudança no Viewer)

Implementar no Node (8090) o mesmo endpoint:

POST /api/v1/webrtc/offer

No Server (9095), o handler atual vira:

valida token + ACL + sessão

descobre node_id da câmera

proxy HTTP para http://node:8090/api/v1/webrtc/offer

retorna o answer pro viewer

✅ Viewer não muda
✅ Você testa node sem “quebrar produção”

Fase 2 — Viewer chama Node direto (reduz CPU do Server)

No POST /api/v1/cameras/{id}/live, o server devolve:

offer_url: http://node:8090/api/v1/webrtc/offer

session_token

expires_at

Viewer passa a postar offer direto no node.

✅ Server sai do caminho do SDP
✅ Menos gargalo
✅ Latência melhora um pouco

Fase 3 — Hardening (profissional)

heartbeat node → server

fallback automático:

se node falhar, server pode negociar local (modo antigo) por um tempo

métricas:

peers ativos por node

falhas de SDP

FFmpeg crash rate