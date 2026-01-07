//! Event types and management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    /// AI detection event
    AIDetection,
    /// Camera status change (online/offline)
    CameraStatus,
    /// Motion detected
    MotionDetection,
    /// Line crossing
    LineCrossing,
    /// Area intrusion
    AreaIntrusion,
    /// Object abandoned
    ObjectAbandoned,
    /// Object removed
    ObjectRemoved,
    /// Loitering (permanência)
    Loitering,
    /// Face recognized
    FaceRecognized,
    /// License plate read
    LPRDetection,
    /// System alarm
    SystemAlarm,
    /// Manual alarm
    ManualAlarm,
    /// Custom event
    Custom,
}

/// Event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Event ID
    pub id: Uuid,
    /// Event type
    pub event_type: EventType,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Camera ID (optional)
    pub camera_id: Option<String>,
    /// Event data (JSON)
    pub data: serde_json::Value,
}

impl Event {
    /// Create new event
    pub fn new(event_type: EventType, camera_id: Option<String>, data: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            timestamp: Utc::now(),
            camera_id,
            data,
        }
    }

    /// Check if event matches a pattern
    pub fn matches(&self, event_type: &EventType, camera_id: Option<&str>) -> bool {
        if &self.event_type != event_type {
            return false;
        }

        if let Some(cam_id) = camera_id {
            if let Some(ref self_cam_id) = self.camera_id {
                return self_cam_id == cam_id;
            }
            return false;
        }

        true
    }
}
