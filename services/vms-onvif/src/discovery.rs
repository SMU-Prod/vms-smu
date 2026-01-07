//! ONVIF WS-Discovery
//! Descobre câmeras ONVIF na rede local usando WS-Discovery

use anyhow::Result;
use std::time::Duration;
use tracing::{info, warn};

use crate::camera::Camera;

/// Serviço de descoberta ONVIF
pub struct OnvifDiscovery {
    /// Timeout para descoberta
    timeout: Duration,
}

impl OnvifDiscovery {
    /// Cria nova instância de discovery
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(5),
        }
    }

    /// Define timeout personalizado
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Descobre câmeras ONVIF na rede local
    /// 
    /// NOTA: WS-Discovery requer implementação UDP multicast
    /// Por enquanto retorna lista vazia - será implementado em fase futura
    pub async fn discover(&self, _timeout: Duration) -> Result<Vec<Camera>> {
        info!("🔍 WS-Discovery não implementado ainda");
        warn!("Use configuração manual de câmeras por enquanto");
        
        // TODO: Implementar WS-Discovery (SOAP over UDP multicast)
        // Referência: https://www.onvif.org/specs/core/ONVIF-Core-Specification.pdf
        
        Ok(Vec::new())
    }
    
    /// Descoberta com probe específico para NetworkVideoTransmitter
    pub async fn discover_nvt(&self) -> Result<Vec<Camera>> {
        self.discover(self.timeout).await
    }
}

impl Default for OnvifDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discovery_creates_instance() {
        let discovery = OnvifDiscovery::new();
        assert_eq!(discovery.timeout, Duration::from_secs(5));
    }
}
