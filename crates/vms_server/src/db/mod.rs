//! Database repositories

pub mod user_repository;
pub mod refresh_token_repository;
pub mod node_repository;
pub mod camera_repository;
pub mod session_repository;

use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::sync::Arc;

pub use user_repository::*;
pub use refresh_token_repository::*;
pub use node_repository::*;
pub use camera_repository::*;
pub use session_repository::*;

/// Database connection pool
pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect(url)
            .await?;
        
        Ok(Self { pool })
    }

    /// Run migrations
    pub async fn migrate(&self) -> anyhow::Result<()> {
        // Create tables
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                email TEXT UNIQUE NOT NULL,
                name TEXT NOT NULL,
                password_hash TEXT NOT NULL,
                role TEXT NOT NULL DEFAULT 'viewer',
                enabled BOOLEAN NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
        "#).execute(&self.pool).await?;

        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS refresh_tokens (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                token_hash TEXT UNIQUE NOT NULL,
                expires_at TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES users(id)
            )
        "#).execute(&self.pool).await?;

        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS nodes (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                ip TEXT NOT NULL,
                port INTEGER NOT NULL,
                api_key TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'offline',
                last_seen TEXT,
                created_at TEXT NOT NULL
            )
        "#).execute(&self.pool).await?;

        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS cameras (
                id TEXT PRIMARY KEY,
                node_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                manufacturer TEXT NOT NULL DEFAULT 'Generic',
                model TEXT NOT NULL DEFAULT 'IP Camera',
                firmware TEXT,
                rtsp_url TEXT NOT NULL,
                onvif_url TEXT,
                recording_path TEXT,
                connection_timeout_ms INTEGER DEFAULT 30000,
                latitude REAL,
                longitude REAL,
                notes TEXT,
                transport TEXT DEFAULT 'auto',
                status TEXT NOT NULL DEFAULT 'unknown',
                enabled BOOLEAN NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (node_id) REFERENCES nodes(id)
            )
        "#).execute(&self.pool).await?;

        // Migration: Add new columns to existing cameras table
        let _ = sqlx::query("ALTER TABLE cameras ADD COLUMN firmware TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE cameras ADD COLUMN recording_path TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE cameras ADD COLUMN connection_timeout_ms INTEGER DEFAULT 30000").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE cameras ADD COLUMN latitude REAL").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE cameras ADD COLUMN longitude REAL").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE cameras ADD COLUMN notes TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE cameras ADD COLUMN transport TEXT DEFAULT 'auto'").execute(&self.pool).await;

        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS camera_credentials (
                camera_id TEXT PRIMARY KEY,
                username_encrypted BLOB NOT NULL,
                password_encrypted BLOB NOT NULL,
                FOREIGN KEY (camera_id) REFERENCES cameras(id)
            )
        "#).execute(&self.pool).await?;

        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS live_sessions (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                camera_id TEXT NOT NULL,
                node_id TEXT NOT NULL,
                profile TEXT,
                stream_url TEXT,
                status TEXT NOT NULL DEFAULT 'pending',
                started_at TEXT NOT NULL,
                expires_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES users(id),
                FOREIGN KEY (camera_id) REFERENCES cameras(id),
                FOREIGN KEY (node_id) REFERENCES nodes(id)
            )
        "#).execute(&self.pool).await?;

        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS audit_log (
                id TEXT PRIMARY KEY,
                user_id TEXT,
                action TEXT NOT NULL,
                details TEXT,
                created_at TEXT NOT NULL
            )
        "#).execute(&self.pool).await?;

        // Create default admin user if not exists
        let admin_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = 'admin')"
        ).fetch_one(&self.pool).await?;

        if !admin_exists {
            let admin_hash = crate::services::AuthService::hash_password("admin")?;
            let now = chrono::Utc::now().to_rfc3339();
            sqlx::query(
                "INSERT INTO users (id, email, name, password_hash, role, enabled, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(uuid::Uuid::new_v4().to_string())
            .bind("admin")
            .bind("Administrador")
            .bind(admin_hash)
            .bind("admin")
            .bind(true)
            .bind(&now)
            .bind(&now)
            .execute(&self.pool).await?;
            
            tracing::info!("Created default admin user: admin / admin");
        }

        Ok(())
    }
}
