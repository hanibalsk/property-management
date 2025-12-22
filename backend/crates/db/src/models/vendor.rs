//! Vendor and supplier models (Epic 21).

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Vendor status constants.
pub mod vendor_status {
    pub const ACTIVE: &str = "active";
    pub const INACTIVE: &str = "inactive";
    pub const SUSPENDED: &str = "suspended";
    pub const PENDING_APPROVAL: &str = "pending_approval";
    pub const ALL: &[&str] = &[ACTIVE, INACTIVE, SUSPENDED, PENDING_APPROVAL];
}

/// Vendor service type constants.
pub mod service_type {
    pub const PLUMBING: &str = "plumbing";
    pub const ELECTRICAL: &str = "electrical";
    pub const HVAC: &str = "hvac";
    pub const CLEANING: &str = "cleaning";
    pub const LANDSCAPING: &str = "landscaping";
    pub const SECURITY: &str = "security";
    pub const PAINTING: &str = "painting";
    pub const ROOFING: &str = "roofing";
    pub const GENERAL_MAINTENANCE: &str = "general_maintenance";
    pub const OTHER: &str = "other";
    pub const ALL: &[&str] = &[
        PLUMBING,
        ELECTRICAL,
        HVAC,
        CLEANING,
        LANDSCAPING,
        SECURITY,
        PAINTING,
        ROOFING,
        GENERAL_MAINTENANCE,
        OTHER,
    ];
}

/// Contract type constants.
pub mod contract_type {
    pub const SERVICE: &str = "service";
    pub const MAINTENANCE: &str = "maintenance";
    pub const PROJECT: &str = "project";
    pub const RETAINER: &str = "retainer";
    pub const ONE_TIME: &str = "one_time";
    pub const ALL: &[&str] = &[SERVICE, MAINTENANCE, PROJECT, RETAINER, ONE_TIME];
}

/// Contract status constants.
pub mod contract_status {
    pub const DRAFT: &str = "draft";
    pub const PENDING_APPROVAL: &str = "pending_approval";
    pub const ACTIVE: &str = "active";
    pub const EXPIRED: &str = "expired";
    pub const TERMINATED: &str = "terminated";
    pub const RENEWED: &str = "renewed";
    pub const ALL: &[&str] = &[
        DRAFT,
        PENDING_APPROVAL,
        ACTIVE,
        EXPIRED,
        TERMINATED,
        RENEWED,
    ];
}

/// Invoice status constants.
pub mod invoice_status {
    pub const DRAFT: &str = "draft";
    pub const PENDING: &str = "pending";
    pub const APPROVED: &str = "approved";
    pub const REJECTED: &str = "rejected";
    pub const PARTIALLY_PAID: &str = "partially_paid";
    pub const PAID: &str = "paid";
    pub const CANCELLED: &str = "cancelled";
    pub const OVERDUE: &str = "overdue";
    pub const ALL: &[&str] = &[
        DRAFT,
        PENDING,
        APPROVED,
        REJECTED,
        PARTIALLY_PAID,
        PAID,
        CANCELLED,
        OVERDUE,
    ];
}

// ==================== Vendor ====================

/// Vendor/Supplier entity (Story 21.1).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Vendor {
    pub id: Uuid,
    pub organization_id: Uuid,

    // Company information
    pub company_name: String,
    pub contact_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,

    // Business details
    pub services: Vec<String>,
    pub license_number: Option<String>,
    pub tax_id: Option<String>,

    // Contract information
    pub contract_start: Option<NaiveDate>,
    pub contract_end: Option<NaiveDate>,
    pub hourly_rate: Option<Decimal>,

    // Performance tracking
    pub rating: Option<Decimal>,
    pub total_jobs: Option<i32>,
    pub completed_jobs: Option<i32>,

    // Status
    pub status: String,
    pub is_preferred: Option<bool>,
    pub notes: Option<String>,

    // Metadata
    pub metadata: Option<serde_json::Value>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create vendor request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateVendor {
    pub company_name: String,
    pub contact_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,
    pub services: Vec<String>,
    pub license_number: Option<String>,
    pub tax_id: Option<String>,
    pub contract_start: Option<NaiveDate>,
    pub contract_end: Option<NaiveDate>,
    pub hourly_rate: Option<Decimal>,
    pub is_preferred: Option<bool>,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Update vendor request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateVendor {
    pub company_name: Option<String>,
    pub contact_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,
    pub services: Option<Vec<String>>,
    pub license_number: Option<String>,
    pub tax_id: Option<String>,
    pub contract_start: Option<NaiveDate>,
    pub contract_end: Option<NaiveDate>,
    pub hourly_rate: Option<Decimal>,
    pub status: Option<String>,
    pub is_preferred: Option<bool>,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Vendor query parameters.
#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct VendorQuery {
    pub status: Option<String>,
    pub service: Option<String>,
    pub is_preferred: Option<bool>,
    pub contract_expiring_days: Option<i32>,
    pub search: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

// ==================== Vendor Contact ====================

/// Vendor contact entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct VendorContact {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub name: String,
    pub role: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub is_primary: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create vendor contact request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateVendorContact {
    pub name: String,
    pub role: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub is_primary: Option<bool>,
}

// ==================== Vendor Contract ====================

/// Vendor contract entity (Story 21.3).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct VendorContract {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub organization_id: Uuid,

    // Contract details
    pub contract_number: Option<String>,
    pub title: String,
    pub description: Option<String>,

    // Dates
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub renewal_date: Option<NaiveDate>,

    // Financial terms
    pub contract_value: Option<Decimal>,
    pub payment_terms: Option<String>,

    // Contract type
    pub contract_type: String,

    // Status
    pub status: String,
    pub auto_renew: Option<bool>,

    // Metadata
    pub terms: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub signed_at: Option<DateTime<Utc>>,
}

/// Create vendor contract request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateVendorContract {
    pub vendor_id: Uuid,
    pub contract_number: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub renewal_date: Option<NaiveDate>,
    pub contract_value: Option<Decimal>,
    pub payment_terms: Option<String>,
    pub contract_type: Option<String>,
    pub auto_renew: Option<bool>,
    pub terms: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

/// Update vendor contract request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateVendorContract {
    pub contract_number: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub renewal_date: Option<NaiveDate>,
    pub contract_value: Option<Decimal>,
    pub payment_terms: Option<String>,
    pub contract_type: Option<String>,
    pub status: Option<String>,
    pub auto_renew: Option<bool>,
    pub terms: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

/// Contract query parameters.
#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct ContractQuery {
    pub vendor_id: Option<Uuid>,
    pub status: Option<String>,
    pub contract_type: Option<String>,
    pub expiring_days: Option<i32>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

// ==================== Vendor Invoice ====================

/// Vendor invoice entity (Story 21.4).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct VendorInvoice {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub vendor_id: Uuid,
    pub contract_id: Option<Uuid>,

    // Invoice details
    pub invoice_number: String,
    pub invoice_date: NaiveDate,
    pub due_date: Option<NaiveDate>,

    // Amounts
    pub subtotal: Decimal,
    pub tax_amount: Option<Decimal>,
    pub total_amount: Decimal,
    pub paid_amount: Option<Decimal>,

    // Currency
    pub currency: Option<String>,

    // Status
    pub status: String,

    // Related work orders
    pub work_order_ids: Option<Vec<Uuid>>,

    // Payment info
    pub payment_method: Option<String>,
    pub payment_reference: Option<String>,
    pub paid_at: Option<DateTime<Utc>>,

    // Description
    pub description: Option<String>,
    pub line_items: Option<serde_json::Value>,

    // Approval workflow
    pub submitted_by: Option<Uuid>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_by: Option<Uuid>,
    pub rejection_reason: Option<String>,

    // Metadata
    pub metadata: Option<serde_json::Value>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create vendor invoice request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateVendorInvoice {
    pub vendor_id: Uuid,
    pub contract_id: Option<Uuid>,
    pub invoice_number: String,
    pub invoice_date: NaiveDate,
    pub due_date: Option<NaiveDate>,
    pub subtotal: Decimal,
    pub tax_amount: Option<Decimal>,
    pub total_amount: Decimal,
    pub currency: Option<String>,
    pub work_order_ids: Option<Vec<Uuid>>,
    pub description: Option<String>,
    pub line_items: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

/// Update vendor invoice request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateVendorInvoice {
    pub invoice_number: Option<String>,
    pub invoice_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
    pub subtotal: Option<Decimal>,
    pub tax_amount: Option<Decimal>,
    pub total_amount: Option<Decimal>,
    pub currency: Option<String>,
    pub work_order_ids: Option<Vec<Uuid>>,
    pub description: Option<String>,
    pub line_items: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

/// Invoice query parameters.
#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct InvoiceQuery {
    pub vendor_id: Option<Uuid>,
    pub status: Option<String>,
    pub due_before: Option<NaiveDate>,
    pub due_after: Option<NaiveDate>,
    pub work_order_id: Option<Uuid>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

// ==================== Vendor Rating ====================

/// Vendor rating entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct VendorRating {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub work_order_id: Option<Uuid>,
    pub rated_by: Uuid,

    // Rating details
    pub rating: i32,
    pub quality_rating: Option<i32>,
    pub timeliness_rating: Option<i32>,
    pub communication_rating: Option<i32>,
    pub value_rating: Option<i32>,

    // Review
    pub review_text: Option<String>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create vendor rating request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateVendorRating {
    pub work_order_id: Option<Uuid>,
    pub rating: i32,
    pub quality_rating: Option<i32>,
    pub timeliness_rating: Option<i32>,
    pub communication_rating: Option<i32>,
    pub value_rating: Option<i32>,
    pub review_text: Option<String>,
}

// ==================== Analytics ====================

/// Vendor statistics.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VendorStatistics {
    pub total_vendors: i64,
    pub active_vendors: i64,
    pub preferred_vendors: i64,
    pub by_service: Vec<ServiceCount>,
    pub expiring_contracts: i64,
}

/// Service count for statistics.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ServiceCount {
    pub service: String,
    pub count: i64,
}

/// Invoice summary for a period.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InvoiceSummary {
    pub vendor_id: Uuid,
    pub vendor_name: String,
    pub total_invoices: i64,
    pub total_amount: Decimal,
    pub paid_amount: Decimal,
    pub pending_amount: Decimal,
}

/// Vendor with details (for listing).
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct VendorWithDetails {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub company_name: String,
    pub contact_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub services: Vec<String>,
    pub rating: Option<Decimal>,
    pub total_jobs: Option<i32>,
    pub completed_jobs: Option<i32>,
    pub status: String,
    pub is_preferred: Option<bool>,
    pub contract_end: Option<NaiveDate>,
    pub active_work_orders: i64,
    pub pending_invoices: i64,
}

/// Contracts expiring soon.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ExpiringContract {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub vendor_name: String,
    pub title: String,
    pub end_date: Option<NaiveDate>,
    pub days_until_expiry: i32,
    pub contract_value: Option<Decimal>,
    pub auto_renew: Option<bool>,
}
