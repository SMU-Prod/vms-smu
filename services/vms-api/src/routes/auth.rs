//! Authentication and User API routes

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::user_repository::UserRepository,
    models::user::{
        ChangePasswordRequest, CreateUserRequest, LoginRequest, LoginResponse, UpdateUserRequest,
        User, UserPublic, UserRole,
    },
    AppState,
};

/// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,       // user id
    pub username: String,
    pub role: String,
    pub exp: i64,          // expiration time
    pub iat: i64,          // issued at
}

/// JWT secret (in production, use environment variable)
const JWT_SECRET: &[u8] = b"vms-enterprise-secret-key-change-in-production";
const JWT_EXPIRATION_HOURS: i64 = 24;

/// POST /api/v1/auth/login - User login
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    // Find user
    let user = match state.user_repo.get_by_username(&req.username).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": "Invalid credentials" })),
            )
                .into_response()
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response()
        }
    };

    // Check if enabled
    if !user.enabled {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({ "error": "User account is disabled" })),
        )
            .into_response();
    }

    // Verify password
    if !UserRepository::verify_password(&req.password, &user.password_hash) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "Invalid credentials" })),
        )
            .into_response();
    }

    // Update last login
    let _ = state.user_repo.update_last_login(user.id).await;

    // Generate JWT
    let now = Utc::now();
    let expires_at = now + Duration::hours(JWT_EXPIRATION_HOURS);

    let claims = Claims {
        sub: user.id.to_string(),
        username: user.username.clone(),
        role: user.role.as_str().to_string(),
        exp: expires_at.timestamp(),
        iat: now.timestamp(),
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    ) {
        Ok(t) => t,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("Failed to create token: {}", e) })),
            )
                .into_response()
        }
    };

    let response = LoginResponse {
        token,
        user: UserPublic::from(user),
        expires_at,
    };

    (StatusCode::OK, Json(response)).into_response()
}

/// POST /api/v1/users - Create new user (admin only)
pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> impl IntoResponse {
    // Hash password
    let password_hash = match UserRepository::hash_password(&req.password) {
        Ok(h) => h,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response()
        }
    };

    let mut user = User::new(req.username, password_hash, req.name, req.role);
    user.email = req.email;

    match state.user_repo.create(&user).await {
        Ok(_) => (StatusCode::CREATED, Json(UserPublic::from(user))).into_response(),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("UNIQUE constraint failed") {
                (
                    StatusCode::CONFLICT,
                    Json(serde_json::json!({ "error": "Username already exists" })),
                )
                    .into_response()
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": error_msg })),
                )
                    .into_response()
            }
        }
    }
}

/// GET /api/v1/users - List all users (admin only)
pub async fn list_users(State(state): State<AppState>) -> impl IntoResponse {
    match state.user_repo.list().await {
        Ok(users) => {
            let public_users: Vec<UserPublic> = users.into_iter().map(UserPublic::from).collect();
            (StatusCode::OK, Json(public_users)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// GET /api/v1/users/:id - Get user by ID
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.user_repo.get(id).await {
        Ok(Some(user)) => (StatusCode::OK, Json(UserPublic::from(user))).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "User not found" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// PUT /api/v1/users/:id - Update user
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> impl IntoResponse {
    match state
        .user_repo
        .update(id, req.name, req.email.map(Some), req.role, req.enabled)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// DELETE /api/v1/users/:id - Delete user
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    // Don't allow deleting the last admin
    if let Ok(users) = state.user_repo.list().await {
        let admins: Vec<_> = users.iter().filter(|u| u.role == UserRole::Admin).collect();
        if admins.len() == 1 && admins[0].id == id {
            return (
                StatusCode::FORBIDDEN,
                Json(serde_json::json!({ "error": "Cannot delete the last admin user" })),
            )
                .into_response();
        }
    }

    match state.user_repo.delete(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// POST /api/v1/users/:id/password - Change password
pub async fn change_password(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ChangePasswordRequest>,
) -> impl IntoResponse {
    // Get user
    let user = match state.user_repo.get(id).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "User not found" })),
            )
                .into_response()
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response()
        }
    };

    // Verify current password
    if !UserRepository::verify_password(&req.current_password, &user.password_hash) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "Current password is incorrect" })),
        )
            .into_response();
    }

    // Hash new password
    let new_hash = match UserRepository::hash_password(&req.new_password) {
        Ok(h) => h,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response()
        }
    };

    // Update password
    match state.user_repo.update_password(id, new_hash).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Password changed successfully" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
