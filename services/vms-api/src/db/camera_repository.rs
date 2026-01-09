//! Camera database repository - Professional VMS

use anyhow::Result;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

use crate::models::camera::{Camera, TransportProtocol, RecordingMode};

pub struct CameraRepository {
    pool: SqlitePool,
}

impl CameraRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Create cameras table with all professional VMS fields
    pub async fn create_table(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS cameras (
                id TEXT PRIMARY KEY,
                
                -- Geral
                name TEXT NOT NULL,
                description TEXT,
                manufacturer TEXT,
                model TEXT,
                firmware TEXT,
                enabled BOOLEAN NOT NULL DEFAULT 1,
                
                -- Streaming
                ip_address TEXT NOT NULL,
                rtsp_port INTEGER NOT NULL DEFAULT 554,
                onvif_port INTEGER,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                rtsp_url TEXT NOT NULL,
                onvif_url TEXT,
                transport TEXT NOT NULL DEFAULT 'auto',
                use_ssl BOOLEAN NOT NULL DEFAULT 0,
                timeout_ms INTEGER NOT NULL DEFAULT 30000,
                
                -- Video
                resolution_width INTEGER NOT NULL DEFAULT 1920,
                resolution_height INTEGER NOT NULL DEFAULT 1080,
                framerate REAL NOT NULL DEFAULT 30.0,
                codec TEXT NOT NULL DEFAULT 'h264',
                
                -- Gravação
                recording_mode TEXT NOT NULL DEFAULT 'disabled',
                recording_dir TEXT,
                audio_enabled BOOLEAN NOT NULL DEFAULT 0,
                retention_days INTEGER NOT NULL DEFAULT 30,
                
                -- Localização
                shortcut TEXT,
                latitude REAL,
                longitude REAL,
                
                -- Vinculação
                server_id TEXT,
                
                -- Timestamps
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                
                FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE SET NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Migration: add audio_enabled column if it doesn't exist (for existing databases)
        let _ = sqlx::query("ALTER TABLE cameras ADD COLUMN audio_enabled BOOLEAN NOT NULL DEFAULT 0")
            .execute(&self.pool)
            .await; // Ignore error if column already exists

        Ok(())
    }

    /// Insert new camera
    pub async fn create(&self, camera: &Camera) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO cameras (
                id, name, description, manufacturer, model, firmware, enabled,
                ip_address, rtsp_port, onvif_port, username, password, rtsp_url, onvif_url,
                transport, use_ssl, timeout_ms,
                resolution_width, resolution_height, framerate, codec,
                recording_mode, recording_dir, audio_enabled, retention_days,
                shortcut, latitude, longitude, server_id,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(camera.id.to_string())
        .bind(&camera.name)
        .bind(&camera.description)
        .bind(&camera.manufacturer)
        .bind(&camera.model)
        .bind(&camera.firmware)
        .bind(camera.enabled)
        .bind(&camera.ip_address)
        .bind(camera.rtsp_port as i64)
        .bind(camera.onvif_port.map(|p| p as i64))
        .bind(&camera.username)
        .bind(&camera.password)
        .bind(&camera.rtsp_url)
        .bind(&camera.onvif_url)
        .bind(camera.transport.as_str())
        .bind(camera.use_ssl)
        .bind(camera.timeout_ms as i64)
        .bind(camera.resolution_width as i64)
        .bind(camera.resolution_height as i64)
        .bind(camera.framerate as f64)
        .bind(&camera.codec)
        .bind(camera.recording_mode.as_str())
        .bind(&camera.recording_dir)
        .bind(camera.audio_enabled)
        .bind(camera.retention_days as i64)
        .bind(&camera.shortcut)
        .bind(camera.latitude)
        .bind(camera.longitude)
        .bind(camera.server_id.map(|id| id.to_string()))
        .bind(camera.created_at.to_rfc3339())
        .bind(camera.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get all cameras
    pub async fn list(&self) -> Result<Vec<Camera>> {
        let rows = sqlx::query("SELECT * FROM cameras ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let cameras = rows
            .iter()
            .filter_map(|row| self.row_to_camera(row))
            .collect();

        Ok(cameras)
    }

    /// Get camera by ID
    pub async fn get(&self, id: Uuid) -> Result<Option<Camera>> {
        let row = sqlx::query("SELECT * FROM cameras WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.and_then(|row| self.row_to_camera(&row)))
    }

    /// Get cameras by server ID
    pub async fn list_by_server(&self, server_id: Uuid) -> Result<Vec<Camera>> {
        let rows = sqlx::query("SELECT * FROM cameras WHERE server_id = ? ORDER BY name")
            .bind(server_id.to_string())
            .fetch_all(&self.pool)
            .await?;

        let cameras = rows
            .iter()
            .filter_map(|row| self.row_to_camera(row))
            .collect();

        Ok(cameras)
    }

    /// Update camera
    pub async fn update(&self, id: Uuid, camera: &Camera) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE cameras SET
                name = ?, description = ?, manufacturer = ?, model = ?, firmware = ?, enabled = ?,
                ip_address = ?, rtsp_port = ?, onvif_port = ?, username = ?, password = ?,
                rtsp_url = ?, onvif_url = ?, transport = ?, use_ssl = ?, timeout_ms = ?,
                resolution_width = ?, resolution_height = ?, framerate = ?, codec = ?,
                recording_mode = ?, recording_dir = ?, audio_enabled = ?, retention_days = ?,
                shortcut = ?, latitude = ?, longitude = ?, server_id = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&camera.name)
        .bind(&camera.description)
        .bind(&camera.manufacturer)
        .bind(&camera.model)
        .bind(&camera.firmware)
        .bind(camera.enabled)
        .bind(&camera.ip_address)
        .bind(camera.rtsp_port as i64)
        .bind(camera.onvif_port.map(|p| p as i64))
        .bind(&camera.username)
        .bind(&camera.password)
        .bind(&camera.rtsp_url)
        .bind(&camera.onvif_url)
        .bind(camera.transport.as_str())
        .bind(camera.use_ssl)
        .bind(camera.timeout_ms as i64)
        .bind(camera.resolution_width as i64)
        .bind(camera.resolution_height as i64)
        .bind(camera.framerate as f64)
        .bind(&camera.codec)
        .bind(camera.recording_mode.as_str())
        .bind(&camera.recording_dir)
        .bind(camera.audio_enabled)
        .bind(camera.retention_days as i64)
        .bind(&camera.shortcut)
        .bind(camera.latitude)
        .bind(camera.longitude)
        .bind(camera.server_id.map(|sid| sid.to_string()))
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Delete camera
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM cameras WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Convert row to Camera
    fn row_to_camera(&self, row: &sqlx::sqlite::SqliteRow) -> Option<Camera> {
        Some(Camera {
            id: Uuid::parse_str(row.get("id")).ok()?,
            name: row.get("name"),
            description: row.get("description"),
            manufacturer: row.get("manufacturer"),
            model: row.get("model"),
            firmware: row.get("firmware"),
            enabled: row.get("enabled"),
            
            ip_address: row.get("ip_address"),
            rtsp_port: row.get::<i64, _>("rtsp_port") as u16,
            onvif_port: row.get::<Option<i64>, _>("onvif_port").map(|p| p as u16),
            username: row.get("username"),
            password: row.get("password"),
            rtsp_url: row.get("rtsp_url"),
            onvif_url: row.get("onvif_url"),
            transport: TransportProtocol::from_str(row.get("transport")),
            use_ssl: row.get("use_ssl"),
            timeout_ms: row.get::<i64, _>("timeout_ms") as u32,
            
            resolution_width: row.get::<i64, _>("resolution_width") as u32,
            resolution_height: row.get::<i64, _>("resolution_height") as u32,
            framerate: row.get::<f64, _>("framerate") as f32,
            codec: row.get("codec"),
            
            recording_mode: RecordingMode::from_str(row.get("recording_mode")),
            recording_dir: row.get("recording_dir"),
            audio_enabled: row.get("audio_enabled"),
            retention_days: row.get::<i64, _>("retention_days") as u32,
            
            shortcut: row.get("shortcut"),
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
            
            server_id: row.get::<Option<String>, _>("server_id")
                .and_then(|s| Uuid::parse_str(&s).ok()),
            
            created_at: chrono::DateTime::parse_from_rfc3339(row.get("created_at"))
                .ok()?
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(row.get("updated_at"))
                .ok()?
                .with_timezone(&chrono::Utc),
        })
    }
}
