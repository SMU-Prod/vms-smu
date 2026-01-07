//! Alarm management (tipo Digifort)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Alarm priority
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum AlarmPriority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Alarm status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AlarmStatus {
    Active,
    Acknowledged,
    Resolved,
}

/// Alarm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alarm {
    /// Alarm ID
    pub id: Uuid,
    /// Alarm name/title
    pub name: String,
    /// Description
    pub description: String,
    /// Priority
    pub priority: AlarmPriority,
    /// Status
    pub status: AlarmStatus,
    /// Camera ID (optional)
    pub camera_id: Option<String>,
    /// Event ID that triggered this alarm
    pub event_id: Option<Uuid>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Acknowledged timestamp
    pub acknowledged_at: Option<DateTime<Utc>>,
    /// Acknowledged by (user ID)
    pub acknowledged_by: Option<String>,
    /// Resolved timestamp
    pub resolved_at: Option<DateTime<Utc>>,
    /// Additional metadata
    pub metadata: serde_json::Value,
}

impl Alarm {
    /// Create new alarm
    pub fn new(
        name: String,
        description: String,
        priority: AlarmPriority,
        camera_id: Option<String>,
        event_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            priority,
            status: AlarmStatus::Active,
            camera_id,
            event_id,
            created_at: Utc::now(),
            acknowledged_at: None,
            acknowledged_by: None,
            resolved_at: None,
            metadata: serde_json::json!({}),
        }
    }

    /// Acknowledge alarm
    pub fn acknowledge(&mut self, user_id: String) {
        self.status = AlarmStatus::Acknowledged;
        self.acknowledged_at = Some(Utc::now());
        self.acknowledged_by = Some(user_id);
    }

    /// Resolve alarm
    pub fn resolve(&mut self) {
        self.status = AlarmStatus::Resolved;
        self.resolved_at = Some(Utc::now());
    }
}

/// Alarm manager
pub struct AlarmManager {
    alarms: Arc<RwLock<HashMap<Uuid, Alarm>>>,
}

impl AlarmManager {
    /// Create new alarm manager
    pub fn new() -> Self {
        Self {
            alarms: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add alarm
    pub async fn add(&self, alarm: Alarm) {
        let mut alarms = self.alarms.write().await;
        alarms.insert(alarm.id, alarm);
    }

    /// Get alarm by ID
    pub async fn get(&self, id: Uuid) -> Option<Alarm> {
        let alarms = self.alarms.read().await;
        alarms.get(&id).cloned()
    }

    /// Update alarm
    pub async fn update(&self, id: Uuid, alarm: Alarm) -> Result<(), String> {
        let mut alarms = self.alarms.write().await;
        if alarms.contains_key(&id) {
            alarms.insert(id, alarm);
            Ok(())
        } else {
            Err("Alarm not found".to_string())
        }
    }

    /// Delete alarm
    pub async fn delete(&self, id: Uuid) -> Result<(), String> {
        let mut alarms = self.alarms.write().await;
        alarms
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| "Alarm not found".to_string())
    }

    /// Acknowledge alarm
    pub async fn acknowledge(&self, id: Uuid) -> Result<(), String> {
        let mut alarms = self.alarms.write().await;
        if let Some(alarm) = alarms.get_mut(&id) {
            alarm.acknowledge("system".to_string()); // TODO: Get real user ID
            Ok(())
        } else {
            Err("Alarm not found".to_string())
        }
    }

    /// List all alarms
    pub async fn list_all(&self) -> Vec<Alarm> {
        let alarms = self.alarms.read().await;
        alarms.values().cloned().collect()
    }

    /// List active alarms
    pub async fn list_active(&self) -> Vec<Alarm> {
        let alarms = self.alarms.read().await;
        alarms
            .values()
            .filter(|a| a.status == AlarmStatus::Active)
            .cloned()
            .collect()
    }

    /// Get active alarm count
    pub async fn active_count(&self) -> usize {
        let alarms = self.alarms.read().await;
        alarms
            .values()
            .filter(|a| a.status == AlarmStatus::Active)
            .count()
    }
}

impl Default for AlarmManager {
    fn default() -> Self {
        Self::new()
    }
}
