//! Tenant context extractor.

use crate::AuthUser;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use common::{TenantContext, TenantRole};
use uuid::Uuid;

/// Extractor for tenant context from X-Tenant-ID header.
///
/// SECURITY: This extractor automatically validates JWT authentication by extracting
/// AuthUser first. Routes using TenantExtractor do NOT need to also extract AuthUser.
/// The user_id and role are populated from the JWT claims.
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

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // SECURITY: Always extract and validate JWT authentication first
        // This ensures user_id and role are populated in extensions
        let auth_user = AuthUser::from_request_parts(parts, state).await?;

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

        // User ID comes from authenticated JWT (validated above)
        let user_id = auth_user.user_id;

        // Role comes from JWT claims, fall back to Guest (most restrictive)
        let role = auth_user.role.unwrap_or(TenantRole::Guest);

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
///
/// Returns None if authentication fails or tenant header is missing.
/// Useful for public endpoints that benefit from tenant context when available.
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
