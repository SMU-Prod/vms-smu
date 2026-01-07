//! VMS Stream Service - GStreamer WebRTC Edition
//! Ultra-low latency video streaming via WebRTC

use anyhow::{Context, Result};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

mod gstreamer_webrtc;

use gstreamer_webrtc::GstWebRTCSession;

/// Application state
struct AppState {
    sessions: RwLock<HashMap<String, GstWebRTCSession>>,
}

#[derive(Debug, Deserialize)]
struct WebRTCOfferRequest {
    camera_id: String,
    rtsp_url: String,
    username: String,
    password: String,
    sdp: String,
    #[serde(rename = "type")]
    sdp_type: String,
}

#[derive(Debug, Serialize)]
struct WebRTCOfferResponse {
    sdp: String,
    #[serde(rename = "type")]
    sdp_type: String,
}

#[derive(Debug, Deserialize)]
struct ICECandidateRequest {
    camera_id: String,
    candidate: String,
    #[serde(rename = "sdpMid")]
    sdp_mid: Option<String>,
    #[serde(rename = "sdpMLineIndex")]
    sdp_mline_index: Option<u16>,
}

/// Handle WebRTC offer from browser
async fn webrtc_offer_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<WebRTCOfferRequest>,
) -> Result<Json<WebRTCOfferResponse>, StatusCode> {
    info!("📡 WebRTC offer received for camera: {}", req.camera_id);
    
    // Check if session already exists
    {
        let sessions = state.sessions.read().await;
        if let Some(existing_session) = sessions.get(&req.camera_id) {
            info!("♻️ Session exists for camera: {}, using existing pipeline", req.camera_id);
            
            // Use existing session to process the offer
            let answer = existing_session.handle_offer(req.sdp).map_err(|e| {
                error!("Failed to handle SDP offer with existing session: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            
            return Ok(Json(WebRTCOfferResponse {
                sdp: answer,
                sdp_type: "answer".to_string(),
            }));
        }
    }
    
    // Take write lock for new session creation
    let mut sessions = state.sessions.write().await;
    // Double-check (another request might have won the race)
    if let Some(existing_session) = sessions.get(&req.camera_id) {
        info!("♻️ Session created by another request, using it");
        let answer = existing_session.handle_offer(req.sdp).map_err(|e| {
            error!("Failed to handle SDP offer: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        return Ok(Json(WebRTCOfferResponse {
            sdp: answer,
            sdp_type: "answer".to_string(),
        }));
    }
    
    info!("📹 Creating session for RTSP URL: {} (holding lock)", req.rtsp_url);
    
    // Create GStreamer session while holding write lock
    let session = GstWebRTCSession::new(
        req.camera_id.clone(),
        &req.rtsp_url,
        &req.username,
        &req.password,
    ).map_err(|e| {
        error!("Failed to create GStreamer session: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    // Insert session immediately to prevent other requests from creating
    sessions.insert(req.camera_id.clone(), session);
    
    // Get mutable reference to the session we just inserted
    let session = sessions.get(&req.camera_id).unwrap();
    
    // Start pipeline FIRST - webrtcbin needs PLAYING state for negotiation
    session.start().map_err(|e| {
        error!("Failed to start pipeline: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    // Drop write lock before sleeping to not block other cameras
    drop(sessions);
    
    // Wait for RTSP to connect - GLib main loop thread handles event processing
    info!("⏳ Waiting 3s for RTSP connection...");
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    
    // Re-acquire read lock to get session
    let sessions = state.sessions.read().await;
    let session = sessions.get(&req.camera_id).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Handle SDP offer and get answer
    let answer = session.handle_offer(req.sdp).map_err(|e| {
        error!("Failed to handle SDP offer: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    info!("✅ WebRTC session ready for camera: {}", req.camera_id);
    
    Ok(Json(WebRTCOfferResponse {
        sdp: answer,
        sdp_type: "answer".to_string(),
    }))
}

/// Handle ICE candidate from browser
async fn webrtc_ice_handler(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<ICECandidateRequest>,
) -> StatusCode {
    info!("🧊 ICE candidate received for camera: {}", req.camera_id);
    // ICE candidates are handled by GStreamer automatically via trickle ICE
    StatusCode::OK
}

/// Close WebRTC session
async fn webrtc_close_handler(
    State(state): State<Arc<AppState>>,
    Path(camera_id): Path<String>,
) -> StatusCode {
    info!("🔴 Closing WebRTC session for camera: {}", camera_id);
    
    if let Some(session) = state.sessions.write().await.remove(&camera_id) {
        let _ = session.stop();
    }
    
    StatusCode::NO_CONTENT
}

/// Health check
async fn health() -> &'static str {
    "OK"
}

/// Metrics
async fn metrics(State(state): State<Arc<AppState>>) -> String {
    let sessions = state.sessions.read().await.len();
    format!(
        "# VMS Stream Metrics\nvms_webrtc_sessions {}\n",
        sessions
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🚀 VMS Stream Service (GStreamer Edition) starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Initialize GStreamer
    gstreamer_webrtc::init().context("Failed to initialize GStreamer")?;

    let state = Arc::new(AppState {
        sessions: RwLock::new(HashMap::new()),
    });

    // CORS for web client
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/metrics", get(metrics))
        .route("/api/v1/webrtc/offer", post(webrtc_offer_handler))
        .route("/api/v1/webrtc/ice", post(webrtc_ice_handler))
        .route("/api/v1/webrtc/:camera_id", axum::routing::delete(webrtc_close_handler))
        .layer(cors)
        .with_state(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 9094));
    let listener = TcpListener::bind(addr).await?;

    info!("🌐 HTTP API listening on http://{}", addr);
    info!("📡 WebRTC signaling ready at /api/v1/webrtc/offer");
    info!("⚡ Ultra-low latency mode enabled (GStreamer webrtcbin)");
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
