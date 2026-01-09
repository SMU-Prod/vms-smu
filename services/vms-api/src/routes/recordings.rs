//! Recording routes - Start/Stop/List recordings
//!
//! Endpoints for camera recording control

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::recording_manager::RecordingMode;
use crate::AppState;

/// Recording info response
#[derive(Debug, Serialize)]
pub struct RecordingInfo {
    pub id: String,
    pub camera_id: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub size_bytes: u64,
    pub download_url: String,
}

/// Recording status response
#[derive(Debug, Serialize)]
pub struct RecordingStatusResponse {
    pub camera_id: Uuid,
    pub is_recording: bool,
    pub mode: Option<String>,
    pub started_at: Option<String>,
}

/// Start recording request
#[derive(Debug, Deserialize)]
pub struct StartRecordingRequest {
    pub mode: Option<String>, // continuous, motion, manual
}

/// POST /api/v1/cameras/:id/recording/start - Start recording
pub async fn start_recording(
    State(state): State<AppState>,
    Path(camera_id): Path<Uuid>,
    Json(req): Json<StartRecordingRequest>,
) -> impl IntoResponse {
    // Get camera from DB
    let camera = match state.camera_repo.get(camera_id).await {
        Ok(Some(cam)) => cam,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "Câmera não encontrada" })),
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

    let recording_dir = camera.recording_dir.clone()
        .unwrap_or_else(|| format!("./recordings/{}", camera_id));

    let mode = match req.mode.as_deref() {
        Some("continuous") => RecordingMode::Continuous,
        Some("motion") => RecordingMode::Motion,
        _ => RecordingMode::Manual,
    };

    match state.recording_manager.start(
        camera_id,
        &camera.rtsp_url,
        &camera.username,
        &camera.password,
        &recording_dir,
        mode,
    ).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "success": true,
                "message": "Gravação iniciada",
                "camera_id": camera_id,
                "mode": format!("{:?}", mode)
            })),
        ).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": e })),
        ).into_response(),
    }
}

/// POST /api/v1/cameras/:id/recording/stop - Stop recording
pub async fn stop_recording(
    State(state): State<AppState>,
    Path(camera_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.recording_manager.stop(camera_id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "success": true,
                "message": "Gravação parada",
                "camera_id": camera_id
            })),
        ).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": e })),
        ).into_response(),
    }
}

/// GET /api/v1/cameras/:id/recording/status - Get recording status
pub async fn recording_status(
    State(state): State<AppState>,
    Path(camera_id): Path<Uuid>,
) -> impl IntoResponse {
    let is_recording = state.recording_manager.is_recording(camera_id).await;
    
    (StatusCode::OK, Json(RecordingStatusResponse {
        camera_id,
        is_recording,
        mode: if is_recording { Some("manual".to_string()) } else { None },
        started_at: None, // Could be enhanced to track this
    })).into_response()
}

/// GET /api/v1/cameras/:id/recordings - List recordings for camera
pub async fn list_recordings(
    State(state): State<AppState>,
    Path(camera_id): Path<Uuid>,
) -> impl IntoResponse {
    // Get camera for recording dir
    let camera = match state.camera_repo.get(camera_id).await {
        Ok(Some(cam)) => cam,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(Vec::<RecordingInfo>::new()),
            ).into_response()
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Vec::<RecordingInfo>::new()),
            ).into_response()
        }
    };

    let recording_dir = camera.recording_dir
        .unwrap_or_else(|| format!("./recordings/{}", camera_id));

    // List files in directory
    let mut recordings = Vec::new();
    
    if let Ok(mut dir) = tokio::fs::read_dir(&recording_dir).await {
        while let Ok(Some(entry)) = dir.next_entry().await {
            if let Ok(metadata) = entry.metadata().await {
                let filename = entry.file_name().to_string_lossy().to_string();
                if filename.ends_with(".mkv") || filename.ends_with(".mp4") {
                    recordings.push(RecordingInfo {
                        id: filename.clone(),
                        camera_id: camera_id.to_string(),
                        start_time: metadata.created()
                            .map(|t| chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339())
                            .unwrap_or_default(),
                        end_time: None,
                        size_bytes: metadata.len(),
                        download_url: format!("/api/v1/recordings/{}/{}", camera_id, filename),
                    });
                }
            }
        }
    }

    (StatusCode::OK, Json(recordings)).into_response()
}
