//! Error types para o VMS

use thiserror::Error;

/// Tipo de erro principal do VMS
#[derive(Error, Debug)]
pub enum Error {
    #[error("Camera error: {0}")]
    Camera(String),

    #[error("Stream error: {0}")]
    Stream(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("AI processing error: {0}")]
    AI(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

/// Result type usando o Error do VMS
pub type Result<T> = std::result::Result<T, Error>;
