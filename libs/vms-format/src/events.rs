//! Events storage using Parquet format

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIEvent {
    pub timestamp_ms: u64,
    pub event_type: String,
    pub confidence: f32,
    pub metadata: String,
}
