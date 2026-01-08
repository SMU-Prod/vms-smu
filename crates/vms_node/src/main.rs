//! VMS Node - Data Plane Agent
//!
//! Node agent that runs close to cameras and executes commands from the server:
//! - RTSP/ONVIF camera connections
//! - GStreamer pipelines for transcoding
//! - HLS/WebRTC stream delivery
//! - Recording to disk
//!
//! IMPORTANT: Node does NOT decide anything. It only executes orders from vms_server.

use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Router, routing::get};
use tokio::sync::RwLock;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod pipeline;
mod server_client;
mod routes;

use config::NodeConfig;
use server_client::ServerClient;
use pipeline::PipelineManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("VMS Node v{} starting...", env!("CARGO_PKG_VERSION"));

    // Load config
    let config = NodeConfig::from_env();

    // Create server client
    let mut server_client = ServerClient::new(config.server_url.clone());

    // Register with server
    let node_id = server_client.register(
        config.node_name.clone(),
        get_local_ip(),
        config.media_port,
    ).await?;

    tracing::info!("Node registered with ID: {}", node_id);

    // Create pipeline manager
    let pipeline_manager = Arc::new(PipelineManager::new());

    // Start heartbeat task
    let client_arc = Arc::new(RwLock::new(server_client));
    let heartbeat_client = client_arc.clone();
    tokio::spawn(async move {
        ServerClient::start_heartbeat_loop(heartbeat_client, 30).await;
    });

    // Build router for media delivery
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/live", routes::live_router())
        .nest("/snapshot", routes::snapshot_router());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.media_port));
    tracing::info!("Node media server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Get local IP address
fn get_local_ip() -> String {
    // Try to get local IP, fallback to localhost
    local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string())
}
