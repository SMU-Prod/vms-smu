//! VMS Auth Service
//! Autenticação JWT + RBAC (Role-Based Access Control)

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber;

mod jwt;
mod middleware_auth;
mod password;
mod permission;
mod role;
mod user;

use jwt::{Claims, JwtManager};
use permission::{Permission, PermissionManager};
use role::{Role, RoleManager};
use user::{User, UserManager};

/// App state
#[derive(Clone)]
struct AppState {
    user_manager: Arc<UserManager>,
    role_manager: Arc<RoleManager>,
    permission_manager: Arc<PermissionManager>,
    jwt_manager: Arc<JwtManager>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🚀 VMS Auth Service starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Initialize managers
    let user_manager = Arc::new(UserManager::new());
    let role_manager = Arc::new(RoleManager::new());
    let permission_manager = Arc::new(PermissionManager::new());

    // JWT secret (em produção deve vir de env var)
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "vms-secret-key-change-in-production".to_string());
    let jwt_manager = Arc::new(JwtManager::new(jwt_secret));

    // Seed default roles and permissions
    seed_defaults(&role_manager, &permission_manager).await;

    // Create default admin user if not exists
    create_default_admin(&user_manager, &role_manager).await?;

    let state = AppState {
        user_manager,
        role_manager,
        permission_manager,
        jwt_manager,
    };

    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/metrics", get(metrics))
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/refresh", post(refresh_token));

    // Protected routes (auth required)
    let protected_routes = Router::new()
        // Users
        .route("/api/v1/users", get(list_users).post(create_user))
        .route(
            "/api/v1/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .route("/api/v1/users/:id/roles", post(assign_role))
        .route("/api/v1/users/:id/roles/:role_id", delete(remove_role))
        // Roles
        .route("/api/v1/roles", get(list_roles).post(create_role))
        .route(
            "/api/v1/roles/:id",
            get(get_role).put(update_role).delete(delete_role),
        )
        .route("/api/v1/roles/:id/permissions", post(assign_permission))
        .route(
            "/api/v1/roles/:id/permissions/:permission_id",
            delete(remove_permission),
        )
        // Permissions
        .route("/api/v1/permissions", get(list_permissions).post(create_permission))
        // Current user
        .route("/api/v1/auth/me", get(get_current_user))
        .route("/api/v1/auth/logout", post(logout))
        .layer(middleware::from_fn_with_state(
            state.jwt_manager.clone(),
            middleware_auth::auth_middleware,
        ));

    let app = public_routes.merge(protected_routes).with_state(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 9097));
    let listener = TcpListener::bind(addr).await?;

    info!("🌐 HTTP API listening on http://{}", addr);
    info!("✅ Service initialized successfully");
    info!("📋 Default admin: username='admin', password='admin123' (CHANGE THIS!)");

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
        })
        .await?;

    info!("👋 Goodbye!");
    Ok(())
}

// ============================================================================
// Seeding
// ============================================================================

async fn seed_defaults(role_manager: &RoleManager, permission_manager: &PermissionManager) {
    // Default permissions
    let permissions = vec![
        ("cameras.view", "View cameras"),
        ("cameras.edit", "Edit cameras"),
        ("cameras.delete", "Delete cameras"),
        ("recordings.view", "View recordings"),
        ("recordings.export", "Export recordings"),
        ("recordings.delete", "Delete recordings"),
        ("events.view", "View events"),
        ("events.acknowledge", "Acknowledge events"),
        ("alarms.view", "View alarms"),
        ("alarms.manage", "Manage alarms"),
        ("users.view", "View users"),
        ("users.manage", "Manage users"),
        ("system.settings", "System settings"),
    ];

    for (name, desc) in &permissions {
        if permission_manager.get_by_name(name).await.is_none() {
            let perm = Permission::new(name.to_string(), desc.to_string());
            permission_manager.add(perm).await;
        }
    }

    // Default roles
    if role_manager.get_by_name("admin").await.is_none() {
        let mut admin_role = Role::new("admin".to_string(), "Administrator".to_string());
        // Admin has all permissions
        for (name, _) in permissions.iter() {
            if let Some(perm) = permission_manager.get_by_name(name).await {
                admin_role.permissions.push(perm.id);
            }
        }
        role_manager.add(admin_role).await;
    }

    if role_manager.get_by_name("operator").await.is_none() {
        let mut operator_role = Role::new("operator".to_string(), "Operator".to_string());
        // Operator: view cameras, recordings, events
        let operator_perms = vec!["cameras.view", "recordings.view", "events.view", "alarms.view"];
        for name in operator_perms {
            if let Some(perm) = permission_manager.get_by_name(name).await {
                operator_role.permissions.push(perm.id);
            }
        }
        role_manager.add(operator_role).await;
    }

    if role_manager.get_by_name("viewer").await.is_none() {
        let mut viewer_role = Role::new("viewer".to_string(), "Viewer".to_string());
        // Viewer: only view cameras
        if let Some(perm) = permission_manager.get_by_name("cameras.view").await {
            viewer_role.permissions.push(perm.id);
        }
        role_manager.add(viewer_role).await;
    }

    info!("✅ Default roles and permissions seeded");
}

async fn create_default_admin(
    user_manager: &UserManager,
    role_manager: &RoleManager,
) -> Result<()> {
    if user_manager.get_by_username("admin").await.is_none() {
        let mut admin_user = User::new("admin".to_string(), "admin@vms.local".to_string());
        admin_user.set_password("admin123")?;

        if let Some(admin_role) = role_manager.get_by_name("admin").await {
            admin_user.roles.push(admin_role.id);
        }

        user_manager.add(admin_user).await;
        info!("✅ Default admin user created");
    }

    Ok(())
}

// ============================================================================
// API Handlers
// ============================================================================

async fn metrics() -> String {
    "# Auth metrics\nvms_auth_users_total 0\n".to_string()
}

// Auth
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    refresh_token: String,
    user: User,
}

async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    // Find user
    let user = state
        .user_manager
        .get_by_username(&req.username)
        .await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    // Verify password
    if !user.verify_password(&req.password) {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    // Check if user is active
    if !user.active {
        return Err((StatusCode::FORBIDDEN, "User account is disabled".to_string()));
    }

    // Generate tokens
    let token = state
        .jwt_manager
        .generate_token(user.id, &user.username)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let refresh_token = state
        .jwt_manager
        .generate_refresh_token(user.id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    info!("✅ User logged in: {}", user.username);

    Ok(Json(LoginResponse {
        token,
        refresh_token,
        user,
    }))
}

#[derive(Deserialize)]
struct RefreshRequest {
    refresh_token: String,
}

async fn refresh_token(
    State(state): State<AppState>,
    Json(req): Json<RefreshRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    // Validate refresh token
    let claims = state
        .jwt_manager
        .validate_token(&req.refresh_token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid refresh token".to_string()))?;

    // Get user
    let user = state
        .user_manager
        .get(claims.sub)
        .await
        .ok_or_else(|| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    // Generate new tokens
    let token = state
        .jwt_manager
        .generate_token(user.id, &user.username)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let refresh_token = state
        .jwt_manager
        .generate_refresh_token(user.id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(LoginResponse {
        token,
        refresh_token,
        user,
    }))
}

async fn logout() -> String {
    // TODO: Invalidate token (add to blacklist in Redis)
    "Logged out".to_string()
}

async fn get_current_user(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<User>, (StatusCode, String)> {
    state
        .user_manager
        .get(claims.sub)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "User not found".to_string()))
}

// Users
async fn list_users(State(state): State<AppState>) -> Json<Vec<User>> {
    Json(state.user_manager.list_all().await)
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<User>, (StatusCode, String)> {
    state
        .user_manager
        .get(id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "User not found".to_string()))
}

async fn create_user(
    State(state): State<AppState>,
    Json(user): Json<User>,
) -> Result<Json<User>, (StatusCode, String)> {
    // Set password if provided
    // In real implementation, password would be in separate field
    state.user_manager.add(user.clone()).await;
    Ok(Json(user))
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(user): Json<User>,
) -> Result<Json<User>, (StatusCode, String)> {
    state
        .user_manager
        .update(id, user.clone())
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;
    Ok(Json(user))
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<String, (StatusCode, String)> {
    state
        .user_manager
        .delete(id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;
    Ok("Deleted".to_string())
}

async fn assign_role(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(role_id): Json<uuid::Uuid>,
) -> Result<Json<User>, (StatusCode, String)> {
    state
        .user_manager
        .assign_role(id, role_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    state
        .user_manager
        .get(id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "User not found".to_string()))
}

async fn remove_role(
    State(state): State<AppState>,
    Path((id, role_id)): Path<(uuid::Uuid, uuid::Uuid)>,
) -> Result<Json<User>, (StatusCode, String)> {
    state
        .user_manager
        .remove_role(id, role_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    state
        .user_manager
        .get(id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "User not found".to_string()))
}

// Roles
async fn list_roles(State(state): State<AppState>) -> Json<Vec<Role>> {
    Json(state.role_manager.list_all().await)
}

async fn get_role(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Role>, (StatusCode, String)> {
    state
        .role_manager
        .get(id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Role not found".to_string()))
}

async fn create_role(
    State(state): State<AppState>,
    Json(role): Json<Role>,
) -> Json<Role> {
    state.role_manager.add(role.clone()).await;
    Json(role)
}

async fn update_role(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(role): Json<Role>,
) -> Result<Json<Role>, (StatusCode, String)> {
    state
        .role_manager
        .update(id, role.clone())
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;
    Ok(Json(role))
}

async fn delete_role(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<String, (StatusCode, String)> {
    state
        .role_manager
        .delete(id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;
    Ok("Deleted".to_string())
}

async fn assign_permission(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(permission_id): Json<uuid::Uuid>,
) -> Result<Json<Role>, (StatusCode, String)> {
    state
        .role_manager
        .assign_permission(id, permission_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    state
        .role_manager
        .get(id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Role not found".to_string()))
}

async fn remove_permission(
    State(state): State<AppState>,
    Path((id, permission_id)): Path<(uuid::Uuid, uuid::Uuid)>,
) -> Result<Json<Role>, (StatusCode, String)> {
    state
        .role_manager
        .remove_permission(id, permission_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    state
        .role_manager
        .get(id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Role not found".to_string()))
}

// Permissions
async fn list_permissions(State(state): State<AppState>) -> Json<Vec<Permission>> {
    Json(state.permission_manager.list_all().await)
}

async fn create_permission(
    State(state): State<AppState>,
    Json(permission): Json<Permission>,
) -> Json<Permission> {
    state.permission_manager.add(permission.clone()).await;
    Json(permission)
}
