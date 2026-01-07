//! User model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User role for authorization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,      // Full access
    Operator,   // View + PTZ control
    Viewer,     // View only
}

impl Default for UserRole {
    fn default() -> Self {
        Self::Viewer
    }
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::Operator => "operator",
            Self::Viewer => "viewer",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "admin" => Self::Admin,
            "operator" => Self::Operator,
            _ => Self::Viewer,
        }
    }
}

/// User entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub name: String,
    pub email: Option<String>,
    pub role: UserRole,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(username: String, password_hash: String, name: String, role: UserRole) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            username,
            password_hash,
            name,
            email: None,
            role,
            enabled: true,
            created_at: now,
            updated_at: now,
            last_login: None,
        }
    }

    /// Check if user can access admin features
    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }

    /// Check if user can control PTZ
    pub fn can_control_ptz(&self) -> bool {
        matches!(self.role, UserRole::Admin | UserRole::Operator)
    }
}

/// Request to create new user
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub name: String,
    pub email: Option<String>,
    #[serde(default)]
    pub role: UserRole,
}

/// Request to update user
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<UserRole>,
    pub enabled: Option<bool>,
}

/// Request to change password
#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

/// Login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Login response with JWT token
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserPublic,
    pub expires_at: DateTime<Utc>,
}

/// Public user data (safe to expose)
#[derive(Debug, Serialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub email: Option<String>,
    pub role: UserRole,
    pub enabled: bool,
}

impl From<User> for UserPublic {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            name: user.name,
            email: user.email,
            role: user.role,
            enabled: user.enabled,
        }
    }
}
