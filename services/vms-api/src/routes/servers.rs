//! Server API routes
//!
//! CRUD operations for streaming servers (nodes)

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    models::server::{CreateServerRequest, Server, ServerPublic, UpdateServerRequest},
    AppState,
};

/// GET /api/v1/servers - List all servers
pub async fn list_servers(State(state): State<AppState>) -> impl IntoResponse {
    match state.server_repo.list().await {
        Ok(servers) => {
            let public_servers: Vec<ServerPublic> =
                servers.into_iter().map(ServerPublic::from).collect();
            (StatusCode::OK, Json(public_servers)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// POST /api/v1/servers - Create new server
pub async fn create_server(
    State(state): State<AppState>,
    Json(req): Json<CreateServerRequest>,
) -> impl IntoResponse {
    let server = Server::new(req.name, req.ip, req.port, req.username, req.password);

    match state.server_repo.create(&server).await {
        Ok(_) => {
            tracing::info!("✅ Server created: {} ({}:{})", server.name, server.ip, server.port);
            (StatusCode::CREATED, Json(ServerPublic::from(server))).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// GET /api/v1/servers/:id - Get server by ID
pub async fn get_server(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.server_repo.get(id).await {
        Ok(Some(server)) => (StatusCode::OK, Json(ServerPublic::from(server))).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Server not found" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// PUT /api/v1/servers/:id - Update server
pub async fn update_server(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateServerRequest>,
) -> impl IntoResponse {
    match state
        .server_repo
        .update(id, req.name, req.ip, req.port, req.username, req.password, req.enabled)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// DELETE /api/v1/servers/:id - Delete server
pub async fn delete_server(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.server_repo.delete(id).await {
        Ok(_) => {
            tracing::info!("🗑️ Server deleted: {}", id);
            StatusCode::NO_CONTENT.into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
