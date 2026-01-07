//! VMS Face Service
//! Face Recognition and Access Control

use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use futures::StreamExt;
use serde::Deserialize;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, warn};
use tracing_subscriber;

mod face;

use face::{FaceDetection, FaceEmbedding, FaceManager, FaceStats, RegisteredPerson, WatchlistType};

/// App state
#[derive(Clone)]
struct AppState {
    face_manager: Arc<FaceManager>,
    nats_client: Option<Arc<async_nats::Client>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🚀 VMS Face Service starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    let face_manager = Arc::new(FaceManager::new());

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
        face_manager: face_manager.clone(),
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
        // Persons
        .route("/api/v1/persons", get(list_persons).post(register_person))
        .route(
            "/api/v1/persons/:id",
            get(get_person).put(update_person).delete(delete_person),
        )
        .route("/api/v1/persons/:id/embeddings", post(add_embedding))
        .route("/api/v1/search", post(search_by_face))
        // Detections
        .route("/api/v1/detections", get(list_detections).post(add_detection))
        .route("/api/v1/detections/:id", get(get_detection))
        .route("/api/v1/detections/person/:person_id", get(list_detections_by_person))
        .route("/api/v1/detections/camera/:camera_id", get(list_detections_by_camera))
        .with_state(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 9101));
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

async fn setup_nats_subscriber(state: AppState, client: Arc<async_nats::Client>) -> Result<()> {
    // Subscribe to face detections from vms-ai
    let mut subscriber = client.subscribe("vms.face.detections.>").await?;

    info!("📡 Subscribed to vms.face.detections.>");

    while let Some(msg) = subscriber.next().await {
        if let Ok(detection_data) = serde_json::from_slice::<serde_json::Value>(&msg.payload) {
            info!("📨 Received face detection");

            // Process detection
            if let Err(e) = process_face_detection(&state, detection_data).await {
                warn!("Error processing face detection: {}", e);
            }
        }
    }

    Ok(())
}

async fn process_face_detection(state: &AppState, data: serde_json::Value) -> Result<()> {
    let camera_id = data["camera_id"].as_str().unwrap_or("unknown");
    let bbox = data["bbox"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_f64().map(|f| f as f32))
                .collect::<Vec<f32>>()
        })
        .unwrap_or_default();
    let confidence = data["confidence"].as_f64().unwrap_or(0.0) as f32;

    let mut detection = FaceDetection::new(camera_id.to_string(), bbox, confidence);

    // Extract embedding if available
    if let Some(embedding_array) = data["embedding"].as_array() {
        let vector: Vec<f32> = embedding_array
            .iter()
            .filter_map(|v| v.as_f64().map(|f| f as f32))
            .collect();

        if !vector.is_empty() {
            detection.embedding = Some(FaceEmbedding::new(vector, 1.0));
        }
    }

    state.face_manager.add_detection(detection.clone()).await;

    // If blocklist match, publish alert
    if detection.match_type == Some(WatchlistType::Blocklist) {
        info!("🚨 BLOCKLIST MATCH - Face detected");

        if let Some(nats) = &state.nats_client {
            let alert = serde_json::json!({
                "type": "blocklist_face_match",
                "person_id": detection.matched_person_id,
                "camera_id": camera_id,
                "confidence": detection.match_confidence,
                "timestamp": detection.timestamp,
            });

            nats.publish("vms.events.face.blocklist", alert.to_string().into())
                .await?;
        }
    }

    Ok(())
}

// ============================================================================
// API Handlers
// ============================================================================

async fn metrics() -> String {
    "# Face metrics\nvms_face_detections_total 0\n".to_string()
}

async fn get_stats(State(state): State<AppState>) -> Json<FaceStats> {
    let stats = state.face_manager.get_stats().await;
    Json(stats)
}

// Persons

#[derive(Deserialize)]
struct RegisterPersonRequest {
    name: String,
    watchlist_type: WatchlistType,
    created_by: uuid::Uuid,
    document_id: Option<String>,
    notes: Option<String>,
    department: Option<String>,
}

async fn register_person(
    State(state): State<AppState>,
    Json(req): Json<RegisterPersonRequest>,
) -> Result<Json<RegisteredPerson>, (StatusCode, String)> {
    let mut person = RegisteredPerson::new(req.name, req.watchlist_type, req.created_by);

    person.document_id = req.document_id;
    person.notes = req.notes;
    person.department = req.department;

    state.face_manager.add_person(person.clone()).await;

    info!("✅ Person registered: {}", person.name);
    Ok(Json(person))
}

async fn get_person(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<RegisteredPerson>, (StatusCode, String)> {
    state
        .face_manager
        .get_person(id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Person not found".to_string()))
}

#[derive(Deserialize)]
struct ListPersonsQuery {
    watchlist_type: Option<WatchlistType>,
}

async fn list_persons(
    State(state): State<AppState>,
    Query(query): Query<ListPersonsQuery>,
) -> Json<Vec<RegisteredPerson>> {
    let persons = state.face_manager.list_persons(query.watchlist_type).await;
    Json(persons)
}

async fn update_person(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(person): Json<RegisteredPerson>,
) -> Result<Json<RegisteredPerson>, (StatusCode, String)> {
    state
        .face_manager
        .update_person(id, person.clone())
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;
    Ok(Json(person))
}

async fn delete_person(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<String, (StatusCode, String)> {
    state
        .face_manager
        .delete_person(id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;
    Ok("Deleted".to_string())
}

#[derive(Deserialize)]
struct AddEmbeddingRequest {
    vector: Vec<f32>,
    quality: f32,
    source_image: Option<String>,
}

async fn add_embedding(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(req): Json<AddEmbeddingRequest>,
) -> Result<Json<RegisteredPerson>, (StatusCode, String)> {
    let mut person = state
        .face_manager
        .get_person(id)
        .await
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Person not found".to_string()))?;

    let mut embedding = FaceEmbedding::new(req.vector, req.quality);
    embedding.source_image = req.source_image;

    person.add_embedding(embedding);

    state
        .face_manager
        .update_person(id, person.clone())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(person))
}

#[derive(Deserialize)]
struct SearchRequest {
    vector: Vec<f32>,
    threshold: Option<f32>,
}

async fn search_by_face(
    State(state): State<AppState>,
    Json(req): Json<SearchRequest>,
) -> Json<Vec<(RegisteredPerson, f32)>> {
    let embedding = FaceEmbedding::new(req.vector, 1.0);
    let threshold = req.threshold.unwrap_or(0.6);

    let matches = state
        .face_manager
        .search_by_embedding(&embedding, threshold)
        .await;

    Json(matches)
}

// Detections

async fn add_detection(
    State(state): State<AppState>,
    Json(detection): Json<FaceDetection>,
) -> Json<FaceDetection> {
    state.face_manager.add_detection(detection.clone()).await;
    info!("👤 Face detection");
    Json(detection)
}

async fn get_detection(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<FaceDetection>, (StatusCode, String)> {
    state
        .face_manager
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
) -> Json<Vec<FaceDetection>> {
    let detections = state.face_manager.list_detections(query.limit).await;
    Json(detections)
}

async fn list_detections_by_person(
    State(state): State<AppState>,
    Path(person_id): Path<uuid::Uuid>,
) -> Json<Vec<FaceDetection>> {
    let detections = state
        .face_manager
        .list_detections_by_person(person_id)
        .await;
    Json(detections)
}

async fn list_detections_by_camera(
    State(state): State<AppState>,
    Path(camera_id): Path<String>,
) -> Json<Vec<FaceDetection>> {
    let detections = state
        .face_manager
        .list_detections_by_camera(&camera_id)
        .await;
    Json(detections)
}
