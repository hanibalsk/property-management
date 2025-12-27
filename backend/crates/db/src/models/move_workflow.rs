//! Move-in/Move-out Workflow models (Epic 76).

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub mod workflow_type {
    pub const MOVE_IN: &str = "move_in";
    pub const MOVE_OUT: &str = "move_out";
}

pub mod workflow_status {
    pub const PENDING: &str = "pending";
    pub const SCHEDULED: &str = "scheduled";
    pub const IN_PROGRESS: &str = "in_progress";
    pub const COMPLETED: &str = "completed";
    pub const CANCELLED: &str = "cancelled";
}

pub mod item_condition {
    pub const EXCELLENT: &str = "excellent";
    pub const GOOD: &str = "good";
    pub const FAIR: &str = "fair";
    pub const POOR: &str = "poor";
    pub const DAMAGED: &str = "damaged";
}

pub mod key_type {
    pub const MAIN_ENTRANCE: &str = "main_entrance";
    pub const UNIT_DOOR: &str = "unit_door";
    pub const MAILBOX: &str = "mailbox";
    pub const STORAGE: &str = "storage";
    pub const ACCESS_CARD: &str = "access_card";
}

pub mod deduction_status {
    pub const PROPOSED: &str = "proposed";
    pub const DISPUTED: &str = "disputed";
    pub const ACCEPTED: &str = "accepted";
    pub const FINALIZED: &str = "finalized";
}

pub mod task_status {
    pub const PENDING: &str = "pending";
    pub const IN_PROGRESS: &str = "in_progress";
    pub const COMPLETED: &str = "completed";
    pub const OVERDUE: &str = "overdue";
}

pub mod assignee_type {
    pub const TENANT: &str = "tenant";
    pub const MANAGER: &str = "manager";
    pub const VENDOR: &str = "vendor";
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MoveWorkflow {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Uuid,
    pub unit_id: Uuid,
    pub lease_id: Option<Uuid>,
    pub tenant_user_id: Uuid,
    pub workflow_type: String,
    pub status: String,
    pub scheduled_date: NaiveDate,
    pub scheduled_time: Option<String>,
    pub actual_date: Option<NaiveDate>,
    pub manager_id: Option<Uuid>,
    pub notes: Option<String>,
    pub deposit_amount: Option<f64>,
    pub deposit_returned: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InspectionTemplate {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub workflow_type: String,
    pub is_default: bool,
    pub rooms: sqlx::types::Json<Vec<RoomTemplate>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomTemplate {
    pub name: String,
    pub items: Vec<ItemTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemTemplate {
    pub name: String,
    pub description: Option<String>,
    pub requires_photo: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Inspection {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub template_id: Option<Uuid>,
    pub inspection_type: String,
    pub inspector_id: Uuid,
    pub tenant_present: bool,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub overall_condition: Option<String>,
    pub notes: Option<String>,
    pub tenant_signature: Option<String>,
    pub tenant_signed_at: Option<DateTime<Utc>>,
    pub inspector_signature: Option<String>,
    pub inspector_signed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InspectionItem {
    pub id: Uuid,
    pub inspection_id: Uuid,
    pub room_name: String,
    pub item_name: String,
    pub condition: String,
    pub notes: Option<String>,
    pub requires_repair: bool,
    pub estimated_repair_cost: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InspectionPhoto {
    pub id: Uuid,
    pub inspection_item_id: Uuid,
    pub file_url: String,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub caption: Option<String>,
    pub taken_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct KeyHandoff {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub key_type: String,
    pub key_identifier: Option<String>,
    pub quantity: i32,
    pub handed_by_id: Uuid,
    pub received_by_id: Uuid,
    pub handed_at: DateTime<Utc>,
    pub notes: Option<String>,
    pub receiver_signature: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DepositDeduction {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub inspection_item_id: Option<Uuid>,
    pub description: String,
    pub amount: f64,
    pub status: String,
    pub proposed_by_id: Uuid,
    pub proposed_at: DateTime<Utc>,
    pub dispute_reason: Option<String>,
    pub disputed_at: Option<DateTime<Utc>>,
    pub resolution_notes: Option<String>,
    pub resolved_by_id: Option<Uuid>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MoveTimelineTask {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub task_name: String,
    pub description: Option<String>,
    pub assignee_type: String,
    pub assignee_id: Option<Uuid>,
    pub due_date: NaiveDate,
    pub status: String,
    pub order_index: i32,
    pub is_required: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_by_id: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMoveWorkflow {
    pub building_id: Uuid,
    pub unit_id: Uuid,
    pub lease_id: Option<Uuid>,
    pub tenant_user_id: Uuid,
    pub workflow_type: String,
    pub scheduled_date: NaiveDate,
    pub scheduled_time: Option<String>,
    pub manager_id: Option<Uuid>,
    pub notes: Option<String>,
    pub deposit_amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMoveWorkflow {
    pub status: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub scheduled_time: Option<String>,
    pub actual_date: Option<NaiveDate>,
    pub manager_id: Option<Uuid>,
    pub notes: Option<String>,
    pub deposit_returned: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInspectionTemplate {
    pub name: String,
    pub description: Option<String>,
    pub workflow_type: String,
    pub is_default: bool,
    pub rooms: Vec<RoomTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInspectionTemplate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_default: Option<bool>,
    pub rooms: Option<Vec<RoomTemplate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInspection {
    pub workflow_id: Uuid,
    pub template_id: Option<Uuid>,
    pub inspection_type: String,
    pub inspector_id: Uuid,
    pub tenant_present: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteInspection {
    pub overall_condition: String,
    pub notes: Option<String>,
    pub tenant_signature: Option<String>,
    pub inspector_signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInspectionItem {
    pub inspection_id: Uuid,
    pub room_name: String,
    pub item_name: String,
    pub condition: String,
    pub notes: Option<String>,
    pub requires_repair: bool,
    pub estimated_repair_cost: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInspectionItem {
    pub condition: Option<String>,
    pub notes: Option<String>,
    pub requires_repair: Option<bool>,
    pub estimated_repair_cost: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInspectionPhoto {
    pub inspection_item_id: Uuid,
    pub file_url: String,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub caption: Option<String>,
    pub taken_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKeyHandoff {
    pub workflow_id: Uuid,
    pub key_type: String,
    pub key_identifier: Option<String>,
    pub quantity: i32,
    pub handed_by_id: Uuid,
    pub received_by_id: Uuid,
    pub notes: Option<String>,
    pub receiver_signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepositDeduction {
    pub workflow_id: Uuid,
    pub inspection_item_id: Option<Uuid>,
    pub description: String,
    pub amount: f64,
    pub proposed_by_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeDeduction {
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveDeduction {
    pub status: String,
    pub resolution_notes: Option<String>,
    pub resolved_by_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTimelineTask {
    pub workflow_id: Uuid,
    pub task_name: String,
    pub description: Option<String>,
    pub assignee_type: String,
    pub assignee_id: Option<Uuid>,
    pub due_date: NaiveDate,
    pub order_index: i32,
    pub is_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTimelineTask {
    pub task_name: Option<String>,
    pub description: Option<String>,
    pub assignee_type: Option<String>,
    pub assignee_id: Option<Uuid>,
    pub due_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub order_index: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteTimelineTask {
    pub completed_by_id: Uuid,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowQuery {
    pub building_id: Option<Uuid>,
    pub unit_id: Option<Uuid>,
    pub tenant_user_id: Option<Uuid>,
    pub workflow_type: Option<String>,
    pub status: Option<String>,
    pub scheduled_from: Option<NaiveDate>,
    pub scheduled_to: Option<NaiveDate>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InspectionQuery {
    pub workflow_id: Option<Uuid>,
    pub inspector_id: Option<Uuid>,
    pub inspection_type: Option<String>,
    pub completed: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveWorkflowSummary {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub tenant_user_id: Uuid,
    pub workflow_type: String,
    pub status: String,
    pub scheduled_date: NaiveDate,
    pub building_name: Option<String>,
    pub unit_name: Option<String>,
    pub tenant_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveWorkflowWithDetails {
    #[serde(flatten)]
    pub workflow: MoveWorkflow,
    pub inspections: Vec<InspectionSummary>,
    pub key_handoffs: Vec<KeyHandoff>,
    pub deductions: Vec<DepositDeduction>,
    pub tasks: Vec<MoveTimelineTask>,
    pub building_name: Option<String>,
    pub unit_name: Option<String>,
    pub tenant_name: Option<String>,
    pub manager_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionSummary {
    pub id: Uuid,
    pub inspection_type: String,
    pub inspector_id: Uuid,
    pub inspector_name: Option<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub overall_condition: Option<String>,
    pub item_count: i64,
    pub photo_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionWithDetails {
    #[serde(flatten)]
    pub inspection: Inspection,
    pub items: Vec<InspectionItemWithPhotos>,
    pub inspector_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionItemWithPhotos {
    #[serde(flatten)]
    pub item: InspectionItem,
    pub photos: Vec<InspectionPhoto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveWorkflowStatistics {
    pub total_workflows: i64,
    pub pending_count: i64,
    pub in_progress_count: i64,
    pub completed_count: i64,
    pub move_in_count: i64,
    pub move_out_count: i64,
    pub avg_completion_days: Option<f64>,
    pub total_deposits_held: f64,
    pub total_deductions: f64,
    pub total_refunds: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositSummary {
    pub workflow_id: Uuid,
    pub deposit_amount: f64,
    pub total_deductions: f64,
    pub disputed_amount: f64,
    pub finalized_amount: f64,
    pub pending_amount: f64,
    pub refund_amount: f64,
    pub deductions: Vec<DepositDeduction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineOverview {
    pub workflow_id: Uuid,
    pub total_tasks: i64,
    pub completed_tasks: i64,
    pub pending_tasks: i64,
    pub overdue_tasks: i64,
    pub next_due_task: Option<MoveTimelineTask>,
    pub tasks: Vec<MoveTimelineTask>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionTemplateSummary {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub workflow_type: String,
    pub is_default: bool,
    pub room_count: usize,
    pub item_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTimelineFromTemplate {
    pub workflow_id: Uuid,
    pub base_date: NaiveDate,
    pub tasks: Vec<TimelineTaskTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineTaskTemplate {
    pub task_name: String,
    pub description: Option<String>,
    pub assignee_type: String,
    pub days_offset: i32,
    pub is_required: bool,
}
