//! Teste completo ONVIF - GetProfiles e GetStreamUri
//! 
//! Execução:
//! cargo run --package vms-onvif --example test_full_onvif

use anyhow::Result;
use vms_onvif::OnvifDevice;

#[tokio::main]
async fn main() -> Result<()> {
    // Configurar logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    println!("🎥 Teste Completo ONVIF - Tapo C100");
    println!("====================================\n");

    // Configuração da câmera
    let camera_ip = "192.168.1.169";
    let onvif_port = 2020;
    let username = "adminsmu";
    let password = "Naotemsenha1@";

    let base_url = format!("http://{}:{}", camera_ip, onvif_port);
    
    println!("📡 Conectando a: {}", base_url);
    println!("👤 Usuário: {}", username);
    println!();

    // Criar dispositivo ONVIF
    let mut device = OnvifDevice::new(&base_url, username, password)?;

    // Conectar e obter capabilities
    println!("🔍 Passo 1: Conectar e obter capabilities");
    println!("------------------------------------------");
    device.connect().await?;
    println!("✅ Conectado com sucesso\n");

    // Obter informações do dispositivo
    println!("🔍 Passo 2: GetDeviceInformation");
    println!("------------------------------------------");
    match device.get_device_info().await {
        Ok(info) => {
            println!("✅ Informações obtidas:");
            println!("   Fabricante: {}", info.manufacturer);
            println!("   Modelo: {}", info.model);
            println!("   Firmware: {}", info.firmware_version);
            println!("   Serial: {}", info.serial_number);
        }
        Err(e) => {
            println!("❌ Erro: {}", e);
        }
    }
    println!();

    // Obter perfis de mídia
    println!("🔍 Passo 3: GetProfiles");
    println!("------------------------------------------");
    match device.get_profiles().await {
        Ok(profiles) => {
            println!("✅ Encontrados {} perfis de mídia:", profiles.len());
            for (i, profile) in profiles.iter().enumerate() {
                println!("\n   Perfil {}:", i + 1);
                println!("   - Token: {}", profile.token);
                println!("   - Nome: {}", profile.name);
                println!("   - Codec: {}", profile.video_encoding);
                println!("   - Resolução: {}x{}", profile.resolution.0, profile.resolution.1);
                println!("   - FPS: {}", profile.framerate);
            }

            // Obter Stream URI do primeiro perfil
            if let Some(first_profile) = profiles.first() {
                println!("\n🔍 Passo 4: GetStreamUri (perfil principal)");
                println!("------------------------------------------");
                match device.get_stream_uri(&first_profile.token).await {
                    Ok(stream_uri) => {
                        println!("✅ Stream URI obtida:");
                        println!("   {}", stream_uri);
                        println!("\n📺 Você pode testar o stream com:");
                        println!("   ffplay \"{}\"", stream_uri);
                        println!("   ou");
                        println!("   vlc \"{}\"", stream_uri);
                    }
                    Err(e) => {
                        println!("❌ Erro ao obter stream URI: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Erro ao obter perfis: {}", e);
        }
    }
    println!();

    println!("✅ Teste completo concluído!");
    println!("\n🎯 Próximos passos:");
    println!("   1. Integrar com vms-ingest para processar o stream RTSP");
    println!("   2. Criar pipeline GStreamer de baixa latência");
    println!("   3. Expor via API REST no vms-api");
    
    Ok(())
}
