//! VMS Stream Service
//! Distribuição WebRTC/SRT para clientes

use anyhow::{Context, Result};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber;

mod nats_consumer;
mod srt_server;
mod webrtc_server;

use nats_consumer::StreamDistributor;
use srt_server::SRTServer;
use webrtc_server::WebRTCServer;

type AppState = Arc<ServerState>;

struct ServerState {
    distributor: Arc<StreamDistributor>,
    webrtc: WebRTCServer,
    srt: SRTServer,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateStreamRequest {
    camera_id: String,
    viewer_id: String,
}

#[derive(Debug, Serialize)]
struct CreateStreamResponse {
    stream_id: String,
    webrtc_url: String,
    srt_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SDPOfferRequest {
    sdp: String,
}

#[derive(Debug, Serialize)]
struct SDPAnswerResponse {
    sdp: String,
}

async fn create_stream(
    State(state): State<AppState>,
    Json(req): Json<CreateStreamRequest>,
) -> Result<Json<CreateStreamResponse>, StatusCode> {
    let stream_id = state
        .webrtc
        .create_session(req.camera_id.clone(), req.viewer_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateStreamResponse {
        stream_id: stream_id.to_string(),
        webrtc_url: format!("webrtc://localhost:8443/stream/{}", stream_id),
        srt_url: format!("srt://localhost:9000/stream/{}", stream_id),
    }))
}

async fn handle_sdp_offer(
    State(state): State<AppState>,
    Path(stream_id): Path<String>,
    Json(req): Json<SDPOfferRequest>,
) -> Result<Json<SDPAnswerResponse>, StatusCode> {
    let stream_id = stream_id
        .parse()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let answer = state
        .webrtc
        .handle_offer(stream_id, req.sdp)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(SDPAnswerResponse { sdp: answer }))
}

async fn close_stream(
    State(state): State<AppState>,
    Path(stream_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let stream_id = stream_id
        .parse()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    state
        .webrtc
        .close_session(stream_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn metrics(State(state): State<AppState>) -> String {
    let webrtc_sessions = state.webrtc.active_sessions().await;
    let srt_streams = state.srt.active_streams().await;
    let (cameras, streams, frames) = state.distributor.get_stats().await;

    format!(
        "# Stream metrics\n\
         vms_webrtc_sessions {}\n\
         vms_srt_streams {}\n\
         vms_distributor_cameras {}\n\
         vms_distributor_streams {}\n\
         vms_distributor_frames_total {}\n",
        webrtc_sessions, srt_streams, cameras, streams, frames
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🚀 VMS Stream Service starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Conectar ao NATS e iniciar distribuidor
    let nats_url = std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    let distributor = Arc::new(
        StreamDistributor::connect(&nats_url)
            .await
            .context("Failed to connect to NATS")?
    );

    distributor
        .start_distributing()
        .await
        .context("Failed to start distributing frames")?;

    info!("📡 Frame distributor started: {}", nats_url);

    let webrtc = WebRTCServer::new(distributor.clone());
    let srt = SRTServer::new(9000);

    // Iniciar servidor SRT
    srt.start().await?;

    let state = Arc::new(ServerState {
        distributor: distributor.clone(),
        webrtc,
        srt,
    });

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/metrics", get(metrics))
        .route("/stream", post(create_stream))
        .route("/stream/:id/offer", post(handle_sdp_offer))
        .route("/stream/:id", axum::routing::delete(close_stream))
        .with_state(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 9094));
    let listener = TcpListener::bind(addr).await?;

    info!("🌐 HTTP API listening on http://{}", addr);
    info!("📡 WebRTC signaling ready");
    info!("📺 SRT server ready on port 9000");
    info!("✅ Service initialized successfully");
    info!("Press Ctrl+C to stop");

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
        })
        .await?;

    info!("👋 Goodbye!");
    Ok(())
}
