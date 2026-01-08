//! VMS Viewer - Tauri Backend with vms-player Integration + API Client
//! - Spawns native GStreamer players for ultra-low latency (30-80ms)
//! - Connects to vms_server for auth, cameras, and live sessions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex, RwLock};
use tauri::State;
use vms_core::{Camera, User, Role, Node};

// ============================================================================
// API Client for vms_server
// ============================================================================

/// API Client state
pub struct ApiClient {
    http: reqwest::Client,
    server_url: String,
    access_token: RwLock<Option<String>>,
    refresh_token: RwLock<Option<String>>,
    current_user: RwLock<Option<User>>,
}

impl ApiClient {
    fn new() -> Self {
        let server_url = std::env::var("VMS_SERVER_URL")
            .unwrap_or_else(|_| "http://localhost:9095".to_string());
        
        Self {
            http: reqwest::Client::new(),
            server_url,
            access_token: RwLock::new(None),
            refresh_token: RwLock::new(None),
            current_user: RwLock::new(None),
        }
    }

    fn auth_header(&self) -> Option<String> {
        self.access_token.read().ok()?.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResult {
    pub success: bool,
    pub message: String,
    pub user: Option<User>,
}

/// Login to server
#[tauri::command]
async fn api_login(
    email: String,
    password: String,
    state: State<'_, ApiClient>,
) -> Result<AuthResult, String> {
    let url = format!("{}/api/v1/auth/login", state.server_url);
    
    let response = state.http
        .post(&url)
        .json(&LoginRequest { email: email.clone(), password })
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        let error: ApiError = response.json().await.unwrap_or(ApiError { error: "Unknown error".to_string() });
        return Ok(AuthResult {
            success: false,
            message: error.error,
            user: None,
        });
    }

    let token_response: TokenResponse = response.json().await
        .map_err(|e| format!("Parse error: {}", e))?;

    // Store tokens securely
    *state.access_token.write().unwrap() = Some(token_response.access_token);
    *state.refresh_token.write().unwrap() = Some(token_response.refresh_token);
    *state.current_user.write().unwrap() = Some(token_response.user.clone());

    Ok(AuthResult {
        success: true,
        message: format!("Welcome, {}", token_response.user.name),
        user: Some(token_response.user),
    })
}

/// Logout
#[tauri::command]
async fn api_logout(state: State<'_, ApiClient>) -> Result<AuthResult, String> {
    let refresh = state.refresh_token.read().unwrap().clone();
    
    if let Some(token) = refresh {
        let url = format!("{}/api/v1/auth/logout", state.server_url);
        let _ = state.http
            .post(&url)
            .json(&serde_json::json!({ "refresh_token": token }))
            .send()
            .await;
    }

    // Clear tokens
    *state.access_token.write().unwrap() = None;
    *state.refresh_token.write().unwrap() = None;
    *state.current_user.write().unwrap() = None;

    Ok(AuthResult {
        success: true,
        message: "Logged out".to_string(),
        user: None,
    })
}

/// Get current user
#[tauri::command]
fn api_get_user(state: State<'_, ApiClient>) -> Option<User> {
    state.current_user.read().ok()?.clone()
}

/// Check if logged in
#[tauri::command]
fn api_is_logged_in(state: State<'_, ApiClient>) -> bool {
    state.access_token.read().ok().map(|t| t.is_some()).unwrap_or(false)
}

/// List cameras (with ACL applied by server)
#[tauri::command]
async fn api_list_cameras(state: State<'_, ApiClient>) -> Result<Vec<Camera>, String> {
    let token = state.auth_header()
        .ok_or_else(|| "Not logged in".to_string())?;
    
    let url = format!("{}/api/v1/cameras", state.server_url);
    
    let response = state.http
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err("Failed to fetch cameras".to_string());
    }

    let cameras: Vec<Camera> = response.json().await
        .map_err(|e| format!("Parse error: {}", e))?;

    Ok(cameras)
}

/// List nodes  
#[tauri::command]
async fn api_list_nodes(state: State<'_, ApiClient>) -> Result<Vec<Node>, String> {
    let token = state.auth_header()
        .ok_or_else(|| "Not logged in".to_string())?;
    
    let url = format!("{}/api/v1/nodes", state.server_url);
    
    let response = state.http
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err("Failed to fetch nodes".to_string());
    }

    let nodes: Vec<Node> = response.json().await
        .map_err(|e| format!("Parse error: {}", e))?;

    Ok(nodes)
}

/// Stream URL response from server
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamUrlResponse {
    pub rtsp_url: String,
    pub camera_name: String,
    pub resolution: (u32, u32),
}

/// Get authenticated RTSP URL for native GStreamer playback
#[tauri::command]
async fn api_get_stream_url(state: State<'_, ApiClient>, camera_id: String) -> Result<StreamUrlResponse, String> {
    let token = state.auth_header()
        .ok_or_else(|| "Not logged in".to_string())?;
    
    let url = format!("{}/api/v1/cameras/{}/stream-url", state.server_url, camera_id);
    
    let response = state.http
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err("Failed to fetch stream URL".to_string());
    }

    let stream_info: StreamUrlResponse = response.json().await
        .map_err(|e| format!("Parse error: {}", e))?;

    Ok(stream_info)
}

// ============================================================================
// Player Manager (existing)
// ============================================================================

/// Active player processes
struct PlayerManager {
    players: Mutex<HashMap<String, Child>>,
    gstreamer_path: String,
    player_path: String,
}

impl PlayerManager {
    fn new() -> Self {
        let gstreamer_path = std::env::var("GSTREAMER_1_0_ROOT_MSVC_X86_64")
            .unwrap_or_else(|_| r"C:\Program Files\gstreamer\1.0\msvc_x86_64".to_string());
        
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
    
    if players.contains_key(&camera.id) {
        return PlayerResult {
            success: false,
            message: format!("Player for camera {} already running", camera.id),
            camera_id: Some(camera.id),
        };
    }
    
    let gst_bin = format!("{}/bin", state.gstreamer_path);
    let current_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{};{}", gst_bin, current_path);
    
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

// ============================================================================
// App Entry Point
// ============================================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(PlayerManager::new())
        .manage(ApiClient::new())
        .invoke_handler(tauri::generate_handler![
            // API Commands
            api_login,
            api_logout,
            api_get_user,
            api_is_logged_in,
            api_list_cameras,
            api_list_nodes,
            api_get_stream_url,
            // Player Commands
            start_player,
            stop_player,
            stop_all_players,
            get_active_players,
            check_gstreamer,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
