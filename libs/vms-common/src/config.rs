//! Configuração do sistema

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuração global do VMS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmsConfig {
    /// Configuração de ingestão
    pub ingest: IngestConfig,

    /// Configuração de storage
    pub storage: StorageConfig,

    /// Configuração de streaming
    pub streaming: StreamingConfig,

    /// Configuração de IA
    pub ai: AiConfig,

    /// Configuração de observabilidade
    pub observability: ObservabilityConfig,
}

/// Configuração do serviço de ingestão
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestConfig {
    /// Número máximo de câmeras por instância
    pub max_cameras: usize,

    /// Timeout de conexão em segundos
    pub connection_timeout_secs: u64,

    /// Intervalo de reconexão em segundos
    pub reconnect_interval_secs: u64,

    /// Buffer size para frames
    pub frame_buffer_size: usize,
}

impl Default for IngestConfig {
    fn default() -> Self {
        Self {
            max_cameras: 100,
            connection_timeout_secs: 30,
            reconnect_interval_secs: 5,
            frame_buffer_size: 100,
        }
    }
}

/// Configuração de armazenamento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Diretório base para armazenamento
    pub base_path: PathBuf,

    /// Retenção em dias
    pub retention_days: u32,

    /// Habilitar compressão
    pub enable_compression: bool,

    /// Habilitar criptografia
    pub enable_encryption: bool,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            base_path: PathBuf::from("/storage/vms"),
            retention_days: 30,
            enable_compression: true,
            enable_encryption: false,
        }
    }
}

/// Configuração de streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingConfig {
    /// Porta WebRTC
    pub webrtc_port: u16,

    /// Porta SRT
    pub srt_port: u16,

    /// Número máximo de viewers simultâneos
    pub max_viewers: usize,

    /// Latência alvo em ms
    pub target_latency_ms: u32,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            webrtc_port: 8443,
            srt_port: 9000,
            max_viewers: 1000,
            target_latency_ms: 100,
        }
    }
}

/// Configuração de IA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// Habilitar pipeline de IA
    pub enabled: bool,

    /// Device para inferência (cuda, cpu, rocm, openvino)
    pub device: String,

    /// Número de workers
    pub num_workers: usize,

    /// Tamanho do batch
    pub batch_size: usize,

    /// Threshold de confiança
    pub confidence_threshold: f32,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            device: "cpu".to_string(),
            num_workers: 4,
            batch_size: 8,
            confidence_threshold: 0.5,
        }
    }
}

/// Configuração de observabilidade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    /// Endpoint do OpenTelemetry Collector
    pub otlp_endpoint: String,

    /// Habilitar métricas
    pub enable_metrics: bool,

    /// Habilitar traces
    pub enable_traces: bool,

    /// Habilitar logs estruturados
    pub enable_structured_logs: bool,

    /// Nível de log (trace, debug, info, warn, error)
    pub log_level: String,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            otlp_endpoint: "http://localhost:4317".to_string(),
            enable_metrics: true,
            enable_traces: true,
            enable_structured_logs: true,
            log_level: "info".to_string(),
        }
    }
}

impl Default for VmsConfig {
    fn default() -> Self {
        Self {
            ingest: IngestConfig::default(),
            storage: StorageConfig::default(),
            streaming: StreamingConfig::default(),
            ai: AiConfig::default(),
            observability: ObservabilityConfig::default(),
        }
    }
}
