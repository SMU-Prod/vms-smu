//! Video Analytics structures and definitions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Point in 2D space (normalized coordinates 0.0 - 1.0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Calculate distance to another point
    pub fn distance(&self, other: &Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Bounding box
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl BoundingBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    /// Get center point
    pub fn center(&self) -> Point {
        Point {
            x: self.x + self.width / 2.0,
            y: self.y + self.height / 2.0,
        }
    }

    /// Check if point is inside
    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
}

/// Detection zone (polygon)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionZone {
    /// Zone ID
    pub id: Uuid,
    /// Zone name
    pub name: String,
    /// Camera ID
    pub camera_id: String,
    /// Polygon points (at least 3)
    pub points: Vec<Point>,
    /// Zone type
    pub zone_type: ZoneType,
    /// Enabled
    pub enabled: bool,
    /// Created at
    pub created_at: DateTime<Utc>,
}

impl DetectionZone {
    pub fn new(name: String, camera_id: String, points: Vec<Point>, zone_type: ZoneType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            camera_id,
            points,
            zone_type,
            enabled: true,
            created_at: Utc::now(),
        }
    }

    /// Check if point is inside polygon (ray casting algorithm)
    pub fn contains(&self, point: &Point) -> bool {
        if self.points.len() < 3 {
            return false;
        }

        let mut inside = false;
        let n = self.points.len();

        for i in 0..n {
            let j = (i + 1) % n;
            let pi = &self.points[i];
            let pj = &self.points[j];

            if ((pi.y > point.y) != (pj.y > point.y))
                && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
            {
                inside = !inside;
            }
        }

        inside
    }
}

/// Zone type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ZoneType {
    /// Intrusion detection
    Intrusion,
    /// Include zone (only detect inside)
    Include,
    /// Exclude zone (ignore detections inside)
    Exclude,
    /// Loitering detection
    Loitering,
    /// Counting zone
    Counting,
}

/// Virtual line for line crossing detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualLine {
    /// Line ID
    pub id: Uuid,
    /// Line name
    pub name: String,
    /// Camera ID
    pub camera_id: String,
    /// Start point
    pub start: Point,
    /// End point
    pub end: Point,
    /// Direction matters (true = only one direction triggers)
    pub directional: bool,
    /// Expected direction (if directional)
    pub expected_direction: Option<Direction>,
    /// Enabled
    pub enabled: bool,
    /// Created at
    pub created_at: DateTime<Utc>,
}

impl VirtualLine {
    pub fn new(name: String, camera_id: String, start: Point, end: Point) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            camera_id,
            start,
            end,
            directional: false,
            expected_direction: None,
            enabled: true,
            created_at: Utc::now(),
        }
    }

    /// Check if line segment crosses this line
    pub fn crosses(&self, p1: &Point, p2: &Point) -> Option<Direction> {
        // Line segment intersection using cross product
        let d1 = self.cross_product(p1, p2, &self.start);
        let d2 = self.cross_product(p1, p2, &self.end);
        let d3 = self.cross_product(&self.start, &self.end, p1);
        let d4 = self.cross_product(&self.start, &self.end, p2);

        if d1 * d2 < 0.0 && d3 * d4 < 0.0 {
            // Lines intersect - determine direction
            if d3 > 0.0 {
                Some(Direction::LeftToRight)
            } else {
                Some(Direction::RightToLeft)
            }
        } else {
            None
        }
    }

    fn cross_product(&self, p1: &Point, p2: &Point, p3: &Point) -> f32 {
        (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x)
    }
}

/// Direction for line crossing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

/// Analytics rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsRule {
    /// Rule ID
    pub id: Uuid,
    /// Rule name
    pub name: String,
    /// Camera ID
    pub camera_id: String,
    /// Rule type
    pub rule_type: RuleType,
    /// Enabled
    pub enabled: bool,
    /// Minimum confidence
    pub min_confidence: f32,
    /// Minimum object size (as % of frame)
    pub min_object_size: f32,
    /// Maximum object size (as % of frame)
    pub max_object_size: f32,
    /// Object classes to detect (empty = all)
    pub object_classes: Vec<String>,
    /// Zone ID (if applicable)
    pub zone_id: Option<Uuid>,
    /// Line ID (if applicable)
    pub line_id: Option<Uuid>,
    /// Loitering threshold (seconds)
    pub loitering_threshold_secs: Option<u64>,
    /// Cooldown (seconds between triggers)
    pub cooldown_secs: u64,
    /// Last triggered
    #[serde(skip)]
    pub last_triggered: Option<DateTime<Utc>>,
    /// Created at
    pub created_at: DateTime<Utc>,
}

impl AnalyticsRule {
    pub fn new(name: String, camera_id: String, rule_type: RuleType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            camera_id,
            rule_type,
            enabled: true,
            min_confidence: 0.5,
            min_object_size: 0.0,
            max_object_size: 1.0,
            object_classes: Vec::new(),
            zone_id: None,
            line_id: None,
            loitering_threshold_secs: None,
            cooldown_secs: 0,
            last_triggered: None,
            created_at: Utc::now(),
        }
    }

    /// Check if rule is in cooldown
    pub fn is_in_cooldown(&self) -> bool {
        if let Some(last) = self.last_triggered {
            let elapsed = Utc::now().signed_duration_since(last);
            elapsed.num_seconds() < self.cooldown_secs as i64
        } else {
            false
        }
    }
}

/// Rule type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RuleType {
    /// Motion detection
    Motion,
    /// Line crossing
    LineCrossing,
    /// Area intrusion
    AreaIntrusion,
    /// Loitering
    Loitering,
    /// Object counting
    Counting,
    /// Object abandoned
    Abandoned,
    /// Object removed
    Removed,
    /// Crowd detection
    Crowd,
}

/// Analytics event (output)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    /// Event ID
    pub id: Uuid,
    /// Camera ID
    pub camera_id: String,
    /// Rule ID
    pub rule_id: Uuid,
    /// Event type
    pub event_type: RuleType,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Confidence
    pub confidence: f32,
    /// Bounding box (if applicable)
    pub bbox: Option<BoundingBox>,
    /// Point (if applicable)
    pub point: Option<Point>,
    /// Direction (if line crossing)
    pub direction: Option<Direction>,
    /// Object class
    pub object_class: Option<String>,
    /// Metadata
    pub metadata: serde_json::Value,
}

impl AnalyticsEvent {
    pub fn new(camera_id: String, rule_id: Uuid, event_type: RuleType) -> Self {
        Self {
            id: Uuid::new_v4(),
            camera_id,
            rule_id,
            event_type,
            timestamp: Utc::now(),
            confidence: 1.0,
            bbox: None,
            point: None,
            direction: None,
            object_class: None,
            metadata: serde_json::json!({}),
        }
    }
}

/// Analytics manager
pub struct AnalyticsManager {
    zones: Arc<RwLock<HashMap<Uuid, DetectionZone>>>,
    lines: Arc<RwLock<HashMap<Uuid, VirtualLine>>>,
    rules: Arc<RwLock<HashMap<Uuid, AnalyticsRule>>>,
}

impl AnalyticsManager {
    pub fn new() -> Self {
        Self {
            zones: Arc::new(RwLock::new(HashMap::new())),
            lines: Arc::new(RwLock::new(HashMap::new())),
            rules: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Zones
    pub async fn add_zone(&self, zone: DetectionZone) {
        let mut zones = self.zones.write().await;
        zones.insert(zone.id, zone);
    }

    pub async fn get_zone(&self, id: Uuid) -> Option<DetectionZone> {
        let zones = self.zones.read().await;
        zones.get(&id).cloned()
    }

    pub async fn list_zones(&self, camera_id: Option<String>) -> Vec<DetectionZone> {
        let zones = self.zones.read().await;
        zones
            .values()
            .filter(|z| camera_id.as_ref().map_or(true, |cid| &z.camera_id == cid))
            .cloned()
            .collect()
    }

    pub async fn delete_zone(&self, id: Uuid) -> Result<(), String> {
        let mut zones = self.zones.write().await;
        zones.remove(&id).map(|_| ()).ok_or("Zone not found".to_string())
    }

    // Lines
    pub async fn add_line(&self, line: VirtualLine) {
        let mut lines = self.lines.write().await;
        lines.insert(line.id, line);
    }

    pub async fn get_line(&self, id: Uuid) -> Option<VirtualLine> {
        let lines = self.lines.read().await;
        lines.get(&id).cloned()
    }

    pub async fn list_lines(&self, camera_id: Option<String>) -> Vec<VirtualLine> {
        let lines = self.lines.read().await;
        lines
            .values()
            .filter(|l| camera_id.as_ref().map_or(true, |cid| &l.camera_id == cid))
            .cloned()
            .collect()
    }

    pub async fn delete_line(&self, id: Uuid) -> Result<(), String> {
        let mut lines = self.lines.write().await;
        lines.remove(&id).map(|_| ()).ok_or("Line not found".to_string())
    }

    // Rules
    pub async fn add_rule(&self, rule: AnalyticsRule) {
        let mut rules = self.rules.write().await;
        rules.insert(rule.id, rule);
    }

    pub async fn get_rule(&self, id: Uuid) -> Option<AnalyticsRule> {
        let rules = self.rules.read().await;
        rules.get(&id).cloned()
    }

    pub async fn list_rules(&self, camera_id: Option<String>) -> Vec<AnalyticsRule> {
        let rules = self.rules.read().await;
        rules
            .values()
            .filter(|r| camera_id.as_ref().map_or(true, |cid| &r.camera_id == cid))
            .cloned()
            .collect()
    }

    pub async fn delete_rule(&self, id: Uuid) -> Result<(), String> {
        let mut rules = self.rules.write().await;
        rules.remove(&id).map(|_| ()).ok_or("Rule not found".to_string())
    }
}

impl Default for AnalyticsManager {
    fn default() -> Self {
        Self::new()
    }
}
