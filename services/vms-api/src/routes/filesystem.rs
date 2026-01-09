//! Filesystem routes - Directory listing and creation
//!
//! Allows browsing server directories for recording path selection

use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Directory entry
#[derive(Debug, Serialize)]
pub struct DirEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: Option<u64>,
}

/// List directory response
#[derive(Debug, Serialize)]
pub struct ListDirResponse {
    pub current_path: String,
    pub parent_path: Option<String>,
    pub entries: Vec<DirEntry>,
    pub drives: Vec<String>,
}

/// Query params for list
#[derive(Debug, Deserialize)]
pub struct ListDirQuery {
    pub path: Option<String>,
}

/// Create folder request
#[derive(Debug, Deserialize)]
pub struct CreateFolderRequest {
    pub path: String,
    pub name: String,
}

/// GET /api/v1/filesystem/list - List directory contents
pub async fn list_directory(
    Query(query): Query<ListDirQuery>,
) -> impl IntoResponse {
    // Default to drives list on Windows
    let path = query.path.unwrap_or_else(|| "C:\\".to_string());
    let path_buf = PathBuf::from(&path);

    // Get available drives (Windows)
    let drives = get_windows_drives();

    // Check if path exists
    if !path_buf.exists() {
        return (
            StatusCode::OK,
            Json(ListDirResponse {
                current_path: "".to_string(),
                parent_path: None,
                entries: vec![],
                drives,
            }),
        )
            .into_response();
    }

    // Read directory
    let mut entries = Vec::new();
    
    if let Ok(read_dir) = std::fs::read_dir(&path_buf) {
        for entry in read_dir.flatten() {
            let metadata = entry.metadata().ok();
            let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
            
            // Only show directories for folder selection
            if is_dir {
                entries.push(DirEntry {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: entry.path().to_string_lossy().to_string(),
                    is_dir,
                    size: None,
                });
            }
        }
    }

    // Sort by name
    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    // Get parent path
    let parent_path = path_buf.parent().map(|p| p.to_string_lossy().to_string());

    (
        StatusCode::OK,
        Json(ListDirResponse {
            current_path: path,
            parent_path,
            entries,
            drives,
        }),
    )
        .into_response()
}

/// POST /api/v1/filesystem/create - Create new folder
pub async fn create_folder(
    Json(req): Json<CreateFolderRequest>,
) -> impl IntoResponse {
    let new_path = PathBuf::from(&req.path).join(&req.name);
    
    match std::fs::create_dir_all(&new_path) {
        Ok(_) => (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "success": true,
                "path": new_path.to_string_lossy(),
                "message": format!("Pasta '{}' criada com sucesso", req.name)
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "error": e.to_string()
            })),
        )
            .into_response(),
    }
}

/// Get Windows drives (C:, D:, E:, etc)
fn get_windows_drives() -> Vec<String> {
    let mut drives = Vec::new();
    
    #[cfg(target_os = "windows")]
    {
        for letter in b'A'..=b'Z' {
            let drive = format!("{}:\\", letter as char);
            if PathBuf::from(&drive).exists() {
                drives.push(drive);
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        drives.push("/".to_string());
        if PathBuf::from("/home").exists() {
            drives.push("/home".to_string());
        }
    }
    
    drives
}
