//! User repository

use chrono::{DateTime, Utc};
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use vms_core::{User, Role};

pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Find user by email
    pub async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<(User, String)>> {
        let row = sqlx::query(
            "SELECT id, email, name, password_hash, role, enabled, created_at FROM users WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        let user = row.map(|r| {
            let role_str: String = r.get("role");
            let role = match role_str.as_str() {
                "admin" => Role::Admin,
                "operator" => Role::Operator,
                _ => Role::Viewer,
            };
            
            let user = User {
                id: Uuid::parse_str(r.get("id")).unwrap_or_default(),
                email: r.get("email"),
                name: r.get("name"),
                role,
                enabled: r.get("enabled"),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get("created_at"))
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            };
            let password_hash: String = r.get("password_hash");
            (user, password_hash)
        });

        Ok(user)
    }

    /// Find user by ID
    pub async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<User>> {
        let row = sqlx::query(
            "SELECT id, email, name, role, enabled, created_at FROM users WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        let user = row.map(|r| {
            let role_str: String = r.get("role");
            let role = match role_str.as_str() {
                "admin" => Role::Admin,
                "operator" => Role::Operator,
                _ => Role::Viewer,
            };
            
            User {
                id: Uuid::parse_str(r.get("id")).unwrap_or_default(),
                email: r.get("email"),
                name: r.get("name"),
                role,
                enabled: r.get("enabled"),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get("created_at"))
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }
        });

        Ok(user)
    }

    /// List all users
    pub async fn list(&self) -> anyhow::Result<Vec<User>> {
        let rows = sqlx::query(
            "SELECT id, email, name, role, enabled, created_at FROM users ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let users = rows.into_iter().filter_map(|r| {
            let role_str: String = r.get("role");
            let role = match role_str.as_str() {
                "admin" => Role::Admin,
                "operator" => Role::Operator,
                _ => Role::Viewer,
            };
            
            Some(User {
                id: Uuid::parse_str(r.get("id")).ok()?,
                email: r.get("email"),
                name: r.get("name"),
                role,
                enabled: r.get("enabled"),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get("created_at"))
                    .map(|d| d.with_timezone(&Utc))
                    .ok()?,
            })
        }).collect();

        Ok(users)
    }
}
