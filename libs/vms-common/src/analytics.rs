//! Sistema de Analytics e IA
//!
//! Detecção de objetos, contagem, zonas, reconhecimento facial, LPR.

use crate::types::CameraId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// ID de regra de analytics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AnalyticsRuleId(pub Uuid);

impl AnalyticsRuleId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for AnalyticsRuleId {
    fn default() -> Self {
        Self::new()
    }
}

/// ID de zona de analytics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ZoneId(pub Uuid);

impl ZoneId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ZoneId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ZoneId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// ID de linha de contagem
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LineId(pub Uuid);

impl LineId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for LineId {
    fn default() -> Self {
        Self::new()
    }
}

/// Ponto 2D (normalizado 0-1)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: x.clamp(0.0, 1.0),
            y: y.clamp(0.0, 1.0),
        }
    }
}

/// Retângulo (bounding box)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoundingBox {
    /// Canto superior esquerdo
    pub x: f32,
    pub y: f32,
    /// Largura
    pub width: f32,
    /// Altura
    pub height: f32,
}

impl BoundingBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn center(&self) -> Point2D {
        Point2D::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    pub fn contains(&self, point: Point2D) -> bool {
        point.x >= self.x && point.x <= self.x + self.width &&
        point.y >= self.y && point.y <= self.y + self.height
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }

    pub fn iou(&self, other: &BoundingBox) -> f32 {
        let x1 = self.x.max(other.x);
        let y1 = self.y.max(other.y);
        let x2 = (self.x + self.width).min(other.x + other.width);
        let y2 = (self.y + self.height).min(other.y + other.height);

        if x2 <= x1 || y2 <= y1 {
            return 0.0;
        }

        let intersection = (x2 - x1) * (y2 - y1);
        let union = self.area() + other.area() - intersection;

        intersection / union
    }
}

/// Tipo de objeto detectado
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectClass {
    Person,
    Vehicle,
    Car,
    Truck,
    Bus,
    Motorcycle,
    Bicycle,
    Animal,
    Dog,
    Cat,
    Face,
    LicensePlate,
    Bag,
    Backpack,
    Suitcase,
    Custom(String),
}

impl Default for ObjectClass {
    fn default() -> Self {
        Self::Person
    }
}

impl ObjectClass {
    pub fn is_vehicle(&self) -> bool {
        matches!(self, Self::Vehicle | Self::Car | Self::Truck | Self::Bus | Self::Motorcycle | Self::Bicycle)
    }

    pub fn is_person(&self) -> bool {
        matches!(self, Self::Person | Self::Face)
    }
}

/// Objeto detectado em um frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedObject {
    /// ID de tracking (se tracking ativo)
    pub track_id: Option<u64>,

    /// Classe do objeto
    pub class: ObjectClass,

    /// Confiança (0-1)
    pub confidence: f32,

    /// Bounding box
    pub bbox: BoundingBox,

    /// Atributos extras
    pub attributes: HashMap<String, String>,

    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl DetectedObject {
    pub fn new(class: ObjectClass, confidence: f32, bbox: BoundingBox) -> Self {
        Self {
            track_id: None,
            class,
            confidence,
            bbox,
            attributes: HashMap::new(),
            timestamp: Utc::now(),
        }
    }

    pub fn with_track_id(mut self, id: u64) -> Self {
        self.track_id = Some(id);
        self
    }

    pub fn with_attribute(mut self, key: &str, value: &str) -> Self {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }
}

/// Polígono (zona)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Polygon {
    pub points: Vec<Point2D>,
}

impl Polygon {
    pub fn new(points: Vec<Point2D>) -> Self {
        Self { points }
    }

    pub fn rectangle(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            points: vec![
                Point2D::new(x, y),
                Point2D::new(x + width, y),
                Point2D::new(x + width, y + height),
                Point2D::new(x, y + height),
            ],
        }
    }

    /// Verifica se ponto está dentro do polígono (ray casting)
    pub fn contains(&self, point: Point2D) -> bool {
        let n = self.points.len();
        if n < 3 {
            return false;
        }

        let mut inside = false;
        let mut j = n - 1;

        for i in 0..n {
            let pi = &self.points[i];
            let pj = &self.points[j];

            if ((pi.y > point.y) != (pj.y > point.y)) &&
                (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
            {
                inside = !inside;
            }
            j = i;
        }

        inside
    }
}

/// Zona de analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsZone {
    /// ID da zona
    pub id: ZoneId,

    /// Câmera
    pub camera_id: CameraId,

    /// Nome
    pub name: String,

    /// Polígono da zona
    pub polygon: Polygon,

    /// Cor para visualização (hex)
    pub color: String,

    /// Tipo de zona
    pub zone_type: ZoneType,

    /// Classes de objetos a detectar
    pub detect_classes: Vec<ObjectClass>,

    /// Confiança mínima
    pub min_confidence: f32,

    /// Está ativa
    pub is_active: bool,
}

impl AnalyticsZone {
    pub fn new(camera_id: CameraId, name: &str, polygon: Polygon) -> Self {
        Self {
            id: ZoneId::new(),
            camera_id,
            name: name.to_string(),
            polygon,
            color: "#FF0000".to_string(),
            zone_type: ZoneType::Detection,
            detect_classes: vec![ObjectClass::Person, ObjectClass::Vehicle],
            min_confidence: 0.5,
            is_active: true,
        }
    }
}

/// Tipo de zona
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZoneType {
    /// Zona de detecção
    Detection,
    /// Zona de interesse (ROI)
    ROI,
    /// Zona de exclusão (ignorar)
    Exclusion,
    /// Zona de contagem
    Counting,
    /// Zona de presença
    Presence,
    /// Zona de intrusão
    Intrusion,
}

impl Default for ZoneType {
    fn default() -> Self {
        Self::Detection
    }
}

/// Linha de contagem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountingLine {
    /// ID da linha
    pub id: LineId,

    /// Câmera
    pub camera_id: CameraId,

    /// Nome
    pub name: String,

    /// Ponto inicial
    pub start: Point2D,

    /// Ponto final
    pub end: Point2D,

    /// Cor
    pub color: String,

    /// Direção positiva (entrada)
    pub positive_direction: Direction,

    /// Classes a contar
    pub count_classes: Vec<ObjectClass>,

    /// Contadores
    pub count_in: u64,
    pub count_out: u64,

    /// Está ativa
    pub is_active: bool,

    /// Último reset
    pub last_reset: DateTime<Utc>,
}

impl CountingLine {
    pub fn new(camera_id: CameraId, name: &str, start: Point2D, end: Point2D) -> Self {
        Self {
            id: LineId::new(),
            camera_id,
            name: name.to_string(),
            start,
            end,
            color: "#00FF00".to_string(),
            positive_direction: Direction::Up,
            count_classes: vec![ObjectClass::Person],
            count_in: 0,
            count_out: 0,
            is_active: true,
            last_reset: Utc::now(),
        }
    }

    pub fn reset(&mut self) {
        self.count_in = 0;
        self.count_out = 0;
        self.last_reset = Utc::now();
    }

    pub fn total(&self) -> i64 {
        self.count_in as i64 - self.count_out as i64
    }

    pub fn increment_in(&mut self) {
        self.count_in += 1;
    }

    pub fn increment_out(&mut self) {
        self.count_out += 1;
    }
}

/// Direção
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Tipo de regra de analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalyticsRuleType {
    /// Objeto deixado
    ObjectLeft {
        min_duration_seconds: u32,
        zone_id: Option<ZoneId>,
    },

    /// Objeto retirado
    ObjectRemoved {
        min_duration_seconds: u32,
        zone_id: Option<ZoneId>,
    },

    /// Presença em zona
    Presence {
        zone_id: ZoneId,
        min_objects: u32,
        max_objects: Option<u32>,
    },

    /// Entrada em zona
    ZoneEnter {
        zone_id: ZoneId,
    },

    /// Saída de zona
    ZoneExit {
        zone_id: ZoneId,
    },

    /// Cruzamento de linha
    LineCrossing {
        line_id: LineId,
        direction: Option<Direction>,
    },

    /// Loitering (vadiagem)
    Loitering {
        zone_id: ZoneId,
        min_duration_seconds: u32,
    },

    /// Objeto parado
    Stopped {
        zone_id: Option<ZoneId>,
        min_duration_seconds: u32,
    },

    /// Tailgating (duas pessoas passando juntas)
    Tailgating {
        line_id: LineId,
        max_interval_seconds: f32,
    },

    /// Aglomeração
    Crowding {
        zone_id: ZoneId,
        min_density: f32,
    },

    /// Obstrução de câmera
    CameraObstruction {
        min_coverage_percent: f32,
        min_duration_seconds: u32,
    },

    /// Queda de pessoa
    PersonFall {
        zone_id: Option<ZoneId>,
    },

    /// Briga/agressão
    Fighting {
        zone_id: Option<ZoneId>,
    },

    /// Direção de movimento
    WrongDirection {
        zone_id: ZoneId,
        allowed_direction: Direction,
    },

    /// Velocidade
    Speed {
        zone_id: ZoneId,
        min_speed: Option<f32>,
        max_speed: Option<f32>,
    },

    /// Customizado
    Custom {
        rule_type: String,
        parameters: HashMap<String, String>,
    },
}

/// Regra de analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsRule {
    /// ID da regra
    pub id: AnalyticsRuleId,

    /// Câmera
    pub camera_id: CameraId,

    /// Nome
    pub name: String,

    /// Descrição
    pub description: Option<String>,

    /// Tipo de regra
    pub rule_type: AnalyticsRuleType,

    /// Classes de objetos
    pub object_classes: Vec<ObjectClass>,

    /// Confiança mínima
    pub min_confidence: f32,

    /// Agendamento (None = sempre ativo)
    pub schedule_id: Option<Uuid>,

    /// Ações a executar quando disparar
    pub actions: Vec<Uuid>,

    /// Cooldown em segundos
    pub cooldown_seconds: u32,

    /// Está ativa
    pub is_active: bool,

    /// Último disparo
    pub last_triggered: Option<DateTime<Utc>>,

    /// Contagem de disparos
    pub trigger_count: u64,
}

impl AnalyticsRule {
    pub fn new(camera_id: CameraId, name: &str, rule_type: AnalyticsRuleType) -> Self {
        Self {
            id: AnalyticsRuleId::new(),
            camera_id,
            name: name.to_string(),
            description: None,
            rule_type,
            object_classes: vec![ObjectClass::Person],
            min_confidence: 0.5,
            schedule_id: None,
            actions: Vec::new(),
            cooldown_seconds: 30,
            is_active: true,
            last_triggered: None,
            trigger_count: 0,
        }
    }

    pub fn can_trigger(&self) -> bool {
        if !self.is_active {
            return false;
        }

        if let Some(last) = self.last_triggered {
            let elapsed = Utc::now() - last;
            if elapsed.num_seconds() < self.cooldown_seconds as i64 {
                return false;
            }
        }

        true
    }

    pub fn trigger(&mut self) {
        self.last_triggered = Some(Utc::now());
        self.trigger_count += 1;
    }
}

/// Configuração de analytics para câmera
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraAnalyticsConfig {
    /// Câmera
    pub camera_id: CameraId,

    /// Analytics habilitado
    pub enabled: bool,

    /// Usar GPU
    pub use_gpu: bool,

    /// Modelo de detecção
    pub detection_model: DetectionModel,

    /// FPS para analytics (menor = menos CPU)
    pub analytics_fps: f32,

    /// Confiança mínima global
    pub min_confidence: f32,

    /// Habilitar tracking
    pub tracking_enabled: bool,

    /// Zonas
    pub zones: Vec<AnalyticsZone>,

    /// Linhas de contagem
    pub counting_lines: Vec<CountingLine>,

    /// Regras
    pub rules: Vec<AnalyticsRule>,

    /// Calibração de cena
    pub scene_calibration: Option<SceneCalibration>,
}

impl CameraAnalyticsConfig {
    pub fn new(camera_id: CameraId) -> Self {
        Self {
            camera_id,
            enabled: false,
            use_gpu: true,
            detection_model: DetectionModel::default(),
            analytics_fps: 5.0,
            min_confidence: 0.5,
            tracking_enabled: true,
            zones: Vec::new(),
            counting_lines: Vec::new(),
            rules: Vec::new(),
            scene_calibration: None,
        }
    }
}

/// Modelo de detecção
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DetectionModel {
    /// RT-DETR (Apache 2.0)
    RTDETR { size: ModelSize },
    /// YOLOv8 (GPL)
    YOLOv8 { size: ModelSize },
    /// YOLO-World (Open vocab)
    YOLOWorld { size: ModelSize },
    /// Modelo ONNX customizado
    CustomONNX { path: String },
}

impl Default for DetectionModel {
    fn default() -> Self {
        Self::RTDETR { size: ModelSize::Medium }
    }
}

/// Tamanho do modelo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelSize {
    Nano,
    Small,
    Medium,
    Large,
    XLarge,
}

impl Default for ModelSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Calibração de cena (para medições reais)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneCalibration {
    /// Altura média de pessoa em metros
    pub average_person_height_m: f32,

    /// Pontos de referência com distância conhecida
    pub reference_points: Vec<CalibrationPoint>,

    /// Matriz de homografia
    pub homography_matrix: Option<[[f32; 3]; 3]>,
}

impl Default for SceneCalibration {
    fn default() -> Self {
        Self {
            average_person_height_m: 1.7,
            reference_points: Vec::new(),
            homography_matrix: None,
        }
    }
}

/// Ponto de calibração
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationPoint {
    /// Posição na imagem (0-1)
    pub image_point: Point2D,
    /// Distância real da câmera (metros)
    pub real_distance_m: f32,
}

/// Resultado de analytics para um frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsFrame {
    /// Câmera
    pub camera_id: CameraId,

    /// Timestamp do frame
    pub timestamp: DateTime<Utc>,

    /// Objetos detectados
    pub objects: Vec<DetectedObject>,

    /// Contagens atuais
    pub counts: HashMap<LineId, (u64, u64)>,

    /// Regras disparadas
    pub triggered_rules: Vec<AnalyticsRuleId>,

    /// Tempo de processamento (ms)
    pub processing_time_ms: f32,

    /// Modelo usado
    pub model: String,
}

impl AnalyticsFrame {
    pub fn new(camera_id: CameraId) -> Self {
        Self {
            camera_id,
            timestamp: Utc::now(),
            objects: Vec::new(),
            counts: HashMap::new(),
            triggered_rules: Vec::new(),
            processing_time_ms: 0.0,
            model: String::new(),
        }
    }

    pub fn person_count(&self) -> usize {
        self.objects.iter().filter(|o| o.class.is_person()).count()
    }

    pub fn vehicle_count(&self) -> usize {
        self.objects.iter().filter(|o| o.class.is_vehicle()).count()
    }
}

/// Estatísticas de analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsStats {
    /// Período
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,

    /// Câmera
    pub camera_id: CameraId,

    /// Total de pessoas detectadas
    pub total_persons: u64,

    /// Total de veículos detectados
    pub total_vehicles: u64,

    /// Contagens por linha
    pub line_counts: HashMap<LineId, (u64, u64)>,

    /// Ocupação média por zona
    pub zone_occupancy: HashMap<ZoneId, f32>,

    /// Regras disparadas
    pub rule_triggers: HashMap<AnalyticsRuleId, u64>,

    /// Tempo médio de processamento
    pub avg_processing_time_ms: f32,

    /// Frames processados
    pub frames_processed: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box() {
        let bbox1 = BoundingBox::new(0.0, 0.0, 0.5, 0.5);
        let bbox2 = BoundingBox::new(0.25, 0.25, 0.5, 0.5);

        assert!(bbox1.intersects(&bbox2));
        assert!(bbox1.iou(&bbox2) > 0.0);

        let center = bbox1.center();
        assert_eq!(center.x, 0.25);
        assert_eq!(center.y, 0.25);
    }

    #[test]
    fn test_polygon_contains() {
        let poly = Polygon::rectangle(0.2, 0.2, 0.5, 0.5);

        assert!(poly.contains(Point2D::new(0.4, 0.4)));
        assert!(!poly.contains(Point2D::new(0.0, 0.0)));
        assert!(!poly.contains(Point2D::new(0.9, 0.9)));
    }

    #[test]
    fn test_counting_line() {
        let camera_id = CameraId::new();
        let mut line = CountingLine::new(
            camera_id,
            "Entrance",
            Point2D::new(0.0, 0.5),
            Point2D::new(1.0, 0.5),
        );

        line.increment_in();
        line.increment_in();
        line.increment_out();

        assert_eq!(line.count_in, 2);
        assert_eq!(line.count_out, 1);
        assert_eq!(line.total(), 1);
    }

    #[test]
    fn test_analytics_rule() {
        let camera_id = CameraId::new();
        let zone_id = ZoneId::new();

        let mut rule = AnalyticsRule::new(
            camera_id,
            "Loitering Detection",
            AnalyticsRuleType::Loitering {
                zone_id,
                min_duration_seconds: 30,
            },
        );

        assert!(rule.can_trigger());
        rule.trigger();
        assert!(!rule.can_trigger()); // Cooldown
    }

    #[test]
    fn test_detected_object() {
        let obj = DetectedObject::new(
            ObjectClass::Person,
            0.95,
            BoundingBox::new(0.1, 0.2, 0.1, 0.3),
        )
        .with_track_id(123)
        .with_attribute("color", "red");

        assert_eq!(obj.track_id, Some(123));
        assert_eq!(obj.attributes.get("color"), Some(&"red".to_string()));
    }
}
