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

        // In a real implementation, we would:
        // 1. Validate the tenant exists
        // 2. Get the user's role in this tenant from the JWT
        // 3. Check if the user has access to this tenant

        // For now, we'll use placeholder values
        // The actual user_id and role should come from the JWT token
        let user_id = parts
            .extensions
            .get::<Uuid>()
            .copied()
            .unwrap_or_else(Uuid::nil);

        let role = parts
            .extensions
            .get::<TenantRole>()
            .copied()
            .unwrap_or(TenantRole::Guest);

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
