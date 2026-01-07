//! ONVIF WS-Discovery Implementation
//! Descobre câmeras ONVIF na rede local via multicast

use super::OnvifDevice;
use anyhow::{Context, Result};
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::time::Duration;
use tracing::{debug, info, warn};

const MULTICAST_ADDR: &str = "239.255.255.250:3702";
const DISCOVERY_TIMEOUT_MS: u64 = 5000;

/// Mensagem WS-Discovery Probe
const WS_DISCOVERY_PROBE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope" xmlns:a="http://schemas.xmlsoap.org/ws/2004/08/addressing">
    <s:Header>
        <a:Action s:mustUnderstand="1">http://schemas.xmlsoap.org/ws/2005/04/discovery/Probe</a:Action>
        <a:MessageID>uuid:__UUID__</a:MessageID>
        <a:ReplyTo>
            <a:Address>http://schemas.xmlsoap.org/ws/2004/08/addressing/role/anonymous</a:Address>
        </a:ReplyTo>
        <a:To s:mustUnderstand="1">urn:schemas-xmlsoap-org:ws:2005:04:discovery</a:To>
    </s:Header>
    <s:Body>
        <Probe xmlns="http://schemas.xmlsoap.org/ws/2005/04/discovery">
            <d:Types xmlns:d="http://schemas.xmlsoap.org/ws/2005/04/discovery" xmlns:dp0="http://www.onvif.org/ver10/network/wsdl">dp0:NetworkVideoTransmitter</d:Types>
        </Probe>
    </s:Body>
</s:Envelope>"#;

/// Descobrir dispositivos ONVIF na rede
pub async fn discover_devices() -> Result<Vec<OnvifDevice>> {
    info!("🔍 Starting ONVIF discovery...");

    let socket = UdpSocket::bind("0.0.0.0:0")
        .context("Failed to bind UDP socket for discovery")?;

    socket.set_read_timeout(Some(Duration::from_millis(DISCOVERY_TIMEOUT_MS)))
        .context("Failed to set socket timeout")?;

    socket.set_broadcast(true)
        .context("Failed to enable broadcast")?;

    // Gerar UUID para mensagem
    let uuid = uuid::Uuid::new_v4().to_string();
    let probe_message = WS_DISCOVERY_PROBE.replace("__UUID__", &uuid);

    // Enviar probe multicast
    let multicast_addr: SocketAddr = MULTICAST_ADDR.parse()?;
    socket.send_to(probe_message.as_bytes(), multicast_addr)
        .context("Failed to send discovery probe")?;

    debug!("📡 Sent WS-Discovery probe to {}", MULTICAST_ADDR);

    let mut devices = Vec::new();
    let mut buf = [0u8; 8192];
    let start_time = std::time::Instant::now();

    // Receber respostas
    while start_time.elapsed() < Duration::from_millis(DISCOVERY_TIMEOUT_MS) {
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                let response = String::from_utf8_lossy(&buf[..size]);
                debug!("📥 Received response from {}: {} bytes", addr, size);

                match parse_probe_match(&response, addr.ip()) {
                    Ok(device) => {
                        info!("✅ Found ONVIF device: {} at {}",
                            device.name.as_ref().unwrap_or(&"Unknown".to_string()),
                            device.ip
                        );
                        devices.push(device);
                    }
                    Err(e) => {
                        warn!("⚠️  Failed to parse response from {}: {}", addr, e);
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Timeout - esperado
                break;
            }
            Err(e) => {
                warn!("⚠️  Error receiving discovery response: {}", e);
            }
        }
    }

    info!("✅ Discovery complete: found {} devices", devices.len());
    Ok(devices)
}

/// Parse WS-Discovery ProbeMatch response
fn parse_probe_match(xml: &str, ip: IpAddr) -> Result<OnvifDevice> {
    let doc = roxmltree::Document::parse(xml)
        .context("Failed to parse XML response")?;

    // Extrair XAddrs (ONVIF service URL)
    let xaddrs = doc
        .descendants()
        .find(|n| n.tag_name().name() == "XAddrs")
        .and_then(|n| n.text())
        .ok_or_else(|| anyhow::anyhow!("XAddrs not found in response"))?;

    // Pegar primeira URL
    let service_url = xaddrs
        .split_whitespace()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No service URL in XAddrs"))?
        .to_string();

    // Extrair UUID do EndpointReference
    let uuid = doc
        .descendants()
        .find(|n| n.tag_name().name() == "Address")
        .and_then(|n| n.text())
        .and_then(|addr| addr.strip_prefix("urn:uuid:"))
        .unwrap_or("unknown")
        .to_string();

    // Extrair porta da URL
    let port = service_url
        .split(':')
        .nth(2)
        .and_then(|s| s.split('/').next())
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(80);

    Ok(OnvifDevice {
        uuid,
        ip: ip.to_string(),
        port,
        service_url,
        name: None,  // Será preenchido depois com GetDeviceInformation
        manufacturer: None,
        model: None,
        firmware: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_probe_match() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://www.w3.org/2003/05/soap-envelope">
    <SOAP-ENV:Body>
        <d:ProbeMatches xmlns:d="http://schemas.xmlsoap.org/ws/2005/04/discovery">
            <d:ProbeMatch>
                <a:EndpointReference xmlns:a="http://schemas.xmlsoap.org/ws/2004/08/addressing">
                    <a:Address>urn:uuid:12345678-1234-1234-1234-123456789012</a:Address>
                </a:EndpointReference>
                <d:XAddrs>http://192.168.1.100:80/onvif/device_service</d:XAddrs>
            </d:ProbeMatch>
        </d:ProbeMatches>
    </SOAP-ENV:Body>
</SOAP-ENV:Envelope>"#;

        let device = parse_probe_match(xml, "192.168.1.100".parse().unwrap()).unwrap();
        assert_eq!(device.ip, "192.168.1.100");
        assert_eq!(device.port, 80);
        assert!(device.service_url.contains("onvif/device_service"));
    }
}
