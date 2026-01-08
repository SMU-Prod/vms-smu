//! Camera routes

use axum::{
    extract::{State, Path},
    routing::{get, post},
    Router, Json,
    http::StatusCode,
};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vms_core::{Camera, CreateCameraRequest, UpdateCameraRequest, StartLiveRequest, StartLiveResponse, StreamProfile};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_cameras).post(create_camera))
        .route("/:id", get(get_camera).put(update_camera).delete(delete_camera))
        .route("/:id/live", post(start_live))
        .route("/:id/stream-url", get(get_stream_url))
}

/// Error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// List all cameras
async fn list_cameras(
    State(state): State<AppState>,
) -> Result<Json<Vec<Camera>>, (StatusCode, Json<ErrorResponse>)> {
    let cameras = state.camera_repo
        .list()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    Ok(Json(cameras))
}

/// Create camera
async fn create_camera(
    State(state): State<AppState>,
    Json(req): Json<CreateCameraRequest>,
) -> Result<Json<Camera>, (StatusCode, Json<ErrorResponse>)> {
    let camera = state.camera_repo
        .create(req.node_id, &req)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    Ok(Json(camera))
}

/// Get single camera
async fn get_camera(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Camera>, (StatusCode, Json<ErrorResponse>)> {
    let camera = state.camera_repo
        .find_by_id(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "Camera not found".to_string() })))?;

    Ok(Json(camera))
}

/// Update camera
async fn update_camera(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCameraRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    state.camera_repo
        .update(id, &req)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    Ok(StatusCode::NO_CONTENT)
}

/// Delete camera
async fn delete_camera(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    state.camera_repo
        .delete(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    Ok(StatusCode::NO_CONTENT)
}

/// Start live request
#[derive(Debug, Deserialize)]
pub struct StartLiveReq {
    pub profile: Option<StreamProfile>,
}

/// Start live session
async fn start_live(
    State(state): State<AppState>,
    Path(camera_id): Path<Uuid>,
    Json(req): Json<StartLiveReq>,
) -> Result<Json<StartLiveResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Get camera
    let camera = state.camera_repo
        .find_by_id(camera_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "Camera not found".to_string() })))?;

    // Get node
    let node = state.node_repo
        .find_by_id(camera.node_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "Node not found".to_string() })))?;

    // Create session (use a default user_id for now - TODO: get from auth)
    let profile = req.profile.unwrap_or_default();
    let session = state.session_repo
        .create(Uuid::nil(), camera_id, camera.node_id, profile.clone(), 30) // 30 min TTL
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    // Build stream URL (direct to node)
    let stream_url = format!("http://{}:{}/live/{}/index.m3u8", node.ip, node.port, session.id);

    // Activate session with URL
    state.session_repo
        .activate(session.id, stream_url.clone())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    // TODO: Send START_LIVE command to node via WebSocket/gRPC

    tracing::info!("Started live session {} for camera {}", session.id, camera_id);

    Ok(Json(StartLiveResponse {
        session_id: session.id,
        stream_url,
        expires_at: session.expires_at,
    }))
}

/// Response for stream URL
#[derive(Debug, Serialize)]
pub struct StreamUrlResponse {
    pub rtsp_url: String,
    pub camera_name: String,
    pub resolution: (u32, u32),
}

/// Get authenticated RTSP URL for native GStreamer playback
async fn get_stream_url(
    State(state): State<AppState>,
    Path(camera_id): Path<Uuid>,
) -> Result<Json<StreamUrlResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Get camera
    let camera = state.camera_repo
        .find_by_id(camera_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "Camera not found".to_string() })))?;

    // Get credentials
    let (username, password) = state.camera_repo
        .get_credentials(camera_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "Credentials not found".to_string() })))?;

    // URL-encode credentials to handle special characters
    let username_encoded = utf8_percent_encode(&username, NON_ALPHANUMERIC).to_string();
    let password_encoded = utf8_percent_encode(&password, NON_ALPHANUMERIC).to_string();

    // Build authenticated RTSP URL
    let rtsp_url = camera.rtsp_url.replace("rtsp://", &format!("rtsp://{}:{}@", username_encoded, password_encoded));

    tracing::info!("Returning stream URL for camera {}", camera_id);

    Ok(Json(StreamUrlResponse {
        rtsp_url,
        camera_name: camera.name,
        resolution: (1920, 1080), // TODO: get from camera config
    }))
}
