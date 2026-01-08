//! API Routes

use axum::Router;
use crate::state::AppState;

pub mod auth;
pub mod cameras;
pub mod nodes;
pub mod sessions;
pub mod mjpeg;
pub mod webrtc;

pub fn api_router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/cameras", cameras::router())
        .nest("/nodes", nodes::router())
        .nest("/sessions", sessions::router())
        .nest("/mjpeg", mjpeg::router())
        .nest("/webrtc", webrtc::router())
}
