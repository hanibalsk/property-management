//! Unit resident model (Epic 3, Story 3.3).

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Resident type enum values.
pub mod resident_type {
    pub const OWNER: &str = "owner";
    pub const TENANT: &str = "tenant";
    pub const FAMILY_MEMBER: &str = "family_member";
    pub const SUBTENANT: &str = "subtenant";
}

/// Unit resident entity from database.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct UnitResident {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub user_id: Uuid,
    pub resident_type: String,
    pub is_primary: bool,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub receives_notifications: bool,
    pub receives_mail: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

impl UnitResident {
    /// Check if resident is currently active.
    pub fn is_active(&self) -> bool {
        self.end_date.is_none()
    }

    /// Get resident type display name.
    pub fn resident_type_display(&self) -> &str {
        match self.resident_type.as_str() {
            "owner" => "Owner",
            "tenant" => "Tenant",
            "family_member" => "Family Member",
            "subtenant" => "Subtenant",
            _ => &self.resident_type,
        }
    }
}

/// Summary view of a unit resident.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct UnitResidentSummary {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub user_id: Uuid,
    pub resident_type: String,
    pub is_primary: bool,
    pub is_active: bool,
}

/// Resident with user info for display.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct UnitResidentWithUser {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub resident_type: String,
    pub is_primary: bool,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

/// Data for adding a resident to a unit.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateUnitResident {
    pub unit_id: Uuid,
    pub user_id: Uuid,
    pub resident_type: String,
    #[serde(default)]
    pub is_primary: bool,
    pub start_date: Option<NaiveDate>,
    #[serde(default = "default_true")]
    pub receives_notifications: bool,
    #[serde(default = "default_true")]
    pub receives_mail: bool,
    pub notes: Option<String>,
}

fn default_true() -> bool {
    true
}

/// Data for updating a unit resident.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct UpdateUnitResident {
    pub resident_type: Option<String>,
    pub is_primary: Option<bool>,
    pub end_date: Option<NaiveDate>,
    pub receives_notifications: Option<bool>,
    pub receives_mail: Option<bool>,
    pub notes: Option<String>,
}

/// Request to end a residency.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EndResidency {
    pub end_date: NaiveDate,
}
