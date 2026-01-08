//! WebRTC signaling routes for low-latency video streaming
//!
//! Handles SDP offer/answer exchange and ICE candidates for WebRTC connections.
//! Target latency: 50-150ms

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::post,
    Json, Router,
};
use uuid::Uuid;

use vms_common::{ApiErrorBody, WebRtcAnswerResponse, IceCandidateRequest};

use crate::AppState;

/// Request for SDP offer
#[derive(Debug, serde::Deserialize)]
pub struct OfferRequest {
    pub sdp: String,
    #[serde(default = "default_offer")]
    pub sdp_type: String,
}

fn default_offer() -> String { "offer".to_string() }

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/offer/:camera_id", post(handle_offer))
        .route("/ice/:camera_id", post(handle_ice))
        .route("/stop/:peer_id", post(handle_stop))
}

/// Handle SDP offer from browser and return answer
async fn handle_offer(
    State(state): State<AppState>,
    Path(camera_id): Path<Uuid>,
    Json(offer): Json<OfferRequest>,
) -> Result<Json<WebRtcAnswerResponse>, (StatusCode, Json<ApiErrorBody>)> {
    tracing::info!("📡 WebRTC offer received for camera {} ({} bytes)", camera_id, offer.sdp.len());

    // 1) Get camera to verify it exists
    let _camera = state.camera_repo
        .get(camera_id)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiErrorBody::new("DB_ERROR", e.to_string()))
        ))?
        .ok_or_else(|| (
            StatusCode::NOT_FOUND,
            Json(ApiErrorBody::new("CAMERA_NOT_FOUND", "Camera not found"))
        ))?;

    // 2) Generate peer_id and expiration
    let peer_id = Uuid::new_v4();
    let expires_at = chrono::Utc::now().timestamp() + 3600; // 1 hour

    // TODO: In production, integrate with vms-stream WebRTCServer
    // For now, return a placeholder SDP answer
    tracing::info!("✅ WebRTC session {} created for camera {}", peer_id, camera_id);

    let sdp_answer = format!(
        "v=0\r\no=- {} 2 IN IP4 127.0.0.1\r\ns=-\r\nt=0 0\r\na=group:BUNDLE 0\r\nm=video 9 UDP/TLS/RTP/SAVPF 96\r\nc=IN IP4 0.0.0.0\r\na=rtcp:9 IN IP4 0.0.0.0\r\na=ice-ufrag:vms\r\na=ice-pwd:vmssecret\r\na=setup:active\r\na=mid:0\r\na=sendonly\r\na=rtpmap:96 H264/90000\r\na=fmtp:96 level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42e01f\r\n",
        peer_id.as_u128()
    );

    Ok(Json(WebRtcAnswerResponse {
        sdp: sdp_answer,
        sdp_type: "answer".to_string(),
        peer_id,
        expires_at,
        rtp_port: 0,
    }))
}

/// Handle ICE candidate from browser
async fn handle_ice(
    State(_state): State<AppState>,
    Path(_camera_id): Path<Uuid>,
    Json(_candidate): Json<IceCandidateRequest>,
) -> StatusCode {
    StatusCode::OK
}

/// Stop a WebRTC session
async fn handle_stop(
    State(_state): State<AppState>,
    Path(peer_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ApiErrorBody>)> {
    tracing::info!("🛑 WebRTC session {} stop requested", peer_id);
    Ok(StatusCode::OK)
}
