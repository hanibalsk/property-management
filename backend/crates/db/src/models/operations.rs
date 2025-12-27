//! Infrastructure & Operations models (Epic 73).
//!
//! Provides types for blue-green deployment, database migration safety,
//! disaster recovery, and cost monitoring.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// ENUMS
// ============================================================================

/// Deployment environment type.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "deployment_environment", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DeploymentEnvironment {
    #[default]
    Blue,
    Green,
}

/// Deployment status.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "deployment_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DeploymentStatus {
    #[default]
    Pending,
    Deploying,
    HealthChecking,
    Switching,
    Active,
    RolledBack,
    Failed,
}

/// Migration status.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "migration_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MigrationStatus {
    #[default]
    Pending,
    Running,
    Completed,
    RolledBack,
    Failed,
}

/// Migration strategy type.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "migration_strategy", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MigrationStrategy {
    /// Standard migration with possible table locks
    #[default]
    Standard,
    /// Expand-contract pattern for backward compatibility
    ExpandContract,
    /// Online DDL for zero-downtime
    OnlineDdl,
    /// Shadow table migration for large tables
    ShadowTable,
}

/// Backup type.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "backup_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum BackupType {
    #[default]
    Full,
    Incremental,
    Differential,
    PointInTime,
}

/// Backup status.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "backup_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum BackupStatus {
    #[default]
    InProgress,
    Completed,
    Verified,
    Failed,
    Expired,
}

/// Recovery status.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "recovery_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RecoveryStatus {
    #[default]
    Initiated,
    Restoring,
    Validating,
    Completed,
    Failed,
}

/// Cost alert severity.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "cost_alert_severity", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CostAlertSeverity {
    #[default]
    Info,
    Warning,
    Critical,
}

/// Cloud service type for cost tracking.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "cloud_service_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CloudServiceType {
    #[default]
    Compute,
    Database,
    Storage,
    Network,
    Cache,
    Cdn,
    Monitoring,
    Other,
}

// ============================================================================
// BLUE-GREEN DEPLOYMENT (Story 73.1)
// ============================================================================

/// Deployment record for blue-green deployments.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Deployment {
    pub id: Uuid,
    pub version: String,
    pub environment: DeploymentEnvironment,
    pub status: DeploymentStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployed_by: Option<Uuid>,
    pub started_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switched_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolled_back_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Health check result for deployment verification.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct DeploymentHealthCheck {
    pub id: Uuid,
    pub deployment_id: Uuid,
    pub check_name: String,
    pub is_healthy: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_ms: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    pub checked_at: DateTime<Utc>,
}

/// Request to create a new deployment.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateDeployment {
    pub version: String,
    pub environment: DeploymentEnvironment,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Request to switch traffic to a deployment.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct SwitchTraffic {
    pub deployment_id: Uuid,
    #[serde(default)]
    pub force: bool,
}

/// Deployment status update.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateDeploymentStatus {
    pub status: DeploymentStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// Deployment dashboard response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DeploymentDashboard {
    pub current_blue: Option<Deployment>,
    pub current_green: Option<Deployment>,
    pub active_environment: DeploymentEnvironment,
    pub recent_deployments: Vec<Deployment>,
    pub pending_health_checks: Vec<DeploymentHealthCheck>,
    pub last_switch_at: Option<DateTime<Utc>>,
    pub rollback_available: bool,
}

// ============================================================================
// DATABASE MIGRATION SAFETY (Story 73.2)
// ============================================================================

/// Database migration record.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct DatabaseMigration {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub strategy: MigrationStrategy,
    pub status: MigrationStatus,
    pub is_backward_compatible: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_duration_secs: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_duration_secs: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_tables: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_sql: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_by: Option<Uuid>,
    pub started_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percentage: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Migration log entry for detailed progress tracking.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct MigrationLog {
    pub id: Uuid,
    pub migration_id: Uuid,
    pub message: String,
    pub log_level: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    pub logged_at: DateTime<Utc>,
}

/// Request to create a migration.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateMigration {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub strategy: MigrationStrategy,
    #[serde(default = "default_true")]
    pub is_backward_compatible: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_duration_secs: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_tables: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_sql: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Request to update migration progress.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateMigrationProgress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percentage: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MigrationStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// Schema version tracking.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct SchemaVersion {
    pub id: Uuid,
    pub version: String,
    pub description: String,
    pub checksum: String,
    pub applied_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_by: Option<Uuid>,
}

/// Migration safety check result.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct MigrationSafetyCheck {
    pub is_safe: bool,
    pub has_table_locks: bool,
    pub estimated_lock_time_secs: i32,
    pub affected_rows_estimate: i64,
    pub is_backward_compatible: bool,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
}

// ============================================================================
// DISASTER RECOVERY (Story 73.3)
// ============================================================================

/// Backup record.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Backup {
    pub id: Uuid,
    pub backup_type: BackupType,
    pub status: BackupStatus,
    pub size_bytes: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_region: Option<String>,
    pub is_encrypted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    pub started_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Recovery operation record.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct RecoveryOperation {
    pub id: Uuid,
    pub backup_id: Uuid,
    pub status: RecoveryStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_point_in_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_by: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub started_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_loss_window_secs: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_time_secs: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_result: Option<serde_json::Value>,
}

/// Disaster recovery drill record.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct DisasterRecoveryDrill {
    pub id: Uuid,
    pub drill_type: String,
    pub is_successful: bool,
    pub rto_target_secs: i32,
    pub rto_actual_secs: i32,
    pub rpo_target_secs: i32,
    pub rpo_actual_secs: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conducted_by: Option<Uuid>,
    pub conducted_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub improvements: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_drill_due: Option<NaiveDate>,
}

/// Request to initiate backup.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateBackup {
    #[serde(default)]
    pub backup_type: BackupType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_region: Option<String>,
    #[serde(default = "default_true")]
    pub is_encrypted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Request to initiate recovery.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct InitiateRecovery {
    pub backup_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_point_in_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Request to record DR drill.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct RecordDrDrill {
    pub drill_type: String,
    pub is_successful: bool,
    pub rto_target_secs: i32,
    pub rto_actual_secs: i32,
    pub rpo_target_secs: i32,
    pub rpo_actual_secs: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub improvements: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_drill_due: Option<NaiveDate>,
}

/// Disaster recovery dashboard.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DisasterRecoveryDashboard {
    pub last_backup: Option<Backup>,
    pub backup_frequency_hours: i32,
    pub total_backups: i64,
    pub verified_backups: i64,
    pub storage_used_bytes: i64,
    pub rto_target_hours: i32,
    pub rpo_target_hours: i32,
    pub last_recovery_test: Option<DateTime<Utc>>,
    pub last_drill: Option<DisasterRecoveryDrill>,
    pub next_drill_due: Option<NaiveDate>,
    pub compliance_status: String,
    pub recommendations: Vec<String>,
}

// ============================================================================
// COST MONITORING (Story 73.4)
// ============================================================================

/// Infrastructure cost record.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct InfrastructureCost {
    pub id: Uuid,
    pub service_type: CloudServiceType,
    pub service_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<serde_json::Value>,
    pub cost_amount: Decimal,
    pub currency: String,
    pub usage_quantity: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_unit: Option<String>,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    pub recorded_at: DateTime<Utc>,
}

/// Cost budget record.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct CostBudget {
    pub id: Uuid,
    pub name: String,
    pub budget_amount: Decimal,
    pub currency: String,
    pub period_type: String, // monthly, quarterly, yearly
    pub current_spend: Decimal,
    pub forecasted_spend: Decimal,
    pub alert_threshold_percent: i32,
    pub is_exceeded: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type_filter: Option<CloudServiceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_filter: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Cost alert record.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct CostAlert {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_id: Option<Uuid>,
    pub severity: CostAlertSeverity,
    pub message: String,
    pub current_amount: Decimal,
    pub threshold_amount: Decimal,
    pub currency: String,
    pub is_acknowledged: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged_by: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Resource utilization record.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ResourceUtilization {
    pub id: Uuid,
    pub service_type: CloudServiceType,
    pub resource_id: String,
    pub resource_name: String,
    pub cpu_utilization_percent: Decimal,
    pub memory_utilization_percent: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_utilization_percent: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_in_bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_out_bytes: Option<i64>,
    pub is_underutilized: bool,
    pub is_overutilized: bool,
    pub measured_at: DateTime<Utc>,
}

/// Cost optimization recommendation.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct CostOptimizationRecommendation {
    pub id: Uuid,
    pub resource_id: String,
    pub resource_name: String,
    pub service_type: CloudServiceType,
    pub recommendation_type: String,
    pub description: String,
    pub estimated_savings: Decimal,
    pub currency: String,
    pub effort_level: String, // low, medium, high
    pub is_implemented: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implemented_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implemented_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Request to create cost budget.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateCostBudget {
    pub name: String,
    pub budget_amount: Decimal,
    #[serde(default = "default_currency")]
    pub currency: String,
    pub period_type: String,
    #[serde(default = "default_alert_threshold")]
    pub alert_threshold_percent: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type_filter: Option<CloudServiceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_filter: Option<serde_json::Value>,
}

/// Request to record infrastructure cost.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct RecordInfrastructureCost {
    pub service_type: CloudServiceType,
    pub service_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<serde_json::Value>,
    pub cost_amount: Decimal,
    #[serde(default = "default_currency")]
    pub currency: String,
    pub usage_quantity: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_unit: Option<String>,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// Cost dashboard response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CostDashboard {
    pub total_cost_current_period: Decimal,
    pub total_cost_previous_period: Decimal,
    pub cost_change_percent: Decimal,
    pub currency: String,
    pub costs_by_service: Vec<ServiceCostSummary>,
    pub cost_trend: Vec<CostTrendPoint>,
    pub budgets: Vec<CostBudget>,
    pub active_alerts: Vec<CostAlert>,
    pub underutilized_resources: i64,
    pub optimization_recommendations: Vec<CostOptimizationRecommendation>,
    pub potential_savings: Decimal,
}

/// Cost summary by service type.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ServiceCostSummary {
    pub service_type: CloudServiceType,
    pub total_cost: Decimal,
    pub percentage_of_total: Decimal,
    pub change_from_previous: Decimal,
}

/// Cost trend data point.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CostTrendPoint {
    pub date: NaiveDate,
    pub total_cost: Decimal,
    pub projected: bool,
}

// ============================================================================
// QUERY PARAMETERS
// ============================================================================

/// Deployment list query parameters.
#[derive(Debug, Clone, Default, Deserialize, utoipa::IntoParams)]
pub struct DeploymentQuery {
    /// Environment filter
    pub environment: Option<DeploymentEnvironment>,
    /// Status filter
    pub status: Option<DeploymentStatus>,
    /// Limit
    #[serde(default = "default_limit")]
    pub limit: i64,
    /// Offset
    #[serde(default)]
    pub offset: i64,
}

/// Migration list query parameters.
#[derive(Debug, Clone, Default, Deserialize, utoipa::IntoParams)]
pub struct MigrationQuery {
    /// Status filter
    pub status: Option<MigrationStatus>,
    /// Limit
    #[serde(default = "default_limit")]
    pub limit: i64,
    /// Offset
    #[serde(default)]
    pub offset: i64,
}

/// Backup list query parameters.
#[derive(Debug, Clone, Default, Deserialize, utoipa::IntoParams)]
pub struct BackupQuery {
    /// Backup type filter
    pub backup_type: Option<BackupType>,
    /// Status filter
    pub status: Option<BackupStatus>,
    /// Limit
    #[serde(default = "default_limit")]
    pub limit: i64,
    /// Offset
    #[serde(default)]
    pub offset: i64,
}

/// Cost query parameters.
#[derive(Debug, Clone, Default, Deserialize, utoipa::IntoParams)]
pub struct CostQuery {
    /// Period start
    pub period_start: Option<NaiveDate>,
    /// Period end
    pub period_end: Option<NaiveDate>,
    /// Service type filter
    pub service_type: Option<CloudServiceType>,
    /// Limit
    #[serde(default = "default_limit")]
    pub limit: i64,
    /// Offset
    #[serde(default)]
    pub offset: i64,
}

/// Cost alert query parameters.
#[derive(Debug, Clone, Default, Deserialize, utoipa::IntoParams)]
pub struct CostAlertQuery {
    /// Show only unacknowledged alerts
    #[serde(default = "default_true")]
    pub unacknowledged_only: bool,
    /// Severity filter
    pub severity: Option<CostAlertSeverity>,
    /// Limit
    #[serde(default = "default_limit")]
    pub limit: i64,
    /// Offset
    #[serde(default)]
    pub offset: i64,
}

// ============================================================================
// LIST RESPONSES
// ============================================================================

/// List deployments response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ListDeploymentsResponse {
    pub deployments: Vec<Deployment>,
    pub total: i64,
}

/// List migrations response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ListMigrationsResponse {
    pub migrations: Vec<DatabaseMigration>,
    pub total: i64,
}

/// List backups response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ListBackupsResponse {
    pub backups: Vec<Backup>,
    pub total: i64,
}

/// List costs response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ListCostsResponse {
    pub costs: Vec<InfrastructureCost>,
    pub total: i64,
}

/// List cost alerts response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ListCostAlertsResponse {
    pub alerts: Vec<CostAlert>,
    pub total: i64,
}

/// List budgets response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ListBudgetsResponse {
    pub budgets: Vec<CostBudget>,
    pub total: i64,
}

/// List utilization response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ListUtilizationResponse {
    pub resources: Vec<ResourceUtilization>,
    pub total: i64,
}

/// List recommendations response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ListRecommendationsResponse {
    pub recommendations: Vec<CostOptimizationRecommendation>,
    pub total: i64,
    pub total_potential_savings: Decimal,
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn default_true() -> bool {
    true
}

fn default_limit() -> i64 {
    50
}

fn default_currency() -> String {
    "EUR".to_string()
}

fn default_alert_threshold() -> i32 {
    80
}
