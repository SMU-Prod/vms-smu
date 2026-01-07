//! Role management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    /// Role ID
    pub id: Uuid,
    /// Role name
    pub name: String,
    /// Description
    pub description: String,
    /// Permissions
    pub permissions: Vec<Uuid>,
    /// Created at
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Updated at
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Role {
    /// Create new role
    pub fn new(name: String, description: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            permissions: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

/// Role manager
pub struct RoleManager {
    roles: Arc<RwLock<HashMap<Uuid, Role>>>,
}

impl RoleManager {
    /// Create new role manager
    pub fn new() -> Self {
        Self {
            roles: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add role
    pub async fn add(&self, role: Role) {
        let mut roles = self.roles.write().await;
        roles.insert(role.id, role);
    }

    /// Get role by ID
    pub async fn get(&self, id: Uuid) -> Option<Role> {
        let roles = self.roles.read().await;
        roles.get(&id).cloned()
    }

    /// Get role by name
    pub async fn get_by_name(&self, name: &str) -> Option<Role> {
        let roles = self.roles.read().await;
        roles.values().find(|r| r.name == name).cloned()
    }

    /// Update role
    pub async fn update(&self, id: Uuid, mut role: Role) -> Result<(), String> {
        let mut roles = self.roles.write().await;
        if roles.contains_key(&id) {
            role.updated_at = chrono::Utc::now();
            roles.insert(id, role);
            Ok(())
        } else {
            Err("Role not found".to_string())
        }
    }

    /// Delete role
    pub async fn delete(&self, id: Uuid) -> Result<(), String> {
        let mut roles = self.roles.write().await;
        roles
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| "Role not found".to_string())
    }

    /// List all roles
    pub async fn list_all(&self) -> Vec<Role> {
        let roles = self.roles.read().await;
        roles.values().cloned().collect()
    }

    /// Assign permission to role
    pub async fn assign_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), String> {
        let mut roles = self.roles.write().await;
        if let Some(role) = roles.get_mut(&role_id) {
            if !role.permissions.contains(&permission_id) {
                role.permissions.push(permission_id);
                role.updated_at = chrono::Utc::now();
            }
            Ok(())
        } else {
            Err("Role not found".to_string())
        }
    }

    /// Remove permission from role
    pub async fn remove_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), String> {
        let mut roles = self.roles.write().await;
        if let Some(role) = roles.get_mut(&role_id) {
            role.permissions.retain(|&id| id != permission_id);
            role.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err("Role not found".to_string())
        }
    }
}

impl Default for RoleManager {
    fn default() -> Self {
        Self::new()
    }
}
