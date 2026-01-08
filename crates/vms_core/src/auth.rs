//! Authentication types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: Role,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}

/// User roles with permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Operator,
    Viewer,
}

impl Role {
    pub fn can_manage_cameras(&self) -> bool {
        matches!(self, Role::Admin)
    }

    pub fn can_manage_users(&self) -> bool {
        matches!(self, Role::Admin)
    }

    pub fn can_view_live(&self) -> bool {
        true
    }

    pub fn can_view_recordings(&self) -> bool {
        matches!(self, Role::Admin | Role::Operator)
    }
}

/// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,       // User ID
    pub email: String,
    pub role: Role,
    pub exp: i64,          // Expiration timestamp
    pub iat: i64,          // Issued at
}

/// Login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Login response
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub user: User,
}

/// Refresh token request
#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}
