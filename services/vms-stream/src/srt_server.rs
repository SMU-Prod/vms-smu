//! SRT streaming server

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use vms_common::types::StreamId;

pub struct SRTServer {
    streams: Arc<RwLock<HashMap<StreamId, SRTStream>>>,
    port: u16,
}

struct SRTStream {
    stream_id: StreamId,
    camera_id: String,
}

impl SRTServer {
    pub fn new(port: u16) -> Self {
        Self {
            streams: Arc::new(RwLock::new(HashMap::new())),
            port,
        }
    }

    /// Inicia servidor SRT
    pub async fn start(&self) -> Result<()> {
        info!("SRT server would listen on port {}", self.port);

        // Em produção real:
        // 1. Bind SRT socket
        // 2. Accept connections
        // 3. Stream video data

        Ok(())
    }

    /// Cria stream SRT
    pub async fn create_stream(&self, camera_id: String) -> Result<StreamId> {
        let stream_id = StreamId::new();

        let stream = SRTStream {
            stream_id,
            camera_id: camera_id.clone(),
        };

        self.streams.write().await.insert(stream_id, stream);

        info!(
            "Created SRT stream {} for camera {}",
            stream_id, camera_id
        );

        Ok(stream_id)
    }

    /// Para stream
    pub async fn stop_stream(&self, stream_id: StreamId) -> Result<()> {
        self.streams.write().await.remove(&stream_id);
        Ok(())
    }

    pub async fn active_streams(&self) -> usize {
        self.streams.read().await.len()
    }
}
