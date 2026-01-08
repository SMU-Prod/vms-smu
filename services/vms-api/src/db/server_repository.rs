//! Server database repository

use anyhow::Result;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::models::server::{Server, ServerStatus};

pub struct ServerRepository {
    pool: SqlitePool,
}

impl ServerRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Create servers table
    pub async fn create_table(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS servers (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                ip TEXT NOT NULL,
                port INTEGER NOT NULL DEFAULT 9094,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'offline',
                enabled BOOLEAN NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                last_seen TEXT
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Create new server
    pub async fn create(&self, server: &Server) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO servers (
                id, name, ip, port, username, password, status,
                enabled, created_at, updated_at, last_seen
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(server.id.to_string())
        .bind(&server.name)
        .bind(&server.ip)
        .bind(server.port as i64)
        .bind(&server.username)
        .bind(&server.password)
        .bind(server.status.as_str())
        .bind(server.enabled)
        .bind(server.created_at.to_rfc3339())
        .bind(server.updated_at.to_rfc3339())
        .bind(server.last_seen.map(|dt| dt.to_rfc3339()))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get all servers
    pub async fn list(&self) -> Result<Vec<Server>> {
        let rows = sqlx::query("SELECT * FROM servers ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let servers = rows.iter().filter_map(|row| self.row_to_server(row)).collect();
        Ok(servers)
    }

    /// Get server by ID
    pub async fn get(&self, id: Uuid) -> Result<Option<Server>> {
        let row = sqlx::query("SELECT * FROM servers WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.and_then(|row| self.row_to_server(&row)))
    }

    /// Update server
    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        ip: Option<String>,
        port: Option<u16>,
        username: Option<String>,
        password: Option<String>,
        enabled: Option<bool>,
    ) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();

        if let Some(n) = name {
            sqlx::query("UPDATE servers SET name = ?, updated_at = ? WHERE id = ?")
                .bind(n)
                .bind(&now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }

        if let Some(i) = ip {
            sqlx::query("UPDATE servers SET ip = ?, updated_at = ? WHERE id = ?")
                .bind(i)
                .bind(&now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }

        if let Some(p) = port {
            sqlx::query("UPDATE servers SET port = ?, updated_at = ? WHERE id = ?")
                .bind(p as i64)
                .bind(&now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }

        if let Some(u) = username {
            sqlx::query("UPDATE servers SET username = ?, updated_at = ? WHERE id = ?")
                .bind(u)
                .bind(&now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }

        if let Some(p) = password {
            sqlx::query("UPDATE servers SET password = ?, updated_at = ? WHERE id = ?")
                .bind(p)
                .bind(&now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }

        if let Some(e) = enabled {
            sqlx::query("UPDATE servers SET enabled = ?, updated_at = ? WHERE id = ?")
                .bind(e)
                .bind(&now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    /// Update server status
    pub async fn update_status(&self, id: Uuid, status: ServerStatus) -> Result<()> {
        sqlx::query("UPDATE servers SET status = ?, last_seen = ?, updated_at = ? WHERE id = ?")
            .bind(status.as_str())
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Delete server
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM servers WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Convert SQLite row to Server
    fn row_to_server(&self, row: &sqlx::sqlite::SqliteRow) -> Option<Server> {
        Some(Server {
            id: Uuid::parse_str(row.get("id")).ok()?,
            name: row.get("name"),
            ip: row.get("ip"),
            port: row.get::<i64, _>("port") as u16,
            username: row.get("username"),
            password: row.get("password"),
            status: ServerStatus::from_str(row.get("status")),
            enabled: row.get("enabled"),
            created_at: chrono::DateTime::parse_from_rfc3339(row.get("created_at"))
                .ok()?
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(row.get("updated_at"))
                .ok()?
                .with_timezone(&chrono::Utc),
            last_seen: row
                .get::<Option<String>, _>("last_seen")
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc)),
        })
    }
}
