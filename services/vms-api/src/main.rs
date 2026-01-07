//! VMS API Service
//! Gateway REST/GraphQL com suporte a câmeras

use anyhow::Result;
use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber;

mod routes;
mod models;
mod db;

use db::camera_repository::CameraRepository;

#[derive(Clone)]
pub struct AppState {
    pub camera_repo: Arc<CameraRepository>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🚀 VMS API Service starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Conectar ao SQLite
    let database_url = "sqlite:./vms.db";
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    info!("📦 Connected to SQLite database");

    // Criar tabela de câmeras
    let camera_repo = CameraRepository::new(pool);
    camera_repo.create_table().await?;

    info!("✅ Database tables created");

    let state = AppState {
        camera_repo: Arc::new(camera_repo),
    };

    // Rotas de câmeras (novas)
    let camera_routes = Router::new()
        .route("/", get(routes::cameras_v2::list_cameras).post(routes::cameras_v2::create_camera))
        .route("/:id", get(routes::cameras_v2::get_camera).put(routes::cameras_v2::update_camera).delete(routes::cameras_v2::delete_camera))
        .with_state(state.clone());

    // Rotas antigas (manter compatibilidade)
    use routes::streams::*;
    use routes::recordings::*;
    
    use vms_common::camera::CameraInfo;
    let camera_store = Arc::new(tokio::sync::RwLock::new(Vec::<CameraInfo>::new()));
    
    let legacy_routes = Router::new()
        // .route("/cameras/discover", post(discover_cameras))
        // .route("/cameras/profiles", post(get_camera_profiles))
        // .route("/cameras/ptz", post(control_ptz))
        .route("/streams", post(start_stream))
        .route("/streams/:id", delete(stop_stream))
        .route("/recordings/:camera_id", get(list_recordings))
        .route("/recordings/:camera_id/:recording_id", get(download_recording))
        .with_state(camera_store);

    let api_routes = Router::new()
        .nest("/cameras", camera_routes)
        .merge(legacy_routes);

    let app = Router::new()
        .route("/health", get(routes::health_check))
        .route("/metrics", get(|| async { "# API metrics\nvms_api_requests_total 0\n" }))
        .nest("/api/v1", api_routes)
        .fallback(routes::not_found);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 9095));
    let listener = TcpListener::bind(addr).await?;

    info!("🌐 HTTP API listening on http://{}", addr);
    info!("📚 Camera API: http://{}/api/v1/cameras", addr);
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
