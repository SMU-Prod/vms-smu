//! Node types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Node (Data Plane agent) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub api_key: String,
    pub status: NodeStatus,
    pub capabilities: NodeCapabilities,
    pub last_seen: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Node status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeStatus {
    Online,
    Offline,
    Degraded,
}

/// Node capabilities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeCapabilities {
    pub max_cameras: u32,
    pub supports_gpu: bool,
    pub supports_recording: bool,
    pub supports_ai: bool,
}

/// Register node request
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterNodeRequest {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub capabilities: NodeCapabilities,
}

/// Node heartbeat
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeHeartbeat {
    pub node_id: Uuid,
    pub status: NodeStatus,
    pub active_sessions: u32,
    pub cpu_usage: f32,
    pub memory_usage: f32,
}

/// Command from Server to Node
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeCommand {
    StartLive {
        session_id: Uuid,
        camera_id: Uuid,
        profile: StreamProfile,
    },
    StopLive {
        session_id: Uuid,
    },
    StartRecording {
        camera_id: Uuid,
        profile: StreamProfile,
    },
    StopRecording {
        camera_id: Uuid,
    },
    HealthCheck,
}

/// Stream profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamProfile {
    pub resolution: (u32, u32),
    pub framerate: u32,
    pub bitrate: u32,
    pub codec: String,
}

impl Default for StreamProfile {
    fn default() -> Self {
        Self {
            resolution: (1920, 1080),
            framerate: 30,
            bitrate: 4000,
            codec: "h264".to_string(),
        }
    }
}
