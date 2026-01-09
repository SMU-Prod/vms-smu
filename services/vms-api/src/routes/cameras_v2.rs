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
    // Get existing camera
    let existing = match state.camera_repo.get(id).await {
        Ok(Some(cam)) => cam,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "Camera not found" })),
            )
                .into_response()
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response()
        }
    };

    // Apply updates - compute derived values first
    let new_ip = req.ip_address.clone().unwrap_or(existing.ip_address.clone());
    let new_port = req.rtsp_port.unwrap_or(existing.rtsp_port);
    let new_rtsp_url = format!("rtsp://{}:{}/stream1", new_ip, new_port);

    let updated = Camera {
        name: req.name.unwrap_or(existing.name),
        description: req.description.or(existing.description),
        manufacturer: req.manufacturer.or(existing.manufacturer),
        model: req.model.or(existing.model),
        firmware: req.firmware.or(existing.firmware),
        enabled: req.enabled.unwrap_or(existing.enabled),
        ip_address: new_ip,
        rtsp_port: new_port,
        onvif_port: req.onvif_port.unwrap_or(existing.onvif_port),
        username: req.username.unwrap_or(existing.username),
        password: req.password.unwrap_or(existing.password),
        transport: req.transport.unwrap_or(existing.transport),
        use_ssl: req.use_ssl.unwrap_or(existing.use_ssl),
        timeout_ms: req.timeout_ms.unwrap_or(existing.timeout_ms),
        recording_mode: req.recording_mode.unwrap_or(existing.recording_mode),
        recording_dir: req.recording_dir.unwrap_or(existing.recording_dir),
        retention_days: req.retention_days.unwrap_or(existing.retention_days),
        shortcut: req.shortcut.unwrap_or(existing.shortcut),
        latitude: req.latitude.unwrap_or(existing.latitude),
        longitude: req.longitude.unwrap_or(existing.longitude),
        server_id: req.server_id.unwrap_or(existing.server_id),
        rtsp_url: new_rtsp_url,
        onvif_url: existing.onvif_url,
        ..existing
    };

    match state.camera_repo.update(id, &updated).await {
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
