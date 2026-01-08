//! Media delivery routes

use axum::{Router, routing::get};

pub fn live_router() -> Router {
    Router::new()
        .route("/:session_id/index.m3u8", get(get_playlist))
        .route("/:session_id/:segment", get(get_segment))
}

pub fn snapshot_router() -> Router {
    Router::new()
        .route("/:camera_id", get(get_snapshot))
}

async fn get_playlist() -> &'static str {
    // TODO: Return HLS playlist for session
    "#EXTM3U\n#EXT-X-VERSION:3\n"
}

async fn get_segment() -> &'static str {
    // TODO: Return HLS segment
    ""
}

async fn get_snapshot() -> &'static str {
    // TODO: Return JPEG snapshot from camera
    ""
}
