//! Communication with vms_server

use serde::{Deserialize, Serialize};
use vms_core::{NodeCommand, NodeHeartbeat, NodeStatus, RegisterNodeRequest, NodeCapabilities};
use uuid::Uuid;
use std::time::Duration;

/// Register response from server
#[derive(Debug, Deserialize)]
pub struct RegisterResponse {
    pub node_id: Uuid,
    pub api_key: String,
}

/// Client for server communication
pub struct ServerClient {
    http: reqwest::Client,
    server_url: String,
    api_key: Option<String>,
    node_id: Option<Uuid>,
}

impl ServerClient {
    pub fn new(server_url: String) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http,
            server_url,
            api_key: None,
            node_id: None,
        }
    }

    /// Get node ID
    pub fn node_id(&self) -> Option<Uuid> {
        self.node_id
    }

    /// Register this node with the server
    pub async fn register(&mut self, name: String, ip: String, port: u16) -> anyhow::Result<Uuid> {
        let req = RegisterNodeRequest {
            name: name.clone(),
            ip,
            port,
            capabilities: NodeCapabilities {
                max_cameras: 16,
                supports_gpu: true,
                supports_recording: true,
                supports_ai: false,
            },
        };

        let url = format!("{}/api/v1/nodes", self.server_url);
        
        let response = self.http
            .post(&url)
            .json(&req)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("Registration failed: {}", error_text));
        }

        let reg_response: RegisterResponse = response.json().await?;
        
        self.node_id = Some(reg_response.node_id);
        self.api_key = Some(reg_response.api_key);

        tracing::info!("Registered as node {} with ID {}", name, reg_response.node_id);
        Ok(reg_response.node_id)
    }

    /// Send heartbeat to server
    pub async fn heartbeat(&self, status: NodeStatus, active_sessions: u32) -> anyhow::Result<()> {
        let Some(node_id) = self.node_id else {
            return Err(anyhow::anyhow!("Node not registered"));
        };

        let heartbeat = NodeHeartbeat {
            node_id,
            status,
            active_sessions,
            cpu_usage: Self::get_cpu_usage(),
            memory_usage: Self::get_memory_usage(),
        };

        let url = format!("{}/api/v1/nodes/{}/heartbeat", self.server_url, node_id);
        
        let response = self.http
            .post(&url)
            .header("X-Api-Key", self.api_key.as_deref().unwrap_or(""))
            .json(&heartbeat)
            .send()
            .await?;

        if !response.status().is_success() {
            tracing::warn!("Heartbeat failed: {}", response.status());
        }

        Ok(())
    }

    /// Start heartbeat loop
    pub async fn start_heartbeat_loop(client: std::sync::Arc<tokio::sync::RwLock<Self>>, interval_secs: u64) {
        let interval = Duration::from_secs(interval_secs);
        
        loop {
            tokio::time::sleep(interval).await;
            
            let guard = client.read().await;
            if let Err(e) = guard.heartbeat(NodeStatus::Online, 0).await {
                tracing::warn!("Heartbeat error: {}", e);
            }
        }
    }

    fn get_cpu_usage() -> f32 {
        // TODO: Implement actual CPU monitoring
        0.0
    }

    fn get_memory_usage() -> f32 {
        // TODO: Implement actual memory monitoring
        0.0
    }
}
