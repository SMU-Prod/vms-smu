//! Face recognition structures

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Person watchlist type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WatchlistType {
    /// Authorized persons (employees, residents, etc)
    Authorized,
    /// VIP list
    Vip,
    /// Watchlist (monitoring)
    Watch,
    /// Blocklist (not allowed)
    Blocklist,
}

/// Registered person
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredPerson {
    /// Person ID
    pub id: Uuid,
    /// Name
    pub name: String,
    /// Document ID (CPF, RG, etc)
    pub document_id: Option<String>,
    /// Watchlist type
    pub watchlist_type: WatchlistType,
    /// Face embeddings (512-dimensional vectors)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub embeddings: Vec<FaceEmbedding>,
    /// Photos (base64 or URLs)
    pub photos: Vec<String>,
    /// Notes
    pub notes: Option<String>,
    /// Department/Group
    pub department: Option<String>,
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

impl RegisteredPerson {
    /// Create new registered person
    pub fn new(name: String, watchlist_type: WatchlistType, created_by: Uuid) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            name,
            document_id: None,
            watchlist_type,
            embeddings: Vec::new(),
            photos: Vec::new(),
            notes: None,
            department: None,
            expires_at: None,
            active: true,
            created_by,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if person is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }

    /// Check if person is active
    pub fn is_active(&self) -> bool {
        self.active && !self.is_expired()
    }

    /// Add embedding
    pub fn add_embedding(&mut self, embedding: FaceEmbedding) {
        self.embeddings.push(embedding);
        self.updated_at = Utc::now();
    }
}

/// Face embedding (feature vector)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceEmbedding {
    /// Embedding ID
    pub id: Uuid,
    /// Feature vector (typically 512 dimensions)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub vector: Vec<f32>,
    /// Source image (base64 or URL)
    pub source_image: Option<String>,
    /// Quality score
    pub quality: f32,
    /// Created at
    pub created_at: DateTime<Utc>,
}

impl FaceEmbedding {
    /// Create new embedding
    pub fn new(vector: Vec<f32>, quality: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            vector,
            source_image: None,
            quality,
            created_at: Utc::now(),
        }
    }

    /// Calculate cosine similarity with another embedding
    pub fn similarity(&self, other: &FaceEmbedding) -> f32 {
        if self.vector.len() != other.vector.len() {
            return 0.0;
        }

        let dot_product: f32 = self
            .vector
            .iter()
            .zip(&other.vector)
            .map(|(a, b)| a * b)
            .sum();

        let norm_a: f32 = self.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = other.vector.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        dot_product / (norm_a * norm_b)
    }
}

/// Face detection (from video/image)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceDetection {
    /// Detection ID
    pub id: Uuid,
    /// Camera ID
    pub camera_id: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Bounding box (x, y, width, height, normalized 0-1)
    pub bbox: Vec<f32>,
    /// Detection confidence
    pub detection_confidence: f32,
    /// Face embedding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<FaceEmbedding>,
    /// Matched person ID
    pub matched_person_id: Option<Uuid>,
    /// Match confidence (similarity score)
    pub match_confidence: Option<f32>,
    /// Match type (if matched)
    pub match_type: Option<WatchlistType>,
    /// Image snapshot (base64 or URL)
    pub image: Option<String>,
    /// Metadata
    pub metadata: serde_json::Value,
}

impl FaceDetection {
    /// Create new detection
    pub fn new(camera_id: String, bbox: Vec<f32>, detection_confidence: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            camera_id,
            timestamp: Utc::now(),
            bbox,
            detection_confidence,
            embedding: None,
            matched_person_id: None,
            match_confidence: None,
            match_type: None,
            image: None,
            metadata: serde_json::json!({}),
        }
    }
}

/// Face recognition statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceStats {
    /// Total detections
    pub total_detections: u64,
    /// Total recognized
    pub total_recognized: u64,
    /// Authorized matches
    pub authorized_matches: u64,
    /// Blocklist matches
    pub blocklist_matches: u64,
    /// Unknown faces
    pub unknown_faces: u64,
    /// Average match confidence
    pub avg_match_confidence: f32,
}

/// Face recognition manager
pub struct FaceManager {
    persons: Arc<RwLock<HashMap<Uuid, RegisteredPerson>>>,
    detections: Arc<RwLock<Vec<FaceDetection>>>,
}

impl FaceManager {
    /// Create new face manager
    pub fn new() -> Self {
        Self {
            persons: Arc::new(RwLock::new(HashMap::new())),
            detections: Arc::new(RwLock::new(Vec::new())),
        }
    }

    // Registered Persons

    /// Add person
    pub async fn add_person(&self, person: RegisteredPerson) {
        let mut persons = self.persons.write().await;
        persons.insert(person.id, person);
    }

    /// Get person by ID
    pub async fn get_person(&self, id: Uuid) -> Option<RegisteredPerson> {
        let persons = self.persons.read().await;
        persons.get(&id).cloned()
    }

    /// Update person
    pub async fn update_person(&self, id: Uuid, mut person: RegisteredPerson) -> Result<(), String> {
        let mut persons = self.persons.write().await;
        if persons.contains_key(&id) {
            person.updated_at = Utc::now();
            persons.insert(id, person);
            Ok(())
        } else {
            Err("Person not found".to_string())
        }
    }

    /// Delete person
    pub async fn delete_person(&self, id: Uuid) -> Result<(), String> {
        let mut persons = self.persons.write().await;
        persons
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| "Person not found".to_string())
    }

    /// List persons
    pub async fn list_persons(&self, watchlist_type: Option<WatchlistType>) -> Vec<RegisteredPerson> {
        let persons = self.persons.read().await;
        persons
            .values()
            .filter(|p| {
                watchlist_type
                    .as_ref()
                    .map_or(true, |wt| &p.watchlist_type == wt)
            })
            .cloned()
            .collect()
    }

    /// Search persons by embedding (face recognition)
    pub async fn search_by_embedding(
        &self,
        embedding: &FaceEmbedding,
        threshold: f32,
    ) -> Vec<(RegisteredPerson, f32)> {
        let persons = self.persons.read().await;
        let mut matches = Vec::new();

        for person in persons.values() {
            if !person.is_active() {
                continue;
            }

            for person_embedding in &person.embeddings {
                let similarity = embedding.similarity(person_embedding);
                if similarity >= threshold {
                    matches.push((person.clone(), similarity));
                    break; // Found match for this person
                }
            }
        }

        // Sort by similarity (descending)
        matches.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        matches
    }

    // Detections

    /// Add detection
    pub async fn add_detection(&self, mut detection: FaceDetection) {
        // Try to match with registered persons
        if let Some(embedding) = &detection.embedding {
            let matches = self.search_by_embedding(embedding, 0.6).await;

            if let Some((person, similarity)) = matches.first() {
                detection.matched_person_id = Some(person.id);
                detection.match_confidence = Some(*similarity);
                detection.match_type = Some(person.watchlist_type.clone());
            }
        }

        let mut detections = self.detections.write().await;
        detections.push(detection);

        // Keep only last 10000 detections
        if detections.len() > 10000 {
            detections.drain(0..1000);
        }
    }

    /// Get detection by ID
    pub async fn get_detection(&self, id: Uuid) -> Option<FaceDetection> {
        let detections = self.detections.read().await;
        detections.iter().find(|d| d.id == id).cloned()
    }

    /// List recent detections
    pub async fn list_detections(&self, limit: usize) -> Vec<FaceDetection> {
        let detections = self.detections.read().await;
        detections.iter().rev().take(limit).cloned().collect()
    }

    /// List detections by person
    pub async fn list_detections_by_person(&self, person_id: Uuid) -> Vec<FaceDetection> {
        let detections = self.detections.read().await;
        detections
            .iter()
            .filter(|d| d.matched_person_id == Some(person_id))
            .cloned()
            .collect()
    }

    /// List detections by camera
    pub async fn list_detections_by_camera(&self, camera_id: &str) -> Vec<FaceDetection> {
        let detections = self.detections.read().await;
        detections
            .iter()
            .filter(|d| d.camera_id == camera_id)
            .cloned()
            .collect()
    }

    /// Get statistics
    pub async fn get_stats(&self) -> FaceStats {
        let detections = self.detections.read().await;

        let total = detections.len() as u64;
        let recognized = detections.iter().filter(|d| d.matched_person_id.is_some()).count() as u64;
        let unknown = total - recognized;

        let authorized = detections
            .iter()
            .filter(|d| d.match_type == Some(WatchlistType::Authorized))
            .count() as u64;
        let blocklist = detections
            .iter()
            .filter(|d| d.match_type == Some(WatchlistType::Blocklist))
            .count() as u64;

        let avg_conf = if recognized > 0 {
            detections
                .iter()
                .filter_map(|d| d.match_confidence)
                .sum::<f32>()
                / recognized as f32
        } else {
            0.0
        };

        FaceStats {
            total_detections: total,
            total_recognized: recognized,
            authorized_matches: authorized,
            blocklist_matches: blocklist,
            unknown_faces: unknown,
            avg_match_confidence: avg_conf,
        }
    }
}

impl Default for FaceManager {
    fn default() -> Self {
        Self::new()
    }
}
