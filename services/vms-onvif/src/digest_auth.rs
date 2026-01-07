//! HTTP Digest Authentication para ONVIF
//! Implementação baseada em RFC 2617/7616

use rand::{RngCore, rngs::OsRng};
use std::collections::HashMap;

/// Calcula MD5 e retorna como string hexadecimal
pub fn md5_hex(s: &str) -> String {
    let digest = md5::compute(s.as_bytes());
    format!("{:x}", digest)
}

/// Gera cnonce aleatório
pub fn gen_cnonce() -> String {
    use base64::Engine;
    let mut b = [0u8; 16];
    OsRng.fill_bytes(&mut b);
    base64::engine::general_purpose::STANDARD_NO_PAD.encode(&b)
}

/// Parse do header WWW-Authenticate Digest
pub fn parse_www_authenticate_digest(header: &str) -> Option<HashMap<String, String>> {
    let h = header.trim();
    if !h.to_ascii_lowercase().starts_with("digest ") {
        return None;
    }
    
    let rest = &h[7..]; // depois de "Digest "
    let mut map = HashMap::new();

    for part in rest.split(',') {
        let part = part.trim();
        let mut it = part.splitn(2, '=');
        let k = it.next()?.trim().to_string();
        let mut v = it.next()?.trim().to_string();
        
        // Remove aspas
        if v.starts_with('"') && v.ends_with('"') && v.len() >= 2 {
            v = v[1..v.len()-1].to_string();
        }
        map.insert(k, v);
    }
    
    Some(map)
}

/// Desafio Digest extraído do WWW-Authenticate
#[derive(Clone, Debug)]
pub struct DigestChallenge {
    pub realm: String,
    pub nonce: String,
    pub qop: Option<String>,
    pub opaque: Option<String>,
    pub algorithm: Option<String>,
}

impl DigestChallenge {
    pub fn from_header(header: &str) -> Option<Self> {
        let map = parse_www_authenticate_digest(header)?;
        
        Some(Self {
            realm: map.get("realm").cloned().unwrap_or_default(),
            nonce: map.get("nonce").cloned().unwrap_or_default(),
            qop: map.get("qop").cloned(),
            opaque: map.get("opaque").cloned(),
            algorithm: map.get("algorithm").cloned(),
        })
    }
}

/// Constrói header Authorization Digest
pub fn build_digest_authorization(
    challenge: &DigestChallenge,
    method: &str,
    uri: &str,          // ex: "/onvif/device_service"
    username: &str,
    password: &str,
    nc: u32,
) -> String {
    let nc_str = format!("{:08x}", nc);
    let cnonce = gen_cnonce();

    // Se qop vier como "auth,auth-int", pega só "auth"
    let qop = challenge.qop.as_deref().unwrap_or("auth");
    let qop = qop.split(',').next().unwrap_or("auth").trim();

    // Cálculo MD5 conforme RFC 2617
    // HA1 = MD5(username:realm:password)
    let ha1 = md5_hex(&format!("{}:{}:{}", username, challenge.realm, password));
    
    // HA2 = MD5(method:uri)
    let ha2 = md5_hex(&format!("{}:{}", method, uri));
    
    // response = MD5(HA1:nonce:nc:cnonce:qop:HA2)
    let response = md5_hex(&format!(
        "{}:{}:{}:{}:{}:{}",
        ha1, challenge.nonce, nc_str, cnonce, qop, ha2
    ));

    // Monta header Authorization
    let mut hdr = format!(
        r#"Digest username="{}", realm="{}", nonce="{}", uri="{}", response="{}", qop={}, nc={}, cnonce="{}""#,
        username, challenge.realm, challenge.nonce, uri, response, qop, nc_str, cnonce
    );

    if let Some(op) = &challenge.opaque {
        hdr.push_str(&format!(r#", opaque="{}""#, op));
    }
    if let Some(algo) = &challenge.algorithm {
        hdr.push_str(&format!(r#", algorithm={}"#, algo));
    }

    hdr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_hex() {
        let result = md5_hex("test");
        assert_eq!(result.len(), 32); // MD5 é sempre 32 chars hex
    }

    #[test]
    fn test_gen_cnonce() {
        let cnonce = gen_cnonce();
        assert!(!cnonce.is_empty());
    }

    #[test]
    fn test_parse_www_authenticate() {
        let header = r#"Digest realm="ONVIF", nonce="abc123", qop="auth", opaque="xyz""#;
        let map = parse_www_authenticate_digest(header).unwrap();
        
        assert_eq!(map.get("realm").unwrap(), "ONVIF");
        assert_eq!(map.get("nonce").unwrap(), "abc123");
        assert_eq!(map.get("qop").unwrap(), "auth");
    }
}
