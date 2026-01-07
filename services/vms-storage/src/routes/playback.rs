//! Playback Routes
//! Rotas HTTP para APIs de playback

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::playback::{
    Bookmark, BookmarkManager, CreateBookmarkRequest, PlaybackStreamer, Timeline,
    TimelineBuilder, TimelineResolution, UpdateBookmarkRequest,
};

/// Estado compartilhado das rotas de playback
#[derive(Clone)]
pub struct PlaybackState {
    pub timeline_builder: Arc<TimelineBuilder>,
    pub streamer: Arc<PlaybackStreamer>,
    pub bookmark_manager: Arc<BookmarkManager>,
}

/// Query params para timeline
#[derive(Debug, Deserialize)]
pub struct TimelineQuery {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    #[serde(default)]
    pub resolution: Option<String>,
}

/// Query params para streaming
#[derive(Debug, Deserialize)]
pub struct StreamQuery {
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    #[serde(default = "default_speed")]
    pub speed: f32,
}

fn default_speed() -> f32 {
    1.0
}

/// Query params para busca de bookmarks
#[derive(Debug, Deserialize)]
pub struct BookmarkQuery {
    pub camera_id: Option<String>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub tags: Option<String>, // Comma-separated tags
}

/// GET /api/v1/recordings/:camera_id/timeline
async fn get_timeline(
    State(state): State<PlaybackState>,
    Path(camera_id): Path<String>,
    Query(query): Query<TimelineQuery>,
) -> Result<Json<Timeline>, (StatusCode, String)> {
    tracing::info!(
        "GET /api/v1/recordings/{}/timeline?start={}&end={}",
        camera_id,
        query.start,
        query.end
    );

    let resolution = match query.resolution.as_deref() {
        Some("1s") => TimelineResolution::OneSecond,
        Some("10s") => TimelineResolution::TenSeconds,
        Some("1m") | None => TimelineResolution::OneMinute,
        Some("10m") => TimelineResolution::TenMinutes,
        Some("1h") => TimelineResolution::OneHour,
        Some(other) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Invalid resolution: {}", other),
            ))
        }
    };

    let timeline = state
        .timeline_builder
        .build_timeline(&camera_id, query.start, query.end, resolution)
        .await
        .map_err(|e| {
            tracing::error!("Failed to build timeline: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok(Json(timeline))
}

/// GET /api/v1/recordings/:camera_id/stream
async fn stream_recording(
    State(state): State<PlaybackState>,
    Path(camera_id): Path<String>,
    Query(query): Query<StreamQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    tracing::info!(
        "GET /api/v1/recordings/{}/stream?start={}&speed={}",
        camera_id,
        query.start,
        query.speed
    );

    let response = state
        .streamer
        .stream_recording(&camera_id, query.start, query.end, query.speed)
        .await
        .map_err(|e| {
            tracing::error!("Failed to stream recording: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok(response)
}

/// POST /api/v1/bookmarks
async fn create_bookmark(
    State(state): State<PlaybackState>,
    Json(request): Json<CreateBookmarkRequest>,
) -> Result<(StatusCode, Json<Bookmark>), (StatusCode, String)> {
    tracing::info!("POST /api/v1/bookmarks for camera {}", request.camera_id);

    let bookmark = state
        .bookmark_manager
        .create_bookmark(request)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create bookmark: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok((StatusCode::CREATED, Json(bookmark)))
}

/// GET /api/v1/bookmarks
async fn list_bookmarks(
    State(state): State<PlaybackState>,
    Query(query): Query<BookmarkQuery>,
) -> Result<Json<Vec<Bookmark>>, (StatusCode, String)> {
    tracing::info!("GET /api/v1/bookmarks");

    let bookmarks = if let (Some(camera_id), Some(start), Some(end)) =
        (query.camera_id, query.start, query.end)
    {
        // Busca por câmera e intervalo de tempo
        state
            .bookmark_manager
            .list_bookmarks(&camera_id, start, end)
            .await
            .map_err(|e| {
                tracing::error!("Failed to list bookmarks: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            })?
    } else if let Some(tags_str) = query.tags {
        // Busca por tags
        let tags: Vec<String> = tags_str.split(',').map(|s| s.trim().to_string()).collect();
        state.bookmark_manager.search_by_tags(&tags).await.map_err(|e| {
            tracing::error!("Failed to search bookmarks by tags: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?
    } else {
        // Se não especificou filtros, retorna erro
        return Err((
            StatusCode::BAD_REQUEST,
            "Must specify camera_id with start/end, or tags".to_string(),
        ));
    };

    Ok(Json(bookmarks))
}

/// GET /api/v1/bookmarks/:id
async fn get_bookmark(
    State(state): State<PlaybackState>,
    Path(id): Path<String>,
) -> Result<Json<Bookmark>, (StatusCode, String)> {
    tracing::info!("GET /api/v1/bookmarks/{}", id);

    let bookmark = state
        .bookmark_manager
        .get_bookmark(&id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get bookmark: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Bookmark not found".to_string()))?;

    Ok(Json(bookmark))
}

/// PUT /api/v1/bookmarks/:id
async fn update_bookmark(
    State(state): State<PlaybackState>,
    Path(id): Path<String>,
    Json(request): Json<UpdateBookmarkRequest>,
) -> Result<Json<Bookmark>, (StatusCode, String)> {
    tracing::info!("PUT /api/v1/bookmarks/{}", id);

    let bookmark = state
        .bookmark_manager
        .update_bookmark(&id, request)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update bookmark: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Bookmark not found".to_string()))?;

    Ok(Json(bookmark))
}

/// DELETE /api/v1/bookmarks/:id
async fn delete_bookmark(
    State(state): State<PlaybackState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    tracing::info!("DELETE /api/v1/bookmarks/{}", id);

    let existed = state.bookmark_manager.delete_bookmark(&id).await.map_err(|e| {
        tracing::error!("Failed to delete bookmark: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    if existed {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((StatusCode::NOT_FOUND, "Bookmark not found".to_string()))
    }
}

/// Criar router de playback
pub fn playback_routes(state: PlaybackState) -> Router {
    Router::new()
        .route(
            "/api/v1/recordings/:camera_id/timeline",
            get(get_timeline),
        )
        .route(
            "/api/v1/recordings/:camera_id/stream",
            get(stream_recording),
        )
        .route("/api/v1/bookmarks", get(list_bookmarks).post(create_bookmark))
        .route(
            "/api/v1/bookmarks/:id",
            get(get_bookmark).put(update_bookmark).delete(delete_bookmark),
        )
        .with_state(state)
}
