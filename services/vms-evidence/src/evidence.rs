//! Evidence management (Ocorrências e evidências)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Evidence type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceType {
    /// Video clip
    Video,
    /// Image snapshot
    Image,
    /// Audio recording
    Audio,
    /// Document/Report
    Document,
    /// Raw data/Metadata
    Data,
}

/// Evidence status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceStatus {
    /// Draft (being created)
    Draft,
    /// Active/Available
    Active,
    /// Exported
    Exported,
    /// Archived
    Archived,
    /// Deleted (soft delete)
    Deleted,
}

/// Evidence priority
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EvidencePriority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Evidence attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceAttachment {
    /// Attachment ID
    pub id: Uuid,
    /// Attachment type
    pub attachment_type: EvidenceType,
    /// File name
    pub file_name: String,
    /// File path (internal storage)
    pub file_path: String,
    /// File size (bytes)
    pub file_size: u64,
    /// SHA256 hash
    pub sha256: String,
    /// MIME type
    pub mime_type: String,
    /// Camera ID (if video/image)
    pub camera_id: Option<String>,
    /// Start timestamp (for video clips)
    pub start_time: Option<DateTime<Utc>>,
    /// End timestamp (for video clips)
    pub end_time: Option<DateTime<Utc>>,
    /// Duration (seconds, for video/audio)
    pub duration_secs: Option<f64>,
    /// Metadata
    pub metadata: serde_json::Value,
    /// Created at
    pub created_at: DateTime<Utc>,
}

impl EvidenceAttachment {
    /// Create new attachment
    pub fn new(
        attachment_type: EvidenceType,
        file_name: String,
        file_path: String,
        file_size: u64,
        sha256: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            attachment_type,
            file_name,
            file_path,
            file_size,
            sha256,
            mime_type: String::new(),
            camera_id: None,
            start_time: None,
            end_time: None,
            duration_secs: None,
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        }
    }
}

/// Evidence (Occurrence/Case)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    /// Evidence ID
    pub id: Uuid,
    /// Case number (auto-generated)
    pub case_number: String,
    /// Title
    pub title: String,
    /// Description
    pub description: String,
    /// Evidence type
    pub evidence_type: String,
    /// Priority
    pub priority: EvidencePriority,
    /// Status
    pub status: EvidenceStatus,
    /// Related event ID
    pub event_id: Option<Uuid>,
    /// Related alarm ID
    pub alarm_id: Option<Uuid>,
    /// Camera IDs involved
    pub camera_ids: Vec<String>,
    /// Tags
    pub tags: Vec<String>,
    /// Location
    pub location: Option<String>,
    /// Created by user ID
    pub created_by: Uuid,
    /// Assigned to user ID
    pub assigned_to: Option<Uuid>,
    /// Attachments
    pub attachments: Vec<EvidenceAttachment>,
    /// Chain of custody (audit log)
    pub custody_chain: Vec<CustodyEntry>,
    /// Custom metadata
    pub metadata: serde_json::Value,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
    /// Closed at
    pub closed_at: Option<DateTime<Utc>>,
}

impl Evidence {
    /// Create new evidence
    pub fn new(
        title: String,
        description: String,
        evidence_type: String,
        priority: EvidencePriority,
        created_by: Uuid,
    ) -> Self {
        let now = Utc::now();
        let case_number = Self::generate_case_number();

        Self {
            id: Uuid::new_v4(),
            case_number,
            title,
            description,
            evidence_type,
            priority,
            status: EvidenceStatus::Draft,
            event_id: None,
            alarm_id: None,
            camera_ids: Vec::new(),
            tags: Vec::new(),
            location: None,
            created_by,
            assigned_to: None,
            attachments: Vec::new(),
            custody_chain: Vec::new(),
            metadata: serde_json::json!({}),
            created_at: now,
            updated_at: now,
            closed_at: None,
        }
    }

    /// Generate case number (format: EVD-YYYYMMDD-NNNN)
    fn generate_case_number() -> String {
        let now = Utc::now();
        let date_part = now.format("%Y%m%d").to_string();
        let random_part = format!("{:04}", rand::random::<u16>() % 10000);
        format!("EVD-{}-{}", date_part, random_part)
    }

    /// Add attachment
    pub fn add_attachment(&mut self, attachment: EvidenceAttachment) {
        self.attachments.push(attachment);
        self.updated_at = Utc::now();
    }

    /// Add custody entry
    pub fn add_custody_entry(&mut self, action: String, user_id: Uuid, details: String) {
        let entry = CustodyEntry::new(action, user_id, details);
        self.custody_chain.push(entry);
        self.updated_at = Utc::now();
    }

    /// Change status
    pub fn change_status(&mut self, new_status: EvidenceStatus, user_id: Uuid) {
        self.status = new_status.clone();
        self.add_custody_entry(
            "status_changed".to_string(),
            user_id,
            format!("Status changed to {:?}", new_status),
        );

        if new_status == EvidenceStatus::Archived || new_status == EvidenceStatus::Deleted {
            self.closed_at = Some(Utc::now());
        }
    }
}

/// Chain of custody entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustodyEntry {
    /// Entry ID
    pub id: Uuid,
    /// Action performed
    pub action: String,
    /// User who performed action
    pub user_id: Uuid,
    /// Details
    pub details: String,
    /// IP address
    pub ip_address: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl CustodyEntry {
    /// Create new custody entry
    pub fn new(action: String, user_id: Uuid, details: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            action,
            user_id,
            details,
            ip_address: None,
            timestamp: Utc::now(),
        }
    }
}

/// Evidence manager
pub struct EvidenceManager {
    evidences: Arc<RwLock<HashMap<Uuid, Evidence>>>,
}

impl EvidenceManager {
    /// Create new evidence manager
    pub fn new() -> Self {
        Self {
            evidences: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add evidence
    pub async fn add(&self, mut evidence: Evidence) {
        evidence.add_custody_entry(
            "created".to_string(),
            evidence.created_by,
            "Evidence created".to_string(),
        );
        let mut evidences = self.evidences.write().await;
        evidences.insert(evidence.id, evidence);
    }

    /// Get evidence by ID
    pub async fn get(&self, id: Uuid) -> Option<Evidence> {
        let evidences = self.evidences.read().await;
        evidences.get(&id).cloned()
    }

    /// Get evidence by case number
    pub async fn get_by_case_number(&self, case_number: &str) -> Option<Evidence> {
        let evidences = self.evidences.read().await;
        evidences
            .values()
            .find(|e| e.case_number == case_number)
            .cloned()
    }

    /// Update evidence
    pub async fn update(&self, id: Uuid, mut evidence: Evidence) -> Result<(), String> {
        let mut evidences = self.evidences.write().await;
        if evidences.contains_key(&id) {
            evidence.updated_at = Utc::now();
            evidences.insert(id, evidence);
            Ok(())
        } else {
            Err("Evidence not found".to_string())
        }
    }

    /// Delete evidence (soft delete)
    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<(), String> {
        let mut evidences = self.evidences.write().await;
        if let Some(evidence) = evidences.get_mut(&id) {
            evidence.change_status(EvidenceStatus::Deleted, user_id);
            Ok(())
        } else {
            Err("Evidence not found".to_string())
        }
    }

    /// List all evidences
    pub async fn list_all(&self) -> Vec<Evidence> {
        let evidences = self.evidences.read().await;
        evidences
            .values()
            .filter(|e| e.status != EvidenceStatus::Deleted)
            .cloned()
            .collect()
    }

    /// List evidences by status
    pub async fn list_by_status(&self, status: EvidenceStatus) -> Vec<Evidence> {
        let evidences = self.evidences.read().await;
        evidences
            .values()
            .filter(|e| e.status == status)
            .cloned()
            .collect()
    }

    /// List evidences by user
    pub async fn list_by_user(&self, user_id: Uuid) -> Vec<Evidence> {
        let evidences = self.evidences.read().await;
        evidences
            .values()
            .filter(|e| e.created_by == user_id || e.assigned_to == Some(user_id))
            .filter(|e| e.status != EvidenceStatus::Deleted)
            .cloned()
            .collect()
    }

    /// Search evidences by tags
    pub async fn search_by_tags(&self, tags: Vec<String>) -> Vec<Evidence> {
        let evidences = self.evidences.read().await;
        evidences
            .values()
            .filter(|e| {
                e.status != EvidenceStatus::Deleted
                    && tags.iter().any(|tag| e.tags.contains(tag))
            })
            .cloned()
            .collect()
    }
}

impl Default for EvidenceManager {
    fn default() -> Self {
        Self::new()
    }
}
