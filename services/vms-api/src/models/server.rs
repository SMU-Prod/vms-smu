//! Server (Node) model
//!
//! Represents a streaming server that handles WebRTC connections

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Server entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub status: ServerStatus,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
}

impl Server {
    pub fn new(name: String, ip: String, port: u16, username: String, password: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            ip,
            port,
            username,
            password,
            status: ServerStatus::Offline,
            enabled: true,
            created_at: now,
            updated_at: now,
            last_seen: None,
        }
    }

    /// Get the base URL for this server
    pub fn base_url(&self) -> String {
        format!("http://{}:{}", self.ip, self.port)
    }

    /// Get the WebRTC offer URL
    pub fn webrtc_offer_url(&self) -> String {
        format!("{}/api/v1/webrtc/offer", self.base_url())
    }
}

/// Server status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServerStatus {
    Online,
    Offline,
    Error,
}

impl ServerStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::Offline => "offline",
            Self::Error => "error",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "online" => Self::Online,
            "error" => Self::Error,
            _ => Self::Offline,
        }
    }
}

/// Public server view (for API responses)
#[derive(Debug, Clone, Serialize)]
pub struct ServerPublic {
    pub id: Uuid,
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub username: String,
    pub status: ServerStatus,
    pub enabled: bool,
    pub webrtc_url: String,
    pub created_at: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
}

impl From<Server> for ServerPublic {
    fn from(s: Server) -> Self {
        Self {
            id: s.id,
            name: s.name.clone(),
            ip: s.ip.clone(),
            port: s.port,
            username: s.username.clone(),
            status: s.status,
            enabled: s.enabled,
            webrtc_url: s.webrtc_offer_url(),
            created_at: s.created_at,
            last_seen: s.last_seen,
        }
    }
}

/// Request to create a new server
#[derive(Debug, Deserialize)]
pub struct CreateServerRequest {
    pub name: String,
    pub ip: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub username: String,
    pub password: String,
}

fn default_port() -> u16 { 9094 }

/// Request to update a server
#[derive(Debug, Deserialize)]
pub struct UpdateServerRequest {
    pub name: Option<String>,
    pub ip: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub enabled: Option<bool>,
}
