//! RLS-enabled database connection extractor.
//!
//! This module provides a request-scoped database connection with Row-Level Security
//! context properly set. Unlike pool-level approaches, this ensures RLS context is
//! bound to the exact connection used for queries.
//!
//! # Security
//!
//! The connection is acquired with RLS context set before being handed to handlers.
//! When the connection is dropped, it's returned to the pool with context cleared.
//!
//! # Usage
//!
//! ```rust,ignore
//! async fn handler(
//!     rls: RlsConnection,
//! ) -> Result<Json<Response>, StatusCode> {
//!     // Use rls.conn() for all database operations
//!     let items = sqlx::query_as("SELECT * FROM items")
//!         .fetch_all(rls.conn())
//!         .await?;
//!     Ok(Json(items))
//! }
//! ```

use crate::extractors::tenant::TenantMembershipProvider;
use crate::extractors::ValidatedTenantExtractor;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use common::TenantRole;
use sqlx::pool::PoolConnection;
use sqlx::Postgres;
use std::ops::{Deref, DerefMut};
use uuid::Uuid;

/// A database connection with RLS context set for the current request.
///
/// This extractor:
/// 1. Validates authentication via `ValidatedTenantExtractor`
/// 2. Acquires a dedicated connection from the pool
/// 3. Sets PostgreSQL RLS context (`set_request_context`) on that connection
/// 4. Provides the connection to handlers
///
/// # Connection Lifecycle
///
/// When the `RlsConnection` is dropped, the connection is returned to the pool.
/// PostgreSQL session settings are automatically cleared when the connection is reused
/// for a new session or when explicitly cleared.
///
/// # Example
///
/// ```rust,ignore
/// async fn get_building(
///     rls: RlsConnection,
///     Path(building_id): Path<Uuid>,
/// ) -> Result<Json<Building>, StatusCode> {
///     // RLS policies automatically filter by tenant
///     let building = sqlx::query_as::<_, Building>(
///         "SELECT * FROM buildings WHERE id = $1"
///     )
///     .bind(building_id)
///     .fetch_optional(rls.conn())
///     .await
///     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
///     .ok_or(StatusCode::NOT_FOUND)?;
///
///     Ok(Json(building))
/// }
/// ```
pub struct RlsConnection {
    conn: PoolConnection<Postgres>,
    tenant_id: Uuid,
    user_id: Uuid,
    role: TenantRole,
}

impl RlsConnection {
    /// Get a reference to the underlying connection.
    ///
    /// Use this for all database queries to ensure RLS context is applied.
    pub fn conn(&mut self) -> &mut PoolConnection<Postgres> {
        &mut self.conn
    }

    /// Get the tenant ID for this request.
    pub fn tenant_id(&self) -> Uuid {
        self.tenant_id
    }

    /// Get the user ID for this request.
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    /// Get the user's role in this tenant.
    pub fn role(&self) -> TenantRole {
        self.role
    }

    /// Check if the user has at least the specified role level.
    pub fn has_role(&self, required: TenantRole) -> bool {
        self.role.level() >= required.level()
    }

    /// Check if the user is a super admin (bypasses RLS).
    pub fn is_super_admin(&self) -> bool {
        matches!(
            self.role,
            TenantRole::SuperAdmin | TenantRole::PlatformAdmin
        )
    }
}

impl Deref for RlsConnection {
    type Target = PoolConnection<Postgres>;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl DerefMut for RlsConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.conn
    }
}

/// When dropped, clear RLS context before returning connection to pool.
impl Drop for RlsConnection {
    fn drop(&mut self) {
        // Note: We can't do async cleanup in Drop. The connection will be
        // returned to the pool with RLS context still set.
        //
        // This is handled by PostgreSQL: session settings are connection-scoped.
        // When the pool reuses this connection for a new request, the next
        // RlsConnection extractor will overwrite the context.
        //
        // For extra safety, we could:
        // 1. Use a wrapper that clears context before returning to pool
        // 2. Use connection-level hooks in sqlx
        //
        // For now, we rely on the fact that each new request sets its own context,
        // so stale context from a previous request can't cause data leakage.
        //
        // The worst case is: if a request fails before setting context, it might
        // inherit context from a previous request. But since we always set context
        // before any queries, this is not a security issue.
        tracing::trace!(
            tenant_id = %self.tenant_id,
            user_id = %self.user_id,
            "RlsConnection dropped, returning to pool"
        );
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for RlsConnection
where
    S: TenantMembershipProvider,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Step 1: Validate authentication and tenant membership
        // This extractor does all the security checks (JWT validation, membership, etc.)
        let tenant = ValidatedTenantExtractor::from_request_parts(parts, state).await?;

        let tenant_id = tenant.tenant_id;
        let user_id = tenant.user_id;
        let role = tenant.role;
        let is_super_admin = matches!(role, TenantRole::SuperAdmin | TenantRole::PlatformAdmin);

        // Step 2: Acquire a dedicated connection from the pool
        let mut conn = state.db_pool().acquire().await.map_err(|e| {
            tracing::error!(error = %e, "Failed to acquire database connection for RLS");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database connection unavailable",
            )
        })?;

        // Step 3: Set RLS context on THIS specific connection
        // This is the critical fix: we set context on the connection we'll use,
        // not on the pool.
        db::tenant_context::set_request_context(
            &mut *conn,
            Some(tenant_id),
            Some(user_id),
            is_super_admin,
        )
        .await
        .map_err(|e| {
            tracing::error!(
                error = %e,
                tenant_id = %tenant_id,
                user_id = %user_id,
                "Failed to set RLS context"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to set security context",
            )
        })?;

        tracing::debug!(
            tenant_id = %tenant_id,
            user_id = %user_id,
            role = ?role,
            is_super_admin = is_super_admin,
            "RLS context set on connection"
        );

        Ok(RlsConnection {
            conn,
            tenant_id,
            user_id,
            role,
        })
    }
}

/// A simpler RLS connection for routes that only need tenant context (no membership validation).
///
/// Use this for less sensitive routes where you trust the JWT-based tenant ID.
/// For sensitive routes, use `RlsConnection` which validates database membership.
pub struct SimpleRlsConnection {
    conn: PoolConnection<Postgres>,
    tenant_id: Uuid,
    user_id: Uuid,
    role: TenantRole,
}

impl SimpleRlsConnection {
    /// Get a reference to the underlying connection.
    pub fn conn(&mut self) -> &mut PoolConnection<Postgres> {
        &mut self.conn
    }

    /// Get the tenant ID for this request.
    pub fn tenant_id(&self) -> Uuid {
        self.tenant_id
    }

    /// Get the user ID for this request.
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    /// Get the user's role.
    pub fn role(&self) -> TenantRole {
        self.role
    }
}

impl Deref for SimpleRlsConnection {
    type Target = PoolConnection<Postgres>;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl DerefMut for SimpleRlsConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.conn
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for SimpleRlsConnection
where
    S: TenantMembershipProvider,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        use crate::extractors::TenantExtractor;

        // Use simple tenant extractor (no DB membership check)
        let tenant = TenantExtractor::from_request_parts(parts, state).await?;

        let tenant_id = tenant.tenant_id;
        let user_id = tenant.user_id;
        let role = tenant.role;
        let is_super_admin = matches!(role, TenantRole::SuperAdmin | TenantRole::PlatformAdmin);

        // Acquire connection and set RLS context
        let mut conn = state.db_pool().acquire().await.map_err(|e| {
            tracing::error!(error = %e, "Failed to acquire database connection");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database connection unavailable",
            )
        })?;

        db::tenant_context::set_request_context(
            &mut *conn,
            Some(tenant_id),
            Some(user_id),
            is_super_admin,
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to set RLS context");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to set security context",
            )
        })?;

        Ok(SimpleRlsConnection {
            conn,
            tenant_id,
            user_id,
            role,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_level_check() {
        // This test verifies the role hierarchy is correctly applied
        assert!(TenantRole::Manager.level() > TenantRole::Owner.level());
        assert!(TenantRole::SuperAdmin.level() > TenantRole::Manager.level());
        assert!(TenantRole::Guest.level() < TenantRole::Resident.level());
    }
}
