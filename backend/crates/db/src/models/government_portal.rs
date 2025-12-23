//! Government Portal Integration models (Epic 30).
//!
//! UC-22.3: Government Portal Integration

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// Enums
// ============================================================================

/// Type of government portal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "government_portal_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum GovernmentPortalType {
    TaxAuthority,
    StatisticalOffice,
    BuildingAuthority,
    HousingRegistry,
    PoliceRegistry,
    EnergyAuthority,
    EnvironmentalAgency,
    LaborOffice,
    SocialInsurance,
    BusinessRegistry,
    DataProtection,
    Other,
}

/// Status of a regulatory submission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "submission_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SubmissionStatus {
    Draft,
    PendingValidation,
    Validated,
    Submitted,
    Acknowledged,
    Processing,
    Accepted,
    Rejected,
    RequiresCorrection,
    Cancelled,
}

// ============================================================================
// Government Portal Connection
// ============================================================================

/// Government portal connection.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GovernmentPortalConnection {
    pub id: Uuid,
    pub organization_id: Uuid,

    pub portal_type: GovernmentPortalType,
    pub portal_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_code: Option<String>,
    pub country_code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<Uuid>,

    pub is_active: bool,
    pub auto_submit: bool,
    pub test_mode: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_connection_test: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_submission: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to create a government portal connection.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatePortalConnection {
    pub portal_type: GovernmentPortalType,
    pub portal_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_code: Option<String>,
    #[serde(default = "default_country_code")]
    pub country_code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_client_id: Option<String>,

    #[serde(default)]
    pub auto_submit: bool,
    #[serde(default = "default_true")]
    pub test_mode: bool,
}

fn default_country_code() -> String {
    "SK".to_string()
}

fn default_true() -> bool {
    true
}

/// Request to update a portal connection.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePortalConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_submit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_mode: Option<bool>,
}

// ============================================================================
// Regulatory Report Templates
// ============================================================================

/// Regulatory report template.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegulatoryReportTemplate {
    pub id: Uuid,
    pub template_code: String,
    pub template_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub portal_type: GovernmentPortalType,
    pub country_code: String,

    pub schema_version: String,
    pub field_mappings: serde_json::Value,
    pub validation_rules: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_day_of_month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_month_of_quarter: Option<i32>,

    pub is_active: bool,
    pub effective_from: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_to: Option<NaiveDate>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Template summary.
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TemplateSummaryGov {
    pub id: Uuid,
    pub template_code: String,
    pub template_name: String,
    pub portal_type: GovernmentPortalType,
    pub country_code: String,
    pub frequency: Option<String>,
}

// ============================================================================
// Regulatory Submissions
// ============================================================================

/// Regulatory submission.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegulatorySubmission {
    pub id: Uuid,
    pub organization_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_connection_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<Uuid>,

    pub submission_reference: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<String>,

    pub report_type: String,
    pub report_period_start: NaiveDate,
    pub report_period_end: NaiveDate,

    pub report_data: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_xml: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_pdf_url: Option<String>,

    pub status: SubmissionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_response: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub validated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_by: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<Uuid>,

    pub submission_attempts: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_retry_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to create a regulatory submission.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateRegulatorySubmission {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_connection_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<Uuid>,
    pub report_type: String,
    pub report_period_start: NaiveDate,
    pub report_period_end: NaiveDate,
    pub report_data: serde_json::Value,
}

/// Request to update a regulatory submission.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRegulatorySubmission {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_xml: Option<String>,
}

/// Submission summary for lists.
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SubmissionSummary {
    pub id: Uuid,
    pub submission_reference: String,
    pub report_type: String,
    pub report_period_start: NaiveDate,
    pub report_period_end: NaiveDate,
    pub status: SubmissionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Query parameters for listing submissions.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SubmissionQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubmissionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<NaiveDate>,
    #[serde(default = "default_limit")]
    pub limit: i32,
    #[serde(default)]
    pub offset: i32,
}

fn default_limit() -> i32 {
    50
}

// ============================================================================
// Submission Audit
// ============================================================================

/// Submission audit entry.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegulatorySubmissionAudit {
    pub id: Uuid,
    pub submission_id: Uuid,
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_status: Option<SubmissionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_status: Option<SubmissionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<Uuid>,
    pub actor_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Request to create audit entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubmissionAudit {
    pub submission_id: Uuid,
    pub action: String,
    pub previous_status: Option<SubmissionStatus>,
    pub new_status: Option<SubmissionStatus>,
    pub actor_id: Option<Uuid>,
    pub actor_type: String,
    pub details: Option<serde_json::Value>,
    pub error_message: Option<String>,
}

// ============================================================================
// Submission Attachments
// ============================================================================

/// Submission attachment.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegulatorySubmissionAttachment {
    pub id: Uuid,
    pub submission_id: Uuid,
    pub file_name: String,
    pub file_type: String,
    pub file_size: i32,
    pub file_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    pub attachment_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_document_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Request to add attachment.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddSubmissionAttachment {
    pub file_name: String,
    pub file_type: String,
    pub file_size: i32,
    pub file_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    pub attachment_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// ============================================================================
// Submission Schedules
// ============================================================================

/// Scheduled recurring submission.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegulatorySubmissionSchedule {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub portal_connection_id: Uuid,
    pub template_id: Uuid,

    pub is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_due_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_generated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_submission_id: Option<Uuid>,

    pub auto_generate: bool,
    pub auto_submit: bool,
    pub notify_before_days: i32,

    pub notify_users: serde_json::Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to create submission schedule.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubmissionSchedule {
    pub portal_connection_id: Uuid,
    pub template_id: Uuid,
    #[serde(default = "default_true")]
    pub auto_generate: bool,
    #[serde(default)]
    pub auto_submit: bool,
    #[serde(default = "default_notify_days")]
    pub notify_before_days: i32,
    #[serde(default)]
    pub notify_users: Vec<Uuid>,
}

fn default_notify_days() -> i32 {
    7
}

/// Request to update submission schedule.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSubmissionSchedule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_generate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_submit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_before_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_users: Option<Vec<Uuid>>,
}

// ============================================================================
// Statistics / Dashboard
// ============================================================================

/// Government portal statistics.
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GovernmentPortalStats {
    pub total_connections: i64,
    pub active_connections: i64,
    pub total_submissions: i64,
    pub submissions_this_month: i64,
    pub pending_submissions: i64,
    pub rejected_submissions: i64,
    pub upcoming_due_dates: Vec<UpcomingDueDate>,
}

/// Upcoming due date.
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpcomingDueDate {
    pub schedule_id: Uuid,
    pub template_name: String,
    pub portal_type: GovernmentPortalType,
    pub due_date: NaiveDate,
    pub days_until_due: i32,
}

// ============================================================================
// Validation Result
// ============================================================================

/// Validation result for a submission.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

/// Validation error.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ValidationError {
    pub field: String,
    pub code: String,
    pub message: String,
}

/// Validation warning.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ValidationWarning {
    pub field: String,
    pub code: String,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portal_type_serialization() {
        let portal_type = GovernmentPortalType::TaxAuthority;
        let json = serde_json::to_string(&portal_type).unwrap();
        assert_eq!(json, "\"tax_authority\"");
    }

    #[test]
    fn test_submission_status_serialization() {
        let status = SubmissionStatus::PendingValidation;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"pending_validation\"");
    }
}
