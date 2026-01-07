//! Rotas de câmeras

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use vms_common::camera::{CameraInfo, CameraStatus};
use vms_common::types::{CameraId, Resolution};

type CameraStore = Arc<RwLock<Vec<CameraInfo>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCameraRequest {
    pub name: String,
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub resolution: Option<Resolution>,
    pub fps: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct CameraResponse {
    pub id: String,
    pub name: String,
    pub status: CameraStatus,
    pub current_fps: f64,
    pub current_bitrate: u64,
}

/// Lista todas as câmeras
pub async fn list_cameras(
    State(store): State<CameraStore>,
) -> Json<Vec<CameraResponse>> {
    let cameras = store.read().await;
    let response: Vec<CameraResponse> = cameras
        .iter()
        .map(|c| CameraResponse {
            id: c.id.to_string(),
            name: c.name.clone(),
            status: c.status,
            current_fps: c.current_fps,
            current_bitrate: c.current_bitrate,
        })
        .collect();

    Json(response)
}

/// Obtém uma câmera específica
pub async fn get_camera(
    State(store): State<CameraStore>,
    Path(camera_id): Path<String>,
) -> Result<Json<CameraResponse>, StatusCode> {
    let cameras = store.read().await;

    let camera = cameras
        .iter()
        .find(|c| c.id.to_string() == camera_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(CameraResponse {
        id: camera.id.to_string(),
        name: camera.name.clone(),
        status: camera.status,
        current_fps: camera.current_fps,
        current_bitrate: camera.current_bitrate,
    }))
}

/// Cria uma nova câmera
pub async fn create_camera(
    State(store): State<CameraStore>,
    Json(req): Json<CreateCameraRequest>,
) -> Result<(StatusCode, Json<CameraResponse>), StatusCode> {
    let camera_id = CameraId::new();

    let camera_info = CameraInfo {
        id: camera_id,
        name: req.name.clone(),
        status: CameraStatus::Offline,
        current_fps: 0.0,
        current_bitrate: 0,
    };

    let mut cameras = store.write().await;
    cameras.push(camera_info.clone());

    Ok((
        StatusCode::CREATED,
        Json(CameraResponse {
            id: camera_id.to_string(),
            name: camera_info.name,
            status: camera_info.status,
            current_fps: camera_info.current_fps,
            current_bitrate: camera_info.current_bitrate,
        }),
    ))
}

/// Deleta uma câmera
pub async fn delete_camera(
    State(store): State<CameraStore>,
    Path(camera_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let mut cameras = store.write().await;

    let index = cameras
        .iter()
        .position(|c| c.id.to_string() == camera_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    cameras.remove(index);

    Ok(StatusCode::NO_CONTENT)
}
