//! Authentication routes

use axum::{
    extract::State,
    routing::post,
    Router, Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use uuid::Uuid;
use vms_core::{User, LoginResponse};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
}

/// Login request (email instead of username for consistency)
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Refresh request
#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

/// Token response
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: User,
}

/// Error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Login endpoint
async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Find user by email
    let (user, password_hash) = state.user_repo
        .find_by_email(&req.email)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: "Invalid credentials".to_string() })))?;

    // Check if user is enabled
    if !user.enabled {
        return Err((StatusCode::FORBIDDEN, Json(ErrorResponse { error: "Account disabled".to_string() })));
    }

    // Verify password
    let valid = crate::services::AuthService::verify_password(&req.password, &password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    if !valid {
        return Err((StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: "Invalid credentials".to_string() })));
    }

    // Generate access token
    let access_token = state.auth_service.generate_token(&user)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    // Generate refresh token (random UUID)
    let refresh_token = Uuid::new_v4().to_string();
    
    // Store refresh token
    state.refresh_repo.create(user.id, &refresh_token, 30) // 30 days
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    tracing::info!("User {} logged in", user.email);

    Ok(Json(TokenResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: 24 * 3600, // 24 hours in seconds
        user,
    }))
}

/// Refresh access token
async fn refresh(
    State(state): State<AppState>,
    Json(req): Json<RefreshRequest>,
) -> Result<Json<TokenResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate refresh token
    let user_id = state.refresh_repo
        .validate(&req.refresh_token)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: "Invalid refresh token".to_string() })))?;

    // Get user
    let user = state.user_repo
        .find_by_id(user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: "User not found".to_string() })))?;

    // Check if user is still enabled
    if !user.enabled {
        return Err((StatusCode::FORBIDDEN, Json(ErrorResponse { error: "Account disabled".to_string() })));
    }

    // Revoke old refresh token
    state.refresh_repo.revoke(&req.refresh_token).await.ok();

    // Generate new tokens
    let access_token = state.auth_service.generate_token(&user)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    let new_refresh_token = Uuid::new_v4().to_string();
    state.refresh_repo.create(user.id, &new_refresh_token, 30)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    Ok(Json(TokenResponse {
        access_token,
        refresh_token: new_refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: 24 * 3600,
        user,
    }))
}

/// Logout (revoke refresh token)
async fn logout(
    State(state): State<AppState>,
    Json(req): Json<RefreshRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    state.refresh_repo.revoke(&req.refresh_token)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    Ok(StatusCode::NO_CONTENT)
}
