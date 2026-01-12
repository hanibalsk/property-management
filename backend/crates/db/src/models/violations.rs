//! Violation Tracking & Enforcement models for Epic 142.
//!
//! Provides types for HOA/condo rule violation tracking, fines, and enforcement.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

// =============================================================================
// ENUMS
// =============================================================================

/// Category of violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "violation_category", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ViolationCategory {
    Noise,
    Parking,
    Pet,
    Maintenance,
    Architectural,
    CommonArea,
    Lease,
    Payment,
    Safety,
    Other,
}

/// Severity level of violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "violation_severity", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ViolationSeverity {
    Minor,
    Moderate,
    Major,
    Critical,
}

/// Status of a violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "violation_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ViolationStatus {
    Reported,
    UnderReview,
    Confirmed,
    Disputed,
    Resolved,
    Dismissed,
    Escalated,
}

/// Type of enforcement action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "enforcement_action_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EnforcementActionType {
    Warning,
    FirstFine,
    IncreasedFine,
    PrivilegeSuspension,
    LegalAction,
    Lien,
    Other,
}

/// Status of an enforcement action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "enforcement_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EnforcementStatus {
    Pending,
    Sent,
    Acknowledged,
    Paid,
    Appealed,
    Completed,
    Cancelled,
}

/// Status of an appeal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "appeal_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AppealStatus {
    Submitted,
    UnderReview,
    HearingScheduled,
    Approved,
    Denied,
    Withdrawn,
}

// =============================================================================
// ENTITY TYPES
// =============================================================================

/// Community rule definition.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CommunityRule {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,
    pub rule_code: String,
    pub title: String,
    pub description: Option<String>,
    pub category: ViolationCategory,
    pub first_offense_fine: Option<Decimal>,
    pub second_offense_fine: Option<Decimal>,
    pub third_offense_fine: Option<Decimal>,
    pub max_fine: Option<Decimal>,
    pub fine_escalation_days: Option<i32>,
    pub effective_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub is_active: bool,
    pub requires_board_approval: bool,
    pub source_document_id: Option<Uuid>,
    pub section_reference: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

/// Violation report.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Violation {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,
    pub unit_id: Option<Uuid>,
    pub violation_number: String,
    pub rule_id: Option<Uuid>,
    pub category: ViolationCategory,
    pub severity: ViolationSeverity,
    pub status: ViolationStatus,
    pub title: String,
    pub description: String,
    pub location: Option<String>,
    pub violator_id: Option<Uuid>,
    pub violator_name: Option<String>,
    pub violator_unit: Option<String>,
    pub reporter_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub occurred_at: DateTime<Utc>,
    pub reported_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub evidence_description: Option<String>,
    pub witness_count: i32,
    pub resolution_notes: Option<String>,
    pub resolution_type: Option<String>,
    pub offense_number: i32,
    pub previous_violation_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Evidence attached to a violation.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ViolationEvidence {
    pub id: Uuid,
    pub violation_id: Uuid,
    pub file_name: String,
    pub file_type: String,
    pub file_size: Option<i32>,
    pub storage_path: Option<String>,
    pub description: Option<String>,
    pub captured_at: Option<DateTime<Utc>>,
    pub uploaded_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Enforcement action (warning, fine, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EnforcementAction {
    pub id: Uuid,
    pub violation_id: Uuid,
    pub organization_id: Uuid,
    pub action_type: EnforcementActionType,
    pub status: EnforcementStatus,
    pub fine_amount: Option<Decimal>,
    pub due_date: Option<NaiveDate>,
    pub paid_amount: Option<Decimal>,
    pub paid_at: Option<DateTime<Utc>>,
    pub notice_sent_at: Option<DateTime<Utc>>,
    pub notice_method: Option<String>,
    pub notice_document_id: Option<Uuid>,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub suspended_privileges: Option<Vec<String>>,
    pub suspension_start: Option<NaiveDate>,
    pub suspension_end: Option<NaiveDate>,
    pub issued_by: Option<Uuid>,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Appeal against a violation or enforcement action.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ViolationAppeal {
    pub id: Uuid,
    pub violation_id: Uuid,
    pub enforcement_action_id: Option<Uuid>,
    pub organization_id: Uuid,
    pub appeal_number: String,
    pub status: AppealStatus,
    pub reason: String,
    pub requested_outcome: Option<String>,
    pub supporting_evidence: Option<String>,
    pub appellant_id: Uuid,
    pub hearing_date: Option<DateTime<Utc>>,
    pub hearing_location: Option<String>,
    pub hearing_notes: Option<String>,
    pub decision: Option<String>,
    pub decision_date: Option<DateTime<Utc>>,
    pub decided_by: Option<Uuid>,
    pub fine_adjustment: Option<Decimal>,
    pub submitted_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Comment or activity on a violation.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ViolationComment {
    pub id: Uuid,
    pub violation_id: Uuid,
    pub comment_type: String,
    pub content: String,
    pub is_internal: bool,
    pub author_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Notification sent regarding a violation.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ViolationNotification {
    pub id: Uuid,
    pub violation_id: Uuid,
    pub notification_type: String,
    pub recipient_id: Option<Uuid>,
    pub recipient_email: Option<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub sent_at: DateTime<Utc>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub read_at: Option<DateTime<Utc>>,
    pub delivery_status: String,
}

/// Payment record for a fine.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FinePayment {
    pub id: Uuid,
    pub enforcement_action_id: Uuid,
    pub organization_id: Uuid,
    pub amount: Decimal,
    pub payment_method: Option<String>,
    pub transaction_reference: Option<String>,
    pub payer_id: Option<Uuid>,
    pub payer_name: Option<String>,
    pub status: String,
    pub processed_at: DateTime<Utc>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub recorded_by: Option<Uuid>,
}

/// Pre-calculated violation statistics.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ViolationStatistics {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub period_type: String,
    pub total_violations: i32,
    pub new_violations: i32,
    pub resolved_violations: i32,
    pub dismissed_violations: i32,
    pub noise_violations: i32,
    pub parking_violations: i32,
    pub pet_violations: i32,
    pub maintenance_violations: i32,
    pub architectural_violations: i32,
    pub other_violations: i32,
    pub total_fines_assessed: Decimal,
    pub total_fines_collected: Decimal,
    pub total_fines_waived: Decimal,
    pub total_appeals: i32,
    pub appeals_approved: i32,
    pub appeals_denied: i32,
    pub avg_resolution_time: Option<Decimal>,
    pub calculated_at: DateTime<Utc>,
}

// =============================================================================
// CREATE DTOs
// =============================================================================

/// Create a new community rule.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCommunityRule {
    pub building_id: Option<Uuid>,
    pub rule_code: String,
    pub title: String,
    pub description: Option<String>,
    pub category: ViolationCategory,
    pub first_offense_fine: Option<Decimal>,
    pub second_offense_fine: Option<Decimal>,
    pub third_offense_fine: Option<Decimal>,
    pub max_fine: Option<Decimal>,
    pub fine_escalation_days: Option<i32>,
    pub effective_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
    pub requires_board_approval: Option<bool>,
    pub source_document_id: Option<Uuid>,
    pub section_reference: Option<String>,
}

/// Create a new violation.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateViolation {
    pub building_id: Option<Uuid>,
    pub unit_id: Option<Uuid>,
    pub rule_id: Option<Uuid>,
    pub category: ViolationCategory,
    pub severity: Option<ViolationSeverity>,
    pub title: String,
    pub description: String,
    pub location: Option<String>,
    pub violator_id: Option<Uuid>,
    pub violator_name: Option<String>,
    pub violator_unit: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub evidence_description: Option<String>,
    pub witness_count: Option<i32>,
}

/// Create violation evidence.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateViolationEvidence {
    pub file_name: String,
    pub file_type: String,
    pub file_size: Option<i32>,
    pub storage_path: Option<String>,
    pub description: Option<String>,
    pub captured_at: Option<DateTime<Utc>>,
}

/// Create an enforcement action.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateEnforcementAction {
    pub action_type: EnforcementActionType,
    pub fine_amount: Option<Decimal>,
    pub due_date: Option<NaiveDate>,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub suspended_privileges: Option<Vec<String>>,
    pub suspension_start: Option<NaiveDate>,
    pub suspension_end: Option<NaiveDate>,
}

/// Create an appeal.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateViolationAppeal {
    pub enforcement_action_id: Option<Uuid>,
    pub reason: String,
    pub requested_outcome: Option<String>,
    pub supporting_evidence: Option<String>,
}

/// Create a violation comment.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateViolationComment {
    pub comment_type: String,
    pub content: String,
    pub is_internal: Option<bool>,
}

/// Record a fine payment.
#[derive(Debug, Clone, Deserialize)]
pub struct RecordFinePayment {
    pub amount: Decimal,
    pub payment_method: Option<String>,
    pub transaction_reference: Option<String>,
    pub payer_id: Option<Uuid>,
    pub payer_name: Option<String>,
    pub notes: Option<String>,
}

// =============================================================================
// UPDATE DTOs
// =============================================================================

/// Update a community rule.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCommunityRule {
    pub title: Option<String>,
    pub description: Option<String>,
    pub first_offense_fine: Option<Decimal>,
    pub second_offense_fine: Option<Decimal>,
    pub third_offense_fine: Option<Decimal>,
    pub max_fine: Option<Decimal>,
    pub fine_escalation_days: Option<i32>,
    pub expiry_date: Option<NaiveDate>,
    pub is_active: Option<bool>,
    pub requires_board_approval: Option<bool>,
}

/// Update a violation.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateViolation {
    pub severity: Option<ViolationSeverity>,
    pub status: Option<ViolationStatus>,
    pub assigned_to: Option<Uuid>,
    pub resolution_notes: Option<String>,
    pub resolution_type: Option<String>,
}

/// Update an enforcement action.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateEnforcementAction {
    pub status: Option<EnforcementStatus>,
    pub notice_sent_at: Option<DateTime<Utc>>,
    pub notice_method: Option<String>,
    pub notes: Option<String>,
}

/// Update an appeal.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateViolationAppeal {
    pub status: Option<AppealStatus>,
    pub hearing_date: Option<DateTime<Utc>>,
    pub hearing_location: Option<String>,
    pub hearing_notes: Option<String>,
    pub decision: Option<String>,
    pub fine_adjustment: Option<Decimal>,
}

// =============================================================================
// QUERY DTOs
// =============================================================================

/// Query parameters for listing violations.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ViolationQuery {
    pub building_id: Option<Uuid>,
    pub unit_id: Option<Uuid>,
    pub category: Option<ViolationCategory>,
    pub severity: Option<ViolationSeverity>,
    pub status: Option<ViolationStatus>,
    pub violator_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Query parameters for listing enforcement actions.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct EnforcementQuery {
    pub violation_id: Option<Uuid>,
    pub action_type: Option<EnforcementActionType>,
    pub status: Option<EnforcementStatus>,
    pub overdue_only: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Query parameters for listing appeals.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AppealQuery {
    pub status: Option<AppealStatus>,
    pub appellant_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// =============================================================================
// RESPONSE DTOs
// =============================================================================

/// Violation summary for list views.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ViolationSummary {
    pub id: Uuid,
    pub violation_number: String,
    pub title: String,
    pub category: ViolationCategory,
    pub severity: ViolationSeverity,
    pub status: ViolationStatus,
    pub violator_name: Option<String>,
    pub violator_unit: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub has_fine: bool,
    pub fine_amount: Option<Decimal>,
}

/// Detailed violation with related data.
#[derive(Debug, Clone, Serialize)]
pub struct ViolationDetail {
    pub violation: Violation,
    pub rule: Option<CommunityRule>,
    pub evidence: Vec<ViolationEvidence>,
    pub enforcement_actions: Vec<EnforcementAction>,
    pub comments: Vec<ViolationComment>,
    pub appeal: Option<ViolationAppeal>,
}

/// Dashboard summary for violations.
#[derive(Debug, Clone, Serialize)]
pub struct ViolationDashboard {
    pub total_open: i32,
    pub reported_this_month: i32,
    pub resolved_this_month: i32,
    pub pending_fines: Decimal,
    pub collected_this_month: Decimal,
    pub by_category: Vec<CategoryCount>,
    pub by_status: Vec<StatusCount>,
    pub recent_violations: Vec<ViolationSummary>,
}

/// Category count for dashboard.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct CategoryCount {
    pub category: ViolationCategory,
    pub count: i64,
}

/// Status count for dashboard.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct StatusCount {
    pub status: ViolationStatus,
    pub count: i64,
}

/// Violator history for repeat offense tracking.
#[derive(Debug, Clone, Serialize)]
pub struct ViolatorHistory {
    pub violator_id: Option<Uuid>,
    pub violator_name: Option<String>,
    pub unit_id: Option<Uuid>,
    pub total_violations: i32,
    pub violations_by_category: Vec<CategoryCount>,
    pub total_fines: Decimal,
    pub outstanding_fines: Decimal,
    pub recent_violations: Vec<ViolationSummary>,
}

/// Rule compliance summary.
#[derive(Debug, Clone, Serialize)]
pub struct RuleComplianceSummary {
    pub rule: CommunityRule,
    pub total_violations: i32,
    pub violations_this_year: i32,
    pub fines_collected: Decimal,
    pub most_violated_building: Option<String>,
}
