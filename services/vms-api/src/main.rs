//! VMS API Service
//! Gateway REST com suporte a câmeras e autenticação

use anyhow::Result;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

mod db;
mod models;
mod routes;
mod recording_manager;

use db::camera_repository::CameraRepository;
use db::user_repository::UserRepository;
use db::server_repository::ServerRepository;
use recording_manager::RecordingManager;

#[derive(Clone)]
pub struct AppState {
    pub camera_repo: Arc<CameraRepository>,
    pub user_repo: Arc<UserRepository>,
    pub server_repo: Arc<ServerRepository>,
    pub recording_manager: Arc<RecordingManager>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🚀 VMS API Service starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Connect to SQLite
    let database_url = "sqlite:./vms.db?mode=rwc";
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    info!("📦 Connected to SQLite database");

    // Initialize repositories
    let camera_repo = CameraRepository::new(pool.clone());
    camera_repo.create_table().await?;

    let user_repo = UserRepository::new(pool.clone());
    user_repo.create_table().await?;

    let server_repo = ServerRepository::new(pool.clone());
    server_repo.create_table().await?;

    info!("✅ Database tables created");

    let state = AppState {
        camera_repo: Arc::new(camera_repo),
        user_repo: Arc::new(user_repo),
        server_repo: Arc::new(server_repo),
        recording_manager: Arc::new(RecordingManager::new()),
    };

    // Auth routes
    let auth_routes = Router::new()
        .route("/login", post(routes::auth::login))
        .with_state(state.clone());

    // User routes
    let user_routes = Router::new()
        .route(
            "/",
            get(routes::auth::list_users).post(routes::auth::create_user),
        )
        .route(
            "/:id",
            get(routes::auth::get_user)
                .put(routes::auth::update_user)
                .delete(routes::auth::delete_user),
        )
        .route("/:id/password", post(routes::auth::change_password))
        .with_state(state.clone());

    // Camera routes
    let camera_routes = Router::new()
        .route(
            "/",
            get(routes::cameras_v2::list_cameras).post(routes::cameras_v2::create_camera),
        )
        .route(
            "/:id",
            get(routes::cameras_v2::get_camera)
                .put(routes::cameras_v2::update_camera)
                .delete(routes::cameras_v2::delete_camera),
        )
        .route("/test", post(routes::cameras_v2::test_camera_connection))
        .route("/:id/recording/start", post(routes::recordings::start_recording))
        .route("/:id/recording/stop", post(routes::recordings::stop_recording))
        .route("/:id/recording/status", get(routes::recordings::recording_status))
        .route("/:id/recordings", get(routes::recordings::list_recordings))
        .with_state(state.clone());

    // Legacy routes (backward compatibility)
    use routes::streams::*;
    use vms_common::camera::CameraInfo;

    let camera_store = Arc::new(tokio::sync::RwLock::new(Vec::<CameraInfo>::new()));

    let legacy_routes = Router::new()
        .route("/streams", post(start_stream))
        .route("/streams/:id", delete(stop_stream))
        .with_state(camera_store);

    // WebRTC routes
    let webrtc_routes = routes::webrtc::router();

    // Server routes
    let server_routes = Router::new()
        .route(
            "/",
            get(routes::servers::list_servers).post(routes::servers::create_server),
        )
        .route(
            "/:id",
            get(routes::servers::get_server)
                .put(routes::servers::update_server)
                .delete(routes::servers::delete_server),
        )
        .route("/:id/health", get(routes::servers::health_check_server))
        .with_state(state.clone());

    // API v1 routes
    let api_routes = Router::new()
        .nest("/auth", auth_routes)
        .nest("/users", user_routes)
        .nest("/cameras", camera_routes)
        .nest("/servers", server_routes)
        .nest("/webrtc", webrtc_routes)
        .merge(legacy_routes)
        .route("/mjpeg/:camera_id", get(routes::mjpeg::mjpeg_stream))
        .route("/filesystem/list", get(routes::filesystem::list_directory))
        .route("/filesystem/create", post(routes::filesystem::create_folder))
        .with_state(state.clone());

    // CORS for clients
    use tower_http::cors::{Any, CorsLayer};
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(routes::health_check))
        .route(
            "/metrics",
            get(|| async { "# API metrics\nvms_api_requests_total 0\n" }),
        )
        .nest("/api/v1", api_routes)
        .layer(cors)
        .fallback(routes::not_found);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 9095));
    let listener = TcpListener::bind(addr).await?;

    info!("🌐 HTTP API listening on http://{}", addr);
    info!("🔐 Auth API: http://{}/api/v1/auth/login", addr);
    info!("👥 Users API: http://{}/api/v1/users", addr);
    info!("📹 Camera API: http://{}/api/v1/cameras", addr);
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
