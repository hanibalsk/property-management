//! Cross-cutting infrastructure models (Epic 71).
//!
//! Types for distributed tracing, feature flags, background jobs, and health monitoring.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ==================== Story 71.1: Distributed Tracing ====================

/// A distributed trace representing a request flow across services.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Trace {
    /// Unique trace identifier
    pub id: Uuid,
    /// Trace ID from OpenTelemetry (128-bit hex string)
    pub trace_id: String,
    /// Root span ID
    pub root_span_id: String,
    /// Service that initiated the trace
    pub service_name: String,
    /// Operation name (e.g., "GET /api/v1/buildings")
    pub operation_name: String,
    /// HTTP method if applicable
    pub http_method: Option<String>,
    /// HTTP path if applicable
    pub http_path: Option<String>,
    /// HTTP status code if applicable
    pub http_status_code: Option<i32>,
    /// Total duration in milliseconds
    pub duration_ms: i64,
    /// Whether the trace has errors
    pub has_error: bool,
    /// User ID if authenticated
    pub user_id: Option<Uuid>,
    /// Organization ID if in context
    pub org_id: Option<Uuid>,
    /// Additional attributes as JSON
    pub attributes: Option<serde_json::Value>,
    /// When the trace started
    pub started_at: DateTime<Utc>,
    /// When the trace completed
    pub completed_at: Option<DateTime<Utc>>,
    /// When this record was created
    pub created_at: DateTime<Utc>,
}

/// A span within a distributed trace.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Span {
    /// Unique span identifier
    pub id: Uuid,
    /// Reference to parent trace
    pub trace_id: Uuid,
    /// Span ID from OpenTelemetry
    pub span_id: String,
    /// Parent span ID (None for root span)
    pub parent_span_id: Option<String>,
    /// Service that created this span
    pub service_name: String,
    /// Operation name
    pub operation_name: String,
    /// Span kind (client, server, producer, consumer, internal)
    pub span_kind: SpanKind,
    /// Duration in milliseconds
    pub duration_ms: i64,
    /// Span status
    pub status: SpanStatus,
    /// Error message if status is error
    pub error_message: Option<String>,
    /// Span attributes as JSON
    pub attributes: Option<serde_json::Value>,
    /// Span events as JSON array
    pub events: Option<serde_json::Value>,
    /// When the span started
    pub started_at: DateTime<Utc>,
    /// When the span ended
    pub ended_at: Option<DateTime<Utc>>,
    /// When this record was created
    pub created_at: DateTime<Utc>,
}

/// Span kind following OpenTelemetry conventions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "span_kind", rename_all = "snake_case")]
pub enum SpanKind {
    /// Server-side handling of a synchronous RPC
    Server,
    /// Client-side of a synchronous RPC
    Client,
    /// Producer of an async message
    Producer,
    /// Consumer of an async message
    Consumer,
    /// Internal operation
    Internal,
}

/// Span status following OpenTelemetry conventions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "span_status", rename_all = "snake_case")]
pub enum SpanStatus {
    /// Unset status
    Unset,
    /// Operation completed successfully
    Ok,
    /// Operation failed with an error
    Error,
}

/// Query parameters for searching traces.
#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct TraceQuery {
    /// Filter by service name
    pub service_name: Option<String>,
    /// Filter by operation name (partial match)
    pub operation_name: Option<String>,
    /// Filter by minimum duration (ms)
    pub min_duration_ms: Option<i64>,
    /// Filter by maximum duration (ms)
    pub max_duration_ms: Option<i64>,
    /// Filter by error status
    pub has_error: Option<bool>,
    /// Filter by user ID
    pub user_id: Option<Uuid>,
    /// Filter by organization ID
    pub org_id: Option<Uuid>,
    /// Filter by HTTP status code
    pub http_status_code: Option<i32>,
    /// Filter by start time (from)
    pub from_time: Option<DateTime<Utc>>,
    /// Filter by start time (to)
    pub to_time: Option<DateTime<Utc>>,
    /// Pagination limit
    pub limit: Option<i64>,
    /// Pagination offset
    pub offset: Option<i64>,
}

/// Trace with all its spans.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TraceWithSpans {
    /// The trace record
    pub trace: Trace,
    /// All spans in this trace
    pub spans: Vec<Span>,
}

// ==================== Story 71.2: Feature Flags ====================

/// A feature flag for controlling feature rollout.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct FeatureFlag {
    /// Unique flag identifier
    pub id: Uuid,
    /// Flag key (unique, lowercase with underscores)
    pub key: String,
    /// Human-readable name
    pub name: String,
    /// Description of what the flag controls
    pub description: Option<String>,
    /// Whether the flag is globally enabled
    pub enabled: bool,
    /// Rollout percentage (0-100)
    pub rollout_percentage: i32,
    /// Targeting rules as JSON
    pub targeting_rules: Option<serde_json::Value>,
    /// Default value when no rules match
    pub default_value: serde_json::Value,
    /// Flag type (boolean, string, number, json)
    pub value_type: FeatureFlagValueType,
    /// Environment (development, staging, production)
    pub environment: String,
    /// Tags for categorization
    pub tags: Option<Vec<String>>,
    /// User who created the flag
    pub created_by: Option<Uuid>,
    /// When the flag was created
    pub created_at: DateTime<Utc>,
    /// When the flag was last updated
    pub updated_at: DateTime<Utc>,
    /// When the flag was last evaluated
    pub last_evaluated_at: Option<DateTime<Utc>>,
}

/// Feature flag value type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "feature_flag_value_type", rename_all = "snake_case")]
pub enum FeatureFlagValueType {
    /// Boolean true/false
    Boolean,
    /// String value
    String,
    /// Numeric value
    Number,
    /// JSON object value
    Json,
}

/// Feature flag override for specific users or organizations.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct FeatureFlagOverride {
    /// Unique override identifier
    pub id: Uuid,
    /// Reference to the feature flag
    pub flag_id: Uuid,
    /// Override type (user, organization, percentage)
    pub override_type: FeatureFlagOverrideType,
    /// Target ID (user_id or org_id depending on type)
    pub target_id: Option<Uuid>,
    /// The override value
    pub value: serde_json::Value,
    /// Whether this override is active
    pub enabled: bool,
    /// When the override expires (optional)
    pub expires_at: Option<DateTime<Utc>>,
    /// When the override was created
    pub created_at: DateTime<Utc>,
}

/// Types of feature flag overrides.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "feature_flag_override_type", rename_all = "snake_case")]
pub enum FeatureFlagOverrideType {
    /// Override for a specific user
    User,
    /// Override for a specific organization
    Organization,
    /// Percentage-based rollout
    Percentage,
}

/// Feature flag audit log entry.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct FeatureFlagAuditLog {
    /// Unique log identifier
    pub id: Uuid,
    /// Reference to the feature flag
    pub flag_id: Uuid,
    /// Action performed
    pub action: FeatureFlagAuditAction,
    /// User who performed the action
    pub performed_by: Option<Uuid>,
    /// Previous state as JSON
    pub previous_state: Option<serde_json::Value>,
    /// New state as JSON
    pub new_state: Option<serde_json::Value>,
    /// Additional context
    pub context: Option<serde_json::Value>,
    /// When the action occurred
    pub created_at: DateTime<Utc>,
}

/// Feature flag audit actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "feature_flag_audit_action", rename_all = "snake_case")]
pub enum FeatureFlagAuditAction {
    /// Flag was created
    Created,
    /// Flag was updated
    Updated,
    /// Flag was enabled
    Enabled,
    /// Flag was disabled
    Disabled,
    /// Rollout percentage changed
    RolloutChanged,
    /// Targeting rules changed
    TargetingChanged,
    /// Override was added
    OverrideAdded,
    /// Override was removed
    OverrideRemoved,
    /// Flag was archived
    Archived,
    /// Flag was deleted
    Deleted,
}

/// Request to create a feature flag.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateFeatureFlag {
    /// Flag key (unique, lowercase with underscores)
    pub key: String,
    /// Human-readable name
    pub name: String,
    /// Description of what the flag controls
    pub description: Option<String>,
    /// Whether the flag is globally enabled
    pub enabled: bool,
    /// Rollout percentage (0-100)
    pub rollout_percentage: Option<i32>,
    /// Default value
    pub default_value: serde_json::Value,
    /// Flag type
    pub value_type: FeatureFlagValueType,
    /// Environment
    pub environment: String,
    /// Tags for categorization
    pub tags: Option<Vec<String>>,
}

/// Request to update a feature flag.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateFeatureFlag {
    /// Human-readable name
    pub name: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Whether the flag is globally enabled
    pub enabled: Option<bool>,
    /// Rollout percentage (0-100)
    pub rollout_percentage: Option<i32>,
    /// Targeting rules
    pub targeting_rules: Option<serde_json::Value>,
    /// Default value
    pub default_value: Option<serde_json::Value>,
    /// Tags for categorization
    pub tags: Option<Vec<String>>,
}

/// Request to evaluate a feature flag.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EvaluateFeatureFlag {
    /// Flag key
    pub key: String,
    /// User ID for targeting
    pub user_id: Option<Uuid>,
    /// Organization ID for targeting
    pub org_id: Option<Uuid>,
    /// Additional context for targeting rules
    pub context: Option<serde_json::Value>,
}

/// Result of feature flag evaluation.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FeatureFlagEvaluation {
    /// Flag key
    pub key: String,
    /// Evaluated value
    pub value: serde_json::Value,
    /// Whether the flag is enabled
    pub enabled: bool,
    /// Reason for the evaluation result
    pub reason: String,
    /// Rule that matched (if any)
    pub matched_rule: Option<String>,
}

// ==================== Story 71.3: Background Jobs ====================

/// A background job definition.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct BackgroundJob {
    /// Unique job identifier
    pub id: Uuid,
    /// Job type/name
    pub job_type: String,
    /// Job priority (higher = more urgent)
    pub priority: i32,
    /// Current job status
    pub status: BackgroundJobStatus,
    /// Job payload as JSON
    pub payload: serde_json::Value,
    /// Job result as JSON (when completed)
    pub result: Option<serde_json::Value>,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Error details/stack trace
    pub error_details: Option<serde_json::Value>,
    /// Number of attempts made
    pub attempts: i32,
    /// Maximum allowed attempts
    pub max_attempts: i32,
    /// When the job should run (for scheduled jobs)
    pub scheduled_at: DateTime<Utc>,
    /// When the job started executing
    pub started_at: Option<DateTime<Utc>>,
    /// When the job completed
    pub completed_at: Option<DateTime<Utc>>,
    /// Processing duration in milliseconds
    pub duration_ms: Option<i64>,
    /// Queue name for job routing
    pub queue: String,
    /// Worker ID that processed this job
    pub worker_id: Option<String>,
    /// Retry delay in seconds (for exponential backoff)
    pub retry_delay_seconds: Option<i32>,
    /// Organization context
    pub org_id: Option<Uuid>,
    /// User who created the job
    pub created_by: Option<Uuid>,
    /// When the job was created
    pub created_at: DateTime<Utc>,
    /// When the job was last updated
    pub updated_at: DateTime<Utc>,
}

/// Background job status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "background_job_status", rename_all = "snake_case")]
pub enum BackgroundJobStatus {
    /// Job is waiting to be processed
    Pending,
    /// Job is scheduled for a future time
    Scheduled,
    /// Job is currently being processed
    Running,
    /// Job completed successfully
    Completed,
    /// Job failed and will not be retried
    Failed,
    /// Job failed but will be retried
    Retrying,
    /// Job was cancelled
    Cancelled,
    /// Job timed out
    TimedOut,
}

impl std::str::FromStr for BackgroundJobStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(BackgroundJobStatus::Pending),
            "scheduled" => Ok(BackgroundJobStatus::Scheduled),
            "running" => Ok(BackgroundJobStatus::Running),
            "completed" => Ok(BackgroundJobStatus::Completed),
            "failed" => Ok(BackgroundJobStatus::Failed),
            "retrying" => Ok(BackgroundJobStatus::Retrying),
            "cancelled" => Ok(BackgroundJobStatus::Cancelled),
            "timed_out" | "timedout" => Ok(BackgroundJobStatus::TimedOut),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
}

/// Background job execution history entry.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct BackgroundJobExecution {
    /// Unique execution identifier
    pub id: Uuid,
    /// Reference to the job
    pub job_id: Uuid,
    /// Attempt number
    pub attempt: i32,
    /// Execution status
    pub status: BackgroundJobStatus,
    /// Worker ID that executed this attempt
    pub worker_id: String,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Error details/stack trace
    pub error_details: Option<serde_json::Value>,
    /// Execution duration in milliseconds
    pub duration_ms: Option<i64>,
    /// When execution started
    pub started_at: DateTime<Utc>,
    /// When execution ended
    pub ended_at: Option<DateTime<Utc>>,
}

/// Background job queue statistics.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BackgroundJobQueueStats {
    /// Queue name
    pub queue: String,
    /// Number of pending jobs
    pub pending_count: i64,
    /// Number of running jobs
    pub running_count: i64,
    /// Number of failed jobs in last 24h
    pub failed_count_24h: i64,
    /// Number of completed jobs in last 24h
    pub completed_count_24h: i64,
    /// Average processing time in ms (last 24h)
    pub avg_duration_ms: Option<f64>,
    /// 95th percentile processing time in ms
    pub p95_duration_ms: Option<f64>,
    /// Number of jobs retrying
    pub retrying_count: i64,
    /// Oldest pending job age in seconds
    pub oldest_pending_age_seconds: Option<i64>,
}

/// Background job type statistics.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BackgroundJobTypeStats {
    /// Job type
    pub job_type: String,
    /// Total jobs created
    pub total_count: i64,
    /// Success rate percentage
    pub success_rate: f64,
    /// Average duration in ms
    pub avg_duration_ms: Option<f64>,
    /// Jobs currently pending
    pub pending_count: i64,
    /// Jobs that failed
    pub failed_count: i64,
}

/// Request to create a background job.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateBackgroundJob {
    /// Job type/name
    pub job_type: String,
    /// Job priority (default: 0)
    pub priority: Option<i32>,
    /// Job payload
    pub payload: serde_json::Value,
    /// When to run (default: now)
    pub scheduled_at: Option<DateTime<Utc>>,
    /// Queue name (default: "default")
    pub queue: Option<String>,
    /// Maximum attempts (default: 3)
    pub max_attempts: Option<i32>,
    /// Organization context
    pub org_id: Option<Uuid>,
}

/// Request to retry a failed job.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RetryBackgroundJob {
    /// When to retry (default: now)
    pub scheduled_at: Option<DateTime<Utc>>,
    /// Reset attempt count
    pub reset_attempts: Option<bool>,
}

/// Query parameters for listing background jobs.
#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct BackgroundJobQuery {
    /// Filter by job type
    pub job_type: Option<String>,
    /// Filter by status
    pub status: Option<BackgroundJobStatus>,
    /// Filter by queue
    pub queue: Option<String>,
    /// Filter by organization
    pub org_id: Option<Uuid>,
    /// Filter by created time (from)
    pub from_time: Option<DateTime<Utc>>,
    /// Filter by created time (to)
    pub to_time: Option<DateTime<Utc>>,
    /// Pagination limit
    pub limit: Option<i64>,
    /// Pagination offset
    pub offset: Option<i64>,
}

// ==================== Story 71.4: Health Monitoring ====================

/// Health check result for a single dependency.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DependencyHealth {
    /// Dependency name (e.g., "database", "redis", "s3")
    pub name: String,
    /// Health status
    pub status: HealthStatus,
    /// Response time in milliseconds
    pub latency_ms: Option<i64>,
    /// Error message if unhealthy
    pub error: Option<String>,
    /// Additional details
    pub details: Option<serde_json::Value>,
    /// When this check was performed
    pub checked_at: DateTime<Utc>,
}

/// Health status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "health_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    /// Healthy and operational
    Healthy,
    /// Degraded but operational
    Degraded,
    /// Not healthy
    Unhealthy,
    /// Unknown status
    Unknown,
}

/// Overall system health response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SystemHealth {
    /// Overall system status
    pub status: HealthStatus,
    /// Service version
    pub version: String,
    /// Service name
    pub service: String,
    /// Uptime in seconds
    pub uptime_seconds: i64,
    /// Health of individual dependencies
    pub dependencies: Vec<DependencyHealth>,
    /// System metrics
    pub metrics: Option<SystemMetrics>,
    /// When this health check was performed
    pub checked_at: DateTime<Utc>,
}

/// System metrics for health monitoring.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SystemMetrics {
    /// CPU usage percentage (0-100)
    pub cpu_usage_percent: Option<f64>,
    /// Memory usage percentage (0-100)
    pub memory_usage_percent: Option<f64>,
    /// Memory used in bytes
    pub memory_used_bytes: Option<i64>,
    /// Memory total in bytes
    pub memory_total_bytes: Option<i64>,
    /// Disk usage percentage (0-100)
    pub disk_usage_percent: Option<f64>,
    /// Disk used in bytes
    pub disk_used_bytes: Option<i64>,
    /// Disk total in bytes
    pub disk_total_bytes: Option<i64>,
    /// Number of active connections (database, etc.)
    pub active_connections: Option<i32>,
    /// Number of open file descriptors
    pub open_file_descriptors: Option<i32>,
    /// Number of threads
    pub thread_count: Option<i32>,
}

/// Health check configuration.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct HealthCheckConfig {
    /// Unique configuration identifier
    pub id: Uuid,
    /// Check name
    pub name: String,
    /// Check type (http, tcp, database, redis, custom)
    pub check_type: HealthCheckType,
    /// Check endpoint or connection string
    pub endpoint: String,
    /// Check interval in seconds
    pub interval_seconds: i32,
    /// Timeout in milliseconds
    pub timeout_ms: i32,
    /// Number of failures before unhealthy
    pub failure_threshold: i32,
    /// Number of successes before healthy
    pub success_threshold: i32,
    /// Whether this check is enabled
    pub enabled: bool,
    /// Additional configuration as JSON
    pub config: Option<serde_json::Value>,
    /// When the configuration was created
    pub created_at: DateTime<Utc>,
    /// When the configuration was last updated
    pub updated_at: DateTime<Utc>,
}

/// Health check type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "health_check_type", rename_all = "snake_case")]
pub enum HealthCheckType {
    /// HTTP/HTTPS endpoint check
    Http,
    /// TCP port check
    Tcp,
    /// Database connection check
    Database,
    /// Redis connection check
    Redis,
    /// S3/object storage check
    S3,
    /// Custom check
    Custom,
}

/// Health check result history.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct HealthCheckResult {
    /// Unique result identifier
    pub id: Uuid,
    /// Reference to the health check config
    pub config_id: Uuid,
    /// Health status
    pub status: HealthStatus,
    /// Response time in milliseconds
    pub latency_ms: Option<i64>,
    /// Error message if unhealthy
    pub error_message: Option<String>,
    /// Response details
    pub response_details: Option<serde_json::Value>,
    /// When the check was performed
    pub checked_at: DateTime<Utc>,
}

/// Alert rule for health monitoring.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct HealthAlertRule {
    /// Unique rule identifier
    pub id: Uuid,
    /// Rule name
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Condition expression (e.g., "cpu_usage > 80")
    pub condition: String,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Notification channels as JSON array
    pub notification_channels: serde_json::Value,
    /// Whether the rule is enabled
    pub enabled: bool,
    /// Cooldown period in seconds (prevent alert spam)
    pub cooldown_seconds: i32,
    /// When the last alert was triggered
    pub last_triggered_at: Option<DateTime<Utc>>,
    /// When the rule was created
    pub created_at: DateTime<Utc>,
    /// When the rule was last updated
    pub updated_at: DateTime<Utc>,
}

/// Alert severity levels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "alert_severity", rename_all = "snake_case")]
pub enum AlertSeverity {
    /// Informational alert
    Info,
    /// Warning alert
    Warning,
    /// Critical alert requiring attention
    Critical,
}

/// Alert instance when a rule is triggered.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct HealthAlert {
    /// Unique alert identifier
    pub id: Uuid,
    /// Reference to the alert rule
    pub rule_id: Uuid,
    /// Alert status
    pub status: AlertStatus,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert message
    pub message: String,
    /// Context that triggered the alert
    pub context: Option<serde_json::Value>,
    /// When the alert was triggered
    pub triggered_at: DateTime<Utc>,
    /// When the alert was acknowledged
    pub acknowledged_at: Option<DateTime<Utc>>,
    /// User who acknowledged the alert
    pub acknowledged_by: Option<Uuid>,
    /// When the alert was resolved
    pub resolved_at: Option<DateTime<Utc>>,
}

/// Alert status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "alert_status", rename_all = "snake_case")]
pub enum AlertStatus {
    /// Alert is active
    Active,
    /// Alert has been acknowledged
    Acknowledged,
    /// Alert has been resolved
    Resolved,
    /// Alert was silenced
    Silenced,
}

/// Prometheus-compatible metric.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PrometheusMetric {
    /// Metric name
    pub name: String,
    /// Metric help text
    pub help: String,
    /// Metric type (counter, gauge, histogram, summary)
    pub metric_type: String,
    /// Metric labels
    pub labels: Option<serde_json::Value>,
    /// Metric value
    pub value: f64,
}

/// Request to acknowledge an alert.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AcknowledgeAlert {
    /// Acknowledgement note
    pub note: Option<String>,
}

/// Request to resolve an alert.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResolveAlert {
    /// Resolution note
    pub note: Option<String>,
}

// ==================== Common Types ====================

/// Pagination response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedResponse<T> {
    /// Items in this page
    pub items: Vec<T>,
    /// Total count of items
    pub total: i64,
    /// Current page limit
    pub limit: i64,
    /// Current page offset
    pub offset: i64,
}

/// Dashboard overview for infrastructure monitoring.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InfrastructureDashboard {
    /// System health
    pub health: SystemHealth,
    /// Active feature flags count
    pub active_feature_flags: i64,
    /// Background job queue stats
    pub job_queue_stats: Vec<BackgroundJobQueueStats>,
    /// Active alerts count
    pub active_alerts: i64,
    /// Recent traces count (last hour)
    pub recent_traces_count: i64,
    /// Error rate (last hour)
    pub error_rate_percent: f64,
    /// Average response time (last hour)
    pub avg_response_time_ms: f64,
}

/// Job type constants for background jobs.
pub mod job_type {
    pub const EMAIL_SEND: &str = "email_send";
    pub const SMS_SEND: &str = "sms_send";
    pub const PUSH_NOTIFICATION: &str = "push_notification";
    pub const REPORT_GENERATE: &str = "report_generate";
    pub const DATA_EXPORT: &str = "data_export";
    pub const DATA_IMPORT: &str = "data_import";
    pub const DOCUMENT_PROCESS: &str = "document_process";
    pub const IMAGE_RESIZE: &str = "image_resize";
    pub const WEBHOOK_DELIVERY: &str = "webhook_delivery";
    pub const SCHEDULED_TASK: &str = "scheduled_task";
    pub const CLEANUP: &str = "cleanup";
    pub const SYNC_EXTERNAL: &str = "sync_external";

    pub const ALL: &[&str] = &[
        EMAIL_SEND,
        SMS_SEND,
        PUSH_NOTIFICATION,
        REPORT_GENERATE,
        DATA_EXPORT,
        DATA_IMPORT,
        DOCUMENT_PROCESS,
        IMAGE_RESIZE,
        WEBHOOK_DELIVERY,
        SCHEDULED_TASK,
        CLEANUP,
        SYNC_EXTERNAL,
    ];
}

/// Queue name constants for background jobs.
pub mod queue {
    pub const DEFAULT: &str = "default";
    pub const HIGH_PRIORITY: &str = "high";
    pub const LOW_PRIORITY: &str = "low";
    pub const NOTIFICATIONS: &str = "notifications";
    pub const REPORTS: &str = "reports";
    pub const IMPORTS: &str = "imports";
    pub const EXPORTS: &str = "exports";

    pub const ALL: &[&str] = &[
        DEFAULT,
        HIGH_PRIORITY,
        LOW_PRIORITY,
        NOTIFICATIONS,
        REPORTS,
        IMPORTS,
        EXPORTS,
    ];
}
