//! Agency models (Epic 17: Agency & Realtor Management).
//!
//! Models for real estate agencies, realtors, and shared listings.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================
// Agency (Story 17.1)
// ============================================

/// Agency status enum.
pub mod agency_status {
    pub const PENDING: &str = "pending";
    pub const VERIFIED: &str = "verified";
    pub const SUSPENDED: &str = "suspended";
}

/// Agency entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Agency {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: String,
    pub phone: Option<String>,
    pub email: String,
    pub website: Option<String>,
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
    pub description: Option<String>,
    pub status: String, // pending, verified, suspended
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create agency request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateAgency {
    pub name: String,
    pub slug: String,
    pub email: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: String,
    pub phone: Option<String>,
    pub website: Option<String>,
}

/// Update agency request.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct UpdateAgency {
    pub name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
    pub description: Option<String>,
}

/// Agency branding settings.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AgencyBranding {
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
}

/// Agency summary for listings.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct AgencySummary {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub logo_url: Option<String>,
    pub city: Option<String>,
}

/// Agency public profile.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AgencyProfile {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: String,
    pub phone: Option<String>,
    pub email: String,
    pub website: Option<String>,
    pub logo_url: Option<String>,
    pub description: Option<String>,
    pub verified: bool,
    pub member_count: i32,
    pub active_listing_count: i64,
}

// ============================================
// Agency Members / Realtors (Story 17.2)
// ============================================

/// Agency member role enum.
pub mod member_role {
    pub const AGENT: &str = "agent"; // Own listings only
    pub const SENIOR: &str = "senior"; // All listings
    pub const ADMIN: &str = "admin"; // Full control
}

/// Agency member entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct AgencyMember {
    pub id: Uuid,
    pub agency_id: Uuid,
    pub user_id: Uuid,
    pub role: String, // agent, senior, admin
    pub is_active: bool,
    pub joined_at: DateTime<Utc>,
    pub left_at: Option<DateTime<Utc>>,
}

/// Agency member with user details.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AgencyMemberWithUser {
    pub id: Uuid,
    pub agency_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub user_avatar: Option<String>,
    pub role: String,
    pub is_active: bool,
    pub joined_at: DateTime<Utc>,
    pub listing_count: i64,
}

/// Row for agency member with user details.
#[derive(Debug, Clone, FromRow)]
pub struct AgencyMemberWithUserRow {
    pub id: Uuid,
    pub agency_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub user_avatar: Option<String>,
    pub role: String,
    pub is_active: bool,
    pub joined_at: DateTime<Utc>,
    pub listing_count: i64,
}

/// Invite member request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InviteMember {
    pub email: String,
    pub role: String,
}

/// Agency invitation entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct AgencyInvitation {
    pub id: Uuid,
    pub agency_id: Uuid,
    pub email: String,
    pub role: String,
    pub invited_by: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Accept invitation request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AcceptInvitation {
    pub token: String,
}

/// Update member role request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateMemberRole {
    pub role: String,
}

/// Members list response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AgencyMembersResponse {
    pub members: Vec<AgencyMemberWithUser>,
    pub total: i64,
}

// ============================================
// Shared Listings (Story 17.3)
// ============================================

/// Listing visibility enum.
pub mod listing_visibility {
    pub const PERSONAL: &str = "personal"; // Only creator
    pub const AGENCY: &str = "agency"; // All agency members
    pub const PUBLIC: &str = "public"; // Published
}

/// Inquiry assignment mode.
pub mod inquiry_assignment {
    pub const POOL: &str = "pool"; // First to claim
    pub const ROUND_ROBIN: &str = "round_robin"; // Auto-assign
    pub const CREATOR: &str = "creator"; // Always to creator
}

/// Agency listing association (links listing to agency/realtor).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct AgencyListing {
    pub id: Uuid,
    pub listing_id: Uuid,
    pub agency_id: Uuid,
    pub realtor_id: Uuid,   // Creator
    pub visibility: String, // personal, agency, public
    pub inquiry_assignment: String,
    pub created_at: DateTime<Utc>,
}

/// Create agency listing request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateAgencyListing {
    pub listing_id: Uuid,
    #[serde(default = "default_agency_visibility")]
    pub visibility: String,
    #[serde(default = "default_inquiry_assignment")]
    pub inquiry_assignment: String,
}

fn default_agency_visibility() -> String {
    listing_visibility::AGENCY.to_string()
}

fn default_inquiry_assignment() -> String {
    inquiry_assignment::POOL.to_string()
}

/// Update listing visibility request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateListingVisibility {
    pub visibility: String,
}

/// Listing edit history entry.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ListingEditHistory {
    pub id: Uuid,
    pub listing_id: Uuid,
    pub editor_id: Uuid,
    pub editor_name: String,
    pub field_changed: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub edited_at: DateTime<Utc>,
}

/// Listing collaborator info.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ListingCollaborator {
    pub user_id: Uuid,
    pub name: String,
    pub avatar: Option<String>,
    pub is_editing: bool,
    pub last_edit: Option<DateTime<Utc>>,
}

// ============================================
// External Import (Story 17.4)
// ============================================

/// Import source types.
pub mod import_source {
    pub const CSV: &str = "csv";
    pub const XML_SREALITY: &str = "xml_sreality";
    pub const API: &str = "api";
}

/// Import status.
pub mod import_status {
    pub const PENDING: &str = "pending";
    pub const PROCESSING: &str = "processing";
    pub const COMPLETED: &str = "completed";
    pub const FAILED: &str = "failed";
}

/// Listing import job.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ListingImportJob {
    pub id: Uuid,
    pub agency_id: Uuid,
    pub user_id: Uuid,
    pub source: String, // csv, xml_sreality, api
    pub status: String,
    pub total_records: i32,
    pub processed_records: i32,
    pub success_count: i32,
    pub failure_count: i32,
    pub error_log: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Create import job request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateImportJob {
    pub source: String,
}

/// Import field mapping.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ImportFieldMapping {
    pub id: Uuid,
    pub import_job_id: Uuid,
    pub source_field: String,
    pub target_field: String,
}

/// Field mapping request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FieldMapping {
    pub source_field: String,
    pub target_field: String,
}

/// Import configuration request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ImportConfig {
    pub mappings: Vec<FieldMapping>,
}

/// Import preview response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ImportPreview {
    pub total_records: i32,
    pub sample_data: Vec<serde_json::Value>,
    pub detected_fields: Vec<String>,
    pub suggested_mappings: Vec<FieldMapping>,
}

/// Import result summary.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ImportResult {
    pub job_id: Uuid,
    pub status: String,
    pub total_records: i32,
    pub success_count: i32,
    pub failure_count: i32,
    pub created_listings: Vec<Uuid>,
    pub errors: Vec<ImportError>,
}

/// Import error detail.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ImportError {
    pub row: i32,
    pub field: String,
    pub message: String,
}

// ============================================
// Agency Listings Response
// ============================================

/// Agency listing summary.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AgencyListingSummary {
    pub id: Uuid,
    pub listing_id: Uuid,
    pub title: String,
    pub price: i64,
    pub currency: String,
    pub city: String,
    pub property_type: String,
    pub status: String,
    pub visibility: String,
    pub realtor_id: Uuid,
    pub realtor_name: String,
    pub photo_url: Option<String>,
    pub inquiry_count: i64,
    pub created_at: DateTime<Utc>,
}

/// Agency listings response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AgencyListingsResponse {
    pub listings: Vec<AgencyListingSummary>,
    pub total: i64,
}
