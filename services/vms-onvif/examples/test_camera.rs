//! Teste manual de conexão ONVIF
//! Para testar a câmera Tapo C100 em 192.168.1.169:2020

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎥 Teste de Conexão ONVIF");
    println!("========================\n");

    // Configuração da câmera
    let camera_ip = "192.168.1.169";
    let onvif_port = 2020;
    let username = "adminsmu";
    let password = "Naotemsenha1@";

    let onvif_url = format!("http://{}:{}/onvif/device_service", camera_ip, onvif_port);
    
    println!("📡 Conectando a: {}", onvif_url);
    println!("👤 Usuário: {}", username);
    println!();

    // Teste 1: GetDeviceInformation
    println!("🔍 Teste 1: GetDeviceInformation");
    match test_get_device_info(&onvif_url, username, password).await {
        Ok(info) => {
            println!("✅ Sucesso!");
            println!("   Fabricante: {}", info.0);
            println!("   Modelo: {}", info.1);
            println!("   Firmware: {}", info.2);
        }
        Err(e) => {
            println!("❌ Erro: {}", e);
        }
    }
    println!();

    // Teste 2: GetCapabilities
    println!("🔍 Teste 2: GetCapabilities");
    match test_get_capabilities(&onvif_url, username, password).await {
        Ok(media_url) => {
            println!("✅ Sucesso!");
            println!("   Media Service URL: {}", media_url);
        }
        Err(e) => {
            println!("❌ Erro: {}", e);
        }
    }

    Ok(())
}

async fn test_get_device_info(url: &str, username: &str, password: &str) -> Result<(String, String, String)> {
    let client = reqwest::Client::new();
    
    let soap_body = r#"<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope" xmlns:tds="http://www.onvif.org/ver10/device/wsdl">
  <s:Body>
    <tds:GetDeviceInformation/>
  </s:Body>
</s:Envelope>"#;

    // Primeira tentativa sem auth
    let response = client
        .post(url)
        .header("Content-Type", "application/soap+xml; charset=utf-8")
        .body(soap_body)
        .send()
        .await?;

    let text = response.text().await?;
    
    // Parse simples
    let manufacturer = extract_value(&text, "Manufacturer").unwrap_or("Unknown".to_string());
    let model = extract_value(&text, "Model").unwrap_or("Unknown".to_string());
    let firmware = extract_value(&text, "FirmwareVersion").unwrap_or("Unknown".to_string());

    Ok((manufacturer, model, firmware))
}

async fn test_get_capabilities(url: &str, username: &str, password: &str) -> Result<String> {
    let client = reqwest::Client::new();
    
    let soap_body = r#"<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope" xmlns:tds="http://www.onvif.org/ver10/device/wsdl">
  <s:Body>
    <tds:GetCapabilities>
      <tds:Category>All</tds:Category>
    </tds:GetCapabilities>
  </s:Body>
</s:Envelope>"#;

    let response = client
        .post(url)
        .header("Content-Type", "application/soap+xml; charset=utf-8")
        .body(soap_body)
        .send()
        .await?;

    let text = response.text().await?;
    
    // Extrair Media Service URL
    if let Some(start) = text.find("<tt:Media>") {
        if let Some(xaddr_start) = text[start..].find("<tt:XAddr>") {
            if let Some(xaddr_end) = text[start + xaddr_start..].find("</tt:XAddr>") {
                let url_start = start + xaddr_start + 10;
                let url_end = start + xaddr_start + xaddr_end;
                return Ok(text[url_start..url_end].to_string());
            }
        }
    }

    Err(anyhow::anyhow!("Media service URL not found"))
}

fn extract_value(xml: &str, tag: &str) -> Option<String> {
    let start_tag = format!("<tds:{}>", tag);
    let end_tag = format!("</tds:{}>", tag);
    
    if let Some(start) = xml.find(&start_tag) {
        if let Some(end) = xml[start..].find(&end_tag) {
            let value_start = start + start_tag.len();
            let value_end = start + end;
            return Some(xml[value_start..value_end].trim().to_string());
        }
    }
    None
}
