//! VMS Common Library
//!
//! Tipos, estruturas e utilitários compartilhados entre todos os serviços do VMS.
//!
//! ## Módulos
//!
//! - `analytics`: Analytics e detecção de IA
//! - `camera`: Tipos relacionados a câmeras
//! - `config`: Configurações do sistema
//! - `error`: Tipos de erro
//! - `event`: Sistema de eventos e ações
//! - `layout`: Layouts e mosaicos
//! - `lpr`: Reconhecimento de placas
//! - `map`: Mapas sinópticos e operacionais
//! - `media_profile`: Perfis de mídia multi-streaming
//! - `playback`: Timeline e reprodução de vídeo
//! - `ptz`: Controle PTZ avançado
//! - `schedule`: Agendamento de gravação
//! - `stream`: Tipos de streaming
//! - `types`: Tipos básicos compartilhados
//! - `user`: Usuários e permissões

pub mod analytics;
pub mod camera;
pub mod config;
pub mod error;
pub mod event;
pub mod layout;
pub mod lpr;
pub mod map;
pub mod media_profile;
pub mod playback;
pub mod ptz;
pub mod schedule;
pub mod stream;
pub mod types;
pub mod user;

// Re-exports principais
pub use error::{Error, Result};
pub use types::{CameraId, StreamId, Resolution, FrameRate, Timestamp};
pub use camera::{CameraConfig, CameraStatus, CameraInfo};
pub use media_profile::{MediaProfile, MediaProfileId, VideoCodec, AudioCodec};
pub use schedule::{RecordingSchedule, RecordingMode, ScheduleId};
pub use event::{Event, EventId, EventTrigger, EventCategory, EventSeverity};
pub use playback::{PlaybackSession, PlaybackMode, PlaybackSpeed, Bookmark, ExportConfig};
pub use ptz::{PTZState, PTZCommand, PTZPreset, PTZTour, PTZCapabilities};
pub use layout::{Layout, LayoutTemplate, Mosaic, MosaicSequence, Workspace};
pub use user::{User, UserId, Role, Permission, Group, Session, AuditLog};
pub use analytics::{AnalyticsZone, AnalyticsRule, DetectedObject, ObjectClass, CountingLine};
pub use lpr::{PlateRead, PlateList, ParkingZone, CameraLPRConfig};
pub use map::{Map, MapObject, MapType, GeoCoordinates};
