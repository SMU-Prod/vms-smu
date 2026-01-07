//! Export module
//! 
//! Video export jobs with FFmpeg

use axum::{http::StatusCode, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateExportRequest {
    pub camera_id: String,
    pub start_time: String,
    pub end_time: String,
    pub format: ExportFormat,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Mp4,
    Mkv,
    Avi,
}

#[derive(Serialize)]
pub struct ExportJobResponse {
    pub id: Uuid,
    pub status: ExportStatus,
    pub progress: f32,
    pub output_url: Option<String>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ExportStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

pub async fn create_export_job(
    Json(req): Json<CreateExportRequest>,
) -> Result<Json<ExportJobResponse>, StatusCode> {
    info!("📤 Export request: {} {} -> {}", req.camera_id, req.start_time, req.end_time);

    // Parse timestamps
    let start_time = DateTime::parse_from_rfc3339(&req.start_time)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let end_time = DateTime::parse_from_rfc3339(&req.end_time)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Create export job
    let job_id = Uuid::new_v4();
    let export_path = std::env::var("EXPORT_PATH")
        .unwrap_or_else(|_| "C:\\exports".to_string());
    
    tokio::fs::create_dir_all(&export_path).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Spawn export task
    tokio::spawn(async move {
        if let Err(e) = process_export(
            job_id,
            req.camera_id,
            start_time,
            end_time,
            req.format,
            PathBuf::from(export_path),
        ).await {
            error!("Export job {} failed: {}", job_id, e);
        }
    });

    Ok(Json(ExportJobResponse {
        id: job_id,
        status: ExportStatus::Processing,
        progress: 0.0,
        output_url: None,
    }))
}

async fn process_export(
    job_id: Uuid,
    camera_id: String,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    format: ExportFormat,
    export_path: PathBuf,
) -> anyhow::Result<()> {
    info!("🎬 Processing export job: {}", job_id);

    let storage_path = std::env::var("STORAGE_PATH")
        .unwrap_or_else(|_| "C:\\storage\\cameras".to_string());

    // Find all video files in time range
    let mut video_files = Vec::new();
    let mut current = start_time;
    
    while current < end_time {
        let date_str = current.format("%Y-%m-%d").to_string();
        let hour = current.format("%H").to_string();
        let video_file = PathBuf::from(&storage_path)
            .join(&camera_id)
            .join(&date_str)
            .join(format!("video_{}.mkv", hour));

        if video_file.exists() {
            video_files.push(video_file);
        }

        current = current + chrono::Duration::hours(1);
    }

    if video_files.is_empty() {
        return Err(anyhow::anyhow!("No video files found in range"));
    }

    // Create concat file for FFmpeg
    let concat_file = export_path.join(format!("{}_concat.txt", job_id));
    let mut concat_content = String::new();
    for file in &video_files {
        concat_content.push_str(&format!("file '{}'\n", file.display()));
    }
    tokio::fs::write(&concat_file, concat_content).await?;

    // Output file
    let ext = match format {
        ExportFormat::Mp4 => "mp4",
        ExportFormat::Mkv => "mkv",
        ExportFormat::Avi => "avi",
    };
    let output_file = export_path.join(format!("{}.{}", job_id, ext));

    // Run FFmpeg
    info!("🎞️  Running FFmpeg for export: {:?}", output_file);
    
    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-y")
       .arg("-f").arg("concat")
       .arg("-safe").arg("0")
       .arg("-i").arg(&concat_file)
       .arg("-c").arg("copy")  // Copy codec (no re-encode)
       .arg("-movflags").arg("+faststart")  // Web-optimized
       .arg(&output_file)
       .stdout(Stdio::null())
       .stderr(Stdio::null());

    let status = cmd.status().await?;

    // Cleanup concat file
    let _ = tokio::fs::remove_file(&concat_file).await;

    if status.success() {
        info!("✅ Export completed: {:?}", output_file);
        Ok(())
    } else {
        Err(anyhow::anyhow!("FFmpeg failed with status: {}", status))
    }
}

