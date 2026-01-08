//! WebRTC streaming - RTSP to RTP task
//!
//! Spawns FFmpeg to convert RTSP to RTP and forwards packets to WebRTC track.

use std::process::Stdio;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::process::Command;
use uuid::Uuid;

use webrtc::rtp::packet::Packet;
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;
use webrtc::util::Unmarshal;
use webrtc::track::track_local::TrackLocalWriter;

/// Spawn RTSP to RTP streaming task
///
/// Starts FFmpeg to transcode RTSP to H.264 RTP, then forwards
/// RTP packets via UDP to the WebRTC track.
///
/// Returns (task_handle, rtp_port) for cleanup and debugging.
pub async fn spawn_rtsp_rtp_task(
    camera_id: Uuid,
    rtsp_url: String,
    track: Arc<TrackLocalStaticRTP>,
) -> Result<(tokio::task::JoinHandle<()>, u16), String> {
    // Bind dynamic local port for RTP
    let socket = UdpSocket::bind("127.0.0.1:0")
        .await
        .map_err(|e| format!("Failed to bind UDP socket: {}", e))?;
    
    let rtp_port = socket
        .local_addr()
        .map_err(|e| format!("Failed to get local addr: {}", e))?
        .port();

    tracing::info!(
        "🎬 Starting RTSP→RTP stream for camera {} on port {}",
        camera_id,
        rtp_port
    );

    // Spawn FFmpeg: RTSP → H.264 Baseline → RTP (HIGH QUALITY, LOW LATENCY)
    let mut child = Command::new("ffmpeg")
        .args([
            "-rtsp_transport", "tcp",
            "-fflags", "+nobuffer+flush_packets",
            "-flags", "low_delay",
            "-probesize", "500000",
            "-analyzeduration", "500000",
            "-i", &rtsp_url,
            "-an",                            // No audio
            "-c:v", "libx264",
            "-preset", "ultrafast",
            "-tune", "zerolatency",
            "-profile:v", "baseline",
            "-level", "4.1",
            "-b:v", "8M",                     // 8Mbps for high quality
            "-maxrate", "8M",
            "-bufsize", "2M",
            "-g", "10",                       // Keyframe every 10 frames
            "-keyint_min", "10",
            "-sc_threshold", "0",
            "-x264-params", "bframes=0:sliced-threads=1:rc-lookahead=0:force-cfr=1",
            "-f", "rtp",
            "-payload_type", "96",
            &format!("rtp://127.0.0.1:{}?pkt_size=1200", rtp_port),
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("FFmpeg spawn failed: {}", e))?;

    // Spawn task: recv UDP → unmarshal → write_rtp
    let handle = tokio::spawn(async move {
        let mut buf = vec![0u8; 2048];
        let mut packet_count: u64 = 0;

        loop {
            tokio::select! {
                result = socket.recv(&mut buf) => {
                    let n = match result {
                        Ok(n) if n > 0 => n,
                        Ok(_) => continue,
                        Err(e) => {
                            tracing::warn!("UDP recv error for camera {}: {}", camera_id, e);
                            break;
                        }
                    };

                    // Unmarshal RTP packet
                    let packet = match Packet::unmarshal(&mut &buf[..n]) {
                        Ok(p) => p,
                        Err(_) => continue, // Skip malformed packets
                    };

                    // Write to WebRTC track
                    if let Err(e) = track.write_rtp(&packet).await {
                        tracing::warn!("Track write error for camera {}: {}", camera_id, e);
                        break;
                    }

                    packet_count += 1;
                    if packet_count % 500 == 1 {
                        tracing::debug!(
                            "📦 RTP packet #{} for camera {} ({} bytes)",
                            packet_count,
                            camera_id,
                            n
                        );
                    }
                }
                status = child.wait() => {
                    match status {
                        Ok(s) => tracing::info!("FFmpeg exited for camera {}: {}", camera_id, s),
                        Err(e) => tracing::error!("FFmpeg error for camera {}: {}", camera_id, e),
                    }
                    break;
                }
            }
        }

        // Cleanup: kill FFmpeg if still running
        let _ = child.kill().await;
        tracing::info!(
            "🛑 Stream stopped for camera {} after {} packets",
            camera_id,
            packet_count
        );
    });

    Ok((handle, rtp_port))
}
