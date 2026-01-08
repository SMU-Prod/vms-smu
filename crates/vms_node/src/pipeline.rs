//! GStreamer pipeline management

use uuid::Uuid;
use vms_core::StreamProfile;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Active pipeline session
pub struct PipelineSession {
    pub session_id: Uuid,
    pub camera_id: Uuid,
    pub rtsp_url: String,
    pub output_url: String,
    // TODO: GStreamer pipeline handle
}

/// Pipeline manager
pub struct PipelineManager {
    sessions: Arc<RwLock<HashMap<Uuid, PipelineSession>>>,
}

impl PipelineManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Start a live streaming pipeline
    pub async fn start_live(
        &self,
        session_id: Uuid,
        camera_id: Uuid,
        rtsp_url: String,
        username: String,
        password: String,
        profile: StreamProfile,
    ) -> anyhow::Result<String> {
        // TODO: Create GStreamer pipeline
        // RTSP source -> decode -> encode -> HLS/WebRTC output
        
        let output_url = format!("http://localhost:8080/live/{}/index.m3u8", session_id);
        
        let session = PipelineSession {
            session_id,
            camera_id,
            rtsp_url,
            output_url: output_url.clone(),
        };

        self.sessions.write().await.insert(session_id, session);

        tracing::info!("Started pipeline for session {}", session_id);
        Ok(output_url)
    }

    /// Stop a live streaming pipeline
    pub async fn stop_live(&self, session_id: Uuid) -> anyhow::Result<()> {
        // TODO: Stop GStreamer pipeline
        self.sessions.write().await.remove(&session_id);
        tracing::info!("Stopped pipeline for session {}", session_id);
        Ok(())
    }

    /// Get active session count
    pub async fn active_count(&self) -> usize {
        self.sessions.read().await.len()
    }
}
