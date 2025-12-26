//! External Integrations models (Epic 61).
//!
//! Models for calendar sync, accounting exports, e-signatures, video conferencing, and webhooks.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================
// Story 61.1: Calendar Integration
// ============================================

/// Calendar provider type.
pub mod calendar_provider {
    pub const GOOGLE: &str = "google";
    pub const OUTLOOK: &str = "outlook";
    pub const APPLE: &str = "apple";
    pub const CALDAV: &str = "caldav";
}

/// Calendar sync status.
pub mod calendar_sync_status {
    pub const ACTIVE: &str = "active";
    pub const PAUSED: &str = "paused";
    pub const ERROR: &str = "error";
    pub const DISCONNECTED: &str = "disconnected";
}

/// Calendar connection entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct CalendarConnection {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub provider_account_id: Option<String>,
    pub calendar_id: Option<String>,
    pub calendar_name: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<DateTime<Utc>>,
    pub sync_status: String,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub sync_direction: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create calendar connection request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCalendarConnection {
    pub provider: String,
    pub auth_code: Option<String>,
    pub calendar_id: Option<String>,
    pub sync_direction: Option<String>,
}

/// Update calendar connection request.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct UpdateCalendarConnection {
    pub calendar_id: Option<String>,
    pub sync_direction: Option<String>,
    pub sync_status: Option<String>,
}

/// Calendar event entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct CalendarEvent {
    pub id: Uuid,
    pub connection_id: Uuid,
    pub external_event_id: Option<String>,
    pub source_type: String,
    pub source_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub all_day: bool,
    pub recurrence_rule: Option<String>,
    pub attendees: Option<serde_json::Value>,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create calendar event request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCalendarEvent {
    pub connection_id: Uuid,
    pub source_type: String,
    pub source_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub all_day: Option<bool>,
    pub recurrence_rule: Option<String>,
    pub attendees: Option<serde_json::Value>,
}

/// Sync calendar request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncCalendarRequest {
    pub full_sync: Option<bool>,
    pub date_range_start: Option<DateTime<Utc>>,
    pub date_range_end: Option<DateTime<Utc>>,
}

/// Calendar sync result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CalendarSyncResult {
    pub events_created: i32,
    pub events_updated: i32,
    pub events_deleted: i32,
    pub errors: Vec<String>,
    pub synced_at: DateTime<Utc>,
}

// ============================================
// Story 61.2: Accounting System Export
// ============================================

/// Accounting system type.
pub mod accounting_system {
    pub const POHODA: &str = "pohoda";
    pub const MONEY_S3: &str = "money_s3";
    pub const QUICKBOOKS: &str = "quickbooks";
    pub const XERO: &str = "xero";
}

/// Export status.
pub mod export_status {
    pub const PENDING: &str = "pending";
    pub const PROCESSING: &str = "processing";
    pub const COMPLETED: &str = "completed";
    pub const FAILED: &str = "failed";
}

/// Accounting export entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct AccountingExport {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub system_type: String,
    pub export_type: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub status: String,
    pub file_path: Option<String>,
    pub file_size: Option<i64>,
    pub record_count: Option<i32>,
    pub error_message: Option<String>,
    pub exported_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Create accounting export request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateAccountingExport {
    pub system_type: String,
    pub export_type: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub include_attachments: Option<bool>,
    pub cost_center_mapping: Option<serde_json::Value>,
}

/// Accounting export settings.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct AccountingExportSettings {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub system_type: String,
    pub default_cost_center: Option<String>,
    pub account_mappings: Option<serde_json::Value>,
    pub vat_settings: Option<serde_json::Value>,
    pub auto_export_enabled: bool,
    pub auto_export_schedule: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Update accounting export settings request.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct UpdateAccountingExportSettings {
    pub default_cost_center: Option<String>,
    pub account_mappings: Option<serde_json::Value>,
    pub vat_settings: Option<serde_json::Value>,
    pub auto_export_enabled: Option<bool>,
    pub auto_export_schedule: Option<String>,
}

/// POHODA XML export data.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PohodaExportData {
    pub invoices: Vec<PohodaInvoice>,
    pub payments: Vec<PohodaPayment>,
}

/// POHODA invoice structure.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PohodaInvoice {
    pub number: String,
    pub date: NaiveDate,
    pub due_date: NaiveDate,
    pub partner_name: String,
    pub partner_ico: Option<String>,
    pub partner_dic: Option<String>,
    pub items: Vec<PohodaInvoiceItem>,
    pub total_without_vat: f64,
    pub total_vat: f64,
    pub total_with_vat: f64,
}

/// POHODA invoice item.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PohodaInvoiceItem {
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub vat_rate: f64,
    pub total: f64,
}

/// POHODA payment structure.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PohodaPayment {
    pub date: NaiveDate,
    pub amount: f64,
    pub invoice_number: Option<String>,
    pub payment_type: String,
}

// ============================================
// Story 61.3: E-Signature Integration
// ============================================

/// E-signature provider type.
pub mod esignature_provider {
    pub const DOCUSIGN: &str = "docusign";
    pub const ADOBE_SIGN: &str = "adobe_sign";
    pub const HELLOSIGN: &str = "hellosign";
    pub const INTERNAL: &str = "internal";
}

/// E-signature workflow status.
pub mod esignature_status {
    pub const DRAFT: &str = "draft";
    pub const SENT: &str = "sent";
    pub const VIEWED: &str = "viewed";
    pub const SIGNED: &str = "signed";
    pub const COMPLETED: &str = "completed";
    pub const DECLINED: &str = "declined";
    pub const VOIDED: &str = "voided";
    pub const EXPIRED: &str = "expired";
}

/// E-signature workflow entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ESignatureWorkflow {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub document_id: Uuid,
    pub provider: String,
    pub external_envelope_id: Option<String>,
    pub title: String,
    pub message: Option<String>,
    pub status: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub reminder_enabled: bool,
    pub reminder_days: Option<i32>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// E-signature recipient entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ESignatureRecipient {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
    pub signing_order: i32,
    pub status: String,
    pub signed_at: Option<DateTime<Utc>>,
    pub declined_at: Option<DateTime<Utc>>,
    pub decline_reason: Option<String>,
    pub reminder_sent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create e-signature workflow request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateESignatureWorkflow {
    pub document_id: Uuid,
    pub provider: Option<String>,
    pub title: String,
    pub message: Option<String>,
    pub recipients: Vec<CreateESignatureRecipient>,
    pub expires_in_days: Option<i32>,
    pub reminder_enabled: Option<bool>,
    pub reminder_days: Option<i32>,
}

/// Create e-signature recipient.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateESignatureRecipient {
    pub email: String,
    pub name: String,
    pub role: String,
    pub signing_order: Option<i32>,
}

/// E-signature workflow with recipients.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ESignatureWorkflowWithRecipients {
    #[serde(flatten)]
    pub workflow: ESignatureWorkflow,
    pub recipients: Vec<ESignatureRecipient>,
}

/// E-signature event (webhook).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ESignatureEvent {
    pub event_type: String,
    pub envelope_id: String,
    pub recipient_email: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub data: Option<serde_json::Value>,
}

// ============================================
// Story 61.4: Video Conferencing
// ============================================

/// Video conferencing provider type.
pub mod video_provider {
    pub const ZOOM: &str = "zoom";
    pub const TEAMS: &str = "teams";
    pub const GOOGLE_MEET: &str = "google_meet";
    pub const WEBEX: &str = "webex";
}

/// Meeting status.
pub mod meeting_status {
    pub const SCHEDULED: &str = "scheduled";
    pub const STARTED: &str = "started";
    pub const ENDED: &str = "ended";
    pub const CANCELLED: &str = "cancelled";
}

/// Video conference connection entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct VideoConferenceConnection {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub provider_user_id: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create video conference connection request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateVideoConferenceConnection {
    pub provider: String,
    pub auth_code: String,
}

/// Video meeting entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct VideoMeeting {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub connection_id: Uuid,
    pub external_meeting_id: Option<String>,
    pub source_type: String,
    pub source_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub duration_minutes: i32,
    pub timezone: Option<String>,
    pub join_url: Option<String>,
    pub host_url: Option<String>,
    pub password: Option<String>,
    pub status: String,
    pub recording_url: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create video meeting request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateVideoMeeting {
    pub connection_id: Option<Uuid>,
    pub provider: Option<String>,
    pub source_type: String,
    pub source_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub duration_minutes: i32,
    pub timezone: Option<String>,
    pub participants: Option<Vec<MeetingParticipant>>,
    pub settings: Option<MeetingSettings>,
}

/// Meeting participant.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MeetingParticipant {
    pub email: String,
    pub name: Option<String>,
    pub is_host: Option<bool>,
}

/// Meeting settings.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MeetingSettings {
    pub waiting_room: Option<bool>,
    pub mute_on_entry: Option<bool>,
    pub auto_recording: Option<bool>,
    pub allow_screen_share: Option<bool>,
}

/// Update video meeting request.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct UpdateVideoMeeting {
    pub title: Option<String>,
    pub description: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i32>,
    pub status: Option<String>,
}

// ============================================
// Story 61.5: Webhook Notifications
// ============================================

/// Webhook status.
pub mod webhook_status {
    pub const ACTIVE: &str = "active";
    pub const PAUSED: &str = "paused";
    pub const DISABLED: &str = "disabled";
}

/// Delivery status.
pub mod delivery_status {
    pub const PENDING: &str = "pending";
    pub const DELIVERED: &str = "delivered";
    pub const FAILED: &str = "failed";
    pub const RETRYING: &str = "retrying";
}

/// Webhook subscription entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct WebhookSubscription {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub secret: Option<String>,
    pub events: Vec<String>,
    pub status: String,
    pub headers: Option<serde_json::Value>,
    pub retry_policy: Option<serde_json::Value>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create webhook subscription request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateWebhookSubscription {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub events: Vec<String>,
    pub secret: Option<String>,
    pub headers: Option<serde_json::Value>,
    pub retry_policy: Option<WebhookRetryPolicy>,
}

/// Update webhook subscription request.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct UpdateWebhookSubscription {
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub events: Option<Vec<String>>,
    pub secret: Option<String>,
    pub headers: Option<serde_json::Value>,
    pub status: Option<String>,
    pub retry_policy: Option<WebhookRetryPolicy>,
}

/// Webhook retry policy.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WebhookRetryPolicy {
    pub max_retries: i32,
    pub retry_interval_seconds: i32,
    pub exponential_backoff: bool,
}

/// Webhook delivery log entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct WebhookDeliveryLog {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub event_type: String,
    pub event_id: Uuid,
    pub payload: serde_json::Value,
    pub status: String,
    pub attempts: i32,
    pub last_attempt_at: Option<DateTime<Utc>>,
    pub next_retry_at: Option<DateTime<Utc>>,
    pub response_status: Option<i32>,
    pub response_body: Option<String>,
    pub error_message: Option<String>,
    pub duration_ms: Option<i32>,
    pub created_at: DateTime<Utc>,
}

/// Webhook delivery log query.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WebhookDeliveryQuery {
    pub subscription_id: Option<Uuid>,
    pub event_type: Option<String>,
    pub status: Option<String>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Webhook event types.
pub mod webhook_event {
    // Fault events
    pub const FAULT_CREATED: &str = "fault.created";
    pub const FAULT_UPDATED: &str = "fault.updated";
    pub const FAULT_RESOLVED: &str = "fault.resolved";

    // Document events
    pub const DOCUMENT_UPLOADED: &str = "document.uploaded";
    pub const DOCUMENT_SIGNED: &str = "document.signed";

    // Payment events
    pub const PAYMENT_RECEIVED: &str = "payment.received";
    pub const PAYMENT_OVERDUE: &str = "payment.overdue";

    // Meeting events
    pub const MEETING_SCHEDULED: &str = "meeting.scheduled";
    pub const MEETING_STARTED: &str = "meeting.started";
    pub const MEETING_ENDED: &str = "meeting.ended";

    // Announcement events
    pub const ANNOUNCEMENT_PUBLISHED: &str = "announcement.published";

    // Vote events
    pub const VOTE_STARTED: &str = "vote.started";
    pub const VOTE_ENDED: &str = "vote.ended";

    // Visitor events
    pub const VISITOR_CHECKED_IN: &str = "visitor.checked_in";
    pub const VISITOR_CHECKED_OUT: &str = "visitor.checked_out";

    // Package events
    pub const PACKAGE_RECEIVED: &str = "package.received";
    pub const PACKAGE_PICKED_UP: &str = "package.picked_up";
}

/// Webhook statistics.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WebhookStatistics {
    pub total_deliveries: i64,
    pub successful_deliveries: i64,
    pub failed_deliveries: i64,
    pub pending_deliveries: i64,
    pub average_response_time_ms: Option<f64>,
    pub success_rate: f64,
}

/// Test webhook request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TestWebhookRequest {
    pub event_type: String,
    pub payload: Option<serde_json::Value>,
}

/// Test webhook response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TestWebhookResponse {
    pub success: bool,
    pub status_code: Option<i32>,
    pub response_time_ms: Option<i32>,
    pub error: Option<String>,
}

// ============================================
// Integration Statistics
// ============================================

/// Integration dashboard statistics.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct IntegrationStatistics {
    pub calendar_connections: i32,
    pub active_calendar_syncs: i32,
    pub accounting_exports_this_month: i32,
    pub esignature_workflows_pending: i32,
    pub esignature_workflows_completed: i32,
    pub video_meetings_scheduled: i32,
    pub webhook_subscriptions: i32,
    pub webhook_deliveries_today: i64,
    pub webhook_success_rate: f64,
}
