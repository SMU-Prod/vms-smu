//! WebRTC runtime state management
//!
//! Maintains active peer connections for cleanup and monitoring.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use webrtc::peer_connection::RTCPeerConnection;
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;

/// Runtime state for a single WebRTC peer connection
pub struct PeerRuntime {
    /// The RTCPeerConnection
    pub peer: Arc<RTCPeerConnection>,
    /// The video track being sent
    pub track: Arc<TrackLocalStaticRTP>,
    /// UDP port where FFmpeg sends RTP
    pub rtp_port: u16,
    /// Handle to the streaming task (for abort on cleanup)
    pub task_handle: tokio::task::JoinHandle<()>,
    /// Camera ID for logging
    pub camera_id: Uuid,
}

/// Global WebRTC runtime state
/// 
/// Manages all active peer connections for cleanup and monitoring.
#[derive(Clone, Default)]
pub struct WebRtcRuntime {
    peers: Arc<RwLock<HashMap<Uuid, PeerRuntime>>>,
}

impl WebRtcRuntime {
    /// Create new runtime
    pub fn new() -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Insert a new peer runtime
    pub async fn insert(&self, peer_id: Uuid, runtime: PeerRuntime) {
        self.peers.write().await.insert(peer_id, runtime);
        tracing::info!("📡 WebRTC peer {} registered", peer_id);
    }

    /// Remove and cleanup a peer (stops task, closes peer)
    pub async fn remove(&self, peer_id: Uuid) -> Option<PeerRuntime> {
        if let Some(rt) = self.peers.write().await.remove(&peer_id) {
            // Abort the streaming task
            rt.task_handle.abort();
            // Close the peer connection
            let _ = rt.peer.close().await;
            tracing::info!("🔌 WebRTC peer {} removed and cleaned up", peer_id);
            Some(rt)
        } else {
            None
        }
    }

    /// Get count of active peers
    pub async fn count(&self) -> usize {
        self.peers.read().await.len()
    }

    /// Check if a peer exists
    pub async fn contains(&self, peer_id: Uuid) -> bool {
        self.peers.read().await.contains_key(&peer_id)
    }

    /// Cleanup all peers (on shutdown)
    pub async fn cleanup_all(&self) {
        let mut peers = self.peers.write().await;
        for (peer_id, rt) in peers.drain() {
            rt.task_handle.abort();
            let _ = rt.peer.close().await;
            tracing::info!("🧹 Cleaned up peer {} on shutdown", peer_id);
        }
    }
}
