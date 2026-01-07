//! WebRTC streaming server - Real implementation with webrtc-rs
//!
//! Professional WebRTC server that:
//! 1. Parses browser SDP offer
//! 2. Creates real RTCPeerConnection
//! 3. Adds H264 video track
//! 4. Generates proper SDP answer
//! 5. Handles ICE negotiation
//! 6. Streams video frames from NATS

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use vms_common::types::StreamId;
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::{MediaEngine, MIME_TYPE_H264};
use webrtc::api::APIBuilder;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::rtp_transceiver::rtp_codec::RTCRtpCodecCapability;
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;
use webrtc::track::track_local::TrackLocal;

use crate::nats_consumer::StreamDistributor;

/// Active WebRTC session with peer connection
pub struct WebRTCSession {
    pub id: StreamId,
    pub camera_id: String,
    pub viewer_id: String,
    pub peer_connection: Arc<RTCPeerConnection>,
    pub video_track: Arc<TrackLocalStaticRTP>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct WebRTCServer {
    sessions: Arc<RwLock<HashMap<StreamId, Arc<WebRTCSession>>>>,
    distributor: Arc<StreamDistributor>,
}

impl WebRTCServer {
    pub fn new(distributor: Arc<StreamDistributor>) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            distributor,
        }
    }

    /// Create WebRTC API with H264 codec support
    fn create_api() -> Result<webrtc::api::API> {
        // Create media engine with H264 support
        let mut media_engine = MediaEngine::default();
        
        // Register H264 codec
        media_engine.register_codec(
            webrtc::rtp_transceiver::rtp_codec::RTCRtpCodecParameters {
                capability: RTCRtpCodecCapability {
                    mime_type: MIME_TYPE_H264.to_owned(),
                    clock_rate: 90000,
                    channels: 0,
                    sdp_fmtp_line: "level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42e01f".to_owned(),
                    rtcp_feedback: vec![],
                },
                payload_type: 96,
                ..Default::default()
            },
            webrtc::rtp_transceiver::rtp_codec::RTPCodecType::Video,
        )?;

        // Create interceptor registry
        let mut registry = Registry::new();
        registry = register_default_interceptors(registry, &mut media_engine)?;

        // Build API
        let api = APIBuilder::new()
            .with_media_engine(media_engine)
            .with_interceptor_registry(registry)
            .build();

        Ok(api)
    }

    /// Create a new WebRTC session for a camera
    pub async fn create_session(
        &self,
        camera_id: String,
        viewer_id: String,
    ) -> Result<StreamId> {
        let stream_id = StreamId::new();
        
        info!(
            "🚀 Creating WebRTC session {} for camera {} viewer {}",
            stream_id, camera_id, viewer_id
        );

        // Create WebRTC API
        let api = Self::create_api().context("Failed to create WebRTC API")?;

        // ICE configuration
        let config = RTCConfiguration {
            ice_servers: vec![RTCIceServer {
                urls: vec!["stun:stun.l.google.com:19302".to_owned()],
                ..Default::default()
            }],
            ..Default::default()
        };

        // Create peer connection
        let peer_connection = Arc::new(
            api.new_peer_connection(config)
                .await
                .context("Failed to create peer connection")?
        );

        // Create H264 video track
        let video_track = Arc::new(TrackLocalStaticRTP::new(
            RTCRtpCodecCapability {
                mime_type: MIME_TYPE_H264.to_owned(),
                clock_rate: 90000,
                channels: 0,
                sdp_fmtp_line: "level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42e01f".to_owned(),
                rtcp_feedback: vec![],
            },
            "video".to_owned(),
            format!("vms-stream-{}", stream_id),
        ));

        // Add track to peer connection
        let _rtp_sender = peer_connection
            .add_track(Arc::clone(&video_track) as Arc<dyn TrackLocal + Send + Sync>)
            .await
            .context("Failed to add video track")?;

        // Set up connection state handler
        let stream_id_clone = stream_id;
        peer_connection.on_peer_connection_state_change(Box::new(move |state: RTCPeerConnectionState| {
            info!("📡 Session {} connection state: {:?}", stream_id_clone, state);
            
            match state {
                RTCPeerConnectionState::Connected => {
                    info!("✅ Session {} connected!", stream_id_clone);
                }
                RTCPeerConnectionState::Disconnected | RTCPeerConnectionState::Failed | RTCPeerConnectionState::Closed => {
                    warn!("⚠️ Session {} disconnected: {:?}", stream_id_clone, state);
                }
                _ => {}
            }
            
            Box::pin(async {})
        }));

        // Create session
        let session = Arc::new(WebRTCSession {
            id: stream_id,
            camera_id: camera_id.clone(),
            viewer_id: viewer_id.clone(),
            peer_connection,
            video_track,
            created_at: chrono::Utc::now(),
        });

        self.sessions.write().await.insert(stream_id, session);

        info!(
            "✅ Created WebRTC session {} for camera {}",
            stream_id, camera_id
        );

        Ok(stream_id)
    }

    /// Handle SDP offer from browser and return answer
    pub async fn handle_offer(&self, stream_id: StreamId, offer_sdp: String) -> Result<String> {
        let sessions = self.sessions.read().await;
        
        let session = sessions
            .get(&stream_id)
            .ok_or_else(|| anyhow::anyhow!("Session {} not found", stream_id))?;

        info!(
            "📡 Processing SDP offer for session {} ({} bytes)",
            stream_id,
            offer_sdp.len()
        );

        // Parse browser's offer
        let offer = RTCSessionDescription::offer(offer_sdp.clone())
            .context("Failed to parse SDP offer")?;

        // Set remote description (browser's offer)
        session
            .peer_connection
            .set_remote_description(offer)
            .await
            .context("Failed to set remote description")?;

        info!("📥 Set remote description for session {}", stream_id);

        // Create answer
        let answer = session
            .peer_connection
            .create_answer(None)
            .await
            .context("Failed to create answer")?;

        // Set local description (our answer)
        session
            .peer_connection
            .set_local_description(answer.clone())
            .await
            .context("Failed to set local description")?;

        info!(
            "📤 Generated SDP answer for session {} ({} bytes)",
            stream_id,
            answer.sdp.len()
        );

        Ok(answer.sdp)
    }

    /// Handle ICE candidate from browser
    pub async fn handle_ice_candidate(
        &self,
        stream_id: StreamId,
        candidate: String,
    ) -> Result<()> {
        let sessions = self.sessions.read().await;
        
        let session = sessions
            .get(&stream_id)
            .ok_or_else(|| anyhow::anyhow!("Session {} not found", stream_id))?;

        info!("🧊 Adding ICE candidate for session {}", stream_id);

        // Parse and add ICE candidate
        let ice_candidate = webrtc::ice_transport::ice_candidate::RTCIceCandidateInit {
            candidate,
            ..Default::default()
        };

        session
            .peer_connection
            .add_ice_candidate(ice_candidate)
            .await
            .context("Failed to add ICE candidate")?;

        Ok(())
    }

    /// Close a WebRTC session
    pub async fn close_session(&self, stream_id: StreamId) -> Result<()> {
        let mut sessions = self.sessions.write().await;

        if let Some(session) = sessions.remove(&stream_id) {
            info!(
                "🔴 Closing WebRTC session {} for camera {}",
                stream_id, session.camera_id
            );

            session
                .peer_connection
                .close()
                .await
                .context("Failed to close peer connection")?;
        }

        Ok(())
    }

    /// Get number of active sessions
    pub async fn active_sessions(&self) -> usize {
        self.sessions.read().await.len()
    }

    /// Get session for sending video frames
    pub async fn get_session(&self, stream_id: StreamId) -> Option<Arc<WebRTCSession>> {
        self.sessions.read().await.get(&stream_id).cloned()
    }

    /// Get all sessions for a camera
    pub async fn get_camera_sessions(&self, camera_id: &str) -> Vec<Arc<WebRTCSession>> {
        self.sessions
            .read()
            .await
            .values()
            .filter(|s| s.camera_id == camera_id)
            .cloned()
            .collect()
    }
}
