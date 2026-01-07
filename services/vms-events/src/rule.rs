//! Rule engine (motor de regras tipo Digifort)

use super::alarm::{Alarm, AlarmManager, AlarmPriority};
use super::event::{Event, EventType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

/// Rule condition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RuleCondition {
    /// Event type matches
    EventType { event_type: EventType },
    /// Camera ID matches
    CameraId { camera_id: String },
    /// AI object detected
    ObjectDetected { class_name: String, confidence: f32 },
    /// Motion in area
    MotionInArea { area_id: String },
    /// Line crossed
    LineCrossed { line_id: String, direction: String },
    /// Time range
    TimeRange { start: String, end: String },
    /// Combine multiple conditions (AND)
    And { conditions: Vec<RuleCondition> },
    /// Combine multiple conditions (OR)
    Or { conditions: Vec<RuleCondition> },
}

impl RuleCondition {
    /// Check if event matches condition
    pub fn matches(&self, event: &Event) -> bool {
        match self {
            RuleCondition::EventType { event_type } => &event.event_type == event_type,

            RuleCondition::CameraId { camera_id } => {
                if let Some(ref event_camera_id) = event.camera_id {
                    event_camera_id == camera_id
                } else {
                    false
                }
            }

            RuleCondition::ObjectDetected {
                class_name,
                confidence,
            } => {
                // Check if AI event has detected object with given class and confidence
                if let Some(detections) = event.data.get("detections").and_then(|d| d.as_array()) {
                    detections.iter().any(|det| {
                        let det_class = det.get("class_name").and_then(|c| c.as_str()).unwrap_or("");
                        let det_conf = det.get("confidence").and_then(|c| c.as_f64()).unwrap_or(0.0);
                        det_class == class_name && det_conf as f32 >= *confidence
                    })
                } else {
                    false
                }
            }

            RuleCondition::MotionInArea { area_id: _ } => {
                // TODO: Implement area-based motion detection
                false
            }

            RuleCondition::LineCrossed {
                line_id: _,
                direction: _,
            } => {
                // TODO: Implement line crossing detection
                false
            }

            RuleCondition::TimeRange { start: _, end: _ } => {
                // TODO: Implement time range check
                true // For now, always true
            }

            RuleCondition::And { conditions } => {
                conditions.iter().all(|cond| cond.matches(event))
            }

            RuleCondition::Or { conditions } => {
                conditions.iter().any(|cond| cond.matches(event))
            }
        }
    }
}

/// Rule action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RuleAction {
    /// Create alarm
    CreateAlarm {
        name: String,
        description: String,
        priority: AlarmPriority,
    },
    /// Send notification
    SendNotification {
        channel: String,  // email, sms, push
        recipient: String,
        message: String,
    },
    /// Start recording
    StartRecording { camera_id: String, duration_secs: u64 },
    /// PTZ goto preset
    PTZGotoPreset {
        camera_id: String,
        preset: String,
    },
    /// Activate output
    ActivateOutput { output_id: String, duration_secs: u64 },
    /// Run script
    RunScript { script_path: String, args: Vec<String> },
}

impl RuleAction {
    /// Execute action
    pub async fn execute(&self, event: &Event, alarm_manager: &AlarmManager) {
        match self {
            RuleAction::CreateAlarm {
                name,
                description,
                priority,
            } => {
                let alarm = Alarm::new(
                    name.clone(),
                    description.clone(),
                    *priority,
                    event.camera_id.clone(),
                    Some(event.id),
                );
                alarm_manager.add(alarm).await;
                info!("🚨 Created alarm: {}", name);
            }

            RuleAction::SendNotification {
                channel,
                recipient,
                message,
            } => {
                info!(
                    "📧 Sending notification via {} to {}: {}",
                    channel, recipient, message
                );
                // TODO: Integrate with vms-notifications service
            }

            RuleAction::StartRecording {
                camera_id,
                duration_secs,
            } => {
                info!(
                    "🎬 Starting recording on camera {} for {}s",
                    camera_id, duration_secs
                );
                // TODO: Send command to vms-storage
            }

            RuleAction::PTZGotoPreset { camera_id, preset } => {
                info!("📹 PTZ goto preset {} on camera {}", preset, camera_id);
                // TODO: Send command to vms-ingest ONVIF PTZ
            }

            RuleAction::ActivateOutput {
                output_id,
                duration_secs,
            } => {
                info!(
                    "⚡ Activating output {} for {}s",
                    output_id, duration_secs
                );
                // TODO: Integrate with automation system
            }

            RuleAction::RunScript { script_path, args } => {
                info!("🔧 Running script: {} {:?}", script_path, args);
                // TODO: Execute script safely
            }
        }
    }
}

/// Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// Rule ID
    pub id: Uuid,
    /// Rule name
    pub name: String,
    /// Description
    pub description: String,
    /// Enabled
    pub enabled: bool,
    /// Conditions (AND logic between them)
    pub conditions: Vec<RuleCondition>,
    /// Actions to execute
    pub actions: Vec<RuleAction>,
    /// Cooldown seconds (prevent spam)
    #[serde(default)]
    pub cooldown_secs: u64,
    /// Last triggered timestamp
    #[serde(skip)]
    pub last_triggered: Option<chrono::DateTime<chrono::Utc>>,
}

impl Rule {
    /// Create new rule
    pub fn new(
        name: String,
        description: String,
        conditions: Vec<RuleCondition>,
        actions: Vec<RuleAction>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            enabled: true,
            conditions,
            actions,
            cooldown_secs: 0,
            last_triggered: None,
        }
    }

    /// Check if rule matches event
    pub fn matches(&self, event: &Event) -> bool {
        if !self.enabled {
            return false;
        }

        // Check cooldown
        if let Some(last) = self.last_triggered {
            let elapsed = chrono::Utc::now().signed_duration_since(last);
            if elapsed.num_seconds() < self.cooldown_secs as i64 {
                return false; // Still in cooldown
            }
        }

        // All conditions must match
        self.conditions.iter().all(|cond| cond.matches(event))
    }
}

/// Rule engine
pub struct RuleEngine {
    rules: Arc<RwLock<HashMap<Uuid, Rule>>>,
}

impl RuleEngine {
    /// Create new rule engine
    pub fn new() -> Self {
        Self {
            rules: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add rule
    pub async fn add(&self, rule: Rule) {
        let mut rules = self.rules.write().await;
        rules.insert(rule.id, rule);
    }

    /// Get rule
    pub async fn get(&self, id: Uuid) -> Option<Rule> {
        let rules = self.rules.read().await;
        rules.get(&id).cloned()
    }

    /// Update rule
    pub async fn update(&self, id: Uuid, rule: Rule) -> Result<(), String> {
        let mut rules = self.rules.write().await;
        if rules.contains_key(&id) {
            rules.insert(id, rule);
            Ok(())
        } else {
            Err("Rule not found".to_string())
        }
    }

    /// Delete rule
    pub async fn delete(&self, id: Uuid) -> Result<(), String> {
        let mut rules = self.rules.write().await;
        rules
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| "Rule not found".to_string())
    }

    /// Enable rule
    pub async fn enable(&self, id: Uuid) -> Result<(), String> {
        let mut rules = self.rules.write().await;
        if let Some(rule) = rules.get_mut(&id) {
            rule.enabled = true;
            Ok(())
        } else {
            Err("Rule not found".to_string())
        }
    }

    /// Disable rule
    pub async fn disable(&self, id: Uuid) -> Result<(), String> {
        let mut rules = self.rules.write().await;
        if let Some(rule) = rules.get_mut(&id) {
            rule.enabled = false;
            Ok(())
        } else {
            Err("Rule not found".to_string())
        }
    }

    /// List all rules
    pub async fn list_all(&self) -> Vec<Rule> {
        let rules = self.rules.read().await;
        rules.values().cloned().collect()
    }

    /// Process event through all rules
    pub async fn process_event(&self, event: &Event, alarm_manager: &AlarmManager) {
        let mut rules = self.rules.write().await;

        for rule in rules.values_mut() {
            if rule.matches(event) {
                info!("✅ Rule matched: {}", rule.name);

                // Execute actions
                for action in &rule.actions {
                    action.execute(event, alarm_manager).await;
                }

                // Update last triggered
                rule.last_triggered = Some(chrono::Utc::now());
            }
        }
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}
