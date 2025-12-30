//! Tenant context extractor.

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use common::{TenantContext, TenantRole};
use uuid::Uuid;

/// Extractor for tenant context from X-Tenant-ID header.
#[derive(Debug, Clone)]
pub struct TenantExtractor(pub TenantContext);

impl std::ops::Deref for TenantExtractor {
    type Target = TenantContext;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for TenantExtractor
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Get X-Tenant-ID header
        let tenant_id = parts
            .headers
            .get("X-Tenant-ID")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| Uuid::parse_str(s).ok())
            .ok_or((
                StatusCode::BAD_REQUEST,
                "Missing or invalid X-Tenant-ID header",
            ))?;

        // SECURITY: User ID must come from authenticated JWT (stored in extensions by AuthUser extractor)
        // Never fall back to a placeholder - this prevents unauthorized access
        let user_id = parts.extensions.get::<Uuid>().copied().ok_or((
            StatusCode::UNAUTHORIZED,
            "Authentication required for tenant access",
        ))?;

        // SECURITY: Role must come from JWT or be validated against database
        // Fall back to Guest only if no role is present (most restrictive)
        let role = parts
            .extensions
            .get::<TenantRole>()
            .copied()
            .unwrap_or(TenantRole::Guest);

        // TODO: For production, add database validation:
        // 1. Verify tenant_id exists in database
        // 2. Verify user has membership in the tenant
        // 3. Verify the role matches their actual permissions
        // This requires access to the database pool via State

        Ok(TenantExtractor(TenantContext::new(
            tenant_id, user_id, role,
        )))
    }
}

/// Optional tenant extractor (for endpoints that work with or without tenant context).
#[derive(Debug, Clone)]
pub struct OptionalTenant(pub Option<TenantContext>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalTenant
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match TenantExtractor::from_request_parts(parts, state).await {
            Ok(TenantExtractor(ctx)) => Ok(OptionalTenant(Some(ctx))),
            Err(_) => Ok(OptionalTenant(None)),
        }
    }
}
