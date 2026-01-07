//! Consumer NATS para processamento de IA em frames

use anyhow::{Context, Result};
use async_nats::{Client, Subscriber};
use std::sync::Arc;
use tokio_stream::StreamExt;
use tracing::{debug, error, info, warn};
use vms_common::stream::VideoFrame;
use vms_common::types::CameraId;

use crate::detector::{Detection, ObjectDetector};
use crate::tracker::Tracker;

/// Evento de IA gerado
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AIEvent {
    pub camera_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: String,
    pub detections: Vec<Detection>,
    pub frame_number: u64,
}

/// Consumer e processador de IA
pub struct AIProcessor {
    client: Client,
    detector: Option<Arc<ObjectDetector>>,
    trackers: Arc<tokio::sync::RwLock<std::collections::HashMap<CameraId, Tracker>>>,
    frame_count: Arc<std::sync::atomic::AtomicU64>,
    detection_count: Arc<std::sync::atomic::AtomicU64>,
}

impl AIProcessor {
    /// Conecta ao NATS
    pub async fn connect(nats_url: &str) -> Result<Self> {
        info!("🤖 Connecting AI processor to NATS at {}", nats_url);

        let client = async_nats::connect(nats_url)
            .await
            .context("Failed to connect to NATS")?;

        info!("✅ AI processor connected");

        Ok(Self {
            client,
            detector: None, // Será inicializado quando modelo estiver disponível
            trackers: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            frame_count: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            detection_count: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        })
    }

    /// Define o detector (quando modelo ONNX estiver disponível)
    pub fn set_detector(&mut self, detector: ObjectDetector) {
        self.detector = Some(Arc::new(detector));
        info!("🎯 Object detector loaded and ready");
    }

    /// Inicia processamento de frames
    pub async fn start_processing(&self) -> Result<()> {
        info!("🎬 Starting AI frame processor");

        let subscriber = self
            .client
            .subscribe("vms.frames.>")
            .await
            .context("Failed to subscribe to frames")?;

        let detector = self.detector.clone();
        let trackers = self.trackers.clone();
        let frame_count = self.frame_count.clone();
        let detection_count = self.detection_count.clone();
        let client = self.client.clone();

        tokio::spawn(async move {
            Self::process_frames(
                subscriber,
                detector,
                trackers,
                frame_count,
                detection_count,
                client,
            )
            .await;
        });

        Ok(())
    }

    async fn process_frames(
        mut subscriber: Subscriber,
        detector: Option<Arc<ObjectDetector>>,
        trackers: Arc<tokio::sync::RwLock<std::collections::HashMap<CameraId, Tracker>>>,
        frame_count: Arc<std::sync::atomic::AtomicU64>,
        detection_count: Arc<std::sync::atomic::AtomicU64>,
        client: Client,
    ) {
        info!("🤖 AI processor worker started");

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
                    let count = frame_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                    // Process every N frames (reduce load)
                    let process_interval = 30; // Process 1 frame per second at 30fps
                    if count % process_interval != 0 {
                        continue;
                    }

                    if let Ok(uuid) = camera_id_str.parse::<uuid::Uuid>() {
                        let camera_id = CameraId::from_uuid(uuid);

                        // Se detector disponível, processar
                        if let Some(ref det) = detector {
                            match det.detect(&frame.data, frame.width, frame.height) {
                                Ok(detections) => {
                                    if !detections.is_empty() {
                                        debug!("🎯 Detected {} objects", detections.len());

                                        // Get or create tracker
                                        let mut trackers_lock = trackers.write().await;
                                        let tracker = trackers_lock
                                            .entry(camera_id)
                                            .or_insert_with(|| Tracker::new(30, 3, 0.3));

                                        // Update tracker
                                        let tracks = tracker.update(detections.clone());

                                        detection_count.fetch_add(
                                            detections.len() as u64,
                                            std::sync::atomic::Ordering::Relaxed,
                                        );

                                        // Create AI event
                                        let event = AIEvent {
                                            camera_id: camera_id.to_string(),
                                            timestamp: chrono::Utc::now(),
                                            event_type: "object_detection".to_string(),
                                            detections,
                                            frame_number: count,
                                        };

                                        // Publish event
                                        let subject = format!("vms.events.ai.{}", camera_id);
                                        if let Ok(payload) = serde_json::to_vec(&event) {
                                            if let Err(e) =
                                                client.publish(subject, payload.into()).await
                                            {
                                                error!("Failed to publish AI event: {}", e);
                                            } else {
                                                debug!(
                                                    "📤 Published AI event: {} detections, {} tracks",
                                                    event.detections.len(),
                                                    tracks.len()
                                                );
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    warn!("Detection failed: {}", e);
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

        info!("🤖 AI processor worker stopped");
    }

    /// Retorna estatísticas
    pub fn get_stats(&self) -> (u64, u64) {
        let frames = self.frame_count.load(std::sync::atomic::Ordering::Relaxed);
        let detections = self
            .detection_count
            .load(std::sync::atomic::Ordering::Relaxed);
        (frames, detections)
    }
}
