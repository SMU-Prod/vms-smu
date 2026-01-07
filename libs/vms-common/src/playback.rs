//! Sistema de Playback e Timeline
//!
//! Gerencia reprodução de vídeo gravado, timeline, seek, exportação.

use crate::types::CameraId;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// ID de sessão de playback
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlaybackSessionId(pub Uuid);

impl PlaybackSessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for PlaybackSessionId {
    fn default() -> Self {
        Self::new()
    }
}

/// ID de bookmark
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BookmarkId(pub Uuid);

impl BookmarkId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for BookmarkId {
    fn default() -> Self {
        Self::new()
    }
}

/// ID de exportação
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExportId(pub Uuid);

impl ExportId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ExportId {
    fn default() -> Self {
        Self::new()
    }
}

/// Modo de reprodução
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaybackMode {
    /// Normal (1x)
    Normal,
    /// Em pausa
    Paused,
    /// Velocidade aumentada
    FastForward,
    /// Reverso
    Reverse,
    /// Frame a frame
    FrameByFrame,
}

impl Default for PlaybackMode {
    fn default() -> Self {
        Self::Normal
    }
}

/// Velocidade de reprodução
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PlaybackSpeed {
    /// 1/8x
    Slowest,
    /// 1/4x
    VerySlowly,
    /// 1/2x
    Slow,
    /// 1x normal
    Normal,
    /// 2x
    Fast,
    /// 4x
    Faster,
    /// 8x
    VeryFast,
    /// 16x
    Fastest,
    /// 32x
    Ultra,
}

impl PlaybackSpeed {
    pub fn multiplier(&self) -> f64 {
        match self {
            Self::Slowest => 0.125,
            Self::VerySlowly => 0.25,
            Self::Slow => 0.5,
            Self::Normal => 1.0,
            Self::Fast => 2.0,
            Self::Faster => 4.0,
            Self::VeryFast => 8.0,
            Self::Fastest => 16.0,
            Self::Ultra => 32.0,
        }
    }

    pub fn next_faster(&self) -> Self {
        match self {
            Self::Slowest => Self::VerySlowly,
            Self::VerySlowly => Self::Slow,
            Self::Slow => Self::Normal,
            Self::Normal => Self::Fast,
            Self::Fast => Self::Faster,
            Self::Faster => Self::VeryFast,
            Self::VeryFast => Self::Fastest,
            Self::Fastest => Self::Ultra,
            Self::Ultra => Self::Ultra,
        }
    }

    pub fn next_slower(&self) -> Self {
        match self {
            Self::Ultra => Self::Fastest,
            Self::Fastest => Self::VeryFast,
            Self::VeryFast => Self::Faster,
            Self::Faster => Self::Fast,
            Self::Fast => Self::Normal,
            Self::Normal => Self::Slow,
            Self::Slow => Self::VerySlowly,
            Self::VerySlowly => Self::Slowest,
            Self::Slowest => Self::Slowest,
        }
    }
}

impl Default for PlaybackSpeed {
    fn default() -> Self {
        Self::Normal
    }
}

/// Tipo de segmento na timeline
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimelineSegmentType {
    /// Gravação contínua
    Continuous,
    /// Gravação por movimento
    Motion,
    /// Gravação por evento
    Event,
    /// Sem gravação
    NoRecording,
    /// Falha/corrompido
    Corrupted,
}

/// Segmento de timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineSegment {
    /// Início do segmento
    pub start: DateTime<Utc>,

    /// Fim do segmento
    pub end: DateTime<Utc>,

    /// Tipo de segmento
    pub segment_type: TimelineSegmentType,

    /// Tem áudio
    pub has_audio: bool,

    /// Tamanho em bytes
    pub size_bytes: u64,

    /// Caminho do arquivo
    pub file_path: String,

    /// Índice no arquivo
    pub index_path: Option<String>,

    /// Metadados adicionais
    pub metadata: HashMap<String, String>,
}

impl TimelineSegment {
    pub fn duration(&self) -> Duration {
        self.end - self.start
    }

    pub fn contains(&self, dt: DateTime<Utc>) -> bool {
        dt >= self.start && dt <= self.end
    }
}

/// Timeline completa de uma câmera
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraTimeline {
    /// ID da câmera
    pub camera_id: CameraId,

    /// Intervalo de tempo disponível
    pub range_start: DateTime<Utc>,
    pub range_end: DateTime<Utc>,

    /// Segmentos de gravação
    pub segments: Vec<TimelineSegment>,

    /// Eventos nesta timeline
    pub events: Vec<TimelineEvent>,

    /// Bookmarks
    pub bookmarks: Vec<Bookmark>,
}

impl CameraTimeline {
    pub fn new(camera_id: CameraId) -> Self {
        Self {
            camera_id,
            range_start: Utc::now(),
            range_end: Utc::now(),
            segments: Vec::new(),
            events: Vec::new(),
            bookmarks: Vec::new(),
        }
    }

    /// Encontra segmentos que cobrem um período
    pub fn get_segments_for_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<&TimelineSegment> {
        self.segments
            .iter()
            .filter(|s| s.end >= start && s.start <= end)
            .collect()
    }

    /// Encontra o segmento que contém um timestamp
    pub fn get_segment_at(&self, dt: DateTime<Utc>) -> Option<&TimelineSegment> {
        self.segments.iter().find(|s| s.contains(dt))
    }

    /// Calcula taxa de cobertura (percentual de tempo com gravação)
    pub fn coverage_percent(&self) -> f64 {
        if self.range_end <= self.range_start {
            return 0.0;
        }

        let total_ms = (self.range_end - self.range_start).num_milliseconds() as f64;
        let recorded_ms: f64 = self
            .segments
            .iter()
            .filter(|s| s.segment_type != TimelineSegmentType::NoRecording)
            .map(|s| s.duration().num_milliseconds() as f64)
            .sum();

        (recorded_ms / total_ms) * 100.0
    }

    /// Espaço total ocupado
    pub fn total_size_bytes(&self) -> u64 {
        self.segments.iter().map(|s| s.size_bytes).sum()
    }
}

/// Evento na timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    /// Timestamp do evento
    pub timestamp: DateTime<Utc>,

    /// Tipo de evento
    pub event_type: String,

    /// Descrição
    pub description: String,

    /// Severidade (para coloração na UI)
    pub severity: String,

    /// Thumbnail
    pub thumbnail: Option<String>,
}

/// Bookmark de playback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    /// ID do bookmark
    pub id: BookmarkId,

    /// Câmera
    pub camera_id: CameraId,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Nome
    pub name: String,

    /// Descrição
    pub description: Option<String>,

    /// Criado por
    pub created_by: String,

    /// Data de criação
    pub created_at: DateTime<Utc>,

    /// Tags
    pub tags: Vec<String>,

    /// É público (visível para todos)
    pub is_public: bool,

    /// Thumbnail
    pub thumbnail: Option<String>,
}

impl Bookmark {
    pub fn new(camera_id: CameraId, timestamp: DateTime<Utc>, name: &str, created_by: &str) -> Self {
        Self {
            id: BookmarkId::new(),
            camera_id,
            timestamp,
            name: name.to_string(),
            description: None,
            created_by: created_by.to_string(),
            created_at: Utc::now(),
            tags: Vec::new(),
            is_public: true,
            thumbnail: None,
        }
    }
}

/// Estado de uma sessão de playback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackSession {
    /// ID da sessão
    pub id: PlaybackSessionId,

    /// Câmeras na sessão
    pub camera_ids: Vec<CameraId>,

    /// Modo de reprodução
    pub mode: PlaybackMode,

    /// Velocidade
    pub speed: PlaybackSpeed,

    /// Posição atual
    pub current_position: DateTime<Utc>,

    /// Início do período selecionado
    pub range_start: DateTime<Utc>,

    /// Fim do período selecionado
    pub range_end: DateTime<Utc>,

    /// Sincronização multi-câmera
    pub is_synchronized: bool,

    /// Mudo
    pub is_muted: bool,

    /// Volume (0-100)
    pub volume: u8,

    /// Zoom digital ativo
    pub digital_zoom: Option<DigitalZoom>,

    /// Filtros de enhancement
    pub video_filters: VideoFilters,

    /// Timelines carregadas
    pub timelines: HashMap<CameraId, CameraTimeline>,

    /// Criada em
    pub created_at: DateTime<Utc>,

    /// Usuário
    pub user_id: String,
}

impl PlaybackSession {
    pub fn new(user_id: &str) -> Self {
        Self {
            id: PlaybackSessionId::new(),
            camera_ids: Vec::new(),
            mode: PlaybackMode::Paused,
            speed: PlaybackSpeed::Normal,
            current_position: Utc::now() - Duration::hours(1),
            range_start: Utc::now() - Duration::hours(24),
            range_end: Utc::now(),
            is_synchronized: true,
            is_muted: false,
            volume: 100,
            digital_zoom: None,
            video_filters: VideoFilters::default(),
            timelines: HashMap::new(),
            created_at: Utc::now(),
            user_id: user_id.to_string(),
        }
    }

    /// Play
    pub fn play(&mut self) {
        self.mode = PlaybackMode::Normal;
    }

    /// Pause
    pub fn pause(&mut self) {
        self.mode = PlaybackMode::Paused;
    }

    /// Stop (volta ao início)
    pub fn stop(&mut self) {
        self.mode = PlaybackMode::Paused;
        self.current_position = self.range_start;
    }

    /// Fast forward
    pub fn fast_forward(&mut self) {
        self.mode = PlaybackMode::FastForward;
        self.speed = self.speed.next_faster();
    }

    /// Reverso
    pub fn reverse(&mut self) {
        self.mode = PlaybackMode::Reverse;
    }

    /// Próximo frame
    pub fn next_frame(&mut self) {
        self.mode = PlaybackMode::FrameByFrame;
        // Avança aproximadamente 1 frame (40ms para 25fps)
        self.current_position = self.current_position + Duration::milliseconds(40);
    }

    /// Frame anterior
    pub fn prev_frame(&mut self) {
        self.mode = PlaybackMode::FrameByFrame;
        self.current_position = self.current_position - Duration::milliseconds(40);
    }

    /// Seek para posição específica
    pub fn seek(&mut self, position: DateTime<Utc>) {
        self.current_position = position.max(self.range_start).min(self.range_end);
    }

    /// Seek relativo (em segundos)
    pub fn seek_relative(&mut self, seconds: i64) {
        let new_pos = if seconds >= 0 {
            self.current_position + Duration::seconds(seconds)
        } else {
            self.current_position - Duration::seconds(-seconds)
        };
        self.seek(new_pos);
    }
}

/// Zoom digital
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalZoom {
    /// Fator de zoom (1.0 = normal)
    pub factor: f32,

    /// Centro X (0-1)
    pub center_x: f32,

    /// Centro Y (0-1)
    pub center_y: f32,
}

impl DigitalZoom {
    pub fn new(factor: f32, center_x: f32, center_y: f32) -> Self {
        Self {
            factor: factor.max(1.0).min(10.0),
            center_x: center_x.max(0.0).min(1.0),
            center_y: center_y.max(0.0).min(1.0),
        }
    }
}

/// Filtros de vídeo para enhancement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFilters {
    /// Brilho (-100 a 100)
    pub brightness: i8,

    /// Contraste (-100 a 100)
    pub contrast: i8,

    /// Saturação (-100 a 100)
    pub saturation: i8,

    /// Gamma (0.1 a 3.0)
    pub gamma: f32,

    /// Sharpen (0 a 100)
    pub sharpen: u8,

    /// Denoising (0 a 100)
    pub denoise: u8,

    /// Grayscale
    pub grayscale: bool,

    /// Inverter cores
    pub invert: bool,

    /// Dewarping (para fisheye)
    pub dewarping: Option<DewarpingConfig>,
}

impl Default for VideoFilters {
    fn default() -> Self {
        Self {
            brightness: 0,
            contrast: 0,
            saturation: 0,
            gamma: 1.0,
            sharpen: 0,
            denoise: 0,
            grayscale: false,
            invert: false,
            dewarping: None,
        }
    }
}

/// Configuração de dewarping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DewarpingConfig {
    /// Modo de montagem (teto, parede, chão)
    pub mount_mode: MountMode,

    /// Modo de visualização
    pub view_mode: DewarpViewMode,

    /// Ângulo de pan
    pub pan: f32,

    /// Ângulo de tilt
    pub tilt: f32,

    /// Zoom
    pub zoom: f32,
}

/// Modo de montagem da câmera 360
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MountMode {
    Ceiling,
    Wall,
    Ground,
}

/// Modo de visualização dewarping
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DewarpViewMode {
    /// Fisheye original
    Original,
    /// Panorama 360
    Panorama,
    /// Single view retificado
    Single,
    /// Quad split
    Quad,
    /// PTZ virtual
    VirtualPTZ,
}

/// Formato de exportação
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    /// MKV nativo (mais rápido, preserva qualidade)
    MKV,
    /// MP4 (mais compatível)
    MP4,
    /// AVI
    AVI,
    /// WebM
    WebM,
    /// Sequência de imagens
    ImageSequence,
}

impl Default for ExportFormat {
    fn default() -> Self {
        Self::MP4
    }
}

/// Configuração de exportação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Câmeras a exportar
    pub camera_ids: Vec<CameraId>,

    /// Início
    pub start: DateTime<Utc>,

    /// Fim
    pub end: DateTime<Utc>,

    /// Formato
    pub format: ExportFormat,

    /// Incluir áudio
    pub include_audio: bool,

    /// Incluir timestamp overlay
    pub include_timestamp: bool,

    /// Incluir nome da câmera
    pub include_camera_name: bool,

    /// Recomprimir (transcodificar)
    pub transcode: bool,

    /// Resolução de saída (None = original)
    pub output_resolution: Option<(u32, u32)>,

    /// Bitrate de saída (None = automático)
    pub output_bitrate: Option<u32>,

    /// Criptografar
    pub encrypt: bool,

    /// Senha de criptografia
    pub encryption_password: Option<String>,

    /// Marca d'água
    pub watermark: Option<WatermarkConfig>,

    /// Diretório de destino
    pub output_dir: String,

    /// Nome do arquivo (sem extensão)
    pub output_filename: String,
}

impl ExportConfig {
    pub fn quick_export(camera_id: CameraId, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self {
            camera_ids: vec![camera_id],
            start,
            end,
            format: ExportFormat::MP4,
            include_audio: true,
            include_timestamp: true,
            include_camera_name: true,
            transcode: false,
            output_resolution: None,
            output_bitrate: None,
            encrypt: false,
            encryption_password: None,
            watermark: None,
            output_dir: "./exports".to_string(),
            output_filename: format!("export_{}", Utc::now().format("%Y%m%d_%H%M%S")),
        }
    }
}

/// Configuração de marca d'água
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatermarkConfig {
    /// Texto da marca d'água
    pub text: Option<String>,

    /// Imagem da marca d'água
    pub image_path: Option<String>,

    /// Posição (0-1 para x e y do canto superior esquerdo)
    pub position: (f32, f32),

    /// Opacidade (0-1)
    pub opacity: f32,
}

/// Status de exportação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportStatus {
    /// ID da exportação
    pub id: ExportId,

    /// Configuração
    pub config: ExportConfig,

    /// Progresso (0-100)
    pub progress_percent: f32,

    /// Status
    pub status: ExportJobStatus,

    /// Mensagem de erro (se houver)
    pub error_message: Option<String>,

    /// Caminho do arquivo gerado
    pub output_path: Option<String>,

    /// Tamanho do arquivo (bytes)
    pub output_size: Option<u64>,

    /// Iniciado em
    pub started_at: DateTime<Utc>,

    /// Completado em
    pub completed_at: Option<DateTime<Utc>>,
}

/// Status do job de exportação
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportJobStatus {
    Queued,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playback_speed() {
        let speed = PlaybackSpeed::Normal;
        assert_eq!(speed.multiplier(), 1.0);

        let faster = speed.next_faster();
        assert_eq!(faster.multiplier(), 2.0);
    }

    #[test]
    fn test_playback_session() {
        let mut session = PlaybackSession::new("admin");

        session.play();
        assert_eq!(session.mode, PlaybackMode::Normal);

        session.fast_forward();
        assert_eq!(session.speed, PlaybackSpeed::Fast);

        session.pause();
        assert_eq!(session.mode, PlaybackMode::Paused);
    }

    #[test]
    fn test_timeline_segment() {
        let now = Utc::now();
        let segment = TimelineSegment {
            start: now - Duration::hours(1),
            end: now,
            segment_type: TimelineSegmentType::Continuous,
            has_audio: true,
            size_bytes: 1_000_000,
            file_path: "/storage/camera1/2024-12-13/video_10.mkv".to_string(),
            index_path: Some("/storage/camera1/2024-12-13/index_10.json".to_string()),
            metadata: HashMap::new(),
        };

        assert!(segment.contains(now - Duration::minutes(30)));
        assert!(!segment.contains(now + Duration::hours(1)));
    }

    #[test]
    fn test_bookmark() {
        let camera_id = CameraId::new();
        let bookmark = Bookmark::new(
            camera_id,
            Utc::now(),
            "Incident #123",
            "admin",
        );

        assert!(bookmark.is_public);
        assert!(bookmark.tags.is_empty());
    }
}
