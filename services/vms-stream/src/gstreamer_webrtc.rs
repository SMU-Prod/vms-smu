//! GStreamer WebRTC Pipeline - Ultra Low Latency
//!
//! Creates RTSP → webrtcbin pipeline for sub-100ms latency streaming
//! Uses GLib main context for proper GStreamer async operation

use anyhow::{Context, Result};
use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_sdp as gst_sdp;
use gstreamer_webrtc as gst_webrtc;
use std::sync::{Arc, Mutex};
use tracing::{error, info};

/// WebRTC stream session using GStreamer
pub struct GstWebRTCSession {
    pub pipeline: gst::Pipeline,
    pub webrtcbin: gst::Element,
    pub camera_id: String,
}

impl GstWebRTCSession {
    /// Create a new GStreamer WebRTC session for a camera
    pub fn new(
        camera_id: String,
        rtsp_url: &str,
        username: &str,
        password: &str,
    ) -> Result<Self> {
        info!("🚀 Creating GStreamer WebRTC session for camera: {}", camera_id);
        
        info!("📹 RTSP URL: {} (user: {})", rtsp_url, username);
        
        // Create pipeline
        let pipeline = gst::Pipeline::new();
        
        // RTSP Source with authentication
        info!("🎬 Creating RTSP pipeline with authentication");
        let rtspsrc = gst::ElementFactory::make("rtspsrc")
            .name("source")
            .property("location", rtsp_url)
            .property("user-id", username)
            .property("user-pw", password)
            .property("latency", 0u32)
            .property("drop-on-latency", true)
            .build()
            .context("Failed to create rtspsrc")?;
        
        // RTP Depayloader for H.264
        let rtph264depay = gst::ElementFactory::make("rtph264depay")
            .name("depay")
            .build()
            .context("Failed to create rtph264depay")?;
            
        // H.264 Parser
        let h264parse = gst::ElementFactory::make("h264parse")
            .name("parse")
            .build()
            .context("Failed to create h264parse")?;
        
        // RTP Payloader for WebRTC
        let rtph264pay = gst::ElementFactory::make("rtph264pay")
            .name("pay")
            .property("config-interval", -1i32)
            .property("pt", 96u32)
            .build()
            .context("Failed to create rtph264pay")?;
        
        // Caps filter for RTP
        let capsfilter = gst::ElementFactory::make("capsfilter")
            .name("capsfilter")
            .build()
            .context("Failed to create capsfilter")?;
        
        let caps = gst::Caps::builder("application/x-rtp")
            .field("media", "video")
            .field("clock-rate", 90000i32)
            .field("encoding-name", "H264")
            .field("payload", 96i32)
            .build();
        capsfilter.set_property("caps", &caps);
        
        // WebRTC bin
        let webrtcbin = gst::ElementFactory::make("webrtcbin")
            .name("webrtc")
            .property_from_str("bundle-policy", "max-bundle")
            .property_from_str("stun-server", "stun://stun.l.google.com:19302")
            .build()
            .context("Failed to create webrtcbin")?;
        
        // Add elements to pipeline
        pipeline.add_many([&rtspsrc, &rtph264depay, &h264parse, &rtph264pay, &capsfilter, &webrtcbin])?;
        
        // Link static elements (depay -> parse -> pay -> capsfilter)
        gst::Element::link_many([&rtph264depay, &h264parse, &rtph264pay, &capsfilter])
            .context("Failed to link video elements")?;
        
        // Link capsfilter to webrtcbin
        let caps_src = capsfilter.static_pad("src").expect("No src pad on capsfilter");
        let webrtc_sink = webrtcbin.request_pad_simple("sink_%u").expect("No sink pad on webrtcbin");
        caps_src.link(&webrtc_sink)?;
        
        // Set the transceiver direction to SENDONLY and configure H.264 codec
        webrtcbin.connect("on-new-transceiver", false, |values| {
            if let Some(transceiver) = values.get(1).and_then(|v| v.get::<gst_webrtc::WebRTCRTPTransceiver>().ok()) {
                transceiver.set_property("direction", gst_webrtc::WebRTCRTPTransceiverDirection::Sendonly);
                
                // Force H.264 codec by setting codec preferences
                let h264_caps = gst::Caps::builder("application/x-rtp")
                    .field("media", "video")
                    .field("encoding-name", "H264")
                    .field("clock-rate", 90000i32)
                    .field("payload", 96i32)
                    .build();
                transceiver.set_property("codec-preferences", &h264_caps);
                
                info!("📡 Transceiver configured: SENDONLY + H.264 codec");
            }
            None
        });
        
        // Connect RTSP dynamic pads to depayloader
        let depay_clone = rtph264depay.clone();
        rtspsrc.connect_pad_added(move |_src, src_pad| {
            let pad_name = src_pad.name().to_string();
            info!("🔗 RTSP pad added: {}", pad_name);
            
            // Only link video RTP pads
            if pad_name.contains("recv_rtp_src") {
                let sink_pad = depay_clone.static_pad("sink").expect("No sink pad");
                if !sink_pad.is_linked() {
                    match src_pad.link(&sink_pad) {
                        Ok(_) => info!("✅ RTSP video stream linked!"),
                        Err(e) => error!("❌ Failed to link RTSP: {}", e),
                    }
                }
            }
        });
        
        pipeline.set_latency(gst::ClockTime::from_mseconds(0));
        
        info!("✅ GStreamer WebRTC pipeline created for camera: {}", camera_id);
        
        Ok(Self {
            pipeline,
            webrtcbin,
            camera_id,
        })
    }
    
    /// Handle SDP offer from browser - runs GLib main context to process GStreamer events
    pub fn handle_offer(&self, offer_sdp: String) -> Result<String> {
        info!("📥 Processing SDP offer for camera: {} ({} bytes)", self.camera_id, offer_sdp.len());
        
        // Parse SDP offer
        let sdp = gst_sdp::SDPMessage::parse_buffer(offer_sdp.as_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to parse SDP: {:?}", e))?;
        
        let offer = gst_webrtc::WebRTCSessionDescription::new(
            gst_webrtc::WebRTCSDPType::Offer,
            sdp,
        );
        
        // Set remote description
        info!("📥 Setting remote description");
        self.webrtcbin.emit_by_name::<()>(
            "set-remote-description",
            &[&offer, &None::<gst::Promise>],
        );
        
        // Use shared state for answer
        let answer_result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
        let answer_clone = answer_result.clone();
        let webrtc = self.webrtcbin.clone();
        
        // Create answer with callback
        info!("📤 Creating SDP answer");
        let promise = gst::Promise::with_change_func(move |reply| {
            match reply {
                Ok(Some(reply)) => {
                    if let Ok(answer_value) = reply.value("answer") {
                        if let Ok(answer) = answer_value.get::<gst_webrtc::WebRTCSessionDescription>() {
                            // Set local description
                            webrtc.emit_by_name::<()>(
                                "set-local-description",
                                &[&answer, &None::<gst::Promise>],
                            );
                            
                            let sdp_text = answer.sdp().to_string();
                            info!("📤 Answer created ({} bytes)", sdp_text.len());
                            info!("📤 SDP Answer content:\n{}", sdp_text);
                            
                            if let Ok(mut guard) = answer_clone.lock() {
                                *guard = Some(sdp_text);
                            }
                        }
                    }
                }
                Ok(None) => error!("create-answer returned None"),
                Err(e) => error!("create-answer failed: {:?}", e),
            }
        });
        
        self.webrtcbin.emit_by_name::<()>("create-answer", &[&None::<gst::Structure>, &promise]);
        
        // Pump GLib main context to process events
        let main_context = glib::MainContext::default();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        
        loop {
            // Check if we have an answer
            if let Ok(guard) = answer_result.lock() {
                if guard.is_some() {
                    break;
                }
            }
            
            // Check timeout
            if std::time::Instant::now() > deadline {
                return Err(anyhow::anyhow!("Timeout waiting for SDP answer"));
            }
            
            // Pump main context to process GStreamer events
            while main_context.iteration(false) {}
            
            // Small sleep to prevent busy-waiting
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        
        // Extract answer
        let answer = answer_result.lock()
            .map_err(|_| anyhow::anyhow!("Lock poisoned"))?
            .take()
            .ok_or_else(|| anyhow::anyhow!("No answer generated"))?;
        
        info!("✅ SDP answer ready for camera: {}", self.camera_id);
        
        Ok(answer)
    }
    
    /// Start the pipeline
    pub fn start(&self) -> Result<()> {
        info!("▶️ Starting GStreamer pipeline for camera: {}", self.camera_id);
        self.pipeline.set_state(gst::State::Playing)?;
        Ok(())
    }
    
    /// Stop the pipeline
    pub fn stop(&self) -> Result<()> {
        info!("⏹️ Stopping GStreamer pipeline for camera: {}", self.camera_id);
        self.pipeline.set_state(gst::State::Null)?;
        Ok(())
    }
}

impl Drop for GstWebRTCSession {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Initialize GStreamer and start GLib main loop (call once at startup)
pub fn init() -> Result<()> {
    gst::init().context("Failed to initialize GStreamer")?;
    info!("✅ GStreamer initialized");
    
    // Start a dedicated thread for GLib main loop
    // This is REQUIRED for GStreamer callbacks and data flow to work!
    std::thread::spawn(|| {
        let main_context = glib::MainContext::default();
        
        // IMPORTANT: Must acquire context for this thread to process callbacks
        main_context.acquire();
        info!("🔄 GLib main loop thread started and context acquired");
        
        // Run iteration loop forever - this processes all GStreamer events
        loop {
            // process all pending events
            while main_context.iteration(false) {}
            // small sleep to avoid busy-wait
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    });
    
    // Give the main loop time to start
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    info!("✅ GLib event processing thread started");
    Ok(())
}
