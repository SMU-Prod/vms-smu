//! Rotas de discovery ONVIF
//! Descobre câmeras na rede local

use axum::{
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize)]
pub struct DiscoveredCamera {
    pub name: String,
    pub url: String,
    pub device_type: Option<String>,
    pub scopes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DiscoverRequest {
    /// Timeout em segundos (default: 5)
    pub timeout_secs: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct DiscoverResponse {
    pub cameras: Vec<DiscoveredCamera>,
    pub count: usize,
}

/// POST /cameras/discover - Descobrir câmeras ONVIF na rede
pub async fn discover_cameras(
    Json(_req): Json<DiscoverRequest>,
) -> Result<Json<DiscoverResponse>, StatusCode> {
    info!("🔍 Iniciando discovery ONVIF...");

    let mut cameras = Vec::new();

    // Usar onvif-cam-rs v0.2 para discovery
    match onvif_cam_rs::client::discover().await {
        Ok(devices) => {
            for device in devices {
                cameras.push(DiscoveredCamera {
                    name: format!("Câmera ONVIF - {}", device.url_onvif),
                    url: device.url_onvif.to_string(),
                    device_type: Some("NetworkVideoTransmitter".to_string()),
                    scopes: device.scopes.iter().map(|s: &onvif_cam_rs::Scope| s.to_string()).collect(),
                });
            }
        }
        Err(e) => {
            info!("Erro no discovery: {}", e);
            // Retorna lista vazia, não erro
        }
    }

    let count = cameras.len();
    info!("✅ Discovery completo: {} câmeras", count);

    Ok(Json(DiscoverResponse { cameras, count }))
}

/// Request para conectar a uma câmera
#[derive(Debug, Deserialize)]
pub struct ConnectRequest {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

/// Profile de câmera
#[derive(Debug, Serialize)]
pub struct CameraProfile {
    pub name: String,
    pub rtsp_url: Option<String>,
}

/// POST /cameras/profiles - Listar profiles de uma câmera
/// NOTA: API simplificada devido à mudança na biblioteca onvif-cam-rs v0.2
pub async fn get_camera_profiles(
    Json(_req): Json<ConnectRequest>,
) -> Result<Json<Vec<CameraProfile>>, StatusCode> {
    info!("📹 Funcionalidade de profiles temporariamente desabilitada");

    // TODO: Reimplementar usando onvif-cam-rs v0.2 Camera builder
    // A API v0.2 mudou significativamente e requer uma abordagem diferente
    // usando Camera::new(device) e camera.build_all().await

    Ok(Json(vec![]))
}

/// Request para controle PTZ
#[derive(Debug, Deserialize)]
pub struct PtzRequest {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub pan: f32,
    pub tilt: f32,
    pub zoom: f32,
}

/// POST /cameras/ptz - Controle PTZ
/// NOTA: API simplificada devido à mudança na biblioteca onvif-cam-rs v0.2
pub async fn control_ptz(
    Json(_req): Json<PtzRequest>,
) -> Result<StatusCode, StatusCode> {
    info!("📹 Funcionalidade PTZ temporariamente desabilitada");

    // TODO: Reimplementar usando onvif-cam-rs v0.2
    // A API v0.2 mudou e pode não ter suporte direto a PTZ

    Ok(StatusCode::NOT_IMPLEMENTED)
}
