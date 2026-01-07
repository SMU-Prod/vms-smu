//! Timeline API
//! Gera timeline de gravações com eventos, movimento e bookmarks

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Timeline {
    pub camera_id: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub resolution: TimelineResolution,
    pub segments: Vec<RecordingSegment>,
    pub motion_zones: Vec<MotionZone>,
    pub events: Vec<TimelineEvent>,
    pub bookmarks: Vec<Bookmark>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TimelineResolution {
    #[serde(rename = "1s")]
    OneSecond,
    #[serde(rename = "10s")]
    TenSeconds,
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "10m")]
    TenMinutes,
    #[serde(rename = "1h")]
    OneHour,
}

impl Default for TimelineResolution {
    fn default() -> Self {
        TimelineResolution::OneMinute
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordingSegment {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub file_path: String,
    pub size_bytes: u64,
    pub has_motion: bool,
    pub has_events: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MotionZone {
    pub timestamp: DateTime<Utc>,
    pub duration_ms: u64,
    pub confidence: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "event_type")]
    pub event_type: String,
    pub priority: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bookmark {
    pub id: String,
    pub camera_id: String,
    pub timestamp: DateTime<Utc>,
    pub user_id: String,
    pub note: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct TimelineBuilder {
    storage_path: PathBuf,
}

impl TimelineBuilder {
    pub fn new(storage_path: PathBuf) -> Self {
        Self { storage_path }
    }

    pub async fn build_timeline(
        &self,
        camera_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        resolution: TimelineResolution,
    ) -> Result<Timeline> {
        tracing::info!(
            "Building timeline for camera {} from {} to {}",
            camera_id,
            start,
            end
        );

        // 1. Buscar segmentos de gravação
        let segments = self.find_segments(camera_id, start, end).await?;
        tracing::debug!("Found {} recording segments", segments.len());

        // 2. Buscar zonas de movimento (TODO: implementar quando tiver detecção)
        let motion_zones = self.find_motion_zones(camera_id, start, end).await?;
        tracing::debug!("Found {} motion zones", motion_zones.len());

        // 3. Buscar eventos (TODO: integrar com vms-events)
        let events = self.find_events(camera_id, start, end).await?;
        tracing::debug!("Found {} events", events.len());

        // 4. Buscar bookmarks (TODO: implementar quando tiver DB)
        let bookmarks = self.find_bookmarks(camera_id, start, end).await?;
        tracing::debug!("Found {} bookmarks", bookmarks.len());

        Ok(Timeline {
            camera_id: camera_id.to_string(),
            start,
            end,
            resolution,
            segments,
            motion_zones,
            events,
            bookmarks,
        })
    }

    async fn find_segments(
        &self,
        camera_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<RecordingSegment>> {
        use tokio::fs;

        let camera_path = self.storage_path.join(camera_id);

        // Verificar se diretório da câmera existe
        if !camera_path.exists() {
            tracing::warn!("Camera directory not found: {:?}", camera_path);
            return Ok(vec![]);
        }

        let mut segments = Vec::new();

        // Iterar por datas no range
        let mut current_date = start.date_naive();
        let end_date = end.date_naive();

        while current_date <= end_date {
            let date_str = current_date.format("%Y-%m-%d").to_string();
            let date_path = camera_path.join(&date_str);

            if date_path.exists() {
                // Listar arquivos de vídeo no diretório da data
                let mut entries = fs::read_dir(&date_path).await?;

                while let Some(entry) = entries.next_entry().await? {
                    let path = entry.path();

                    // Procurar por arquivos .mkv ou .mp4
                    if let Some(ext) = path.extension() {
                        if ext == "mkv" || ext == "mp4" {
                            // Extrair timestamp do nome do arquivo
                            // Formato esperado: video_{hour}.mkv ou video_{timestamp}.mkv
                            if let Some(file_name) = path.file_stem() {
                                let file_name_str = file_name.to_string_lossy();

                                // Obter metadados do arquivo
                                if let Ok(metadata) = entry.metadata().await {
                                    let size = metadata.len();

                                    // Criar segmento (timestamp aproximado pela data + hora do arquivo)
                                    // TODO: melhorar parsing do timestamp
                                    let segment = RecordingSegment {
                                        start: start, // TODO: calcular real baseado no nome
                                        end: end,     // TODO: calcular real
                                        file_path: path.to_string_lossy().to_string(),
                                        size_bytes: size,
                                        has_motion: false, // TODO: buscar de índice
                                        has_events: vec![], // TODO: buscar de índice
                                    };

                                    segments.push(segment);
                                }
                            }
                        }
                    }
                }
            }

            current_date = current_date.succ_opt().unwrap_or(current_date);
        }

        // Ordenar por timestamp
        segments.sort_by_key(|s| s.start);

        Ok(segments)
    }

    async fn find_motion_zones(
        &self,
        _camera_id: &str,
        _start: DateTime<Utc>,
        _end: DateTime<Utc>,
    ) -> Result<Vec<MotionZone>> {
        // TODO: Implementar quando tiver detecção de movimento
        // Por enquanto retorna vazio
        Ok(vec![])
    }

    async fn find_events(
        &self,
        _camera_id: &str,
        _start: DateTime<Utc>,
        _end: DateTime<Utc>,
    ) -> Result<Vec<TimelineEvent>> {
        // TODO: Integrar com vms-events via API ou DB compartilhado
        // Por enquanto retorna vazio
        Ok(vec![])
    }

    async fn find_bookmarks(
        &self,
        _camera_id: &str,
        _start: DateTime<Utc>,
        _end: DateTime<Utc>,
    ) -> Result<Vec<Bookmark>> {
        // TODO: Implementar quando tiver BookmarkManager com DB
        // Por enquanto retorna vazio
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timeline_builder() {
        let builder = TimelineBuilder::new(PathBuf::from("./test_storage"));

        let start = Utc::now() - chrono::Duration::hours(1);
        let end = Utc::now();

        let timeline = builder
            .build_timeline("test_camera", start, end, TimelineResolution::OneMinute)
            .await;

        assert!(timeline.is_ok());
    }
}
