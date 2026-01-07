//! VMS LPR Service
//! License Plate Recognition and Management

use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, warn};
use tracing_subscriber;

mod lpr;

use lpr::{LprDetection, LprManager, LprStats, PlateFormat, PlateListType, RegisteredPlate};

/// App state
#[derive(Clone)]
struct AppState {
    lpr_manager: Arc<LprManager>,
    nats_client: Option<Arc<async_nats::Client>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🚀 VMS LPR Service starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    let lpr_manager = Arc::new(LprManager::new());

    // Connect to NATS (optional)
    let nats_client = match connect_nats().await {
        Ok(client) => {
            let client = Arc::new(client);
            Some(client.clone())
        }
        Err(e) => {
            warn!("Failed to connect to NATS: {}", e);
            None
        }
    };

    let state = AppState {
        lpr_manager: lpr_manager.clone(),
        nats_client: nats_client.clone(),
    };

    // Setup NATS subscriber (if connected)
    if let Some(client) = nats_client {
        let nats_state = state.clone();
        tokio::spawn(async move {
            if let Err(e) = setup_nats_subscriber(nats_state, client).await {
                warn!("NATS subscriber error: {}", e);
            }
        });
    }

    // Build router
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/metrics", get(metrics))
        .route("/api/v1/stats", get(get_stats))
        // Registered plates
        .route("/api/v1/plates", get(list_plates).post(register_plate))
        .route(
            "/api/v1/plates/:id",
            get(get_plate).put(update_plate).delete(delete_plate),
        )
        .route("/api/v1/plates/lookup/:plate_number", get(lookup_plate))
        // Detections
        .route("/api/v1/detections", get(list_detections).post(add_detection))
        .route("/api/v1/detections/:id", get(get_detection))
        .route("/api/v1/detections/plate/:plate_number", get(list_detections_by_plate))
        .route("/api/v1/detections/camera/:camera_id", get(list_detections_by_camera))
        .with_state(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 9100));
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

async fn connect_nats() -> Result<async_nats::Client> {
    let nats_url =
        std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    let client = async_nats::connect(&nats_url).await?;
    info!("✅ Connected to NATS at {}", nats_url);
    Ok(client)
}

async fn setup_nats_subscriber(
    state: AppState,
    client: Arc<async_nats::Client>,
) -> Result<()> {
    // Subscribe to LPR detections from vms-ai or vms-ingest
    let mut subscriber = client.subscribe("vms.lpr.detections.>").await?;

    info!("📡 Subscribed to vms.lpr.detections.>");

    while let Some(msg) = subscriber.next().await {
        if let Ok(detection_data) = serde_json::from_slice::<serde_json::Value>(&msg.payload) {
            info!("📨 Received LPR detection: {:?}", detection_data);

            // Process detection
            if let Err(e) = process_lpr_detection(&state, detection_data).await {
                warn!("Error processing LPR detection: {}", e);
            }
        }
    }

    Ok(())
}

async fn process_lpr_detection(
    state: &AppState,
    data: serde_json::Value,
) -> Result<()> {
    // Extract plate info from detection
    let plate_number = data["plate"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing plate"))?;
    let camera_id = data["camera_id"]
        .as_str()
        .unwrap_or("unknown");
    let confidence = data["confidence"]
        .as_f64()
        .unwrap_or(0.0) as f32;

    let detection = LprDetection::new(
        plate_number.to_string(),
        camera_id.to_string(),
        confidence,
    );

    state.lpr_manager.add_detection(detection.clone()).await;

    // If blocklist match, publish alert
    if detection.match_type == Some(PlateListType::Blocklist) {
        info!("🚨 BLOCKLIST MATCH: {}", plate_number);

        if let Some(nats) = &state.nats_client {
            let alert = serde_json::json!({
                "type": "blocklist_match",
                "plate": plate_number,
                "camera_id": camera_id,
                "timestamp": detection.timestamp,
            });

            nats.publish("vms.events.lpr.blocklist", alert.to_string().into())
                .await?;
        }
    }

    Ok(())
}

// ============================================================================
// API Handlers
// ============================================================================

async fn metrics() -> String {
    "# LPR metrics\nvms_lpr_detections_total 0\n".to_string()
}

async fn get_stats(State(state): State<AppState>) -> Json<LprStats> {
    let stats = state.lpr_manager.get_stats().await;
    Json(stats)
}

// Registered Plates

#[derive(Deserialize)]
struct RegisterPlateRequest {
    plate_number: String,
    list_type: PlateListType,
    format: PlateFormat,
    created_by: uuid::Uuid,
    owner_name: Option<String>,
    vehicle_description: Option<String>,
    notes: Option<String>,
}

async fn register_plate(
    State(state): State<AppState>,
    Json(req): Json<RegisterPlateRequest>,
) -> Result<Json<RegisteredPlate>, (StatusCode, String)> {
    let mut plate = RegisteredPlate::new(req.plate_number, req.list_type, req.format, req.created_by);

    plate.owner_name = req.owner_name;
    plate.vehicle_description = req.vehicle_description;
    plate.notes = req.notes;

    state.lpr_manager.add_plate(plate.clone()).await;

    info!("✅ Plate registered: {}", plate.plate_number);
    Ok(Json(plate))
}

async fn get_plate(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<RegisteredPlate>, (StatusCode, String)> {
    state
        .lpr_manager
        .get_plate(id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Plate not found".to_string()))
}

async fn lookup_plate(
    State(state): State<AppState>,
    Path(plate_number): Path<String>,
) -> Result<Json<RegisteredPlate>, (StatusCode, String)> {
    state
        .lpr_manager
        .get_plate_by_number(&plate_number)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Plate not found".to_string()))
}

#[derive(Deserialize)]
struct ListPlatesQuery {
    list_type: Option<PlateListType>,
}

async fn list_plates(
    State(state): State<AppState>,
    Query(query): Query<ListPlatesQuery>,
) -> Json<Vec<RegisteredPlate>> {
    let plates = state.lpr_manager.list_plates(query.list_type).await;
    Json(plates)
}

async fn update_plate(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(plate): Json<RegisteredPlate>,
) -> Result<Json<RegisteredPlate>, (StatusCode, String)> {
    state
        .lpr_manager
        .update_plate(id, plate.clone())
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;
    Ok(Json(plate))
}

async fn delete_plate(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<String, (StatusCode, String)> {
    state
        .lpr_manager
        .delete_plate(id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;
    Ok("Deleted".to_string())
}

// Detections

async fn add_detection(
    State(state): State<AppState>,
    Json(detection): Json<LprDetection>,
) -> Json<LprDetection> {
    state.lpr_manager.add_detection(detection.clone()).await;
    info!("📸 LPR detection: {}", detection.plate_number);
    Json(detection)
}

async fn get_detection(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<LprDetection>, (StatusCode, String)> {
    state
        .lpr_manager
        .get_detection(id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Detection not found".to_string()))
}

#[derive(Deserialize)]
struct ListDetectionsQuery {
    #[serde(default = "default_limit")]
    limit: usize,
}

fn default_limit() -> usize {
    100
}

async fn list_detections(
    State(state): State<AppState>,
    Query(query): Query<ListDetectionsQuery>,
) -> Json<Vec<LprDetection>> {
    let detections = state.lpr_manager.list_detections(query.limit).await;
    Json(detections)
}

async fn list_detections_by_plate(
    State(state): State<AppState>,
    Path(plate_number): Path<String>,
) -> Json<Vec<LprDetection>> {
    let detections = state
        .lpr_manager
        .list_detections_by_plate(&plate_number)
        .await;
    Json(detections)
}

async fn list_detections_by_camera(
    State(state): State<AppState>,
    Path(camera_id): Path<String>,
) -> Json<Vec<LprDetection>> {
    let detections = state
        .lpr_manager
        .list_detections_by_camera(&camera_id)
        .await;
    Json(detections)
}
