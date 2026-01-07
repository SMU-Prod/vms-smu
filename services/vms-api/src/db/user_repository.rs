//! User database repository

use anyhow::Result;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::models::user::{User, UserRole};

pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Create users table
    pub async fn create_table(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                name TEXT NOT NULL,
                email TEXT,
                role TEXT NOT NULL DEFAULT 'viewer',
                enabled BOOLEAN NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                last_login TEXT
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create default admin if not exists
        self.ensure_default_admin().await?;

        Ok(())
    }

    /// Ensure default admin user exists
    async fn ensure_default_admin(&self) -> Result<()> {
        let exists = sqlx::query("SELECT id FROM users WHERE username = 'admin'")
            .fetch_optional(&self.pool)
            .await?
            .is_some();

        if !exists {
            // Hash for "admin" using argon2
            let password_hash = Self::hash_password("admin")?;
            let user = User::new(
                "admin".to_string(),
                password_hash,
                "Administrator".to_string(),
                UserRole::Admin,
            );
            self.create(&user).await?;
            tracing::info!("✅ Default admin user created (username: admin, password: admin)");
        }

        Ok(())
    }

    /// Hash password using argon2
    pub fn hash_password(password: &str) -> Result<String> {
        use argon2::{
            password_hash::{PasswordHasher, SaltString},
            Argon2,
        };
        use rand::rngs::OsRng;

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
            .to_string();

        Ok(hash)
    }

    /// Verify password against hash
    pub fn verify_password(password: &str, hash: &str) -> bool {
        use argon2::{
            password_hash::{PasswordHash, PasswordVerifier},
            Argon2,
        };

        let parsed_hash = match PasswordHash::new(hash) {
            Ok(h) => h,
            Err(_) => return false,
        };

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    /// Create new user
    pub async fn create(&self, user: &User) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO users (
                id, username, password_hash, name, email, role,
                enabled, created_at, updated_at, last_login
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(user.id.to_string())
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(&user.name)
        .bind(&user.email)
        .bind(user.role.as_str())
        .bind(user.enabled)
        .bind(user.created_at.to_rfc3339())
        .bind(user.updated_at.to_rfc3339())
        .bind(user.last_login.map(|dt| dt.to_rfc3339()))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get user by username
    pub async fn get_by_username(&self, username: &str) -> Result<Option<User>> {
        let row = sqlx::query("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.and_then(|row| self.row_to_user(&row)))
    }

    /// Get user by ID
    pub async fn get(&self, id: Uuid) -> Result<Option<User>> {
        let row = sqlx::query("SELECT * FROM users WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.and_then(|row| self.row_to_user(&row)))
    }

    /// List all users
    pub async fn list(&self) -> Result<Vec<User>> {
        let rows = sqlx::query("SELECT * FROM users ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let users = rows.iter().filter_map(|row| self.row_to_user(row)).collect();
        Ok(users)
    }

    /// Update user
    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        email: Option<Option<String>>,
        role: Option<UserRole>,
        enabled: Option<bool>,
    ) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();

        if let Some(n) = name {
            sqlx::query("UPDATE users SET name = ?, updated_at = ? WHERE id = ?")
                .bind(n)
                .bind(&now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }

        if let Some(e) = email {
            sqlx::query("UPDATE users SET email = ?, updated_at = ? WHERE id = ?")
                .bind(e)
                .bind(&now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }

        if let Some(r) = role {
            sqlx::query("UPDATE users SET role = ?, updated_at = ? WHERE id = ?")
                .bind(r.as_str())
                .bind(&now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }

        if let Some(en) = enabled {
            sqlx::query("UPDATE users SET enabled = ?, updated_at = ? WHERE id = ?")
                .bind(en)
                .bind(&now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    /// Update password
    pub async fn update_password(&self, id: Uuid, password_hash: String) -> Result<()> {
        sqlx::query("UPDATE users SET password_hash = ?, updated_at = ? WHERE id = ?")
            .bind(password_hash)
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Update last login time
    pub async fn update_last_login(&self, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE users SET last_login = ? WHERE id = ?")
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Delete user
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Convert SQLite row to User
    fn row_to_user(&self, row: &sqlx::sqlite::SqliteRow) -> Option<User> {
        Some(User {
            id: Uuid::parse_str(row.get("id")).ok()?,
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            name: row.get("name"),
            email: row.get("email"),
            role: UserRole::from_str(row.get("role")),
            enabled: row.get("enabled"),
            created_at: chrono::DateTime::parse_from_rfc3339(row.get("created_at"))
                .ok()?
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(row.get("updated_at"))
                .ok()?
                .with_timezone(&chrono::Utc),
            last_login: row
                .get::<Option<String>, _>("last_login")
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc)),
        })
    }
}
