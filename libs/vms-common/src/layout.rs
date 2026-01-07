//! Sistema de Layouts e Mosaicos
//!
//! Gerencia layouts de câmeras, mosaicos salvos, sequenciamento e multi-monitor.

use crate::types::CameraId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ID de layout
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LayoutId(pub Uuid);

impl LayoutId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for LayoutId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for LayoutId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// ID de mosaico
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MosaicId(pub Uuid);

impl MosaicId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for MosaicId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MosaicId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Template de layout predefinido
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutTemplate {
    /// 1 câmera (tela cheia)
    Single,
    /// 2 câmeras lado a lado
    Split2x1,
    /// 2 câmeras empilhadas
    Split1x2,
    /// 4 câmeras (2x2)
    Grid2x2,
    /// 6 câmeras (3x2)
    Grid3x2,
    /// 9 câmeras (3x3)
    Grid3x3,
    /// 16 câmeras (4x4)
    Grid4x4,
    /// 25 câmeras (5x5)
    Grid5x5,
    /// 36 câmeras (6x6)
    Grid6x6,
    /// 1 grande + 5 pequenas
    Split1Plus5,
    /// 1 grande + 7 pequenas
    Split1Plus7,
    /// 1 grande + 12 pequenas
    Split1Plus12,
    /// 2 grandes + 8 pequenas
    Split2Plus8,
    /// Layout personalizado
    Custom,
}

impl Default for LayoutTemplate {
    fn default() -> Self {
        Self::Grid2x2
    }
}

impl LayoutTemplate {
    /// Número de slots disponíveis no template
    pub fn slot_count(&self) -> usize {
        match self {
            Self::Single => 1,
            Self::Split2x1 | Self::Split1x2 => 2,
            Self::Grid2x2 => 4,
            Self::Grid3x2 | Self::Split1Plus5 => 6,
            Self::Split1Plus7 => 8,
            Self::Grid3x3 => 9,
            Self::Split2Plus8 => 10,
            Self::Split1Plus12 => 13,
            Self::Grid4x4 => 16,
            Self::Grid5x5 => 25,
            Self::Grid6x6 => 36,
            Self::Custom => 0, // Definido pelo usuário
        }
    }

    /// Descrição amigável
    pub fn description(&self) -> &'static str {
        match self {
            Self::Single => "1 Camera (Full Screen)",
            Self::Split2x1 => "2 Cameras (Side by Side)",
            Self::Split1x2 => "2 Cameras (Stacked)",
            Self::Grid2x2 => "4 Cameras (2x2 Grid)",
            Self::Grid3x2 => "6 Cameras (3x2 Grid)",
            Self::Grid3x3 => "9 Cameras (3x3 Grid)",
            Self::Grid4x4 => "16 Cameras (4x4 Grid)",
            Self::Grid5x5 => "25 Cameras (5x5 Grid)",
            Self::Grid6x6 => "36 Cameras (6x6 Grid)",
            Self::Split1Plus5 => "1 Main + 5 Small",
            Self::Split1Plus7 => "1 Main + 7 Small",
            Self::Split1Plus12 => "1 Main + 12 Small",
            Self::Split2Plus8 => "2 Main + 8 Small",
            Self::Custom => "Custom Layout",
        }
    }
}

/// Posição e tamanho de um slot no layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutSlot {
    /// Índice do slot (0-based)
    pub index: usize,

    /// Posição X (0-100%)
    pub x: f32,

    /// Posição Y (0-100%)
    pub y: f32,

    /// Largura (0-100%)
    pub width: f32,

    /// Altura (0-100%)
    pub height: f32,

    /// Câmera atribuída (None = vazio)
    pub camera_id: Option<CameraId>,

    /// Está selecionado
    pub is_selected: bool,

    /// É slot principal (destaque)
    pub is_main: bool,
}

impl LayoutSlot {
    pub fn new(index: usize, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            index,
            x,
            y,
            width,
            height,
            camera_id: None,
            is_selected: false,
            is_main: false,
        }
    }

    pub fn with_camera(mut self, camera_id: CameraId) -> Self {
        self.camera_id = Some(camera_id);
        self
    }

    pub fn as_main(mut self) -> Self {
        self.is_main = true;
        self
    }
}

/// Definição de layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    /// ID do layout
    pub id: LayoutId,

    /// Nome
    pub name: String,

    /// Template base
    pub template: LayoutTemplate,

    /// Slots do layout
    pub slots: Vec<LayoutSlot>,

    /// Linhas do grid (para Custom)
    pub rows: u8,

    /// Colunas do grid (para Custom)
    pub cols: u8,

    /// Espaçamento entre células (pixels)
    pub gap: u8,

    /// Cor de fundo
    pub background_color: String,

    /// Cor da borda
    pub border_color: String,

    /// Espessura da borda
    pub border_width: u8,
}

impl Layout {
    /// Cria layout a partir de template
    pub fn from_template(name: &str, template: LayoutTemplate) -> Self {
        let slots = Self::generate_slots_for_template(template);
        let (rows, cols) = Self::grid_size_for_template(template);

        Self {
            id: LayoutId::new(),
            name: name.to_string(),
            template,
            slots,
            rows,
            cols,
            gap: 2,
            background_color: "#1a1a2e".to_string(),
            border_color: "#16213e".to_string(),
            border_width: 1,
        }
    }

    /// Cria layout custom
    pub fn custom(name: &str, rows: u8, cols: u8) -> Self {
        let mut slots = Vec::new();
        let slot_width = 100.0 / cols as f32;
        let slot_height = 100.0 / rows as f32;

        for row in 0..rows {
            for col in 0..cols {
                let index = (row * cols + col) as usize;
                slots.push(LayoutSlot::new(
                    index,
                    col as f32 * slot_width,
                    row as f32 * slot_height,
                    slot_width,
                    slot_height,
                ));
            }
        }

        Self {
            id: LayoutId::new(),
            name: name.to_string(),
            template: LayoutTemplate::Custom,
            slots,
            rows,
            cols,
            gap: 2,
            background_color: "#1a1a2e".to_string(),
            border_color: "#16213e".to_string(),
            border_width: 1,
        }
    }

    fn grid_size_for_template(template: LayoutTemplate) -> (u8, u8) {
        match template {
            LayoutTemplate::Single => (1, 1),
            LayoutTemplate::Split2x1 => (1, 2),
            LayoutTemplate::Split1x2 => (2, 1),
            LayoutTemplate::Grid2x2 => (2, 2),
            LayoutTemplate::Grid3x2 => (2, 3),
            LayoutTemplate::Grid3x3 => (3, 3),
            LayoutTemplate::Grid4x4 => (4, 4),
            LayoutTemplate::Grid5x5 => (5, 5),
            LayoutTemplate::Grid6x6 => (6, 6),
            _ => (3, 3), // Default para layouts especiais
        }
    }

    fn generate_slots_for_template(template: LayoutTemplate) -> Vec<LayoutSlot> {
        match template {
            LayoutTemplate::Single => vec![
                LayoutSlot::new(0, 0.0, 0.0, 100.0, 100.0).as_main()
            ],
            LayoutTemplate::Split2x1 => vec![
                LayoutSlot::new(0, 0.0, 0.0, 50.0, 100.0),
                LayoutSlot::new(1, 50.0, 0.0, 50.0, 100.0),
            ],
            LayoutTemplate::Split1x2 => vec![
                LayoutSlot::new(0, 0.0, 0.0, 100.0, 50.0),
                LayoutSlot::new(1, 0.0, 50.0, 100.0, 50.0),
            ],
            LayoutTemplate::Grid2x2 => {
                vec![
                    LayoutSlot::new(0, 0.0, 0.0, 50.0, 50.0),
                    LayoutSlot::new(1, 50.0, 0.0, 50.0, 50.0),
                    LayoutSlot::new(2, 0.0, 50.0, 50.0, 50.0),
                    LayoutSlot::new(3, 50.0, 50.0, 50.0, 50.0),
                ]
            },
            LayoutTemplate::Grid3x3 => {
                let size = 100.0 / 3.0;
                (0..9).map(|i| {
                    LayoutSlot::new(
                        i,
                        (i % 3) as f32 * size,
                        (i / 3) as f32 * size,
                        size,
                        size,
                    )
                }).collect()
            },
            LayoutTemplate::Grid4x4 => {
                let size = 25.0;
                (0..16).map(|i| {
                    LayoutSlot::new(
                        i,
                        (i % 4) as f32 * size,
                        (i / 4) as f32 * size,
                        size,
                        size,
                    )
                }).collect()
            },
            LayoutTemplate::Split1Plus5 => {
                // Grande à esquerda, 5 pequenas à direita
                let mut slots = vec![
                    LayoutSlot::new(0, 0.0, 0.0, 66.67, 100.0).as_main(),
                ];
                let small_height = 100.0 / 5.0;
                for i in 0..5 {
                    slots.push(LayoutSlot::new(
                        i + 1,
                        66.67,
                        i as f32 * small_height,
                        33.33,
                        small_height,
                    ));
                }
                slots
            },
            LayoutTemplate::Split1Plus7 => {
                // Grande no centro-esquerda, 7 pequenas ao redor
                vec![
                    LayoutSlot::new(0, 0.0, 0.0, 75.0, 75.0).as_main(),
                    LayoutSlot::new(1, 75.0, 0.0, 25.0, 25.0),
                    LayoutSlot::new(2, 75.0, 25.0, 25.0, 25.0),
                    LayoutSlot::new(3, 75.0, 50.0, 25.0, 25.0),
                    LayoutSlot::new(4, 75.0, 75.0, 25.0, 25.0),
                    LayoutSlot::new(5, 0.0, 75.0, 25.0, 25.0),
                    LayoutSlot::new(6, 25.0, 75.0, 25.0, 25.0),
                    LayoutSlot::new(7, 50.0, 75.0, 25.0, 25.0),
                ]
            },
            _ => {
                // Fallback: grid simples
                let (rows, cols) = Self::grid_size_for_template(template);
                let slot_width = 100.0 / cols as f32;
                let slot_height = 100.0 / rows as f32;
                let count = rows as usize * cols as usize;

                (0..count).map(|i| {
                    LayoutSlot::new(
                        i,
                        (i % cols as usize) as f32 * slot_width,
                        (i / cols as usize) as f32 * slot_height,
                        slot_width,
                        slot_height,
                    )
                }).collect()
            }
        }
    }

    /// Atribui câmera a um slot
    pub fn assign_camera(&mut self, slot_index: usize, camera_id: CameraId) {
        if let Some(slot) = self.slots.get_mut(slot_index) {
            slot.camera_id = Some(camera_id);
        }
    }

    /// Remove câmera de um slot
    pub fn clear_slot(&mut self, slot_index: usize) {
        if let Some(slot) = self.slots.get_mut(slot_index) {
            slot.camera_id = None;
        }
    }

    /// Atribui câmeras em ordem
    pub fn assign_cameras(&mut self, camera_ids: &[CameraId]) {
        for (i, camera_id) in camera_ids.iter().enumerate() {
            if i < self.slots.len() {
                self.slots[i].camera_id = Some(*camera_id);
            }
        }
    }

    /// Obtém câmeras atribuídas
    pub fn get_cameras(&self) -> Vec<CameraId> {
        self.slots.iter().filter_map(|s| s.camera_id).collect()
    }
}

/// Mosaico salvo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mosaic {
    /// ID do mosaico
    pub id: MosaicId,

    /// Nome
    pub name: String,

    /// Descrição
    pub description: Option<String>,

    /// Layout do mosaico
    pub layout: Layout,

    /// É público (visível para todos)
    pub is_public: bool,

    /// Criado por
    pub created_by: String,

    /// Criado em
    pub created_at: DateTime<Utc>,

    /// Atualizado em
    pub updated_at: DateTime<Utc>,

    /// Tags para organização
    pub tags: Vec<String>,

    /// Ícone/thumbnail
    pub thumbnail: Option<String>,

    /// Ordem de exibição
    pub sort_order: i32,
}

impl Mosaic {
    pub fn new(name: &str, layout: Layout, created_by: &str) -> Self {
        Self {
            id: MosaicId::new(),
            name: name.to_string(),
            description: None,
            layout,
            is_public: true,
            created_by: created_by.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            tags: Vec::new(),
            thumbnail: None,
            sort_order: 0,
        }
    }
}

/// Item de sequência para timer de mosaicos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceItem {
    /// Mosaico a exibir
    pub mosaic_id: MosaicId,

    /// Tempo de exibição em segundos
    pub duration_seconds: u32,

    /// Transição
    pub transition: Transition,
}

/// Tipo de transição entre mosaicos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Transition {
    /// Troca instantânea
    None,
    /// Fade
    Fade,
    /// Slide horizontal
    SlideHorizontal,
    /// Slide vertical
    SlideVertical,
}

impl Default for Transition {
    fn default() -> Self {
        Self::Fade
    }
}

/// Sequência de mosaicos (timer)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosaicSequence {
    /// ID da sequência
    pub id: Uuid,

    /// Nome
    pub name: String,

    /// Itens da sequência
    pub items: Vec<SequenceItem>,

    /// Repetir indefinidamente
    pub loop_forever: bool,

    /// Está ativo
    pub is_active: bool,

    /// Pausado
    pub is_paused: bool,

    /// Índice atual
    pub current_index: usize,
}

impl MosaicSequence {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            items: Vec::new(),
            loop_forever: true,
            is_active: false,
            is_paused: false,
            current_index: 0,
        }
    }

    pub fn add(&mut self, mosaic_id: MosaicId, duration_seconds: u32) {
        self.items.push(SequenceItem {
            mosaic_id,
            duration_seconds,
            transition: Transition::Fade,
        });
    }

    pub fn total_duration_seconds(&self) -> u32 {
        self.items.iter().map(|i| i.duration_seconds).sum()
    }

    pub fn next(&mut self) -> Option<&SequenceItem> {
        if self.items.is_empty() {
            return None;
        }

        self.current_index += 1;
        if self.current_index >= self.items.len() {
            if self.loop_forever {
                self.current_index = 0;
            } else {
                self.is_active = false;
                return None;
            }
        }

        self.items.get(self.current_index)
    }

    pub fn current(&self) -> Option<&SequenceItem> {
        self.items.get(self.current_index)
    }
}

/// Configuração de monitor (multi-monitor)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfig {
    /// ID do monitor
    pub id: u8,

    /// Nome
    pub name: String,

    /// Posição X (pixels)
    pub x: i32,

    /// Posição Y (pixels)
    pub y: i32,

    /// Largura
    pub width: u32,

    /// Altura
    pub height: u32,

    /// É monitor primário
    pub is_primary: bool,

    /// Mosaico atribuído
    pub mosaic_id: Option<MosaicId>,

    /// Sequência ativa
    pub sequence_id: Option<Uuid>,

    /// Tela cheia
    pub fullscreen: bool,
}

impl MonitorConfig {
    pub fn new(id: u8, name: &str, width: u32, height: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            x: 0,
            y: 0,
            width,
            height,
            is_primary: id == 0,
            mosaic_id: None,
            sequence_id: None,
            fullscreen: false,
        }
    }
}

/// Configuração de workspace (múltiplos monitores)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    /// ID do workspace
    pub id: Uuid,

    /// Nome
    pub name: String,

    /// Monitores
    pub monitors: Vec<MonitorConfig>,

    /// Mosaicos disponíveis
    pub mosaics: Vec<Mosaic>,

    /// Sequências
    pub sequences: Vec<MosaicSequence>,
}

impl Workspace {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            monitors: Vec::new(),
            mosaics: Vec::new(),
            sequences: Vec::new(),
        }
    }

    /// Adiciona monitor
    pub fn add_monitor(&mut self, config: MonitorConfig) {
        self.monitors.push(config);
    }

    /// Adiciona mosaico
    pub fn add_mosaic(&mut self, mosaic: Mosaic) {
        self.mosaics.push(mosaic);
    }

    /// Atribui mosaico a monitor
    pub fn assign_mosaic_to_monitor(&mut self, monitor_id: u8, mosaic_id: MosaicId) {
        if let Some(monitor) = self.monitors.iter_mut().find(|m| m.id == monitor_id) {
            monitor.mosaic_id = Some(mosaic_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_templates() {
        assert_eq!(LayoutTemplate::Single.slot_count(), 1);
        assert_eq!(LayoutTemplate::Grid4x4.slot_count(), 16);
    }

    #[test]
    fn test_layout_from_template() {
        let layout = Layout::from_template("Test 2x2", LayoutTemplate::Grid2x2);
        assert_eq!(layout.slots.len(), 4);
    }

    #[test]
    fn test_layout_assign_cameras() {
        let mut layout = Layout::from_template("Test", LayoutTemplate::Grid2x2);
        let cameras = vec![CameraId::new(), CameraId::new(), CameraId::new()];

        layout.assign_cameras(&cameras);

        assert!(layout.slots[0].camera_id.is_some());
        assert!(layout.slots[1].camera_id.is_some());
        assert!(layout.slots[2].camera_id.is_some());
        assert!(layout.slots[3].camera_id.is_none());
    }

    #[test]
    fn test_mosaic_sequence() {
        let mosaic1 = Mosaic::new(
            "Mosaic 1",
            Layout::from_template("L1", LayoutTemplate::Grid2x2),
            "admin",
        );
        let mosaic2 = Mosaic::new(
            "Mosaic 2",
            Layout::from_template("L2", LayoutTemplate::Grid3x3),
            "admin",
        );

        let mut sequence = MosaicSequence::new("Rotation");
        sequence.add(mosaic1.id, 30);
        sequence.add(mosaic2.id, 30);

        assert_eq!(sequence.total_duration_seconds(), 60);
        assert_eq!(sequence.current_index, 0);

        sequence.next();
        assert_eq!(sequence.current_index, 1);
    }
}
