//! Authentication service

use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use vms_core::{Claims, Role, User};
use uuid::Uuid;

pub struct AuthService {
    jwt_secret: String,
    expiry_hours: i64,
}

impl AuthService {
    pub fn new(jwt_secret: String, expiry_hours: i64) -> Self {
        Self { jwt_secret, expiry_hours }
    }

    /// Generate JWT access token
    pub fn generate_token(&self, user: &User) -> anyhow::Result<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.expiry_hours);

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            role: user.role,
            iat: now.timestamp(),
            exp: exp.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok(token)
    }

    /// Validate JWT token
    pub fn validate_token(&self, token: &str) -> anyhow::Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    /// Hash password with Argon2
    pub fn hash_password(password: &str) -> anyhow::Result<String> {
        use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
        use rand::rngs::OsRng;
        
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Hash error: {}", e))?;
        
        Ok(hash.to_string())
    }

    /// Verify password
    pub fn verify_password(password: &str, hash: &str) -> anyhow::Result<bool> {
        use argon2::{Argon2, PasswordVerifier, PasswordHash};
        
        let parsed = PasswordHash::new(hash)
            .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;
        
        Ok(Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok())
    }
}
