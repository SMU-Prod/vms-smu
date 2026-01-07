//! Sistema avançado de PTZ
//!
//! Controle completo de Pan-Tilt-Zoom, presets, patrulhas, bloqueio por usuário.

use crate::types::CameraId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ID de preset PTZ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PTZPresetId(pub Uuid);

impl PTZPresetId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for PTZPresetId {
    fn default() -> Self {
        Self::new()
    }
}

/// ID de tour/patrulha
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PTZTourId(pub Uuid);

impl PTZTourId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for PTZTourId {
    fn default() -> Self {
        Self::new()
    }
}

/// Capacidades PTZ de uma câmera
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PTZCapabilities {
    /// Suporta pan (horizontal)
    pub pan: bool,

    /// Suporta tilt (vertical)
    pub tilt: bool,

    /// Suporta zoom ótico
    pub zoom: bool,

    /// Suporta foco
    pub focus: bool,

    /// Suporta íris
    pub iris: bool,

    /// Presets disponíveis (máximo)
    pub max_presets: u32,

    /// Velocidade mínima (0-1)
    pub min_speed: f32,

    /// Velocidade máxima (0-1)
    pub max_speed: f32,

    /// Suporta movimento contínuo
    pub continuous_move: bool,

    /// Suporta movimento absoluto
    pub absolute_move: bool,

    /// Suporta movimento relativo
    pub relative_move: bool,

    /// Funções auxiliares disponíveis
    pub aux_functions: Vec<String>,
}

impl Default for PTZCapabilities {
    fn default() -> Self {
        Self {
            pan: true,
            tilt: true,
            zoom: true,
            focus: true,
            iris: false,
            max_presets: 255,
            min_speed: 0.0,
            max_speed: 1.0,
            continuous_move: true,
            absolute_move: true,
            relative_move: true,
            aux_functions: Vec::new(),
        }
    }
}

/// Posição PTZ
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PTZPosition {
    /// Pan (-1.0 a 1.0 onde -1 = esquerda, 1 = direita)
    pub pan: f32,

    /// Tilt (-1.0 a 1.0 onde -1 = baixo, 1 = cima)
    pub tilt: f32,

    /// Zoom (0 a 1.0 onde 0 = wide, 1 = telephoto)
    pub zoom: f32,
}

impl Default for PTZPosition {
    fn default() -> Self {
        Self {
            pan: 0.0,
            tilt: 0.0,
            zoom: 0.0,
        }
    }
}

impl PTZPosition {
    pub fn new(pan: f32, tilt: f32, zoom: f32) -> Self {
        Self {
            pan: pan.clamp(-1.0, 1.0),
            tilt: tilt.clamp(-1.0, 1.0),
            zoom: zoom.clamp(0.0, 1.0),
        }
    }
}

/// Velocidade de movimento PTZ
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PTZSpeed {
    /// Velocidade de pan
    pub pan: f32,

    /// Velocidade de tilt
    pub tilt: f32,

    /// Velocidade de zoom
    pub zoom: f32,
}

impl Default for PTZSpeed {
    fn default() -> Self {
        Self {
            pan: 0.5,
            tilt: 0.5,
            zoom: 0.5,
        }
    }
}

impl PTZSpeed {
    pub fn slow() -> Self {
        Self {
            pan: 0.2,
            tilt: 0.2,
            zoom: 0.2,
        }
    }

    pub fn fast() -> Self {
        Self {
            pan: 1.0,
            tilt: 1.0,
            zoom: 1.0,
        }
    }
}

/// Direção de movimento
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PTZDirection {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    ZoomIn,
    ZoomOut,
    Stop,
}

/// Preset PTZ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PTZPreset {
    /// ID do preset
    pub id: PTZPresetId,

    /// Câmera
    pub camera_id: CameraId,

    /// Índice do preset na câmera (1-255 tipicamente)
    pub preset_index: u8,

    /// Nome
    pub name: String,

    /// Descrição
    pub description: Option<String>,

    /// Posição salva
    pub position: PTZPosition,

    /// Ícone (nome ou URL)
    pub icon: Option<String>,

    /// É home position
    pub is_home: bool,

    /// Thumbnail do preset
    pub thumbnail: Option<String>,

    /// Criado em
    pub created_at: DateTime<Utc>,
}

impl PTZPreset {
    pub fn new(camera_id: CameraId, preset_index: u8, name: &str, position: PTZPosition) -> Self {
        Self {
            id: PTZPresetId::new(),
            camera_id,
            preset_index,
            name: name.to_string(),
            description: None,
            position,
            icon: None,
            is_home: false,
            thumbnail: None,
            created_at: Utc::now(),
        }
    }

    pub fn as_home(mut self) -> Self {
        self.is_home = true;
        self
    }
}

/// Ponto em uma patrulha
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TourPoint {
    /// Preset a ir
    pub preset_id: PTZPresetId,

    /// Tempo de permanência em segundos
    pub dwell_time_seconds: u32,

    /// Velocidade para chegar ao ponto
    pub speed: PTZSpeed,
}

/// Tour/patrulha de PTZ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PTZTour {
    /// ID do tour
    pub id: PTZTourId,

    /// Câmera
    pub camera_id: CameraId,

    /// Nome
    pub name: String,

    /// Descrição
    pub description: Option<String>,

    /// Pontos do tour
    pub points: Vec<TourPoint>,

    /// Repetir indefinidamente
    pub loop_forever: bool,

    /// Número de repetições (se loop_forever = false)
    pub repeat_count: u32,

    /// Está ativo
    pub is_active: bool,
}

impl PTZTour {
    pub fn new(camera_id: CameraId, name: &str) -> Self {
        Self {
            id: PTZTourId::new(),
            camera_id,
            name: name.to_string(),
            description: None,
            points: Vec::new(),
            loop_forever: true,
            repeat_count: 0,
            is_active: false,
        }
    }

    pub fn add_point(&mut self, preset_id: PTZPresetId, dwell_time_seconds: u32) {
        self.points.push(TourPoint {
            preset_id,
            dwell_time_seconds,
            speed: PTZSpeed::default(),
        });
    }

    pub fn total_duration_seconds(&self) -> u32 {
        self.points.iter().map(|p| p.dwell_time_seconds).sum()
    }
}

/// Prioridade de controle PTZ
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PTZPriority {
    /// Mais baixa (tour automático)
    Low = 1,
    /// Normal (operadores)
    Normal = 5,
    /// Alta (supervisores)
    High = 8,
    /// Máxima (administradores/eventos)
    Critical = 10,
}

impl Default for PTZPriority {
    fn default() -> Self {
        Self::Normal
    }
}

/// Bloqueio de controle PTZ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PTZLock {
    /// Câmera bloqueada
    pub camera_id: CameraId,

    /// Usuário que tem o bloqueio
    pub locked_by: String,

    /// Prioridade do bloqueio
    pub priority: PTZPriority,

    /// Início do bloqueio
    pub locked_at: DateTime<Utc>,

    /// Tempo máximo do bloqueio (segundos, None = indefinido)
    pub max_duration_seconds: Option<u32>,

    /// Motivo
    pub reason: Option<String>,
}

impl PTZLock {
    pub fn new(camera_id: CameraId, user: &str, priority: PTZPriority) -> Self {
        Self {
            camera_id,
            locked_by: user.to_string(),
            priority,
            locked_at: Utc::now(),
            max_duration_seconds: Some(300), // 5 minutos padrão
            reason: None,
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(max_seconds) = self.max_duration_seconds {
            let elapsed = Utc::now() - self.locked_at;
            elapsed.num_seconds() > max_seconds as i64
        } else {
            false
        }
    }
}

/// Estado PTZ atual de uma câmera
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PTZState {
    /// Câmera
    pub camera_id: CameraId,

    /// Capacidades
    pub capabilities: PTZCapabilities,

    /// Posição atual
    pub current_position: PTZPosition,

    /// Está em movimento
    pub is_moving: bool,

    /// Direção de movimento atual
    pub current_direction: Option<PTZDirection>,

    /// Presets salvos
    pub presets: Vec<PTZPreset>,

    /// Tours configurados
    pub tours: Vec<PTZTour>,

    /// Tour ativo
    pub active_tour_id: Option<PTZTourId>,

    /// Bloqueio atual
    pub lock: Option<PTZLock>,

    /// Foco automático ativo
    pub auto_focus: bool,

    /// Íris automática ativa
    pub auto_iris: bool,

    /// Última atualização
    pub last_update: DateTime<Utc>,
}

impl PTZState {
    pub fn new(camera_id: CameraId, capabilities: PTZCapabilities) -> Self {
        Self {
            camera_id,
            capabilities,
            current_position: PTZPosition::default(),
            is_moving: false,
            current_direction: None,
            presets: Vec::new(),
            tours: Vec::new(),
            active_tour_id: None,
            lock: None,
            auto_focus: true,
            auto_iris: true,
            last_update: Utc::now(),
        }
    }

    /// Verifica se um usuário pode controlar
    pub fn can_control(&self, user: &str, priority: PTZPriority) -> bool {
        match &self.lock {
            None => true,
            Some(lock) => {
                if lock.is_expired() {
                    return true;
                }
                lock.locked_by == user || priority > lock.priority
            }
        }
    }

    /// Obtém preset por índice
    pub fn get_preset_by_index(&self, index: u8) -> Option<&PTZPreset> {
        self.presets.iter().find(|p| p.preset_index == index)
    }

    /// Obtém home preset
    pub fn get_home_preset(&self) -> Option<&PTZPreset> {
        self.presets.iter().find(|p| p.is_home)
    }
}

/// Comando PTZ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PTZCommand {
    /// Movimento contínuo
    ContinuousMove {
        pan: f32,
        tilt: f32,
        zoom: f32,
    },

    /// Parar movimento
    Stop,

    /// Ir para posição absoluta
    AbsoluteMove {
        position: PTZPosition,
        speed: Option<PTZSpeed>,
    },

    /// Movimento relativo
    RelativeMove {
        delta: PTZPosition,
        speed: Option<PTZSpeed>,
    },

    /// Ir para preset
    GotoPreset {
        preset_index: u8,
        speed: Option<PTZSpeed>,
    },

    /// Ir para home
    GotoHome,

    /// Salvar preset
    SetPreset {
        preset_index: u8,
        name: String,
    },

    /// Apagar preset
    RemovePreset {
        preset_index: u8,
    },

    /// Iniciar tour
    StartTour {
        tour_id: PTZTourId,
    },

    /// Parar tour
    StopTour,

    /// Foco automático
    AutoFocus,

    /// Foco manual
    ManualFocus {
        direction: i8, // -1 = near, 1 = far
    },

    /// Íris automática
    AutoIris,

    /// Íris manual
    ManualIris {
        direction: i8, // -1 = close, 1 = open
    },

    /// Função auxiliar
    AuxFunction {
        function: String,
        enabled: bool,
    },
}

/// Configuração de joystick virtual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualJoystickConfig {
    /// Sensibilidade do joystick (0.1 a 3.0)
    pub sensitivity: f32,

    /// Zona morta (centro onde não há movimento)
    pub deadzone: f32,

    /// Inverter pan
    pub invert_pan: bool,

    /// Inverter tilt
    pub invert_tilt: bool,

    /// Modo de zoom (scroll ou slider)
    pub zoom_mode: ZoomMode,
}

impl Default for VirtualJoystickConfig {
    fn default() -> Self {
        Self {
            sensitivity: 1.0,
            deadzone: 0.1,
            invert_pan: false,
            invert_tilt: false,
            zoom_mode: ZoomMode::Scroll,
        }
    }
}

/// Modo de controle de zoom
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZoomMode {
    /// Zoom com scroll do mouse
    Scroll,
    /// Zoom com slider
    Slider,
    /// Zoom com botões +/-
    Buttons,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ptz_position() {
        let pos = PTZPosition::new(0.5, -0.5, 0.25);
        assert_eq!(pos.pan, 0.5);
        assert_eq!(pos.tilt, -0.5);
        assert_eq!(pos.zoom, 0.25);
    }

    #[test]
    fn test_ptz_preset() {
        let camera_id = CameraId::new();
        let preset = PTZPreset::new(
            camera_id,
            1,
            "Entrance",
            PTZPosition::new(0.0, 0.0, 0.5),
        ).as_home();

        assert!(preset.is_home);
        assert_eq!(preset.preset_index, 1);
    }

    #[test]
    fn test_ptz_lock() {
        let camera_id = CameraId::new();
        let lock = PTZLock::new(camera_id, "admin", PTZPriority::High);

        assert!(!lock.is_expired());
        assert_eq!(lock.priority, PTZPriority::High);
    }

    #[test]
    fn test_ptz_state_control() {
        let camera_id = CameraId::new();
        let mut state = PTZState::new(camera_id, PTZCapabilities::default());

        // Sem bloqueio, qualquer um pode controlar
        assert!(state.can_control("user1", PTZPriority::Low));

        // Com bloqueio normal
        state.lock = Some(PTZLock::new(camera_id, "user1", PTZPriority::Normal));

        // Mesmo usuário pode controlar
        assert!(state.can_control("user1", PTZPriority::Low));

        // Outro usuário com mesma ou menor prioridade não pode
        assert!(!state.can_control("user2", PTZPriority::Normal));

        // Outro usuário com maior prioridade pode
        assert!(state.can_control("user2", PTZPriority::High));
    }
}
