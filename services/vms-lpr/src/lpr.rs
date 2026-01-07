//! License Plate Recognition structures

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Plate format/country
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlateFormat {
    /// Brazil (ABC-1234 or ABC1D23 Mercosul)
    Brazil,
    /// USA (various formats by state)
    USA,
    /// Europe (country-specific)
    Europe,
    /// Other/Unknown
    Other,
}

/// Plate list type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlateListType {
    /// Allowed plates (whitelist)
    Allowlist,
    /// Blocked plates (blacklist)
    Blocklist,
    /// Watchlist (monitoring)
    Watchlist,
}

/// Registered plate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredPlate {
    /// Plate ID
    pub id: Uuid,
    /// Plate number (normalized, uppercase, no special chars)
    pub plate_number: String,
    /// Original plate (as registered)
    pub original_plate: String,
    /// List type
    pub list_type: PlateListType,
    /// Plate format
    pub format: PlateFormat,
    /// Owner name
    pub owner_name: Option<String>,
    /// Vehicle description
    pub vehicle_description: Option<String>,
    /// Notes
    pub notes: Option<String>,
    /// Expiration date
    pub expires_at: Option<DateTime<Utc>>,
    /// Active
    pub active: bool,
    /// Created by user ID
    pub created_by: Uuid,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

impl RegisteredPlate {
    /// Create new registered plate
    pub fn new(
        plate_number: String,
        list_type: PlateListType,
        format: PlateFormat,
        created_by: Uuid,
    ) -> Self {
        let normalized = Self::normalize_plate(&plate_number);
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            plate_number: normalized,
            original_plate: plate_number,
            list_type,
            format,
            owner_name: None,
            vehicle_description: None,
            notes: None,
            expires_at: None,
            active: true,
            created_by,
            created_at: now,
            updated_at: now,
        }
    }

    /// Normalize plate number (uppercase, remove special chars)
    pub fn normalize_plate(plate: &str) -> String {
        plate
            .to_uppercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect()
    }

    /// Check if plate is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }

    /// Check if plate is active (not expired and active flag)
    pub fn is_active(&self) -> bool {
        self.active && !self.is_expired()
    }
}

/// LPR detection (plate read)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LprDetection {
    /// Detection ID
    pub id: Uuid,
    /// Plate number (normalized)
    pub plate_number: String,
    /// Raw plate (as detected)
    pub raw_plate: String,
    /// Camera ID
    pub camera_id: String,
    /// Confidence (0.0 - 1.0)
    pub confidence: f32,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Bounding box (x, y, width, height, normalized 0-1)
    pub bbox: Option<Vec<f32>>,
    /// Image path/URL
    pub image_path: Option<String>,
    /// Vehicle color (if detected)
    pub vehicle_color: Option<String>,
    /// Vehicle type (if detected)
    pub vehicle_type: Option<String>,
    /// Direction (in/out)
    pub direction: Option<String>,
    /// Matched registered plate
    pub matched_plate: Option<Uuid>,
    /// Match type (if matched)
    pub match_type: Option<PlateListType>,
    /// Metadata
    pub metadata: serde_json::Value,
}

impl LprDetection {
    /// Create new detection
    pub fn new(plate_number: String, camera_id: String, confidence: f32) -> Self {
        let normalized = RegisteredPlate::normalize_plate(&plate_number);

        Self {
            id: Uuid::new_v4(),
            plate_number: normalized,
            raw_plate: plate_number,
            camera_id,
            confidence,
            timestamp: Utc::now(),
            bbox: None,
            image_path: None,
            vehicle_color: None,
            vehicle_type: None,
            direction: None,
            matched_plate: None,
            match_type: None,
            metadata: serde_json::json!({}),
        }
    }
}

/// LPR statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LprStats {
    /// Total detections
    pub total_detections: u64,
    /// Total unique plates
    pub unique_plates: u64,
    /// Allowlist matches
    pub allowlist_matches: u64,
    /// Blocklist matches
    pub blocklist_matches: u64,
    /// Watchlist matches
    pub watchlist_matches: u64,
    /// Average confidence
    pub avg_confidence: f32,
}

/// LPR manager
pub struct LprManager {
    registered_plates: Arc<RwLock<HashMap<Uuid, RegisteredPlate>>>,
    detections: Arc<RwLock<Vec<LprDetection>>>,
    /// Plate number -> RegisteredPlate mapping for fast lookup
    plate_index: Arc<RwLock<HashMap<String, Uuid>>>,
}

impl LprManager {
    /// Create new LPR manager
    pub fn new() -> Self {
        Self {
            registered_plates: Arc::new(RwLock::new(HashMap::new())),
            detections: Arc::new(RwLock::new(Vec::new())),
            plate_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Registered Plates

    /// Add registered plate
    pub async fn add_plate(&self, plate: RegisteredPlate) {
        let mut plates = self.registered_plates.write().await;
        let mut index = self.plate_index.write().await;

        index.insert(plate.plate_number.clone(), plate.id);
        plates.insert(plate.id, plate);
    }

    /// Get registered plate by ID
    pub async fn get_plate(&self, id: Uuid) -> Option<RegisteredPlate> {
        let plates = self.registered_plates.read().await;
        plates.get(&id).cloned()
    }

    /// Get registered plate by number
    pub async fn get_plate_by_number(&self, plate_number: &str) -> Option<RegisteredPlate> {
        let normalized = RegisteredPlate::normalize_plate(plate_number);
        let index = self.plate_index.read().await;

        if let Some(plate_id) = index.get(&normalized) {
            let plates = self.registered_plates.read().await;
            plates.get(plate_id).cloned()
        } else {
            None
        }
    }

    /// Update registered plate
    pub async fn update_plate(&self, id: Uuid, mut plate: RegisteredPlate) -> Result<(), String> {
        let mut plates = self.registered_plates.write().await;
        if plates.contains_key(&id) {
            plate.updated_at = Utc::now();
            plates.insert(id, plate);
            Ok(())
        } else {
            Err("Plate not found".to_string())
        }
    }

    /// Delete registered plate
    pub async fn delete_plate(&self, id: Uuid) -> Result<(), String> {
        let mut plates = self.registered_plates.write().await;
        let mut index = self.plate_index.write().await;

        if let Some(plate) = plates.remove(&id) {
            index.remove(&plate.plate_number);
            Ok(())
        } else {
            Err("Plate not found".to_string())
        }
    }

    /// List registered plates
    pub async fn list_plates(&self, list_type: Option<PlateListType>) -> Vec<RegisteredPlate> {
        let plates = self.registered_plates.read().await;
        plates
            .values()
            .filter(|p| {
                list_type
                    .as_ref()
                    .map_or(true, |lt| &p.list_type == lt)
            })
            .cloned()
            .collect()
    }

    // Detections

    /// Add detection
    pub async fn add_detection(&self, mut detection: LprDetection) {
        // Check if plate is registered
        if let Some(registered) = self.get_plate_by_number(&detection.plate_number).await {
            if registered.is_active() {
                detection.matched_plate = Some(registered.id);
                detection.match_type = Some(registered.list_type);
            }
        }

        let mut detections = self.detections.write().await;
        detections.push(detection);

        // Keep only last 10000 detections in memory
        if detections.len() > 10000 {
            detections.drain(0..1000);
        }
    }

    /// Get detection by ID
    pub async fn get_detection(&self, id: Uuid) -> Option<LprDetection> {
        let detections = self.detections.read().await;
        detections.iter().find(|d| d.id == id).cloned()
    }

    /// List recent detections
    pub async fn list_detections(&self, limit: usize) -> Vec<LprDetection> {
        let detections = self.detections.read().await;
        detections
            .iter()
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    /// List detections by plate
    pub async fn list_detections_by_plate(&self, plate_number: &str) -> Vec<LprDetection> {
        let normalized = RegisteredPlate::normalize_plate(plate_number);
        let detections = self.detections.read().await;
        detections
            .iter()
            .filter(|d| d.plate_number == normalized)
            .cloned()
            .collect()
    }

    /// List detections by camera
    pub async fn list_detections_by_camera(&self, camera_id: &str) -> Vec<LprDetection> {
        let detections = self.detections.read().await;
        detections
            .iter()
            .filter(|d| d.camera_id == camera_id)
            .cloned()
            .collect()
    }

    /// Get statistics
    pub async fn get_stats(&self) -> LprStats {
        let detections = self.detections.read().await;
        let plates = self.registered_plates.read().await;

        let total = detections.len() as u64;
        let unique: std::collections::HashSet<_> =
            detections.iter().map(|d| &d.plate_number).collect();

        let allowlist = detections
            .iter()
            .filter(|d| d.match_type == Some(PlateListType::Allowlist))
            .count() as u64;
        let blocklist = detections
            .iter()
            .filter(|d| d.match_type == Some(PlateListType::Blocklist))
            .count() as u64;
        let watchlist = detections
            .iter()
            .filter(|d| d.match_type == Some(PlateListType::Watchlist))
            .count() as u64;

        let avg_conf = if !detections.is_empty() {
            detections.iter().map(|d| d.confidence).sum::<f32>() / detections.len() as f32
        } else {
            0.0
        };

        LprStats {
            total_detections: total,
            unique_plates: unique.len() as u64,
            allowlist_matches: allowlist,
            blocklist_matches: blocklist,
            watchlist_matches: watchlist,
            avg_confidence: avg_conf,
        }
    }
}

impl Default for LprManager {
    fn default() -> Self {
        Self::new()
    }
}
