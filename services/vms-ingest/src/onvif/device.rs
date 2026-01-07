//! ONVIF Device Management
//! GetDeviceInformation, GetCapabilities, etc

use super::{auth, OnvifClient, OnvifDevice};
use anyhow::{Context, Result};
use tracing::debug;

/// Get device information (manufacturer, model, firmware, etc)
pub async fn get_device_information(client: &OnvifClient) -> Result<OnvifDevice> {
    debug!("📡 Getting device information from {}", client.device_url);

    let body = r#"<tds:GetDeviceInformation xmlns:tds="http://www.onvif.org/ver10/device/wsdl"/>"#;

    let soap_request = auth::create_authenticated_soap_request(
        &client.username,
        &client.password,
        body,
    );

    let response = client
        .http_client
        .post(&client.device_url)
        .header("Content-Type", "application/soap+xml; charset=utf-8")
        .body(soap_request)
        .send()
        .await
        .context("Failed to send GetDeviceInformation request")?;

    let response_text = response
        .text()
        .await
        .context("Failed to read response body")?;

    debug!("📥 Received device information response");

    parse_device_information(&response_text, &client.device_url)
}

/// Parse GetDeviceInformation response
fn parse_device_information(xml: &str, device_url: &str) -> Result<OnvifDevice> {
    let doc = roxmltree::Document::parse(xml)
        .context("Failed to parse GetDeviceInformation response")?;

    // Extract manufacturer
    let manufacturer = doc
        .descendants()
        .find(|n| n.tag_name().name() == "Manufacturer")
        .and_then(|n| n.text())
        .map(|s| s.to_string());

    // Extract model
    let model = doc
        .descendants()
        .find(|n| n.tag_name().name() == "Model")
        .and_then(|n| n.text())
        .map(|s| s.to_string());

    // Extract firmware version
    let firmware = doc
        .descendants()
        .find(|n| n.tag_name().name() == "FirmwareVersion")
        .and_then(|n| n.text())
        .map(|s| s.to_string());

    // Extract serial number (use as name if available)
    let name = doc
        .descendants()
        .find(|n| n.tag_name().name() == "SerialNumber")
        .and_then(|n| n.text())
        .map(|s| s.to_string())
        .or_else(|| model.clone());

    // Extract IP from URL
    let ip = device_url
        .split("//")
        .nth(1)
        .and_then(|s| s.split(':').next())
        .unwrap_or("unknown")
        .to_string();

    // Extract port from URL
    let port = device_url
        .split(':')
        .nth(2)
        .and_then(|s| s.split('/').next())
        .and_then(|s| s.parse().ok())
        .unwrap_or(80);

    Ok(OnvifDevice {
        uuid: uuid::Uuid::new_v4().to_string(),
        ip,
        port,
        service_url: device_url.to_string(),
        name,
        manufacturer,
        model,
        firmware,
    })
}

/// Get device capabilities
pub async fn get_capabilities(client: &OnvifClient) -> Result<String> {
    debug!("📡 Getting device capabilities from {}", client.device_url);

    let body = r#"<tds:GetCapabilities xmlns:tds="http://www.onvif.org/ver10/device/wsdl">
        <tds:Category>All</tds:Category>
    </tds:GetCapabilities>"#;

    let soap_request = auth::create_authenticated_soap_request(
        &client.username,
        &client.password,
        body,
    );

    let response = client
        .http_client
        .post(&client.device_url)
        .header("Content-Type", "application/soap+xml; charset=utf-8")
        .body(soap_request)
        .send()
        .await
        .context("Failed to send GetCapabilities request")?;

    let response_text = response
        .text()
        .await
        .context("Failed to read capabilities response")?;

    Ok(response_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_device_information() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope">
    <s:Body>
        <tds:GetDeviceInformationResponse xmlns:tds="http://www.onvif.org/ver10/device/wsdl">
            <tds:Manufacturer>Hikvision</tds:Manufacturer>
            <tds:Model>DS-2CD2143G0-I</tds:Model>
            <tds:FirmwareVersion>V5.6.3</tds:FirmwareVersion>
            <tds:SerialNumber>DS2CD2143G0I20190101AAWRC12345678</tds:SerialNumber>
            <tds:HardwareId>88</tds:HardwareId>
        </tds:GetDeviceInformationResponse>
    </s:Body>
</s:Envelope>"#;

        let device = parse_device_information(xml, "http://192.168.1.100:80/onvif/device_service").unwrap();
        assert_eq!(device.manufacturer, Some("Hikvision".to_string()));
        assert_eq!(device.model, Some("DS-2CD2143G0-I".to_string()));
        assert_eq!(device.firmware, Some("V5.6.3".to_string()));
    }
}
