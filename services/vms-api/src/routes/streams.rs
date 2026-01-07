//! Rotas de streams

use axum::{extract::Path, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamRequest {
    pub camera_id: String,
    pub protocol: String, // "webrtc" ou "srt"
}

#[derive(Debug, Serialize)]
pub struct StreamResponse {
    pub stream_id: String,
    pub url: String,
    pub protocol: String,
}

/// Inicia um stream
pub async fn start_stream(
    Json(req): Json<StreamRequest>,
) -> Result<(StatusCode, Json<StreamResponse>), StatusCode> {
    // Em produção, isso iniciaria um stream real
    let stream_id = uuid::Uuid::new_v4().to_string();
    let url = match req.protocol.as_str() {
        "webrtc" => format!("webrtc://localhost:8443/stream/{}", stream_id),
        "srt" => format!("srt://localhost:9000/stream/{}", stream_id),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    Ok((
        StatusCode::CREATED,
        Json(StreamResponse {
            stream_id,
            url,
            protocol: req.protocol,
        }),
    ))
}

/// Para um stream
pub async fn stop_stream(
    Path(_stream_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // Em produção, isso pararia o stream
    Ok(StatusCode::NO_CONTENT)
}
