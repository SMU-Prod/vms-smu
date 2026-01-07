//! Pipeline GStreamer para ingestão de vídeo

use anyhow::{Context, Result};
use gstreamer as gst;
use gstreamer::prelude::*;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info};
use vms_common::camera::CameraConfig;
use vms_common::stream::VideoFrame;

/// Pipeline de ingestão de vídeo
pub struct IngestPipeline {
    pipeline: gst::Pipeline,
    config: Arc<CameraConfig>,
    frame_tx: Option<mpsc::Sender<VideoFrame>>,
}

impl IngestPipeline {
    /// Cria um novo pipeline de ingestão
    pub fn new(config: CameraConfig) -> Result<Self> {
        let pipeline = gst::Pipeline::new();

        // Criar elementos do pipeline
        // RTSP Source -> RTP Depay -> H264 Parse -> Decode -> Video Convert -> App Sink

        let rtspsrc = gst::ElementFactory::make("rtspsrc")
            .name("source")
            .property("location", &config.url)
            .property("latency", 0u32)
            .property("drop-on-latency", true)
            .build()
            .context("Failed to create rtspsrc")?;

        // Configurar autenticação se necessário
        if let (Some(user), Some(pass)) = (&config.username, &config.password) {
            rtspsrc.set_property("user-id", user);
            rtspsrc.set_property("user-pw", pass);
        }

        let depay = gst::ElementFactory::make("rtph264depay")
            .name("depay")
            .build()
            .context("Failed to create depay")?;

        let parse = gst::ElementFactory::make("h264parse")
            .name("parse")
            .build()
            .context("Failed to create h264parse")?;

        let decode = gst::ElementFactory::make("avdec_h264")
            .name("decode")
            .build()
            .context("Failed to create decoder")?;

        let convert = gst::ElementFactory::make("videoconvert")
            .name("convert")
            .build()
            .context("Failed to create videoconvert")?;

        let sink = gst::ElementFactory::make("appsink")
            .name("sink")
            .property("emit-signals", true)
            .property("sync", false)
            .build()
            .context("Failed to create appsink")?;

        // Adicionar elementos ao pipeline
        pipeline.add_many(&[&depay, &parse, &decode, &convert, &sink])?;

        // Link elementos estáticos (rtspsrc precisa de pad-added callback)
        gst::Element::link_many(&[&depay, &parse, &decode, &convert, &sink])?;

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

                if media_type.starts_with("application/x-rtp") {
                    if let Err(e) = src_pad.link(&sink_pad) {
                        error!("Failed to link pads: {}", e);
                    } else {
                        info!("Successfully linked RTSP source");
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

    /// Define o canal para enviar frames processados
    pub fn set_frame_sender(&mut self, tx: mpsc::Sender<VideoFrame>) {
        self.frame_tx = Some(tx);
    }

    /// Inicia o pipeline
    pub fn start(&self) -> Result<()> {
        info!("Starting pipeline for camera: {}", self.config.name);

        self.pipeline
            .set_state(gst::State::Playing)
            .context("Failed to set pipeline to Playing")?;

        Ok(())
    }

    /// Para o pipeline
    pub fn stop(&self) -> Result<()> {
        info!("Stopping pipeline for camera: {}", self.config.name);

        self.pipeline
            .set_state(gst::State::Null)
            .context("Failed to set pipeline to Null")?;

        Ok(())
    }

    /// Verifica se o pipeline está rodando
    pub fn is_running(&self) -> bool {
        matches!(
            self.pipeline.current_state(),
            gst::State::Playing | gst::State::Paused
        )
    }

    /// Retorna o bus para mensagens do pipeline
    pub fn bus(&self) -> Option<gst::Bus> {
        self.pipeline.bus()
    }
}

impl Drop for IngestPipeline {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Handler para processar frames do appsink
pub struct FrameHandler {
    tx: mpsc::Sender<VideoFrame>,
}

impl FrameHandler {
    pub fn new(tx: mpsc::Sender<VideoFrame>) -> Self {
        Self { tx }
    }

    /// Processa um sample do appsink
    pub async fn handle_sample(&self, sample: gst::Sample) -> Result<()> {
        let buffer = sample.buffer().context("Failed to get buffer")?;
        let caps = sample.caps().context("Failed to get caps")?;

        // Extrair informações do vídeo
        let structure = caps.structure(0).context("Failed to get structure")?;
        let width = structure.get::<i32>("width")?;
        let height = structure.get::<i32>("height")?;

        // Mapear buffer para leitura
        let map = buffer.map_readable().context("Failed to map buffer")?;
        let data = map.as_slice().to_vec();

        // Criar VideoFrame
        let frame = VideoFrame::new(
            vms_common::types::StreamId::new(),
            data,
            width as u32,
            height as u32,
        );

        // Enviar frame
        self.tx
            .send(frame)
            .await
            .context("Failed to send frame")?;

        debug!("Processed frame: {}x{} ({} bytes)", width, height, map.len());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_creation() {
        gst::init().unwrap();

        let config = CameraConfig::new(
            "Test".to_string(),
            "rtsp://test".to_string(),
        );

        let pipeline = IngestPipeline::new(config);
        assert!(pipeline.is_ok());
    }
}
