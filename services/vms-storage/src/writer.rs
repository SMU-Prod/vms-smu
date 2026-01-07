//! Gravador de vídeo

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::{debug, info};
use vms_common::types::CameraId;
use vms_format::{IndexEntry, VideoIndex};

/// Gravador de vídeo para uma câmera
pub struct VideoWriter {
    camera_id: CameraId,
    base_path: PathBuf,
    current_file: Option<File>,
    current_index: VideoIndex,
    current_hour: Option<DateTime<Utc>>,
    bytes_written: u64,
}

impl VideoWriter {
    pub fn new(camera_id: CameraId, base_path: PathBuf) -> Result<Self> {
        Ok(Self {
            camera_id,
            base_path,
            current_file: None,
            current_index: VideoIndex {
                version: 1,
                entries: Vec::new(),
            },
            current_hour: None,
            bytes_written: 0,
        })
    }

    /// Escreve um frame
    pub fn write_frame(
        &mut self,
        data: &[u8],
        timestamp: DateTime<Utc>,
        is_keyframe: bool,
    ) -> Result<()> {
        // Verificar se precisa rotacionar arquivo (a cada hora)
        let current_hour = timestamp.format("%Y-%m-%d-%H").to_string();
        let needs_rotation = match &self.current_hour {
            None => true,
            Some(last_ts) => {
                let last_hour = last_ts.format("%Y-%m-%d-%H").to_string();
                current_hour != last_hour
            }
        };

        if needs_rotation {
            self.rotate_file(timestamp)?;
        }

        // Escrever dados
        if let Some(ref mut file) = self.current_file {
            let offset = self.bytes_written;
            file.write_all(data)?;
            self.bytes_written += data.len() as u64;

            // Adicionar entrada no índice
            self.current_index.entries.push(IndexEntry {
                timestamp_ms: timestamp.timestamp_millis() as u64,
                offset,
                is_keyframe,
            });

            debug!(
                "Wrote frame: {} bytes, offset: {}, keyframe: {}",
                data.len(),
                offset,
                is_keyframe
            );
        }

        Ok(())
    }

    /// Rotaciona arquivo (nova hora)
    fn rotate_file(&mut self, timestamp: DateTime<Utc>) -> Result<()> {
        // Fechar arquivo atual e salvar índice
        if self.current_file.is_some() {
            self.close_current_file()?;
        }

        // Criar novo arquivo
        let date_str = timestamp.format("%Y-%m-%d").to_string();
        let hour_str = timestamp.format("%H").to_string();

        let dir_path = self.base_path
            .join("cameras")
            .join(self.camera_id.to_string())
            .join(&date_str);

        create_dir_all(&dir_path)?;

        let video_path = dir_path.join(format!("video_{}.mkv", hour_str));

        info!("Creating new video file: {}", video_path.display());

        let file = File::create(&video_path)?;
        self.current_file = Some(file);
        self.current_hour = Some(timestamp);
        self.bytes_written = 0;
        self.current_index = VideoIndex {
            version: 1,
            entries: Vec::new(),
        };

        Ok(())
    }

    /// Fecha arquivo atual e salva índice
    fn close_current_file(&mut self) -> Result<()> {
        if let Some(ref mut file) = self.current_file {
            file.flush()?;
            info!("Closed video file, {} bytes written", self.bytes_written);
        }

        // Salvar índice
        if let Some(hour) = self.current_hour {
            let date_str = hour.format("%Y-%m-%d").to_string();
            let hour_str = hour.format("%H").to_string();

            let dir_path = self.base_path
                .join("cameras")
                .join(self.camera_id.to_string())
                .join(&date_str);

            let index_path = dir_path.join(format!("index_{}.json", hour_str));

            let index_json = serde_json::to_string_pretty(&self.current_index)?;
            std::fs::write(&index_path, index_json)?;

            info!("Saved index with {} entries", self.current_index.entries.len());
        }

        self.current_file = None;
        Ok(())
    }

    /// Flush dados
    pub fn flush(&mut self) -> Result<()> {
        if let Some(ref mut file) = self.current_file {
            file.flush()?;
        }
        Ok(())
    }
}

impl Drop for VideoWriter {
    fn drop(&mut self) {
        let _ = self.close_current_file();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_video_writer() {
        let dir = tempdir().unwrap();
        let camera_id = CameraId::new();

        let mut writer = VideoWriter::new(camera_id, dir.path().to_path_buf()).unwrap();

        let data = b"test frame data";
        let timestamp = Utc::now();

        writer.write_frame(data, timestamp, true).unwrap();
        writer.flush().unwrap();
    }
}
