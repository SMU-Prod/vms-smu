//! User management

use super::password::hash_password;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// User
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: Uuid,
    /// Username
    pub username: String,
    /// Email
    pub email: String,
    /// Password hash
    #[serde(skip_serializing)]
    pub password_hash: String,
    /// Active status
    pub active: bool,
    /// Roles
    pub roles: Vec<Uuid>,
    /// Created at
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Updated at
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    /// Create new user
    pub fn new(username: String, email: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash: String::new(),
            active: true,
            roles: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Set password
    pub fn set_password(&mut self, password: &str) -> anyhow::Result<()> {
        self.password_hash = hash_password(password)?;
        Ok(())
    }

    /// Verify password
    pub fn verify_password(&self, password: &str) -> bool {
        use argon2::{Argon2, PasswordHash, PasswordVerifier};

        let parsed_hash = match PasswordHash::new(&self.password_hash) {
            Ok(hash) => hash,
            Err(_) => return false,
        };

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

/// User manager
pub struct UserManager {
    users: Arc<RwLock<HashMap<Uuid, User>>>,
}

impl UserManager {
    /// Create new user manager
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add user
    pub async fn add(&self, user: User) {
        let mut users = self.users.write().await;
        users.insert(user.id, user);
    }

    /// Get user by ID
    pub async fn get(&self, id: Uuid) -> Option<User> {
        let users = self.users.read().await;
        users.get(&id).cloned()
    }

    /// Get user by username
    pub async fn get_by_username(&self, username: &str) -> Option<User> {
        let users = self.users.read().await;
        users.values().find(|u| u.username == username).cloned()
    }

    /// Update user
    pub async fn update(&self, id: Uuid, mut user: User) -> Result<(), String> {
        let mut users = self.users.write().await;
        if users.contains_key(&id) {
            user.updated_at = chrono::Utc::now();
            users.insert(id, user);
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }

    /// Delete user
    pub async fn delete(&self, id: Uuid) -> Result<(), String> {
        let mut users = self.users.write().await;
        users
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| "User not found".to_string())
    }

    /// List all users
    pub async fn list_all(&self) -> Vec<User> {
        let users = self.users.read().await;
        users.values().cloned().collect()
    }

    /// Assign role to user
    pub async fn assign_role(&self, user_id: Uuid, role_id: Uuid) -> Result<(), String> {
        let mut users = self.users.write().await;
        if let Some(user) = users.get_mut(&user_id) {
            if !user.roles.contains(&role_id) {
                user.roles.push(role_id);
                user.updated_at = chrono::Utc::now();
            }
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }

    /// Remove role from user
    pub async fn remove_role(&self, user_id: Uuid, role_id: Uuid) -> Result<(), String> {
        let mut users = self.users.write().await;
        if let Some(user) = users.get_mut(&user_id) {
            user.roles.retain(|&id| id != role_id);
            user.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }
}

impl Default for UserManager {
    fn default() -> Self {
        Self::new()
    }
}
