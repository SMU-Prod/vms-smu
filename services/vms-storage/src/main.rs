//! VMS Storage Service - DVR/Recording System
//! 
//! Provides 24/7 continuous recording with:
//! - H264 passthrough (no re-encode)
//! - Hourly segmentation
//! - Fast seek indexing
//! - Playback API
//! - Export functionality
//! - Retention policies

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::{info, error};

mod recorder;
mod indexer;
mod playback;
mod export;
mod storage;
mod retention;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🎬 VMS Storage Service starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Initialize GStreamer
    gstreamer::init()?;
    info!("✅ GStreamer initialized");

    // Connect to NATS
    let nats_url = std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    let nats_client = async_nats::connect(&nats_url).await?;
    info!("✅ Connected to NATS at {}", nats_url);

    // Storage configuration
    let storage_path = std::env::var("STORAGE_PATH")
        .unwrap_or_else(|_| "C:\\storage\\cameras".to_string());
    info!("📁 Storage path: {}", storage_path);

    // Create storage directory
    tokio::fs::create_dir_all(&storage_path).await?;

    // TODO: Start recorder for each camera
    // For now, we'll implement the basic structure

    // Build HTTP API
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(metrics_handler))
        .route("/api/v1/playback/:camera_id", get(playback::stream_handler))
        .route("/api/v1/playback/:camera_id/timeline", get(playback::timeline_handler))
        .route("/api/v1/export", post(export::create_export_job))
        .layer(TraceLayer::new_for_http());

    // Start HTTP server
    let addr = SocketAddr::from(([0, 0, 0, 0], 9092));
    info!("🌐 HTTP API listening on http://{}", addr);
    info!("📊 Metrics: http://{}/metrics", addr);
    info!("✅ Service initialized successfully");
    info!("Press Ctrl+C to stop");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}

async fn metrics_handler() -> String {
    // TODO: Implement Prometheus metrics
    "# VMS Storage Metrics\n\
     vms_storage_recordings 0\n\
     vms_storage_disk_usage_bytes 0\n".to_string()
}
