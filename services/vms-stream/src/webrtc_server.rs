//! WebRTC streaming server

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
            "Created WebRTC session {} for camera {} viewer {}",
            stream_id, camera_id, viewer_id
        );

        Ok(stream_id)
    }

    /// Manipula oferta SDP
    pub async fn handle_offer(&self, stream_id: StreamId, _sdp: String) -> Result<String> {
        let sessions = self.sessions.read().await;

        if !sessions.contains_key(&stream_id) {
            anyhow::bail!("Session not found");
        }

        // Em produção real:
        // 1. Criar PeerConnection
        // 2. Set remote description (offer)
        // 3. Create answer
        // 4. Set local description
        // 5. Return answer SDP

        let answer_sdp = "v=0\r\no=- 0 0 IN IP4 127.0.0.1\r\ns=-\r\nt=0 0\r\n".to_string();

        Ok(answer_sdp)
    }

    /// Manipula candidatos ICE
    pub async fn handle_ice_candidate(
        &self,
        stream_id: StreamId,
        _candidate: String,
    ) -> Result<()> {
        let sessions = self.sessions.read().await;

        if !sessions.contains_key(&stream_id) {
            anyhow::bail!("Session not found");
        }

        // Em produção: adicionar ICE candidate ao PeerConnection

        Ok(())
    }

    /// Encerra uma sessão
    pub async fn close_session(&self, stream_id: StreamId) -> Result<()> {
        let mut sessions = self.sessions.write().await;

        if let Some(session) = sessions.remove(&stream_id) {
            info!(
                "Closed WebRTC session {} for camera {}",
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
