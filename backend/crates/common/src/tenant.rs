//! Multi-tenancy context and types.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Tenant context - required for all multi-tenant operations.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TenantContext {
    /// Tenant/Organization ID
    pub tenant_id: Uuid,
    /// User ID
    pub user_id: Uuid,
    /// User's role in this tenant
    pub role: TenantRole,
}

impl TenantContext {
    pub fn new(tenant_id: Uuid, user_id: Uuid, role: TenantRole) -> Self {
        Self {
            tenant_id,
            user_id,
            role,
        }
    }

    /// Check if user has at least the specified role level.
    pub fn has_role(&self, required: TenantRole) -> bool {
        self.role.level() >= required.level()
    }

    /// Check if user can access a specific building.
    pub fn can_access_building(&self, _building_id: Uuid) -> bool {
        // TODO: Implement building-level access control
        true
    }
}

/// User role within tenant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TenantRole {
    /// Super administrator (platform level)
    SuperAdmin,
    /// Organization administrator
    OrgAdmin,
    /// Building manager
    Manager,
    /// Technical manager
    TechnicalManager,
    /// Property owner
    Owner,
    /// Owner's delegate
    OwnerDelegate,
    /// Tenant/renter
    Tenant,
    /// Resident (no ownership)
    Resident,
    /// Short-term rental property manager
    PropertyManager,
    /// Real estate agent
    RealEstateAgent,
    /// Guest (temporary access)
    Guest,
}

impl TenantRole {
    /// Get role hierarchy level (higher = more permissions).
    pub fn level(&self) -> u8 {
        match self {
            TenantRole::SuperAdmin => 100,
            TenantRole::OrgAdmin => 90,
            TenantRole::Manager => 80,
            TenantRole::TechnicalManager => 75,
            TenantRole::Owner => 60,
            TenantRole::OwnerDelegate => 55,
            TenantRole::PropertyManager => 50,
            TenantRole::RealEstateAgent => 45,
            TenantRole::Tenant => 40,
            TenantRole::Resident => 30,
            TenantRole::Guest => 10,
        }
    }

    /// Check if role is admin-level.
    pub fn is_admin(&self) -> bool {
        matches!(self, TenantRole::SuperAdmin | TenantRole::OrgAdmin)
    }

    /// Check if role is manager-level.
    pub fn is_manager(&self) -> bool {
        matches!(
            self,
            TenantRole::SuperAdmin | TenantRole::OrgAdmin | TenantRole::Manager | TenantRole::TechnicalManager
        )
    }
}

impl std::fmt::Display for TenantRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TenantRole::SuperAdmin => write!(f, "Super Admin"),
            TenantRole::OrgAdmin => write!(f, "Organization Admin"),
            TenantRole::Manager => write!(f, "Manager"),
            TenantRole::TechnicalManager => write!(f, "Technical Manager"),
            TenantRole::Owner => write!(f, "Owner"),
            TenantRole::OwnerDelegate => write!(f, "Owner Delegate"),
            TenantRole::Tenant => write!(f, "Tenant"),
            TenantRole::Resident => write!(f, "Resident"),
            TenantRole::PropertyManager => write!(f, "Property Manager"),
            TenantRole::RealEstateAgent => write!(f, "Real Estate Agent"),
            TenantRole::Guest => write!(f, "Guest"),
        }
    }
}
