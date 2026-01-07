//! Sistema de Mapas
//!
//! Mapas sinópticos (imagem) e operacionais (geográficos).

use crate::types::CameraId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ID de mapa
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MapId(pub Uuid);

impl MapId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for MapId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MapId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// ID de objeto no mapa
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MapObjectId(pub Uuid);

impl MapObjectId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for MapObjectId {
    fn default() -> Self {
        Self::new()
    }
}

/// Tipo de mapa
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MapType {
    /// Mapa sinóptico (imagem de planta baixa)
    Synoptic,
    /// Mapa operacional (geográfico real)
    Operational,
}

impl Default for MapType {
    fn default() -> Self {
        Self::Synoptic
    }
}

/// Provedor de mapas operacionais
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapProvider {
    /// OpenStreetMap (gratuito)
    OpenStreetMap,
    /// Google Maps
    GoogleMaps { api_key: String },
    /// Mapbox
    Mapbox { access_token: String },
    /// Mapas offline (TileServer)
    OfflineTiles { url: String },
}

impl Default for MapProvider {
    fn default() -> Self {
        Self::OpenStreetMap
    }
}

/// Coordenadas geográficas
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GeoCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}

impl GeoCoordinates {
    pub fn new(lat: f64, lon: f64) -> Self {
        Self {
            latitude: lat,
            longitude: lon,
        }
    }

    /// Calcula distância em metros até outro ponto (Haversine)
    pub fn distance_to(&self, other: &GeoCoordinates) -> f64 {
        const R: f64 = 6371000.0; // Raio da Terra em metros

        let lat1 = self.latitude.to_radians();
        let lat2 = other.latitude.to_radians();
        let dlat = (other.latitude - self.latitude).to_radians();
        let dlon = (other.longitude - self.longitude).to_radians();

        let a = (dlat / 2.0).sin().powi(2) +
                lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        R * c
    }
}

impl Default for GeoCoordinates {
    fn default() -> Self {
        // São Paulo, BR
        Self::new(-23.5505, -46.6333)
    }
}

/// Posição em mapa sinóptico (normalizada 0-1)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MapPosition {
    pub x: f32,
    pub y: f32,
}

impl MapPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: x.clamp(0.0, 1.0),
            y: y.clamp(0.0, 1.0),
        }
    }
}

impl Default for MapPosition {
    fn default() -> Self {
        Self::new(0.5, 0.5)
    }
}

/// Tipo de objeto no mapa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapObjectType {
    /// Câmera
    Camera {
        camera_id: CameraId,
        fov_degrees: Option<f32>,
        fov_direction: Option<f32>,
        fov_range: Option<f32>,
    },

    /// Sensor de alarme
    AlarmSensor {
        sensor_id: String,
        sensor_type: String,
    },

    /// Relé/saída digital
    DigitalOutput {
        device_id: String,
        port: u8,
    },

    /// Link para outro mapa
    MapLink {
        target_map_id: MapId,
    },

    /// Ícone customizado
    CustomIcon {
        icon: String,
        tooltip: String,
    },

    /// Área/zona (polígono)
    Zone {
        zone_id: String,
        polygon: Vec<MapPosition>,
    },

    /// Contador
    Counter {
        counter_id: String,
        display_value: String,
    },

    /// Texto/label
    Label {
        text: String,
        font_size: u8,
    },
}

/// Status de objeto no mapa
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MapObjectStatus {
    Normal,
    Active,
    Alert,
    Alarm,
    Offline,
    Disabled,
}

impl Default for MapObjectStatus {
    fn default() -> Self {
        Self::Normal
    }
}

impl MapObjectStatus {
    pub fn color(&self) -> &'static str {
        match self {
            Self::Normal => "#00FF00",
            Self::Active => "#0088FF",
            Self::Alert => "#FFFF00",
            Self::Alarm => "#FF0000",
            Self::Offline => "#808080",
            Self::Disabled => "#404040",
        }
    }
}

/// Objeto em mapa sinóptico
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapObject {
    /// ID do objeto
    pub id: MapObjectId,

    /// Tipo de objeto
    pub object_type: MapObjectType,

    /// Posição (para sinóptico)
    pub position: MapPosition,

    /// Coordenadas geográficas (para operacional)
    pub geo_coords: Option<GeoCoordinates>,

    /// Rotação (graus)
    pub rotation: f32,

    /// Escala
    pub scale: f32,

    /// Ícone customizado (override)
    pub custom_icon: Option<String>,

    /// Status atual
    pub status: MapObjectStatus,

    /// Visível
    pub is_visible: bool,

    /// Tooltip
    pub tooltip: Option<String>,

    /// Ações ao clicar
    pub click_actions: Vec<ObjectClickAction>,

    /// Z-index (ordem de desenho)
    pub z_index: i32,
}

impl MapObject {
    pub fn camera(camera_id: CameraId, position: MapPosition) -> Self {
        Self {
            id: MapObjectId::new(),
            object_type: MapObjectType::Camera {
                camera_id,
                fov_degrees: Some(90.0),
                fov_direction: Some(0.0),
                fov_range: Some(0.1),
            },
            position,
            geo_coords: None,
            rotation: 0.0,
            scale: 1.0,
            custom_icon: None,
            status: MapObjectStatus::Normal,
            is_visible: true,
            tooltip: None,
            click_actions: vec![ObjectClickAction::ShowLiveView],
            z_index: 10,
        }
    }

    pub fn map_link(target_map_id: MapId, position: MapPosition, tooltip: &str) -> Self {
        Self {
            id: MapObjectId::new(),
            object_type: MapObjectType::MapLink { target_map_id },
            position,
            geo_coords: None,
            rotation: 0.0,
            scale: 1.0,
            custom_icon: None,
            status: MapObjectStatus::Normal,
            is_visible: true,
            tooltip: Some(tooltip.to_string()),
            click_actions: vec![ObjectClickAction::NavigateToMap],
            z_index: 5,
        }
    }

    pub fn with_geo_coords(mut self, coords: GeoCoordinates) -> Self {
        self.geo_coords = Some(coords);
        self
    }

    pub fn with_fov(mut self, degrees: f32, direction: f32, range: f32) -> Self {
        if let MapObjectType::Camera { ref mut fov_degrees, ref mut fov_direction, ref mut fov_range, .. } = self.object_type {
            *fov_degrees = Some(degrees);
            *fov_direction = Some(direction);
            *fov_range = Some(range);
        }
        self
    }
}

/// Ação ao clicar em objeto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectClickAction {
    /// Mostrar live view
    ShowLiveView,
    /// Mostrar popup com detalhes
    ShowDetails,
    /// Navegar para outro mapa
    NavigateToMap,
    /// Executar ação de evento
    ExecuteAction { action_id: Uuid },
    /// Acionar saída digital
    TriggerOutput { device_id: String, port: u8, state: bool },
    /// Controlar PTZ
    ControlPTZ { preset_id: Option<String> },
    /// URL customizado
    OpenURL { url: String },
}

/// Camada do mapa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapLayer {
    /// ID da camada
    pub id: Uuid,

    /// Nome
    pub name: String,

    /// Objetos na camada
    pub objects: Vec<MapObject>,

    /// Visível
    pub is_visible: bool,

    /// Opacidade (0-1)
    pub opacity: f32,

    /// Ordem
    pub order: i32,
}

impl MapLayer {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            objects: Vec::new(),
            is_visible: true,
            opacity: 1.0,
            order: 0,
        }
    }

    pub fn cameras_layer() -> Self {
        let mut layer = Self::new("Cameras");
        layer.order = 10;
        layer
    }

    pub fn sensors_layer() -> Self {
        let mut layer = Self::new("Sensors");
        layer.order = 5;
        layer
    }

    pub fn add_object(&mut self, object: MapObject) {
        self.objects.push(object);
    }
}

/// Mapa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    /// ID do mapa
    pub id: MapId,

    /// Nome
    pub name: String,

    /// Descrição
    pub description: Option<String>,

    /// Tipo de mapa
    pub map_type: MapType,

    /// Imagem de fundo (para sinóptico)
    pub background_image: Option<String>,

    /// Cor de fundo
    pub background_color: String,

    /// Provedor de mapa (para operacional)
    pub map_provider: Option<MapProvider>,

    /// Centro do mapa (para operacional)
    pub center_coords: GeoCoordinates,

    /// Zoom inicial (para operacional)
    pub initial_zoom: u8,

    /// Camadas
    pub layers: Vec<MapLayer>,

    /// Mapa pai (para navegação hierárquica)
    pub parent_map_id: Option<MapId>,

    /// Mapas filhos
    pub child_map_ids: Vec<MapId>,

    /// Criado em
    pub created_at: DateTime<Utc>,

    /// Atualizado em
    pub updated_at: DateTime<Utc>,

    /// Criado por
    pub created_by: String,

    /// Tags
    pub tags: Vec<String>,

    /// É público
    pub is_public: bool,
}

impl Map {
    /// Cria mapa sinóptico
    pub fn synoptic(name: &str, background_image: &str, created_by: &str) -> Self {
        let mut cameras_layer = MapLayer::cameras_layer();
        cameras_layer.order = 10;

        Self {
            id: MapId::new(),
            name: name.to_string(),
            description: None,
            map_type: MapType::Synoptic,
            background_image: Some(background_image.to_string()),
            background_color: "#1a1a2e".to_string(),
            map_provider: None,
            center_coords: GeoCoordinates::default(),
            initial_zoom: 15,
            layers: vec![cameras_layer],
            parent_map_id: None,
            child_map_ids: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: created_by.to_string(),
            tags: Vec::new(),
            is_public: true,
        }
    }

    /// Cria mapa operacional
    pub fn operational(name: &str, center: GeoCoordinates, created_by: &str) -> Self {
        Self {
            id: MapId::new(),
            name: name.to_string(),
            description: None,
            map_type: MapType::Operational,
            background_image: None,
            background_color: "#1a1a2e".to_string(),
            map_provider: Some(MapProvider::OpenStreetMap),
            center_coords: center,
            initial_zoom: 15,
            layers: vec![MapLayer::cameras_layer()],
            parent_map_id: None,
            child_map_ids: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: created_by.to_string(),
            tags: Vec::new(),
            is_public: true,
        }
    }

    /// Adiciona camada
    pub fn add_layer(&mut self, layer: MapLayer) {
        self.layers.push(layer);
        self.updated_at = Utc::now();
    }

    /// Adiciona objeto à primeira camada
    pub fn add_object(&mut self, object: MapObject) {
        if let Some(layer) = self.layers.first_mut() {
            layer.add_object(object);
            self.updated_at = Utc::now();
        }
    }

    /// Obtém todos os objetos
    pub fn all_objects(&self) -> Vec<&MapObject> {
        self.layers.iter()
            .flat_map(|l| &l.objects)
            .collect()
    }

    /// Obtém objeto por ID
    pub fn get_object(&self, id: MapObjectId) -> Option<&MapObject> {
        self.layers.iter()
            .flat_map(|l| &l.objects)
            .find(|o| o.id == id)
    }

    /// Obtém objetos de câmera
    pub fn camera_objects(&self) -> Vec<(&MapObject, CameraId)> {
        self.all_objects().iter()
            .filter_map(|o| {
                if let MapObjectType::Camera { camera_id, .. } = &o.object_type {
                    Some((*o, *camera_id))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Atualiza status de câmera
    pub fn update_camera_status(&mut self, camera_id: CameraId, status: MapObjectStatus) {
        for layer in &mut self.layers {
            for object in &mut layer.objects {
                if let MapObjectType::Camera { camera_id: obj_camera_id, .. } = &object.object_type {
                    if *obj_camera_id == camera_id {
                        object.status = status;
                    }
                }
            }
        }
    }
}

/// Navegador de mapas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapNavigator {
    /// Mapa atual
    pub current_map_id: MapId,

    /// Histórico de navegação
    pub history: Vec<MapId>,

    /// Zoom atual
    pub current_zoom: f32,

    /// Posição de visualização (centro)
    pub view_center: MapPosition,
}

impl MapNavigator {
    pub fn new(initial_map_id: MapId) -> Self {
        Self {
            current_map_id: initial_map_id,
            history: Vec::new(),
            current_zoom: 1.0,
            view_center: MapPosition::default(),
        }
    }

    pub fn navigate_to(&mut self, map_id: MapId) {
        self.history.push(self.current_map_id);
        self.current_map_id = map_id;
        self.current_zoom = 1.0;
        self.view_center = MapPosition::default();
    }

    pub fn go_back(&mut self) -> bool {
        if let Some(previous) = self.history.pop() {
            self.current_map_id = previous;
            true
        } else {
            false
        }
    }

    pub fn can_go_back(&self) -> bool {
        !self.history.is_empty()
    }

    pub fn zoom_in(&mut self) {
        self.current_zoom = (self.current_zoom * 1.2).min(10.0);
    }

    pub fn zoom_out(&mut self) {
        self.current_zoom = (self.current_zoom / 1.2).max(0.5);
    }

    pub fn reset_view(&mut self) {
        self.current_zoom = 1.0;
        self.view_center = MapPosition::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geo_distance() {
        let sp = GeoCoordinates::new(-23.5505, -46.6333);
        let rj = GeoCoordinates::new(-22.9068, -43.1729);

        let distance = sp.distance_to(&rj);
        // São Paulo - Rio de Janeiro ~ 357 km
        assert!(distance > 350_000.0 && distance < 400_000.0);
    }

    #[test]
    fn test_synoptic_map() {
        let mut map = Map::synoptic("Floor 1", "/images/floor1.png", "admin");

        let camera_id = CameraId::new();
        let camera = MapObject::camera(camera_id, MapPosition::new(0.3, 0.5));

        map.add_object(camera);

        assert_eq!(map.camera_objects().len(), 1);
    }

    #[test]
    fn test_operational_map() {
        let center = GeoCoordinates::new(-23.5505, -46.6333);
        let map = Map::operational("São Paulo", center, "admin");

        assert_eq!(map.map_type, MapType::Operational);
        assert!(map.map_provider.is_some());
    }

    #[test]
    fn test_map_navigator() {
        let map1 = MapId::new();
        let map2 = MapId::new();

        let mut nav = MapNavigator::new(map1);
        assert!(!nav.can_go_back());

        nav.navigate_to(map2);
        assert!(nav.can_go_back());
        assert_eq!(nav.current_map_id, map2);

        nav.go_back();
        assert_eq!(nav.current_map_id, map1);
        assert!(!nav.can_go_back());
    }
}
