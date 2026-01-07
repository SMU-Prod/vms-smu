//! VMS Storage Service
//! Serviço responsável por gravação e playback de vídeo

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tracing::info;
use tracing_subscriber;

mod nats_consumer;
mod retention;
mod writer;
mod playback;
mod routes;

use nats_consumer::NatsConsumer;
use retention::RetentionManager;
use writer::VideoWriter;
use routes::playback_routes;
use playback::{TimelineBuilder, PlaybackStreamer, BookmarkManager};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🚀 VMS Storage Service starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    let base_path = std::env::var("STORAGE_PATH").unwrap_or_else(|_| "./storage".to_string());
    let retention_days = 30;

    info!("📁 Storage path: {}", base_path);
    info!("🗓️  Retention: {} days", retention_days);

    // Conectar ao NATS e iniciar consumer
    let nats_url = std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    let consumer = Arc::new(
        NatsConsumer::connect(&nats_url, PathBuf::from(&base_path))
            .await
            .context("Failed to connect to NATS")?
    );

    consumer
        .start_consuming()
        .await
        .context("Failed to start consuming frames")?;

    info!("📡 NATS consumer started: {}", nats_url);

    // Criar gerenciador de retenção
    let retention_manager = Arc::new(RetentionManager::new(base_path.clone(), retention_days));

    // Task de limpeza diária
    let retention_clone = retention_manager.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(86400)); // 24h
        loop {
            interval.tick().await;
            if let Err(e) = retention_clone.cleanup().await {
                tracing::error!("Retention cleanup failed: {}", e);
            }
        }
    });

    // Writers ativos (em produção seria gerenciado dinamicamente)
    let _writers: Arc<RwLock<HashMap<String, VideoWriter>>> = Arc::new(RwLock::new(HashMap::new()));

    // Criar componentes de playback
    let timeline_builder = Arc::new(TimelineBuilder::new(PathBuf::from(&base_path)));
    let playback_streamer = Arc::new(PlaybackStreamer::new(PathBuf::from(&base_path)));
    let bookmark_manager = Arc::new(BookmarkManager::new());

    let playback_state = routes::playback::PlaybackState {
        timeline_builder,
        streamer: playback_streamer,
        bookmark_manager,
    };

    info!("🎬 Playback system initialized");

    // API HTTP básica
    use axum::{routing::get, Router};

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/metrics", get(|| async { "# Storage metrics\nvms_storage_writers 0\n" }))
        .merge(playback_routes(playback_state));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 9092));
    let listener = TcpListener::bind(addr).await?;

    info!("📊 HTTP server listening on http://{}", addr);
    info!("✅ Service initialized successfully");
    info!("Press Ctrl+C to stop");

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
        })
        .await?;

    info!("👋 Goodbye!");
    Ok(())
}
