//! Tipos relacionados a câmeras

use crate::types::{CameraId, FrameRate, Resolution};
use serde::{Deserialize, Serialize};

/// Status de uma câmera
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CameraStatus {
    /// Câmera online e funcionando
    Online,
    /// Câmera offline
    Offline,
    /// Erro de conexão
    Error,
    /// Conectando...
    Connecting,
}

/// Tipo de protocolo da câmera
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CameraProtocol {
    /// RTSP (Real Time Streaming Protocol)
    RTSP,
    /// ONVIF
    ONVIF,
    /// HTTP/MJPEG
    MJPEG,
}

/// Configuração de uma câmera
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    /// ID único da câmera
    pub id: CameraId,

    /// Nome amigável
    pub name: String,

    /// URL de conexão (ex: rtsp://192.168.1.100:554/stream)
    pub url: String,

    /// Protocolo
    pub protocol: CameraProtocol,

    /// Usuário para autenticação
    pub username: Option<String>,

    /// Senha para autenticação
    pub password: Option<String>,

    /// Resolução desejada
    pub resolution: Resolution,

    /// Frame rate desejado
    pub fps: FrameRate,

    /// Habilitar gravação
    pub recording_enabled: bool,

    /// Habilitar análise de IA
    pub ai_enabled: bool,
}

impl CameraConfig {
    /// Cria uma nova configuração de câmera
    pub fn new(name: String, url: String) -> Self {
        Self {
            id: CameraId::new(),
            name,
            url,
            protocol: CameraProtocol::RTSP,
            username: None,
            password: None,
            resolution: Resolution::FULL_HD,
            fps: FrameRate::new(25.0),
            recording_enabled: true,
            ai_enabled: false,
        }
    }

    /// Define credenciais de autenticação
    pub fn with_credentials(mut self, username: String, password: String) -> Self {
        self.username = Some(username);
        self.password = Some(password);
        self
    }

    /// Define resolução
    pub fn with_resolution(mut self, resolution: Resolution) -> Self {
        self.resolution = resolution;
        self
    }

    /// Habilita IA
    pub fn with_ai(mut self, enabled: bool) -> Self {
        self.ai_enabled = enabled;
        self
    }
}

/// Informações de uma câmera
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraInfo {
    pub id: CameraId,
    pub name: String,
    pub status: CameraStatus,
    pub current_fps: f64,
    pub current_bitrate: u64, // bits por segundo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_config_builder() {
        let config = CameraConfig::new(
            "Camera 1".to_string(),
            "rtsp://192.168.1.100:554/stream".to_string(),
        )
        .with_credentials("admin".to_string(), "password".to_string())
        .with_ai(true);

        assert_eq!(config.name, "Camera 1");
        assert!(config.ai_enabled);
        assert_eq!(config.username, Some("admin".to_string()));
    }
}
