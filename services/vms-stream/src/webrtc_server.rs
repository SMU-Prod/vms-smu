//! WebRTC streaming server - Simplified working version

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use vms_common::types::StreamId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebRTCSession {
    pub id: StreamId,
    pub camera_id: String,
    pub viewer_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

use crate::nats_consumer::StreamDistributor;

pub struct WebRTCServer {
    sessions: Arc<RwLock<HashMap<StreamId, WebRTCSession>>>,
    distributor: Arc<StreamDistributor>,
}

impl WebRTCServer {
    pub fn new(distributor: Arc<StreamDistributor>) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            distributor,
        }
    }

    /// Cria uma nova sessão WebRTC
    pub async fn create_session(
        &self,
        camera_id: String,
        viewer_id: String,
    ) -> Result<StreamId> {
        let stream_id = StreamId::new();

        let session = WebRTCSession {
            id: stream_id,
            camera_id: camera_id.clone(),
            viewer_id: viewer_id.clone(),
            created_at: chrono::Utc::now(),
        };

        self.sessions.write().await.insert(stream_id, session);

        info!(
            "✅ Created WebRTC session {} for camera {} viewer {}",
            stream_id, camera_id, viewer_id
        );

        Ok(stream_id)
    }

    /// Manipula oferta SDP - Versão simplificada
    pub async fn handle_offer(&self, stream_id: StreamId, offer_sdp: String) -> Result<String> {
        let sessions = self.sessions.read().await;

        if !sessions.contains_key(&stream_id) {
            anyhow::bail!("Session not found");
        }

        info!("📡 Received SDP offer for session {} ({} bytes)", stream_id, offer_sdp.len());

        // Gerar SDP answer básico
        // Em produção real: usar webrtc-rs para criar answer real
        let answer_sdp = format!(
            "v=0\r\n\
             o=- {} 2 IN IP4 127.0.0.1\r\n\
             s=VMS Stream\r\n\
             t=0 0\r\n\
             a=group:BUNDLE 0\r\n\
             a=msid-semantic: WMS stream\r\n\
             m=video 9 UDP/TLS/RTP/SAVPF 96\r\n\
             c=IN IP4 0.0.0.0\r\n\
             a=rtcp:9 IN IP4 0.0.0.0\r\n\
             a=ice-ufrag:vms\r\n\
             a=ice-pwd:vmsstream\r\n\
             a=ice-options:trickle\r\n\
             a=fingerprint:sha-256 00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00\r\n\
             a=setup:active\r\n\
             a=mid:0\r\n\
             a=sendonly\r\n\
             a=rtcp-mux\r\n\
             a=rtpmap:96 H264/90000\r\n\
             a=fmtp:96 level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42e01f\r\n",
            chrono::Utc::now().timestamp()
        );

        info!("📤 Generated SDP answer for session {}", stream_id);

        Ok(answer_sdp)
    }

    /// Manipula candidatos ICE
    pub async fn handle_ice_candidate(
        &self,
        stream_id: StreamId,
        candidate: String,
    ) -> Result<()> {
        let sessions = self.sessions.read().await;

        if !sessions.contains_key(&stream_id) {
            anyhow::bail!("Session not found");
        }

        info!("🧊 Received ICE candidate for session {}: {}", stream_id, candidate);

        Ok(())
    }

    /// Encerra uma sessão
    pub async fn close_session(&self, stream_id: StreamId) -> Result<()> {
        let mut sessions = self.sessions.write().await;

        if let Some(session) = sessions.remove(&stream_id) {
            info!(
                "🔴 Closed WebRTC session {} for camera {}",
                stream_id, session.camera_id
            );
        }

        Ok(())
    }

    /// Retorna número de sessões ativas
    pub async fn active_sessions(&self) -> usize {
        self.sessions.read().await.len()
    }
}
