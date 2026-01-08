//! Application state

use std::sync::Arc;
use crate::db::{Database, UserRepository, RefreshTokenRepository, NodeRepository, CameraRepository, SessionRepository};
use crate::services::AuthService;
use crate::webrtc::WebRtcRuntime;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub auth_service: Arc<AuthService>,
    pub user_repo: Arc<UserRepository>,
    pub refresh_repo: Arc<RefreshTokenRepository>,
    pub node_repo: Arc<NodeRepository>,
    pub camera_repo: Arc<CameraRepository>,
    pub session_repo: Arc<SessionRepository>,
    /// WebRTC runtime for peer connection management
    pub webrtc_runtime: WebRtcRuntime,
}

impl AppState {
    pub async fn new(database_url: &str, jwt_secret: String) -> anyhow::Result<Self> {
        let db = Database::connect(database_url).await?;
        db.migrate().await?;

        let auth_service = AuthService::new(jwt_secret, 24);
        let user_repo = UserRepository::new(db.pool.clone());
        let refresh_repo = RefreshTokenRepository::new(db.pool.clone());
        let node_repo = NodeRepository::new(db.pool.clone());
        let camera_repo = CameraRepository::new(db.pool.clone());
        let session_repo = SessionRepository::new(db.pool.clone());

        Ok(Self {
            db: Arc::new(db),
            auth_service: Arc::new(auth_service),
            user_repo: Arc::new(user_repo),
            refresh_repo: Arc::new(refresh_repo),
            node_repo: Arc::new(node_repo),
            camera_repo: Arc::new(camera_repo),
            session_repo: Arc::new(session_repo),
            webrtc_runtime: WebRtcRuntime::new(),
        })
    }
}

