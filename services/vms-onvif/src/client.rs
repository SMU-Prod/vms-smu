//! Cliente ONVIF com HTTP Digest Authentication
//! Implementação completa para comunicação SOAP com câmeras ONVIF

use anyhow::Result;
use reqwest::{Client, StatusCode, header};
use std::sync::Mutex;
use thiserror::Error;
use tracing::{debug, info, warn};
use url::Url;

use crate::digest_auth::{DigestChallenge, build_digest_authorization};

#[derive(Error, Debug)]
pub enum OnvifError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),
    
    #[error("Missing WWW-Authenticate digest header on 401")]
    MissingDigestHeader,
    
    #[error("Could not parse digest challenge")]
    ParseDigest,
    
    #[error("Unexpected status: {0}")]
    UnexpectedStatus(StatusCode),
    
    #[error("SOAP fault: {0}")]
    SoapFault(String),
}

/// Cliente ONVIF com suporte a HTTP Digest
pub struct OnvifClient {
    http: Client,
    base: Url,
    username: String,
    password: String,
    nc: Mutex<u32>, // Nonce count
}

impl OnvifClient {
    /// Cria novo cliente ONVIF
    pub fn new(base: &str, username: &str, password: &str) -> Result<Self, OnvifError> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .danger_accept_invalid_certs(true) // Para câmeras com cert auto-assinado
            .build()?;

        Ok(Self {
            http: client,
            base: Url::parse(base)?,
            username: username.to_string(),
            password: password.to_string(),
            nc: Mutex::new(1),
        })
    }

    /// Envia requisição SOAP com HTTP Digest authentication
    /// Usa SOAP 1.2 (application/soap+xml) para compatibilidade com Tapo C100
    pub async fn soap_post_digest(
        &self,
        service_path: &str,   // "/onvif/device_service"
        soap_action: Option<&str>,
        body_xml: &str,
    ) -> Result<String, OnvifError> {
        let url = self.base.join(service_path)?;
        let method = "POST";

        debug!("SOAP request to: {}", url);

        // Primeira tentativa sem autenticação
        // SOAP 1.2 usa application/soap+xml
        let mut req = self.http
            .post(url.clone())
            .header(header::CONTENT_TYPE, "application/soap+xml; charset=utf-8")
            .body(body_xml.to_string());

        if let Some(action) = soap_action {
            req = req.header("SOAPAction", action);
        }

        let resp = req.send().await?;
        let status = resp.status();

        // Se não for 401, retorna direto
        if status != StatusCode::UNAUTHORIZED {
            if status.is_success() {
                let text = resp.text().await?;
                debug!("SOAP response OK ({} bytes)", text.len());
                return Ok(text);
            }
            let text = resp.text().await?;
            warn!("SOAP error {}: {}", status, &text[..text.len().min(200)]);
            return Err(OnvifError::UnexpectedStatus(status));
        }

        // 401 - Extrair desafio Digest
        info!("Got 401, performing Digest authentication");
        
        let www_auth = resp.headers()
            .get(header::WWW_AUTHENTICATE)
            .and_then(|v| v.to_str().ok())
            .ok_or(OnvifError::MissingDigestHeader)?;

        debug!("WWW-Authenticate: {}", www_auth);

        let challenge = DigestChallenge::from_header(www_auth)
            .ok_or(OnvifError::ParseDigest)?;

        // Incrementa nonce count
        let mut nc_guard = self.nc.lock().unwrap();
        let nc = *nc_guard;
        *nc_guard = nc.saturating_add(1);
        drop(nc_guard);

        // URI para digest é path + query
        let uri = match url.query() {
            Some(q) => format!("{}?{}", url.path(), q),
            None => url.path().to_string(),
        };

        // Gera header Authorization
        let auth = build_digest_authorization(
            &challenge,
            method,
            &uri,
            &self.username,
            &self.password,
            nc,
        );

        debug!("Authorization: {}", &auth[..auth.len().min(100)]);

        // Segunda tentativa com autenticação
        // SOAP 1.2 usa application/soap+xml
        let mut req2 = self.http
            .post(url)
            .header(header::CONTENT_TYPE, "application/soap+xml; charset=utf-8")
            .header(header::AUTHORIZATION, auth)
            .body(body_xml.to_string());

        if let Some(action) = soap_action {
            req2 = req2.header("SOAPAction", action);
        }

        let resp2 = req2.send().await?;
        let status2 = resp2.status();
        let text = resp2.text().await?;

        if !status2.is_success() {
            warn!("SOAP auth failed {}: {}", status2, &text[..text.len().min(200)]);
            return Err(OnvifError::UnexpectedStatus(status2));
        }

        info!("SOAP authenticated successfully");
        debug!("Response ({} bytes)", text.len());
        
        Ok(text)
    }
}
