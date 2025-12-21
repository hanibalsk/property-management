//! Fault model (Epic 4: Fault Reporting & Resolution).

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Fault category enum values.
pub mod fault_category {
    pub const PLUMBING: &str = "plumbing";
    pub const ELECTRICAL: &str = "electrical";
    pub const HEATING: &str = "heating";
    pub const STRUCTURAL: &str = "structural";
    pub const EXTERIOR: &str = "exterior";
    pub const ELEVATOR: &str = "elevator";
    pub const COMMON_AREA: &str = "common_area";
    pub const SECURITY: &str = "security";
    pub const CLEANING: &str = "cleaning";
    pub const OTHER: &str = "other";

    pub const ALL: &[&str] = &[
        PLUMBING,
        ELECTRICAL,
        HEATING,
        STRUCTURAL,
        EXTERIOR,
        ELEVATOR,
        COMMON_AREA,
        SECURITY,
        CLEANING,
        OTHER,
    ];
}

/// Fault status enum values.
pub mod fault_status {
    pub const NEW: &str = "new";
    pub const TRIAGED: &str = "triaged";
    pub const IN_PROGRESS: &str = "in_progress";
    pub const WAITING_PARTS: &str = "waiting_parts";
    pub const SCHEDULED: &str = "scheduled";
    pub const RESOLVED: &str = "resolved";
    pub const CLOSED: &str = "closed";
    pub const REOPENED: &str = "reopened";

    pub const ALL: &[&str] = &[
        NEW,
        TRIAGED,
        IN_PROGRESS,
        WAITING_PARTS,
        SCHEDULED,
        RESOLVED,
        CLOSED,
        REOPENED,
    ];
}

/// Fault priority enum values.
pub mod fault_priority {
    pub const LOW: &str = "low";
    pub const MEDIUM: &str = "medium";
    pub const HIGH: &str = "high";
    pub const URGENT: &str = "urgent";

    pub const ALL: &[&str] = &[LOW, MEDIUM, HIGH, URGENT];
}

/// Timeline action enum values.
pub mod timeline_action {
    pub const CREATED: &str = "created";
    pub const TRIAGED: &str = "triaged";
    pub const ASSIGNED: &str = "assigned";
    pub const STATUS_CHANGED: &str = "status_changed";
    pub const PRIORITY_CHANGED: &str = "priority_changed";
    pub const WORK_NOTE: &str = "work_note";
    pub const COMMENT: &str = "comment";
    pub const ATTACHMENT_ADDED: &str = "attachment_added";
    pub const SCHEDULED: &str = "scheduled";
    pub const RESOLVED: &str = "resolved";
    pub const CONFIRMED: &str = "confirmed";
    pub const REOPENED: &str = "reopened";
    pub const RATED: &str = "rated";
}

/// Fault entity from database.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Fault {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub reporter_id: Uuid,
    pub title: String,
    pub description: String,
    pub location_description: Option<String>,
    pub category: String,
    pub priority: String,
    pub status: String,
    pub ai_category: Option<String>,
    pub ai_priority: Option<String>,
    pub ai_confidence: Option<rust_decimal::Decimal>,
    pub ai_processed_at: Option<DateTime<Utc>>,
    pub assigned_to: Option<Uuid>,
    pub assigned_at: Option<DateTime<Utc>>,
    pub triaged_by: Option<Uuid>,
    pub triaged_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<Uuid>,
    pub resolution_notes: Option<String>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub confirmed_by: Option<Uuid>,
    pub rating: Option<i32>,
    pub feedback: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub estimated_completion: Option<NaiveDate>,
    pub idempotency_key: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Fault {
    /// Check if fault is open (not closed).
    pub fn is_open(&self) -> bool {
        self.status != fault_status::CLOSED
    }

    /// Check if fault can be edited by reporter.
    pub fn can_reporter_edit(&self) -> bool {
        self.status == fault_status::NEW
    }

    /// Check if fault is awaiting confirmation.
    pub fn is_awaiting_confirmation(&self) -> bool {
        self.status == fault_status::RESOLVED
    }

    /// Get status display name.
    pub fn status_display(&self) -> &str {
        match self.status.as_str() {
            "new" => "New",
            "triaged" => "Triaged",
            "in_progress" => "In Progress",
            "waiting_parts" => "Waiting for Parts",
            "scheduled" => "Scheduled",
            "resolved" => "Resolved",
            "closed" => "Closed",
            "reopened" => "Reopened",
            _ => &self.status,
        }
    }

    /// Get priority display name.
    pub fn priority_display(&self) -> &str {
        match self.priority.as_str() {
            "low" => "Low",
            "medium" => "Medium",
            "high" => "High",
            "urgent" => "Urgent",
            _ => &self.priority,
        }
    }

    /// Get category display name.
    pub fn category_display(&self) -> &str {
        match self.category.as_str() {
            "plumbing" => "Plumbing",
            "electrical" => "Electrical",
            "heating" => "Heating",
            "structural" => "Structural",
            "exterior" => "Exterior",
            "elevator" => "Elevator",
            "common_area" => "Common Area",
            "security" => "Security",
            "cleaning" => "Cleaning",
            "other" => "Other",
            _ => &self.category,
        }
    }
}

/// Summary view of a fault for lists.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct FaultSummary {
    pub id: Uuid,
    pub building_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub title: String,
    pub category: String,
    pub priority: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

/// Fault with additional display information.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FaultWithDetails {
    #[serde(flatten)]
    pub fault: Fault,
    pub reporter_name: String,
    pub reporter_email: String,
    pub building_name: String,
    pub building_address: String,
    pub unit_designation: Option<String>,
    pub assigned_to_name: Option<String>,
    pub attachment_count: i64,
    pub comment_count: i64,
}

/// Data for creating a new fault.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateFault {
    pub organization_id: Uuid,
    pub building_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub reporter_id: Uuid,
    pub title: String,
    pub description: String,
    pub location_description: Option<String>,
    pub category: String,
    pub priority: Option<String>,
    pub idempotency_key: Option<String>,
}

impl CreateFault {
    /// Validate the create request.
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.title.is_empty() {
            return Err("Title is required");
        }
        if self.title.len() > 255 {
            return Err("Title must be 255 characters or less");
        }
        if self.description.is_empty() {
            return Err("Description is required");
        }
        if !fault_category::ALL.contains(&self.category.as_str()) {
            return Err("Invalid category");
        }
        if let Some(ref priority) = self.priority {
            if !fault_priority::ALL.contains(&priority.as_str()) {
                return Err("Invalid priority");
            }
        }
        Ok(())
    }
}

/// Data for updating a fault.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateFault {
    pub title: Option<String>,
    pub description: Option<String>,
    pub location_description: Option<String>,
    pub category: Option<String>,
}

/// Data for triaging a fault.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TriageFault {
    pub priority: String,
    pub category: Option<String>,
    pub assigned_to: Option<Uuid>,
}

impl TriageFault {
    /// Validate triage request.
    pub fn validate(&self) -> Result<(), &'static str> {
        if !fault_priority::ALL.contains(&self.priority.as_str()) {
            return Err("Invalid priority");
        }
        if let Some(ref category) = self.category {
            if !fault_category::ALL.contains(&category.as_str()) {
                return Err("Invalid category");
            }
        }
        Ok(())
    }
}

/// Data for updating fault status.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateFaultStatus {
    pub status: String,
    pub note: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub estimated_completion: Option<NaiveDate>,
}

impl UpdateFaultStatus {
    /// Validate status update.
    pub fn validate(&self) -> Result<(), &'static str> {
        if !fault_status::ALL.contains(&self.status.as_str()) {
            return Err("Invalid status");
        }
        Ok(())
    }
}

/// Data for resolving a fault.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResolveFault {
    pub resolution_notes: String,
}

/// Data for confirming fault resolution.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ConfirmFault {
    pub rating: Option<i32>,
    pub feedback: Option<String>,
}

impl ConfirmFault {
    /// Validate confirmation.
    pub fn validate(&self) -> Result<(), &'static str> {
        if let Some(rating) = self.rating {
            if !(1..=5).contains(&rating) {
                return Err("Rating must be between 1 and 5");
            }
        }
        Ok(())
    }
}

/// Data for reopening a fault.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReopenFault {
    pub reason: String,
}

/// Data for assigning a fault.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AssignFault {
    pub assigned_to: Uuid,
}

/// AI suggestion result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AiSuggestion {
    pub category: String,
    pub confidence: f64,
    pub priority: Option<String>,
}

// ============================================================================
// Fault Attachment
// ============================================================================

/// Fault attachment entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct FaultAttachment {
    pub id: Uuid,
    pub fault_id: Uuid,
    pub filename: String,
    pub original_filename: String,
    pub content_type: String,
    pub size_bytes: i32,
    pub storage_url: String,
    pub thumbnail_url: Option<String>,
    pub uploaded_by: Uuid,
    pub description: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub created_at: DateTime<Utc>,
}

impl FaultAttachment {
    /// Check if attachment is an image.
    pub fn is_image(&self) -> bool {
        self.content_type.starts_with("image/")
    }
}

/// Data for creating an attachment.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateFaultAttachment {
    pub fault_id: Uuid,
    pub filename: String,
    pub original_filename: String,
    pub content_type: String,
    pub size_bytes: i32,
    pub storage_url: String,
    pub thumbnail_url: Option<String>,
    pub uploaded_by: Uuid,
    pub description: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

// ============================================================================
// Fault Timeline
// ============================================================================

/// Fault timeline entry entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct FaultTimelineEntry {
    pub id: Uuid,
    pub fault_id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub note: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub metadata: serde_json::Value,
    pub is_internal: bool,
    pub created_at: DateTime<Utc>,
}

/// Timeline entry with user info.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FaultTimelineEntryWithUser {
    #[serde(flatten)]
    pub entry: FaultTimelineEntry,
    pub user_name: String,
    pub user_email: String,
}

/// Data for creating a timeline entry.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateFaultTimelineEntry {
    pub fault_id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub note: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub is_internal: bool,
}

/// Data for adding a comment.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AddFaultComment {
    pub note: String,
    pub is_internal: bool,
}

/// Data for adding a work note.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AddWorkNote {
    pub note: String,
}

// ============================================================================
// Query/Filter types
// ============================================================================

/// Fault list query parameters.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct FaultListQuery {
    pub building_id: Option<Uuid>,
    pub unit_id: Option<Uuid>,
    pub status: Option<Vec<String>>,
    pub priority: Option<Vec<String>>,
    pub category: Option<Vec<String>>,
    pub assigned_to: Option<Uuid>,
    pub reporter_id: Option<Uuid>,
    pub search: Option<String>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

/// Fault statistics for reporting.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FaultStatistics {
    pub total_count: i64,
    pub open_count: i64,
    pub closed_count: i64,
    pub by_status: Vec<StatusCount>,
    pub by_category: Vec<CategoryCount>,
    pub by_priority: Vec<PriorityCount>,
    pub average_resolution_time_hours: Option<f64>,
    pub average_rating: Option<f64>,
}

/// Count by status.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct StatusCount {
    pub status: String,
    pub count: i64,
}

/// Count by category.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct CategoryCount {
    pub category: String,
    pub count: i64,
}

/// Count by priority.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct PriorityCount {
    pub priority: String,
    pub count: i64,
}
