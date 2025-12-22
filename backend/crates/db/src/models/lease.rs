//! Lease management models (Epic 19: Lease Management & Tenant Screening).

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// =============================================================================
// Status Modules
// =============================================================================

/// Application status values.
pub mod application_status {
    pub const DRAFT: &str = "draft";
    pub const SUBMITTED: &str = "submitted";
    pub const UNDER_REVIEW: &str = "under_review";
    pub const SCREENING_PENDING: &str = "screening_pending";
    pub const SCREENING_COMPLETE: &str = "screening_complete";
    pub const APPROVED: &str = "approved";
    pub const REJECTED: &str = "rejected";
    pub const WITHDRAWN: &str = "withdrawn";
}

/// Screening status values.
pub mod screening_status {
    pub const PENDING_CONSENT: &str = "pending_consent";
    pub const CONSENT_RECEIVED: &str = "consent_received";
    pub const IN_PROGRESS: &str = "in_progress";
    pub const COMPLETED: &str = "completed";
    pub const FAILED: &str = "failed";
    pub const EXPIRED: &str = "expired";
}

/// Screening type values.
pub mod screening_type {
    pub const BACKGROUND_CHECK: &str = "background_check";
    pub const CREDIT_CHECK: &str = "credit_check";
    pub const REFERENCE_CHECK: &str = "reference_check";
    pub const EMPLOYMENT_VERIFICATION: &str = "employment_verification";
    pub const INCOME_VERIFICATION: &str = "income_verification";
    pub const RENTAL_HISTORY: &str = "rental_history";
}

/// Lease status values.
pub mod lease_status {
    pub const DRAFT: &str = "draft";
    pub const PENDING_SIGNATURE: &str = "pending_signature";
    pub const ACTIVE: &str = "active";
    pub const RENEWED: &str = "renewed";
    pub const TERMINATED: &str = "terminated";
    pub const EXPIRED: &str = "expired";
    pub const CANCELLED: &str = "cancelled";
}

/// Termination reason values.
pub mod termination_reason {
    pub const END_OF_TERM: &str = "end_of_term";
    pub const MUTUAL_AGREEMENT: &str = "mutual_agreement";
    pub const TENANT_BREACH: &str = "tenant_breach";
    pub const LANDLORD_BREACH: &str = "landlord_breach";
    pub const PROPERTY_SALE: &str = "property_sale";
    pub const RENOVATION: &str = "renovation";
    pub const OTHER: &str = "other";
}

// =============================================================================
// Core Entities
// =============================================================================

/// Tenant application entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TenantApplication {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub unit_id: Uuid,

    // Applicant info
    pub applicant_name: String,
    pub applicant_email: String,
    pub applicant_phone: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub national_id: Option<String>,

    // Current address
    pub current_address: Option<String>,
    pub current_landlord_name: Option<String>,
    pub current_landlord_phone: Option<String>,
    pub current_rent_amount: Option<Decimal>,
    pub current_tenancy_start: Option<NaiveDate>,

    // Employment
    pub employer_name: Option<String>,
    pub employer_phone: Option<String>,
    pub job_title: Option<String>,
    pub employment_start: Option<NaiveDate>,
    pub monthly_income: Option<Decimal>,

    // Desired lease
    pub desired_move_in: Option<NaiveDate>,
    pub desired_lease_term_months: Option<i32>,
    pub proposed_rent: Option<Decimal>,

    // Co-applicants (JSON)
    pub co_applicants: Option<serde_json::Value>,

    // Documents (JSON)
    pub documents: Option<serde_json::Value>,

    // Status
    pub status: String,
    pub submitted_at: Option<DateTime<Utc>>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub decision_notes: Option<String>,

    // Metadata
    pub source: Option<String>,
    pub referral_code: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Tenant screening entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TenantScreening {
    pub id: Uuid,
    pub application_id: Uuid,
    pub organization_id: Uuid,

    pub screening_type: String,
    pub provider: Option<String>,

    // Consent
    pub consent_requested_at: Option<DateTime<Utc>>,
    pub consent_received_at: Option<DateTime<Utc>>,
    pub consent_document_url: Option<String>,

    // Status
    pub status: String,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,

    // Results
    pub result_summary: Option<String>,
    pub risk_score: Option<i32>,
    pub passed: Option<bool>,
    pub flags: Option<serde_json::Value>,

    pub expires_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Lease template entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LeaseTemplate {
    pub id: Uuid,
    pub organization_id: Uuid,

    pub name: String,
    pub description: Option<String>,

    pub content_html: String,
    pub content_variables: Option<serde_json::Value>,

    pub default_term_months: Option<i32>,
    pub default_security_deposit_months: Option<Decimal>,
    pub default_notice_period_days: Option<i32>,

    pub clauses: Option<serde_json::Value>,

    pub is_default: bool,
    pub is_active: bool,
    pub version: i32,

    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Lease entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Lease {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub unit_id: Uuid,
    pub application_id: Option<Uuid>,
    pub template_id: Option<Uuid>,

    // Landlord
    pub landlord_user_id: Option<Uuid>,
    pub landlord_name: String,
    pub landlord_address: Option<String>,

    // Tenant
    pub tenant_user_id: Option<Uuid>,
    pub tenant_name: String,
    pub tenant_email: String,
    pub tenant_phone: Option<String>,

    // Occupants (JSON)
    pub occupants: Option<serde_json::Value>,

    // Terms
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub term_months: i32,
    pub is_fixed_term: bool,

    // Financial
    pub monthly_rent: Decimal,
    pub security_deposit: Decimal,
    pub deposit_held_by: Option<String>,
    pub rent_due_day: i32,
    pub late_fee_amount: Option<Decimal>,
    pub late_fee_grace_days: Option<i32>,

    // Utilities & inclusions (JSON)
    pub utilities_included: Option<serde_json::Value>,
    pub parking_spaces: i32,
    pub storage_units: i32,

    // Rules
    pub pets_allowed: bool,
    pub pet_deposit: Option<Decimal>,
    pub max_occupants: Option<i32>,
    pub smoking_allowed: bool,

    // Document
    pub document_url: Option<String>,
    pub document_version: i32,

    // Status
    pub status: String,

    // Signatures
    pub landlord_signed_at: Option<DateTime<Utc>>,
    pub tenant_signed_at: Option<DateTime<Utc>>,
    pub signature_request_id: Option<Uuid>,

    // Termination
    pub terminated_at: Option<DateTime<Utc>>,
    pub termination_reason: Option<String>,
    pub termination_notes: Option<String>,
    pub termination_initiated_by: Option<Uuid>,

    // Renewal
    pub previous_lease_id: Option<Uuid>,
    pub renewed_to_lease_id: Option<Uuid>,
    pub renewal_offered_at: Option<DateTime<Utc>>,
    pub renewal_offer_expires_at: Option<DateTime<Utc>>,

    // Metadata
    pub notes: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Lease amendment entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LeaseAmendment {
    pub id: Uuid,
    pub lease_id: Uuid,

    pub amendment_number: i32,
    pub title: String,
    pub description: Option<String>,

    pub changes: serde_json::Value,
    pub effective_date: NaiveDate,

    pub document_url: Option<String>,

    pub landlord_signed_at: Option<DateTime<Utc>>,
    pub tenant_signed_at: Option<DateTime<Utc>>,

    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Lease payment entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LeasePayment {
    pub id: Uuid,
    pub lease_id: Uuid,
    pub organization_id: Uuid,

    pub due_date: NaiveDate,
    pub amount: Decimal,
    pub payment_type: String,
    pub description: Option<String>,

    pub paid_at: Option<DateTime<Utc>>,
    pub paid_amount: Option<Decimal>,
    pub payment_method: Option<String>,
    pub payment_reference: Option<String>,

    pub is_late: bool,
    pub late_fee_applied: Option<Decimal>,

    pub invoice_id: Option<Uuid>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Lease reminder entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LeaseReminder {
    pub id: Uuid,
    pub lease_id: Uuid,

    pub reminder_type: String,

    pub trigger_date: NaiveDate,
    pub days_before_event: Option<i32>,

    pub subject: String,
    pub message: String,

    pub recipients: serde_json::Value,

    pub sent_at: Option<DateTime<Utc>>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub acknowledged_by: Option<Uuid>,

    pub is_recurring: bool,
    pub recurrence_pattern: Option<String>,

    pub created_at: DateTime<Utc>,
}

// =============================================================================
// DTOs
// =============================================================================

/// Create tenant application request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateApplication {
    pub unit_id: Uuid,
    pub applicant_name: String,
    pub applicant_email: String,
    pub applicant_phone: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub national_id: Option<String>,
    pub current_address: Option<String>,
    pub current_landlord_name: Option<String>,
    pub current_landlord_phone: Option<String>,
    pub current_rent_amount: Option<Decimal>,
    pub current_tenancy_start: Option<NaiveDate>,
    pub employer_name: Option<String>,
    pub employer_phone: Option<String>,
    pub job_title: Option<String>,
    pub employment_start: Option<NaiveDate>,
    pub monthly_income: Option<Decimal>,
    pub desired_move_in: Option<NaiveDate>,
    pub desired_lease_term_months: Option<i32>,
    pub proposed_rent: Option<Decimal>,
    pub co_applicants: Option<serde_json::Value>,
    pub source: Option<String>,
    pub referral_code: Option<String>,
}

/// Update tenant application request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateApplication {
    pub applicant_name: Option<String>,
    pub applicant_email: Option<String>,
    pub applicant_phone: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub national_id: Option<String>,
    pub current_address: Option<String>,
    pub current_landlord_name: Option<String>,
    pub current_landlord_phone: Option<String>,
    pub current_rent_amount: Option<Decimal>,
    pub current_tenancy_start: Option<NaiveDate>,
    pub employer_name: Option<String>,
    pub employer_phone: Option<String>,
    pub job_title: Option<String>,
    pub employment_start: Option<NaiveDate>,
    pub monthly_income: Option<Decimal>,
    pub desired_move_in: Option<NaiveDate>,
    pub desired_lease_term_months: Option<i32>,
    pub proposed_rent: Option<Decimal>,
    pub co_applicants: Option<serde_json::Value>,
}

/// Submit application request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct SubmitApplication {
    pub documents: Option<serde_json::Value>,
}

/// Review application request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ReviewApplication {
    pub status: String, // 'approved', 'rejected'
    pub decision_notes: Option<String>,
}

/// Initiate screening request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct InitiateScreening {
    pub screening_types: Vec<String>,
    pub provider: Option<String>,
}

/// Submit screening consent request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ScreeningConsent {
    pub consent_document_url: Option<String>,
}

/// Update screening result request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateScreeningResult {
    pub result_summary: Option<String>,
    pub risk_score: Option<i32>,
    pub passed: Option<bool>,
    pub flags: Option<serde_json::Value>,
}

/// Create lease template request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateLeaseTemplate {
    pub name: String,
    pub description: Option<String>,
    pub content_html: String,
    pub content_variables: Option<serde_json::Value>,
    pub default_term_months: Option<i32>,
    pub default_security_deposit_months: Option<Decimal>,
    pub default_notice_period_days: Option<i32>,
    pub clauses: Option<serde_json::Value>,
    pub is_default: Option<bool>,
}

/// Update lease template request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateLeaseTemplate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub content_html: Option<String>,
    pub content_variables: Option<serde_json::Value>,
    pub default_term_months: Option<i32>,
    pub default_security_deposit_months: Option<Decimal>,
    pub default_notice_period_days: Option<i32>,
    pub clauses: Option<serde_json::Value>,
    pub is_default: Option<bool>,
    pub is_active: Option<bool>,
}

/// Create lease request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateLease {
    pub unit_id: Uuid,
    pub application_id: Option<Uuid>,
    pub template_id: Option<Uuid>,

    pub landlord_user_id: Option<Uuid>,
    pub landlord_name: String,
    pub landlord_address: Option<String>,

    pub tenant_user_id: Option<Uuid>,
    pub tenant_name: String,
    pub tenant_email: String,
    pub tenant_phone: Option<String>,

    pub occupants: Option<serde_json::Value>,

    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub term_months: i32,
    pub is_fixed_term: Option<bool>,

    pub monthly_rent: Decimal,
    pub security_deposit: Decimal,
    pub deposit_held_by: Option<String>,
    pub rent_due_day: Option<i32>,
    pub late_fee_amount: Option<Decimal>,
    pub late_fee_grace_days: Option<i32>,

    pub utilities_included: Option<serde_json::Value>,
    pub parking_spaces: Option<i32>,
    pub storage_units: Option<i32>,

    pub pets_allowed: Option<bool>,
    pub pet_deposit: Option<Decimal>,
    pub max_occupants: Option<i32>,
    pub smoking_allowed: Option<bool>,

    pub notes: Option<String>,
}

/// Update lease request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateLease {
    pub landlord_address: Option<String>,
    pub tenant_phone: Option<String>,
    pub occupants: Option<serde_json::Value>,
    pub utilities_included: Option<serde_json::Value>,
    pub parking_spaces: Option<i32>,
    pub storage_units: Option<i32>,
    pub pets_allowed: Option<bool>,
    pub pet_deposit: Option<Decimal>,
    pub max_occupants: Option<i32>,
    pub smoking_allowed: Option<bool>,
    pub notes: Option<String>,
}

/// Terminate lease request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct TerminateLease {
    pub termination_reason: String,
    pub termination_notes: Option<String>,
    pub effective_date: Option<NaiveDate>,
}

/// Renew lease request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct RenewLease {
    pub new_end_date: NaiveDate,
    pub new_monthly_rent: Option<Decimal>,
    pub new_security_deposit: Option<Decimal>,
    pub term_months: i32,
    pub notes: Option<String>,
}

/// Create lease amendment request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateAmendment {
    pub title: String,
    pub description: Option<String>,
    pub changes: serde_json::Value,
    pub effective_date: NaiveDate,
}

/// Record payment request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct RecordPayment {
    pub paid_amount: Decimal,
    pub payment_method: String,
    pub payment_reference: Option<String>,
    pub paid_at: Option<DateTime<Utc>>,
}

/// Create reminder request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateReminder {
    pub reminder_type: String,
    pub trigger_date: NaiveDate,
    pub days_before_event: Option<i32>,
    pub subject: String,
    pub message: String,
    pub recipients: serde_json::Value,
    pub is_recurring: Option<bool>,
    pub recurrence_pattern: Option<String>,
}

// =============================================================================
// Summary Types
// =============================================================================

/// Application summary for lists.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ApplicationSummary {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub unit_name: String,
    pub building_name: String,
    pub applicant_name: String,
    pub applicant_email: String,
    pub status: String,
    pub submitted_at: Option<DateTime<Utc>>,
    pub monthly_income: Option<Decimal>,
    pub desired_move_in: Option<NaiveDate>,
    pub screening_count: i64,
    pub screening_passed: bool,
}

/// Screening summary for lists.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ScreeningSummary {
    pub id: Uuid,
    pub screening_type: String,
    pub provider: Option<String>,
    pub status: String,
    pub risk_score: Option<i32>,
    pub passed: Option<bool>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Lease summary for lists.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LeaseSummary {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub unit_name: String,
    pub building_name: String,
    pub tenant_name: String,
    pub tenant_email: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub monthly_rent: Decimal,
    pub status: String,
    pub days_until_expiry: i64,
}

/// Lease with full details.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LeaseWithDetails {
    pub lease: Lease,
    pub unit_name: String,
    pub building_name: String,
    pub amendments: Vec<LeaseAmendment>,
    pub upcoming_payments: Vec<LeasePayment>,
    pub reminders: Vec<LeaseReminder>,
}

/// Payment summary.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PaymentSummary {
    pub id: Uuid,
    pub due_date: NaiveDate,
    pub amount: Decimal,
    pub payment_type: String,
    pub paid_at: Option<DateTime<Utc>>,
    pub paid_amount: Option<Decimal>,
    pub is_late: bool,
    pub late_fee_applied: Option<Decimal>,
}

/// Lease expiration overview.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ExpirationOverview {
    pub expiring_30_days: Vec<LeaseSummary>,
    pub expiring_60_days: Vec<LeaseSummary>,
    pub expiring_90_days: Vec<LeaseSummary>,
    pub total_active_leases: i64,
    pub total_expiring_soon: i64,
}

/// Lease statistics.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LeaseStatistics {
    pub total_leases: i64,
    pub active_leases: i64,
    pub pending_signatures: i64,
    pub expiring_soon: i64,
    pub total_applications: i64,
    pub pending_applications: i64,
    pub total_monthly_rent: Decimal,
    pub occupancy_rate: f64,
}

/// Application list query params.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ApplicationListQuery {
    pub unit_id: Option<Uuid>,
    pub status: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Lease list query params.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct LeaseListQuery {
    pub unit_id: Option<Uuid>,
    pub tenant_id: Option<Uuid>,
    pub status: Option<String>,
    pub expiring_within_days: Option<i32>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}
