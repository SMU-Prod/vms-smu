//! Tipos básicos compartilhados

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ID único de uma câmera
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CameraId(pub Uuid);

impl CameraId {
    /// Cria um novo ID de câmera
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Cria ID de câmera a partir de UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Retorna o UUID interno
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for CameraId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for CameraId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// ID único de um stream
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StreamId(pub Uuid);

impl StreamId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Cria ID de stream a partir de UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Retorna o UUID interno
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for StreamId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for StreamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for StreamId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

/// Resolução de vídeo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Resolution {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub const HD_720P: Self = Self::new(1280, 720);
    pub const FULL_HD: Self = Self::new(1920, 1080);
    pub const UHD_4K: Self = Self::new(3840, 2160);
    pub const UHD_8K: Self = Self::new(7680, 4320);
}

/// Frame rate (FPS)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FrameRate(pub f64);

impl FrameRate {
    pub fn new(fps: f64) -> Self {
        Self(fps)
    }

    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

/// Timestamp com precisão de microssegundos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timestamp(pub DateTime<Utc>);

impl Timestamp {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }

    pub fn as_datetime(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_id_creation() {
        let id1 = CameraId::new();
        let id2 = CameraId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_resolutions() {
        assert_eq!(Resolution::FULL_HD.width, 1920);
        assert_eq!(Resolution::FULL_HD.height, 1080);
    }
}
