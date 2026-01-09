//! VMS Native Player - Ultra Low Latency (30-80ms)
//! Hardware accelerated video player with GStreamer

use anyhow::Result;
use clap::Parser;
use gstreamer as gst;
use gstreamer::prelude::*;
use tracing::{error, info};

#[derive(Parser, Debug)]
#[command(name = "vms-player")]
#[command(about = "Ultra low-latency video player for VMS", long_about = None)]
struct Args {
    /// RTSP URL
    #[arg(short, long)]
    url: String,

    /// Username
    #[arg(short = 'U', long)]
    username: Option<String>,

    /// Password
    #[arg(short = 'P', long)]
    password: Option<String>,

    /// Enable hardware decode (NVIDIA/Intel)
    #[arg(long, default_value_t = true)]
    hw_decode: bool,

    /// Window width (default: 960)
    #[arg(long, default_value_t = 960)]
    width: u32,

    /// Window height (default: 540)
    #[arg(long, default_value_t = 540)]
    height: u32,

    /// Fullscreen mode
    #[arg(long, default_value_t = false)]
    fullscreen: bool,
}

fn main() -> Result<()> {
    // Setup logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    let args = Args::parse();

    info!("🎬 VMS Ultra Low-Latency Player");
    info!("📹 URL: {}", args.url);
    info!("⚡ Hardware decode: {}", args.hw_decode);

    // Initialize GStreamer
    gst::init()?;

    // Create pipeline
    let pipeline = create_ultra_low_latency_pipeline(&args)?;

    info!("▶️  Starting playback...");
    info!("⚡ Target latency: < 80ms");
    info!("Press Ctrl+C to stop");

    // Start pipeline
    pipeline.set_state(gst::State::Playing)?;

    // Wait for EOS or error
    let bus = pipeline.bus().expect("Pipeline has no bus");
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => {
                info!("End of stream");
                break;
            }
            MessageView::Error(err) => {
                error!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            MessageView::StateChanged(state_changed) => {
                if state_changed
                    .src()
                    .map(|s| s == &pipeline)
                    .unwrap_or(false)
                {
                    info!(
                        "Pipeline state: {:?} -> {:?}",
                        state_changed.old(),
                        state_changed.current()
                    );
                }
            }
            _ => (),
        }
    }

    // Cleanup
    pipeline.set_state(gst::State::Null)?;
    info!("👋 Stopped");

    Ok(())
}

fn create_ultra_low_latency_pipeline(args: &Args) -> Result<gst::Pipeline> {
    let pipeline = gst::Pipeline::new();

    // RTSP Source - BALANCED LATENCY/QUALITY
    // 100ms buffer prevents frame drops during motion while keeping latency low
    let rtspsrc = gst::ElementFactory::make("rtspsrc")
        .name("source")
        .property("location", &args.url)
        .property("latency", 100u32)         // 100ms buffer for quality
        .property("drop-on-latency", false)  // Don't drop frames - preserves motion
        .build()?;

    // Auth
    if let (Some(user), Some(pass)) = (&args.username, &args.password) {
        rtspsrc.set_property("user-id", user);
        rtspsrc.set_property("user-pw", pass);
    }

    // RTP Depay
    let depay = gst::ElementFactory::make("rtph264depay")
        .name("depay")
        .build()?;

    // H264 Parse
    let parse = gst::ElementFactory::make("h264parse")
        .name("parse")
        .build()?;

    // Decoder - Try hardware first with QUALITY settings
    let decode = if args.hw_decode {
        // Try NVIDIA first
        if let Ok(nvdec) = gst::ElementFactory::make("nvh264dec").build() {
            info!("✅ Using NVIDIA hardware decode (NVDEC)");
            nvdec
        }
        // Try Intel QuickSync
        else if let Ok(qsvdec) = gst::ElementFactory::make("msdkh264dec").build() {
            info!("✅ Using Intel QuickSync hardware decode");
            qsvdec
        }
        // Try VA-API
        else if let Ok(vaapi) = gst::ElementFactory::make("vaapih264dec").build() {
            info!("✅ Using VA-API hardware decode");
            vaapi
        }
        // Fallback to software
        else {
            info!("⚠️  No hardware decoder found, using software");
            gst::ElementFactory::make("avdec_h264").build()?
        }
    } else {
        info!("Using software decode (avdec_h264)");
        gst::ElementFactory::make("avdec_h264").build()?
    };

    // Video convert - simple conversion preserving quality
    let convert = gst::ElementFactory::make("videoconvert")
        .name("convert")
        .build()?;

    // Video balance - adjust to match Digifort-style colors
    let balance = gst::ElementFactory::make("videobalance")
        .name("balance")
        .property("brightness", -0.02f64)   // -2% brightness (10% brighter)
        .property("saturation", 1.10f64)    // Increase saturation 10% (richer colors)
        .property("contrast", 1.05f64)      // Slight contrast boost 5%
        .build()?;

    // Video scale - uses default high-quality algorithm
    let scale = gst::ElementFactory::make("videoscale")
        .name("scale")
        .build()?;

    // Caps filter - use specified resolution or native
    let capsfilter = gst::ElementFactory::make("capsfilter")
        .name("capsfilter")
        .build()?;

    // Only apply caps if non-zero dimensions specified
    if args.width > 0 && args.height > 0 {
        let caps = gst::Caps::builder("video/x-raw")
            .field("width", args.width as i32)
            .field("height", args.height as i32)
            .build();
        capsfilter.set_property("caps", &caps);
        info!("📐 Output resolution: {}x{}", args.width, args.height);
    } else {
        info!("📐 Using native camera resolution");
    }

    // Video sink - D3D12 for best Windows quality, sync=true for quality
    let videosink = gst::ElementFactory::make("d3d12videosink")
        .name("videosink")
        .property("sync", true)  // Enable sync for smooth playback
        .property("fullscreen", args.fullscreen)
        .build()
        .unwrap_or_else(|_| {
            info!("D3D12 sink not available, using autovideosink");
            gst::ElementFactory::make("autovideosink")
                .name("videosink")
                .property("sync", true)
                .build()
                .expect("Failed to create video sink")
        });

    // AUDIO PIPELINE - with buffering to prevent dropouts
    let audio_decodebin = gst::ElementFactory::make("decodebin")
        .name("audio_decodebin")
        .build()?;
    
    // Audio queue - CRITICAL for preventing dropouts
    // 500ms buffer absorbs network jitter without adding noticeable latency
    let audio_queue = gst::ElementFactory::make("queue")
        .name("audio_queue")
        .property("max-size-time", 500_000_000u64)  // 500ms buffer
        .property("max-size-buffers", 0u32)         // Unlimited buffers
        .property("max-size-bytes", 0u32)           // Unlimited bytes
        .build()?;
    
    let audio_convert = gst::ElementFactory::make("audioconvert")
        .name("audio_convert")
        .build()?;
    
    let audio_resample = gst::ElementFactory::make("audioresample")
        .name("audio_resample")
        .property("quality", 10i32)  // Highest quality resampling
        .build()?;
    
    // Audio sink with buffer
    let audio_sink = gst::ElementFactory::make("autoaudiosink")
        .name("audiosink")
        .property("sync", true)
        .property("buffer-time", 100_000i64)  // 100ms output buffer
        .property("latency-time", 20_000i64)  // 20ms latency
        .build()?;

    // Add video pipeline
    pipeline.add_many(&[
        &depay, &parse, &decode, &convert, &balance, &scale, &capsfilter, &videosink
    ])?;

    // Add audio pipeline with queue buffer
    pipeline.add_many(&[
        &audio_decodebin, &audio_queue, &audio_convert, &audio_resample, &audio_sink
    ])?;

    // Link video pipeline
    gst::Element::link_many(&[
        &depay, &parse, &decode, &convert, &balance, &scale, &capsfilter, &videosink
    ])?;

    // Link audio output chain: queue -> convert -> resample -> sink
    gst::Element::link_many(&[
        &audio_queue, &audio_convert, &audio_resample, &audio_sink
    ])?;

    // Connect decodebin pad-added for audio -> connect to queue for buffering
    let audio_queue_clone = audio_queue.clone();
    audio_decodebin.connect_pad_added(move |_element, pad| {
        let sink_pad = audio_queue_clone
            .static_pad("sink")
            .expect("audio queue has no sink pad");
        
        if sink_pad.is_linked() {
            return;
        }
        
        if let Err(e) = pad.link(&sink_pad) {
            error!("Failed to link audio decodebin: {}", e);
        } else {
            info!("🔊 Audio pipeline linked with buffer");
        }
    });

    // Connect RTSP source dynamic pads
    let depay_clone = depay.clone();
    let audio_decodebin_clone = audio_decodebin.clone();
    rtspsrc.connect_pad_added(move |_src, src_pad| {
        if let Some(caps) = src_pad.current_caps() {
            if let Some(structure) = caps.structure(0) {
                if structure.name().starts_with("application/x-rtp") {
                    if let Ok(media) = structure.get::<&str>("media") {
                        match media {
                            "video" => {
                                let sink_pad = depay_clone
                                    .static_pad("sink")
                                    .expect("Failed to get video sink pad");
                                if !sink_pad.is_linked() {
                                    if let Err(e) = src_pad.link(&sink_pad) {
                                        error!("Failed to link video: {}", e);
                                    } else {
                                        info!("✅ Video stream linked");
                                    }
                                }
                            }
                            "audio" => {
                                let sink_pad = audio_decodebin_clone
                                    .static_pad("sink")
                                    .expect("Failed to get audio sink pad");
                                if !sink_pad.is_linked() {
                                    if let Err(e) = src_pad.link(&sink_pad) {
                                        error!("Failed to link audio: {}", e);
                                    } else {
                                        info!("🔊 Audio stream linked");
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    });

    pipeline.add(&rtspsrc)?;

    Ok(pipeline)
}
