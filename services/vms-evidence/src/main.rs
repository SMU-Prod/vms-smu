//! VMS Evidence Service
//! Sistema de ocorrências e evidências (Evidence Management)

use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use futures::StreamExt;
use serde::Deserialize;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, warn};
use tracing_subscriber;

mod evidence;
mod export;

use evidence::{Evidence, EvidenceManager, EvidencePriority, EvidenceStatus};
use export::{EvidenceExporter, ExportRequest};

/// App state
#[derive(Clone)]
struct AppState {
    evidence_manager: Arc<EvidenceManager>,
    exporter: Arc<EvidenceExporter>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🚀 VMS Evidence Service starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Initialize managers
    let evidence_manager = Arc::new(EvidenceManager::new());
    let exporter = Arc::new(EvidenceExporter::new("./exports".to_string()));

    // Create export directory
    std::fs::create_dir_all("./exports").ok();

    let state = AppState {
        evidence_manager,
        exporter,
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
        // Evidence endpoints
        .route("/api/v1/evidences", get(list_evidences).post(create_evidence))
        .route(
            "/api/v1/evidences/:id",
            get(get_evidence).put(update_evidence).delete(delete_evidence),
        )
        .route("/api/v1/evidences/case/:case_number", get(get_evidence_by_case))
        .route("/api/v1/evidences/user/:user_id", get(list_user_evidences))
        .route("/api/v1/evidences/status/:status", get(list_evidences_by_status))
        .route("/api/v1/evidences/search", post(search_evidences))
        // Attachment endpoints
        .route("/api/v1/evidences/:id/attachments", post(add_attachment))
        // Export endpoints
        .route("/api/v1/evidences/:id/export", post(export_evidence))
        // Custody chain
        .route("/api/v1/evidences/:id/custody", get(get_custody_chain))
        .with_state(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 9098));
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

async fn setup_nats_subscriber(_state: AppState) -> Result<()> {
    // Connect to NATS
    let nats_url = std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    let client = async_nats::connect(&nats_url).await?;

    info!("✅ Connected to NATS at {}", nats_url);

    // Subscribe to alarm events to auto-create evidence
    let mut subscriber = client.subscribe("vms.events.alarms.>").await?;

    info!("📡 Subscribed to vms.events.alarms.>");

    while let Some(msg) = subscriber.next().await {
        if let Ok(alarm_data) = serde_json::from_slice::<serde_json::Value>(&msg.payload) {
            info!("📨 Received alarm event: {}", alarm_data);
            // TODO: Auto-create evidence from critical alarms
        }
    }

    Ok(())
}

// ============================================================================
// API Handlers
// ============================================================================

async fn metrics() -> String {
    "# Evidence metrics\nvms_evidence_total 0\n".to_string()
}

// Evidence CRUD

#[derive(Deserialize)]
struct CreateEvidenceRequest {
    title: String,
    description: String,
    evidence_type: String,
    priority: EvidencePriority,
    created_by: uuid::Uuid,
    #[serde(default)]
    tags: Vec<String>,
    location: Option<String>,
}

async fn create_evidence(
    State(state): State<AppState>,
    Json(req): Json<CreateEvidenceRequest>,
) -> Result<Json<Evidence>, (StatusCode, String)> {
    let mut evidence = Evidence::new(
        req.title,
        req.description,
        req.evidence_type,
        req.priority,
        req.created_by,
    );

    evidence.tags = req.tags;
    evidence.location = req.location;
    evidence.status = EvidenceStatus::Active;

    state.evidence_manager.add(evidence.clone()).await;

    info!("✅ Evidence created: {}", evidence.case_number);
    Ok(Json(evidence))
}

async fn get_evidence(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Evidence>, (StatusCode, String)> {
    state
        .evidence_manager
        .get(id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Evidence not found".to_string()))
}

async fn get_evidence_by_case(
    State(state): State<AppState>,
    Path(case_number): Path<String>,
) -> Result<Json<Evidence>, (StatusCode, String)> {
    state
        .evidence_manager
        .get_by_case_number(&case_number)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Evidence not found".to_string()))
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ListQuery {
    #[serde(default)]
    limit: Option<usize>,
    #[serde(default)]
    offset: Option<usize>,
}

async fn list_evidences(
    State(state): State<AppState>,
    Query(_query): Query<ListQuery>,
) -> Json<Vec<Evidence>> {
    let evidences = state.evidence_manager.list_all().await;
    Json(evidences)
}

async fn list_user_evidences(
    State(state): State<AppState>,
    Path(user_id): Path<uuid::Uuid>,
) -> Json<Vec<Evidence>> {
    let evidences = state.evidence_manager.list_by_user(user_id).await;
    Json(evidences)
}

async fn list_evidences_by_status(
    State(state): State<AppState>,
    Path(status): Path<String>,
) -> Result<Json<Vec<Evidence>>, (StatusCode, String)> {
    let status = match status.as_str() {
        "draft" => EvidenceStatus::Draft,
        "active" => EvidenceStatus::Active,
        "exported" => EvidenceStatus::Exported,
        "archived" => EvidenceStatus::Archived,
        _ => return Err((StatusCode::BAD_REQUEST, "Invalid status".to_string())),
    };

    let evidences = state.evidence_manager.list_by_status(status).await;
    Ok(Json(evidences))
}

async fn update_evidence(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(evidence): Json<Evidence>,
) -> Result<Json<Evidence>, (StatusCode, String)> {
    state
        .evidence_manager
        .update(id, evidence.clone())
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;
    Ok(Json(evidence))
}

async fn delete_evidence(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<String, (StatusCode, String)> {
    // Get current user ID from auth (hardcoded for now)
    let user_id = uuid::Uuid::new_v4();

    state
        .evidence_manager
        .delete(id, user_id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;
    Ok("Deleted".to_string())
}

// Search

#[derive(Deserialize)]
struct SearchRequest {
    tags: Vec<String>,
}

async fn search_evidences(
    State(state): State<AppState>,
    Json(req): Json<SearchRequest>,
) -> Json<Vec<Evidence>> {
    let evidences = state.evidence_manager.search_by_tags(req.tags).await;
    Json(evidences)
}

// Attachments

#[derive(Deserialize)]
struct AddAttachmentRequest {
    attachment_type: evidence::EvidenceType,
    file_name: String,
    file_path: String,
    file_size: u64,
    sha256: String,
    camera_id: Option<String>,
}

async fn add_attachment(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(req): Json<AddAttachmentRequest>,
) -> Result<Json<Evidence>, (StatusCode, String)> {
    let mut evidence = state
        .evidence_manager
        .get(id)
        .await
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Evidence not found".to_string()))?;

    let mut attachment = evidence::EvidenceAttachment::new(
        req.attachment_type,
        req.file_name,
        req.file_path,
        req.file_size,
        req.sha256,
    );

    attachment.camera_id = req.camera_id;
    evidence.add_attachment(attachment);

    state
        .evidence_manager
        .update(id, evidence.clone())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(evidence))
}

// Export

async fn export_evidence(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(req): Json<ExportRequest>,
) -> Result<Json<export::ExportResult>, (StatusCode, String)> {
    let evidence = state
        .evidence_manager
        .get(id)
        .await
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Evidence not found".to_string()))?;

    let result = state
        .exporter
        .export(&evidence, &req)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Add custody entry
    let mut evidence_mut = evidence.clone();
    evidence_mut.add_custody_entry(
        "exported".to_string(),
        uuid::Uuid::new_v4(), // TODO: Get from auth
        format!("Exported as {:?}", req.format),
    );
    state.evidence_manager.update(id, evidence_mut).await.ok();

    info!("📦 Evidence exported: {} -> {}", evidence.case_number, result.file_path);
    Ok(Json(result))
}

// Custody chain

async fn get_custody_chain(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Vec<evidence::CustodyEntry>>, (StatusCode, String)> {
    let evidence = state
        .evidence_manager
        .get(id)
        .await
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Evidence not found".to_string()))?;

    Ok(Json(evidence.custody_chain))
}
