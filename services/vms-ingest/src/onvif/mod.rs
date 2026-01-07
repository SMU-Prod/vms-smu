//! ONVIF Client Implementation
//! Suporta WS-Discovery, autenticação e controle de câmeras

pub mod discovery;
pub mod auth;
pub mod device;
pub mod media;
pub mod ptz;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Device information descoberto via WS-Discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnvifDevice {
    /// UUID do dispositivo
    pub uuid: String,
    /// Endereço IP
    pub ip: String,
    /// Porta
    pub port: u16,
    /// URL do serviço ONVIF
    pub service_url: String,
    /// Nome do dispositivo
    pub name: Option<String>,
    /// Fabricante
    pub manufacturer: Option<String>,
    /// Modelo
    pub model: Option<String>,
    /// Firmware
    pub firmware: Option<String>,
}

/// Perfil de mídia ONVIF
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaProfile {
    /// Token do perfil
    pub token: String,
    /// Nome do perfil
    pub name: String,
    /// URL RTSP do stream
    pub rtsp_uri: String,
    /// Resolução (width x height)
    pub resolution: (u32, u32),
    /// Codec de vídeo
    pub video_codec: String,
    /// Frame rate
    pub framerate: f32,
    /// Bitrate (kbps)
    pub bitrate: u32,
}

/// Cliente ONVIF
#[derive(Debug, Clone)]
pub struct OnvifClient {
    /// URL base do dispositivo
    pub device_url: String,
    /// Username para autenticação
    pub username: String,
    /// Password para autenticação
    pub password: String,
    /// HTTP client
    http_client: reqwest::Client,
}

impl OnvifClient {
    /// Cria novo cliente ONVIF
    pub fn new(device_url: String, username: String, password: String) -> Self {
        Self {
            device_url,
            username,
            password,
            http_client: reqwest::Client::new(),
        }
    }

    /// Obter informações do dispositivo
    pub async fn get_device_information(&self) -> Result<OnvifDevice> {
        device::get_device_information(self).await
    }

    /// Listar perfis de mídia disponíveis
    pub async fn get_profiles(&self) -> Result<Vec<MediaProfile>> {
        media::get_profiles(self).await
    }

    /// Obter URI RTSP de um perfil
    pub async fn get_stream_uri(&self, profile_token: &str) -> Result<String> {
        media::get_stream_uri(self, profile_token).await
    }

    /// Mover PTZ para posição absoluta
    pub async fn ptz_absolute_move(&self, profile_token: &str, x: f32, y: f32, zoom: f32) -> Result<()> {
        ptz::absolute_move(self, profile_token, x, y, zoom).await
    }

    /// PTZ continuous move
    pub async fn ptz_continuous_move(&self, profile_token: &str, x: f32, y: f32, zoom: f32) -> Result<()> {
        ptz::continuous_move(self, profile_token, x, y, zoom).await
    }

    /// Parar movimento PTZ
    pub async fn ptz_stop(&self, profile_token: &str) -> Result<()> {
        ptz::stop(self, profile_token).await
    }
}
