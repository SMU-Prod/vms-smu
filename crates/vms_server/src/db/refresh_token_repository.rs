//! Refresh token repository

use chrono::{DateTime, Utc, Duration};
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use sha2::{Sha256, Digest};

pub struct RefreshTokenRepository {
    pool: SqlitePool,
}

impl RefreshTokenRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a new refresh token
    pub async fn create(&self, user_id: Uuid, token: &str, expires_in_days: i64) -> anyhow::Result<()> {
        let token_hash = Self::hash_token(token);
        let now = Utc::now();
        let expires_at = now + Duration::days(expires_in_days);

        sqlx::query(
            "INSERT INTO refresh_tokens (id, user_id, token_hash, expires_at, created_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(Uuid::new_v4().to_string())
        .bind(user_id.to_string())
        .bind(&token_hash)
        .bind(expires_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Validate and get user_id for refresh token
    pub async fn validate(&self, token: &str) -> anyhow::Result<Option<Uuid>> {
        let token_hash = Self::hash_token(token);
        let now = Utc::now().to_rfc3339();

        let row = sqlx::query(
            "SELECT user_id FROM refresh_tokens WHERE token_hash = ? AND expires_at > ?"
        )
        .bind(&token_hash)
        .bind(&now)
        .fetch_optional(&self.pool)
        .await?;

        let user_id = row.and_then(|r| {
            let id_str: String = r.get("user_id");
            Uuid::parse_str(&id_str).ok()
        });

        Ok(user_id)
    }

    /// Revoke a refresh token
    pub async fn revoke(&self, token: &str) -> anyhow::Result<()> {
        let token_hash = Self::hash_token(token);

        sqlx::query("DELETE FROM refresh_tokens WHERE token_hash = ?")
            .bind(&token_hash)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Revoke all refresh tokens for a user
    pub async fn revoke_all_for_user(&self, user_id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM refresh_tokens WHERE user_id = ?")
            .bind(user_id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Hash token with SHA256
    fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
