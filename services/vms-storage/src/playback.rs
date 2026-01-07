//! Playback module
//! 
//! HTTP streaming and timeline API

use axum::{
    body::Body,
    extract::{Path, Query},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use tracing::{error, info};

#[derive(Deserialize)]
pub struct PlaybackParams {
    pub start: Option<String>,
    pub end: Option<String>,
}

/// Stream video file with HTTP range support
pub async fn stream_handler(
    Path(camera_id): Path<String>,
    Query(params): Query<PlaybackParams>,
    headers: HeaderMap,
) -> Result<Response, StatusCode> {
    info!("📹 Playback request for camera: {}", camera_id);

    // Parse start time (default to now - 1 hour)
    let start_time = if let Some(start_str) = params.start {
        DateTime::parse_from_rfc3339(&start_str)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|_| StatusCode::BAD_REQUEST)?
    } else {
        Utc::now() - chrono::Duration::hours(1)
    };

    // Find video file for timestamp
    let storage_path = std::env::var("STORAGE_PATH")
        .unwrap_or_else(|_| "C:\\storage\\cameras".to_string());
    
    let date_str = start_time.format("%Y-%m-%d").to_string();
    let hour = start_time.format("%H").to_string();
    let video_file = PathBuf::from(&storage_path)
        .join(&camera_id)
        .join(&date_str)
        .join(format!("video_{}.mkv", hour));

    if !video_file.exists() {
        error!("Video file not found: {:?}", video_file);
        return Err(StatusCode::NOT_FOUND);
    }

    // Get file metadata
    let file_metadata = tokio::fs::metadata(&video_file)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let file_size = file_metadata.len();

    // Check for Range header
    if let Some(range_header) = headers.get(header::RANGE) {
        // Parse range header (e.g., "bytes=0-1023")
        if let Ok(range_str) = range_header.to_str() {
            if let Some(range) = parse_range_header(range_str, file_size) {
                return serve_range(&video_file, range, file_size).await;
            }
        }
    }

    // Serve entire file
    serve_file(&video_file, file_size).await
}

/// Serve entire file
async fn serve_file(path: &PathBuf, file_size: u64) -> Result<Response, StatusCode> {
    let file = File::open(path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "video/x-matroska")
        .header(header::CONTENT_LENGTH, file_size.to_string())
        .header(header::ACCEPT_RANGES, "bytes")
        .body(body)
        .unwrap())
}

/// Serve file range (for seeking)
async fn serve_range(
    path: &PathBuf,
    range: (u64, u64),
    file_size: u64,
) -> Result<Response, StatusCode> {
    use tokio::io::{AsyncReadExt, AsyncSeekExt};

    let mut file = File::open(path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Seek to start position
    file.seek(std::io::SeekFrom::Start(range.0))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Read range
    let length = range.1 - range.0 + 1;
    let mut buffer = vec![0u8; length as usize];
    file.read_exact(&mut buffer)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let content_range = format!("bytes {}-{}/{}", range.0, range.1, file_size);

    Ok(Response::builder()
        .status(StatusCode::PARTIAL_CONTENT)
        .header(header::CONTENT_TYPE, "video/x-matroska")
        .header(header::CONTENT_LENGTH, length.to_string())
        .header(header::CONTENT_RANGE, content_range)
        .header(header::ACCEPT_RANGES, "bytes")
        .body(Body::from(buffer))
        .unwrap())
}

/// Parse Range header
fn parse_range_header(range_str: &str, file_size: u64) -> Option<(u64, u64)> {
    // Format: "bytes=start-end"
    if !range_str.starts_with("bytes=") {
        return None;
    }

    let range_part = &range_str[6..];
    let parts: Vec<&str> = range_part.split('-').collect();
    
    if parts.len() != 2 {
        return None;
    }

    let start: u64 = parts[0].parse().ok()?;
    let end: u64 = if parts[1].is_empty() {
        file_size - 1
    } else {
        parts[1].parse().ok()?
    };

    Some((start, end.min(file_size - 1)))
}

#[derive(Deserialize)]
pub struct TimelineParams {
    pub date: String,
}

#[derive(Serialize)]
pub struct Timeline {
    pub camera_id: String,
    pub date: String,
    pub segments: Vec<TimelineSegment>,
}

#[derive(Serialize)]
pub struct TimelineSegment {
    pub start: String,
    pub end: String,
    pub duration_seconds: u64,
    pub has_video: bool,
    pub file_size: u64,
    pub thumbnail: Option<String>,
}

/// Get timeline for a specific date
pub async fn timeline_handler(
    Path(camera_id): Path<String>,
    Query(params): Query<TimelineParams>,
) -> Result<Json<Timeline>, StatusCode> {
    info!("📅 Timeline request for camera: {} date: {}", camera_id, params.date);

    // Parse date
    let date = NaiveDate::parse_from_str(&params.date, "%Y-%m-%d")
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let storage_path = std::env::var("STORAGE_PATH")
        .unwrap_or_else(|_| "C:\\storage\\cameras".to_string());
    
    let date_dir = PathBuf::from(&storage_path)
        .join(&camera_id)
        .join(&params.date);

    let mut segments = Vec::new();

    // Check all 24 hours
    for hour in 0..24 {
        let video_file = date_dir.join(format!("video_{:02}.mkv", hour));
        
        if video_file.exists() {
            let metadata = tokio::fs::metadata(&video_file)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let start_time = date
                .and_hms_opt(hour, 0, 0)
                .unwrap()
                .and_utc();
            
            let end_time = if hour == 23 {
                date.and_hms_opt(23, 59, 59).unwrap().and_utc()
            } else {
                date.and_hms_opt(hour + 1, 0, 0).unwrap().and_utc()
            };

            segments.push(TimelineSegment {
                start: start_time.to_rfc3339(),
                end: end_time.to_rfc3339(),
                duration_seconds: 3600,
                has_video: true,
                file_size: metadata.len(),
                thumbnail: Some(format!("/thumbnails/{}/{}/thumb_{:02}.webp", 
                    camera_id, params.date, hour)),
            });
        }
    }

    Ok(Json(Timeline {
        camera_id,
        date: params.date,
        segments,
    }))
}

