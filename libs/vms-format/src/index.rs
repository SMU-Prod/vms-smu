//! Proprietary index format for fast seeking

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoIndex {
    pub version: u32,
    pub entries: Vec<IndexEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub timestamp_ms: u64,
    pub offset: u64,
    pub is_keyframe: bool,
}
