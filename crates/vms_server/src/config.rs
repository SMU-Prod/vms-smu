//! Server configuration

use std::env;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:vms.db?mode=rwc".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "vms-enterprise-secret-key-change-in-production".to_string()),
            jwt_expiry_hours: env::var("JWT_EXPIRY_HOURS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(24),
            server_port: env::var("PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(9095),
        }
    }
}
