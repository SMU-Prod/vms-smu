//! Consumer NATS para frames de vídeo

use anyhow::{Context, Result};
use async_nats::{Client, Subscriber};
use chrono::Utc;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_stream::StreamExt;
use tracing::{debug, error, info, warn};
use vms_common::stream::VideoFrame;
use vms_common::types::CameraId;

use crate::writer::VideoWriter;

/// Consumer de frames do NATS
pub struct NatsConsumer {
    client: Client,
    writers: Arc<RwLock<std::collections::HashMap<CameraId, VideoWriter>>>,
    base_storage_path: PathBuf,
}

impl NatsConsumer {
    /// Conecta ao NATS
    pub async fn connect(nats_url: &str, storage_path: PathBuf) -> Result<Self> {
        info!("Connecting to NATS at {}", nats_url);

        let client = async_nats::connect(nats_url)
            .await
            .context("Failed to connect to NATS")?;

        info!("✅ NATS consumer connected");

        Ok(Self {
            client,
            writers: Arc::new(RwLock::new(std::collections::HashMap::new())),
            base_storage_path: storage_path,
        })
    }

    /// Inicia subscription para receber frames
    pub async fn start_consuming(&self) -> Result<()> {
        info!("🎬 Starting frame consumer");

        // Subscribe to all camera frames
        let subscriber = self
            .client
            .subscribe("vms.frames.>")
            .await
            .context("Failed to subscribe to vms.frames")?;

        let writers = self.writers.clone();
        let base_path = self.base_storage_path.clone();

        tokio::spawn(async move {
            Self::consume_frames(subscriber, writers, base_path).await;
        });

        Ok(())
    }

    async fn consume_frames(
        mut subscriber: Subscriber,
        writers: Arc<RwLock<std::collections::HashMap<CameraId, VideoWriter>>>,
        base_path: PathBuf,
    ) {
        info!("📥 Frame consumer worker started");
        let mut frame_count = 0u64;

        while let Some(message) = subscriber.next().await {
            // Extract camera_id from subject (vms.frames.{camera_id})
            let subject_parts: Vec<&str> = message.subject.split('.').collect();
            if subject_parts.len() < 3 {
                warn!("Invalid subject format: {}", message.subject);
                continue;
            }

            let camera_id_str = subject_parts[2];

            // Deserializar frame
            match serde_json::from_slice::<VideoFrame>(&message.payload) {
                Ok(frame) => {
                    frame_count += 1;

                    // Parse camera ID
                    if let Ok(uuid) = camera_id_str.parse::<uuid::Uuid>() {
                        let camera_id = CameraId::from_uuid(uuid);

                        // Get or create writer
                        let mut writers_lock = writers.write().await;
                        let writer = writers_lock
                            .entry(camera_id)
                            .or_insert_with(|| {
                                info!("📝 Creating new video writer for camera {}", camera_id);
                                VideoWriter::new(camera_id, base_path.clone())
                                    .expect("Failed to create writer")
                            });

                        // Write frame
                        let timestamp = Utc::now();
                        let is_keyframe = frame_count % 30 == 1; // Simplified keyframe detection

                        if let Err(e) = writer.write_frame(&frame.data, timestamp, is_keyframe) {
                            error!("Failed to write frame for camera {}: {}", camera_id, e);
                        }

                        // Flush periodically
                        if frame_count % 30 == 0 {
                            if let Err(e) = writer.flush() {
                                error!("Failed to flush writer: {}", e);
                            }
                            debug!(
                                "💾 Stored {} frames for camera {}",
                                frame_count, camera_id
                            );
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to deserialize frame: {}", e);
                }
            }
        }

        info!("📥 Frame consumer worker stopped");
    }

    /// Retorna estatísticas
    pub async fn get_stats(&self) -> (usize, u64) {
        let writers = self.writers.read().await;
        (writers.len(), 0) // TODO: add frame count
    }
}
