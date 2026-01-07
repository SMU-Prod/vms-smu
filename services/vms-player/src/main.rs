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

    // RTSP Source - LOW LATENCY
    let rtspsrc = gst::ElementFactory::make("rtspsrc")
        .name("source")
        .property("location", &args.url)
        .property("latency", 0u32)
        .property("drop-on-latency", true)
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

    // Video convert - HIGH QUALITY
    let convert = gst::ElementFactory::make("videoconvert")
        .name("convert")
        .build()?;

    // Deinterlace - remove interlacing artifacts
    let deinterlace = gst::ElementFactory::make("deinterlace")
        .name("deinterlace")
        .build()?;

    // Video balance - adjust brightness/contrast
    let balance = gst::ElementFactory::make("videobalance")
        .name("balance")
        .property("brightness", -0.05f64)  // Slightly reduce brightness
        .property("contrast", 1.1f64)      // Increase contrast
        .build()?;

    // Gamma correction
    let gamma = gst::ElementFactory::make("gamma")
        .name("gamma")
        .property("gamma", 0.9f64)  // Slightly darker for better detail
        .build()?;

    // Video scale - preserve quality
    let scale = gst::ElementFactory::make("videoscale")
        .name("scale")
        .build()?;

    // Caps filter - force output resolution
    let capsfilter = gst::ElementFactory::make("capsfilter")
        .name("capsfilter")
        .build()?;

    let caps = gst::Caps::builder("video/x-raw")
        .field("width", args.width as i32)
        .field("height", args.height as i32)
        .build();
    capsfilter.set_property("caps", &caps);

    // Video sink - OPTIMIZED for quality
    let videosink = if args.fullscreen {
        gst::ElementFactory::make("autovideosink")
            .name("sink")
            .property("sync", false)
            .property("fullscreen", true)
            .build()?
    } else {
        gst::ElementFactory::make("autovideosink")
            .name("sink")
            .property("sync", false)
            .build()?
    };

    // Add ALL elements including quality filters
    pipeline.add_many(&[
        &depay, &parse, &decode, &convert, &deinterlace, 
        &balance, &gamma, &scale, &capsfilter, &videosink
    ])?;

    // Link pipeline with ALL quality enhancements
    gst::Element::link_many(&[
        &depay, &parse, &decode, &convert, &deinterlace,
        &balance, &gamma, &scale, &capsfilter, &videosink
    ])?;

    // Connect dynamic pads
    let depay_clone = depay.clone();
    rtspsrc.connect_pad_added(move |_src, src_pad| {
        let sink_pad = depay_clone
            .static_pad("sink")
            .expect("Failed to get sink pad");

        if sink_pad.is_linked() {
            return;
        }

        if let Some(caps) = src_pad.current_caps() {
            if let Some(structure) = caps.structure(0) {
                if structure.name().starts_with("application/x-rtp") {
                    if let Ok(media) = structure.get::<&str>("media") {
                        if media == "video" {
                            if let Err(e) = src_pad.link(&sink_pad) {
                                error!("Failed to link: {}", e);
                            } else {
                                info!("✅ Video stream linked");
                            }
                        }
                    }
                }
            }
        }
    });

    pipeline.add(&rtspsrc)?;

    // Set pipeline latency to minimum
    pipeline.set_latency(gst::ClockTime::from_mseconds(0));

    Ok(pipeline)
}
