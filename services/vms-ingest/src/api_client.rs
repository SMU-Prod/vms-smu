//! Cliente HTTP para vms-api
//! Busca câmeras do banco de dados

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCamera {
    pub id: String,
    pub name: String,
    pub rtsp_url: String,
    #[serde(default)]
    pub onvif_url: Option<String>,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub resolution_width: u32,
    #[serde(default)]
    pub resolution_height: u32,
    #[serde(default)]
    pub framerate: f32,
    #[serde(default)]
    pub codec: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Busca todas as câmeras da API
    pub async fn get_cameras(&self) -> Result<Vec<ApiCamera>> {
        let url = format!("{}/api/v1/cameras", self.base_url);
        debug!("Fetching cameras from: {}", url);

        let response = self.client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("API returned error: {}", response.status());
        }

        let cameras: Vec<ApiCamera> = response.json().await?;
        info!("Fetched {} cameras from API", cameras.len());

        Ok(cameras)
    }

    /// Busca câmeras habilitadas
    pub async fn get_enabled_cameras(&self) -> Result<Vec<ApiCamera>> {
        let cameras = self.get_cameras().await?;
        let enabled: Vec<ApiCamera> = cameras
            .into_iter()
            .filter(|c| c.enabled)
            .collect();

        info!("Found {} enabled cameras", enabled.len());
        Ok(enabled)
    }
}
