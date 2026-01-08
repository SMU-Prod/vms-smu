//! Error types

use thiserror::Error;
use uuid::Uuid;

/// VMS Core errors
#[derive(Debug, Error)]
pub enum VmsError {
    // Authentication
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("Token invalid")]
    TokenInvalid,
    
    #[error("Insufficient permissions")]
    Forbidden,
    
    // Resources
    #[error("User not found: {0}")]
    UserNotFound(Uuid),
    
    #[error("Camera not found: {0}")]
    CameraNotFound(Uuid),
    
    #[error("Node not found: {0}")]
    NodeNotFound(Uuid),
    
    #[error("Session not found: {0}")]
    SessionNotFound(Uuid),
    
    // Node
    #[error("Node offline: {0}")]
    NodeOffline(Uuid),
    
    #[error("Node command failed: {0}")]
    NodeCommandFailed(String),
    
    // Database
    #[error("Database error: {0}")]
    Database(String),
    
    // General
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
}

impl VmsError {
    pub fn status_code(&self) -> u16 {
        match self {
            VmsError::InvalidCredentials => 401,
            VmsError::TokenExpired => 401,
            VmsError::TokenInvalid => 401,
            VmsError::Forbidden => 403,
            VmsError::UserNotFound(_) => 404,
            VmsError::CameraNotFound(_) => 404,
            VmsError::NodeNotFound(_) => 404,
            VmsError::SessionNotFound(_) => 404,
            VmsError::NodeOffline(_) => 503,
            VmsError::NodeCommandFailed(_) => 502,
            VmsError::Database(_) => 500,
            VmsError::Internal(_) => 500,
            VmsError::Validation(_) => 400,
        }
    }
}
