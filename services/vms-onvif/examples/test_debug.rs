//! Teste de debug ONVIF - Log detalhado
//! 
//! Execução:
//! RUST_LOG=debug cargo run --package vms-onvif --example test_debug

use anyhow::Result;
use vms_onvif::OnvifClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Configurar logging com nível DEBUG
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    println!("🔍 Teste de Debug ONVIF - Tapo C100");
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

    // Criar cliente ONVIF
    let client = OnvifClient::new(&base_url, username, password)?;

    // Teste simples: GetDeviceInformation
    println!("🔍 Enviando GetDeviceInformation");
    println!("----------------------------------");
    
    let soap_body = r#"<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/" xmlns:tds="http://www.onvif.org/ver10/device/wsdl">
  <s:Body>
    <tds:GetDeviceInformation/>
  </s:Body>
</s:Envelope>"#;

    println!("📤 Request SOAP:");
    println!("{}", soap_body);
    println!();

    match client.soap_post_digest("/onvif/device_service", None, soap_body).await {
        Ok(response) => {
            println!("✅ Sucesso!");
            println!("📥 Response:");
            println!("{}", &response[..response.len().min(500)]);
        }
        Err(e) => {
            println!("❌ Erro: {}", e);
            println!("\n💡 Dica: Verifique os logs DEBUG acima para detalhes");
        }
    }
    
    Ok(())
}
