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
                description TEXT,
                manufacturer TEXT NOT NULL DEFAULT 'Generic',
                model TEXT NOT NULL DEFAULT 'IP Camera',
                firmware TEXT,
                rtsp_url TEXT NOT NULL,
                onvif_url TEXT,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                shortcut TEXT,
                recording_dir TEXT,
                notes TEXT,
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
                id, name, description, manufacturer, model, firmware,
                rtsp_url, onvif_url, username, password,
                shortcut, recording_dir, notes,
                resolution_width, resolution_height, framerate, codec,
                enabled, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(camera.id.to_string())
        .bind(&camera.name)
        .bind(&camera.description)
        .bind(&camera.manufacturer)
        .bind(&camera.model)
        .bind(&camera.firmware)
        .bind(&camera.rtsp_url)
        .bind(&camera.onvif_url)
        .bind(&camera.username)
        .bind(&camera.password)
        .bind(&camera.shortcut)
        .bind(&camera.recording_dir)
        .bind(&camera.notes)
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
                    description: row.try_get("description").ok().flatten(),
                    manufacturer: row.try_get("manufacturer").unwrap_or_else(|_| "Generic".to_string()),
                    model: row.try_get("model").unwrap_or_else(|_| "IP Camera".to_string()),
                    firmware: row.try_get("firmware").ok().flatten(),
                    rtsp_url: row.get("rtsp_url"),
                    onvif_url: row.get("onvif_url"),
                    username: row.get("username"),
                    password: row.get("password"),
                    shortcut: row.try_get("shortcut").ok().flatten(),
                    recording_dir: row.try_get("recording_dir").ok().flatten(),
                    notes: row.try_get("notes").ok().flatten(),
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
                description: row.try_get("description").ok().flatten(),
                manufacturer: row.try_get("manufacturer").unwrap_or_else(|_| "Generic".to_string()),
                model: row.try_get("model").unwrap_or_else(|_| "IP Camera".to_string()),
                firmware: row.try_get("firmware").ok().flatten(),
                rtsp_url: row.get("rtsp_url"),
                onvif_url: row.get("onvif_url"),
                username: row.get("username"),
                password: row.get("password"),
                shortcut: row.try_get("shortcut").ok().flatten(),
                recording_dir: row.try_get("recording_dir").ok().flatten(),
                notes: row.try_get("notes").ok().flatten(),
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

    /// Update camera with partial data
    pub async fn update(&self, id: Uuid, req: &crate::models::camera::UpdateCameraRequest) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        
        // Build dynamic UPDATE query
        let mut updates = vec!["updated_at = ?".to_string()];
        let mut values: Vec<String> = vec![now.clone()];
        
        if let Some(ref v) = req.name { updates.push("name = ?".to_string()); values.push(v.clone()); }
        if let Some(ref v) = req.description { updates.push("description = ?".to_string()); values.push(v.clone()); }
        if let Some(ref v) = req.manufacturer { updates.push("manufacturer = ?".to_string()); values.push(v.clone()); }
        if let Some(ref v) = req.model { updates.push("model = ?".to_string()); values.push(v.clone()); }
        if let Some(ref v) = req.firmware { updates.push("firmware = ?".to_string()); values.push(v.clone()); }
        if let Some(ref v) = req.shortcut { updates.push("shortcut = ?".to_string()); values.push(v.clone()); }
        if let Some(ref v) = req.recording_dir { updates.push("recording_dir = ?".to_string()); values.push(v.clone()); }
        if let Some(ref v) = req.notes { updates.push("notes = ?".to_string()); values.push(v.clone()); }
        
        let sql = format!(
            "UPDATE cameras SET {} WHERE id = ?",
            updates.join(", ")
        );
        
        let mut query = sqlx::query(&sql);
        for v in &values {
            query = query.bind(v);
        }
        
        // Bind enabled separately as bool
        if let Some(e) = req.enabled {
            sqlx::query("UPDATE cameras SET enabled = ?, updated_at = ? WHERE id = ?")
                .bind(e)
                .bind(&now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }
        
        query = query.bind(id.to_string());
        query.execute(&self.pool).await?;

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
