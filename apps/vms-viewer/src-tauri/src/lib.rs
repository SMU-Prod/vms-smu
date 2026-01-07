//! VMS Viewer - Tauri Backend with vms-player Integration
//! Spawns native GStreamer players for ultra-low latency (30-80ms)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Child, Command};
use std::sync::Mutex;
use tauri::State;

/// Active player processes
struct PlayerManager {
    players: Mutex<HashMap<String, Child>>,
    gstreamer_path: String,
    player_path: String,
}

impl PlayerManager {
    fn new() -> Self {
        // GStreamer path
        let gstreamer_path = std::env::var("GSTREAMER_1_0_ROOT_MSVC_X86_64")
            .unwrap_or_else(|_| r"C:\Program Files\gstreamer\1.0\msvc_x86_64".to_string());
        
        // Search for vms-player.exe in multiple locations
        let possible_paths: Vec<String> = vec![
            r"C:\monitoring\backend\vms\target\release\vms-player.exe".to_string(),
            r"C:\monitoring\backend\vms\target\debug\vms-player.exe".to_string(),
            std::env::current_exe()
                .map(|p| p.parent().unwrap().join("vms-player.exe").to_string_lossy().to_string())
                .unwrap_or_default(),
        ];
        
        let player_path = possible_paths
            .into_iter()
            .find(|p| !p.is_empty() && std::path::Path::new(p).exists())
            .unwrap_or_else(|| r"C:\monitoring\backend\vms\target\release\vms-player.exe".to_string());
        
        Self {
            players: Mutex::new(HashMap::new()),
            gstreamer_path,
            player_path,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CameraConfig {
    pub id: String,
    pub name: String,
    pub rtsp_url: String,
    pub username: String,
    pub password: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerResult {
    pub success: bool,
    pub message: String,
    pub camera_id: Option<String>,
}

/// Start a camera player
#[tauri::command]
fn start_player(
    camera: CameraConfig,
    state: State<PlayerManager>,
) -> PlayerResult {
    let mut players = state.players.lock().unwrap();
    
    // Check if already running
    if players.contains_key(&camera.id) {
        return PlayerResult {
            success: false,
            message: format!("Player for camera {} already running", camera.id),
            camera_id: Some(camera.id),
        };
    }
    
    // Build PATH with GStreamer
    let gst_bin = format!("{}/bin", state.gstreamer_path);
    let current_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{};{}", gst_bin, current_path);
    
    // Spawn vms-player
    let result = Command::new(&state.player_path)
        .env("PATH", &new_path)
        .env("GSTREAMER_1_0_ROOT_MSVC_X86_64", &state.gstreamer_path)
        .args([
            "--url", &camera.rtsp_url,
            "--username", &camera.username,
            "--password", &camera.password,
            "--width", &camera.width.to_string(),
            "--height", &camera.height.to_string(),
        ])
        .spawn();
    
    match result {
        Ok(child) => {
            players.insert(camera.id.clone(), child);
            PlayerResult {
                success: true,
                message: format!("Started player for {}", camera.name),
                camera_id: Some(camera.id),
            }
        }
        Err(e) => PlayerResult {
            success: false,
            message: format!("Failed to start player: {}", e),
            camera_id: Some(camera.id),
        },
    }
}

/// Stop a camera player
#[tauri::command]
fn stop_player(camera_id: String, state: State<PlayerManager>) -> PlayerResult {
    let mut players = state.players.lock().unwrap();
    
    if let Some(mut child) = players.remove(&camera_id) {
        match child.kill() {
            Ok(_) => PlayerResult {
                success: true,
                message: format!("Stopped player for camera {}", camera_id),
                camera_id: Some(camera_id),
            },
            Err(e) => PlayerResult {
                success: false,
                message: format!("Failed to stop player: {}", e),
                camera_id: Some(camera_id),
            },
        }
    } else {
        PlayerResult {
            success: false,
            message: format!("No player running for camera {}", camera_id),
            camera_id: Some(camera_id),
        }
    }
}

/// Stop all players
#[tauri::command]
fn stop_all_players(state: State<PlayerManager>) -> PlayerResult {
    let mut players = state.players.lock().unwrap();
    let count = players.len();
    
    for (_, mut child) in players.drain() {
        let _ = child.kill();
    }
    
    PlayerResult {
        success: true,
        message: format!("Stopped {} players", count),
        camera_id: None,
    }
}

/// Get list of active players
#[tauri::command]
fn get_active_players(state: State<PlayerManager>) -> Vec<String> {
    let players = state.players.lock().unwrap();
    players.keys().cloned().collect()
}

/// Check if GStreamer is available
#[tauri::command]
fn check_gstreamer(state: State<PlayerManager>) -> PlayerResult {
    let gst_bin = format!("{}/bin/gst-inspect-1.0.exe", state.gstreamer_path);
    
    if std::path::Path::new(&gst_bin).exists() {
        PlayerResult {
            success: true,
            message: format!("GStreamer found at {}", state.gstreamer_path),
            camera_id: None,
        }
    } else {
        PlayerResult {
            success: false,
            message: "GStreamer not found. Please install GStreamer MSVC runtime".to_string(),
            camera_id: None,
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(PlayerManager::new())
        .invoke_handler(tauri::generate_handler![
            start_player,
            stop_player,
            stop_all_players,
            get_active_players,
            check_gstreamer,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
