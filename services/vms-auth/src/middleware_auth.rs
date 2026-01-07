//! Authentication middleware

use super::jwt::JwtManager;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

/// Authentication middleware
pub async fn auth_middleware(
    State(jwt_manager): State<Arc<JwtManager>>,
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    // Extract Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                "Missing Authorization header".to_string(),
            )
        })?;

    // Extract token from "Bearer <token>"
    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            "Invalid Authorization header format".to_string(),
        )
    })?;

    // Validate token
    let claims = jwt_manager.validate_token(token).map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            format!("Invalid token: {}", e),
        )
    })?;

    // Insert claims into request extensions
    req.extensions_mut().insert(claims);

    // Continue to next middleware/handler
    Ok(next.run(req).await)
}
