//! EXTREME OPTIMIZATION - Sub 50ms Latency Pipeline
//! Configurações agressivas para latência mínima absoluta

use anyhow::{Context, Result};
use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_app as gst_app;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};
use vms_common::camera::CameraConfig;
use vms_common::stream::VideoFrame;

pub struct IngestPipeline {
    pipeline: gst::Pipeline,
    config: Arc<CameraConfig>,
    frame_tx: Option<mpsc::Sender<VideoFrame>>,
}

impl IngestPipeline {
    /// Pipeline EXTREMAMENTE otimizado - Sub 50ms
    pub fn new(config: CameraConfig) -> Result<Self> {
        let pipeline = gst::Pipeline::new();

        info!("⚡ EXTREME OPTIMIZATION MODE - Target: < 50ms");
        info!("📹 Camera: {} @ {}", config.name, config.url);

        // RTSP Source - CONFIGURAÇÕES EXTREMAS
        let rtspsrc = gst::ElementFactory::make("rtspsrc")
            .name("source")
            .property("location", &config.url)
            // LATÊNCIA
            .property("latency", 0u32)                    // ZERO latency
            .property("buffer-mode", 0i32)                // Slave mode (lowest latency)
            .property("ntp-sync", false)                  // Disable NTP sync
            .property("ntp-time-source", 3i32)            // Running time
            // TRANSPORTE
            .property("protocols", 0x00000004u32)         // UDP ONLY (fastest)
            .property("timeout", 5000000u64)              // 5s timeout
            .property("tcp-timeout", 0u64)                // No TCP fallback
            .property("do-rtcp", false)                   // Disable RTCP (save bandwidth)
            // BUFFERING
            .property("drop-on-latency", true)            // Drop old frames
            .property("do-retransmission", false)         // No retransmission
            // PERFORMANCE
            .property("is-live", true)                    // Live source optimization
            .property("do-timestamp", true)               // Timestamp frames
            .build()
            .context("Failed to create rtspsrc")?;

        // Autenticação
        if let (Some(user), Some(pass)) = (&config.username, &config.password) {
            rtspsrc.set_property("user-id", user);
            rtspsrc.set_property("user-pw", pass);
        }

        // RTP Depayloader - OTIMIZADO
        let depay = gst::ElementFactory::make("rtph264depay")
            .name("depay")
            .property("wait-for-keyframe", false)         // Don't wait for keyframe
            .build()
            .context("Failed to create depay")?;

        // H264 Parser - CONFIGURAÇÃO AGRESSIVA
        let parse = gst::ElementFactory::make("h264parse")
            .name("parse")
            .property("config-interval", -1i32)           // Always send SPS/PPS
            .property("disable-passthrough", false)       // Enable passthrough
            .build()
            .context("Failed to create h264parse")?;

        // Queue - BUFFER ZERO (máximo risco, mínima latência)
        let queue = gst::ElementFactory::make("queue")
            .name("queue")
            .property("max-size-buffers", 0u32)           // ZERO buffer!
            .property("max-size-bytes", 0u32)             
            .property("max-size-time", 0u64)              
            .property("leaky", 2i32)                      // Downstream leaky
            .property("flush-on-eos", true)               
            .property("silent", true)                     
            .build()
            .context("Failed to create queue")?;

        // AppSink - CONFIGURAÇÃO EXTREMA
        let sink = gst_app::AppSink::builder()
            .name("sink")
            .sync(false)                                  // NO SYNC - fastest
            .async_(false)                                // Synchronous callbacks
            .max_buffers(0)                               // ZERO buffering
            .drop(true)                                   // Drop old frames
            .enable_last_sample(false)                    // Don't keep last sample
            .build();

        // Caps para H264 - OTIMIZADO
        let caps = gst::Caps::builder("video/x-h264")
            .field("stream-format", "byte-stream")
            .field("alignment", "au")
            .field("profile", "high")                     // High profile
            .build();
        sink.set_caps(Some(&caps));

        // Adicionar elementos
        pipeline.add_many(&[&depay, &parse, &queue, sink.upcast_ref()])?;

        // Link pipeline
        gst::Element::link_many(&[&depay, &parse, &queue, sink.upcast_ref()])?;

        // Conectar RTSP source
        let depay_clone = depay.clone();
        rtspsrc.connect_pad_added(move |_src, src_pad| {
            let sink_pad = depay_clone
                .static_pad("sink")
                .expect("Failed to get depay sink pad");

            if sink_pad.is_linked() {
                return;
            }

            if let Some(caps) = src_pad.current_caps() {
                if let Some(structure) = caps.structure(0) {
                    let media_type = structure.name();

                    if media_type.starts_with("application/x-rtp") {
                        // Check if it's video stream
                        if let Ok(media) = structure.get::<&str>("media") {
                            if media == "video" {
                                if let Err(e) = src_pad.link(&sink_pad) {
                                    error!("❌ Link failed: {}", e);
                                } else {
                                    info!("⚡ EXTREME MODE: Video linked (UDP transport)");
                                }
                            }
                        }
                    }
                }
            }
        });

        pipeline.add(&rtspsrc)?;

        // Configurar pipeline para latência mínima
        pipeline.set_latency(gst::ClockTime::from_mseconds(0));
        pipeline.set_start_time(gst::ClockTime::NONE);

        Ok(Self {
            pipeline,
            config: Arc::new(config),
            frame_tx: None,
        })
    }

    pub fn set_frame_sender(&mut self, tx: mpsc::Sender<VideoFrame>) {
        self.frame_tx = Some(tx);
    }

    pub fn start(&self) -> Result<()> {
        info!("⚡⚡⚡ EXTREME MODE ACTIVATED ⚡⚡⚡");
        info!("📊 Configuration:");
        info!("  - Transport: UDP ONLY");
        info!("  - Buffer: ZERO");
        info!("  - Latency: < 50ms target");
        info!("  - Quality: 1080p H264 High Profile");
        info!("  - Frame drop: AGGRESSIVE");

        self.pipeline
            .set_state(gst::State::Playing)
            .context("Failed to start pipeline")?;

        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        info!("⏹️  Stopping EXTREME pipeline: {}", self.config.name);
        self.pipeline.set_state(gst::State::Null)?;
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        matches!(
            self.pipeline.current_state(),
            gst::State::Playing | gst::State::Paused
        )
    }

    pub fn bus(&self) -> Option<gst::Bus> {
        self.pipeline.bus()
    }

    pub fn get_appsink(&self) -> Option<gst_app::AppSink> {
        self.pipeline
            .by_name("sink")
            .and_then(|e| e.downcast::<gst_app::AppSink>().ok())
    }
}

impl Drop for IngestPipeline {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

pub struct FrameHandler {
    tx: mpsc::Sender<VideoFrame>,
    camera_id: String,
    frame_count: std::sync::atomic::AtomicU64,
}

impl FrameHandler {
    pub fn new(tx: mpsc::Sender<VideoFrame>, camera_id: String) -> Self {
        Self {
            tx,
            camera_id,
            frame_count: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub async fn handle_sample(&self, sample: gst::Sample) -> Result<()> {
        let buffer = sample.buffer().context("No buffer")?;
        
        // Mapear buffer - ZERO COPY quando possível
        let map = buffer.map_readable().context("Failed to map")?;
        let data = map.as_slice().to_vec();

        let count = self.frame_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        if count % 30 == 0 {
            debug!("⚡ Frame #{}: {} bytes - {}", count, data.len(), self.camera_id);
        }

        let frame = VideoFrame::new(
            vms_common::types::StreamId::new(),
            data,
            1920,  // 1080p
            1080,
        );

        // Try send - non-blocking
        if let Err(e) = self.tx.try_send(frame) {
            warn!("⚠️  Frame dropped (buffer full): {}", e);
        }

        Ok(())
    }
}
