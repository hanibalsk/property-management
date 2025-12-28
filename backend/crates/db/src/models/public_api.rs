//! Public API and Developer Ecosystem models (Epic 69).
//!
//! This module provides types for API key management, webhooks, rate limiting,
//! and SDK generation for third-party integrations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

// ==================== API Key Status Constants ====================

/// API key status constants.
pub mod api_key_status {
    pub const ACTIVE: &str = "active";
    pub const REVOKED: &str = "revoked";
    pub const EXPIRED: &str = "expired";
    pub const SUSPENDED: &str = "suspended";
    pub const ALL: &[&str] = &[ACTIVE, REVOKED, EXPIRED, SUSPENDED];
}

/// API key scope constants.
pub mod api_key_scope {
    pub const READ: &str = "read";
    pub const WRITE: &str = "write";
    pub const ADMIN: &str = "admin";
    pub const BUILDINGS_READ: &str = "buildings:read";
    pub const BUILDINGS_WRITE: &str = "buildings:write";
    pub const FAULTS_READ: &str = "faults:read";
    pub const FAULTS_WRITE: &str = "faults:write";
    pub const FINANCIAL_READ: &str = "financial:read";
    pub const FINANCIAL_WRITE: &str = "financial:write";
    pub const RESIDENTS_READ: &str = "residents:read";
    pub const RESIDENTS_WRITE: &str = "residents:write";
    pub const WEBHOOKS_MANAGE: &str = "webhooks:manage";
    pub const ALL: &[&str] = &[
        READ,
        WRITE,
        ADMIN,
        BUILDINGS_READ,
        BUILDINGS_WRITE,
        FAULTS_READ,
        FAULTS_WRITE,
        FINANCIAL_READ,
        FINANCIAL_WRITE,
        RESIDENTS_READ,
        RESIDENTS_WRITE,
        WEBHOOKS_MANAGE,
    ];
}

/// Webhook event type constants.
pub mod webhook_event_type {
    pub const FAULT_CREATED: &str = "fault.created";
    pub const FAULT_UPDATED: &str = "fault.updated";
    pub const FAULT_RESOLVED: &str = "fault.resolved";
    pub const PAYMENT_RECEIVED: &str = "payment.received";
    pub const PAYMENT_OVERDUE: &str = "payment.overdue";
    pub const RESIDENT_MOVED_IN: &str = "resident.moved_in";
    pub const RESIDENT_MOVED_OUT: &str = "resident.moved_out";
    pub const VOTE_STARTED: &str = "vote.started";
    pub const VOTE_ENDED: &str = "vote.ended";
    pub const ANNOUNCEMENT_PUBLISHED: &str = "announcement.published";
    pub const DOCUMENT_UPLOADED: &str = "document.uploaded";
    pub const WORK_ORDER_CREATED: &str = "work_order.created";
    pub const WORK_ORDER_COMPLETED: &str = "work_order.completed";
    pub const ALL: &[&str] = &[
        FAULT_CREATED,
        FAULT_UPDATED,
        FAULT_RESOLVED,
        PAYMENT_RECEIVED,
        PAYMENT_OVERDUE,
        RESIDENT_MOVED_IN,
        RESIDENT_MOVED_OUT,
        VOTE_STARTED,
        VOTE_ENDED,
        ANNOUNCEMENT_PUBLISHED,
        DOCUMENT_UPLOADED,
        WORK_ORDER_CREATED,
        WORK_ORDER_COMPLETED,
    ];
}

/// Webhook delivery status constants.
pub mod webhook_delivery_status {
    pub const PENDING: &str = "pending";
    pub const DELIVERED: &str = "delivered";
    pub const FAILED: &str = "failed";
    pub const RETRYING: &str = "retrying";
    pub const EXHAUSTED: &str = "exhausted";
    pub const ALL: &[&str] = &[PENDING, DELIVERED, FAILED, RETRYING, EXHAUSTED];
}

/// Rate limit tier constants.
pub mod rate_limit_tier {
    pub const FREE: &str = "free";
    pub const BASIC: &str = "basic";
    pub const PROFESSIONAL: &str = "professional";
    pub const ENTERPRISE: &str = "enterprise";
    pub const ALL: &[&str] = &[FREE, BASIC, PROFESSIONAL, ENTERPRISE];
}

/// SDK language constants.
pub mod sdk_language {
    pub const TYPESCRIPT: &str = "typescript";
    pub const PYTHON: &str = "python";
    pub const GO: &str = "go";
    pub const JAVA: &str = "java";
    pub const CSHARP: &str = "csharp";
    pub const ALL: &[&str] = &[TYPESCRIPT, PYTHON, GO, JAVA, CSHARP];
}

// ==================== Developer Account (Story 69.1) ====================

/// Developer account entity for API access.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct DeveloperAccount {
    pub id: Uuid,
    pub user_id: Uuid,
    pub organization_id: Option<Uuid>,

    // Account information
    pub company_name: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,

    // Contact info
    pub contact_email: String,
    pub contact_name: Option<String>,

    // Rate limit tier
    pub tier: String,

    // Status
    pub is_verified: Option<bool>,
    pub is_active: Option<bool>,

    // Metadata
    pub metadata: Option<serde_json::Value>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub verified_at: Option<DateTime<Utc>>,
}

/// Create developer account request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateDeveloperAccount {
    pub company_name: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub contact_email: String,
    pub contact_name: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Update developer account request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateDeveloperAccount {
    pub company_name: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub contact_email: Option<String>,
    pub contact_name: Option<String>,
    pub tier: Option<String>,
    pub is_active: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}

// ==================== API Key (Story 69.1) ====================

/// API key entity for authentication.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ApiKey {
    pub id: Uuid,
    pub developer_account_id: Uuid,
    pub organization_id: Option<Uuid>,

    // Key information
    pub name: String,
    pub key_prefix: String,
    pub key_hash: String,

    // Permissions
    pub scopes: Vec<String>,

    // Rate limiting
    pub rate_limit_per_minute: Option<i32>,
    pub rate_limit_per_hour: Option<i32>,
    pub rate_limit_per_day: Option<i32>,

    // Usage tracking
    pub last_used_at: Option<DateTime<Utc>>,
    pub total_requests: Option<i64>,

    // Status
    pub status: String,
    pub expires_at: Option<DateTime<Utc>>,

    // Metadata
    pub allowed_ips: Option<Vec<String>>,
    pub allowed_origins: Option<Vec<String>>,
    pub metadata: Option<serde_json::Value>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
}

/// API key with masked secret (for display).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiKeyDisplay {
    pub id: Uuid,
    pub developer_account_id: Uuid,
    pub name: String,
    pub key_prefix: String,
    pub scopes: Vec<String>,
    pub rate_limit_per_minute: Option<i32>,
    pub rate_limit_per_hour: Option<i32>,
    pub rate_limit_per_day: Option<i32>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub total_requests: Option<i64>,
    pub status: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

/// Create API key request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateApiKey {
    pub name: String,
    pub scopes: Vec<String>,
    pub rate_limit_per_minute: Option<i32>,
    pub rate_limit_per_hour: Option<i32>,
    pub rate_limit_per_day: Option<i32>,
    pub expires_at: Option<DateTime<Utc>>,
    pub allowed_ips: Option<Vec<String>>,
    pub allowed_origins: Option<Vec<String>>,
    pub metadata: Option<serde_json::Value>,
}

/// Create API key response (includes secret only once).
#[derive(Debug, Serialize, ToSchema)]
pub struct CreateApiKeyResponse {
    pub id: Uuid,
    pub name: String,
    pub key_prefix: String,
    pub secret: String,
    pub scopes: Vec<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Update API key request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateApiKey {
    pub name: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub rate_limit_per_minute: Option<i32>,
    pub rate_limit_per_hour: Option<i32>,
    pub rate_limit_per_day: Option<i32>,
    pub expires_at: Option<DateTime<Utc>>,
    pub allowed_ips: Option<Vec<String>>,
    pub allowed_origins: Option<Vec<String>>,
    pub metadata: Option<serde_json::Value>,
}

/// API key query parameters.
#[derive(Debug, Default, Deserialize, ToSchema, IntoParams)]
pub struct ApiKeyQuery {
    pub status: Option<String>,
    pub scope: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// API key usage statistics.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ApiKeyUsageStats {
    pub api_key_id: Uuid,
    pub date: chrono::NaiveDate,
    pub total_requests: i64,
    pub successful_requests: i64,
    pub failed_requests: i64,
    pub rate_limited_requests: i64,
    pub avg_response_time_ms: Option<f64>,
}

// ==================== Webhook Subscription (Story 69.3) ====================

/// Webhook subscription entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct WebhookSubscription {
    pub id: Uuid,
    pub developer_account_id: Uuid,
    pub organization_id: Option<Uuid>,

    // Webhook configuration
    pub name: String,
    pub endpoint_url: String,
    pub secret: String,

    // Events
    pub event_types: Vec<String>,

    // Delivery settings
    pub is_active: Option<bool>,
    pub retry_count: Option<i32>,
    pub timeout_seconds: Option<i32>,

    // Headers
    pub custom_headers: Option<serde_json::Value>,

    // Statistics
    pub last_triggered_at: Option<DateTime<Utc>>,
    pub last_success_at: Option<DateTime<Utc>>,
    pub last_failure_at: Option<DateTime<Utc>>,
    pub total_deliveries: Option<i64>,
    pub successful_deliveries: Option<i64>,
    pub failed_deliveries: Option<i64>,

    // Metadata
    pub metadata: Option<serde_json::Value>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create webhook subscription request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateWebhookSubscription {
    pub name: String,
    pub endpoint_url: String,
    pub event_types: Vec<String>,
    pub retry_count: Option<i32>,
    pub timeout_seconds: Option<i32>,
    pub custom_headers: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

/// Create webhook subscription response (includes secret only once).
#[derive(Debug, Serialize, ToSchema)]
pub struct CreateWebhookResponse {
    pub id: Uuid,
    pub name: String,
    pub endpoint_url: String,
    pub secret: String,
    pub event_types: Vec<String>,
    pub created_at: DateTime<Utc>,
}

/// Update webhook subscription request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateWebhookSubscription {
    pub name: Option<String>,
    pub endpoint_url: Option<String>,
    pub event_types: Option<Vec<String>>,
    pub is_active: Option<bool>,
    pub retry_count: Option<i32>,
    pub timeout_seconds: Option<i32>,
    pub custom_headers: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

/// Webhook subscription query parameters.
#[derive(Debug, Default, Deserialize, ToSchema, IntoParams)]
pub struct WebhookSubscriptionQuery {
    pub event_type: Option<String>,
    pub is_active: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Webhook delivery log entry.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct WebhookDelivery {
    pub id: Uuid,
    pub subscription_id: Uuid,

    // Event information
    pub event_type: String,
    pub event_id: Uuid,
    pub payload: serde_json::Value,

    // Delivery attempt
    pub attempt_number: i32,
    pub status: String,

    // Response details
    pub response_status_code: Option<i32>,
    pub response_body: Option<String>,
    pub response_time_ms: Option<i32>,

    // Error information
    pub error_message: Option<String>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub next_retry_at: Option<DateTime<Utc>>,
}

/// Test webhook request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct TestWebhookRequest {
    pub event_type: String,
    pub payload: Option<serde_json::Value>,
}

/// Test webhook response.
#[derive(Debug, Serialize, ToSchema)]
pub struct TestWebhookResponse {
    pub success: bool,
    pub response_status_code: Option<i32>,
    pub response_body: Option<String>,
    pub response_time_ms: Option<i32>,
    pub error_message: Option<String>,
}

/// Webhook delivery query parameters.
#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct WebhookDeliveryQuery {
    pub subscription_id: Option<Uuid>,
    pub event_type: Option<String>,
    pub status: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

// ==================== Rate Limiting (Story 69.4) ====================

/// Rate limit configuration entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct RateLimitConfig {
    pub id: Uuid,
    pub tier: String,

    // Limits
    pub requests_per_minute: i32,
    pub requests_per_hour: i32,
    pub requests_per_day: i32,
    pub burst_limit: Option<i32>,

    // Endpoint-specific limits
    pub endpoint_limits: Option<serde_json::Value>,

    // Metadata
    pub description: Option<String>,
    pub is_active: Option<bool>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create rate limit config request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRateLimitConfig {
    pub tier: String,
    pub requests_per_minute: i32,
    pub requests_per_hour: i32,
    pub requests_per_day: i32,
    pub burst_limit: Option<i32>,
    pub endpoint_limits: Option<serde_json::Value>,
    pub description: Option<String>,
}

/// Update rate limit config request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateRateLimitConfig {
    pub requests_per_minute: Option<i32>,
    pub requests_per_hour: Option<i32>,
    pub requests_per_day: Option<i32>,
    pub burst_limit: Option<i32>,
    pub endpoint_limits: Option<serde_json::Value>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

/// Rate limit status response.
#[derive(Debug, Serialize, ToSchema)]
pub struct RateLimitStatus {
    pub tier: String,
    pub requests_per_minute: RateLimitWindow,
    pub requests_per_hour: RateLimitWindow,
    pub requests_per_day: RateLimitWindow,
}

/// Rate limit window information.
#[derive(Debug, Serialize, ToSchema)]
pub struct RateLimitWindow {
    pub limit: i32,
    pub remaining: i32,
    pub reset_at: DateTime<Utc>,
}

/// Rate limit headers for API responses.
#[derive(Debug, Serialize, ToSchema)]
pub struct RateLimitHeaders {
    pub x_ratelimit_limit: i32,
    pub x_ratelimit_remaining: i32,
    pub x_ratelimit_reset: i64,
    pub retry_after: Option<i32>,
}

// ==================== API Documentation (Story 69.2) ====================

/// API endpoint documentation.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ApiEndpointDoc {
    pub id: Uuid,

    // Endpoint info
    pub path: String,
    pub method: String,
    pub summary: String,
    pub description: Option<String>,

    // Grouping
    pub tag: String,
    pub category: Option<String>,

    // Request/Response
    pub request_body: Option<serde_json::Value>,
    pub response_body: Option<serde_json::Value>,
    pub parameters: Option<serde_json::Value>,

    // Authentication
    pub requires_auth: Option<bool>,
    pub required_scopes: Option<Vec<String>>,

    // Examples
    pub examples: Option<serde_json::Value>,

    // Rate limiting
    pub rate_limit_tier: Option<String>,

    // Metadata
    pub is_deprecated: Option<bool>,
    pub deprecated_message: Option<String>,
    pub version: Option<String>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// API changelog entry.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ApiChangelog {
    pub id: Uuid,
    pub version: String,
    pub release_date: chrono::NaiveDate,
    pub title: String,
    pub description: Option<String>,
    pub changes: serde_json::Value,
    pub breaking_changes: Option<serde_json::Value>,
    pub migration_guide: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

// ==================== SDK Generation (Story 69.5) ====================

/// SDK version entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SdkVersion {
    pub id: Uuid,
    pub language: String,
    pub version: String,
    pub api_version: String,

    // Download info
    pub download_url: Option<String>,
    pub package_name: Option<String>,
    pub package_manager_url: Option<String>,

    // Build info
    pub build_status: String,
    pub build_log: Option<String>,

    // Checksums
    pub checksum_sha256: Option<String>,

    // Statistics
    pub download_count: Option<i64>,

    // Metadata
    pub release_notes: Option<String>,
    pub is_latest: Option<bool>,
    pub is_stable: Option<bool>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
}

/// SDK download response.
#[derive(Debug, Serialize, ToSchema)]
pub struct SdkDownloadInfo {
    pub language: String,
    pub version: String,
    pub api_version: String,
    pub download_url: String,
    pub package_name: Option<String>,
    pub package_manager_url: Option<String>,
    pub checksum_sha256: Option<String>,
    pub release_notes: Option<String>,
}

/// SDK language info.
#[derive(Debug, Serialize, ToSchema)]
pub struct SdkLanguageInfo {
    pub language: String,
    pub display_name: String,
    pub package_manager: String,
    pub latest_version: Option<String>,
    pub installation_command: String,
    pub documentation_url: Option<String>,
}

// ==================== Developer Portal Analytics ====================

/// Developer portal statistics.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeveloperPortalStats {
    pub total_developers: i64,
    pub active_api_keys: i64,
    pub total_api_requests_today: i64,
    pub total_api_requests_month: i64,
    pub webhook_deliveries_today: i64,
    pub successful_webhook_rate: f64,
    pub top_endpoints: Vec<EndpointUsage>,
    pub requests_by_tier: Vec<TierUsage>,
}

/// Endpoint usage statistics.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EndpointUsage {
    pub endpoint: String,
    pub method: String,
    pub request_count: i64,
    pub avg_response_time_ms: Option<f64>,
    pub error_rate: Option<f64>,
}

/// Usage by rate limit tier.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TierUsage {
    pub tier: String,
    pub developer_count: i64,
    pub request_count: i64,
    pub percentage: f64,
}

/// Developer usage summary.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeveloperUsageSummary {
    pub developer_account_id: Uuid,
    pub company_name: Option<String>,
    pub tier: String,
    pub api_keys_count: i64,
    pub webhooks_count: i64,
    pub total_requests_today: i64,
    pub total_requests_month: i64,
    pub rate_limit_hits: i64,
    pub last_api_call: Option<DateTime<Utc>>,
}

// ==================== Sandbox Environment (Story 69.2) ====================

/// Sandbox environment entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SandboxEnvironment {
    pub id: Uuid,
    pub developer_account_id: Uuid,

    // Environment info
    pub name: String,
    pub description: Option<String>,

    // Configuration
    pub mock_data_enabled: Option<bool>,
    pub rate_limits_enabled: Option<bool>,

    // Status
    pub is_active: Option<bool>,
    pub expires_at: Option<DateTime<Utc>>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Sandbox test request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct SandboxTestRequest {
    pub endpoint: String,
    pub method: String,
    pub headers: Option<serde_json::Value>,
    pub body: Option<serde_json::Value>,
}

/// Sandbox test response.
#[derive(Debug, Serialize, ToSchema)]
pub struct SandboxTestResponse {
    pub status_code: i32,
    pub headers: serde_json::Value,
    pub body: serde_json::Value,
    pub response_time_ms: i32,
}

// ==================== API Request Log ====================

/// API request log entry.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ApiRequestLog {
    pub id: Uuid,
    pub api_key_id: Option<Uuid>,
    pub developer_account_id: Option<Uuid>,

    // Request info
    pub method: String,
    pub path: String,
    pub query_params: Option<String>,
    pub request_body_size: Option<i64>,

    // Response info
    pub status_code: i32,
    pub response_body_size: Option<i64>,
    pub response_time_ms: i32,

    // Client info
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,

    // Rate limiting
    pub was_rate_limited: Option<bool>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
}

/// API request log query parameters.
#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct ApiRequestLogQuery {
    pub api_key_id: Option<Uuid>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub status_code: Option<i32>,
    pub was_rate_limited: Option<bool>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

// ==================== Helper Responses ====================

/// Paginated response wrapper.
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub limit: i32,
    pub offset: i32,
    pub has_more: bool,
}

/// API key rotation response.
#[derive(Debug, Serialize, ToSchema)]
pub struct RotateApiKeyResponse {
    pub old_key_id: Uuid,
    pub new_key: CreateApiKeyResponse,
    pub old_key_expires_at: DateTime<Utc>,
}

/// Webhook secret rotation response.
#[derive(Debug, Serialize, ToSchema)]
pub struct RotateWebhookSecretResponse {
    pub webhook_id: Uuid,
    pub new_secret: String,
    pub rotated_at: DateTime<Utc>,
}
