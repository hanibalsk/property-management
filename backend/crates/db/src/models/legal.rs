//! Legal document and compliance models (Epic 25).

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Legal document type constants.
pub mod document_type {
    pub const CONTRACT: &str = "contract";
    pub const LEASE_TEMPLATE: &str = "lease_template";
    pub const REGULATION: &str = "regulation";
    pub const COURT_ORDER: &str = "court_order";
    pub const LEGAL_NOTICE: &str = "legal_notice";
    pub const MINUTES: &str = "minutes";
    pub const POLICY: &str = "policy";
    pub const PERMIT: &str = "permit";
    pub const CERTIFICATE: &str = "certificate";
    pub const OTHER: &str = "other";
    pub const ALL: &[&str] = &[
        CONTRACT,
        LEASE_TEMPLATE,
        REGULATION,
        COURT_ORDER,
        LEGAL_NOTICE,
        MINUTES,
        POLICY,
        PERMIT,
        CERTIFICATE,
        OTHER,
    ];
}

/// Compliance category constants.
pub mod compliance_category {
    pub const FIRE_SAFETY: &str = "fire_safety";
    pub const ELEVATOR: &str = "elevator";
    pub const ELECTRICAL: &str = "electrical";
    pub const PLUMBING: &str = "plumbing";
    pub const ACCESSIBILITY: &str = "accessibility";
    pub const ENVIRONMENTAL: &str = "environmental";
    pub const HEALTH: &str = "health";
    pub const PRIVACY: &str = "privacy";
    pub const OTHER: &str = "other";
    pub const ALL: &[&str] = &[
        FIRE_SAFETY,
        ELEVATOR,
        ELECTRICAL,
        PLUMBING,
        ACCESSIBILITY,
        ENVIRONMENTAL,
        HEALTH,
        PRIVACY,
        OTHER,
    ];
}

/// Compliance frequency constants.
pub mod compliance_frequency {
    pub const ONCE: &str = "once";
    pub const MONTHLY: &str = "monthly";
    pub const QUARTERLY: &str = "quarterly";
    pub const SEMI_ANNUALLY: &str = "semi_annually";
    pub const ANNUALLY: &str = "annually";
    pub const BIENNIALLY: &str = "biennially";
    pub const AS_NEEDED: &str = "as_needed";
    pub const ALL: &[&str] = &[
        ONCE,
        MONTHLY,
        QUARTERLY,
        SEMI_ANNUALLY,
        ANNUALLY,
        BIENNIALLY,
        AS_NEEDED,
    ];
}

/// Compliance status constants.
pub mod compliance_status {
    pub const PENDING: &str = "pending";
    pub const COMPLIANT: &str = "compliant";
    pub const NON_COMPLIANT: &str = "non_compliant";
    pub const IN_PROGRESS: &str = "in_progress";
    pub const WAIVED: &str = "waived";
    pub const EXPIRED: &str = "expired";
    pub const ALL: &[&str] = &[
        PENDING,
        COMPLIANT,
        NON_COMPLIANT,
        IN_PROGRESS,
        WAIVED,
        EXPIRED,
    ];
}

/// Notice type constants.
pub mod notice_type {
    pub const RENT_INCREASE: &str = "rent_increase";
    pub const LEASE_VIOLATION: &str = "lease_violation";
    pub const EVICTION_WARNING: &str = "eviction_warning";
    pub const RULES_UPDATE: &str = "rules_update";
    pub const GENERAL_NOTICE: &str = "general_notice";
    pub const ASSEMBLY_MEETING: &str = "assembly_meeting";
    pub const OTHER: &str = "other";
    pub const ALL: &[&str] = &[
        RENT_INCREASE,
        LEASE_VIOLATION,
        EVICTION_WARNING,
        RULES_UPDATE,
        GENERAL_NOTICE,
        ASSEMBLY_MEETING,
        OTHER,
    ];
}

/// Notice priority constants.
pub mod notice_priority {
    pub const LOW: &str = "low";
    pub const NORMAL: &str = "normal";
    pub const HIGH: &str = "high";
    pub const URGENT: &str = "urgent";
    pub const ALL: &[&str] = &[LOW, NORMAL, HIGH, URGENT];
}

/// Notice delivery method constants.
pub mod delivery_method {
    pub const EMAIL: &str = "email";
    pub const MAIL: &str = "mail";
    pub const BOTH: &str = "both";
    pub const IN_APP: &str = "in_app";
    pub const ALL: &[&str] = &[EMAIL, MAIL, BOTH, IN_APP];
}

/// Notice recipient type constants.
pub mod recipient_type {
    pub const USER: &str = "user";
    pub const UNIT: &str = "unit";
    pub const BUILDING: &str = "building";
    pub const ALL: &str = "all";
    pub const ALL_TYPES: &[&str] = &[USER, UNIT, BUILDING, ALL];
}

/// Delivery status constants.
pub mod delivery_status {
    pub const PENDING: &str = "pending";
    pub const SENT: &str = "sent";
    pub const DELIVERED: &str = "delivered";
    pub const FAILED: &str = "failed";
    pub const BOUNCED: &str = "bounced";
    pub const ALL: &[&str] = &[PENDING, SENT, DELIVERED, FAILED, BOUNCED];
}

// ==================== Legal Document ====================

/// Legal document entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LegalDocument {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,

    // Document details
    pub document_type: String,
    pub title: String,
    pub description: Option<String>,

    // Parties involved
    pub parties: Option<serde_json::Value>,

    // Dates
    pub effective_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,

    // File information
    pub file_path: Option<String>,
    pub file_name: Option<String>,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,

    // Confidentiality
    pub is_confidential: Option<bool>,

    // Retention
    pub retention_period_months: Option<i32>,
    pub retention_expires_at: Option<NaiveDate>,

    // Metadata
    pub tags: Option<Vec<String>>,
    pub metadata: Option<serde_json::Value>,

    // Audit
    pub created_by: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create legal document request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateLegalDocument {
    pub building_id: Option<Uuid>,
    pub document_type: String,
    pub title: String,
    pub description: Option<String>,
    pub parties: Option<serde_json::Value>,
    pub effective_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
    pub file_path: Option<String>,
    pub file_name: Option<String>,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub is_confidential: Option<bool>,
    pub retention_period_months: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<serde_json::Value>,
}

/// Update legal document request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateLegalDocument {
    pub building_id: Option<Uuid>,
    pub document_type: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub parties: Option<serde_json::Value>,
    pub effective_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
    pub file_path: Option<String>,
    pub file_name: Option<String>,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub is_confidential: Option<bool>,
    pub retention_period_months: Option<i32>,
    pub retention_expires_at: Option<NaiveDate>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<serde_json::Value>,
}

/// Legal document query parameters.
#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct LegalDocumentQuery {
    pub building_id: Option<Uuid>,
    pub document_type: Option<String>,
    pub is_confidential: Option<bool>,
    pub expiring_days: Option<i32>,
    pub tag: Option<String>,
    pub search: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

// ==================== Legal Document Version ====================

/// Legal document version entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LegalDocumentVersion {
    pub id: Uuid,
    pub document_id: Uuid,
    pub version_number: i32,

    // File information
    pub file_path: String,
    pub file_name: String,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,

    // Notes
    pub change_notes: Option<String>,

    // Audit
    pub created_by: Uuid,
    pub created_at: Option<DateTime<Utc>>,
}

/// Create legal document version request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateLegalDocumentVersion {
    pub file_path: String,
    pub file_name: String,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub change_notes: Option<String>,
}

// ==================== Compliance Requirement ====================

/// Compliance requirement entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ComplianceRequirement {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,

    // Requirement details
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub regulation_reference: Option<String>,

    // Schedule
    pub frequency: String,
    pub last_verified_at: Option<DateTime<Utc>>,
    pub last_verified_by: Option<Uuid>,
    pub next_due_date: Option<NaiveDate>,

    // Status
    pub status: String,
    pub is_mandatory: Option<bool>,

    // Responsibility
    pub responsible_party: Option<String>,
    pub notes: Option<String>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create compliance requirement request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateComplianceRequirement {
    pub building_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub regulation_reference: Option<String>,
    pub frequency: Option<String>,
    pub next_due_date: Option<NaiveDate>,
    pub is_mandatory: Option<bool>,
    pub responsible_party: Option<String>,
    pub notes: Option<String>,
}

/// Update compliance requirement request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateComplianceRequirement {
    pub building_id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub regulation_reference: Option<String>,
    pub frequency: Option<String>,
    pub next_due_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub is_mandatory: Option<bool>,
    pub responsible_party: Option<String>,
    pub notes: Option<String>,
}

/// Compliance requirement query parameters.
#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct ComplianceQuery {
    pub building_id: Option<Uuid>,
    pub category: Option<String>,
    pub status: Option<String>,
    pub is_mandatory: Option<bool>,
    pub due_before: Option<NaiveDate>,
    pub overdue_only: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

// ==================== Compliance Verification ====================

/// Compliance verification entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ComplianceVerification {
    pub id: Uuid,
    pub requirement_id: Uuid,

    // Verification details
    pub verified_at: Option<DateTime<Utc>>,
    pub verified_by: Uuid,
    pub status: String,
    pub notes: Option<String>,

    // Evidence
    pub evidence_document_id: Option<Uuid>,

    // Inspector
    pub inspector_name: Option<String>,
    pub certificate_number: Option<String>,
    pub valid_until: Option<NaiveDate>,

    // Issues
    pub issues_found: Option<String>,
    pub corrective_actions: Option<String>,

    pub created_at: Option<DateTime<Utc>>,
}

/// Create compliance verification request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateComplianceVerification {
    pub status: String,
    pub notes: Option<String>,
    pub evidence_document_id: Option<Uuid>,
    pub inspector_name: Option<String>,
    pub certificate_number: Option<String>,
    pub valid_until: Option<NaiveDate>,
    pub issues_found: Option<String>,
    pub corrective_actions: Option<String>,
}

// ==================== Legal Notice ====================

/// Legal notice entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LegalNotice {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,

    // Notice details
    pub notice_type: String,
    pub subject: String,
    pub content: String,
    pub priority: Option<String>,
    pub delivery_method: String,

    // Acknowledgment
    pub requires_acknowledgment: Option<bool>,
    pub acknowledgment_deadline: Option<DateTime<Utc>>,

    // Sent info
    pub sent_at: Option<DateTime<Utc>>,

    // Audit
    pub created_by: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create legal notice request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateLegalNotice {
    pub building_id: Option<Uuid>,
    pub notice_type: String,
    pub subject: String,
    pub content: String,
    pub priority: Option<String>,
    pub delivery_method: Option<String>,
    pub requires_acknowledgment: Option<bool>,
    pub acknowledgment_deadline: Option<DateTime<Utc>>,
    pub recipient_ids: Vec<NoticeRecipientInput>,
}

/// Notice recipient input.
#[derive(Debug, Deserialize, ToSchema)]
pub struct NoticeRecipientInput {
    pub recipient_type: String,
    pub recipient_id: Uuid,
    pub recipient_name: Option<String>,
    pub recipient_email: Option<String>,
}

/// Update legal notice request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateLegalNotice {
    pub subject: Option<String>,
    pub content: Option<String>,
    pub priority: Option<String>,
    pub delivery_method: Option<String>,
    pub requires_acknowledgment: Option<bool>,
    pub acknowledgment_deadline: Option<DateTime<Utc>>,
}

/// Legal notice query parameters.
#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct LegalNoticeQuery {
    pub building_id: Option<Uuid>,
    pub notice_type: Option<String>,
    pub priority: Option<String>,
    pub sent: Option<bool>,
    pub requires_acknowledgment: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

// ==================== Legal Notice Recipient ====================

/// Legal notice recipient entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LegalNoticeRecipient {
    pub id: Uuid,
    pub notice_id: Uuid,
    pub recipient_type: String,
    pub recipient_id: Uuid,
    pub recipient_name: Option<String>,
    pub recipient_email: Option<String>,

    // Delivery
    pub delivery_status: Option<String>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub delivery_error: Option<String>,

    // Acknowledgment
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub acknowledgment_method: Option<String>,

    pub created_at: Option<DateTime<Utc>>,
}

/// Acknowledge notice request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct AcknowledgeNotice {
    pub acknowledgment_method: Option<String>,
}

// ==================== Compliance Template ====================

/// Compliance template entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ComplianceTemplate {
    pub id: Uuid,
    pub organization_id: Option<Uuid>,

    // Template details
    pub name: String,
    pub category: String,
    pub description: Option<String>,

    // Checklist
    pub checklist_items: Option<serde_json::Value>,

    // Schedule
    pub frequency: String,

    // System template flag
    pub is_system: Option<bool>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create compliance template request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateComplianceTemplate {
    pub name: String,
    pub category: String,
    pub description: Option<String>,
    pub checklist_items: Option<serde_json::Value>,
    pub frequency: Option<String>,
}

/// Update compliance template request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateComplianceTemplate {
    pub name: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub checklist_items: Option<serde_json::Value>,
    pub frequency: Option<String>,
}

// ==================== Compliance Audit Trail ====================

/// Compliance audit trail entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ComplianceAuditTrail {
    pub id: Uuid,
    pub organization_id: Uuid,

    // Related entities
    pub requirement_id: Option<Uuid>,
    pub document_id: Option<Uuid>,
    pub notice_id: Option<Uuid>,

    // Action details
    pub action: String,
    pub action_by: Uuid,
    pub action_at: Option<DateTime<Utc>>,

    // Values
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub notes: Option<String>,
}

/// Create audit trail entry request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAuditTrailEntry {
    pub requirement_id: Option<Uuid>,
    pub document_id: Option<Uuid>,
    pub notice_id: Option<Uuid>,
    pub action: String,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub notes: Option<String>,
}

// ==================== Analytics ====================

/// Compliance statistics.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ComplianceStatistics {
    pub total_requirements: i64,
    pub compliant_count: i64,
    pub non_compliant_count: i64,
    pub pending_count: i64,
    pub overdue_count: i64,
    pub by_category: Vec<ComplianceCategoryCount>,
    pub upcoming_verifications: Vec<UpcomingVerification>,
}

/// Compliance category count for statistics.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ComplianceCategoryCount {
    pub category: String,
    pub count: i64,
}

/// Upcoming verification.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UpcomingVerification {
    pub id: Uuid,
    pub name: String,
    pub category: String,
    pub next_due_date: Option<NaiveDate>,
    pub days_until_due: Option<i32>,
    pub building_id: Option<Uuid>,
}

/// Notice delivery statistics.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NoticeStatistics {
    pub total_notices: i64,
    pub sent_count: i64,
    pub pending_count: i64,
    pub by_type: Vec<NoticeTypeCount>,
    pub acknowledgment_stats: NoticeAcknowledgmentStats,
}

/// Notice type count.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct NoticeTypeCount {
    pub notice_type: String,
    pub count: i64,
}

/// Notice acknowledgment statistics.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NoticeAcknowledgmentStats {
    pub total_requiring: i64,
    pub acknowledged: i64,
    pub pending: i64,
    pub overdue: i64,
}

/// Legal document with version count.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LegalDocumentSummary {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,
    pub document_type: String,
    pub title: String,
    pub effective_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
    pub is_confidential: Option<bool>,
    pub version_count: i64,
    pub created_at: Option<DateTime<Utc>>,
}

/// Compliance requirement with details.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ComplianceRequirementWithDetails {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub frequency: String,
    pub status: String,
    pub is_mandatory: Option<bool>,
    pub next_due_date: Option<NaiveDate>,
    pub last_verified_at: Option<DateTime<Utc>>,
    pub verification_count: i64,
}

/// Notice with recipient summary.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct NoticeWithRecipients {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,
    pub notice_type: String,
    pub subject: String,
    pub priority: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub requires_acknowledgment: Option<bool>,
    pub total_recipients: i64,
    pub delivered_count: i64,
    pub acknowledged_count: i64,
}

/// Apply template to building request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ApplyTemplate {
    pub template_id: Uuid,
    pub building_id: Option<Uuid>,
}
