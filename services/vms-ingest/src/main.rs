//! VMS Ingest Service
//!
//! Serviço responsável por ingestão de streams de câmeras via RTSP/ONVIF

use anyhow::{Context, Result};
use std::sync::Arc;
use tracing::info;
use tracing_subscriber;
use vms_common::camera::CameraConfig;

mod camera_manager;
mod metrics;
mod nats_publisher;
mod onvif;
mod pipeline;
mod api_client;

use camera_manager::CameraManager;
use metrics::IngestMetrics;
use nats_publisher::NatsPublisher;
use api_client::ApiClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Inicializar logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .init();

    info!("🚀 VMS Ingest Service starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Inicializar GStreamer
    gstreamer::init()?;
    info!("✅ GStreamer initialized");

    // Conectar ao NATS
    let nats_url = std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    let nats_publisher = Arc::new(
        NatsPublisher::connect(&nats_url)
            .await
            .context("Failed to connect to NATS")?
    );
    info!("📡 NATS connected: {}", nats_url);

    // Criar gerenciador de câmeras
    let manager = Arc::new(CameraManager::new(100));
    let metrics = Arc::new(IngestMetrics::new());

    // Buscar câmeras da API
    let api_url = std::env::var("VMS_API_URL").unwrap_or_else(|_| "http://localhost:9095".to_string());
    let api_client = ApiClient::new(api_url);
    
    info!("📡 Fetching cameras from vms-api...");
    match api_client.get_enabled_cameras().await {
        Ok(api_cameras) => {
            info!("✅ Found {} enabled cameras", api_cameras.len());
            
            for api_camera in api_cameras {
                info!("📹 Adding camera: {}", api_camera.name);
                
                // Converter ApiCamera para CameraConfig with credentials
                let camera_config = CameraConfig::new(
                    api_camera.name.clone(),
                    api_camera.rtsp_url.clone(),
                ).with_credentials(
                    api_camera.username.clone(),
                    api_camera.password.clone(),
                );
                
                if let Err(e) = manager.add_camera(camera_config).await {
                    info!("⚠️  Failed to add camera {}: {}", api_camera.name, e);
                }
            }
        }
        Err(e) => {
            info!("⚠️  Could not fetch cameras from API: {}", e);
            info!("⚠️  Service will start without cameras. Add cameras via vms-api.");
        }
    }

    // Iniciar todas as câmeras
    info!("▶️  Starting all cameras...");
    manager.start_all().await?;

    // Health check task
    let manager_clone = manager.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
        loop {
            interval.tick().await;
            manager_clone.health_check().await;
        }
    });

    // Auto-reconnect task
    let manager_clone = manager.clone();
    let metrics_clone = metrics.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            manager_clone.auto_reconnect().await;
            metrics_clone.increment_reconnects();
        }
    });

    // Metrics endpoint (simple HTTP server)
    let metrics_clone = metrics.clone();
    tokio::spawn(async move {
        use std::net::SocketAddr;
        use tokio::io::AsyncWriteExt;
        use tokio::net::TcpListener;

        let addr = SocketAddr::from(([0, 0, 0, 0], 9091));
        let listener = TcpListener::bind(addr).await.unwrap();
        info!("📊 Metrics server listening on http://{}/metrics", addr);

        loop {
            if let Ok((mut socket, _)) = listener.accept().await {
                let metrics = metrics_clone.clone();
                tokio::spawn(async move {
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n{}",
                        metrics.export()
                    );
                    let _ = socket.write_all(response.as_bytes()).await;
                });
            }
        }
    });

    info!("✅ Service initialized successfully");
    info!("📊 Metrics: http://localhost:9091/metrics");
    info!("Press Ctrl+C to stop");

    // Aguardar sinal de parada
    tokio::signal::ctrl_c().await?;
    info!("Shutting down...");

    manager.stop_all().await?;
    info!("👋 Goodbye!");

    Ok(())
}
