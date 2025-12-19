//! Route handlers for auth server.

use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Health check response.
#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    status: String,
    version: String,
}

/// Health check endpoint.
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Login request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    /// Email address
    pub email: String,
    /// Password
    pub password: String,
    /// 2FA code (optional)
    pub two_factor_code: Option<String>,
}

/// Login response.
#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    /// JWT access token
    pub access_token: String,
    /// Refresh token
    pub refresh_token: String,
    /// Token expiration in seconds
    pub expires_in: i32,
}

/// Login endpoint.
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
pub async fn login(Json(req): Json<LoginRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    // TODO: Implement actual authentication
    tracing::info!(email = %req.email, "Login attempt");

    // Placeholder response
    Ok(Json(LoginResponse {
        access_token: "placeholder-token".to_string(),
        refresh_token: "placeholder-refresh".to_string(),
        expires_in: 3600,
    }))
}

/// Register request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    /// Email address
    pub email: String,
    /// Password (min 8 characters)
    pub password: String,
    /// Display name
    pub display_name: String,
    /// Phone number (optional)
    pub phone: Option<String>,
    /// Invitation code (optional)
    pub invitation_code: Option<String>,
}

/// Register endpoint.
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "Authentication",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration successful"),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "Email already exists")
    )
)]
pub async fn register(Json(req): Json<RegisterRequest>) -> Result<StatusCode, StatusCode> {
    // TODO: Implement actual registration
    tracing::info!(email = %req.email, name = %req.display_name, "Registration attempt");

    Ok(StatusCode::CREATED)
}
