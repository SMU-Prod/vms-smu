//! Sistema de Eventos e Ações
//!
//! Define eventos que podem ser detectados e ações que podem ser executadas.
//! Similar ao sistema de eventos do Digifort.

use crate::types::CameraId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// ID único de um evento
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventId(pub Uuid);

impl EventId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// ID de definição de evento
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventDefinitionId(pub Uuid);

impl EventDefinitionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for EventDefinitionId {
    fn default() -> Self {
        Self::new()
    }
}

/// ID de ação
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActionId(pub Uuid);

impl ActionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ActionId {
    fn default() -> Self {
        Self::new()
    }
}

/// Severidade do evento
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventSeverity {
    Debug,
    Info,
    Warning,
    Alert,
    Critical,
    Emergency,
}

impl Default for EventSeverity {
    fn default() -> Self {
        Self::Info
    }
}

/// Categoria do evento
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventCategory {
    /// Sistema (conectividade, erros, etc)
    System,
    /// Movimento detectado
    Motion,
    /// Analytics (objetos, contagem, etc)
    Analytics,
    /// LPR (placas)
    LPR,
    /// Facial (reconhecimento)
    Facial,
    /// I/O (sensores, relés)
    IO,
    /// PTZ (movimento de câmera)
    PTZ,
    /// Gravação
    Recording,
    /// Alarme manual
    Manual,
    /// Customizado
    Custom(String),
}

impl Default for EventCategory {
    fn default() -> Self {
        Self::System
    }
}

/// Tipo de trigger de evento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventTrigger {
    /// Movimento detectado em câmera
    MotionDetected {
        camera_id: CameraId,
        zone_id: Option<String>,
        sensitivity: f32,
    },

    /// Objeto detectado por analytics
    ObjectDetected {
        camera_id: CameraId,
        object_type: String,
        confidence: f32,
    },

    /// Linha cruzada (contagem)
    LineCrossed {
        camera_id: CameraId,
        line_id: String,
        direction: String,
        count: u32,
    },

    /// Presença em zona
    ZonePresence {
        camera_id: CameraId,
        zone_id: String,
        object_count: u32,
        duration_seconds: u32,
    },

    /// Placa reconhecida
    PlateRecognized {
        camera_id: CameraId,
        plate: String,
        confidence: f32,
        list_match: Option<String>,
    },

    /// Face reconhecida
    FaceRecognized {
        camera_id: CameraId,
        person_id: Option<String>,
        confidence: f32,
    },

    /// Câmera offline
    CameraOffline { camera_id: CameraId },

    /// Câmera online
    CameraOnline { camera_id: CameraId },

    /// Obstrução de câmera
    CameraObstructed { camera_id: CameraId },

    /// Disco cheio
    DiskFull {
        disk_path: String,
        usage_percent: f32,
    },

    /// Timer/agendado
    Scheduled {
        schedule_id: String,
        schedule_name: String,
    },

    /// I/O digital
    DigitalIO {
        device_id: String,
        port: u8,
        state: bool,
    },

    /// PTZ preset atingido
    PTZPresetReached {
        camera_id: CameraId,
        preset_id: String,
    },

    /// Manual/API
    Manual {
        user_id: Option<String>,
        reason: String,
    },

    /// Customizado
    Custom {
        event_type: String,
        data: HashMap<String, String>,
    },
}

/// Ocorrência de evento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// ID único do evento
    pub id: EventId,

    /// Timestamp do evento
    pub timestamp: DateTime<Utc>,

    /// Trigger que causou o evento
    pub trigger: EventTrigger,

    /// Categoria
    pub category: EventCategory,

    /// Severidade
    pub severity: EventSeverity,

    /// Mensagem descritiva
    pub message: String,

    /// Câmera relacionada (se aplicável)
    pub camera_id: Option<CameraId>,

    /// Dados adicionais
    pub metadata: HashMap<String, String>,

    /// Snapshot/Thumbnail (path ou base64)
    pub snapshot: Option<String>,

    /// Clip de vídeo (path)
    pub video_clip: Option<String>,

    /// Foi reconhecido pelo operador
    pub acknowledged: bool,

    /// Timestamp do reconhecimento
    pub acknowledged_at: Option<DateTime<Utc>>,

    /// Usuário que reconheceu
    pub acknowledged_by: Option<String>,

    /// Notas do operador
    pub notes: Option<String>,
}

impl Event {
    pub fn new(trigger: EventTrigger, category: EventCategory, message: &str) -> Self {
        let camera_id = match &trigger {
            EventTrigger::MotionDetected { camera_id, .. } => Some(*camera_id),
            EventTrigger::ObjectDetected { camera_id, .. } => Some(*camera_id),
            EventTrigger::LineCrossed { camera_id, .. } => Some(*camera_id),
            EventTrigger::ZonePresence { camera_id, .. } => Some(*camera_id),
            EventTrigger::PlateRecognized { camera_id, .. } => Some(*camera_id),
            EventTrigger::FaceRecognized { camera_id, .. } => Some(*camera_id),
            EventTrigger::CameraOffline { camera_id } => Some(*camera_id),
            EventTrigger::CameraOnline { camera_id } => Some(*camera_id),
            EventTrigger::CameraObstructed { camera_id } => Some(*camera_id),
            EventTrigger::PTZPresetReached { camera_id, .. } => Some(*camera_id),
            _ => None,
        };

        Self {
            id: EventId::new(),
            timestamp: Utc::now(),
            trigger,
            category,
            severity: EventSeverity::Info,
            message: message.to_string(),
            camera_id,
            metadata: HashMap::new(),
            snapshot: None,
            video_clip: None,
            acknowledged: false,
            acknowledged_at: None,
            acknowledged_by: None,
            notes: None,
        }
    }

    pub fn with_severity(mut self, severity: EventSeverity) -> Self {
        self.severity = severity;
        self
    }

    pub fn with_snapshot(mut self, path: String) -> Self {
        self.snapshot = Some(path);
        self
    }

    pub fn acknowledge(&mut self, user: &str, notes: Option<&str>) {
        self.acknowledged = true;
        self.acknowledged_at = Some(Utc::now());
        self.acknowledged_by = Some(user.to_string());
        self.notes = notes.map(|s| s.to_string());
    }
}

/// Tipo de ação a executar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    /// Enviar e-mail
    SendEmail {
        to: Vec<String>,
        subject: String,
        body_template: String,
        attach_snapshot: bool,
        attach_clip: bool,
    },

    /// Push notification mobile
    PushNotification {
        title: String,
        body: String,
        data: HashMap<String, String>,
    },

    /// Tocar som
    PlaySound {
        sound_file: String,
        device: Option<String>,
    },

    /// Pop-up de alarme
    ShowPopup {
        title: String,
        message: String,
        camera_ids: Vec<CameraId>,
        auto_dismiss_seconds: Option<u32>,
    },

    /// Mover PTZ para preset
    PTZGotoPreset {
        camera_id: CameraId,
        preset_id: String,
    },

    /// Iniciar gravação
    StartRecording {
        camera_ids: Vec<CameraId>,
        duration_seconds: u32,
    },

    /// Criar bookmark
    CreateBookmark {
        camera_id: CameraId,
        name: String,
        description: Option<String>,
    },

    /// Webhook HTTP
    Webhook {
        url: String,
        method: String,
        headers: HashMap<String, String>,
        body_template: String,
    },

    /// Ativar/Desativar I/O
    SetDigitalOutput {
        device_id: String,
        port: u8,
        state: bool,
        pulse_ms: Option<u32>,
    },

    /// Executar script
    RunScript {
        script_path: String,
        arguments: Vec<String>,
    },

    /// Enviar para matriz virtual
    SendToMatrix {
        matrix_id: String,
        camera_ids: Vec<CameraId>,
        layout: Option<String>,
    },

    /// Enviar clipe de áudio
    PlayAudioClip {
        camera_id: CameraId,
        audio_file: String,
    },

    /// Chamar API externa
    APICall {
        endpoint: String,
        method: String,
        payload: String,
    },
}

/// Definição de ação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: ActionId,
    pub name: String,
    pub description: Option<String>,
    pub action_type: ActionType,
    pub enabled: bool,
    pub delay_seconds: u32,
    pub cooldown_seconds: u32,
}

impl Action {
    pub fn new(name: &str, action_type: ActionType) -> Self {
        Self {
            id: ActionId::new(),
            name: name.to_string(),
            description: None,
            action_type,
            enabled: true,
            delay_seconds: 0,
            cooldown_seconds: 60,
        }
    }
}

/// Operador lógico para condições
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogicalOperator {
    And,
    Or,
}

/// Condição para trigger de evento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCondition {
    /// Categorias de evento que ativam
    pub categories: Vec<EventCategory>,

    /// Severidade mínima
    pub min_severity: EventSeverity,

    /// Câmeras específicas (vazio = todas)
    pub camera_ids: Vec<CameraId>,

    /// Palavras-chave na mensagem
    pub keywords: Vec<String>,

    /// Operador entre condições
    pub operator: LogicalOperator,
}

impl Default for EventCondition {
    fn default() -> Self {
        Self {
            categories: Vec::new(),
            min_severity: EventSeverity::Info,
            camera_ids: Vec::new(),
            keywords: Vec::new(),
            operator: LogicalOperator::And,
        }
    }
}

/// Regra de evento (condição + ações)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRule {
    pub id: EventDefinitionId,
    pub name: String,
    pub description: Option<String>,
    pub condition: EventCondition,
    pub actions: Vec<Action>,
    pub enabled: bool,
    pub priority: u8,
}

impl EventRule {
    pub fn new(name: &str, condition: EventCondition) -> Self {
        Self {
            id: EventDefinitionId::new(),
            name: name.to_string(),
            description: None,
            condition,
            actions: Vec::new(),
            enabled: true,
            priority: 5,
        }
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }
}

/// Contato para notificações
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub push_enabled: bool,
    pub groups: Vec<String>,
}

impl Contact {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            email: None,
            phone: None,
            push_enabled: false,
            groups: Vec::new(),
        }
    }

    pub fn with_email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let trigger = EventTrigger::MotionDetected {
            camera_id: CameraId::new(),
            zone_id: None,
            sensitivity: 0.8,
        };

        let event = Event::new(trigger, EventCategory::Motion, "Motion detected in lobby");

        assert!(event.camera_id.is_some());
        assert!(!event.acknowledged);
    }

    #[test]
    fn test_event_acknowledge() {
        let trigger = EventTrigger::Manual {
            user_id: None,
            reason: "Test".to_string(),
        };

        let mut event = Event::new(trigger, EventCategory::Manual, "Manual trigger");
        event.acknowledge("admin", Some("Test note"));

        assert!(event.acknowledged);
        assert_eq!(event.acknowledged_by, Some("admin".to_string()));
    }

    #[test]
    fn test_event_rule() {
        let condition = EventCondition {
            categories: vec![EventCategory::Motion],
            ..Default::default()
        };

        let mut rule = EventRule::new("Motion Alert", condition);

        let email_action = Action::new(
            "Send Alert Email",
            ActionType::SendEmail {
                to: vec!["admin@example.com".to_string()],
                subject: "Motion Detected".to_string(),
                body_template: "Motion detected at {{camera_name}}".to_string(),
                attach_snapshot: true,
                attach_clip: false,
            },
        );

        rule.add_action(email_action);

        assert_eq!(rule.actions.len(), 1);
    }
}
