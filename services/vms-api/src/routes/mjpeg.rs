//! MJPEG streaming - Optimized for Low Latency
//! 
//! Uses substream for faster response, aggressive flush settings

use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::Response,
};
use bytes::Bytes;
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::process::Command;
use tokio::sync::{broadcast, RwLock};
use tracing::{info, warn};
use uuid::Uuid;

use crate::AppState;

const MJPEG_BOUNDARY: &str = "frame";

lazy_static::lazy_static! {
    static ref STREAM_CACHE: Arc<RwLock<HashMap<String, broadcast::Sender<Bytes>>>> = 
        Arc::new(RwLock::new(HashMap::new()));
}

pub async fn mjpeg_stream(
    Path(camera_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Response<Body>, StatusCode> {
    let existing_tx = {
        let cache = STREAM_CACHE.read().await;
        cache.get(&camera_id).cloned()
    };
    
    let rx = if let Some(tx) = existing_tx {
        tx.subscribe()
    } else {
        let uuid = Uuid::parse_str(&camera_id).map_err(|_| StatusCode::BAD_REQUEST)?;
        
        let camera = state.camera_repo.get(uuid).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;
        
        // Use substream for lower latency (stream2 instead stream1)
        let rtsp_url = camera.rtsp_url.replace("/stream1", "/stream2");
        let encoded_password = urlencoding::encode(&camera.password);
        
        let full_rtsp_url = if rtsp_url.starts_with("rtsp://") {
            format!("rtsp://{}:{}@{}", camera.username, encoded_password, &rtsp_url[7..])
        } else {
            rtsp_url.clone()
        };
        
        info!("📹 Using low-latency substream: {}", rtsp_url.split('@').last().unwrap_or(&rtsp_url));
        
        // Small buffer = low latency
        let (tx, rx) = broadcast::channel::<Bytes>(4);
        
        {
            let mut cache = STREAM_CACHE.write().await;
            cache.insert(camera_id.clone(), tx.clone());
        }
        
        let camera_id_clone = camera_id.clone();
        tokio::spawn(async move {
            info!("🚀 Starting optimized FFmpeg for: {}", camera_id_clone);
            let _ = run_optimized_ffmpeg(&full_rtsp_url, tx).await;
            let mut cache = STREAM_CACHE.write().await;
            cache.remove(&camera_id_clone);
        });
        
        rx
    };
    
    let stream = async_stream::stream! {
        let mut rx = rx;
        loop {
            match rx.recv().await {
                Ok(frame) => yield Ok::<_, std::io::Error>(frame),
                Err(broadcast::error::RecvError::Closed) => break,
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    warn!("Dropped {} frames to catch up", n);
                    continue;
                }
            }
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, format!("multipart/x-mixed-replace; boundary={}", MJPEG_BOUNDARY))
        .header(header::CACHE_CONTROL, "no-cache, no-store, must-revalidate")
        .header("X-Accel-Buffering", "no")
        .header("Access-Control-Allow-Origin", "*")
        .body(Body::from_stream(stream))
        .unwrap())
}

async fn run_optimized_ffmpeg(
    rtsp_url: &str,
    tx: broadcast::Sender<Bytes>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    // OPTIMIZED FOR LOW LATENCY
    let mut child = Command::new("ffmpeg")
        .args([
            // Input - fast connection
            "-rtsp_transport", "tcp",
            "-fflags", "nobuffer",           // No input buffering
            "-flags", "low_delay",           // Low delay decoding
            "-i", rtsp_url,
            
            // Output - fast encoding
            "-f", "mjpeg",
            "-q:v", "15",                    // Lower quality = MUCH faster encoding
            "-r", "30",                      // 30 FPS to reduce motion blur/ghosting
            "-flush_packets", "1",           // Flush every packet immediately
            "-an",                           // No audio
            "-"
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;

    let stdout = child.stdout.take().ok_or("No stdout")?;
    
    // TINY buffer for minimum latency
    let mut reader = tokio::io::BufReader::with_capacity(32 * 1024, stdout);
    let mut buffer = Vec::with_capacity(64 * 1024);
    let mut temp = [0u8; 8192];
    
    loop {
        match reader.read(&mut temp).await {
            Ok(0) => break,
            Ok(n) => {
                buffer.extend_from_slice(&temp[..n]);
                
                // Send frames immediately as they complete
                while let Some((start, end)) = find_jpeg(&buffer) {
                    let jpeg = buffer[start..=end].to_vec();
                    buffer.drain(..=end);
                    
                    let header = format!(
                        "--{}\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
                        MJPEG_BOUNDARY, jpeg.len()
                    );
                    let mut frame = header.into_bytes();
                    frame.extend_from_slice(&jpeg);
                    frame.extend_from_slice(b"\r\n");
                    
                    // Send immediately, drop if no subscribers
                    let _ = tx.send(Bytes::from(frame));
                }
                
                // Keep buffer tiny
                if buffer.len() > 64 * 1024 {
                    buffer.drain(..buffer.len() - 16 * 1024);
                }
            }
            Err(_) => break,
        }
    }
    
    Ok(())
}

fn find_jpeg(data: &[u8]) -> Option<(usize, usize)> {
    let start = data.windows(2).position(|w| w == [0xFF, 0xD8])?;
    for i in (start + 2)..data.len().saturating_sub(1) {
        if data[i] == 0xFF && data[i + 1] == 0xD9 {
            return Some((start, i + 1));
        }
    }
    None
}
