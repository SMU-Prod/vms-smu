//! WebRTC signaling routes for low-latency video streaming
//!
//! Handles SDP offer/answer exchange and ICE candidates for WebRTC connections.
//! Target latency: 50-150ms

use axum::{
    extract::{State, Path},
    routing::post,
    Router, Json,
    http::StatusCode,
};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::sync::Arc;
use uuid::Uuid;

use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::{MediaEngine, MIME_TYPE_H264};
use webrtc::api::APIBuilder;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::rtp_transceiver::rtp_codec::{RTCRtpCodecCapability, RTCRtpCodecParameters, RTPCodecType};
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;
use webrtc::track::track_local::TrackLocal;

use vms_core::{ApiErrorBody, IceCandidateRequest, WebRtcAnswerResponse};
use crate::state::AppState;
use crate::webrtc::{PeerRuntime, spawn_rtsp_rtp_task};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/offer/:camera_id", post(handle_offer))
        .route("/ice/:camera_id", post(handle_ice))
        .route("/stop/:peer_id", post(handle_stop))
}

/// Create WebRTC API with H264 codec support
fn create_webrtc_api() -> Result<webrtc::api::API, String> {
    let mut media_engine = MediaEngine::default();
    
    // Register H264 codec for low-latency video
    media_engine.register_codec(
        RTCRtpCodecParameters {
            capability: RTCRtpCodecCapability {
                mime_type: MIME_TYPE_H264.to_owned(),
                clock_rate: 90000,
                channels: 0,
                sdp_fmtp_line: "level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42e01f".to_owned(),
                rtcp_feedback: vec![],
            },
            payload_type: 96,
            ..Default::default()
        },
        RTPCodecType::Video,
    ).map_err(|e| e.to_string())?;

    let mut registry = Registry::new();
    registry = register_default_interceptors(registry, &mut media_engine)
        .map_err(|e| e.to_string())?;

    let api = APIBuilder::new()
        .with_media_engine(media_engine)
        .with_interceptor_registry(registry)
        .build();

    Ok(api)
}

/// SDP Offer request (simplified - camera_id comes from path)
#[derive(Debug, serde::Deserialize)]
pub struct OfferRequest {
    pub sdp: String,
    #[serde(rename = "type", default = "default_offer")]
    pub sdp_type: String,
}

fn default_offer() -> String { "offer".to_string() }

/// Handle SDP offer from browser and return answer
async fn handle_offer(
    State(state): State<AppState>,
    Path(camera_id): Path<Uuid>,
    Json(offer): Json<OfferRequest>,
) -> Result<Json<WebRtcAnswerResponse>, (StatusCode, Json<ApiErrorBody>)> {
    tracing::info!("📡 WebRTC offer received for camera {}", camera_id);

    // 1) Get camera to verify it exists and get RTSP URL
    let camera = state.camera_repo
        .find_by_id(camera_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiErrorBody::new("DB_ERROR", e.to_string()))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(ApiErrorBody::new("CAMERA_NOT_FOUND", "Camera not found"))))?;

    // 2) Create WebRTC API
    let api = create_webrtc_api()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiErrorBody::new("WEBRTC_API_ERROR", e))))?;

    // 3) ICE configuration with STUN server
    let config = RTCConfiguration {
        ice_servers: vec![RTCIceServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            ..Default::default()
        }],
        ..Default::default()
    };

    // 4) Create peer connection
    let peer_id = Uuid::new_v4();
    let peer = Arc::new(
        api.new_peer_connection(config)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiErrorBody::new("PEER_CREATE_FAILED", e.to_string()))))?
    );

    // 5) Create H264 video track
    let video_track = Arc::new(TrackLocalStaticRTP::new(
        RTCRtpCodecCapability {
            mime_type: MIME_TYPE_H264.to_owned(),
            clock_rate: 90000,
            channels: 0,
            sdp_fmtp_line: "level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42e01f".to_owned(),
            rtcp_feedback: vec![],
        },
        "video".to_owned(),
        format!("vms-camera-{}", camera_id),
    ));

    // 6) Add track to peer connection
    if let Err(e) = peer.add_track(Arc::clone(&video_track) as Arc<dyn TrackLocal + Send + Sync>).await {
        let _ = peer.close().await;
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ApiErrorBody::new("ADD_TRACK_FAILED", e.to_string()))));
    }

    // 7) Parse browser's offer and set remote description
    let browser_offer = RTCSessionDescription::offer(offer.sdp)
        .map_err(|e| {
            let _ = futures::executor::block_on(peer.close());
            (StatusCode::BAD_REQUEST, Json(ApiErrorBody::new("SDP_OFFER_INVALID", e.to_string())))
        })?;

    peer.set_remote_description(browser_offer)
        .await
        .map_err(|e| {
            let _ = futures::executor::block_on(peer.close());
            (StatusCode::BAD_REQUEST, Json(ApiErrorBody::new("SET_REMOTE_FAILED", e.to_string())))
        })?;

    // 8) Create answer
    let answer = peer
        .create_answer(None)
        .await
        .map_err(|e| {
            let _ = futures::executor::block_on(peer.close());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiErrorBody::new("CREATE_ANSWER_FAILED", e.to_string())))
        })?;

    // 9) Set local description
    peer.set_local_description(answer.clone())
        .await
        .map_err(|e| {
            let _ = futures::executor::block_on(peer.close());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiErrorBody::new("SET_LOCAL_FAILED", e.to_string())))
        })?;

    // 10) Get camera credentials for RTSP URL
    let (username, password) = state.camera_repo
        .get_credentials(camera_id)
        .await
        .ok()
        .flatten()
        .unwrap_or_else(|| ("".to_string(), "".to_string()));

    // Build authenticated RTSP URL
    let username_enc = utf8_percent_encode(&username, NON_ALPHANUMERIC).to_string();
    let password_enc = utf8_percent_encode(&password, NON_ALPHANUMERIC).to_string();
    let rtsp_url = if !username.is_empty() {
        camera.rtsp_url.replace("rtsp://", &format!("rtsp://{}:{}@", username_enc, password_enc))
    } else {
        camera.rtsp_url.clone()
    };

    // 11) Start streaming task
    let (task_handle, rtp_port) = spawn_rtsp_rtp_task(
        camera_id,
        rtsp_url,
        video_track.clone(),
    ).await.map_err(|e| {
        let _ = futures::executor::block_on(peer.close());
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiErrorBody::new("STREAM_START_FAILED", e)))
    })?;

    // 12) Register runtime for cleanup
    let expires_at = chrono::Utc::now().timestamp() + 3600; // 1 hour
    state.webrtc_runtime.insert(peer_id, PeerRuntime {
        peer: peer.clone(),
        track: video_track,
        rtp_port,
        task_handle,
        camera_id,
    }).await;

    let local = peer.local_description().await;
    let sdp_answer = local.map(|d| d.sdp).unwrap_or(answer.sdp);

    tracing::info!("✅ WebRTC session {} started for camera {} on port {}", peer_id, camera_id, rtp_port);

    Ok(Json(WebRtcAnswerResponse {
        sdp: sdp_answer,
        sdp_type: "answer".to_string(),
        peer_id,
        expires_at,
        rtp_port,
    }))
}

/// Handle ICE candidate from browser
async fn handle_ice(
    State(_state): State<AppState>,
    Path(_camera_id): Path<Uuid>,
    Json(_candidate): Json<IceCandidateRequest>,
) -> StatusCode {
    // ICE candidates are handled automatically by webrtc-rs trickle ICE
    // This endpoint exists for compatibility but is currently a no-op
    StatusCode::OK
}

/// Stop a WebRTC session
async fn handle_stop(
    State(state): State<AppState>,
    Path(peer_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ApiErrorBody>)> {
    if state.webrtc_runtime.remove(peer_id).await.is_some() {
        tracing::info!("🛑 WebRTC session {} stopped", peer_id);
        Ok(StatusCode::OK)
    } else {
        Err((StatusCode::NOT_FOUND, Json(ApiErrorBody::new("PEER_NOT_FOUND", "Peer not found"))))
    }
}
