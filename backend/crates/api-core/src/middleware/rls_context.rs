//! RLS (Row-Level Security) context middleware.
//!
//! # DEPRECATED
//!
//! **This middleware approach is fundamentally flawed and should NOT be used.**
//!
//! Problems with middleware-based RLS:
//! 1. Middleware runs before extractors, so user/tenant context isn't available
//! 2. Setting RLS on the pool doesn't affect individual connections
//! 3. PostgreSQL session settings are connection-scoped, not pool-scoped
//!
//! # Correct Approach
//!
//! Use the `RlsConnection` extractor instead:
//!
//! ```rust,ignore
//! use api_core::extractors::RlsConnection;
//!
//! async fn handler(mut rls: RlsConnection) -> Result<Json<Data>, StatusCode> {
//!     // RlsConnection:
//!     // 1. Validates authentication (via ValidatedTenantExtractor)
//!     // 2. Acquires a dedicated connection from the pool
//!     // 3. Sets RLS context on THAT specific connection
//!     // 4. Provides the connection for queries
//!
//!     let items = sqlx::query_as("SELECT * FROM items")
//!         .fetch_all(rls.conn())
//!         .await?;
//!     Ok(Json(items))
//! }
//! ```
//!
//! See `crate::extractors::RlsConnection` for the correct implementation.
//!
//! ---
//!
//! The code below is kept for reference but should not be used.

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use db::DbPool;
use std::sync::Arc;
use tracing::{error, warn};
use uuid::Uuid;

/// Extension to store RLS context information for a request.
#[derive(Debug, Clone)]
pub struct RlsContext {
    pub org_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub is_super_admin: bool,
}

/// Middleware to set RLS context for database operations.
///
/// This middleware:
/// 1. Extracts authentication info from request extensions (set by AuthUser extractor)
/// 2. Extracts tenant ID from X-Tenant-ID header
/// 3. Calls `set_request_context()` on the database before the handler
/// 4. Calls `clear_request_context()` after the response is sent
///
/// # Skipped Paths
///
/// Public paths (auth, health, metrics, swagger) are skipped and don't set RLS context.
pub async fn rls_context_middleware(
    State(pool): State<Arc<DbPool>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = request.uri().path();

    // Skip RLS context for public endpoints
    if is_public_path(path) {
        return Ok(next.run(request).await);
    }

    // Extract user_id from request extensions (set by AuthUser extractor in previous middleware)
    // Note: We check extensions because AuthUser stores user_id there during extraction
    let user_id = request.extensions().get::<Uuid>().copied();

    // Extract tenant_id from X-Tenant-ID header
    let tenant_id = request
        .headers()
        .get("X-Tenant-ID")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok());

    // Determine if user is super admin (check role in extensions)
    let is_super_admin = request
        .extensions()
        .get::<common::TenantRole>()
        .map(|role| {
            matches!(
                role,
                common::TenantRole::SuperAdmin | common::TenantRole::PlatformAdmin
            )
        })
        .unwrap_or(false);

    // If we have auth context, set RLS
    let should_set_rls = user_id.is_some() || tenant_id.is_some();

    if should_set_rls {
        // Set RLS context before handler
        if let Err(e) = db::tenant_context::set_request_context(
            pool.as_ref(),
            tenant_id,
            user_id,
            is_super_admin,
        )
        .await
        {
            error!(
                error = %e,
                tenant_id = ?tenant_id,
                user_id = ?user_id,
                "Failed to set RLS context"
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        tracing::debug!(
            tenant_id = ?tenant_id,
            user_id = ?user_id,
            is_super_admin = is_super_admin,
            path = %path,
            "RLS context set"
        );
    }

    // Store RLS context in extensions for handlers that need it
    request.extensions_mut().insert(RlsContext {
        org_id: tenant_id,
        user_id,
        is_super_admin,
    });

    // Run the handler
    let response = next.run(request).await;

    // Clear RLS context after handler (even on error responses)
    if should_set_rls {
        if let Err(e) = db::tenant_context::clear_request_context(pool.as_ref()).await {
            // Log but don't fail - the response is already being sent
            warn!(error = %e, "Failed to clear RLS context");
        }
    }

    Ok(response)
}

/// Check if path is public (doesn't require RLS context).
fn is_public_path(path: &str) -> bool {
    let public_prefixes = [
        "/api/v1/auth/",
        "/health",
        "/ready",
        "/metrics",
        "/swagger-ui",
        "/api-docs",
        // OAuth callback endpoints
        "/api/v1/oauth/callback",
        // Public feature flags
        "/api/v1/feature-flags",
        // Public system announcements
        "/api/v1/system-announcements",
        // Public maintenance status
        "/api/v1/maintenance",
    ];

    public_prefixes
        .iter()
        .any(|prefix| path.starts_with(prefix))
}

/// Middleware that validates tenant context and sets RLS.
///
/// This is a stricter version that requires both authentication AND tenant context.
/// Use this for routes that MUST have tenant isolation.
pub async fn require_rls_context(
    State(pool): State<Arc<DbPool>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, Response> {
    let path = request.uri().path();

    // Skip for public paths
    if is_public_path(path) {
        return Ok(next.run(request).await);
    }

    // Extract user_id - REQUIRED for this middleware
    let user_id = request.extensions().get::<Uuid>().copied().ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            "Authentication required for this resource",
        )
            .into_response()
    })?;

    // Extract tenant_id - REQUIRED for this middleware
    let tenant_id = request
        .headers()
        .get("X-Tenant-ID")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "X-Tenant-ID header required for this resource",
            )
                .into_response()
        })?;

    let is_super_admin = request
        .extensions()
        .get::<common::TenantRole>()
        .map(|role| {
            matches!(
                role,
                common::TenantRole::SuperAdmin | common::TenantRole::PlatformAdmin
            )
        })
        .unwrap_or(false);

    // Set RLS context
    db::tenant_context::set_request_context(
        pool.as_ref(),
        Some(tenant_id),
        Some(user_id),
        is_super_admin,
    )
    .await
    .map_err(|e| {
        error!(error = %e, "Failed to set required RLS context");
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
    })?;

    // Run handler
    let response = next.run(request).await;

    // Clear context
    let _ = db::tenant_context::clear_request_context(pool.as_ref()).await;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_paths() {
        assert!(is_public_path("/api/v1/auth/login"));
        assert!(is_public_path("/api/v1/auth/register"));
        assert!(is_public_path("/health"));
        assert!(is_public_path("/metrics"));
        assert!(is_public_path("/swagger-ui/index.html"));

        assert!(!is_public_path("/api/v1/buildings"));
        assert!(!is_public_path("/api/v1/organizations"));
        assert!(!is_public_path("/api/v1/faults"));
    }
}
