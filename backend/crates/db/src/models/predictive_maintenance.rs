//! Models for Epic 134: Predictive Maintenance & Equipment Intelligence.
//!
//! - Story 134.1: Equipment Registry
//! - Story 134.2: Maintenance History Tracking
//! - Story 134.3: Failure Prediction Engine
//! - Story 134.4: Predictive Maintenance Dashboard

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// ENUMS
// ============================================================================

/// Equipment type categories.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EquipmentType {
    Hvac,
    Elevator,
    Plumbing,
    Electrical,
    FireSafety,
    Security,
    Generator,
    Pump,
    Boiler,
    Chiller,
    Other,
}

impl std::fmt::Display for EquipmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hvac => write!(f, "hvac"),
            Self::Elevator => write!(f, "elevator"),
            Self::Plumbing => write!(f, "plumbing"),
            Self::Electrical => write!(f, "electrical"),
            Self::FireSafety => write!(f, "fire_safety"),
            Self::Security => write!(f, "security"),
            Self::Generator => write!(f, "generator"),
            Self::Pump => write!(f, "pump"),
            Self::Boiler => write!(f, "boiler"),
            Self::Chiller => write!(f, "chiller"),
            Self::Other => write!(f, "other"),
        }
    }
}

/// Equipment operational status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EquipmentStatus {
    Operational,
    NeedsMaintenance,
    UnderRepair,
    Decommissioned,
}

impl std::fmt::Display for EquipmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Operational => write!(f, "operational"),
            Self::NeedsMaintenance => write!(f, "needs_maintenance"),
            Self::UnderRepair => write!(f, "under_repair"),
            Self::Decommissioned => write!(f, "decommissioned"),
        }
    }
}

/// Maintenance type categories.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MaintenanceType {
    Preventive,
    Corrective,
    Emergency,
    Inspection,
}

impl std::fmt::Display for MaintenanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Preventive => write!(f, "preventive"),
            Self::Corrective => write!(f, "corrective"),
            Self::Emergency => write!(f, "emergency"),
            Self::Inspection => write!(f, "inspection"),
        }
    }
}

/// Maintenance outcome.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MaintenanceOutcome {
    Completed,
    Partial,
    Failed,
    Deferred,
}

/// Alert severity levels.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AlertSeverity {
    Critical,
    High,
    Medium,
    Low,
}

impl std::fmt::Display for AlertSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Critical => write!(f, "critical"),
            Self::High => write!(f, "high"),
            Self::Medium => write!(f, "medium"),
            Self::Low => write!(f, "low"),
        }
    }
}

/// Alert status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AlertStatus {
    Active,
    Acknowledged,
    Resolved,
    Dismissed,
}

/// Alert type categories.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AlertType {
    HealthThreshold,
    PredictedFailure,
    WarrantyExpiry,
    OverdueMaintenance,
}

/// Recommended action from prediction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RecommendedAction {
    ScheduleMaintenance,
    Replace,
    Monitor,
    None,
}

// ============================================================================
// EQUIPMENT REGISTRY (Story 134.1)
// ============================================================================

/// Equipment registry entry.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Equipment {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Uuid,

    // Basic Info
    pub name: String,
    pub equipment_type: String,
    pub category: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,

    // Location
    pub location_description: Option<String>,
    pub floor_number: Option<i32>,
    pub unit_id: Option<Uuid>,

    // Lifecycle
    pub installation_date: Option<NaiveDate>,
    pub warranty_expiry_date: Option<NaiveDate>,
    pub expected_lifespan_years: Option<i32>,
    pub replacement_cost: Option<Decimal>,

    // Health
    pub health_score: Option<i32>,
    pub status: String,
    pub last_prediction_at: Option<DateTime<Utc>>,
    pub next_predicted_failure: Option<NaiveDate>,
    pub failure_probability: Option<Decimal>,

    pub notes: Option<String>,
    pub specifications: Option<serde_json::Value>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

/// Request to create equipment.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateEquipment {
    pub building_id: Uuid,
    pub name: String,
    pub equipment_type: String,
    pub category: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub location_description: Option<String>,
    pub floor_number: Option<i32>,
    pub unit_id: Option<Uuid>,
    pub installation_date: Option<NaiveDate>,
    pub warranty_expiry_date: Option<NaiveDate>,
    pub expected_lifespan_years: Option<i32>,
    pub replacement_cost: Option<Decimal>,
    pub notes: Option<String>,
    pub specifications: Option<serde_json::Value>,
}

/// Request to update equipment.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateEquipment {
    pub name: Option<String>,
    pub equipment_type: Option<String>,
    pub category: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub location_description: Option<String>,
    pub floor_number: Option<i32>,
    pub unit_id: Option<Uuid>,
    pub installation_date: Option<NaiveDate>,
    pub warranty_expiry_date: Option<NaiveDate>,
    pub expected_lifespan_years: Option<i32>,
    pub replacement_cost: Option<Decimal>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub specifications: Option<serde_json::Value>,
}

/// Equipment query parameters.
#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct EquipmentQuery {
    pub building_id: Option<Uuid>,
    pub equipment_type: Option<String>,
    pub status: Option<String>,
    pub min_health_score: Option<i32>,
    pub max_health_score: Option<i32>,
    pub sort_by: Option<String>, // health_score, name, next_predicted_failure
    pub sort_order: Option<String>, // asc, desc
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Equipment summary for dashboard.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EquipmentSummary {
    pub id: Uuid,
    pub name: String,
    pub equipment_type: String,
    pub building_name: String,
    pub health_score: Option<i32>,
    pub status: String,
    pub next_predicted_failure: Option<NaiveDate>,
    pub failure_probability: Option<Decimal>,
}

// ============================================================================
// EQUIPMENT DOCUMENTS
// ============================================================================

/// Equipment document (manual, warranty, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EquipmentDocument {
    pub id: Uuid,
    pub equipment_id: Uuid,
    pub organization_id: Uuid,
    pub document_type: String,
    pub title: String,
    pub file_path: Option<String>,
    pub file_size: Option<i32>,
    pub mime_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub uploaded_by: Option<Uuid>,
}

/// Request to add equipment document.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateEquipmentDocument {
    pub document_type: String,
    pub title: String,
    pub file_path: String,
    pub file_size: Option<i32>,
    pub mime_type: Option<String>,
}

// ============================================================================
// MAINTENANCE LOGS (Story 134.2)
// ============================================================================

/// Maintenance log entry.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct MaintenanceLog {
    pub id: Uuid,
    pub equipment_id: Uuid,
    pub organization_id: Uuid,

    pub maintenance_type: String,
    pub description: String,

    pub scheduled_date: Option<NaiveDate>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i32>,

    pub cost: Option<Decimal>,
    pub currency: Option<String>,
    pub vendor_id: Option<Uuid>,
    pub vendor_name: Option<String>,
    pub technician_name: Option<String>,

    pub parts_replaced: Option<serde_json::Value>,
    pub work_performed: Option<String>,

    pub fault_id: Option<Uuid>,
    pub work_order_id: Option<Uuid>,

    pub outcome: Option<String>,
    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

/// Request to create maintenance log.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateMaintenanceLog {
    pub equipment_id: Uuid,
    pub maintenance_type: String,
    pub description: String,
    pub scheduled_date: Option<NaiveDate>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i32>,
    pub cost: Option<Decimal>,
    pub currency: Option<String>,
    pub vendor_id: Option<Uuid>,
    pub vendor_name: Option<String>,
    pub technician_name: Option<String>,
    pub parts_replaced: Option<serde_json::Value>,
    pub work_performed: Option<String>,
    pub fault_id: Option<Uuid>,
    pub work_order_id: Option<Uuid>,
    pub outcome: Option<String>,
    pub notes: Option<String>,
}

/// Request to update maintenance log.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateMaintenanceLog {
    pub maintenance_type: Option<String>,
    pub description: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i32>,
    pub cost: Option<Decimal>,
    pub currency: Option<String>,
    pub vendor_name: Option<String>,
    pub technician_name: Option<String>,
    pub parts_replaced: Option<serde_json::Value>,
    pub work_performed: Option<String>,
    pub outcome: Option<String>,
    pub notes: Option<String>,
}

/// Maintenance log photo.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct MaintenanceLogPhoto {
    pub id: Uuid,
    pub maintenance_log_id: Uuid,
    pub file_path: String,
    pub file_size: Option<i32>,
    pub mime_type: Option<String>,
    pub caption: Option<String>,
    pub photo_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub uploaded_by: Option<Uuid>,
}

// ============================================================================
// PREDICTIONS (Story 134.3)
// ============================================================================

/// Prediction factor.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PredictionFactor {
    pub factor: String,
    pub weight: f64,
    pub value: String,
    pub description: Option<String>,
}

/// Equipment prediction record.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EquipmentPrediction {
    pub id: Uuid,
    pub equipment_id: Uuid,
    pub organization_id: Uuid,

    pub health_score: i32,
    pub failure_probability: Decimal,
    pub predicted_failure_date: Option<NaiveDate>,
    pub confidence_level: Option<Decimal>,

    pub factors: serde_json::Value,

    pub model_version: Option<String>,
    pub model_type: Option<String>,

    pub recommended_action: Option<String>,
    pub recommended_date: Option<NaiveDate>,
    pub urgency: Option<String>,

    pub created_at: DateTime<Utc>,
}

/// Request to run prediction.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RunPredictionRequest {
    pub equipment_ids: Option<Vec<Uuid>>, // None = all equipment
    pub building_id: Option<Uuid>,        // Filter by building
}

/// Prediction result for API response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PredictionResult {
    pub equipment_id: Uuid,
    pub equipment_name: String,
    pub health_score: i32,
    pub failure_probability: f64,
    pub predicted_failure_date: Option<NaiveDate>,
    pub recommended_action: String,
    pub urgency: String,
    pub factors: Vec<PredictionFactor>,
}

// ============================================================================
// ALERTS
// ============================================================================

/// Maintenance alert.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct MaintenanceAlert {
    pub id: Uuid,
    pub equipment_id: Uuid,
    pub organization_id: Uuid,
    pub prediction_id: Option<Uuid>,

    pub alert_type: String,
    pub severity: String,
    pub title: String,
    pub message: String,

    pub status: String,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub acknowledged_by: Option<Uuid>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<Uuid>,

    pub maintenance_log_id: Option<Uuid>,

    pub created_at: DateTime<Utc>,
}

/// Alert with equipment info.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AlertWithEquipment {
    pub id: Uuid,
    pub equipment_id: Uuid,
    pub equipment_name: String,
    pub building_name: String,
    pub alert_type: String,
    pub severity: String,
    pub title: String,
    pub message: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

/// Request to acknowledge alert.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AcknowledgeAlertRequest {
    pub notes: Option<String>,
}

/// Request to resolve alert.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResolveAlertRequest {
    pub maintenance_log_id: Option<Uuid>,
    pub notes: Option<String>,
}

// ============================================================================
// HEALTH THRESHOLDS
// ============================================================================

/// Health threshold configuration.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct HealthThreshold {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub equipment_type: String,
    pub critical_threshold: i32,
    pub warning_threshold: i32,
    pub alert_on_critical: bool,
    pub alert_on_warning: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to set health threshold.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SetHealthThreshold {
    pub equipment_type: String,
    pub critical_threshold: i32,
    pub warning_threshold: i32,
    pub alert_on_critical: Option<bool>,
    pub alert_on_warning: Option<bool>,
}

// ============================================================================
// DASHBOARD (Story 134.4)
// ============================================================================

/// Dashboard statistics.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MaintenanceDashboard {
    pub total_equipment: i32,
    pub equipment_by_status: EquipmentByStatus,
    pub health_distribution: HealthDistribution,
    pub active_alerts: i32,
    pub alerts_by_severity: AlertsBySeverity,
    pub upcoming_maintenance: i32,
    pub overdue_maintenance: i32,
    pub critical_equipment: Vec<EquipmentSummary>,
}

/// Equipment count by status.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EquipmentByStatus {
    pub operational: i32,
    pub needs_maintenance: i32,
    pub under_repair: i32,
    pub decommissioned: i32,
}

/// Equipment health distribution.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthDistribution {
    pub excellent: i32, // 80-100
    pub good: i32,      // 60-79
    pub fair: i32,      // 40-59
    pub poor: i32,      // 20-39
    pub critical: i32,  // 0-19
}

/// Alerts by severity.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AlertsBySeverity {
    pub critical: i32,
    pub high: i32,
    pub medium: i32,
    pub low: i32,
}

/// Maintenance trend data point.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MaintenanceTrend {
    pub date: NaiveDate,
    pub preventive: i32,
    pub corrective: i32,
    pub emergency: i32,
}
