//! Sistema de LPR (License Plate Recognition)
//!
//! Reconhecimento de placas, listas, zonas de estacionamento.

use crate::types::CameraId;
use crate::analytics::BoundingBox;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// ID de registro de placa
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlateReadId(pub Uuid);

impl PlateReadId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for PlateReadId {
    fn default() -> Self {
        Self::new()
    }
}

/// ID de lista de placas
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlateListId(pub Uuid);

impl PlateListId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for PlateListId {
    fn default() -> Self {
        Self::new()
    }
}

/// ID de zona de estacionamento
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ParkingZoneId(pub Uuid);

impl ParkingZoneId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ParkingZoneId {
    fn default() -> Self {
        Self::new()
    }
}

/// País/região para formatação de placa
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlateRegion {
    /// Brasil (padrão Mercosul)
    Brazil,
    /// Brasil (padrão antigo)
    BrazilOld,
    /// Argentina
    Argentina,
    /// EUA
    USA,
    /// Europa genérica
    Europe,
    /// Customizado
    Custom(String),
}

impl Default for PlateRegion {
    fn default() -> Self {
        Self::Brazil
    }
}

impl PlateRegion {
    /// Regex para validação de placa
    pub fn validation_regex(&self) -> &'static str {
        match self {
            Self::Brazil => r"^[A-Z]{3}[0-9][A-Z0-9][0-9]{2}$", // Mercosul: ABC1D23
            Self::BrazilOld => r"^[A-Z]{3}[0-9]{4}$", // Antigo: ABC1234
            Self::Argentina => r"^[A-Z]{2}[0-9]{3}[A-Z]{2}$", // AA123BB
            Self::USA => r"^[A-Z0-9]{5,8}$",
            Self::Europe => r"^[A-Z0-9]{4,10}$",
            Self::Custom(_) => r".*",
        }
    }
}

/// Tipo de veículo detectado
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VehicleType {
    Unknown,
    Car,
    Motorcycle,
    Truck,
    Bus,
    Van,
}

impl Default for VehicleType {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Cor do veículo
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VehicleColor {
    Unknown,
    White,
    Black,
    Silver,
    Gray,
    Red,
    Blue,
    Green,
    Yellow,
    Orange,
    Brown,
    Custom(String),
}

impl Default for VehicleColor {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Leitura de placa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlateRead {
    /// ID da leitura
    pub id: PlateReadId,

    /// Placa reconhecida
    pub plate: String,

    /// Placa normalizada (sem caracteres especiais)
    pub plate_normalized: String,

    /// Confiança do OCR (0-1)
    pub confidence: f32,

    /// Câmera que fez a leitura
    pub camera_id: CameraId,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Região/país detectado
    pub region: PlateRegion,

    /// Tipo de veículo
    pub vehicle_type: VehicleType,

    /// Cor do veículo
    pub vehicle_color: VehicleColor,

    /// Marca/modelo (se disponível)
    pub vehicle_make: Option<String>,

    /// Direção de movimento
    pub direction: Option<String>,

    /// Velocidade estimada (km/h)
    pub speed_kmh: Option<f32>,

    /// Bounding box da placa
    pub plate_bbox: BoundingBox,

    /// Bounding box do veículo
    pub vehicle_bbox: Option<BoundingBox>,

    /// Imagem da placa (path ou base64)
    pub plate_image: Option<String>,

    /// Imagem do veículo
    pub vehicle_image: Option<String>,

    /// Lista que deu match (se houver)
    pub matched_list_id: Option<PlateListId>,

    /// Nome da lista que deu match
    pub matched_list_name: Option<String>,

    /// Zona de estacionamento
    pub parking_zone_id: Option<ParkingZoneId>,

    /// Metadados extras
    pub metadata: HashMap<String, String>,
}

impl PlateRead {
    pub fn new(plate: &str, camera_id: CameraId, confidence: f32, bbox: BoundingBox) -> Self {
        let normalized = plate.chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_uppercase();

        Self {
            id: PlateReadId::new(),
            plate: plate.to_uppercase(),
            plate_normalized: normalized,
            confidence,
            camera_id,
            timestamp: Utc::now(),
            region: PlateRegion::Brazil,
            vehicle_type: VehicleType::Unknown,
            vehicle_color: VehicleColor::Unknown,
            vehicle_make: None,
            direction: None,
            speed_kmh: None,
            plate_bbox: bbox,
            vehicle_bbox: None,
            plate_image: None,
            vehicle_image: None,
            matched_list_id: None,
            matched_list_name: None,
            parking_zone_id: None,
            metadata: HashMap::new(),
        }
    }

    /// Verifica se a placa é válida para a região
    pub fn is_valid_for_region(&self) -> bool {
        use regex::Regex;
        if let Ok(re) = Regex::new(self.region.validation_regex()) {
            re.is_match(&self.plate_normalized)
        } else {
            true
        }
    }
}

/// Entrada em lista de placas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlateListEntry {
    /// Placa
    pub plate: String,

    /// Placa normalizada
    pub plate_normalized: String,

    /// Descrição/motivo
    pub description: Option<String>,

    /// Proprietário
    pub owner_name: Option<String>,

    /// Veículo
    pub vehicle_info: Option<String>,

    /// Data de expiração (None = nunca)
    pub expires_at: Option<DateTime<Utc>>,

    /// Adicionado em
    pub created_at: DateTime<Utc>,

    /// Adicionado por
    pub created_by: String,

    /// Tags
    pub tags: Vec<String>,

    /// Metadados
    pub metadata: HashMap<String, String>,
}

impl PlateListEntry {
    pub fn new(plate: &str, created_by: &str) -> Self {
        let normalized = plate.chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_uppercase();

        Self {
            plate: plate.to_uppercase(),
            plate_normalized: normalized,
            description: None,
            owner_name: None,
            vehicle_info: None,
            expires_at: None,
            created_at: Utc::now(),
            created_by: created_by.to_string(),
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at.map_or(false, |exp| Utc::now() > exp)
    }
}

/// Tipo de lista de placas
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlateListType {
    /// Permitido (whitelist)
    Allowed,
    /// Bloqueado (blacklist)
    Blocked,
    /// VIP
    VIP,
    /// Alerta
    Alert,
    /// Funcionários
    Employees,
    /// Visitantes
    Visitors,
    /// Customizado
    Custom,
}

impl Default for PlateListType {
    fn default() -> Self {
        Self::Allowed
    }
}

/// Lista de placas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlateList {
    /// ID da lista
    pub id: PlateListId,

    /// Nome
    pub name: String,

    /// Descrição
    pub description: Option<String>,

    /// Tipo de lista
    pub list_type: PlateListType,

    /// Entradas
    pub entries: Vec<PlateListEntry>,

    /// Máscaras (ex: ABC* para qualquer placa começando com ABC)
    pub masks: Vec<String>,

    /// Cor para destaque na UI
    pub color: String,

    /// Prioridade (maior = verificar primeiro)
    pub priority: u8,

    /// Ações a executar em match
    pub actions: Vec<Uuid>,

    /// Está ativa
    pub is_active: bool,

    /// Criada em
    pub created_at: DateTime<Utc>,
}

impl PlateList {
    pub fn new(name: &str, list_type: PlateListType) -> Self {
        Self {
            id: PlateListId::new(),
            name: name.to_string(),
            description: None,
            list_type,
            entries: Vec::new(),
            masks: Vec::new(),
            color: match list_type {
                PlateListType::Allowed => "#00FF00".to_string(),
                PlateListType::Blocked => "#FF0000".to_string(),
                PlateListType::VIP => "#FFD700".to_string(),
                PlateListType::Alert => "#FF6600".to_string(),
                _ => "#0088FF".to_string(),
            },
            priority: 5,
            actions: Vec::new(),
            is_active: true,
            created_at: Utc::now(),
        }
    }

    pub fn add_plate(&mut self, plate: &str, created_by: &str) {
        self.entries.push(PlateListEntry::new(plate, created_by));
    }

    pub fn add_mask(&mut self, mask: &str) {
        self.masks.push(mask.to_uppercase());
    }

    /// Verifica se uma placa está na lista
    pub fn contains(&self, plate: &str) -> bool {
        let normalized = plate.chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_uppercase();

        // Verificar entradas diretas
        if self.entries.iter()
            .filter(|e| !e.is_expired())
            .any(|e| e.plate_normalized == normalized)
        {
            return true;
        }

        // Verificar máscaras
        for mask in &self.masks {
            if self.matches_mask(&normalized, mask) {
                return true;
            }
        }

        false
    }

    fn matches_mask(&self, plate: &str, mask: &str) -> bool {
        let mask_normalized = mask.chars()
            .filter(|c| c.is_alphanumeric() || *c == '*' || *c == '?')
            .collect::<String>();

        // Implementação simples de wildcard
        if mask_normalized.contains('*') {
            let parts: Vec<&str> = mask_normalized.split('*').collect();
            if parts.len() == 2 {
                let starts_with = parts[0].is_empty() || plate.starts_with(parts[0]);
                let ends_with = parts[1].is_empty() || plate.ends_with(parts[1]);
                return starts_with && ends_with;
            }
        }

        plate == mask_normalized
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len() + self.masks.len()
    }
}

/// Configuração de LPR para câmera
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraLPRConfig {
    /// Câmera
    pub camera_id: CameraId,

    /// LPR habilitado
    pub enabled: bool,

    /// Engine de OCR
    pub ocr_engine: OCREngine,

    /// Região padrão
    pub default_region: PlateRegion,

    /// Confiança mínima
    pub min_confidence: f32,

    /// Área de detecção (ROI)
    pub detection_roi: Option<BoundingBox>,

    /// Listas para verificar
    pub check_lists: Vec<PlateListId>,

    /// Registrar todas as leituras (mesmo sem match)
    pub log_all_reads: bool,

    /// Deduplicate intervalo (não registrar mesma placa em X segundos)
    pub dedup_interval_seconds: u32,

    /// Salvar imagem da placa
    pub save_plate_image: bool,

    /// Salvar imagem do veículo
    pub save_vehicle_image: bool,

    /// Câmeras periféricas (overview)
    pub peripheral_cameras: Vec<CameraId>,

    /// Zona de estacionamento associada
    pub parking_zone_id: Option<ParkingZoneId>,

    /// Está ativo
    pub is_active: bool,
}

impl CameraLPRConfig {
    pub fn new(camera_id: CameraId) -> Self {
        Self {
            camera_id,
            enabled: false,
            ocr_engine: OCREngine::default(),
            default_region: PlateRegion::Brazil,
            min_confidence: 0.7,
            detection_roi: None,
            check_lists: Vec::new(),
            log_all_reads: true,
            dedup_interval_seconds: 60,
            save_plate_image: true,
            save_vehicle_image: true,
            peripheral_cameras: Vec::new(),
            parking_zone_id: None,
            is_active: true,
        }
    }
}

/// Engine de OCR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OCREngine {
    /// PaddleOCR (Apache 2.0)
    PaddleOCR,
    /// EasyOCR
    EasyOCR,
    /// Tesseract
    Tesseract,
    /// Engine comercial
    Commercial { name: String, api_key: Option<String> },
    /// Modelo ONNX customizado
    CustomONNX { path: String },
}

impl Default for OCREngine {
    fn default() -> Self {
        Self::PaddleOCR
    }
}

/// Zona de estacionamento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParkingZone {
    /// ID da zona
    pub id: ParkingZoneId,

    /// Nome
    pub name: String,

    /// Descrição
    pub description: Option<String>,

    /// Capacidade total
    pub total_capacity: u32,

    /// Ocupação atual
    pub current_occupancy: u32,

    /// Câmeras de entrada
    pub entry_cameras: Vec<CameraId>,

    /// Câmeras de saída
    pub exit_cameras: Vec<CameraId>,

    /// Tempo máximo de permanência (minutos, 0 = ilimitado)
    pub max_stay_minutes: u32,

    /// Alertar quando ocupação > X%
    pub alert_threshold_percent: u8,

    /// Lista de placas autorizadas
    pub authorized_list_id: Option<PlateListId>,

    /// Histórico de entradas hoje
    pub entries_today: u32,

    /// Histórico de saídas hoje
    pub exits_today: u32,

    /// Última atualização
    pub last_update: DateTime<Utc>,
}

impl ParkingZone {
    pub fn new(name: &str, capacity: u32) -> Self {
        Self {
            id: ParkingZoneId::new(),
            name: name.to_string(),
            description: None,
            total_capacity: capacity,
            current_occupancy: 0,
            entry_cameras: Vec::new(),
            exit_cameras: Vec::new(),
            max_stay_minutes: 0,
            alert_threshold_percent: 90,
            authorized_list_id: None,
            entries_today: 0,
            exits_today: 0,
            last_update: Utc::now(),
        }
    }

    pub fn occupancy_percent(&self) -> f32 {
        if self.total_capacity == 0 {
            return 0.0;
        }
        (self.current_occupancy as f32 / self.total_capacity as f32) * 100.0
    }

    pub fn available_spots(&self) -> u32 {
        self.total_capacity.saturating_sub(self.current_occupancy)
    }

    pub fn is_full(&self) -> bool {
        self.current_occupancy >= self.total_capacity
    }

    pub fn is_above_threshold(&self) -> bool {
        self.occupancy_percent() > self.alert_threshold_percent as f32
    }

    pub fn record_entry(&mut self) {
        self.current_occupancy = self.current_occupancy.saturating_add(1);
        self.entries_today += 1;
        self.last_update = Utc::now();
    }

    pub fn record_exit(&mut self) {
        self.current_occupancy = self.current_occupancy.saturating_sub(1);
        self.exits_today += 1;
        self.last_update = Utc::now();
    }
}

/// Registro de permanência no estacionamento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParkingSession {
    /// ID
    pub id: Uuid,

    /// Zona
    pub zone_id: ParkingZoneId,

    /// Placa
    pub plate: String,

    /// Entrada
    pub entry_time: DateTime<Utc>,

    /// Câmera de entrada
    pub entry_camera_id: CameraId,

    /// Saída
    pub exit_time: Option<DateTime<Utc>>,

    /// Câmera de saída
    pub exit_camera_id: Option<CameraId>,

    /// Duração (se saiu)
    pub duration_minutes: Option<u32>,

    /// Excedeu tempo máximo
    pub exceeded_max_stay: bool,
}

impl ParkingSession {
    pub fn new(zone_id: ParkingZoneId, plate: &str, entry_camera_id: CameraId) -> Self {
        Self {
            id: Uuid::new_v4(),
            zone_id,
            plate: plate.to_uppercase(),
            entry_time: Utc::now(),
            entry_camera_id,
            exit_time: None,
            exit_camera_id: None,
            duration_minutes: None,
            exceeded_max_stay: false,
        }
    }

    pub fn record_exit(&mut self, exit_camera_id: CameraId, max_stay_minutes: u32) {
        self.exit_time = Some(Utc::now());
        self.exit_camera_id = Some(exit_camera_id);

        if let Some(exit) = self.exit_time {
            let duration = exit - self.entry_time;
            self.duration_minutes = Some(duration.num_minutes() as u32);

            if max_stay_minutes > 0 && self.duration_minutes.unwrap_or(0) > max_stay_minutes {
                self.exceeded_max_stay = true;
            }
        }
    }

    pub fn is_active(&self) -> bool {
        self.exit_time.is_none()
    }

    pub fn current_duration_minutes(&self) -> u32 {
        if let Some(exit) = self.exit_time {
            (exit - self.entry_time).num_minutes() as u32
        } else {
            (Utc::now() - self.entry_time).num_minutes() as u32
        }
    }
}

/// Estatísticas de LPR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LPRStats {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub camera_id: Option<CameraId>,

    /// Total de leituras
    pub total_reads: u64,

    /// Leituras com match em lista
    pub matched_reads: u64,

    /// Placas únicas
    pub unique_plates: u64,

    /// Por tipo de lista
    pub by_list_type: HashMap<String, u64>,

    /// Por tipo de veículo
    pub by_vehicle_type: HashMap<String, u64>,

    /// Confiança média
    pub avg_confidence: f32,

    /// Horário de pico
    pub peak_hour: Option<u8>,

    /// Leituras no pico
    pub peak_hour_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plate_read() {
        let camera_id = CameraId::new();
        let read = PlateRead::new(
            "ABC1D23",
            camera_id,
            0.95,
            BoundingBox::new(0.1, 0.1, 0.2, 0.1),
        );

        assert_eq!(read.plate, "ABC1D23");
        assert_eq!(read.plate_normalized, "ABC1D23");
    }

    #[test]
    fn test_plate_list() {
        let mut list = PlateList::new("Allowed Vehicles", PlateListType::Allowed);
        list.add_plate("ABC1234", "admin");
        list.add_mask("DEF*");

        assert!(list.contains("ABC1234"));
        assert!(list.contains("ABC-1234")); // Normalização
        assert!(list.contains("DEF5678")); // Máscara
        assert!(!list.contains("XYZ9999"));
    }

    #[test]
    fn test_parking_zone() {
        let mut zone = ParkingZone::new("Parking A", 100);

        assert_eq!(zone.available_spots(), 100);
        assert!(!zone.is_full());

        for _ in 0..100 {
            zone.record_entry();
        }

        assert!(zone.is_full());
        assert_eq!(zone.available_spots(), 0);

        zone.record_exit();
        assert!(!zone.is_full());
        assert_eq!(zone.available_spots(), 1);
    }

    #[test]
    fn test_parking_session() {
        let zone_id = ParkingZoneId::new();
        let camera_id = CameraId::new();

        let mut session = ParkingSession::new(zone_id, "ABC1234", camera_id);
        assert!(session.is_active());

        session.record_exit(camera_id, 60);
        assert!(!session.is_active());
    }
}
