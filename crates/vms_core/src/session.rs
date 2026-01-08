//! Session types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::StreamProfile;

/// Live viewing session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub camera_id: Uuid,
    pub node_id: Uuid,
    pub profile: StreamProfile,
    pub stream_url: Option<String>,
    pub status: SessionStatus,
    pub started_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// Session status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    Pending,
    Active,
    Expired,
    Error,
}

/// Start live request
#[derive(Debug, Deserialize)]
pub struct StartLiveRequest {
    pub camera_id: Uuid,
    pub profile: Option<StreamProfile>,
}

/// Start live response (contains signed URL)
#[derive(Debug, Serialize)]
pub struct StartLiveResponse {
    pub session_id: Uuid,
    pub stream_url: String,
    pub expires_at: DateTime<Utc>,
}

/// Signed URL for direct node access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedUrl {
    pub url: String,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

impl SignedUrl {
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}
