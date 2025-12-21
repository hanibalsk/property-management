//! Delegation model (Epic 3, Story 3.4).

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Delegation scope enum values.
pub mod delegation_scope {
    pub const ALL: &str = "all";
    pub const VOTING: &str = "voting";
    pub const DOCUMENTS: &str = "documents";
    pub const FAULTS: &str = "faults";
    pub const FINANCIAL: &str = "financial";
}

/// Delegation status enum values.
pub mod delegation_status {
    pub const PENDING: &str = "pending";
    pub const ACTIVE: &str = "active";
    pub const REVOKED: &str = "revoked";
    pub const EXPIRED: &str = "expired";
    pub const DECLINED: &str = "declined";
}

/// Delegation entity from database.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Delegation {
    pub id: Uuid,
    pub owner_user_id: Uuid,
    pub delegate_user_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub scopes: Vec<String>,
    pub status: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub invitation_token: Option<String>,
    pub invitation_sent_at: Option<DateTime<Utc>>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub declined_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Delegation {
    /// Check if delegation is currently active.
    pub fn is_active(&self) -> bool {
        self.status == delegation_status::ACTIVE
    }

    /// Check if delegation is pending acceptance.
    pub fn is_pending(&self) -> bool {
        self.status == delegation_status::PENDING
    }

    /// Check if delegation has a specific scope.
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.iter().any(|s| s == "all" || s == scope)
    }

    /// Get status display name.
    pub fn status_display(&self) -> &str {
        match self.status.as_str() {
            "pending" => "Pending",
            "active" => "Active",
            "revoked" => "Revoked",
            "expired" => "Expired",
            "declined" => "Declined",
            _ => &self.status,
        }
    }
}

/// Summary view of a delegation.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct DelegationSummary {
    pub id: Uuid,
    pub owner_user_id: Uuid,
    pub delegate_user_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub scopes: Vec<String>,
    pub status: String,
}

/// Delegation with user info for display.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DelegationWithUsers {
    #[serde(flatten)]
    pub delegation: Delegation,
    pub owner_name: String,
    pub owner_email: String,
    pub delegate_name: String,
    pub delegate_email: String,
    pub unit_designation: Option<String>,
}

/// Data for creating a new delegation.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateDelegation {
    pub delegate_user_id: Uuid,
    pub unit_id: Option<Uuid>,
    #[serde(default = "default_scopes")]
    pub scopes: Vec<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

fn default_scopes() -> Vec<String> {
    vec!["all".to_string()]
}

/// Data for updating a delegation.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct UpdateDelegation {
    pub scopes: Option<Vec<String>>,
    pub end_date: Option<NaiveDate>,
}

/// Request to accept a delegation.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AcceptDelegation {
    pub invitation_token: String,
}

/// Request to decline a delegation.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DeclineDelegation {
    pub invitation_token: String,
}

/// Request to revoke a delegation.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RevokeDelegation {
    pub reason: Option<String>,
}

/// Delegation audit log entry.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct DelegationAuditLog {
    pub id: Uuid,
    pub delegation_id: Uuid,
    pub action: String,
    pub actor_user_id: Option<Uuid>,
    pub details: serde_json::Value,
    pub created_at: DateTime<Utc>,
}
