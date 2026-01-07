//! Métricas Prometheus para vms-ingest

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Métricas do serviço de ingestão
#[derive(Clone)]
pub struct IngestMetrics {
    pub cameras_online: Arc<AtomicU64>,
    pub cameras_offline: Arc<AtomicU64>,
    pub cameras_error: Arc<AtomicU64>,
    pub total_frames: Arc<AtomicU64>,
    pub total_bytes: Arc<AtomicU64>,
    pub reconnect_attempts: Arc<AtomicU64>,
}

impl IngestMetrics {
    pub fn new() -> Self {
        Self {
            cameras_online: Arc::new(AtomicU64::new(0)),
            cameras_offline: Arc::new(AtomicU64::new(0)),
            cameras_error: Arc::new(AtomicU64::new(0)),
            total_frames: Arc::new(AtomicU64::new(0)),
            total_bytes: Arc::new(AtomicU64::new(0)),
            reconnect_attempts: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn increment_online(&self) {
        self.cameras_online.fetch_add(1, Ordering::Relaxed);
    }

    pub fn decrement_online(&self) {
        self.cameras_online.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn increment_frames(&self) {
        self.total_frames.fetch_add(1, Ordering::Relaxed);
    }

    pub fn add_bytes(&self, bytes: u64) {
        self.total_bytes.fetch_add(bytes, Ordering::Relaxed);
    }

    pub fn increment_reconnects(&self) {
        self.reconnect_attempts.fetch_add(1, Ordering::Relaxed);
    }

    /// Exporta métricas em formato Prometheus
    pub fn export(&self) -> String {
        format!(
            "# HELP vms_cameras_online Number of cameras online\n\
             # TYPE vms_cameras_online gauge\n\
             vms_cameras_online {}\n\
             # HELP vms_cameras_offline Number of cameras offline\n\
             # TYPE vms_cameras_offline gauge\n\
             vms_cameras_offline {}\n\
             # HELP vms_cameras_error Number of cameras in error state\n\
             # TYPE vms_cameras_error gauge\n\
             vms_cameras_error {}\n\
             # HELP vms_total_frames_ingested Total frames ingested\n\
             # TYPE vms_total_frames_ingested counter\n\
             vms_total_frames_ingested {}\n\
             # HELP vms_total_bytes_ingested Total bytes ingested\n\
             # TYPE vms_total_bytes_ingested counter\n\
             vms_total_bytes_ingested {}\n\
             # HELP vms_reconnect_attempts Total reconnection attempts\n\
             # TYPE vms_reconnect_attempts counter\n\
             vms_reconnect_attempts {}\n",
            self.cameras_online.load(Ordering::Relaxed),
            self.cameras_offline.load(Ordering::Relaxed),
            self.cameras_error.load(Ordering::Relaxed),
            self.total_frames.load(Ordering::Relaxed),
            self.total_bytes.load(Ordering::Relaxed),
            self.reconnect_attempts.load(Ordering::Relaxed),
        )
    }
}

impl Default for IngestMetrics {
    fn default() -> Self {
        Self::new()
    }
}
