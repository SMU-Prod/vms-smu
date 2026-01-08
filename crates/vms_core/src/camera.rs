//! Camera types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Camera configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub id: Uuid,
    pub node_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub manufacturer: String,
    pub model: String,
    pub firmware: Option<String>,
    pub rtsp_url: String,
    pub onvif_url: Option<String>,
    pub recording_path: Option<String>,
    pub connection_timeout_ms: Option<u32>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub notes: Option<String>,
    pub transport: Option<String>,
    pub status: CameraStatus,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Camera status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CameraStatus {
    Online,
    Offline,
    Error,
    Unknown,
}

/// Create camera request
#[derive(Debug, Deserialize)]
pub struct CreateCameraRequest {
    pub node_id: Uuid,
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
    pub recording_path: Option<String>,
    pub connection_timeout_ms: Option<u32>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub notes: Option<String>,
    pub transport: Option<String>,
    pub enabled: Option<bool>,
}

/// Update camera request (partial)
#[derive(Debug, Deserialize, Default)]
pub struct UpdateCameraRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub firmware: Option<String>,
    pub ip: Option<String>,
    pub rtsp_port: Option<u16>,
    pub onvif_port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub stream_path: Option<String>,
    pub recording_path: Option<String>,
    pub connection_timeout_ms: Option<u32>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub notes: Option<String>,
    pub transport: Option<String>,
    pub enabled: Option<bool>,
}

/// Camera credentials (stored encrypted, never in plain camera table)
#[derive(Debug, Clone)]
pub struct CameraCredentials {
    pub camera_id: Uuid,
    pub username: String,
    pub password: String,
}
