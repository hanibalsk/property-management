//! Organization model (Epic 2A, Story 2A.1).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Organization status enumeration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum OrganizationStatus {
    Active,
    Suspended,
    Deleted,
}

impl OrganizationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrganizationStatus::Active => "active",
            OrganizationStatus::Suspended => "suspended",
            OrganizationStatus::Deleted => "deleted",
        }
    }
}

impl std::fmt::Display for OrganizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Organization entity from database.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub contact_email: String,
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
    pub settings: serde_json::Value,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Organization {
    /// Check if organization is active.
    pub fn is_active(&self) -> bool {
        self.status == "active"
    }

    /// Get status as enum.
    pub fn status_enum(&self) -> OrganizationStatus {
        match self.status.as_str() {
            "active" => OrganizationStatus::Active,
            "suspended" => OrganizationStatus::Suspended,
            "deleted" => OrganizationStatus::Deleted,
            _ => OrganizationStatus::Active,
        }
    }
}

/// Data for creating a new organization.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateOrganization {
    pub name: String,
    pub slug: String,
    pub contact_email: String,
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
}

/// Data for updating an organization.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct UpdateOrganization {
    pub name: Option<String>,
    pub contact_email: Option<String>,
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
    pub settings: Option<serde_json::Value>,
}

/// Organization summary for lists.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct OrganizationSummary {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub logo_url: Option<String>,
    pub status: String,
}
