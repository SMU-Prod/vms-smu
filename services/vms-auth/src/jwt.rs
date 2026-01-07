//! JWT token management

use anyhow::{Context, Result};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT Claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: Uuid,
    /// Username
    pub username: String,
    /// Issued at
    pub iat: i64,
    /// Expiration time
    pub exp: i64,
}

/// JWT Manager
pub struct JwtManager {
    secret: String,
    access_token_duration: i64,  // seconds
    refresh_token_duration: i64, // seconds
}

impl JwtManager {
    /// Create new JWT manager
    pub fn new(secret: String) -> Self {
        Self {
            secret,
            access_token_duration: 3600,      // 1 hour
            refresh_token_duration: 86400 * 7, // 7 days
        }
    }

    /// Generate access token
    pub fn generate_token(&self, user_id: Uuid, username: &str) -> Result<String> {
        let now = chrono::Utc::now().timestamp();
        let claims = Claims {
            sub: user_id,
            username: username.to_string(),
            iat: now,
            exp: now + self.access_token_duration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .context("Failed to generate token")
    }

    /// Generate refresh token
    pub fn generate_refresh_token(&self, user_id: Uuid) -> Result<String> {
        let now = chrono::Utc::now().timestamp();
        let claims = Claims {
            sub: user_id,
            username: String::new(), // Refresh token doesn't need username
            iat: now,
            exp: now + self.refresh_token_duration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .context("Failed to generate refresh token")
    }

    /// Validate token
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .context("Failed to validate token")?;

        Ok(token_data.claims)
    }
}

/// Axum extractor for Claims
/// The middleware injects the claims into request extensions
#[axum::async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract claims from request extensions (set by auth middleware)
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Missing authentication claims".to_string(),
                )
            })
    }
}
