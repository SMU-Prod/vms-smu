//! Camera model and types - Professional VMS Configuration

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Transport protocol for RTSP connection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TransportProtocol {
    #[default]
    Auto,
    Tcp,
    Udp,
}

impl TransportProtocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Tcp => "tcp",
            Self::Udp => "udp",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "tcp" => Self::Tcp,
            "udp" => Self::Udp,
            _ => Self::Auto,
        }
    }
}

/// Recording mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum RecordingMode {
    #[default]
    Continuous,
    Motion,
    Manual,
    Disabled,
}

impl RecordingMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Continuous => "continuous",
            Self::Motion => "motion",
            Self::Manual => "manual",
            Self::Disabled => "disabled",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "continuous" => Self::Continuous,
            "motion" => Self::Motion,
            "manual" => Self::Manual,
            _ => Self::Disabled,
        }
    }
}

/// Camera configuration - Full Professional VMS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub id: Uuid,
    
    // === Câmera / Geral ===
    pub name: String,
    pub description: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub firmware: Option<String>,
    pub enabled: bool,
    
    // === Streaming ===
    pub ip_address: String,
    pub rtsp_port: u16,
    pub onvif_port: Option<u16>,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub rtsp_url: String,
    pub onvif_url: Option<String>,
    pub transport: TransportProtocol,
    pub use_ssl: bool,
    pub timeout_ms: u32,
    
    // === Video ===
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub framerate: f32,
    pub codec: String,
    
    // === Gravação ===
    pub recording_mode: RecordingMode,
    pub recording_dir: Option<String>,
    pub retention_days: u32,
    
    // === Localização ===
    pub shortcut: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    
    // === Vinculação ===
    pub server_id: Option<Uuid>,
    
    // === Timestamps ===
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to create a new camera (full form)
#[derive(Debug, Deserialize)]
pub struct CreateCameraRequest {
    // Geral
    pub name: String,
    pub description: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub firmware: Option<String>,
    
    // Streaming
    pub ip_address: String,
    #[serde(default = "default_rtsp_port")]
    pub rtsp_port: u16,
    pub onvif_port: Option<u16>,
    pub username: String,
    pub password: String,
    pub stream_path: Option<String>,
    #[serde(default)]
    pub transport: TransportProtocol,
    #[serde(default)]
    pub use_ssl: bool,
    #[serde(default = "default_timeout")]
    pub timeout_ms: u32,
    
    // Gravação
    #[serde(default)]
    pub recording_mode: RecordingMode,
    pub recording_dir: Option<String>,
    #[serde(default = "default_retention")]
    pub retention_days: u32,
    
    // Localização
    pub shortcut: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    
    // Vinculação
    pub server_id: Option<Uuid>,
}

fn default_rtsp_port() -> u16 { 554 }
fn default_timeout() -> u32 { 30000 }
fn default_retention() -> u32 { 30 }

/// Request to update a camera
#[derive(Debug, Deserialize)]
pub struct UpdateCameraRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub firmware: Option<String>,
    pub enabled: Option<bool>,
    
    pub ip_address: Option<String>,
    pub rtsp_port: Option<u16>,
    pub onvif_port: Option<Option<u16>>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub transport: Option<TransportProtocol>,
    pub use_ssl: Option<bool>,
    pub timeout_ms: Option<u32>,
    
    pub recording_mode: Option<RecordingMode>,
    pub recording_dir: Option<Option<String>>,
    pub retention_days: Option<u32>,
    
    pub shortcut: Option<Option<String>>,
    pub latitude: Option<Option<f64>>,
    pub longitude: Option<Option<f64>>,
    
    pub server_id: Option<Option<Uuid>>,
}

/// Camera validation result
#[derive(Debug, Serialize)]
pub struct CameraValidation {
    pub valid: bool,
    pub onvif_available: bool,
    pub rtsp_available: bool,
    pub error: Option<String>,
}

/// Public camera view (for API responses)
#[derive(Debug, Clone, Serialize)]
pub struct CameraPublic {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub ip_address: String,
    pub rtsp_port: u16,
    pub rtsp_url: String,
    pub transport: TransportProtocol,
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub enabled: bool,
    pub recording_mode: RecordingMode,
    pub shortcut: Option<String>,
    pub server_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl From<Camera> for CameraPublic {
    fn from(c: Camera) -> Self {
        Self {
            id: c.id,
            name: c.name,
            description: c.description,
            manufacturer: c.manufacturer,
            model: c.model,
            ip_address: c.ip_address,
            rtsp_port: c.rtsp_port,
            rtsp_url: c.rtsp_url,
            transport: c.transport,
            resolution_width: c.resolution_width,
            resolution_height: c.resolution_height,
            enabled: c.enabled,
            recording_mode: c.recording_mode,
            shortcut: c.shortcut,
            server_id: c.server_id,
            created_at: c.created_at,
        }
    }
}

impl Camera {
    /// Create new camera from request
    pub fn from_request(req: CreateCameraRequest) -> Self {
        let stream_path = req.stream_path.unwrap_or_else(|| "stream1".to_string());
        
        // Build RTSP URL
        let rtsp_url = format!(
            "rtsp://{}:{}/{}",
            req.ip_address, req.rtsp_port, stream_path
        );
        
        let onvif_url = req.onvif_port.map(|port| {
            format!("http://{}:{}", req.ip_address, port)
        });

        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            name: req.name,
            description: req.description,
            manufacturer: req.manufacturer,
            model: req.model,
            firmware: req.firmware,
            enabled: true,
            
            ip_address: req.ip_address,
            rtsp_port: req.rtsp_port,
            onvif_port: req.onvif_port,
            username: req.username,
            password: req.password,
            rtsp_url,
            onvif_url,
            transport: req.transport,
            use_ssl: req.use_ssl,
            timeout_ms: req.timeout_ms,
            
            resolution_width: 1920,
            resolution_height: 1080,
            framerate: 30.0,
            codec: "h264".to_string(),
            
            recording_mode: req.recording_mode,
            recording_dir: req.recording_dir,
            retention_days: req.retention_days,
            
            shortcut: req.shortcut,
            latitude: req.latitude,
            longitude: req.longitude,
            
            server_id: req.server_id,
            
            created_at: now,
            updated_at: now,
        }
    }
}
