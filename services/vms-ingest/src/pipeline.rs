//! Pipeline GStreamer OTIMIZADO - Ultra Baixa Latência + Alta Qualidade
//! H264 Passthrough - Zero decode/encode

use anyhow::{Context, Result};
use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_app as gst_app;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};
use vms_common::camera::CameraConfig;
use vms_common::stream::VideoFrame;

/// Pipeline de ingestão OTIMIZADO
pub struct IngestPipeline {
    pipeline: gst::Pipeline,
    config: Arc<CameraConfig>,
    frame_tx: Option<mpsc::Sender<VideoFrame>>,
}

impl IngestPipeline {
    /// Cria pipeline OTIMIZADO para ultra-baixa latência
    pub fn new(config: CameraConfig) -> Result<Self> {
        let pipeline = gst::Pipeline::new();

        info!("🚀 Creating OPTIMIZED pipeline for: {}", config.name);

        // RTSP Source com configurações de ZERO LATÊNCIA
        let rtspsrc = gst::ElementFactory::make("rtspsrc")
            .name("source")
            .property("location", &config.url)
            .property("latency", 0u32)                    // ZERO latency
            .property("buffer-mode", 0i32)                // Low latency mode
            .property("drop-on-latency", true)            // Drop frames se necessário
            .property("do-retransmission", false)         // Sem retransmissão
            .property("ntp-sync", false)                  // Sem sync NTP
            .property("ntp-time-source", 3i32)            // Running time
            .build()
            .context("Failed to create rtspsrc")?;

        // Autenticação
        if let (Some(user), Some(pass)) = (&config.username, &config.password) {
            rtspsrc.set_property("user-id", user);
            rtspsrc.set_property("user-pw", pass);
        }

        // RTP H264 Depayloader - extrai H264 do RTP
        let depay = gst::ElementFactory::make("rtph264depay")
            .name("depay")
            .build()
            .context("Failed to create depay")?;

        // H264 Parser - apenas parse, SEM decode
        let parse = gst::ElementFactory::make("h264parse")
            .name("parse")
            .property("config-interval", -1i32)           // Enviar SPS/PPS sempre
            .build()
            .context("Failed to create h264parse")?;

        // Queue com buffer MÍNIMO para baixa latência
        let queue = gst::ElementFactory::make("queue")
            .name("queue")
            .property("max-size-buffers", 1u32)           // Apenas 1 frame no buffer
            .property("max-size-bytes", 0u32)             // Sem limite de bytes
            .property("max-size-time", 0u64)              // Sem limite de tempo
            .property("leaky", 2i32)                      // Downstream leaky - drop old frames
            .build()
            .context("Failed to create queue")?;

        // AppSink - recebe H264 RAW (sem decode!)
        let sink = gst_app::AppSink::builder()
            .name("sink")
            .sync(false)                                  // Sem sync - máxima velocidade
            .max_buffers(1)                               // Buffer mínimo
            .drop(true)                                   // Drop frames antigos
            .build();

        // Configurar caps para H264
        let caps = gst::Caps::builder("video/x-h264")
            .field("stream-format", "byte-stream")
            .field("alignment", "au")
            .build();
        sink.set_caps(Some(&caps));

        // Adicionar elementos
        pipeline.add_many(&[&depay, &parse, &queue, sink.upcast_ref()])?;

        // Link pipeline: depay -> parse -> queue -> sink
        gst::Element::link_many(&[&depay, &parse, &queue, sink.upcast_ref()])?;

        // Conectar pads dinâmicos do rtspsrc
        let depay_clone = depay.clone();
        rtspsrc.connect_pad_added(move |_src, src_pad| {
            let sink_pad = depay_clone
                .static_pad("sink")
                .expect("Failed to get depay sink pad");

            if sink_pad.is_linked() {
                return;
            }

            let pad_caps = src_pad.current_caps();
            if let Some(caps) = pad_caps {
                let structure = caps.structure(0).expect("Failed to get caps structure");
                let media_type = structure.name();

                // Conectar apenas video RTP
                if media_type.starts_with("application/x-rtp") {
                    if let Some(media) = structure.get::<String>("media").ok() {
                        if media == "video" {
                            if let Err(e) = src_pad.link(&sink_pad) {
                                error!("❌ Failed to link pads: {}", e);
                            } else {
                                info!("✅ Linked RTSP video source (H264 passthrough)");
                            }
                        }
                    }
                }
            }
        });

        pipeline.add(&rtspsrc)?;

        Ok(Self {
            pipeline,
            config: Arc::new(config),
            frame_tx: None,
        })
    }

    /// Define o canal para enviar frames H264 RAW
    pub fn set_frame_sender(&mut self, tx: mpsc::Sender<VideoFrame>) {
        self.frame_tx = Some(tx);
    }

    /// Inicia o pipeline
    pub fn start(&self) -> Result<()> {
        info!("▶️  Starting OPTIMIZED pipeline: {}", self.config.name);
        info!("📊 Mode: H264 Passthrough (Zero decode/encode)");
        info!("⚡ Target latency: < 50ms");

        self.pipeline
            .set_state(gst::State::Playing)
            .context("Failed to set pipeline to Playing")?;

        Ok(())
    }

    /// Para o pipeline
    pub fn stop(&self) -> Result<()> {
        info!("⏹️  Stopping pipeline: {}", self.config.name);

        self.pipeline
            .set_state(gst::State::Null)
            .context("Failed to set pipeline to Null")?;

        Ok(())
    }

    /// Verifica se está rodando
    pub fn is_running(&self) -> bool {
        matches!(
            self.pipeline.current_state(),
            gst::State::Playing | gst::State::Paused
        )
    }

    /// Retorna o bus
    pub fn bus(&self) -> Option<gst::Bus> {
        self.pipeline.bus()
    }

    /// Obtém o appsink para processar frames
    pub fn get_appsink(&self) -> Option<gst_app::AppSink> {
        self.pipeline
            .by_name("sink")
            .and_then(|e| e.downcast::<gst_app::AppSink>().ok())
    }
}

impl Drop for IngestPipeline {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Handler para processar frames H264 RAW
pub struct FrameHandler {
    tx: mpsc::Sender<VideoFrame>,
    camera_id: String,
}

impl FrameHandler {
    pub fn new(tx: mpsc::Sender<VideoFrame>, camera_id: String) -> Self {
        Self { tx, camera_id }
    }

    /// Processa sample H264 RAW do appsink
    pub async fn handle_sample(&self, sample: gst::Sample) -> Result<()> {
        let buffer = sample.buffer().context("Failed to get buffer")?;
        let caps = sample.caps().context("Failed to get caps")?;

        // Extrair info
        let structure = caps.structure(0).context("Failed to get structure")?;
        
        // H264 não tem width/height no caps, usar valores da câmera
        let width = 1920u32;  // TODO: pegar do config
        let height = 1080u32;

        // Mapear buffer H264 RAW
        let map = buffer.map_readable().context("Failed to map buffer")?;
        let data = map.as_slice().to_vec();

        debug!(
            "📦 H264 frame: {} bytes ({}x{}) - camera: {}",
            data.len(),
            width,
            height,
            self.camera_id
        );

        // Criar VideoFrame com H264 RAW
        let frame = VideoFrame::new(
            vms_common::types::StreamId::new(),
            data,
            width,
            height,
        );

        // Enviar frame
        if let Err(e) = self.tx.try_send(frame) {
            warn!("⚠️  Failed to send frame (buffer full?): {}", e);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimized_pipeline_creation() {
        gst::init().unwrap();

        let config = CameraConfig::new(
            "Test Camera".to_string(),
            "rtsp://test:554/stream1".to_string(),
        );

        let pipeline = IngestPipeline::new(config);
        assert!(pipeline.is_ok());
    }
}
