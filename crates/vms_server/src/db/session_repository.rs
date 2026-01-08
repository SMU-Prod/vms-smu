//! Session repository for live sessions and recordings

use chrono::{DateTime, Duration, Utc};
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use vms_core::{LiveSession, SessionStatus, StreamProfile};

pub struct SessionRepository {
    pool: SqlitePool,
}

impl SessionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a new live session
    pub async fn create(
        &self,
        user_id: Uuid,
        camera_id: Uuid,
        node_id: Uuid,
        profile: StreamProfile,
        ttl_minutes: i64,
    ) -> anyhow::Result<LiveSession> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let expires_at = now + Duration::minutes(ttl_minutes);

        sqlx::query(r#"
            INSERT INTO live_sessions (id, user_id, camera_id, node_id, profile, status, started_at, expires_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(id.to_string())
        .bind(user_id.to_string())
        .bind(camera_id.to_string())
        .bind(node_id.to_string())
        .bind(serde_json::to_string(&profile).unwrap_or_default())
        .bind("pending")
        .bind(now.to_rfc3339())
        .bind(expires_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(LiveSession {
            id,
            user_id,
            camera_id,
            node_id,
            profile,
            stream_url: None,
            status: SessionStatus::Pending,
            started_at: now,
            expires_at,
        })
    }

    /// Update session with stream URL and status
    pub async fn activate(&self, id: Uuid, stream_url: String) -> anyhow::Result<()> {
        sqlx::query("UPDATE live_sessions SET stream_url = ?, status = ? WHERE id = ?")
            .bind(&stream_url)
            .bind("active")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Mark session as expired
    pub async fn expire(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("UPDATE live_sessions SET status = ? WHERE id = ?")
            .bind("expired")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Find session by ID
    pub async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<LiveSession>> {
        let row = sqlx::query("SELECT * FROM live_sessions WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.and_then(|r| self.row_to_session(r)))
    }

    /// List active sessions
    pub async fn list_active(&self) -> anyhow::Result<Vec<LiveSession>> {
        let now = Utc::now().to_rfc3339();
        let rows = sqlx::query("SELECT * FROM live_sessions WHERE status = 'active' AND expires_at > ? ORDER BY started_at DESC")
            .bind(&now)
            .fetch_all(&self.pool)
            .await?;

        let sessions = rows.into_iter().filter_map(|r| self.row_to_session(r)).collect();
        Ok(sessions)
    }

    /// List sessions by user
    pub async fn list_by_user(&self, user_id: Uuid) -> anyhow::Result<Vec<LiveSession>> {
        let rows = sqlx::query("SELECT * FROM live_sessions WHERE user_id = ? ORDER BY started_at DESC LIMIT 50")
            .bind(user_id.to_string())
            .fetch_all(&self.pool)
            .await?;

        let sessions = rows.into_iter().filter_map(|r| self.row_to_session(r)).collect();
        Ok(sessions)
    }

    /// Cleanup expired sessions
    pub async fn cleanup_expired(&self) -> anyhow::Result<u64> {
        let now = Utc::now().to_rfc3339();
        let result = sqlx::query("UPDATE live_sessions SET status = 'expired' WHERE status = 'active' AND expires_at < ?")
            .bind(&now)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    fn row_to_session(&self, r: sqlx::sqlite::SqliteRow) -> Option<LiveSession> {
        let status_str: String = r.get("status");
        let status = match status_str.as_str() {
            "active" => SessionStatus::Active,
            "expired" => SessionStatus::Expired,
            "error" => SessionStatus::Error,
            _ => SessionStatus::Pending,
        };

        let profile_str: String = r.try_get("profile").ok()?;
        let profile: StreamProfile = serde_json::from_str(&profile_str).unwrap_or_default();

        Some(LiveSession {
            id: Uuid::parse_str(r.get("id")).ok()?,
            user_id: Uuid::parse_str(r.get("user_id")).ok()?,
            camera_id: Uuid::parse_str(r.get("camera_id")).ok()?,
            node_id: Uuid::parse_str(r.get("node_id")).ok()?,
            profile,
            stream_url: r.try_get("stream_url").ok().flatten(),
            status,
            started_at: chrono::DateTime::parse_from_rfc3339(r.get("started_at"))
                .map(|d| d.with_timezone(&Utc)).ok()?,
            expires_at: chrono::DateTime::parse_from_rfc3339(r.get("expires_at"))
                .map(|d| d.with_timezone(&Utc)).ok()?,
        })
    }
}
