//! Camera types
//! Structs e enums para representar câmeras

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Representa uma câmera descoberta ou configurada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    /// ID único da câmera
    pub id: Uuid,
    /// Nome amigável
    pub name: String,
    /// URL do serviço ONVIF
    pub url: String,
    /// Fabricante (ex: Hikvision, Intelbras, TP-Link)
    pub manufacturer: Option<String>,
    /// Modelo da câmera
    pub model: Option<String>,
    /// Número de série
    pub serial_number: Option<String>,
    /// ID de hardware
    pub hardware_id: Option<String>,
    /// Status atual
    pub status: CameraStatus,
    /// Profiles de mídia disponíveis
    pub profiles: Vec<CameraProfile>,
}

/// Status da câmera
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CameraStatus {
    /// Câmera online e funcionando
    Online,
    /// Câmera offline ou sem conexão
    Offline,
    /// Erro de conexão ou autenticação
    Error,
    /// Gravando
    Recording,
}

/// Profile de mídia da câmera (compatível com device.rs)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraProfile {
    /// Token do profile ONVIF
    pub token: String,
    /// Nome do profile (ex: "Main Stream", "Sub Stream")
    pub name: String,
    /// Codec de vídeo (ex: "H264", "H265")
    pub video_encoding: String,
    /// Resolução (largura, altura)
    pub resolution: (u32, u32),
    /// FPS configurado
    pub framerate: f32,
}

/// Request para adicionar uma câmera
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCameraRequest {
    /// Nome da câmera
    pub name: String,
    /// URL do serviço ONVIF
    pub onvif_url: String,
    /// Usuário para autenticação
    pub username: Option<String>,
    /// Senha para autenticação
    pub password: Option<String>,
}

/// Comando PTZ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtzCommand {
    /// Profile token
    pub profile_token: String,
    /// Movimento horizontal (-1.0 a 1.0)
    pub pan: f32,
    /// Movimento vertical (-1.0 a 1.0)
    pub tilt: f32,
    /// Zoom (-1.0 a 1.0)
    pub zoom: f32,
}

impl Camera {
    /// Cria uma nova câmera com valores padrão
    pub fn new(name: String, url: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            url,
            manufacturer: None,
            model: None,
            serial_number: None,
            hardware_id: None,
            status: CameraStatus::Offline,
            profiles: Vec::new(),
        }
    }

    /// Retorna o profile principal
    pub fn main_profile(&self) -> Option<&CameraProfile> {
        self.profiles.first()
    }

    /// Verifica se a câmera está online
    pub fn is_online(&self) -> bool {
        self.status == CameraStatus::Online || self.status == CameraStatus::Recording
    }
}

impl std::fmt::Display for CameraStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CameraStatus::Online => write!(f, "online"),
            CameraStatus::Offline => write!(f, "offline"),
            CameraStatus::Error => write!(f, "error"),
            CameraStatus::Recording => write!(f, "recording"),
        }
    }
}
