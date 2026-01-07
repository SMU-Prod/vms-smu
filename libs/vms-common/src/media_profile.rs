//! Sistema de Perfis de Mídia
//!
//! Suporta múltiplos perfis por câmera para cenários diferentes:
//! - Alta qualidade para gravação
//! - Média qualidade para visualização ao vivo
//! - Baixa qualidade para mobile

use crate::types::{CameraId, FrameRate, Resolution};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ID único de um perfil de mídia
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MediaProfileId(pub Uuid);

impl MediaProfileId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for MediaProfileId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MediaProfileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Codec de vídeo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoCodec {
    /// Motion JPEG
    MJPEG,
    /// H.264/AVC
    H264,
    /// H.265/HEVC
    H265,
    /// AV1 (novo codec aberto)
    AV1,
}

impl Default for VideoCodec {
    fn default() -> Self {
        Self::H264
    }
}

/// Codec de áudio
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioCodec {
    /// PCM não comprimido
    PCM,
    /// G.711 µ-law
    G711U,
    /// G.711 A-law
    G711A,
    /// G.726
    G726,
    /// AAC
    AAC,
    /// Opus
    Opus,
}

impl Default for AudioCodec {
    fn default() -> Self {
        Self::AAC
    }
}

/// Modo de bitrate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BitrateMode {
    /// Bitrate constante
    CBR,
    /// Bitrate variável
    VBR,
}

impl Default for BitrateMode {
    fn default() -> Self {
        Self::VBR
    }
}

/// Uso pretendido do perfil
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaProfileUsage {
    /// Para gravação (alta qualidade)
    Recording,
    /// Para visualização ao vivo
    LiveView,
    /// Para mobile (baixa resolução)
    Mobile,
    /// Para analytics (menor FPS, suficiente para IA)
    Analytics,
    /// Troca automática por movimento
    MotionAware,
}

impl Default for MediaProfileUsage {
    fn default() -> Self {
        Self::LiveView
    }
}

/// Perfil de mídia completo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaProfile {
    /// ID único do perfil
    pub id: MediaProfileId,

    /// Nome do perfil
    pub name: String,

    /// Uso pretendido
    pub usage: MediaProfileUsage,

    /// Resolução de vídeo
    pub resolution: Resolution,

    /// Frame rate
    pub fps: FrameRate,

    /// Codec de vídeo
    pub video_codec: VideoCodec,

    /// Bitrate alvo (bits/s)
    pub bitrate_target: u32,

    /// Bitrate máximo (bits/s)
    pub bitrate_max: Option<u32>,

    /// Modo de bitrate
    pub bitrate_mode: BitrateMode,

    /// GOP (Group of Pictures) em frames
    pub gop_size: u32,

    /// Perfil do codec (ex: "main", "high", "baseline")
    pub codec_profile: Option<String>,

    /// Level do codec (ex: 4.0, 5.1)
    pub codec_level: Option<String>,

    /// Habilitar áudio
    pub audio_enabled: bool,

    /// Codec de áudio
    pub audio_codec: AudioCodec,

    /// Sample rate de áudio (Hz)
    pub audio_sample_rate: u32,

    /// Canais de áudio
    pub audio_channels: u8,

    /// Bitrate de áudio (bits/s)
    pub audio_bitrate: u32,

    /// Prioridade do perfil (maior = mais prioritário)
    pub priority: u8,

    /// Perfil está ativo
    pub is_active: bool,
}

impl MediaProfile {
    /// Cria um perfil padrão para gravação
    pub fn recording_default() -> Self {
        Self {
            id: MediaProfileId::new(),
            name: "Recording - High Quality".to_string(),
            usage: MediaProfileUsage::Recording,
            resolution: Resolution::FULL_HD,
            fps: FrameRate::new(25.0),
            video_codec: VideoCodec::H265,
            bitrate_target: 4_000_000,
            bitrate_max: Some(6_000_000),
            bitrate_mode: BitrateMode::VBR,
            gop_size: 50, // 2 segundos a 25fps
            codec_profile: Some("main".to_string()),
            codec_level: Some("4.1".to_string()),
            audio_enabled: true,
            audio_codec: AudioCodec::AAC,
            audio_sample_rate: 48000,
            audio_channels: 2,
            audio_bitrate: 128_000,
            priority: 10,
            is_active: true,
        }
    }

    /// Cria um perfil padrão para visualização ao vivo
    pub fn liveview_default() -> Self {
        Self {
            id: MediaProfileId::new(),
            name: "Live View - Balanced".to_string(),
            usage: MediaProfileUsage::LiveView,
            resolution: Resolution::FULL_HD,
            fps: FrameRate::new(25.0),
            video_codec: VideoCodec::H264,
            bitrate_target: 2_000_000,
            bitrate_max: Some(4_000_000),
            bitrate_mode: BitrateMode::VBR,
            gop_size: 25, // 1 segundo
            codec_profile: Some("main".to_string()),
            codec_level: Some("4.0".to_string()),
            audio_enabled: true,
            audio_codec: AudioCodec::AAC,
            audio_sample_rate: 48000,
            audio_channels: 1,
            audio_bitrate: 64_000,
            priority: 8,
            is_active: true,
        }
    }

    /// Cria um perfil padrão para mobile
    pub fn mobile_default() -> Self {
        Self {
            id: MediaProfileId::new(),
            name: "Mobile - Low Bandwidth".to_string(),
            usage: MediaProfileUsage::Mobile,
            resolution: Resolution::HD_720P,
            fps: FrameRate::new(15.0),
            video_codec: VideoCodec::H264,
            bitrate_target: 500_000,
            bitrate_max: Some(1_000_000),
            bitrate_mode: BitrateMode::VBR,
            gop_size: 30, // 2 segundos a 15fps
            codec_profile: Some("baseline".to_string()),
            codec_level: Some("3.1".to_string()),
            audio_enabled: true,
            audio_codec: AudioCodec::AAC,
            audio_sample_rate: 44100,
            audio_channels: 1,
            audio_bitrate: 32_000,
            priority: 5,
            is_active: true,
        }
    }

    /// Cria um perfil para analytics
    pub fn analytics_default() -> Self {
        Self {
            id: MediaProfileId::new(),
            name: "Analytics - AI Processing".to_string(),
            usage: MediaProfileUsage::Analytics,
            resolution: Resolution::HD_720P,
            fps: FrameRate::new(10.0),
            video_codec: VideoCodec::H264,
            bitrate_target: 1_000_000,
            bitrate_max: Some(2_000_000),
            bitrate_mode: BitrateMode::VBR,
            gop_size: 10, // 1 segundo a 10fps
            codec_profile: Some("main".to_string()),
            codec_level: Some("3.1".to_string()),
            audio_enabled: false,
            audio_codec: AudioCodec::AAC,
            audio_sample_rate: 44100,
            audio_channels: 1,
            audio_bitrate: 0,
            priority: 6,
            is_active: true,
        }
    }

    /// Builder: Define resolução
    pub fn with_resolution(mut self, resolution: Resolution) -> Self {
        self.resolution = resolution;
        self
    }

    /// Builder: Define FPS
    pub fn with_fps(mut self, fps: f64) -> Self {
        self.fps = FrameRate::new(fps);
        self
    }

    /// Builder: Define codec
    pub fn with_codec(mut self, codec: VideoCodec) -> Self {
        self.video_codec = codec;
        self
    }

    /// Builder: Define bitrate
    pub fn with_bitrate(mut self, target: u32, max: Option<u32>) -> Self {
        self.bitrate_target = target;
        self.bitrate_max = max;
        self
    }

    /// Calcula estimativa de espaço em disco por hora
    pub fn estimated_storage_per_hour_mb(&self) -> u64 {
        // Bitrate médio em bits/s * 3600 segundos / 8 bits / 1MB
        let video_bytes = (self.bitrate_target as u64 * 3600) / 8 / 1_000_000;
        let audio_bytes = if self.audio_enabled {
            (self.audio_bitrate as u64 * 3600) / 8 / 1_000_000
        } else {
            0
        };
        video_bytes + audio_bytes
    }
}

/// Mapeamento entre perfis e câmeras
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraMediaProfiles {
    pub camera_id: CameraId,
    pub profiles: Vec<MediaProfile>,

    /// Perfil ativo para gravação
    pub recording_profile_id: Option<MediaProfileId>,

    /// Perfil ativo para live view
    pub liveview_profile_id: Option<MediaProfileId>,

    /// Perfil ativo para mobile
    pub mobile_profile_id: Option<MediaProfileId>,

    /// Perfil ativo para analytics
    pub analytics_profile_id: Option<MediaProfileId>,

    /// Habilitar troca automática por movimento
    pub motion_switching_enabled: bool,

    /// Perfil a usar quando movimento detectado
    pub motion_profile_id: Option<MediaProfileId>,

    /// Perfil a usar quando sem movimento
    pub no_motion_profile_id: Option<MediaProfileId>,
}

impl CameraMediaProfiles {
    pub fn new(camera_id: CameraId) -> Self {
        let recording = MediaProfile::recording_default();
        let liveview = MediaProfile::liveview_default();
        let mobile = MediaProfile::mobile_default();
        let analytics = MediaProfile::analytics_default();

        Self {
            camera_id,
            recording_profile_id: Some(recording.id),
            liveview_profile_id: Some(liveview.id),
            mobile_profile_id: Some(mobile.id),
            analytics_profile_id: Some(analytics.id),
            profiles: vec![recording, liveview, mobile, analytics],
            motion_switching_enabled: false,
            motion_profile_id: None,
            no_motion_profile_id: None,
        }
    }

    /// Obtém perfil por uso
    pub fn get_profile_for_usage(&self, usage: MediaProfileUsage) -> Option<&MediaProfile> {
        let target_id = match usage {
            MediaProfileUsage::Recording => self.recording_profile_id,
            MediaProfileUsage::LiveView => self.liveview_profile_id,
            MediaProfileUsage::Mobile => self.mobile_profile_id,
            MediaProfileUsage::Analytics => self.analytics_profile_id,
            MediaProfileUsage::MotionAware => self.motion_profile_id,
        };

        target_id.and_then(|id| self.profiles.iter().find(|p| p.id == id))
    }

    /// Adiciona um perfil customizado
    pub fn add_profile(&mut self, profile: MediaProfile) {
        self.profiles.push(profile);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_profile_defaults() {
        let recording = MediaProfile::recording_default();
        assert_eq!(recording.video_codec, VideoCodec::H265);
        assert_eq!(recording.resolution, Resolution::FULL_HD);

        let mobile = MediaProfile::mobile_default();
        assert_eq!(mobile.video_codec, VideoCodec::H264);
        assert_eq!(mobile.resolution, Resolution::HD_720P);
    }

    #[test]
    fn test_storage_estimation() {
        let profile = MediaProfile::recording_default();
        let mb_per_hour = profile.estimated_storage_per_hour_mb();
        // 4Mbps * 3600s / 8 / 1MB = 1800 MB + audio
        assert!(mb_per_hour > 1700 && mb_per_hour < 2000);
    }

    #[test]
    fn test_camera_profiles() {
        let camera_id = CameraId::new();
        let profiles = CameraMediaProfiles::new(camera_id);

        assert!(profiles.get_profile_for_usage(MediaProfileUsage::Recording).is_some());
        assert!(profiles.get_profile_for_usage(MediaProfileUsage::Mobile).is_some());
    }
}
