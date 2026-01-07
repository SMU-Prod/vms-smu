//! Playback Streamer
//! Stream de gravações via HTTP

use anyhow::Result;
use axum::body::Body;
use axum::http::{Response, StatusCode};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use futures::stream::{self, Stream};
use futures::StreamExt;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::pin::Pin;

pub struct PlaybackStreamer {
    storage_path: PathBuf,
}

impl PlaybackStreamer {
    pub fn new(storage_path: PathBuf) -> Self {
        Self { storage_path }
    }

    /// Stream uma gravação para o cliente
    pub async fn stream_recording(
        &self,
        camera_id: &str,
        start: DateTime<Utc>,
        end: Option<DateTime<Utc>>,
        speed: f32,
    ) -> Result<Response<Body>> {
        tracing::info!(
            "Streaming recording for camera {} from {} (speed: {}x)",
            camera_id,
            start,
            speed
        );

        // 1. Encontrar arquivos de vídeo que cobrem o período
        let files = self.find_recording_files(camera_id, start, end).await?;

        if files.is_empty() {
            tracing::warn!("No recordings found for camera {} at {}", camera_id, start);
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("No recordings found"))
                .unwrap());
        }

        tracing::debug!("Found {} files to stream", files.len());

        // 2. Criar stream que lê múltiplos arquivos sequencialmente
        let stream = self.create_multi_file_stream(files).await?;

        // 3. Retornar response com stream
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "video/mp4")
            .header("Cache-Control", "no-cache")
            .header("Accept-Ranges", "bytes")
            .body(Body::from_stream(stream))
            .unwrap())
    }

    async fn find_recording_files(
        &self,
        camera_id: &str,
        start: DateTime<Utc>,
        end: Option<DateTime<Utc>>,
    ) -> Result<Vec<PathBuf>> {
        use tokio::fs;

        let camera_path = self.storage_path.join(camera_id);

        if !camera_path.exists() {
            anyhow::bail!("Camera directory not found: {:?}", camera_path);
        }

        let mut files = Vec::new();

        // Iterar por datas
        let mut current_date = start.date_naive();
        let end_date = end.map(|e| e.date_naive()).unwrap_or(start.date_naive());

        while current_date <= end_date {
            let date_str = current_date.format("%Y-%m-%d").to_string();
            let date_path = camera_path.join(&date_str);

            if date_path.exists() {
                let mut entries = fs::read_dir(&date_path).await?;

                while let Some(entry) = entries.next_entry().await? {
                    let path = entry.path();

                    if let Some(ext) = path.extension() {
                        if ext == "mkv" || ext == "mp4" {
                            // TODO: Filtrar por hora do arquivo se necessário
                            files.push(path);
                        }
                    }
                }
            }

            if let Some(next_date) = current_date.succ_opt() {
                current_date = next_date;
            } else {
                break;
            }
        }

        // Ordenar arquivos por nome (que geralmente segue ordem cronológica)
        files.sort();

        Ok(files)
    }

    async fn create_multi_file_stream(
        &self,
        files: Vec<PathBuf>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>> {
        // Criar stream que lê cada arquivo sequencialmente
        let stream = stream::iter(files)
            .then(|file_path| async move {
                tracing::debug!("Streaming file: {:?}", file_path);

                let mut file = match File::open(&file_path).await {
                    Ok(f) => f,
                    Err(e) => {
                        tracing::error!("Failed to open file {:?}: {}", file_path, e);
                        return Err(e);
                    }
                };

                let mut buffer = Vec::new();
                match file.read_to_end(&mut buffer).await {
                    Ok(_) => Ok(Bytes::from(buffer)),
                    Err(e) => {
                        tracing::error!("Failed to read file {:?}: {}", file_path, e);
                        Err(e)
                    }
                }
            })
            .filter_map(|result| async move {
                match result {
                    Ok(bytes) => Some(Ok(bytes)),
                    Err(_) => None, // Skip arquivos com erro
                }
            });

        Ok(Box::pin(stream))
    }

    /// Stream com seek em posição específica (byte offset)
    pub async fn stream_with_seek(
        &self,
        camera_id: &str,
        start: DateTime<Utc>,
        byte_offset: u64,
    ) -> Result<Response<Body>> {
        tracing::info!(
            "Streaming recording for camera {} from {} with offset {}",
            camera_id,
            start,
            byte_offset
        );

        let files = self.find_recording_files(camera_id, start, None).await?;

        if files.is_empty() {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("No recordings found"))
                .unwrap());
        }

        // TODO: Implementar seek correto calculando offset entre múltiplos arquivos
        // Por enquanto, retorna o primeiro arquivo

        let file_path = &files[0];
        let mut file = File::open(file_path).await?;

        // Fazer seek no arquivo
        use tokio::io::AsyncSeekExt;
        file.seek(std::io::SeekFrom::Start(byte_offset)).await?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        Ok(Response::builder()
            .status(StatusCode::PARTIAL_CONTENT)
            .header("Content-Type", "video/mp4")
            .header("Cache-Control", "no-cache")
            .body(Body::from(buffer))
            .unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_playback_streamer() {
        let streamer = PlaybackStreamer::new(PathBuf::from("./test_storage"));

        let start = Utc::now() - chrono::Duration::hours(1);

        // Teste básico - não deve dar panic
        let result = streamer.stream_recording("test_camera", start, None, 1.0).await;

        // Esperado: NOT_FOUND porque não há storage real
        assert!(result.is_ok());
    }
}
