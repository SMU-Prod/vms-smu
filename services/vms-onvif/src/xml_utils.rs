//! Utilitários para parsing de XML ONVIF
//! Implementação manual sem dependências pesadas

/// Extrai texto da primeira tag encontrada (ignorando namespace)
pub fn extract_first_tag_text(xml: &str, tag_local: &str) -> Option<String> {
    // Procura por <prefix:tag> ou <tag>
    let patterns = vec![
        format!("<{}:", tag_local),
        format!("<{}>", tag_local),
        format!("<tt:{}>", tag_local),
        format!("<tds:{}>", tag_local),
        format!("<trt:{}>", tag_local),
    ];

    for start_pattern in &patterns {
        if let Some(start_pos) = xml.find(start_pattern) {
            // Encontra o fim da tag de abertura
            if let Some(tag_end) = xml[start_pos..].find('>') {
                let content_start = start_pos + tag_end + 1;
                
                // Procura pela tag de fechamento
                let end_patterns = vec![
                    format!("</{}>", tag_local),
                    format!("</{}:", tag_local),
                    format!("</tt:{}>", tag_local),
                    format!("</tds:{}>", tag_local),
                    format!("</trt:{}>", tag_local),
                ];
                
                for end_pattern in &end_patterns {
                    if let Some(end_pos) = xml[content_start..].find(end_pattern) {
                        let text = &xml[content_start..content_start + end_pos];
                        return Some(text.trim().to_string());
                    }
                }
            }
        }
    }
    
    None
}

/// Extrai atributo de uma tag XML
pub fn extract_attribute(xml: &str, tag: &str, attr: &str) -> Option<String> {
    // Procura pela tag
    if let Some(tag_start) = xml.find(&format!("<{}", tag)) {
        if let Some(tag_end) = xml[tag_start..].find('>') {
            let tag_content = &xml[tag_start..tag_start + tag_end];
            
            // Procura pelo atributo
            let pattern = format!("{}=\"", attr);
            if let Some(attr_start) = tag_content.find(&pattern) {
                let value_start = attr_start + pattern.len();
                if let Some(quote_end) = tag_content[value_start..].find('"') {
                    return Some(tag_content[value_start..value_start + quote_end].to_string());
                }
            }
        }
    }
    None
}

/// Extrai todas as ocorrências de uma tag
pub fn extract_all_tags(xml: &str, tag_local: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut search_from = 0;

    loop {
        let remaining = &xml[search_from..];
        if let Some(tag_xml) = extract_tag_block(remaining, tag_local) {
            let tag_start = remaining.find(&tag_xml).unwrap();
            results.push(tag_xml.clone());
            search_from += tag_start + tag_xml.len();
        } else {
            break;
        }
    }

    results
}

/// Extrai bloco XML completo de uma tag (incluindo a tag)
fn extract_tag_block(xml: &str, tag_local: &str) -> Option<String> {
    // Procura início da tag
    let start_pattern = format!("<{}", tag_local);
    if let Some(start_pos) = xml.find(&start_pattern) {
        // Encontra fim da tag de fechamento
        let end_pattern = format!("</{}>", tag_local);
        if let Some(end_pos) = xml[start_pos..].find(&end_pattern) {
            let block_end = start_pos + end_pos + end_pattern.len();
            return Some(xml[start_pos..block_end].to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_first_tag_text() {
        let xml = r#"<root><tt:XAddr>http://192.168.1.1/onvif</tt:XAddr></root>"#;
        let result = extract_first_tag_text(xml, "XAddr");
        assert_eq!(result, Some("http://192.168.1.1/onvif".to_string()));
    }

    #[test]
    fn test_extract_attribute() {
        let xml = r#"<Profile token="profile_1" fixed="true">content</Profile>"#;
        let result = extract_attribute(xml, "Profile", "token");
        assert_eq!(result, Some("profile_1".to_string()));
    }

    #[test]
    fn test_extract_all_tags() {
        let xml = r#"<root><item>1</item><item>2</item><item>3</item></root>"#;
        let results = extract_all_tags(xml, "item");
        assert_eq!(results.len(), 3);
    }
}
