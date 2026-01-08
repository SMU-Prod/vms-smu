//! Camera model and types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Camera configuration and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub manufacturer: String,
    pub model: String,
    pub firmware: Option<String>,
    pub rtsp_url: String,
    pub onvif_url: Option<String>,
    pub username: String,
    pub password: String,
    pub shortcut: Option<String>,
    pub recording_dir: Option<String>,
    pub notes: Option<String>,
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub framerate: f32,
    pub codec: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to create a new camera
#[derive(Debug, Deserialize)]
pub struct CreateCameraRequest {
    pub name: String,
    pub description: Option<String>,
    pub manufacturer: String,
    pub model: String,
    pub firmware: Option<String>,
    pub ip: String,
    pub rtsp_port: u16,
    pub onvif_port: Option<u16>,
    pub username: String,
    pub password: String,
    pub stream_path: Option<String>,
    pub shortcut: Option<String>,
    pub recording_dir: Option<String>,
    pub notes: Option<String>,
    pub enabled: Option<bool>,
}

/// Request to update a camera
#[derive(Debug, Deserialize)]
pub struct UpdateCameraRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub firmware: Option<String>,
    pub shortcut: Option<String>,
    pub recording_dir: Option<String>,
    pub notes: Option<String>,
    pub enabled: Option<bool>,
}

/// Camera validation result
#[derive(Debug, Serialize)]
pub struct CameraValidation {
    pub valid: bool,
    pub onvif_available: bool,
    pub rtsp_available: bool,
    pub error: Option<String>,
}

impl Camera {
    /// Create new camera from request
    pub fn from_request(req: CreateCameraRequest) -> Self {
        let stream_path = req.stream_path.unwrap_or_else(|| "stream1".to_string());
        
        // Generate clean RTSP URL without inline credentials
        let rtsp_url = format!(
            "rtsp://{}:{}/{}",
            req.ip, req.rtsp_port, stream_path
        );
        
        let onvif_url = req.onvif_port.map(|port| {
            format!("http://{}:{}", req.ip, port)
        });

        Self {
            id: Uuid::new_v4(),
            name: req.name,
            description: req.description,
            manufacturer: req.manufacturer,
            model: req.model,
            firmware: req.firmware,
            rtsp_url,
            onvif_url,
            username: req.username,
            password: req.password,
            shortcut: req.shortcut,
            recording_dir: req.recording_dir,
            notes: req.notes,
            resolution_width: 1920,
            resolution_height: 1080,
            framerate: 30.0,
            codec: "h264".to_string(),
            enabled: req.enabled.unwrap_or(true),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
