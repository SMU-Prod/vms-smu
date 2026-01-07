//! Publicador NATS para frames de vídeo

use anyhow::{Context, Result};
use async_nats::Client;
use tokio::sync::mpsc;
use tracing::{debug, error, info};
use vms_common::stream::VideoFrame;

/// Publicador de frames para NATS
pub struct NatsPublisher {
    client: Client,
    subject_prefix: String,
}

impl NatsPublisher {
    /// Conecta ao NATS
    pub async fn connect(nats_url: &str) -> Result<Self> {
        info!("Connecting to NATS at {}", nats_url);

        let client = async_nats::connect(nats_url)
            .await
            .context("Failed to connect to NATS")?;

        info!("Connected to NATS successfully");

        Ok(Self {
            client,
            subject_prefix: "vms.frames".to_string(),
        })
    }

    /// Inicia worker para publicar frames
    pub async fn start_publishing(
        &self,
        mut rx: mpsc::Receiver<VideoFrame>,
        camera_id: String,
    ) -> Result<()> {
        let client = self.client.clone();
        let subject = format!("{}.{}", self.subject_prefix, camera_id);

        info!("Starting frame publisher for camera: {}", camera_id);

        tokio::spawn(async move {
            let mut frame_count = 0u64;

            while let Some(frame) = rx.recv().await {
                // Serializar frame para JSON (em produção usar protobuf)
                match serde_json::to_vec(&frame) {
                    Ok(payload) => {
                        if let Err(e) = client.publish(subject.clone(), payload.into()).await {
                            error!("Failed to publish frame: {}", e);
                        } else {
                            frame_count += 1;
                            if frame_count % 30 == 0 {
                                debug!("Published {} frames for camera {}", frame_count, camera_id);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to serialize frame: {}", e);
                    }
                }
            }

            info!("Frame publisher stopped for camera: {}", camera_id);
        });

        Ok(())
    }

    /// Publica um frame individual
    pub async fn publish_frame(&self, camera_id: &str, frame: &VideoFrame) -> Result<()> {
        let subject = format!("{}.{}", self.subject_prefix, camera_id);
        let payload = serde_json::to_vec(frame)?;

        self.client
            .publish(subject, payload.into())
            .await
            .context("Failed to publish frame")?;

        Ok(())
    }
}
