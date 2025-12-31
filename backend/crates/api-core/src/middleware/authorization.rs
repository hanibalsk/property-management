//! Authorization middleware for role-based access control.
//!
//! Provides middleware for enforcing permission requirements on routes.
//! Works with the ValidatedTenantExtractor to ensure proper authorization.

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use common::TenantRole;
use std::collections::HashSet;

/// Permission requirements for route access.
///
/// Use this to define what roles can access specific routes.
#[derive(Debug, Clone)]
pub struct Permission {
    /// Minimum role level required (based on TenantRole hierarchy)
    pub min_role_level: u8,
    /// Specific roles allowed (if Some, overrides min_role_level)
    pub allowed_roles: Option<HashSet<TenantRole>>,
    /// Description for logging/documentation
    pub description: &'static str,
}

impl Permission {
    /// Create a permission requiring minimum role level.
    pub const fn min_level(level: u8, description: &'static str) -> Self {
        Self {
            min_role_level: level,
            allowed_roles: None,
            description,
        }
    }

    /// Check if a role satisfies this permission.
    pub fn is_satisfied_by(&self, role: &TenantRole) -> bool {
        if let Some(ref allowed) = self.allowed_roles {
            allowed.contains(role)
        } else {
            role.level() >= self.min_role_level
        }
    }
}

/// Common permission levels based on TenantRole hierarchy.
pub mod permissions {
    use super::Permission;

    /// Super admin only (level 100)
    pub const SUPER_ADMIN: Permission = Permission::min_level(100, "Super admin access");

    /// Platform admin and above (level 95+)
    pub const PLATFORM_ADMIN: Permission = Permission::min_level(95, "Platform admin access");

    /// Organization admin and above (level 90+)
    pub const ORG_ADMIN: Permission = Permission::min_level(90, "Organization admin access");

    /// Manager and above (level 80+)
    pub const MANAGER: Permission = Permission::min_level(80, "Manager access");

    /// Technical manager and above (level 75+)
    pub const TECHNICAL_MANAGER: Permission = Permission::min_level(75, "Technical manager access");

    /// Owner and above (level 60+)
    pub const OWNER: Permission = Permission::min_level(60, "Owner access");

    /// Owner delegate and above (level 55+)
    pub const OWNER_DELEGATE: Permission = Permission::min_level(55, "Owner delegate access");

    /// Tenant/Resident and above (level 40+)
    pub const RESIDENT: Permission = Permission::min_level(40, "Resident access");

    /// Property manager and above (level 30+)
    pub const PROPERTY_MANAGER: Permission = Permission::min_level(30, "Property manager access");

    /// Any authenticated user (level 1+)
    pub const AUTHENTICATED: Permission = Permission::min_level(1, "Any authenticated user");
}

/// Authorization error response.
#[derive(Debug)]
pub struct AuthorizationError {
    pub status: StatusCode,
    pub message: &'static str,
}

impl IntoResponse for AuthorizationError {
    fn into_response(self) -> Response {
        (self.status, self.message).into_response()
    }
}

/// Middleware layer that requires a specific permission.
///
/// This is designed to be used with `axum::middleware::from_fn_with_state`
/// for routes that need permission checks.
///
/// # Example
/// ```ignore
/// use api_core::middleware::authorization::{require_permission, permissions};
///
/// let protected_routes = Router::new()
///     .route("/admin", get(admin_handler))
///     .layer(middleware::from_fn(|req, next| {
///         require_permission(permissions::MANAGER, req, next)
///     }));
/// ```
pub async fn require_permission(
    required: Permission,
    request: Request<Body>,
    next: Next,
) -> Result<Response, AuthorizationError> {
    // Get role from request extensions (set by TenantExtractor or ValidatedTenantExtractor)
    let role = request
        .extensions()
        .get::<TenantRole>()
        .cloned()
        .unwrap_or(TenantRole::Guest);

    if required.is_satisfied_by(&role) {
        tracing::debug!(
            role = ?role,
            permission = required.description,
            "Authorization granted"
        );
        Ok(next.run(request).await)
    } else {
        tracing::warn!(
            role = ?role,
            required_level = required.min_role_level,
            permission = required.description,
            "Authorization denied - insufficient permissions"
        );
        Err(AuthorizationError {
            status: StatusCode::FORBIDDEN,
            message: "Insufficient permissions for this operation",
        })
    }
}

/// Convenience middleware factory for manager-level access.
pub async fn require_manager(
    request: Request<Body>,
    next: Next,
) -> Result<Response, AuthorizationError> {
    require_permission(permissions::MANAGER, request, next).await
}

/// Convenience middleware factory for owner-level access.
pub async fn require_owner(
    request: Request<Body>,
    next: Next,
) -> Result<Response, AuthorizationError> {
    require_permission(permissions::OWNER, request, next).await
}

/// Convenience middleware factory for admin-level access.
pub async fn require_admin(
    request: Request<Body>,
    next: Next,
) -> Result<Response, AuthorizationError> {
    require_permission(permissions::ORG_ADMIN, request, next).await
}

/// Convenience middleware factory for platform admin access.
pub async fn require_platform_admin(
    request: Request<Body>,
    next: Next,
) -> Result<Response, AuthorizationError> {
    require_permission(permissions::PLATFORM_ADMIN, request, next).await
}

/// Convenience middleware factory for any authenticated user.
pub async fn require_authenticated(
    request: Request<Body>,
    next: Next,
) -> Result<Response, AuthorizationError> {
    require_permission(permissions::AUTHENTICATED, request, next).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_levels() {
        // Super admin can access everything
        assert!(permissions::SUPER_ADMIN.is_satisfied_by(&TenantRole::SuperAdmin));
        assert!(!permissions::SUPER_ADMIN.is_satisfied_by(&TenantRole::Manager));

        // Manager can access manager-level and below
        assert!(permissions::MANAGER.is_satisfied_by(&TenantRole::Manager));
        assert!(permissions::MANAGER.is_satisfied_by(&TenantRole::SuperAdmin));
        assert!(!permissions::MANAGER.is_satisfied_by(&TenantRole::Owner));

        // Owner can access owner-level and below
        assert!(permissions::OWNER.is_satisfied_by(&TenantRole::Owner));
        assert!(permissions::OWNER.is_satisfied_by(&TenantRole::Manager));
        assert!(!permissions::OWNER.is_satisfied_by(&TenantRole::Tenant));

        // Any authenticated user
        assert!(permissions::AUTHENTICATED.is_satisfied_by(&TenantRole::Guest));
        assert!(permissions::AUTHENTICATED.is_satisfied_by(&TenantRole::Tenant));
    }
}
