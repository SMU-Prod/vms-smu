//! Indexing module
//! 
//! Creates and reads .vms-idx files for fast seeking

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::info;

/// Index file format version
const INDEX_VERSION: u32 = 1;

/// VMS Index file (.vms-idx)
#[derive(Debug, Serialize, Deserialize)]
pub struct VmsIndex {
    pub version: u32,
    pub camera_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub entries: Vec<IndexEntry>,
}

/// Single frame entry in index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub timestamp: i64,
    pub offset: u64,
    pub size: u32,
    pub is_keyframe: bool,
}

impl VmsIndex {
    pub fn new(camera_id: String, start_time: DateTime<Utc>) -> Self {
        Self {
            version: INDEX_VERSION,
            camera_id,
            start_time,
            end_time: start_time,
            entries: Vec::new(),
        }
    }

    pub async fn save(&self, path: &Path) -> Result<()> {
        let data = bincode::serialize(self)?;
        tokio::fs::write(path, data).await?;
        info!("💾 Saved index: {:?} ({} entries)", path, self.entries.len());
        Ok(())
    }

    pub async fn load(path: &Path) -> Result<Self> {
        let data = tokio::fs::read(path).await?;
        let index: VmsIndex = bincode::deserialize(&data)?;
        Ok(index)
    }
}

/// Index builder
pub struct IndexBuilder {
    camera_id: String,
    start_time: DateTime<Utc>,
    entries: Vec<IndexEntry>,
    current_offset: u64,
}

impl IndexBuilder {
    pub fn new(camera_id: String, start_time: DateTime<Utc>) -> Self {
        Self {
            camera_id,
            start_time,
            entries: Vec::new(),
            current_offset: 0,
        }
    }

    pub fn add_frame(&mut self, timestamp_ms: i64, size: u32, is_keyframe: bool) {
        self.entries.push(IndexEntry {
            timestamp: timestamp_ms,
            offset: self.current_offset,
            size,
            is_keyframe,
        });
        self.current_offset += size as u64;
    }

    pub fn build(self) -> VmsIndex {
        let end_time = if let Some(last) = self.entries.last() {
            DateTime::from_timestamp_millis(last.timestamp)
                .unwrap_or_else(|| Utc::now())
        } else {
            self.start_time
        };

        VmsIndex {
            version: INDEX_VERSION,
            camera_id: self.camera_id,
            start_time: self.start_time,
            end_time,
            entries: self.entries,
        }
    }
}
