//! VMS Analytics Service
//! Video Analytics - Motion, Lines, Areas, Loitering, Counting

use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, warn};
use tracing_subscriber;

mod analytics;

use analytics::{
    AnalyticsEvent, AnalyticsManager, AnalyticsRule, DetectionZone, Direction, Point, RuleType,
    VirtualLine, ZoneType,
};

/// App state
#[derive(Clone)]
struct AppState {
    analytics_manager: Arc<AnalyticsManager>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🚀 VMS Analytics Service starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    let analytics_manager = Arc::new(AnalyticsManager::new());

    let state = AppState {
        analytics_manager: analytics_manager.clone(),
    };

    // Setup NATS subscriber (async task)
    let nats_state = state.clone();
    tokio::spawn(async move {
        if let Err(e) = setup_nats_subscriber(nats_state).await {
            warn!("NATS subscriber error: {}", e);
        }
    });

    // Build router
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/metrics", get(metrics))
        // Zones
        .route("/api/v1/zones", get(list_zones).post(create_zone))
        .route("/api/v1/zones/:id", get(get_zone).delete(delete_zone))
        .route("/api/v1/zones/camera/:camera_id", get(list_camera_zones))
        // Lines
        .route("/api/v1/lines", get(list_lines).post(create_line))
        .route("/api/v1/lines/:id", get(get_line).delete(delete_line))
        .route("/api/v1/lines/camera/:camera_id", get(list_camera_lines))
        // Rules
        .route("/api/v1/rules", get(list_rules).post(create_rule))
        .route("/api/v1/rules/:id", get(get_rule).delete(delete_rule))
        .route("/api/v1/rules/camera/:camera_id", get(list_camera_rules))
        .with_state(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 9099));
    let listener = TcpListener::bind(addr).await?;

    info!("🌐 HTTP API listening on http://{}", addr);
    info!("✅ Service initialized successfully");

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
        })
        .await?;

    info!("👋 Goodbye!");
    Ok(())
}

// ============================================================================
// NATS Integration
// ============================================================================

async fn setup_nats_subscriber(state: AppState) -> Result<()> {
    let nats_url = std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    let client = async_nats::connect(&nats_url).await?;

    info!("✅ Connected to NATS at {}", nats_url);

    // Subscribe to AI detection events
    let mut subscriber = client.subscribe("vms.ai.detections.>").await?;

    info!("📡 Subscribed to vms.ai.detections.>");

    while let Some(msg) = subscriber.next().await {
        if let Ok(detection) = serde_json::from_slice::<serde_json::Value>(&msg.payload) {
            // Process detection through analytics rules
            if let Err(e) = process_detection(&state, detection).await {
                warn!("Error processing detection: {}", e);
            }
        }
    }

    Ok(())
}

async fn process_detection(state: &AppState, detection: serde_json::Value) -> Result<()> {
    // Extract camera_id, bbox, class, etc from detection
    let camera_id = detection["camera_id"].as_str().unwrap_or("unknown");

    info!("📊 Processing detection from camera: {}", camera_id);

    // Get all rules for this camera
    let rules = state
        .analytics_manager
        .list_rules(Some(camera_id.to_string()))
        .await;

    for rule in rules {
        if !rule.enabled || rule.is_in_cooldown() {
            continue;
        }

        // Process based on rule type
        match rule.rule_type {
            RuleType::Motion => {
                // Motion detection already handled by vms-ai
                // Could add zone-based motion filtering here
            }
            RuleType::LineCrossing => {
                // Check if object crossed any lines
                // Would need trajectory tracking (TODO)
            }
            RuleType::AreaIntrusion => {
                // Check if object in intrusion zone
                if let Some(zone_id) = rule.zone_id {
                    if let Some(_zone) = state.analytics_manager.get_zone(zone_id).await {
                        // Check if detection bbox center is in zone
                        // Generate event if intrusion detected
                    }
                }
            }
            RuleType::Loitering => {
                // Track object presence in zone over time
                // Generate event if threshold exceeded
            }
            RuleType::Counting => {
                // Count objects crossing line or in zone
            }
            RuleType::Abandoned | RuleType::Removed => {
                // Track stationary objects
            }
            RuleType::Crowd => {
                // Count people in zone
            }
        }
    }

    Ok(())
}

// ============================================================================
// API Handlers
// ============================================================================

async fn metrics() -> String {
    "# Analytics metrics\nvms_analytics_zones 0\n".to_string()
}

// Zones

#[derive(Deserialize)]
struct CreateZoneRequest {
    name: String,
    camera_id: String,
    points: Vec<Point>,
    zone_type: ZoneType,
}

async fn create_zone(
    State(state): State<AppState>,
    Json(req): Json<CreateZoneRequest>,
) -> Result<Json<DetectionZone>, (StatusCode, String)> {
    if req.points.len() < 3 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Zone must have at least 3 points".to_string(),
        ));
    }

    let zone = DetectionZone::new(req.name, req.camera_id, req.points, req.zone_type);
    state.analytics_manager.add_zone(zone.clone()).await;

    info!("✅ Zone created: {}", zone.name);
    Ok(Json(zone))
}

async fn get_zone(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<DetectionZone>, (StatusCode, String)> {
    state
        .analytics_manager
        .get_zone(id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Zone not found".to_string()))
}

async fn list_zones(State(state): State<AppState>) -> Json<Vec<DetectionZone>> {
    let zones = state.analytics_manager.list_zones(None).await;
    Json(zones)
}

async fn list_camera_zones(
    State(state): State<AppState>,
    Path(camera_id): Path<String>,
) -> Json<Vec<DetectionZone>> {
    let zones = state.analytics_manager.list_zones(Some(camera_id)).await;
    Json(zones)
}

async fn delete_zone(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<String, (StatusCode, String)> {
    state
        .analytics_manager
        .delete_zone(id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;
    Ok("Deleted".to_string())
}

// Lines

#[derive(Deserialize)]
struct CreateLineRequest {
    name: String,
    camera_id: String,
    start: Point,
    end: Point,
    directional: bool,
    expected_direction: Option<Direction>,
}

async fn create_line(
    State(state): State<AppState>,
    Json(req): Json<CreateLineRequest>,
) -> Json<VirtualLine> {
    let mut line = VirtualLine::new(req.name, req.camera_id, req.start, req.end);
    line.directional = req.directional;
    line.expected_direction = req.expected_direction;

    state.analytics_manager.add_line(line.clone()).await;

    info!("✅ Line created: {}", line.name);
    Json(line)
}

async fn get_line(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<VirtualLine>, (StatusCode, String)> {
    state
        .analytics_manager
        .get_line(id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Line not found".to_string()))
}

async fn list_lines(State(state): State<AppState>) -> Json<Vec<VirtualLine>> {
    let lines = state.analytics_manager.list_lines(None).await;
    Json(lines)
}

async fn list_camera_lines(
    State(state): State<AppState>,
    Path(camera_id): Path<String>,
) -> Json<Vec<VirtualLine>> {
    let lines = state.analytics_manager.list_lines(Some(camera_id)).await;
    Json(lines)
}

async fn delete_line(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<String, (StatusCode, String)> {
    state
        .analytics_manager
        .delete_line(id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;
    Ok("Deleted".to_string())
}

// Rules

#[derive(Deserialize)]
struct CreateRuleRequest {
    name: String,
    camera_id: String,
    rule_type: RuleType,
    min_confidence: Option<f32>,
    zone_id: Option<uuid::Uuid>,
    line_id: Option<uuid::Uuid>,
    loitering_threshold_secs: Option<u64>,
    cooldown_secs: Option<u64>,
}

async fn create_rule(
    State(state): State<AppState>,
    Json(req): Json<CreateRuleRequest>,
) -> Json<AnalyticsRule> {
    let mut rule = AnalyticsRule::new(req.name, req.camera_id, req.rule_type);

    if let Some(conf) = req.min_confidence {
        rule.min_confidence = conf;
    }
    rule.zone_id = req.zone_id;
    rule.line_id = req.line_id;
    rule.loitering_threshold_secs = req.loitering_threshold_secs;
    if let Some(cooldown) = req.cooldown_secs {
        rule.cooldown_secs = cooldown;
    }

    state.analytics_manager.add_rule(rule.clone()).await;

    info!("✅ Rule created: {}", rule.name);
    Json(rule)
}

async fn get_rule(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<AnalyticsRule>, (StatusCode, String)> {
    state
        .analytics_manager
        .get_rule(id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Rule not found".to_string()))
}

async fn list_rules(State(state): State<AppState>) -> Json<Vec<AnalyticsRule>> {
    let rules = state.analytics_manager.list_rules(None).await;
    Json(rules)
}

async fn list_camera_rules(
    State(state): State<AppState>,
    Path(camera_id): Path<String>,
) -> Json<Vec<AnalyticsRule>> {
    let rules = state.analytics_manager.list_rules(Some(camera_id)).await;
    Json(rules)
}

async fn delete_rule(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<String, (StatusCode, String)> {
    state
        .analytics_manager
        .delete_rule(id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;
    Ok("Deleted".to_string())
}
