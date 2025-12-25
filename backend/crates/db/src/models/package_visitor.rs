//! Package and Visitor Management models for Epic 58.
//!
//! Supports package tracking, visitor pre-registration, and access codes.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// Status Constants
// ============================================================================

/// Package status values.
pub mod package_status {
    pub const EXPECTED: &str = "expected";
    pub const RECEIVED: &str = "received";
    pub const NOTIFIED: &str = "notified";
    pub const PICKED_UP: &str = "picked_up";
    pub const RETURNED: &str = "returned";
    pub const UNCLAIMED: &str = "unclaimed";
}

/// Package carrier values.
pub mod package_carrier {
    pub const USPS: &str = "usps";
    pub const UPS: &str = "ups";
    pub const FEDEX: &str = "fedex";
    pub const DHL: &str = "dhl";
    pub const AMAZON: &str = "amazon";
    pub const OTHER: &str = "other";
}

/// Visitor status values.
pub mod visitor_status {
    pub const PENDING: &str = "pending";
    pub const CHECKED_IN: &str = "checked_in";
    pub const CHECKED_OUT: &str = "checked_out";
    pub const EXPIRED: &str = "expired";
    pub const CANCELLED: &str = "cancelled";
}

/// Visitor purpose values.
pub mod visitor_purpose {
    pub const GUEST: &str = "guest";
    pub const DELIVERY: &str = "delivery";
    pub const SERVICE: &str = "service";
    pub const CONTRACTOR: &str = "contractor";
    pub const REAL_ESTATE: &str = "real_estate";
    pub const OTHER: &str = "other";
}

// ============================================================================
// Package Entity
// ============================================================================

/// A package tracking record.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Package {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub building_id: Uuid,
    pub unit_id: Uuid,
    pub resident_id: Uuid,
    pub tracking_number: Option<String>,
    pub carrier: String,
    pub carrier_name: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub expected_date: Option<NaiveDate>,
    pub received_at: Option<DateTime<Utc>>,
    pub received_by: Option<Uuid>,
    pub notified_at: Option<DateTime<Utc>>,
    pub picked_up_at: Option<DateTime<Utc>>,
    pub picked_up_by: Option<Uuid>,
    pub storage_location: Option<String>,
    pub photo_url: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Summary view of a package for list responses.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct PackageSummary {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub unit_number: Option<String>,
    pub resident_id: Uuid,
    pub resident_name: Option<String>,
    pub tracking_number: Option<String>,
    pub carrier: String,
    pub status: String,
    pub expected_date: Option<NaiveDate>,
    pub received_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Package with all related details.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct PackageWithDetails {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub building_id: Uuid,
    pub unit_id: Uuid,
    pub resident_id: Uuid,
    pub tracking_number: Option<String>,
    pub carrier: String,
    pub carrier_name: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub expected_date: Option<NaiveDate>,
    pub received_at: Option<DateTime<Utc>>,
    pub received_by: Option<Uuid>,
    pub notified_at: Option<DateTime<Utc>>,
    pub picked_up_at: Option<DateTime<Utc>>,
    pub picked_up_by: Option<Uuid>,
    pub storage_location: Option<String>,
    pub photo_url: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub unit_number: Option<String>,
    pub building_name: Option<String>,
    pub resident_name: Option<String>,
    pub received_by_name: Option<String>,
}

// ============================================================================
// Visitor Entity
// ============================================================================

/// A visitor pre-registration record.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Visitor {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub building_id: Uuid,
    pub unit_id: Uuid,
    pub host_id: Uuid,
    pub visitor_name: String,
    pub visitor_email: Option<String>,
    pub visitor_phone: Option<String>,
    pub company_name: Option<String>,
    pub purpose: String,
    pub purpose_notes: Option<String>,
    pub access_code: String,
    pub access_code_expires_at: DateTime<Utc>,
    pub expected_arrival: DateTime<Utc>,
    pub expected_departure: Option<DateTime<Utc>>,
    pub status: String,
    pub checked_in_at: Option<DateTime<Utc>>,
    pub checked_in_by: Option<Uuid>,
    pub checked_out_at: Option<DateTime<Utc>>,
    pub checked_out_by: Option<Uuid>,
    pub notification_sent_at: Option<DateTime<Utc>>,
    pub notification_method: Option<String>,
    pub vehicle_license_plate: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Summary view of a visitor for list responses.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct VisitorSummary {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub unit_number: Option<String>,
    pub host_id: Uuid,
    pub host_name: Option<String>,
    pub visitor_name: String,
    pub purpose: String,
    pub expected_arrival: DateTime<Utc>,
    pub status: String,
    pub access_code: String,
    pub created_at: DateTime<Utc>,
}

/// Visitor with all related details.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct VisitorWithDetails {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub building_id: Uuid,
    pub unit_id: Uuid,
    pub host_id: Uuid,
    pub visitor_name: String,
    pub visitor_email: Option<String>,
    pub visitor_phone: Option<String>,
    pub company_name: Option<String>,
    pub purpose: String,
    pub purpose_notes: Option<String>,
    pub access_code: String,
    pub access_code_expires_at: DateTime<Utc>,
    pub expected_arrival: DateTime<Utc>,
    pub expected_departure: Option<DateTime<Utc>>,
    pub status: String,
    pub checked_in_at: Option<DateTime<Utc>>,
    pub checked_in_by: Option<Uuid>,
    pub checked_out_at: Option<DateTime<Utc>>,
    pub checked_out_by: Option<Uuid>,
    pub notification_sent_at: Option<DateTime<Utc>>,
    pub notification_method: Option<String>,
    pub vehicle_license_plate: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub unit_number: Option<String>,
    pub building_name: Option<String>,
    pub host_name: Option<String>,
    pub checked_in_by_name: Option<String>,
}

// ============================================================================
// Settings Entities
// ============================================================================

/// Building package settings.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct BuildingPackageSettings {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub building_id: Uuid,
    pub max_storage_days: i32,
    pub send_reminder_after_days: i32,
    pub require_photo_on_receipt: bool,
    pub allow_resident_self_pickup: bool,
    pub notify_on_arrival: bool,
    pub send_daily_summary: bool,
    pub storage_instructions: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Building visitor settings.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct BuildingVisitorSettings {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub building_id: Uuid,
    pub default_code_validity_hours: i32,
    pub code_length: i32,
    pub require_purpose: bool,
    pub max_visitors_per_day_per_unit: Option<i32>,
    pub max_advance_registration_days: i32,
    pub notify_host_on_checkin: bool,
    pub send_visitor_instructions: bool,
    pub require_id_verification: bool,
    pub require_photo: bool,
    pub visitor_instructions: Option<String>,
    pub staff_instructions: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request to create a package.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePackage {
    pub unit_id: Uuid,
    pub tracking_number: Option<String>,
    pub carrier: String,
    pub carrier_name: Option<String>,
    pub description: Option<String>,
    pub expected_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

/// Request to update a package.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdatePackage {
    pub tracking_number: Option<String>,
    pub carrier: Option<String>,
    pub carrier_name: Option<String>,
    pub description: Option<String>,
    pub expected_date: Option<NaiveDate>,
    pub storage_location: Option<String>,
    pub notes: Option<String>,
}

/// Request to log package receipt.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReceivePackage {
    pub storage_location: Option<String>,
    pub photo_url: Option<String>,
    pub notes: Option<String>,
}

/// Request to log package pickup.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PickupPackage {
    pub picked_up_by: Option<Uuid>, // If different from resident
    pub notes: Option<String>,
}

/// Request to create a visitor registration.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateVisitor {
    pub unit_id: Uuid,
    pub visitor_name: String,
    pub visitor_email: Option<String>,
    pub visitor_phone: Option<String>,
    pub company_name: Option<String>,
    pub purpose: String,
    pub purpose_notes: Option<String>,
    pub expected_arrival: DateTime<Utc>,
    pub expected_departure: Option<DateTime<Utc>>,
    pub vehicle_license_plate: Option<String>,
    pub notes: Option<String>,
    pub send_notification: Option<bool>,
}

/// Request to update a visitor registration.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateVisitor {
    pub visitor_name: Option<String>,
    pub visitor_email: Option<String>,
    pub visitor_phone: Option<String>,
    pub company_name: Option<String>,
    pub purpose: Option<String>,
    pub purpose_notes: Option<String>,
    pub expected_arrival: Option<DateTime<Utc>>,
    pub expected_departure: Option<DateTime<Utc>>,
    pub vehicle_license_plate: Option<String>,
    pub notes: Option<String>,
}

/// Request to check in a visitor.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CheckInVisitor {
    pub notes: Option<String>,
}

/// Request to check out a visitor.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CheckOutVisitor {
    pub notes: Option<String>,
}

/// Request to verify an access code.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VerifyAccessCode {
    pub access_code: String,
    pub building_id: Uuid,
}

/// Response for access code verification.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AccessCodeVerification {
    pub valid: bool,
    pub visitor: Option<VisitorSummary>,
    pub message: String,
}

/// Request to update building package settings.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateBuildingPackageSettings {
    pub max_storage_days: Option<i32>,
    pub send_reminder_after_days: Option<i32>,
    pub require_photo_on_receipt: Option<bool>,
    pub allow_resident_self_pickup: Option<bool>,
    pub notify_on_arrival: Option<bool>,
    pub send_daily_summary: Option<bool>,
    pub storage_instructions: Option<String>,
}

/// Request to update building visitor settings.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateBuildingVisitorSettings {
    pub default_code_validity_hours: Option<i32>,
    pub code_length: Option<i32>,
    pub require_purpose: Option<bool>,
    pub max_visitors_per_day_per_unit: Option<i32>,
    pub max_advance_registration_days: Option<i32>,
    pub notify_host_on_checkin: Option<bool>,
    pub send_visitor_instructions: Option<bool>,
    pub require_id_verification: Option<bool>,
    pub require_photo: Option<bool>,
    pub visitor_instructions: Option<String>,
    pub staff_instructions: Option<String>,
}

// ============================================================================
// Query Parameters
// ============================================================================

/// Query parameters for listing packages.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct PackageQuery {
    pub building_id: Option<Uuid>,
    pub unit_id: Option<Uuid>,
    pub resident_id: Option<Uuid>,
    pub status: Option<String>,
    pub carrier: Option<String>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Query parameters for listing visitors.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct VisitorQuery {
    pub building_id: Option<Uuid>,
    pub unit_id: Option<Uuid>,
    pub host_id: Option<Uuid>,
    pub status: Option<String>,
    pub purpose: Option<String>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub today_only: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// ============================================================================
// Statistics
// ============================================================================

/// Package statistics for a building.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PackageStatistics {
    pub total_packages: i64,
    pub expected_packages: i64,
    pub received_packages: i64,
    pub picked_up_packages: i64,
    pub unclaimed_packages: i64,
    pub avg_pickup_time_hours: Option<f64>,
}

/// Visitor statistics for a building.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VisitorStatistics {
    pub total_visitors_today: i64,
    pub pending_arrivals: i64,
    pub checked_in_now: i64,
    pub total_this_week: i64,
    pub total_this_month: i64,
}
