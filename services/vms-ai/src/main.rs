//! VMS AI Service
//! Pipeline de IA para detecção e análise

use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber;

mod detector;
mod nats_consumer;
mod tracker;

use nats_consumer::AIProcessor;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🚀 VMS AI Service starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Conectar ao NATS
    let nats_url = std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    let mut processor = AIProcessor::connect(&nats_url)
        .await
        .context("Failed to connect to NATS")?;
    info!("📡 NATS connected: {}", nats_url);

    // Carregar modelo (opcional - se arquivo existir)
    let model_path = std::env::var("AI_MODEL_PATH")
        .unwrap_or_else(|_| "./models/rtdetr.onnx".to_string());

    info!("🤖 AI Model path: {}", model_path);
    info!("⚙️  Device: CPU (ONNX Runtime with tract)");

    // Se modelo existir, carregar (caso contrário, roda sem detector)
    if std::path::Path::new(&model_path).exists() {
        match detector::ObjectDetector::new(std::path::Path::new(&model_path), 0.5) {
            Ok(det) => {
                processor.set_detector(det);
                info!("✅ Model loaded successfully");
            }
            Err(e) => {
                info!("⚠️  Model not loaded: {} - Running without detection", e);
            }
        }
    } else {
        info!("⚠️  Model file not found - Running in pass-through mode");
        info!("   To enable AI: place model at {}", model_path);
    }

    // Iniciar processamento
    processor
        .start_processing()
        .await
        .context("Failed to start AI processing")?;

    info!("🤖 AI processor started");

    // HTTP server para métricas
    use axum::{routing::get, Router};

    let processor_stats = Arc::new(processor);

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/metrics", get({
            let proc = processor_stats.clone();
            move || {
                let (frames, detections) = proc.get_stats();
                async move {
                    format!(
                        "# AI metrics\n\
                         vms_ai_frames_processed_total {}\n\
                         vms_ai_detections_total {}\n\
                         vms_ai_inference_time_ms 0\n",
                        frames, detections
                    )
                }
            }
        }));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 9093));
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
