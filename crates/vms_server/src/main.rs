//! VMS Server - Control Plane API
//!
//! Central server that handles:
//! - Authentication (JWT + refresh tokens)
//! - User/Role/Permission management
//! - Node registration and health monitoring
//! - Camera inventory and configuration
//! - Session orchestration (START/STOP live)
//! - Signed URL generation for direct node access

use std::net::SocketAddr;
use axum::{Router, routing::get};
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod db;
mod routes;
mod services;
mod middleware;
mod state;
mod webrtc;

use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("VMS Server v{} starting...", env!("CARGO_PKG_VERSION"));

    // Load config
    let config = config::Config::from_env();

    // Initialize state
    let state = AppState::new(&config.database_url, config.jwt_secret).await?;

    tracing::info!("Database connected and migrated");

    // CORS layer - allow all origins in development
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/api/v1", routes::api_router())
        .layer(cors)
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
