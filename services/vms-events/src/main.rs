//! VMS Events Service
//! Sistema de eventos, alarmes e regras (tipo Digifort)

use anyhow::{Context, Result};
use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tracing::{info, warn};
use tracing_subscriber;

mod alarm;
mod event;
mod rule;

use alarm::{Alarm, AlarmManager, AlarmPriority, AlarmStatus};
use event::{Event, EventType};
use rule::{Rule, RuleEngine};

/// App state
#[derive(Clone)]
struct AppState {
    alarm_manager: Arc<AlarmManager>,
    rule_engine: Arc<RuleEngine>,
    event_history: Arc<RwLock<Vec<Event>>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🚀 VMS Events Service starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Conectar ao NATS
    let nats_url = std::env::var("NATS_URL")
        .unwrap_or_else(|_| "nats://localhost:4222".to_string());
    let nats_client = async_nats::connect(&nats_url)
        .await
        .context("Failed to connect to NATS")?;
    info!("📡 NATS connected: {}", nats_url);

    // Inicializar componentes
    let alarm_manager = Arc::new(AlarmManager::new());
    let rule_engine = Arc::new(RuleEngine::new());
    let event_history = Arc::new(RwLock::new(Vec::new()));

    let state = AppState {
        alarm_manager: alarm_manager.clone(),
        rule_engine: rule_engine.clone(),
        event_history: event_history.clone(),
    };

    // Subscriber para eventos de IA
    let nats_client_clone = nats_client.clone();
    let state_clone = state.clone();
    tokio::spawn(async move {
        if let Err(e) = consume_ai_events(nats_client_clone, state_clone).await {
            warn!("AI events consumer error: {}", e);
        }
    });

    // Subscriber para eventos de câmera
    let nats_client_clone = nats_client.clone();
    let state_clone = state.clone();
    tokio::spawn(async move {
        if let Err(e) = consume_camera_events(nats_client_clone, state_clone).await {
            warn!("Camera events consumer error: {}", e);
        }
    });

    // API REST
    let app = Router::new()
        // Health
        .route("/health", get(|| async { "OK" }))
        .route("/metrics", get(metrics))
        // Events
        .route("/api/v1/events", get(list_events))
        .route("/api/v1/events/:id", get(get_event))
        // Alarms
        .route("/api/v1/alarms", get(list_alarms).post(create_alarm))
        .route(
            "/api/v1/alarms/:id",
            get(get_alarm)
                .put(update_alarm)
                .delete(delete_alarm),
        )
        .route("/api/v1/alarms/:id/acknowledge", post(acknowledge_alarm))
        .route("/api/v1/alarms/active", get(get_active_alarms))
        // Rules
        .route("/api/v1/rules", get(list_rules).post(create_rule))
        .route(
            "/api/v1/rules/:id",
            get(get_rule).put(update_rule).delete(delete_rule),
        )
        .route("/api/v1/rules/:id/enable", post(enable_rule))
        .route("/api/v1/rules/:id/disable", post(disable_rule))
        .with_state(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 9096));
    let listener = TcpListener::bind(addr).await?;

    info!("🌐 HTTP API listening on http://{}", addr);
    info!("✅ Service initialized successfully");

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
        })
        .await?;

    info!("👋 Goodbye!");
    Ok(())
}

// ============================================================================
// Event Consumers
// ============================================================================

async fn consume_ai_events(
    client: async_nats::Client,
    state: AppState,
) -> Result<()> {
    use tokio_stream::StreamExt;

    let mut subscriber = client.subscribe("vms.events.ai.>").await?;
    info!("📡 Subscribed to AI events: vms.events.ai.>");

    while let Some(message) = subscriber.next().await {
        match serde_json::from_slice::<serde_json::Value>(&message.payload) {
            Ok(payload) => {
                // Criar evento
                let event = Event {
                    id: uuid::Uuid::new_v4(),
                    event_type: EventType::AIDetection,
                    timestamp: chrono::Utc::now(),
                    camera_id: payload["camera_id"].as_str().map(|s| s.to_string()),
                    data: payload.clone(),
                };

                // Adicionar ao histórico
                {
                    let mut history = state.event_history.write().await;
                    history.push(event.clone());
                    // Manter apenas últimos 10000 eventos
                    if history.len() > 10000 {
                        history.drain(0..5000);
                    }
                }

                // Processar regras
                state.rule_engine.process_event(&event, &state.alarm_manager).await;
            }
            Err(e) => {
                warn!("Failed to parse AI event: {}", e);
            }
        }
    }

    Ok(())
}

async fn consume_camera_events(
    client: async_nats::Client,
    state: AppState,
) -> Result<()> {
    use tokio_stream::StreamExt;

    let mut subscriber = client.subscribe("vms.events.camera.>").await?;
    info!("📡 Subscribed to camera events: vms.events.camera.>");

    while let Some(message) = subscriber.next().await {
        match serde_json::from_slice::<serde_json::Value>(&message.payload) {
            Ok(payload) => {
                let event = Event {
                    id: uuid::Uuid::new_v4(),
                    event_type: EventType::CameraStatus,
                    timestamp: chrono::Utc::now(),
                    camera_id: payload["camera_id"].as_str().map(|s| s.to_string()),
                    data: payload,
                };

                {
                    let mut history = state.event_history.write().await;
                    history.push(event.clone());
                }

                state.rule_engine.process_event(&event, &state.alarm_manager).await;
            }
            Err(e) => {
                warn!("Failed to parse camera event: {}", e);
            }
        }
    }

    Ok(())
}

// ============================================================================
// API Handlers
// ============================================================================

async fn metrics() -> String {
    format!(
        "# Events metrics\n\
         vms_events_total 0\n\
         vms_alarms_active 0\n"
    )
}

// Events
async fn list_events(State(state): State<AppState>) -> Json<Vec<Event>> {
    let history = state.event_history.read().await;
    Json(history.clone())
}

async fn get_event(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Event>, String> {
    let history = state.event_history.read().await;
    history
        .iter()
        .find(|e| e.id == id)
        .cloned()
        .map(Json)
        .ok_or_else(|| "Event not found".to_string())
}

// Alarms
async fn list_alarms(State(state): State<AppState>) -> Json<Vec<Alarm>> {
    Json(state.alarm_manager.list_all().await)
}

async fn get_active_alarms(State(state): State<AppState>) -> Json<Vec<Alarm>> {
    Json(state.alarm_manager.list_active().await)
}

async fn get_alarm(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Alarm>, String> {
    state
        .alarm_manager
        .get(id)
        .await
        .map(Json)
        .ok_or_else(|| "Alarm not found".to_string())
}

async fn create_alarm(
    State(state): State<AppState>,
    Json(alarm): Json<Alarm>,
) -> Json<Alarm> {
    state.alarm_manager.add(alarm.clone()).await;
    Json(alarm)
}

async fn update_alarm(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(alarm): Json<Alarm>,
) -> Result<Json<Alarm>, String> {
    state.alarm_manager.update(id, alarm.clone()).await?;
    Ok(Json(alarm))
}

async fn delete_alarm(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<String, String> {
    state.alarm_manager.delete(id).await?;
    Ok("Deleted".to_string())
}

async fn acknowledge_alarm(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Alarm>, String> {
    state.alarm_manager.acknowledge(id).await?;
    state
        .alarm_manager
        .get(id)
        .await
        .map(Json)
        .ok_or_else(|| "Alarm not found".to_string())
}

// Rules
async fn list_rules(State(state): State<AppState>) -> Json<Vec<Rule>> {
    Json(state.rule_engine.list_all().await)
}

async fn get_rule(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Rule>, String> {
    state
        .rule_engine
        .get(id)
        .await
        .map(Json)
        .ok_or_else(|| "Rule not found".to_string())
}

async fn create_rule(
    State(state): State<AppState>,
    Json(rule): Json<Rule>,
) -> Json<Rule> {
    state.rule_engine.add(rule.clone()).await;
    Json(rule)
}

async fn update_rule(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(rule): Json<Rule>,
) -> Result<Json<Rule>, String> {
    state.rule_engine.update(id, rule.clone()).await?;
    Ok(Json(rule))
}

async fn delete_rule(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<String, String> {
    state.rule_engine.delete(id).await?;
    Ok("Deleted".to_string())
}

async fn enable_rule(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Rule>, String> {
    state.rule_engine.enable(id).await?;
    state
        .rule_engine
        .get(id)
        .await
        .map(Json)
        .ok_or_else(|| "Rule not found".to_string())
}

async fn disable_rule(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Rule>, String> {
    state.rule_engine.disable(id).await?;
    state
        .rule_engine
        .get(id)
        .await
        .map(Json)
        .ok_or_else(|| "Rule not found".to_string())
}
