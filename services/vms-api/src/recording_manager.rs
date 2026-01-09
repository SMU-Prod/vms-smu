//! Recording Manager - FFmpeg-based continuous recording
//!
//! Supports: continuous, motion (trigger-based), manual modes

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::{Child, Command};
use tokio::sync::RwLock;
use uuid::Uuid;

/// Recording session state
#[derive(Debug, Clone)]
pub struct RecordingSession {
    pub camera_id: Uuid,
    pub recording_dir: PathBuf,
    pub mode: RecordingMode,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub segment_duration_secs: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecordingMode {
    Continuous,
    Motion,
    Manual,
}

/// Global recording manager
pub struct RecordingManager {
    /// Active recordings: camera_id -> (session, process_handle)
    active: Arc<RwLock<HashMap<Uuid, (RecordingSession, Option<tokio::task::JoinHandle<()>>)>>>,
}

impl RecordingManager {
    pub fn new() -> Self {
        Self {
            active: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Start recording for a camera
    pub async fn start(
        &self,
        camera_id: Uuid,
        rtsp_url: &str,
        username: &str,
        password: &str,
        recording_dir: &str,
        mode: RecordingMode,
    ) -> Result<(), String> {
        // Check if already recording
        {
            let active = self.active.read().await;
            if active.contains_key(&camera_id) {
                return Err("Gravação já está ativa para esta câmera".to_string());
            }
        }

        // Ensure directory exists
        let dir = PathBuf::from(recording_dir);
        tokio::fs::create_dir_all(&dir).await
            .map_err(|e| format!("Erro ao criar diretório: {}", e))?;

        // Build authenticated RTSP URL
        let encoded_password = urlencoding::encode(password);
        let full_rtsp_url = if rtsp_url.starts_with("rtsp://") {
            format!("rtsp://{}:{}@{}", username, encoded_password, &rtsp_url[7..])
        } else {
            rtsp_url.to_string()
        };

        let session = RecordingSession {
            camera_id,
            recording_dir: dir.clone(),
            mode,
            started_at: chrono::Utc::now(),
            segment_duration_secs: 300, // 5 minutes per segment
        };

        // Spawn FFmpeg recording task
        let active_clone = self.active.clone();
        let session_clone = session.clone();
        
        let handle = tokio::spawn(async move {
            let _ = run_recording_ffmpeg(
                &full_rtsp_url,
                &session_clone.recording_dir,
                session_clone.segment_duration_secs,
                camera_id,
            ).await;
            
            // Remove from active when done
            let mut active = active_clone.write().await;
            active.remove(&camera_id);
            tracing::info!("📹 Recording ended for camera {}", camera_id);
        });

        // Add to active recordings
        {
            let mut active = self.active.write().await;
            active.insert(camera_id, (session, Some(handle)));
        }

        tracing::info!("🔴 Recording started for camera {} in {:?} mode", camera_id, mode);
        Ok(())
    }

    /// Stop recording for a camera
    pub async fn stop(&self, camera_id: Uuid) -> Result<(), String> {
        let handle = {
            let mut active = self.active.write().await;
            active.remove(&camera_id)
        };

        if let Some((_, Some(handle))) = handle {
            handle.abort();
            tracing::info!("⏹️ Recording stopped for camera {}", camera_id);
            Ok(())
        } else {
            Err("Nenhuma gravação ativa para esta câmera".to_string())
        }
    }

    /// Check if camera is recording
    pub async fn is_recording(&self, camera_id: Uuid) -> bool {
        let active = self.active.read().await;
        active.contains_key(&camera_id)
    }

    /// List active recordings
    pub async fn list_active(&self) -> Vec<RecordingSession> {
        let active = self.active.read().await;
        active.values().map(|(s, _)| s.clone()).collect()
    }
}

impl Default for RecordingManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Run FFmpeg recording with segment split
async fn run_recording_ffmpeg(
    rtsp_url: &str,
    output_dir: &PathBuf,
    segment_secs: u32,
    camera_id: Uuid,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Output pattern: recording_2024-01-08_12-30-00.mkv
    let output_pattern = output_dir.join(format!(
        "{}_%Y-%m-%d_%H-%M-%S.mkv",
        camera_id.to_string().split('-').next().unwrap_or("cam")
    ));

    let mut child = Command::new("ffmpeg")
        .args([
            // Input
            "-rtsp_transport", "tcp",
            "-i", rtsp_url,
            
            // Copy streams (no re-encoding for speed)
            "-c", "copy",
            
            // Segment settings
            "-f", "segment",
            "-segment_time", &segment_secs.to_string(),
            "-segment_format", "matroska",
            "-strftime", "1",
            "-reset_timestamps", "1",
            
            // Output
            output_pattern.to_str().unwrap_or("output_%H-%M-%S.mkv"),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    // Wait for process to finish
    let _ = child.wait().await;
    
    Ok(())
}
