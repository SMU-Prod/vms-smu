//! Node configuration

use std::env;

pub struct NodeConfig {
    pub node_name: String,
    pub server_url: String,
    pub api_key: String,
    pub media_port: u16,
}

impl NodeConfig {
    pub fn from_env() -> Self {
        Self {
            node_name: env::var("NODE_NAME")
                .unwrap_or_else(|_| "node-1".to_string()),
            server_url: env::var("SERVER_URL")
                .unwrap_or_else(|_| "http://localhost:9095".to_string()),
            api_key: env::var("NODE_API_KEY")
                .unwrap_or_else(|_| "".to_string()),
            media_port: env::var("MEDIA_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8090),
        }
    }
}
