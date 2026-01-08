//! Camera repository

use chrono::Utc;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use vms_core::{Camera, CameraStatus, CreateCameraRequest, UpdateCameraRequest};

pub struct CameraRepository {
    pool: SqlitePool,
}

impl CameraRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a new camera
    pub async fn create(&self, node_id: Uuid, req: &CreateCameraRequest) -> anyhow::Result<Camera> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        // Build RTSP URL
        let stream_path = req.stream_path.as_deref().unwrap_or("stream1");
        let rtsp_url = format!("rtsp://{}:{}/{}", req.ip, req.rtsp_port, stream_path);
        let onvif_url = req.onvif_port.map(|p| format!("http://{}:{}", req.ip, p));

        sqlx::query(r#"
            INSERT INTO cameras (id, node_id, name, description, manufacturer, model, firmware, rtsp_url, onvif_url, recording_path, connection_timeout_ms, latitude, longitude, notes, transport, status, enabled, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(id.to_string())
        .bind(node_id.to_string())
        .bind(&req.name)
        .bind(&req.description)
        .bind(&req.manufacturer)
        .bind(&req.model)
        .bind(&req.firmware)
        .bind(&rtsp_url)
        .bind(&onvif_url)
        .bind(&req.recording_path)
        .bind(req.connection_timeout_ms.unwrap_or(30000) as i64)
        .bind(req.latitude)
        .bind(req.longitude)
        .bind(&req.notes)
        .bind(req.transport.as_deref().unwrap_or("auto"))
        .bind("unknown")
        .bind(req.enabled.unwrap_or(true))
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await?;

        // Store credentials separately
        sqlx::query("INSERT INTO camera_credentials (camera_id, username_encrypted, password_encrypted) VALUES (?, ?, ?)")
            .bind(id.to_string())
            .bind(req.username.as_bytes())
            .bind(req.password.as_bytes())
            .execute(&self.pool)
            .await?;

        tracing::info!("Created camera {} with ID {}", req.name, id);

        Ok(Camera {
            id,
            node_id,
            name: req.name.clone(),
            description: req.description.clone(),
            manufacturer: req.manufacturer.clone(),
            model: req.model.clone(),
            firmware: req.firmware.clone(),
            rtsp_url,
            onvif_url,
            recording_path: req.recording_path.clone(),
            connection_timeout_ms: Some(req.connection_timeout_ms.unwrap_or(30000)),
            latitude: req.latitude,
            longitude: req.longitude,
            notes: req.notes.clone(),
            transport: Some(req.transport.clone().unwrap_or_else(|| "auto".to_string())),
            status: CameraStatus::Unknown,
            enabled: req.enabled.unwrap_or(true),
            created_at: now,
            updated_at: now,
        })
    }

    /// List all cameras
    pub async fn list(&self) -> anyhow::Result<Vec<Camera>> {
        let rows = sqlx::query("SELECT * FROM cameras ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let cameras = rows.into_iter().filter_map(|r| self.row_to_camera(r)).collect();
        Ok(cameras)
    }

    /// List cameras by node
    pub async fn list_by_node(&self, node_id: Uuid) -> anyhow::Result<Vec<Camera>> {
        let rows = sqlx::query("SELECT * FROM cameras WHERE node_id = ? ORDER BY name")
            .bind(node_id.to_string())
            .fetch_all(&self.pool)
            .await?;

        let cameras = rows.into_iter().filter_map(|r| self.row_to_camera(r)).collect();
        Ok(cameras)
    }

    /// Find camera by ID
    pub async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Camera>> {
        let row = sqlx::query("SELECT * FROM cameras WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.and_then(|r| self.row_to_camera(r)))
    }

    /// Update camera
    pub async fn update(&self, id: Uuid, req: &UpdateCameraRequest) -> anyhow::Result<()> {
        let now = Utc::now().to_rfc3339();
        
        // Build dynamic update
        let mut updates = vec!["updated_at = ?"];
        let mut binds: Vec<String> = vec![now.clone()];

        if let Some(name) = &req.name {
            updates.push("name = ?");
            binds.push(name.clone());
        }
        if let Some(desc) = &req.description {
            updates.push("description = ?");
            binds.push(desc.clone());
        }
        if let Some(mfr) = &req.manufacturer {
            updates.push("manufacturer = ?");
            binds.push(mfr.clone());
        }
        if let Some(model) = &req.model {
            updates.push("model = ?");
            binds.push(model.clone());
        }
        if let Some(enabled) = req.enabled {
            updates.push("enabled = ?");
            binds.push(if enabled { "1" } else { "0" }.to_string());
        }

        // Rebuild RTSP URL if IP or port changed
        if req.ip.is_some() || req.rtsp_port.is_some() || req.stream_path.is_some() {
            // Get current camera to merge values
            if let Some(camera) = self.find_by_id(id).await? {
                let ip = req.ip.as_ref().map(|s| s.as_str()).unwrap_or_else(|| {
                    camera.rtsp_url.split("://").nth(1)
                        .and_then(|s| s.split(':').next())
                        .unwrap_or("192.168.1.1")
                });
                let port = req.rtsp_port.unwrap_or(554);
                let stream = req.stream_path.as_deref().unwrap_or("stream1");
                let new_url = format!("rtsp://{}:{}/{}", ip, port, stream);
                updates.push("rtsp_url = ?");
                binds.push(new_url);
            }
        }

        let sql = format!("UPDATE cameras SET {} WHERE id = ?", updates.join(", "));
        let mut query = sqlx::query(&sql);
        
        for bind in &binds {
            query = query.bind(bind);
        }
        query = query.bind(id.to_string());
        
        query.execute(&self.pool).await?;

        // Update credentials if provided
        if req.username.is_some() || req.password.is_some() {
            // First check if credentials exist
            let exists = sqlx::query("SELECT 1 FROM camera_credentials WHERE camera_id = ?")
                .bind(id.to_string())
                .fetch_optional(&self.pool)
                .await?
                .is_some();

            if exists {
                // Update existing credentials
                if let (Some(user), Some(pass)) = (&req.username, &req.password) {
                    sqlx::query("UPDATE camera_credentials SET username_encrypted = ?, password_encrypted = ? WHERE camera_id = ?")
                        .bind(user.as_bytes())
                        .bind(pass.as_bytes())
                        .bind(id.to_string())
                        .execute(&self.pool)
                        .await?;
                    tracing::info!("Updated credentials for camera {}", id);
                }
            } else if let (Some(user), Some(pass)) = (&req.username, &req.password) {
                // Insert new credentials
                sqlx::query("INSERT INTO camera_credentials (camera_id, username_encrypted, password_encrypted) VALUES (?, ?, ?)")
                    .bind(id.to_string())
                    .bind(user.as_bytes())
                    .bind(pass.as_bytes())
                    .execute(&self.pool)
                    .await?;
                tracing::info!("Inserted credentials for camera {}", id);
            }
        }

        Ok(())
    }

    /// Delete camera
    pub async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM camera_credentials WHERE camera_id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        
        sqlx::query("DELETE FROM cameras WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Get camera credentials
    pub async fn get_credentials(&self, id: Uuid) -> anyhow::Result<Option<(String, String)>> {
        let row = sqlx::query("SELECT username_encrypted, password_encrypted FROM camera_credentials WHERE camera_id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;

        let creds = row.map(|r| {
            let username: Vec<u8> = r.get("username_encrypted");
            let password: Vec<u8> = r.get("password_encrypted");
            (
                String::from_utf8_lossy(&username).to_string(),
                String::from_utf8_lossy(&password).to_string(),
            )
        });

        Ok(creds)
    }

    fn row_to_camera(&self, r: sqlx::sqlite::SqliteRow) -> Option<Camera> {
        let status_str: String = r.get("status");
        let status = match status_str.as_str() {
            "online" => CameraStatus::Online,
            "offline" => CameraStatus::Offline,
            "error" => CameraStatus::Error,
            _ => CameraStatus::Unknown,
        };

        Some(Camera {
            id: Uuid::parse_str(r.get("id")).ok()?,
            node_id: Uuid::parse_str(r.get("node_id")).ok()?,
            name: r.get("name"),
            description: r.try_get("description").ok().flatten(),
            manufacturer: r.get("manufacturer"),
            model: r.get("model"),
            firmware: r.try_get("firmware").ok().flatten(),
            rtsp_url: r.get("rtsp_url"),
            onvif_url: r.try_get("onvif_url").ok().flatten(),
            recording_path: r.try_get("recording_path").ok().flatten(),
            connection_timeout_ms: r.try_get::<i64, _>("connection_timeout_ms").ok().map(|v| v as u32),
            latitude: r.try_get("latitude").ok().flatten(),
            longitude: r.try_get("longitude").ok().flatten(),
            notes: r.try_get("notes").ok().flatten(),
            transport: r.try_get("transport").ok().flatten(),
            status,
            enabled: r.get("enabled"),
            created_at: chrono::DateTime::parse_from_rfc3339(r.get("created_at"))
                .map(|d| d.with_timezone(&Utc)).ok()?,
            updated_at: chrono::DateTime::parse_from_rfc3339(r.get("updated_at"))
                .map(|d| d.with_timezone(&Utc)).ok()?,
        })
    }
}
