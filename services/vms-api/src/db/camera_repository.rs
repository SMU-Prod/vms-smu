//! Camera database repository

use anyhow::Result;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

use crate::models::camera::Camera;

pub struct CameraRepository {
    pool: SqlitePool,
}

impl CameraRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Create cameras table
    pub async fn create_table(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS cameras (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                rtsp_url TEXT NOT NULL,
                onvif_url TEXT,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                resolution_width INTEGER NOT NULL,
                resolution_height INTEGER NOT NULL,
                framerate REAL NOT NULL,
                codec TEXT NOT NULL,
                enabled BOOLEAN NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Insert new camera
    pub async fn create(&self, camera: &Camera) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO cameras (
                id, name, rtsp_url, onvif_url, username, password,
                resolution_width, resolution_height, framerate, codec,
                enabled, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(camera.id.to_string())
        .bind(&camera.name)
        .bind(&camera.rtsp_url)
        .bind(&camera.onvif_url)
        .bind(&camera.username)
        .bind(&camera.password)
        .bind(camera.resolution_width as i64)
        .bind(camera.resolution_height as i64)
        .bind(camera.framerate as f64)
        .bind(&camera.codec)
        .bind(camera.enabled)
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
            .filter_map(|row| {
                Some(Camera {
                    id: Uuid::parse_str(row.get("id")).ok()?,
                    name: row.get("name"),
                    rtsp_url: row.get("rtsp_url"),
                    onvif_url: row.get("onvif_url"),
                    username: row.get("username"),
                    password: row.get("password"),
                    resolution_width: row.get::<i64, _>("resolution_width") as u32,
                    resolution_height: row.get::<i64, _>("resolution_height") as u32,
                    framerate: row.get::<f64, _>("framerate") as f32,
                    codec: row.get("codec"),
                    enabled: row.get("enabled"),
                    created_at: chrono::DateTime::parse_from_rfc3339(row.get("created_at"))
                        .ok()?
                        .with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(row.get("updated_at"))
                        .ok()?
                        .with_timezone(&chrono::Utc),
                })
            })
            .collect();

        Ok(cameras)
    }

    /// Get camera by ID
    pub async fn get(&self, id: Uuid) -> Result<Option<Camera>> {
        let row = sqlx::query("SELECT * FROM cameras WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;

        let camera = row.and_then(|row| {
            Some(Camera {
                id: Uuid::parse_str(row.get("id")).ok()?,
                name: row.get("name"),
                rtsp_url: row.get("rtsp_url"),
                onvif_url: row.get("onvif_url"),
                username: row.get("username"),
                password: row.get("password"),
                resolution_width: row.get::<i64, _>("resolution_width") as u32,
                resolution_height: row.get::<i64, _>("resolution_height") as u32,
                framerate: row.get::<f64, _>("framerate") as f32,
                codec: row.get("codec"),
                enabled: row.get("enabled"),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get("created_at"))
                    .ok()?
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get("updated_at"))
                    .ok()?
                    .with_timezone(&chrono::Utc),
            })
        });

        Ok(camera)
    }

    /// Update camera
    pub async fn update(&self, id: Uuid, name: Option<String>, enabled: Option<bool>) -> Result<()> {
        if let Some(n) = name {
            sqlx::query("UPDATE cameras SET name = ?, updated_at = ? WHERE id = ?")
                .bind(n)
                .bind(chrono::Utc::now().to_rfc3339())
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }

        if let Some(e) = enabled {
            sqlx::query("UPDATE cameras SET enabled = ?, updated_at = ? WHERE id = ?")
                .bind(e)
                .bind(chrono::Utc::now().to_rfc3339())
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }

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
}
