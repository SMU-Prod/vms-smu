//! Camera API routes v2 (with SQLite)

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    models::camera::{Camera, CreateCameraRequest, UpdateCameraRequest},
    AppState,
};

/// POST /api/v1/cameras - Create new camera
pub async fn create_camera(
    State(state): State<AppState>,
    Json(req): Json<CreateCameraRequest>,
) -> impl IntoResponse {
    let camera = Camera::from_request(req);
    
    match state.camera_repo.create(&camera).await {
        Ok(_) => (StatusCode::CREATED, Json(camera)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// GET /api/v1/cameras - List all cameras
pub async fn list_cameras(State(state): State<AppState>) -> impl IntoResponse {
    match state.camera_repo.list().await {
        Ok(cameras) => (StatusCode::OK, Json(cameras)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// GET /api/v1/cameras/:id - Get camera by ID
pub async fn get_camera(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.camera_repo.get(id).await {
        Ok(Some(camera)) => (StatusCode::OK, Json(camera)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Camera not found" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// PUT /api/v1/cameras/:id - Update camera
pub async fn update_camera(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCameraRequest>,
) -> impl IntoResponse {
    match state.camera_repo.update(id, &req).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// DELETE /api/v1/cameras/:id - Delete camera
pub async fn delete_camera(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.camera_repo.delete(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
