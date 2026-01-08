//! API Routes

pub mod cameras;
pub mod cameras_v2;
pub mod streams;
pub mod recordings;
pub mod mjpeg;
pub mod auth;
pub mod webrtc;
// pub mod onvif; // Temporarily disabled

use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

pub async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}
