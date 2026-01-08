//! Node repository

use chrono::Utc;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use vms_core::{Node, NodeStatus, NodeCapabilities, RegisterNodeRequest};

pub struct NodeRepository {
    pool: SqlitePool,
}

impl NodeRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Register a new node
    pub async fn register(&self, req: &RegisterNodeRequest) -> anyhow::Result<(Uuid, String)> {
        let id = Uuid::new_v4();
        let api_key = format!("node-{}", Uuid::new_v4());
        let now = Utc::now().to_rfc3339();

        sqlx::query(r#"
            INSERT INTO nodes (id, name, ip, port, api_key, status, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(id.to_string())
        .bind(&req.name)
        .bind(&req.ip)
        .bind(req.port as i64)
        .bind(&api_key)
        .bind("offline")
        .bind(&now)
        .execute(&self.pool)
        .await?;

        tracing::info!("Registered node {} with ID {}", req.name, id);
        Ok((id, api_key))
    }

    /// Update node heartbeat
    pub async fn heartbeat(&self, id: Uuid, status: NodeStatus) -> anyhow::Result<()> {
        let now = Utc::now().to_rfc3339();
        let status_str = match status {
            NodeStatus::Online => "online",
            NodeStatus::Offline => "offline",
            NodeStatus::Degraded => "degraded",
        };

        sqlx::query("UPDATE nodes SET status = ?, last_seen = ? WHERE id = ?")
            .bind(status_str)
            .bind(&now)
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// List all nodes
    pub async fn list(&self) -> anyhow::Result<Vec<Node>> {
        let rows = sqlx::query("SELECT * FROM nodes ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let nodes = rows.into_iter().filter_map(|r| {
            let status_str: String = r.get("status");
            let status = match status_str.as_str() {
                "online" => NodeStatus::Online,
                "degraded" => NodeStatus::Degraded,
                _ => NodeStatus::Offline,
            };

            Some(Node {
                id: Uuid::parse_str(r.get("id")).ok()?,
                name: r.get("name"),
                ip: r.get("ip"),
                port: r.get::<i64, _>("port") as u16,
                api_key: r.get("api_key"),
                status,
                capabilities: NodeCapabilities::default(),
                last_seen: r.try_get::<String, _>("last_seen").ok()
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get("created_at"))
                    .map(|d| d.with_timezone(&Utc))
                    .ok()?,
            })
        }).collect();

        Ok(nodes)
    }

    /// Find node by ID
    pub async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Node>> {
        let row = sqlx::query("SELECT * FROM nodes WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;

        let node = row.and_then(|r| {
            let status_str: String = r.get("status");
            let status = match status_str.as_str() {
                "online" => NodeStatus::Online,
                "degraded" => NodeStatus::Degraded,
                _ => NodeStatus::Offline,
            };

            Some(Node {
                id: Uuid::parse_str(r.get("id")).ok()?,
                name: r.get("name"),
                ip: r.get("ip"),
                port: r.get::<i64, _>("port") as u16,
                api_key: r.get("api_key"),
                status,
                capabilities: NodeCapabilities::default(),
                last_seen: r.try_get::<String, _>("last_seen").ok()
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get("created_at"))
                    .map(|d| d.with_timezone(&Utc))
                    .ok()?,
            })
        });

        Ok(node)
    }

    /// Find node by API key
    pub async fn find_by_api_key(&self, api_key: &str) -> anyhow::Result<Option<Uuid>> {
        let row = sqlx::query("SELECT id FROM nodes WHERE api_key = ?")
            .bind(api_key)
            .fetch_optional(&self.pool)
            .await?;

        let id = row.and_then(|r| {
            let id_str: String = r.get("id");
            Uuid::parse_str(&id_str).ok()
        });

        Ok(id)
    }

    /// Delete node
    pub async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM nodes WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
