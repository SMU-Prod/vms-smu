//! Permission management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    /// Permission ID
    pub id: Uuid,
    /// Permission name (e.g., "cameras.view", "recordings.export")
    pub name: String,
    /// Description
    pub description: String,
    /// Created at
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Permission {
    /// Create new permission
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            created_at: chrono::Utc::now(),
        }
    }
}

/// Permission manager
pub struct PermissionManager {
    permissions: Arc<RwLock<HashMap<Uuid, Permission>>>,
}

impl PermissionManager {
    /// Create new permission manager
    pub fn new() -> Self {
        Self {
            permissions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add permission
    pub async fn add(&self, permission: Permission) {
        let mut permissions = self.permissions.write().await;
        permissions.insert(permission.id, permission);
    }

    /// Get permission by ID
    pub async fn get(&self, id: Uuid) -> Option<Permission> {
        let permissions = self.permissions.read().await;
        permissions.get(&id).cloned()
    }

    /// Get permission by name
    pub async fn get_by_name(&self, name: &str) -> Option<Permission> {
        let permissions = self.permissions.read().await;
        permissions.values().find(|p| p.name == name).cloned()
    }

    /// List all permissions
    pub async fn list_all(&self) -> Vec<Permission> {
        let permissions = self.permissions.read().await;
        permissions.values().cloned().collect()
    }

    /// Delete permission
    pub async fn delete(&self, id: Uuid) -> Result<(), String> {
        let mut permissions = self.permissions.write().await;
        permissions
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| "Permission not found".to_string())
    }
}

impl Default for PermissionManager {
    fn default() -> Self {
        Self::new()
    }
}
