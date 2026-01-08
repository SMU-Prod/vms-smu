//! MJPEG streaming routes

use axum::{
    extract::{State, Path},
    routing::get,
    Router,
    http::{StatusCode, header},
    response::Response,
    body::Body,
};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use tokio::process::Command;
use uuid::Uuid;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:camera_id", get(stream_mjpeg))
}

/// Stream MJPEG from camera via FFmpeg
async fn stream_mjpeg(
    State(state): State<AppState>,
    Path(camera_id): Path<Uuid>,
) -> Result<Response<Body>, (StatusCode, String)> {
    // Get camera
    let camera = state.camera_repo
        .find_by_id(camera_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Camera not found".to_string()))?;

    // Get credentials
    let (username, password) = state.camera_repo
        .get_credentials(camera_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Credentials not found".to_string()))?;

    // URL-encode credentials to handle special characters like @
    let username_encoded = utf8_percent_encode(&username, NON_ALPHANUMERIC).to_string();
    let password_encoded = utf8_percent_encode(&password, NON_ALPHANUMERIC).to_string();

    // Build authenticated RTSP URL
    let rtsp_url = if camera.rtsp_url.contains("@") {
        camera.rtsp_url.clone()
    } else {
        camera.rtsp_url.replace("rtsp://", &format!("rtsp://{}:{}@", username_encoded, password_encoded))
    };

    tracing::info!("Starting MJPEG stream for camera {} with URL {}", camera_id, rtsp_url.replace(&password_encoded, "***"));

    // Start FFmpeg process - ULTRA optimized MJPEG (lowest latency, highest quality)
    let mut child = Command::new("ffmpeg")
        .args([
            "-rtsp_transport", "tcp",
            "-fflags", "+nobuffer+flush_packets+discardcorrupt",
            "-flags", "low_delay",
            "-analyzeduration", "500000",   // 0.5 second max analysis
            "-probesize", "500000",          // Fast probing
            "-i", &rtsp_url,
            "-f", "mpjpeg",
            "-q:v", "1",                   // Maximum quality (1 is best)
            "-r", "30",                    // 30 fps for smooth motion
            "-an",                         // No audio
            "pipe:1"                       // Native resolution (no scaling)
        ])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to start FFmpeg: {}", e)))?;

    let stdout = child.stdout.take()
        .ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, "No stdout".to_string()))?;

    // Create async stream from stdout
    let stream = tokio_util::io::ReaderStream::new(stdout);
    let body = Body::from_stream(stream);

    // Build response with proper MJPEG multipart content type
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "multipart/x-mixed-replace; boundary=ffmpeg")
        .header(header::CACHE_CONTROL, "no-cache, no-store, must-revalidate")
        .header(header::PRAGMA, "no-cache")
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .body(body)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(response)
}
