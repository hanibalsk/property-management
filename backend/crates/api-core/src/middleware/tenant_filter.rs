//! Tenant isolation middleware.

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tracing::warn;
use uuid::Uuid;

/// Middleware to ensure tenant isolation.
/// Validates that the X-Tenant-ID header is present and valid for protected routes.
pub async fn tenant_filter(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let path = request.uri().path();

    // Skip tenant validation for public endpoints
    if is_public_path(path) {
        return Ok(next.run(request).await);
    }

    // Get and validate X-Tenant-ID header
    let tenant_id = request
        .headers()
        .get("X-Tenant-ID")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok());

    match tenant_id {
        Some(_id) => {
            // In production, validate:
            // 1. Tenant exists and is active
            // 2. User has access to this tenant
            // 3. Rate limiting per tenant
            Ok(next.run(request).await)
        }
        None => {
            warn!(path = %path, "Request missing X-Tenant-ID header");
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

/// Check if path is public (doesn't require tenant context).
fn is_public_path(path: &str) -> bool {
    let public_prefixes = [
        "/api/v1/auth/",
        "/health",
        "/ready",
        "/metrics",
        "/swagger-ui",
        "/api-docs",
    ];

    public_prefixes.iter().any(|prefix| path.starts_with(prefix))
}

/// Middleware to log requests with tenant context.
pub async fn tenant_logging(request: Request<Body>, next: Next) -> Response {
    let tenant_id = request
        .headers()
        .get("X-Tenant-ID")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "none".to_string());

    let method = request.method().clone();
    let uri = request.uri().clone();

    tracing::info!(
        tenant_id = %tenant_id,
        method = %method,
        uri = %uri,
        "Request"
    );

    let response = next.run(request).await;

    tracing::info!(
        tenant_id = %tenant_id,
        status = %response.status(),
        "Response"
    );

    response
}
