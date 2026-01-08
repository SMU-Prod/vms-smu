//! Auth middleware for protected routes

use axum::{
    extract::Request,
    middleware::Next,
    response::{Response, IntoResponse},
    http::StatusCode,
};

/// Middleware to validate JWT token
pub async fn auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get Authorization header
    let auth_header = request.headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            let _token = &header[7..];
            // TODO: Validate token and extract claims
            // TODO: Add claims to request extensions
            Ok(next.run(request).await)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

/// Middleware to require admin role
pub async fn require_admin(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // TODO: Check claims for admin role
    Ok(next.run(request).await)
}
