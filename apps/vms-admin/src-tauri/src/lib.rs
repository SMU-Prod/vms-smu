// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::process::Command;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Open VMS Player with ultra low-latency GStreamer playback
#[tauri::command]
async fn open_player(
    rtsp_url: String,
    username: String,
    password: String,
    width: u32,
    height: u32,
) -> Result<String, String> {
    // Encode @ as %40 in password for URL
    let safe_password = password.replace("@", "%40");
    
    // Build RTSP URL with credentials
    let full_url = if rtsp_url.starts_with("rtsp://") {
        format!("rtsp://{}:{}@{}", username, safe_password, &rtsp_url[7..])
    } else {
        rtsp_url.clone()
    };
    
    // Path to vms-player (relative to app bundle or dev path)
    let player_path = if cfg!(debug_assertions) {
        // Development: use target/release
        std::env::current_dir()
            .map(|p| p.join("../../../target/release/vms-player.exe"))
            .unwrap_or_else(|_| std::path::PathBuf::from("vms-player.exe"))
    } else {
        // Production: bundled alongside app
        std::env::current_exe()
            .map(|p| p.parent().unwrap().join("vms-player.exe"))
            .unwrap_or_else(|_| std::path::PathBuf::from("vms-player.exe"))
    };
    
    // Set GStreamer PATH
    let gst_bin = r"C:\Program Files\gstreamer\1.0\msvc_x86_64\bin";
    let current_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{};{}", gst_bin, current_path);
    
    // Spawn player process
    match Command::new(&player_path)
        .env("PATH", &new_path)
        .args([
            "--url", &full_url,
            "--width", &width.to_string(),
            "--height", &height.to_string(),
        ])
        .spawn()
    {
        Ok(child) => Ok(format!("Player started with PID {}", child.id())),
        Err(e) => Err(format!("Failed to start player: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, open_player])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
