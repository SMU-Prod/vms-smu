//! ONVIF WS-UsernameToken Authentication
//! Implementa autenticação digest para SOAP requests

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use chrono::Utc;
use sha1::{Digest, Sha1};

/// Gerar WS-Security UsernameToken header para ONVIF
pub fn generate_wsse_header(username: &str, password: &str) -> String {
    // Timestamp UTC
    let created = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

    // Nonce (random bytes)
    let nonce_bytes: [u8; 16] = rand::random();
    let nonce_b64 = BASE64.encode(nonce_bytes);

    // Password Digest = Base64( SHA-1( Nonce + Created + Password ) )
    // ONVIF usa SHA-1 apesar de deprecated
    let mut hasher = Sha1::new();
    hasher.update(&nonce_bytes);
    hasher.update(created.as_bytes());
    hasher.update(password.as_bytes());
    let digest = hasher.finalize();
    let digest_b64 = BASE64.encode(digest);

    format!(
        r#"<Security s:mustUnderstand="1" xmlns="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd">
        <UsernameToken>
            <Username>{}</Username>
            <Password Type="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest">{}</Password>
            <Nonce EncodingType="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-soap-message-security-1.0#Base64Binary">{}</Nonce>
            <Created xmlns="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd">{}</Created>
        </UsernameToken>
    </Security>"#,
        username, digest_b64, nonce_b64, created
    )
}

/// Template SOAP com autenticação
pub fn create_authenticated_soap_request(
    username: &str,
    password: &str,
    body: &str,
) -> String {
    let wsse_header = generate_wsse_header(username, password);

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope">
    <s:Header>
        {}
    </s:Header>
    <s:Body>
        {}
    </s:Body>
</s:Envelope>"#,
        wsse_header, body
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_wsse_header() {
        let header = generate_wsse_header("admin", "password123");

        // Verificar que contém elementos necessários
        assert!(header.contains("<Username>admin</Username>"));
        assert!(header.contains("<Password Type="));
        assert!(header.contains("<Nonce EncodingType="));
        assert!(header.contains("<Created xmlns="));
    }

    #[test]
    fn test_create_authenticated_soap_request() {
        let body = "<tds:GetDeviceInformation xmlns:tds=\"http://www.onvif.org/ver10/device/wsdl\"/>";
        let request = create_authenticated_soap_request("admin", "pass", body);

        assert!(request.contains("<?xml version"));
        assert!(request.contains("<s:Envelope"));
        assert!(request.contains("<Security"));
        assert!(request.contains("<tds:GetDeviceInformation"));
    }
}
