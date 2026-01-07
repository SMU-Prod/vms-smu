//! Rotas de gravações

use axum::{extract::Path, http::StatusCode, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RecordingInfo {
    pub id: String,
    pub camera_id: String,
    pub start_time: String,
    pub end_time: String,
    pub size_bytes: u64,
    pub download_url: String,
}

/// Lista gravações de uma câmera
pub async fn list_recordings(
    Path(_camera_id): Path<String>,
) -> Json<Vec<RecordingInfo>> {
    // Em produção, isso buscaria do storage real
    Json(vec![])
}

/// Baixa uma gravação
pub async fn download_recording(
    Path((camera_id, recording_id)): Path<(String, String)>,
) -> Result<String, StatusCode> {
    // Em produção, isso geraria URL assinada
    Ok(format!("https://storage.vms.local/recordings/{}/{}.mkv", camera_id, recording_id))
}
