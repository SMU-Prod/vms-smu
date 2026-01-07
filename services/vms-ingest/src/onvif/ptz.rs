//! ONVIF PTZ (Pan-Tilt-Zoom) Control

use super::{auth, OnvifClient};
use anyhow::{Context, Result};
use tracing::debug;

/// PTZ absolute move
pub async fn absolute_move(
    client: &OnvifClient,
    profile_token: &str,
    x: f32,
    y: f32,
    zoom: f32,
) -> Result<()> {
    debug!("📡 PTZ absolute move: x={}, y={}, zoom={}", x, y, zoom);

    let ptz_url = get_ptz_service_url(client).await?;

    let body = format!(
        r#"<tptz:AbsoluteMove xmlns:tptz="http://www.onvif.org/ver20/ptz/wsdl">
            <tptz:ProfileToken>{}</tptz:ProfileToken>
            <tptz:Position>
                <tt:PanTilt x="{}" y="{}" xmlns:tt="http://www.onvif.org/ver10/schema"/>
                <tt:Zoom x="{}" xmlns:tt="http://www.onvif.org/ver10/schema"/>
            </tptz:Position>
        </tptz:AbsoluteMove>"#,
        profile_token, x, y, zoom
    );

    let soap_request = auth::create_authenticated_soap_request(
        &client.username,
        &client.password,
        &body,
    );

    client
        .http_client
        .post(&ptz_url)
        .header("Content-Type", "application/soap+xml; charset=utf-8")
        .body(soap_request)
        .send()
        .await
        .context("Failed to send AbsoluteMove request")?;

    Ok(())
}

/// PTZ continuous move
pub async fn continuous_move(
    client: &OnvifClient,
    profile_token: &str,
    x: f32,
    y: f32,
    zoom: f32,
) -> Result<()> {
    debug!("📡 PTZ continuous move: x={}, y={}, zoom={}", x, y, zoom);

    let ptz_url = get_ptz_service_url(client).await?;

    let body = format!(
        r#"<tptz:ContinuousMove xmlns:tptz="http://www.onvif.org/ver20/ptz/wsdl">
            <tptz:ProfileToken>{}</tptz:ProfileToken>
            <tptz:Velocity>
                <tt:PanTilt x="{}" y="{}" xmlns:tt="http://www.onvif.org/ver10/schema"/>
                <tt:Zoom x="{}" xmlns:tt="http://www.onvif.org/ver10/schema"/>
            </tptz:Velocity>
        </tptz:ContinuousMove>"#,
        profile_token, x, y, zoom
    );

    let soap_request = auth::create_authenticated_soap_request(
        &client.username,
        &client.password,
        &body,
    );

    client
        .http_client
        .post(&ptz_url)
        .header("Content-Type", "application/soap+xml; charset=utf-8")
        .body(soap_request)
        .send()
        .await
        .context("Failed to send ContinuousMove request")?;

    Ok(())
}

/// PTZ stop
pub async fn stop(client: &OnvifClient, profile_token: &str) -> Result<()> {
    debug!("📡 PTZ stop");

    let ptz_url = get_ptz_service_url(client).await?;

    let body = format!(
        r#"<tptz:Stop xmlns:tptz="http://www.onvif.org/ver20/ptz/wsdl">
            <tptz:ProfileToken>{}</tptz:ProfileToken>
            <tptz:PanTilt>true</tptz:PanTilt>
            <tptz:Zoom>true</tptz:Zoom>
        </tptz:Stop>"#,
        profile_token
    );

    let soap_request = auth::create_authenticated_soap_request(
        &client.username,
        &client.password,
        &body,
    );

    client
        .http_client
        .post(&ptz_url)
        .header("Content-Type", "application/soap+xml; charset=utf-8")
        .body(soap_request)
        .send()
        .await
        .context("Failed to send Stop request")?;

    Ok(())
}

/// PTZ goto preset
pub async fn goto_preset(
    client: &OnvifClient,
    profile_token: &str,
    preset_token: &str,
) -> Result<()> {
    debug!("📡 PTZ goto preset: {}", preset_token);

    let ptz_url = get_ptz_service_url(client).await?;

    let body = format!(
        r#"<tptz:GotoPreset xmlns:tptz="http://www.onvif.org/ver20/ptz/wsdl">
            <tptz:ProfileToken>{}</tptz:ProfileToken>
            <tptz:PresetToken>{}</tptz:PresetToken>
        </tptz:GotoPreset>"#,
        profile_token, preset_token
    );

    let soap_request = auth::create_authenticated_soap_request(
        &client.username,
        &client.password,
        &body,
    );

    client
        .http_client
        .post(&ptz_url)
        .header("Content-Type", "application/soap+xml; charset=utf-8")
        .body(soap_request)
        .send()
        .await
        .context("Failed to send GotoPreset request")?;

    Ok(())
}

/// Get PTZ service URL
async fn get_ptz_service_url(client: &OnvifClient) -> Result<String> {
    // Tentar endpoint padrão primeiro
    let base_url = client.device_url.trim_end_matches("/onvif/device_service");
    let ptz_url = format!("{}/onvif/ptz_service", base_url);

    // TODO: Se falhar, fazer GetCapabilities para obter URL real
    Ok(ptz_url)
}
