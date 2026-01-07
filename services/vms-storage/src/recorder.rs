//! Continuous recording module
//! 
//! Handles 24/7 recording from NATS frames to disk

use anyhow::Result;
use async_nats::Client;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use vms_common::stream::VideoFrame;

pub struct ContinuousRecorder {
    camera_id: String,
    nats_client: Arc<Client>,
    storage_path: PathBuf,
    current_segment: Arc<RwLock<Option<SegmentWriter>>>,
}

impl ContinuousRecorder {
    pub fn new(camera_id: String, nats_client: Arc<Client>, storage_path: PathBuf) -> Self {
        Self {
            camera_id,
            nats_client,
            storage_path,
            current_segment: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn start(&self) -> Result<()> {
        info!("▶️  Starting recorder for camera: {}", self.camera_id);

        // Subscribe to NATS frames
        let subject = format!("vms.frames.{}", self.camera_id);
        let mut subscriber = self.nats_client.subscribe(subject.clone()).await?;
        
        info!("📡 Subscribed to: {}", subject);

        while let Some(msg) = subscriber.next().await {
            // Deserialize frame
            let frame: VideoFrame = match bincode::deserialize(&msg.payload) {
                Ok(f) => f,
                Err(e) => {
                    warn!("Failed to deserialize frame: {}", e);
                    continue;
                }
            };

            // Check if we need to rotate segment (hourly)
            if self.should_rotate_segment().await {
                self.rotate_segment().await?;
            }

            // Write frame to current segment
            if let Some(ref mut segment) = *self.current_segment.write().await {
                if let Err(e) = segment.write_frame(&frame).await {
                    error!("Failed to write frame: {}", e);
                }
            } else {
                // No segment yet, create first one
                self.rotate_segment().await?;
            }
        }

        Ok(())
    }

    async fn should_rotate_segment(&self) -> bool {
        let segment = self.current_segment.read().await;
        
        match segment.as_ref() {
            None => true, // No segment yet
            Some(seg) => {
                // Rotate every hour
                let elapsed = Utc::now().signed_duration_since(seg.start_time);
                elapsed.num_hours() >= 1
            }
        }
    }

    async fn rotate_segment(&self) -> Result<()> {
        // Close current segment if exists
        if let Some(segment) = self.current_segment.write().await.take() {
            segment.close().await?;
        }

        // Create new segment
        let now = Utc::now();
        let segment = SegmentWriter::new(
            &self.camera_id,
            &self.storage_path,
            now,
        ).await?;

        *self.current_segment.write().await = Some(segment);
        
        info!("🔄 Rotated segment for camera: {}", self.camera_id);
        
        Ok(())
    }
}

pub struct SegmentWriter {
    camera_id: String,
    pub start_time: DateTime<Utc>,
    file_path: PathBuf,
    pipeline: gstreamer::Pipeline,
    appsrc: gstreamer_app::AppSrc,
    frame_count: u64,
}

impl SegmentWriter {
    pub async fn new(
        camera_id: &str,
        storage_path: &PathBuf,
        start_time: DateTime<Utc>,
    ) -> Result<Self> {
        use gstreamer::prelude::*;

        // Create directory structure: /storage/cameras/{camera_id}/{YYYY-MM-DD}/
        let date_str = start_time.format("%Y-%m-%d").to_string();
        let camera_dir = storage_path.join(camera_id).join(&date_str);
        tokio::fs::create_dir_all(&camera_dir).await?;

        // Create file: video_HH.mkv
        let hour = start_time.format("%H").to_string();
        let file_name = format!("video_{}.mkv", hour);
        let file_path = camera_dir.join(&file_name);

        info!("📝 Creating segment: {:?}", file_path);

        // Build GStreamer pipeline for H264 passthrough to MKV
        // appsrc → h264parse → matroskamux → filesink
        
        let pipeline = gstreamer::Pipeline::new();

        // AppSrc - receives H264 frames
        let appsrc = gstreamer_app::AppSrc::builder()
            .name("source")
            .is_live(true)
            .format(gstreamer::Format::Time)
            .build();

        // Set caps for H264
        let caps = gstreamer::Caps::builder("video/x-h264")
            .field("stream-format", "byte-stream")
            .field("alignment", "au")
            .build();
        appsrc.set_caps(Some(&caps));

        // H264 parser
        let h264parse = gstreamer::ElementFactory::make("h264parse")
            .name("parser")
            .build()?;

        // Matroska muxer
        let mux = gstreamer::ElementFactory::make("matroskamux")
            .name("muxer")
            .property("writing-app", "VMS Storage v0.1.0")
            .build()?;

        // File sink
        let filesink = gstreamer::ElementFactory::make("filesink")
            .name("sink")
            .property("location", file_path.to_str().unwrap())
            .build()?;

        // Add elements to pipeline
        pipeline.add_many(&[appsrc.upcast_ref(), &h264parse, &mux, &filesink])?;

        // Link elements
        gstreamer::Element::link_many(&[appsrc.upcast_ref(), &h264parse, &mux, &filesink])?;

        // Start pipeline
        pipeline.set_state(gstreamer::State::Playing)?;

        info!("✅ GStreamer pipeline started for: {:?}", file_path);

        Ok(Self {
            camera_id: camera_id.to_string(),
            start_time,
            file_path,
            pipeline,
            appsrc,
            frame_count: 0,
        })
    }

    pub async fn write_frame(&mut self, frame: &VideoFrame) -> Result<()> {
        use gstreamer::prelude::*;

        // Create GStreamer buffer from frame data
        let mut buffer = gstreamer::Buffer::with_size(frame.data.len())?;
        {
            let buffer_ref = buffer.get_mut().unwrap();
            
            // Convert Timestamp to milliseconds
            let timestamp_ms = frame.timestamp.as_datetime().timestamp_millis() as u64;
            
            // Set timestamp
            buffer_ref.set_pts(gstreamer::ClockTime::from_mseconds(timestamp_ms));
            
            // Set DTS (decode timestamp) same as PTS for simplicity
            buffer_ref.set_dts(gstreamer::ClockTime::from_mseconds(timestamp_ms));
            
            // Mark keyframes
            if frame.is_keyframe {
                buffer_ref.unset_flags(gstreamer::BufferFlags::DELTA_UNIT);
            } else {
                buffer_ref.set_flags(gstreamer::BufferFlags::DELTA_UNIT);
            }

            // Copy frame data
            let mut map = buffer_ref.map_writable()?;
            map.copy_from_slice(&frame.data);
        }

        // Push buffer to appsrc
        self.appsrc.push_buffer(buffer)
            .map_err(|e| anyhow::anyhow!("Failed to push buffer: {:?}", e))?;

        self.frame_count += 1;

        // Log every 300 frames (~10s at 30fps)
        if self.frame_count % 300 == 0 {
            info!("📹 Recorded {} frames to {:?}", self.frame_count, self.file_path);
        }

        Ok(())
    }

    pub async fn close(self) -> Result<()> {
        use gstreamer::prelude::*;

        info!("💾 Closing segment: {:?} ({} frames)", self.file_path, self.frame_count);

        // Send EOS to appsrc
        self.appsrc.end_of_stream()
            .map_err(|e| anyhow::anyhow!("Failed to send EOS: {:?}", e))?;

        // Wait for EOS on pipeline
        let bus = self.pipeline.bus().expect("Pipeline has no bus");
        for msg in bus.iter_timed(gstreamer::ClockTime::from_seconds(5)) {
            use gstreamer::MessageView;
            match msg.view() {
                MessageView::Eos(..) => {
                    info!("✅ EOS received, segment finalized");
                    break;
                }
                MessageView::Error(err) => {
                    error!("❌ Error finalizing segment: {}", err.error());
                    break;
                }
                _ => {}
            }
        }

        // Stop pipeline
        self.pipeline.set_state(gstreamer::State::Null)?;

        info!("✅ Segment closed successfully: {:?}", self.file_path);

        Ok(())
    }
}
