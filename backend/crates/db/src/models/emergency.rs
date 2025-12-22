//! Emergency management models for Epic 23.
//!
//! Covers emergency protocols, contacts, incidents, and broadcasts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

// ============================================
// Constants
// ============================================

/// Protocol types for emergency protocols.
pub mod protocol_type {
    pub const FIRE: &str = "fire";
    pub const FLOOD: &str = "flood";
    pub const GAS_LEAK: &str = "gas_leak";
    pub const POWER_OUTAGE: &str = "power_outage";
    pub const SECURITY_THREAT: &str = "security_threat";
    pub const MEDICAL: &str = "medical";
    pub const NATURAL_DISASTER: &str = "natural_disaster";
    pub const OTHER: &str = "other";
}

/// Contact types for emergency contacts.
pub mod contact_type {
    pub const FIRE_DEPARTMENT: &str = "fire_department";
    pub const POLICE: &str = "police";
    pub const AMBULANCE: &str = "ambulance";
    pub const UTILITY_COMPANY: &str = "utility_company";
    pub const BUILDING_MANAGER: &str = "building_manager";
    pub const SECURITY: &str = "security";
    pub const MAINTENANCE: &str = "maintenance";
    pub const MEDICAL: &str = "medical";
    pub const OTHER: &str = "other";
}

/// Incident types.
pub mod incident_type {
    pub const FIRE: &str = "fire";
    pub const FLOOD: &str = "flood";
    pub const GAS_LEAK: &str = "gas_leak";
    pub const POWER_OUTAGE: &str = "power_outage";
    pub const SECURITY_THREAT: &str = "security_threat";
    pub const MEDICAL: &str = "medical";
    pub const NATURAL_DISASTER: &str = "natural_disaster";
    pub const STRUCTURAL: &str = "structural";
    pub const OTHER: &str = "other";
}

/// Severity levels.
pub mod severity {
    pub const LOW: &str = "low";
    pub const MEDIUM: &str = "medium";
    pub const HIGH: &str = "high";
    pub const CRITICAL: &str = "critical";
}

/// Incident status values.
pub mod incident_status {
    pub const REPORTED: &str = "reported";
    pub const ACKNOWLEDGED: &str = "acknowledged";
    pub const RESPONDING: &str = "responding";
    pub const CONTAINED: &str = "contained";
    pub const RESOLVED: &str = "resolved";
    pub const CLOSED: &str = "closed";
}

/// Acknowledgment status values.
pub mod acknowledgment_status {
    pub const SAFE: &str = "safe";
    pub const NEED_HELP: &str = "need_help";
    pub const EVACUATED: &str = "evacuated";
    pub const OTHER: &str = "other";
}

/// Drill types.
pub mod drill_type {
    pub const FIRE: &str = "fire";
    pub const EVACUATION: &str = "evacuation";
    pub const LOCKDOWN: &str = "lockdown";
    pub const FIRST_AID: &str = "first_aid";
    pub const FULL_SCALE: &str = "full_scale";
}

/// Drill status values.
pub mod drill_status {
    pub const SCHEDULED: &str = "scheduled";
    pub const IN_PROGRESS: &str = "in_progress";
    pub const COMPLETED: &str = "completed";
    pub const CANCELLED: &str = "cancelled";
}

// ============================================
// Emergency Protocol Models
// ============================================

/// Emergency protocol definition.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EmergencyProtocol {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,
    pub name: String,
    pub protocol_type: String,
    pub description: Option<String>,
    pub steps: serde_json::Value,
    pub contacts: serde_json::Value,
    pub evacuation_info: Option<String>,
    pub attachments: Option<serde_json::Value>,
    pub is_active: bool,
    pub priority: i32,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create emergency protocol request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateEmergencyProtocol {
    pub building_id: Option<Uuid>,
    pub name: String,
    pub protocol_type: String,
    pub description: Option<String>,
    pub steps: serde_json::Value,
    pub contacts: Option<serde_json::Value>,
    pub evacuation_info: Option<String>,
    pub attachments: Option<serde_json::Value>,
    pub is_active: Option<bool>,
    pub priority: Option<i32>,
}

/// Update emergency protocol request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateEmergencyProtocol {
    pub building_id: Option<Uuid>,
    pub name: Option<String>,
    pub protocol_type: Option<String>,
    pub description: Option<String>,
    pub steps: Option<serde_json::Value>,
    pub contacts: Option<serde_json::Value>,
    pub evacuation_info: Option<String>,
    pub attachments: Option<serde_json::Value>,
    pub is_active: Option<bool>,
    pub priority: Option<i32>,
}

/// Query parameters for emergency protocols.
#[derive(Debug, Clone, Serialize, Deserialize, IntoParams)]
pub struct EmergencyProtocolQuery {
    pub building_id: Option<Uuid>,
    pub protocol_type: Option<String>,
    pub is_active: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// ============================================
// Emergency Contact Models
// ============================================

/// Emergency contact record.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EmergencyContact {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,
    pub name: String,
    pub role: String,
    pub phone: Option<String>,
    pub phone_secondary: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
    pub priority_order: i32,
    pub contact_type: String,
    pub is_active: bool,
    pub available_hours: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create emergency contact request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateEmergencyContact {
    pub building_id: Option<Uuid>,
    pub name: String,
    pub role: String,
    pub phone: Option<String>,
    pub phone_secondary: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
    pub priority_order: Option<i32>,
    pub contact_type: String,
    pub available_hours: Option<String>,
}

/// Update emergency contact request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateEmergencyContact {
    pub building_id: Option<Uuid>,
    pub name: Option<String>,
    pub role: Option<String>,
    pub phone: Option<String>,
    pub phone_secondary: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
    pub priority_order: Option<i32>,
    pub contact_type: Option<String>,
    pub is_active: Option<bool>,
    pub available_hours: Option<String>,
}

/// Query parameters for emergency contacts.
#[derive(Debug, Clone, Serialize, Deserialize, IntoParams)]
pub struct EmergencyContactQuery {
    pub building_id: Option<Uuid>,
    pub contact_type: Option<String>,
    pub is_active: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// ============================================
// Emergency Incident Models
// ============================================

/// Emergency incident record.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EmergencyIncident {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub reported_by: Uuid,
    pub incident_type: String,
    pub severity: String,
    pub title: String,
    pub description: String,
    pub location_details: Option<String>,
    pub latitude: Option<rust_decimal::Decimal>,
    pub longitude: Option<rust_decimal::Decimal>,
    pub status: String,
    pub resolution: Option<String>,
    pub resolved_by: Option<Uuid>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub protocol_id: Option<Uuid>,
    pub fault_id: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
    pub reported_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create emergency incident request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateEmergencyIncident {
    pub building_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub incident_type: String,
    pub severity: Option<String>,
    pub title: String,
    pub description: String,
    pub location_details: Option<String>,
    pub latitude: Option<rust_decimal::Decimal>,
    pub longitude: Option<rust_decimal::Decimal>,
    pub protocol_id: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
}

/// Update emergency incident request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateEmergencyIncident {
    pub incident_type: Option<String>,
    pub severity: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub location_details: Option<String>,
    pub status: Option<String>,
    pub resolution: Option<String>,
    pub protocol_id: Option<Uuid>,
    pub fault_id: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
}

/// Query parameters for emergency incidents.
#[derive(Debug, Clone, Serialize, Deserialize, IntoParams)]
pub struct EmergencyIncidentQuery {
    pub building_id: Option<Uuid>,
    pub incident_type: Option<String>,
    pub severity: Option<String>,
    pub status: Option<String>,
    pub active_only: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Emergency incident attachment.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EmergencyIncidentAttachment {
    pub id: Uuid,
    pub incident_id: Uuid,
    pub document_id: Uuid,
    pub attachment_type: String,
    pub description: Option<String>,
    pub uploaded_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Add incident attachment request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AddIncidentAttachment {
    pub document_id: Uuid,
    pub attachment_type: Option<String>,
    pub description: Option<String>,
}

/// Emergency incident update/timeline entry.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EmergencyIncidentUpdate {
    pub id: Uuid,
    pub incident_id: Uuid,
    pub update_type: String,
    pub previous_status: Option<String>,
    pub new_status: Option<String>,
    pub message: String,
    pub updated_by: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Create incident update request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateIncidentUpdate {
    pub update_type: Option<String>,
    pub previous_status: Option<String>,
    pub new_status: Option<String>,
    pub message: String,
}

// ============================================
// Emergency Broadcast Models
// ============================================

/// Emergency broadcast record.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EmergencyBroadcast {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Uuid,
    pub incident_id: Option<Uuid>,
    pub title: String,
    pub message: String,
    pub severity: String,
    pub channels: serde_json::Value,
    pub sent_by: Uuid,
    pub sent_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub recipient_count: Option<i32>,
    pub delivered_count: Option<i32>,
    pub acknowledged_count: Option<i32>,
    pub metadata: Option<serde_json::Value>,
}

/// Create emergency broadcast request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateEmergencyBroadcast {
    pub building_id: Uuid,
    pub incident_id: Option<Uuid>,
    pub title: String,
    pub message: String,
    pub severity: Option<String>,
    pub channels: Option<serde_json::Value>,
    pub expires_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

/// Query parameters for emergency broadcasts.
#[derive(Debug, Clone, Serialize, Deserialize, IntoParams)]
pub struct EmergencyBroadcastQuery {
    pub building_id: Option<Uuid>,
    pub incident_id: Option<Uuid>,
    pub is_active: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Emergency broadcast acknowledgment.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EmergencyBroadcastAcknowledgment {
    pub id: Uuid,
    pub broadcast_id: Uuid,
    pub user_id: Uuid,
    pub status: String,
    pub message: Option<String>,
    pub acknowledged_at: DateTime<Utc>,
}

/// Acknowledge broadcast request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AcknowledgeBroadcast {
    pub status: String,
    pub message: Option<String>,
}

// ============================================
// Emergency Drill Models
// ============================================

/// Emergency drill record.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EmergencyDrill {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Uuid,
    pub protocol_id: Option<Uuid>,
    pub drill_type: String,
    pub title: String,
    pub description: Option<String>,
    pub scheduled_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: String,
    pub participants_expected: Option<i32>,
    pub participants_actual: Option<i32>,
    pub duration_minutes: Option<i32>,
    pub notes: Option<String>,
    pub issues_found: Option<serde_json::Value>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create emergency drill request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateEmergencyDrill {
    pub building_id: Uuid,
    pub protocol_id: Option<Uuid>,
    pub drill_type: String,
    pub title: String,
    pub description: Option<String>,
    pub scheduled_at: DateTime<Utc>,
    pub participants_expected: Option<i32>,
}

/// Update emergency drill request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateEmergencyDrill {
    pub protocol_id: Option<Uuid>,
    pub drill_type: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub participants_expected: Option<i32>,
    pub participants_actual: Option<i32>,
    pub duration_minutes: Option<i32>,
    pub notes: Option<String>,
    pub issues_found: Option<serde_json::Value>,
}

/// Complete drill request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompleteDrill {
    pub participants_actual: i32,
    pub duration_minutes: i32,
    pub notes: Option<String>,
    pub issues_found: Option<serde_json::Value>,
}

/// Query parameters for emergency drills.
#[derive(Debug, Clone, Serialize, Deserialize, IntoParams)]
pub struct EmergencyDrillQuery {
    pub building_id: Option<Uuid>,
    pub drill_type: Option<String>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// ============================================
// Statistics and Summary Models
// ============================================

/// Emergency statistics summary.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EmergencyStatistics {
    pub total_protocols: i64,
    pub active_protocols: i64,
    pub total_contacts: i64,
    pub total_incidents: i64,
    pub active_incidents: i64,
    pub resolved_incidents: i64,
    pub total_broadcasts: i64,
    pub active_broadcasts: i64,
    pub total_drills: i64,
    pub completed_drills: i64,
}

/// Incident summary by type.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct IncidentTypeSummary {
    pub incident_type: String,
    pub count: i64,
    pub active_count: i64,
}

/// Incident summary by severity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct IncidentSeveritySummary {
    pub severity: String,
    pub count: i64,
}

/// Broadcast delivery statistics.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BroadcastDeliveryStats {
    pub broadcast_id: Uuid,
    pub recipient_count: i32,
    pub delivered_count: i32,
    pub acknowledged_count: i32,
    pub safe_count: i64,
    pub need_help_count: i64,
}
