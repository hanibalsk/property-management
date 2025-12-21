//! Role model for RBAC (Epic 2A, Story 2A.6).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Role entity from database.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Role {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: serde_json::Value,
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Role {
    /// Check if role has a specific permission.
    pub fn has_permission(&self, permission: &str) -> bool {
        if let Some(perms) = self.permissions.as_array() {
            // Check for wildcard
            if perms
                .iter()
                .any(|p: &serde_json::Value| p.as_str() == Some("*"))
            {
                return true;
            }

            // Check for exact match
            if perms
                .iter()
                .any(|p: &serde_json::Value| p.as_str() == Some(permission))
            {
                return true;
            }

            // Check for resource wildcard (e.g., "faults:*" matches "faults:create")
            let parts: Vec<&str> = permission.split(':').collect();
            if parts.len() == 2 {
                let resource_wildcard = format!("{}:*", parts[0]);
                if perms
                    .iter()
                    .any(|p: &serde_json::Value| p.as_str() == Some(&resource_wildcard))
                {
                    return true;
                }
            }
        }
        false
    }

    /// Get all permissions as a list of strings.
    pub fn permission_list(&self) -> Vec<String> {
        self.permissions
            .as_array()
            .map(|arr: &Vec<serde_json::Value>| {
                arr.iter()
                    .filter_map(|v: &serde_json::Value| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// Data for creating a new role.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRole {
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
}

/// Data for updating a role.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct UpdateRole {
    pub name: Option<String>,
    pub description: Option<String>,
    pub permissions: Option<Vec<String>>,
}

/// Permission definition for documentation.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PermissionDefinition {
    pub resource: String,
    pub action: String,
    pub description: String,
}

/// System role names (created automatically for each org).
pub mod system_roles {
    pub const SUPER_ADMIN: &str = "Super Admin";
    pub const ORG_ADMIN: &str = "Organization Admin";
    pub const MANAGER: &str = "Manager";
    pub const TECHNICAL_MANAGER: &str = "Technical Manager";
    pub const OWNER: &str = "Owner";
    pub const TENANT: &str = "Tenant";
    pub const RESIDENT: &str = "Resident";
    pub const GUEST: &str = "Guest";
}

/// Permission resources.
pub mod permissions {
    // Organization permissions
    pub const ORGANIZATION_READ: &str = "organization:read";
    pub const ORGANIZATION_UPDATE: &str = "organization:update";
    pub const ORGANIZATION_DELETE: &str = "organization:delete";
    pub const ORGANIZATION_ALL: &str = "organization:*";

    // User/Member permissions
    pub const USERS_READ: &str = "users:read";
    pub const USERS_CREATE: &str = "users:create";
    pub const USERS_UPDATE: &str = "users:update";
    pub const USERS_DELETE: &str = "users:delete";
    pub const USERS_ALL: &str = "users:*";

    // Building permissions
    pub const BUILDINGS_READ: &str = "buildings:read";
    pub const BUILDINGS_CREATE: &str = "buildings:create";
    pub const BUILDINGS_UPDATE: &str = "buildings:update";
    pub const BUILDINGS_DELETE: &str = "buildings:delete";
    pub const BUILDINGS_ALL: &str = "buildings:*";

    // Unit permissions
    pub const UNITS_READ: &str = "units:read";
    pub const UNITS_CREATE: &str = "units:create";
    pub const UNITS_UPDATE: &str = "units:update";
    pub const UNITS_DELETE: &str = "units:delete";
    pub const UNITS_ALL: &str = "units:*";

    // Fault permissions
    pub const FAULTS_READ: &str = "faults:read";
    pub const FAULTS_CREATE: &str = "faults:create";
    pub const FAULTS_UPDATE: &str = "faults:update";
    pub const FAULTS_DELETE: &str = "faults:delete";
    pub const FAULTS_ASSIGN: &str = "faults:assign";
    pub const FAULTS_ALL: &str = "faults:*";

    // Vote permissions
    pub const VOTES_READ: &str = "votes:read";
    pub const VOTES_CREATE: &str = "votes:create";
    pub const VOTES_VOTE: &str = "votes:vote";
    pub const VOTES_MANAGE: &str = "votes:manage";
    pub const VOTES_ALL: &str = "votes:*";

    // Document permissions
    pub const DOCUMENTS_READ: &str = "documents:read";
    pub const DOCUMENTS_CREATE: &str = "documents:create";
    pub const DOCUMENTS_UPDATE: &str = "documents:update";
    pub const DOCUMENTS_DELETE: &str = "documents:delete";
    pub const DOCUMENTS_MANAGE: &str = "documents:manage";
    pub const DOCUMENTS_ALL: &str = "documents:*";

    // Announcement permissions
    pub const ANNOUNCEMENTS_READ: &str = "announcements:read";
    pub const ANNOUNCEMENTS_CREATE: &str = "announcements:create";
    pub const ANNOUNCEMENTS_UPDATE: &str = "announcements:update";
    pub const ANNOUNCEMENTS_DELETE: &str = "announcements:delete";
    pub const ANNOUNCEMENTS_ALL: &str = "announcements:*";

    // Wildcard (all permissions)
    pub const ALL: &str = "*";
}
