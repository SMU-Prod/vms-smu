//! Consumer NATS para distribuição de frames

use anyhow::{Context, Result};
use async_nats::{Client, Subscriber};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::StreamExt;
use tracing::{error, info, warn};
use vms_common::stream::VideoFrame;
use vms_common::types::{CameraId, StreamId};

/// Frame buffer para um stream ativo
struct StreamBuffer {
    tx: mpsc::Sender<VideoFrame>,
    viewer_count: usize,
}

/// Consumer e distribuidor de frames
pub struct StreamDistributor {
    client: Client,
    // Map: camera_id -> streams para essa câmera
    streams: Arc<RwLock<HashMap<CameraId, HashMap<StreamId, StreamBuffer>>>>,
    frame_count: Arc<std::sync::atomic::AtomicU64>,
}

impl StreamDistributor {
    /// Conecta ao NATS
    pub async fn connect(nats_url: &str) -> Result<Self> {
        info!("🔗 Connecting to NATS at {}", nats_url);

        let client = async_nats::connect(nats_url)
            .await
            .context("Failed to connect to NATS")?;

        info!("✅ Stream distributor connected");

        Ok(Self {
            client,
            streams: Arc::new(RwLock::new(HashMap::new())),
            frame_count: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        })
    }

    /// Inicia distribuição de frames
    pub async fn start_distributing(&self) -> Result<()> {
        info!("📡 Starting frame distributor");

        let subscriber = self
            .client
            .subscribe("vms.frames.>")
            .await
            .context("Failed to subscribe to frames")?;

        let streams = self.streams.clone();
        let frame_count = self.frame_count.clone();

        tokio::spawn(async move {
            Self::distribute_frames(subscriber, streams, frame_count).await;
        });

        Ok(())
    }

    async fn distribute_frames(
        mut subscriber: Subscriber,
        streams: Arc<RwLock<HashMap<CameraId, HashMap<StreamId, StreamBuffer>>>>,
        frame_count: Arc<std::sync::atomic::AtomicU64>,
    ) {
        info!("📺 Frame distributor worker started");

        while let Some(message) = subscriber.next().await {
            // Extract camera_id from subject
            let subject_parts: Vec<&str> = message.subject.split('.').collect();
            if subject_parts.len() < 3 {
                continue;
            }

            let camera_id_str = subject_parts[2];

            // Deserialize frame
            match serde_json::from_slice::<VideoFrame>(&message.payload) {
                Ok(frame) => {
                    frame_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                    if let Ok(uuid) = camera_id_str.parse::<uuid::Uuid>() {
                        let camera_id = CameraId::from_uuid(uuid);

                        // Distribuir para todos os streams dessa câmera
                        let streams_lock = streams.read().await;

                        if let Some(camera_streams) = streams_lock.get(&camera_id) {
                            for (stream_id, buffer) in camera_streams.iter() {
                                if let Err(e) = buffer.tx.try_send(frame.clone()) {
                                    if !matches!(e, mpsc::error::TrySendError::Full(_)) {
                                        warn!("Stream {} closed, will cleanup", stream_id);
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to deserialize frame: {}", e);
                }
            }
        }

        info!("📺 Frame distributor worker stopped");
    }

    /// Cria um novo stream para uma câmera
    pub async fn create_stream(
        &self,
        camera_id: CameraId,
        buffer_size: usize,
    ) -> Result<(StreamId, mpsc::Receiver<VideoFrame>)> {
        let stream_id = StreamId::new();
        let (tx, rx) = mpsc::channel(buffer_size);

        let buffer = StreamBuffer {
            tx,
            viewer_count: 1,
        };

        let mut streams = self.streams.write().await;
        streams
            .entry(camera_id)
            .or_insert_with(HashMap::new)
            .insert(stream_id, buffer);

        info!(
            "➕ Created stream {} for camera {}",
            stream_id, camera_id
        );

        Ok((stream_id, rx))
    }

    /// Remove um stream
    pub async fn remove_stream(&self, camera_id: CameraId, stream_id: StreamId) -> Result<()> {
        let mut streams = self.streams.write().await;

        if let Some(camera_streams) = streams.get_mut(&camera_id) {
            camera_streams.remove(&stream_id);

            if camera_streams.is_empty() {
                streams.remove(&camera_id);
            }

            info!(
                "➖ Removed stream {} for camera {}",
                stream_id, camera_id
            );
        }

        Ok(())
    }

    /// Retorna estatísticas
    pub async fn get_stats(&self) -> (usize, usize, u64) {
        let streams = self.streams.read().await;
        let total_cameras = streams.len();
        let total_streams: usize = streams.values().map(|s| s.len()).sum();
        let frames = self.frame_count.load(std::sync::atomic::Ordering::Relaxed);

        (total_cameras, total_streams, frames)
    }
}
