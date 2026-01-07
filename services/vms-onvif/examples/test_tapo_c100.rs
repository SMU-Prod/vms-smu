//! Teste de conexão ONVIF com câmera Tapo C100
//! 
//! Execução:
//! cargo run --package vms-onvif --example test_tapo_c100

use anyhow::Result;

// Importar do crate vms_onvif (com underscore)
use vms_onvif::OnvifClient;
use vms_onvif::xml_utils;

fn soap_get_capabilities() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope"
            xmlns:tds="http://www.onvif.org/ver10/device/wsdl">
  <s:Body>
    <tds:GetCapabilities>
      <tds:Category>All</tds:Category>
    </tds:GetCapabilities>
  </s:Body>
</s:Envelope>"#.to_string()
}

fn soap_get_device_information() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope"
            xmlns:tds="http://www.onvif.org/ver10/device/wsdl">
  <s:Body>
    <tds:GetDeviceInformation/>
  </s:Body>
</s:Envelope>"#.to_string()
}

#[tokio::main]
async fn main() -> Result<()> {
    // Configurar logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    println!("🎥 Teste ONVIF - Câmera Tapo C100");
    println!("==================================\n");

    // Configuração da câmera
    let camera_ip = "192.168.1.169";
    let onvif_port = 2020;
    let username = "adminsmu";
    let password = "Naotemsenha1@";

    let base_url = format!("http://{}:{}", camera_ip, onvif_port);
    
    println!("📡 Conectando a: {}", base_url);
    println!("👤 Usuário: {}", username);
    println!();

    // Criar cliente ONVIF
    let client = OnvifClient::new(&base_url, username, password)?;

    // Teste 1: GetDeviceInformation
    println!("🔍 Teste 1: GetDeviceInformation");
    println!("----------------------------------");
    match client.soap_post_digest(
        "/onvif/device_service",
        None,
        &soap_get_device_information()
    ).await {
        Ok(response) => {
            println!("✅ Sucesso!");
            
            // Parse informações
            if let Some(manufacturer) = xml_utils::extract_first_tag_text(&response, "Manufacturer") {
                println!("   Fabricante: {}", manufacturer);
            }
            if let Some(model) = xml_utils::extract_first_tag_text(&response, "Model") {
                println!("   Modelo: {}", model);
            }
            if let Some(firmware) = xml_utils::extract_first_tag_text(&response, "FirmwareVersion") {
                println!("   Firmware: {}", firmware);
            }
            if let Some(serial) = xml_utils::extract_first_tag_text(&response, "SerialNumber") {
                println!("   Serial: {}", serial);
            }
        }
        Err(e) => {
            println!("❌ Erro: {}", e);
            println!("   Verifique se:");
            println!("   - A câmera está ligada e acessível em {}", base_url);
            println!("   - As credenciais estão corretas");
            println!("   - A porta ONVIF é 2020");
        }
    }
    println!();

    // Teste 2: GetCapabilities
    println!("🔍 Teste 2: GetCapabilities");
    println!("----------------------------------");
    match client.soap_post_digest(
        "/onvif/device_service",
        None,
        &soap_get_capabilities()
    ).await {
        Ok(response) => {
            println!("✅ Sucesso!");
            
            // Extrair Media Service URL
            if let Some(media_url) = xml_utils::extract_first_tag_text(&response, "XAddr") {
                println!("   Media Service URL: {}", media_url);
            }
            
            // Verificar capacidades
            if response.contains("PTZ") {
                println!("   ✓ Suporta PTZ");
            }
            if response.contains("Analytics") {
                println!("   ✓ Suporta Analytics");
            }
            if response.contains("Media") {
                println!("   ✓ Suporta Media");
            }
        }
        Err(e) => {
            println!("❌ Erro: {}", e);
        }
    }
    println!();

    println!("✅ Testes concluídos!");
    
    Ok(())
}
