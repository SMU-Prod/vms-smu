//! ONVIF Media Profile Management
//! GetProfiles, GetStreamUri

use super::{auth, MediaProfile, OnvifClient};
use anyhow::{Context, Result};
use tracing::debug;

/// Get all media profiles
pub async fn get_profiles(client: &OnvifClient) -> Result<Vec<MediaProfile>> {
    debug!("📡 Getting media profiles from {}", client.device_url);

    // Primeiro precisamos obter a URL do serviço Media
    let media_url = get_media_service_url(client).await?;

    let body = r#"<trt:GetProfiles xmlns:trt="http://www.onvif.org/ver10/media/wsdl"/>"#;

    let soap_request = auth::create_authenticated_soap_request(
        &client.username,
        &client.password,
        body,
    );

    let response = client
        .http_client
        .post(&media_url)
        .header("Content-Type", "application/soap+xml; charset=utf-8")
        .body(soap_request)
        .send()
        .await
        .context("Failed to send GetProfiles request")?;

    let response_text = response
        .text()
        .await
        .context("Failed to read profiles response")?;

    parse_profiles(&response_text)
}

/// Get RTSP stream URI for a profile
pub async fn get_stream_uri(client: &OnvifClient, profile_token: &str) -> Result<String> {
    debug!("📡 Getting stream URI for profile {}", profile_token);

    let media_url = get_media_service_url(client).await?;

    let body = format!(
        r#"<trt:GetStreamUri xmlns:trt="http://www.onvif.org/ver10/media/wsdl">
            <trt:StreamSetup>
                <tt:Stream xmlns:tt="http://www.onvif.org/ver10/schema">RTP-Unicast</tt:Stream>
                <tt:Transport xmlns:tt="http://www.onvif.org/ver10/schema">
                    <tt:Protocol>RTSP</tt:Protocol>
                </tt:Transport>
            </trt:StreamSetup>
            <trt:ProfileToken>{}</trt:ProfileToken>
        </trt:GetStreamUri>"#,
        profile_token
    );

    let soap_request = auth::create_authenticated_soap_request(
        &client.username,
        &client.password,
        &body,
    );

    let response = client
        .http_client
        .post(&media_url)
        .header("Content-Type", "application/soap+xml; charset=utf-8")
        .body(soap_request)
        .send()
        .await
        .context("Failed to send GetStreamUri request")?;

    let response_text = response
        .text()
        .await
        .context("Failed to read stream URI response")?;

    parse_stream_uri(&response_text)
}

/// Get Media service URL from capabilities
async fn get_media_service_url(client: &OnvifClient) -> Result<String> {
    // Tentar endpoint padrão primeiro
    let base_url = client.device_url.trim_end_matches("/onvif/device_service");
    let media_url = format!("{}/onvif/media_service", base_url);

    // TODO: Se falhar, fazer GetCapabilities para obter URL real
    Ok(media_url)
}

/// Parse GetProfiles response
fn parse_profiles(xml: &str) -> Result<Vec<MediaProfile>> {
    let doc = roxmltree::Document::parse(xml)
        .context("Failed to parse GetProfiles response")?;

    let mut profiles = Vec::new();

    for profile_node in doc.descendants().filter(|n| n.tag_name().name() == "Profiles") {
        // Extract profile token
        let token = profile_node
            .attribute("token")
            .ok_or_else(|| anyhow::anyhow!("Profile missing token"))?
            .to_string();

        // Extract profile name
        let name = profile_node
            .descendants()
            .find(|n| n.tag_name().name() == "Name")
            .and_then(|n| n.text())
            .unwrap_or("Unknown")
            .to_string();

        // Extract video encoder configuration
        let width = profile_node
            .descendants()
            .find(|n| n.tag_name().name() == "Width")
            .and_then(|n| n.text())
            .and_then(|s| s.parse().ok())
            .unwrap_or(1920);

        let height = profile_node
            .descendants()
            .find(|n| n.tag_name().name() == "Height")
            .and_then(|n| n.text())
            .and_then(|s| s.parse().ok())
            .unwrap_or(1080);

        let codec = profile_node
            .descendants()
            .find(|n| n.tag_name().name() == "Encoding")
            .and_then(|n| n.text())
            .unwrap_or("H264")
            .to_string();

        let framerate = profile_node
            .descendants()
            .find(|n| n.tag_name().name() == "FrameRateLimit")
            .and_then(|n| n.text())
            .and_then(|s| s.parse().ok())
            .unwrap_or(30.0);

        let bitrate = profile_node
            .descendants()
            .find(|n| n.tag_name().name() == "BitrateLimit")
            .and_then(|n| n.text())
            .and_then(|s| s.parse().ok())
            .unwrap_or(4096);

        profiles.push(MediaProfile {
            token,
            name,
            rtsp_uri: String::new(), // Será preenchido com GetStreamUri
            resolution: (width, height),
            video_codec: codec,
            framerate,
            bitrate,
        });
    }

    if profiles.is_empty() {
        return Err(anyhow::anyhow!("No profiles found in response"));
    }

    Ok(profiles)
}

/// Parse GetStreamUri response
fn parse_stream_uri(xml: &str) -> Result<String> {
    let doc = roxmltree::Document::parse(xml)
        .context("Failed to parse GetStreamUri response")?;

    let uri = doc
        .descendants()
        .find(|n| n.tag_name().name() == "Uri")
        .and_then(|n| n.text())
        .ok_or_else(|| anyhow::anyhow!("Uri not found in response"))?
        .to_string();

    Ok(uri)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_stream_uri() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope">
    <s:Body>
        <trt:GetStreamUriResponse xmlns:trt="http://www.onvif.org/ver10/media/wsdl">
            <trt:MediaUri>
                <tt:Uri xmlns:tt="http://www.onvif.org/ver10/schema">rtsp://192.168.1.100:554/Streaming/Channels/101</tt:Uri>
                <tt:InvalidAfterConnect xmlns:tt="http://www.onvif.org/ver10/schema">false</tt:InvalidAfterConnect>
                <tt:InvalidAfterReboot xmlns:tt="http://www.onvif.org/ver10/schema">false</tt:InvalidAfterReboot>
                <tt:Timeout xmlns:tt="http://www.onvif.org/ver10/schema">PT0S</tt:Timeout>
            </trt:MediaUri>
        </trt:GetStreamUriResponse>
    </s:Body>
</s:Envelope>"#;

        let uri = parse_stream_uri(xml).unwrap();
        assert!(uri.starts_with("rtsp://"));
        assert!(uri.contains("192.168.1.100"));
    }

    #[test]
    fn test_parse_profiles() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope">
    <s:Body>
        <trt:GetProfilesResponse xmlns:trt="http://www.onvif.org/ver10/media/wsdl">
            <trt:Profiles token="Profile_1" fixed="true">
                <tt:Name xmlns:tt="http://www.onvif.org/ver10/schema">MainStream</tt:Name>
                <tt:VideoEncoderConfiguration xmlns:tt="http://www.onvif.org/ver10/schema">
                    <tt:Encoding>H264</tt:Encoding>
                    <tt:Resolution>
                        <tt:Width>1920</tt:Width>
                        <tt:Height>1080</tt:Height>
                    </tt:Resolution>
                    <tt:RateControl>
                        <tt:FrameRateLimit>30</tt:FrameRateLimit>
                        <tt:BitrateLimit>4096</tt:BitrateLimit>
                    </tt:RateControl>
                </tt:VideoEncoderConfiguration>
            </trt:Profiles>
        </trt:GetProfilesResponse>
    </s:Body>
</s:Envelope>"#;

        let profiles = parse_profiles(xml).unwrap();
        assert_eq!(profiles.len(), 1);
        assert_eq!(profiles[0].token, "Profile_1");
        assert_eq!(profiles[0].name, "MainStream");
        assert_eq!(profiles[0].resolution, (1920, 1080));
        assert_eq!(profiles[0].video_codec, "H264");
    }
}
