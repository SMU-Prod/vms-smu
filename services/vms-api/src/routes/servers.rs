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

/// Health check response
#[derive(Debug, serde::Serialize)]
pub struct HealthCheckResponse {
    pub server_id: Uuid,
    pub status: String,
    pub online: bool,
    pub latency_ms: Option<u64>,
    pub message: String,
}

/// GET /api/v1/servers/:id/health - Check server health via TCP ping
pub async fn health_check_server(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    // Get server from DB
    let server = match state.server_repo.get(id).await {
        Ok(Some(s)) => s,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(HealthCheckResponse {
                    server_id: id,
                    status: "not_found".to_string(),
                    online: false,
                    latency_ms: None,
                    message: "Servidor não encontrado".to_string(),
                }),
            )
                .into_response()
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(HealthCheckResponse {
                    server_id: id,
                    status: "error".to_string(),
                    online: false,
                    latency_ms: None,
                    message: e.to_string(),
                }),
            )
                .into_response()
        }
    };

    // TCP ping to check server availability
    use std::net::TcpStream;
    use std::time::{Duration, Instant};

    let address = format!("{}:{}", server.ip, server.port);
    let start = Instant::now();
    
    let result = TcpStream::connect_timeout(
        &address.parse().unwrap_or_else(|_| "0.0.0.0:0".parse().unwrap()),
        Duration::from_secs(5),
    );

    match result {
        Ok(_) => {
            let latency = start.elapsed().as_millis() as u64;
            (
                StatusCode::OK,
                Json(HealthCheckResponse {
                    server_id: id,
                    status: "online".to_string(),
                    online: true,
                    latency_ms: Some(latency),
                    message: format!("Servidor {} respondendo em {}ms", server.name, latency),
                }),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::OK,
            Json(HealthCheckResponse {
                server_id: id,
                status: "offline".to_string(),
                online: false,
                latency_ms: None,
                message: format!("Servidor {} offline: {}", server.name, e),
            }),
        )
            .into_response(),
    }
}
