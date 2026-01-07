//! Gerenciador de múltiplas câmeras

use crate::pipeline::IngestPipeline;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use vms_common::camera::{CameraConfig, CameraStatus};
use vms_common::types::CameraId;

/// Gerenciador de câmeras
pub struct CameraManager {
    cameras: Arc<RwLock<HashMap<CameraId, CameraInstance>>>,
    max_cameras: usize,
}

struct CameraInstance {
    config: CameraConfig,
    pipeline: Option<IngestPipeline>,
    status: CameraStatus,
    retry_count: u32,
}

impl CameraManager {
    pub fn new(max_cameras: usize) -> Self {
        Self {
            cameras: Arc::new(RwLock::new(HashMap::new())),
            max_cameras,
        }
    }

    /// Adiciona uma câmera
    pub async fn add_camera(&self, config: CameraConfig) -> Result<()> {
        let mut cameras = self.cameras.write().await;

        if cameras.len() >= self.max_cameras {
            anyhow::bail!("Maximum camera limit reached: {}", self.max_cameras);
        }

        let camera_id = config.id;
        info!("Adding camera: {} ({})", config.name, camera_id);

        cameras.insert(
            camera_id,
            CameraInstance {
                config,
                pipeline: None,
                status: CameraStatus::Offline,
                retry_count: 0,
            },
        );

        Ok(())
    }

    /// Remove uma câmera
    pub async fn remove_camera(&self, camera_id: CameraId) -> Result<()> {
        let mut cameras = self.cameras.write().await;

        if let Some(mut instance) = cameras.remove(&camera_id) {
            if let Some(pipeline) = instance.pipeline.take() {
                pipeline.stop()?;
            }
            info!("Removed camera: {}", camera_id);
        }

        Ok(())
    }

    /// Inicia uma câmera
    pub async fn start_camera(&self, camera_id: CameraId) -> Result<()> {
        let mut cameras = self.cameras.write().await;

        let instance = cameras
            .get_mut(&camera_id)
            .context("Camera not found")?;

        instance.status = CameraStatus::Connecting;

        match IngestPipeline::new(instance.config.clone()) {
            Ok(pipeline) => {
                pipeline.start()?;
                instance.pipeline = Some(pipeline);
                instance.status = CameraStatus::Online;
                instance.retry_count = 0;
                info!("Started camera: {} ({})", instance.config.name, camera_id);
                Ok(())
            }
            Err(e) => {
                instance.status = CameraStatus::Error;
                instance.retry_count += 1;
                error!("Failed to start camera {}: {}", camera_id, e);
                Err(e)
            }
        }
    }

    /// Para uma câmera
    pub async fn stop_camera(&self, camera_id: CameraId) -> Result<()> {
        let mut cameras = self.cameras.write().await;

        let instance = cameras
            .get_mut(&camera_id)
            .context("Camera not found")?;

        if let Some(pipeline) = instance.pipeline.take() {
            pipeline.stop()?;
            instance.status = CameraStatus::Offline;
            info!("Stopped camera: {}", camera_id);
        }

        Ok(())
    }

    /// Reconecta câmera
    pub async fn reconnect_camera(&self, camera_id: CameraId) -> Result<()> {
        info!("Reconnecting camera: {}", camera_id);
        self.stop_camera(camera_id).await.ok();
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        self.start_camera(camera_id).await
    }

    /// Inicia todas as câmeras
    pub async fn start_all(&self) -> Result<()> {
        let camera_ids: Vec<CameraId> = {
            let cameras = self.cameras.read().await;
            cameras.keys().copied().collect()
        };

        for camera_id in camera_ids {
            if let Err(e) = self.start_camera(camera_id).await {
                warn!("Failed to start camera {}: {}", camera_id, e);
            }
        }

        Ok(())
    }

    /// Para todas as câmeras
    pub async fn stop_all(&self) -> Result<()> {
        let camera_ids: Vec<CameraId> = {
            let cameras = self.cameras.read().await;
            cameras.keys().copied().collect()
        };

        for camera_id in camera_ids {
            if let Err(e) = self.stop_camera(camera_id).await {
                warn!("Failed to stop camera {}: {}", camera_id, e);
            }
        }

        Ok(())
    }

    /// Health check de todas as câmeras
    pub async fn health_check(&self) {
        let mut cameras = self.cameras.write().await;

        for (camera_id, instance) in cameras.iter_mut() {
            if let Some(pipeline) = &instance.pipeline {
                if !pipeline.is_running() {
                    warn!("Camera {} is not running, will retry", camera_id);
                    instance.status = CameraStatus::Error;
                    instance.retry_count += 1;
                }
            }
        }
    }

    /// Auto-reconexão de câmeras com erro
    pub async fn auto_reconnect(&self) {
        let cameras_to_reconnect: Vec<CameraId> = {
            let cameras = self.cameras.read().await;
            cameras
                .iter()
                .filter(|(_, instance)| {
                    instance.status == CameraStatus::Error && instance.retry_count < 10
                })
                .map(|(id, _)| *id)
                .collect()
        };

        for camera_id in cameras_to_reconnect {
            info!("Auto-reconnecting camera: {}", camera_id);
            if let Err(e) = self.reconnect_camera(camera_id).await {
                error!("Auto-reconnect failed for {}: {}", camera_id, e);
            }
        }
    }

    /// Retorna status de todas as câmeras
    pub async fn get_all_status(&self) -> Vec<(CameraId, CameraStatus)> {
        let cameras = self.cameras.read().await;
        cameras
            .iter()
            .map(|(id, instance)| (*id, instance.status))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_camera_manager() {
        let manager = CameraManager::new(10);

        let config = CameraConfig::new(
            "Test Camera".to_string(),
            "rtsp://test".to_string(),
        );

        manager.add_camera(config).await.unwrap();
        assert_eq!(manager.cameras.read().await.len(), 1);
    }
}
