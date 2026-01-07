//! VMS ONVIF Service
//! Descoberta e controle de câmeras ONVIF

use anyhow::Result;
use tracing::info;
use tracing_subscriber;

mod digest_auth;
mod client;
mod discovery;
mod device;
mod camera;

pub mod xml_utils;

pub use client::{OnvifClient, OnvifError};
pub use discovery::OnvifDiscovery;
pub use device::OnvifDevice;
pub use camera::{Camera, CameraProfile};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🎥 VMS ONVIF Service starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Demo: Descobrir câmeras na rede
    info!("🔍 Buscando câmeras ONVIF na rede...");
    
    let discovery = OnvifDiscovery::new();
    let cameras = discovery.discover(std::time::Duration::from_secs(5)).await?;
    
    info!("📹 Encontradas {} câmeras:", cameras.len());
    for camera in &cameras {
        info!("  - {} ({})", camera.name, camera.url);
    }

    info!("✅ ONVIF Service ready");
    
    // TODO: Integrar com vms-api via gRPC ou NATS
    // Por enquanto, apenas mantém o serviço rodando
    tokio::signal::ctrl_c().await?;
    
    info!("👋 Goodbye!");
    Ok(())
}
