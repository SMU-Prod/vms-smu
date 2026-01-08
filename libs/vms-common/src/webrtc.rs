//! WebRTC types for signaling and streaming
//!
//! Contract types for WebRTC offer/answer exchange between Viewer and Server/Node.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// WebRTC offer request from browser/viewer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebRtcOfferRequest {
    /// Camera ID to stream
    pub camera_id: Uuid,
    /// SDP offer from browser
    pub sdp: String,
    /// SDP type (always "offer")
    #[serde(default = "default_offer_type")]
    pub sdp_type: String,
    /// Short-lived session token for security
    pub session_token: Option<String>,
}

fn default_offer_type() -> String {
    "offer".to_string()
}

/// WebRTC answer response to browser/viewer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebRtcAnswerResponse {
    /// SDP answer
    pub sdp: String,
    /// SDP type (always "answer")
    #[serde(default = "default_answer_type")]
    pub sdp_type: String,
    /// Peer connection ID (for cleanup/debugging)
    pub peer_id: Uuid,
    /// Session expiration timestamp
    pub expires_at: i64,
    /// RTP port where FFmpeg sends data (for debugging/migration)
    #[serde(default)]
    pub rtp_port: u16,
}

fn default_answer_type() -> String {
    "answer".to_string()
}

/// ICE candidate request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceCandidateRequest {
    /// Session/Peer ID
    pub peer_id: Uuid,
    /// ICE candidate string
    pub candidate: String,
    /// SDP media ID
    #[serde(rename = "sdpMid")]
    pub sdp_mid: Option<String>,
    /// SDP media line index
    #[serde(rename = "sdpMLineIndex")]
    pub sdp_m_line_index: Option<u16>,
}

/// API error response (standardized)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorBody {
    /// Error code (e.g., "SESSION_TOKEN_INVALID", "CAMERA_NOT_FOUND")
    pub code: String,
    /// Human-readable error message
    pub message: String,
}

impl ApiErrorBody {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

/// WebRTC session stop request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebRtcStopRequest {
    pub peer_id: Uuid,
}
