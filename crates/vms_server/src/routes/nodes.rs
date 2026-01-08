//! Node routes

use axum::{
    extract::{State, Path},
    routing::{get, post},
    Router, Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vms_core::{RegisterNodeRequest, NodeHeartbeat, Node, NodeStatus};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_nodes).post(register_node))
        .route("/:id", get(get_node).delete(delete_node))
        .route("/:id/heartbeat", post(heartbeat))
}

/// Register response
#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub node_id: Uuid,
    pub api_key: String,
}

/// Error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// List all nodes
async fn list_nodes(
    State(state): State<AppState>,
) -> Result<Json<Vec<Node>>, (StatusCode, Json<ErrorResponse>)> {
    let nodes = state.node_repo
        .list()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    Ok(Json(nodes))
}

/// Register new node
async fn register_node(
    State(state): State<AppState>,
    Json(req): Json<RegisterNodeRequest>,
) -> Result<Json<RegisterResponse>, (StatusCode, Json<ErrorResponse>)> {
    let (node_id, api_key) = state.node_repo
        .register(&req)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    tracing::info!("Registered node {} from {}", req.name, req.ip);

    Ok(Json(RegisterResponse { node_id, api_key }))
}

/// Get single node
async fn get_node(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Node>, (StatusCode, Json<ErrorResponse>)> {
    let node = state.node_repo
        .find_by_id(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "Node not found".to_string() })))?;

    Ok(Json(node))
}

/// Delete node
async fn delete_node(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    state.node_repo
        .delete(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    Ok(StatusCode::NO_CONTENT)
}

/// Node heartbeat
async fn heartbeat(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<NodeHeartbeat>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    state.node_repo
        .heartbeat(id, req.status)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    tracing::debug!("Heartbeat from node {}: {:?}", id, req.status);

    Ok(Json(serde_json::json!({ "ok": true })))
}
