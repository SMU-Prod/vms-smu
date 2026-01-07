//! Tipos relacionados a streaming

use crate::types::{FrameRate, Resolution, StreamId, Timestamp};
use serde::{Deserialize, Serialize};

/// Protocolo de streaming
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamProtocol {
    /// SRT (Secure Reliable Transport)
    SRT,
    /// WebRTC
    WebRTC,
    /// QUIC/HTTP3
    QUIC,
    /// Low-Latency HLS
    LLHLS,
}

/// Codec de vídeo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoCodec {
    H264,
    H265,
    AV1,
    VP9,
}

/// Configuração de stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    pub id: StreamId,
    pub protocol: StreamProtocol,
    pub codec: VideoCodec,
    pub resolution: Resolution,
    pub fps: FrameRate,
    pub bitrate: u64, // bits por segundo
}

/// Estatísticas de um stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamStats {
    pub stream_id: StreamId,
    pub timestamp: Timestamp,
    pub current_fps: f64,
    pub current_bitrate: u64,
    pub latency_ms: u32,
    pub packet_loss: f32, // porcentagem
    pub viewers: usize,
}

/// Frame de vídeo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFrame {
    pub stream_id: StreamId,
    pub timestamp: Timestamp,
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub is_keyframe: bool,
}

impl VideoFrame {
    /// Cria um novo frame de vídeo
    pub fn new(stream_id: StreamId, data: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            stream_id,
            timestamp: Timestamp::now(),
            data,
            width,
            height,
            is_keyframe: false,
        }
    }

    /// Marca frame como keyframe
    pub fn as_keyframe(mut self) -> Self {
        self.is_keyframe = true;
        self
    }

    /// Retorna o tamanho do frame em bytes
    pub fn size(&self) -> usize {
        self.data.len()
    }
}
