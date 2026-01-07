//! ONVIF Device - Implementação Completa
//! Conexão e comunicação com dispositivo ONVIF usando SOAP 1.1 com WS-Addressing

use anyhow::{anyhow, Context, Result};
use tracing::{debug, info};

use crate::camera::CameraProfile;
use crate::client::OnvifClient;
use crate::xml_utils;

/// Representa uma conexão ativa com um dispositivo ONVIF
pub struct OnvifDevice {
    /// Cliente ONVIF
    client: OnvifClient,
    /// URL do serviço de mídia
    media_service_url: Option<String>,
}

/// Informações do dispositivo ONVIF
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub manufacturer: String,
    pub model: String,
    pub firmware_version: String,
    pub serial_number: String,
    pub hardware_id: String,
}

/// Capacidades do dispositivo
#[derive(Debug, Clone)]
pub struct DeviceCapabilities {
    pub analytics: bool,
    pub device: bool,
    pub events: bool,
    pub imaging: bool,
    pub media: bool,
    pub ptz: bool,
}

impl OnvifDevice {
    /// Cria nova conexão com dispositivo ONVIF
    pub fn new(base_url: &str, username: &str, password: &str) -> Result<Self> {
        let client = OnvifClient::new(base_url, username, password)?;

        Ok(Self {
            client,
            media_service_url: None,
        })
    }

    /// Conecta ao dispositivo ONVIF e obtém capacidades
    pub async fn connect(&mut self) -> Result<()> {
        info!("📡 Conectando ao dispositivo ONVIF");

        // Obter capacidades do dispositivo
        let capabilities = self.get_capabilities().await?;
        
        info!("✅ Conectado com sucesso");
        debug!("Capacidades: {:?}", capabilities);

        Ok(())
    }

    /// Obtém capacidades do dispositivo (GetCapabilities)
    /// Usa SOAP 1.1 com WS-Addressing para compatibilidade com Tapo C100
    pub async fn get_capabilities(&mut self) -> Result<DeviceCapabilities> {
        let soap_body = r#"<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope" xmlns:a="http://www.w3.org/2005/08/addressing" xmlns:tds="http://www.onvif.org/ver10/device/wsdl">
  <s:Header>
    <a:Action s:mustUnderstand="1">http://www.onvif.org/ver10/device/wsdl/GetCapabilities</a:Action>
  </s:Header>
  <s:Body>
    <tds:GetCapabilities>
      <tds:Category>All</tds:Category>
    </tds:GetCapabilities>
  </s:Body>
</s:Envelope>"#;

        let response = self.client.soap_post_digest("/onvif/device_service", None, soap_body).await?;
        
        // Parse básico
        let has_media = response.contains("Media");
        let has_ptz = response.contains("PTZ");
        let has_analytics = response.contains("Analytics");
        
        // Extrair URL do serviço de mídia
        if let Some(media_url) = xml_utils::extract_first_tag_text(&response, "XAddr") {
            self.media_service_url = Some(media_url.clone());
            info!("📹 Media service URL: {}", media_url);
        }

        Ok(DeviceCapabilities {
            analytics: has_analytics,
            device: true,
            events: response.contains("Events"),
            imaging: response.contains("Imaging"),
            media: has_media,
            ptz: has_ptz,
        })
    }

    /// Obtém informações do dispositivo (GetDeviceInformation)
    /// Usa SOAP 1.1 com WS-Addressing para compatibilidade com Tapo C100
    pub async fn get_device_info(&self) -> Result<DeviceInfo> {
        let soap_body = r#"<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope" xmlns:a="http://www.w3.org/2005/08/addressing" xmlns:tds="http://www.onvif.org/ver10/device/wsdl">
  <s:Header>
    <a:Action s:mustUnderstand="1">http://www.onvif.org/ver10/device/wsdl/GetDeviceInformation</a:Action>
  </s:Header>
  <s:Body>
    <tds:GetDeviceInformation/>
  </s:Body>
</s:Envelope>"#;

        let response = self.client.soap_post_digest("/onvif/device_service", None, soap_body).await?;

        Ok(DeviceInfo {
            manufacturer: xml_utils::extract_first_tag_text(&response, "Manufacturer").unwrap_or_else(|| "Unknown".to_string()),
            model: xml_utils::extract_first_tag_text(&response, "Model").unwrap_or_else(|| "Unknown".to_string()),
            firmware_version: xml_utils::extract_first_tag_text(&response, "FirmwareVersion").unwrap_or_else(|| "Unknown".to_string()),
            serial_number: xml_utils::extract_first_tag_text(&response, "SerialNumber").unwrap_or_else(|| "Unknown".to_string()),
            hardware_id: xml_utils::extract_first_tag_text(&response, "HardwareId").unwrap_or_else(|| "Unknown".to_string()),
        })
    }

    /// Lista os profiles de mídia disponíveis (GetProfiles)
    /// Usa SOAP 1.1 com WS-Addressing para compatibilidade com Tapo C100
    pub async fn get_profiles(&self) -> Result<Vec<CameraProfile>> {
        // Garantir que temos a URL do serviço de mídia
        if self.media_service_url.is_none() {
            return Err(anyhow!("Media service URL not available. Call connect() first"));
        }

        let soap_body = r#"<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope" xmlns:a="http://www.w3.org/2005/08/addressing" xmlns:trt="http://www.onvif.org/ver10/media/wsdl">
  <s:Header>
    <a:Action s:mustUnderstand="1">http://www.onvif.org/ver10/media/wsdl/GetProfiles</a:Action>
  </s:Header>
  <s:Body>
    <trt:GetProfiles/>
  </s:Body>
</s:Envelope>"#;

        // Extrair path da media service URL
        let media_path = self.extract_path_from_url(self.media_service_url.as_ref().unwrap())?;
        
        let response = self.client.soap_post_digest(&media_path, None, soap_body).await?;
        
        let mut profiles = Vec::new();
        
        // Parse profiles - procurar por tags <trt:Profiles>
        let mut pos = 0;
        while let Some(profile_start) = response[pos..].find("<trt:Profiles") {
            let abs_start = pos + profile_start;
            
            if let Some(profile_end) = response[abs_start..].find("</trt:Profiles>") {
                let profile_xml = &response[abs_start..abs_start + profile_end + 15];
                
                // Extrair token do atributo
                if let Some(token) = xml_utils::extract_attribute(profile_xml, "trt:Profiles", "token") {
                    let name = xml_utils::extract_first_tag_text(profile_xml, "Name").unwrap_or_else(|| "Default".to_string());
                    let encoding = xml_utils::extract_first_tag_text(profile_xml, "Encoding").unwrap_or_else(|| "H264".to_string());
                    
                    let width = xml_utils::extract_first_tag_text(profile_xml, "Width")
                        .and_then(|w| w.parse().ok())
                        .unwrap_or(1920);
                    let height = xml_utils::extract_first_tag_text(profile_xml, "Height")
                        .and_then(|h| h.parse().ok())
                        .unwrap_or(1080);
                    
                    profiles.push(CameraProfile {
                        token,
                        name,
                        video_encoding: encoding,
                        resolution: (width, height),
                        framerate: 25.0,
                    });
                }
                
                pos = abs_start + profile_end + 15;
            } else {
                break;
            }
        }

        info!("📹 Found {} media profiles", profiles.len());
        Ok(profiles)
    }

    /// Obtém URL RTSP do stream (GetStreamUri)
    /// Usa SOAP 1.1 com WS-Addressing para compatibilidade com Tapo C100
    pub async fn get_stream_uri(&self, profile_token: &str) -> Result<String> {
        if self.media_service_url.is_none() {
            return Err(anyhow!("Media service URL not available. Call connect() first"));
        }

        let soap_body = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope" xmlns:a="http://www.w3.org/2005/08/addressing" xmlns:trt="http://www.onvif.org/ver10/media/wsdl" xmlns:tt="http://www.onvif.org/ver10/schema">
  <s:Header>
    <a:Action s:mustUnderstand="1">http://www.onvif.org/ver10/media/wsdl/GetStreamUri</a:Action>
  </s:Header>
  <s:Body>
    <trt:GetStreamUri>
      <trt:StreamSetup>
        <tt:Stream>RTP-Unicast</tt:Stream>
        <tt:Transport>
          <tt:Protocol>RTSP</tt:Protocol>
        </tt:Transport>
      </trt:StreamSetup>
      <trt:ProfileToken>{}</trt:ProfileToken>
    </trt:GetStreamUri>
  </s:Body>
</s:Envelope>"#, profile_token);

        let media_path = self.extract_path_from_url(self.media_service_url.as_ref().unwrap())?;
        
        let response = self.client.soap_post_digest(&media_path, None, &soap_body).await?;
        
        // Extrair URI
        let uri = xml_utils::extract_first_tag_text(&response, "Uri")
            .ok_or_else(|| anyhow!("Stream URI not found in response"))?;

        info!("📺 Stream URI: {}", uri);
        Ok(uri)
    }

    /// Extrai path de uma URL completa
    fn extract_path_from_url(&self, url: &str) -> Result<String> {
        use url::Url;
        let parsed = Url::parse(url).context("Failed to parse media service URL")?;
        Ok(parsed.path().to_string())
    }

    /// Verifica se o dispositivo suporta PTZ
    pub async fn supports_ptz(&mut self) -> Result<bool> {
        let capabilities = self.get_capabilities().await?;
        Ok(capabilities.ptz)
    }
}
