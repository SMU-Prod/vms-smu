//! Session routes

use axum::{extract::State, routing::get, Router, Json};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_sessions))
        .route("/:id", get(get_session).delete(stop_session))
}

async fn list_sessions(State(_state): State<AppState>) -> Json<serde_json::Value> {
    // TODO: List active sessions
    Json(serde_json::json!({ "sessions": [] }))
}

async fn get_session(State(_state): State<AppState>) -> Json<serde_json::Value> {
    // TODO: Get session details
    Json(serde_json::json!({ "message": "Get session - TODO" }))
}

async fn stop_session(State(_state): State<AppState>) -> Json<serde_json::Value> {
    // TODO: Stop session
    Json(serde_json::json!({ "message": "Stop session - TODO" }))
}
