//! Fault handlers (UC-03, Epic 4).
//!
//! Implements fault reporting, tracking, resolution workflow, and statistics.
//! Provides business logic for fault lifecycle management including creation,
//! triage, assignment, status updates, resolution, and confirmation.

use crate::state::AppState;
use common::{errors::ErrorResponse, TenantContext};
use db::models::{
    AddFaultComment, AddWorkNote, AssignFault, ConfirmFault, CreateFault, CreateFaultAttachment,
    Fault, FaultAttachment, FaultListQuery, FaultStatistics, FaultSummary,
    FaultTimelineEntryWithUser, FaultWithDetails, ReopenFault, ResolveFault, TriageFault,
    UpdateFault, UpdateFaultStatus,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// Error Types
// ============================================================================

/// Fault handler errors.
#[derive(Debug, Error)]
pub enum FaultHandlerError {
    #[error("Fault not found")]
    FaultNotFound,

    #[error("Attachment not found")]
    AttachmentNotFound,

    #[error("Not authorized")]
    NotAuthorized,

    #[error("Invalid status transition: {0}")]
    InvalidStatusTransition(String),

    #[error("Cannot edit fault after triage")]
    CannotEditAfterTriage,

    #[error("Fault already triaged")]
    AlreadyTriaged,

    #[error("Invalid priority: {0}")]
    InvalidPriority(String),

    #[error("Invalid category: {0}")]
    InvalidCategory(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Duplicate request: {0}")]
    DuplicateRequest(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<FaultHandlerError> for ErrorResponse {
    fn from(err: FaultHandlerError) -> Self {
        match err {
            FaultHandlerError::FaultNotFound => ErrorResponse::new("NOT_FOUND", "Fault not found"),
            FaultHandlerError::AttachmentNotFound => {
                ErrorResponse::new("NOT_FOUND", "Attachment not found")
            }
            FaultHandlerError::NotAuthorized => ErrorResponse::new(
                "NOT_AUTHORIZED",
                "You are not authorized to perform this action",
            ),
            FaultHandlerError::InvalidStatusTransition(msg) => {
                ErrorResponse::new("INVALID_STATUS_TRANSITION", msg)
            }
            FaultHandlerError::CannotEditAfterTriage => {
                ErrorResponse::new("CANNOT_EDIT", "Fault cannot be edited after triage")
            }
            FaultHandlerError::AlreadyTriaged => {
                ErrorResponse::new("ALREADY_TRIAGED", "Fault has already been triaged")
            }
            FaultHandlerError::InvalidPriority(msg) => ErrorResponse::new("INVALID_PRIORITY", msg),
            FaultHandlerError::InvalidCategory(msg) => ErrorResponse::new("INVALID_CATEGORY", msg),
            FaultHandlerError::InvalidInput(msg) => ErrorResponse::new("INVALID_INPUT", msg),
            FaultHandlerError::DuplicateRequest(msg) => {
                ErrorResponse::new("DUPLICATE_REQUEST", msg)
            }
            FaultHandlerError::Database(msg) => ErrorResponse::new("DB_ERROR", msg),
            FaultHandlerError::Internal(msg) => ErrorResponse::new("INTERNAL_ERROR", msg),
        }
    }
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Create fault request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateFaultData {
    pub building_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub location_description: Option<String>,
    pub category: String,
    pub priority: Option<String>,
    pub idempotency_key: Option<String>,
}

/// Update fault request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateFaultData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub location_description: Option<String>,
    pub category: Option<String>,
}

/// Triage fault request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct TriageFaultData {
    pub priority: String,
    pub category: Option<String>,
    pub assigned_to: Option<Uuid>,
}

/// Update status request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateStatusData {
    pub status: String,
    pub note: Option<String>,
    pub scheduled_date: Option<chrono::NaiveDate>,
    pub estimated_completion: Option<chrono::NaiveDate>,
}

/// Resolve fault request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ResolveFaultData {
    pub resolution_notes: String,
}

/// Confirm fault request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ConfirmFaultData {
    pub rating: Option<i32>,
    pub feedback: Option<String>,
}

/// Reopen fault request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ReopenFaultData {
    pub reason: String,
}

/// Assign fault request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AssignFaultData {
    pub assigned_to: Uuid,
}

/// Add comment request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AddCommentData {
    pub note: String,
    pub is_internal: bool,
}

/// Add work note request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AddWorkNoteData {
    pub note: String,
}

/// Add attachment request data.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AddAttachmentData {
    pub filename: String,
    pub original_filename: String,
    pub content_type: String,
    pub size_bytes: i32,
    pub storage_url: String,
    pub thumbnail_url: Option<String>,
    pub description: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

/// List faults query parameters.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListFaultsParams {
    pub building_id: Option<Uuid>,
    pub unit_id: Option<Uuid>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub category: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub search: Option<String>,
    pub from_date: Option<chrono::NaiveDate>,
    pub to_date: Option<chrono::NaiveDate>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

/// Fault list response.
#[derive(Debug, Serialize, ToSchema)]
pub struct FaultListResult {
    pub faults: Vec<FaultSummary>,
    pub count: usize,
}

/// Fault detail response.
#[derive(Debug, Serialize, ToSchema)]
pub struct FaultDetailResult {
    pub fault: FaultWithDetails,
    pub timeline: Vec<FaultTimelineEntryWithUser>,
    pub attachments: Vec<FaultAttachment>,
}

/// Fault action response.
#[derive(Debug, Serialize, ToSchema)]
pub struct FaultActionResult {
    pub message: String,
    pub fault: Fault,
}

/// AI suggestion result.
#[derive(Debug, Serialize, ToSchema)]
pub struct AiSuggestionResult {
    pub category: String,
    pub confidence: f64,
    pub priority: Option<String>,
}

// ============================================================================
// Constants
// ============================================================================

const VALID_PRIORITIES: [&str; 4] = ["low", "medium", "high", "urgent"];
const VALID_STATUSES: [&str; 7] = [
    "new",
    "triaged",
    "in_progress",
    "scheduled",
    "resolved",
    "confirmed",
    "closed",
];
const VALID_CATEGORIES: [&str; 10] = [
    "plumbing",
    "electrical",
    "heating",
    "structural",
    "exterior",
    "elevator",
    "common_area",
    "security",
    "cleaning",
    "other",
];

// ============================================================================
// Handler Implementation
// ============================================================================

/// Fault handler providing business logic for fault operations.
pub struct FaultHandler;

impl FaultHandler {
    // ========================================================================
    // Validation Helpers
    // ========================================================================

    /// Validate priority value.
    fn validate_priority(priority: &str) -> Result<(), FaultHandlerError> {
        if !VALID_PRIORITIES.contains(&priority) {
            return Err(FaultHandlerError::InvalidPriority(format!(
                "Invalid priority. Must be one of: {}",
                VALID_PRIORITIES.join(", ")
            )));
        }
        Ok(())
    }

    /// Validate status value.
    fn validate_status(status: &str) -> Result<(), FaultHandlerError> {
        if !VALID_STATUSES.contains(&status) {
            return Err(FaultHandlerError::InvalidStatusTransition(format!(
                "Invalid status. Must be one of: {}",
                VALID_STATUSES.join(", ")
            )));
        }
        Ok(())
    }

    /// Validate category value.
    fn validate_category(category: &str) -> Result<(), FaultHandlerError> {
        if !VALID_CATEGORIES.contains(&category) {
            return Err(FaultHandlerError::InvalidCategory(format!(
                "Invalid category. Must be one of: {}",
                VALID_CATEGORIES.join(", ")
            )));
        }
        Ok(())
    }

    // ========================================================================
    // CRUD Operations
    // ========================================================================

    /// Create a new fault (Story 4.1).
    pub async fn create_fault(
        state: &AppState,
        context: &TenantContext,
        data: CreateFaultData,
    ) -> Result<Fault, FaultHandlerError> {
        // Validate required fields
        if data.title.trim().is_empty() {
            return Err(FaultHandlerError::InvalidInput("Title is required".into()));
        }

        if data.description.trim().is_empty() {
            return Err(FaultHandlerError::InvalidInput(
                "Description is required".into(),
            ));
        }

        // Validate category
        Self::validate_category(&data.category)?;

        // Validate priority if provided
        if let Some(ref priority) = data.priority {
            Self::validate_priority(priority)?;
        }

        // Create fault
        let create_data = CreateFault {
            organization_id: context.tenant_id,
            building_id: data.building_id,
            unit_id: data.unit_id,
            reporter_id: context.user_id,
            title: data.title,
            description: data.description,
            location_description: data.location_description,
            category: data.category,
            priority: data.priority,
            idempotency_key: data.idempotency_key,
        };

        // TODO: Migrate to create_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let fault = state.fault_repo.create(create_data).await.map_err(|e| {
            // Check for idempotency key conflict
            if e.to_string().contains("idempotency") {
                return FaultHandlerError::DuplicateRequest(
                    "A fault with this idempotency key already exists".into(),
                );
            }
            tracing::error!(error = %e, "Failed to create fault");
            FaultHandlerError::Database("Failed to create fault".into())
        })?;

        tracing::info!(
            fault_id = %fault.id,
            building_id = %fault.building_id,
            reporter_id = %fault.reporter_id,
            "Fault created"
        );

        Ok(fault)
    }

    /// List faults with filters (Story 4.3).
    pub async fn list_faults(
        state: &AppState,
        context: &TenantContext,
        params: ListFaultsParams,
    ) -> Result<FaultListResult, FaultHandlerError> {
        let list_query = FaultListQuery {
            building_id: params.building_id,
            unit_id: params.unit_id,
            status: params.status.map(|s| vec![s]),
            priority: params.priority.map(|p| vec![p]),
            category: params.category.map(|c| vec![c]),
            assigned_to: params.assigned_to,
            reporter_id: None,
            search: params.search,
            from_date: params.from_date,
            to_date: params.to_date,
            limit: params.limit,
            offset: params.offset,
            sort_by: params.sort_by,
            sort_order: params.sort_order,
        };

        let faults = state
            .fault_repo
            .list(context.tenant_id, list_query)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to list faults");
                FaultHandlerError::Database("Failed to list faults".into())
            })?;

        let count = faults.len();
        Ok(FaultListResult { faults, count })
    }

    /// List faults reported by the current user (Story 4.5).
    pub async fn list_my_faults(
        state: &AppState,
        context: &TenantContext,
        limit: i64,
        offset: i64,
    ) -> Result<FaultListResult, FaultHandlerError> {
        let faults = state
            .fault_repo
            .list_by_reporter(context.user_id, limit, offset)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to list my faults");
                FaultHandlerError::Database("Failed to list faults".into())
            })?;

        let count = faults.len();
        Ok(FaultListResult { faults, count })
    }

    /// Get fault details.
    pub async fn get_fault(
        state: &AppState,
        context: &TenantContext,
        fault_id: Uuid,
    ) -> Result<FaultDetailResult, FaultHandlerError> {
        let is_manager = context.role.is_manager();

        let fault = state
            .fault_repo
            .find_by_id_with_details(fault_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to get fault");
                FaultHandlerError::Database("Failed to get fault".into())
            })?
            .ok_or(FaultHandlerError::FaultNotFound)?;

        let timeline = state
            .fault_repo
            .list_timeline(fault_id, is_manager)
            .await
            .unwrap_or_default();

        let attachments = state
            .fault_repo
            .list_attachments(fault_id)
            .await
            .unwrap_or_default();

        Ok(FaultDetailResult {
            fault,
            timeline,
            attachments,
        })
    }

    /// Update fault details.
    pub async fn update_fault(
        state: &AppState,
        fault_id: Uuid,
        data: UpdateFaultData,
    ) -> Result<Fault, FaultHandlerError> {
        // Get existing fault
        // TODO: Migrate to find_by_id_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let existing = state
            .fault_repo
            .find_by_id(fault_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to find fault");
                FaultHandlerError::Database("Failed to find fault".into())
            })?
            .ok_or(FaultHandlerError::FaultNotFound)?;

        // Check if fault can be edited
        if !existing.can_reporter_edit() {
            return Err(FaultHandlerError::CannotEditAfterTriage);
        }

        // Validate category if provided
        if let Some(ref category) = data.category {
            Self::validate_category(category)?;
        }

        let update_data = UpdateFault {
            title: data.title,
            description: data.description,
            location_description: data.location_description,
            category: data.category,
        };

        // TODO: Migrate to update_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let fault = state
            .fault_repo
            .update(fault_id, update_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to update fault");
                FaultHandlerError::Database("Failed to update fault".into())
            })?;

        tracing::info!(fault_id = %fault_id, "Fault updated");

        Ok(fault)
    }

    // ========================================================================
    // Workflow Operations
    // ========================================================================

    /// Triage a fault (Story 4.3).
    pub async fn triage_fault(
        state: &AppState,
        context: &TenantContext,
        fault_id: Uuid,
        data: TriageFaultData,
    ) -> Result<Fault, FaultHandlerError> {
        // Get existing fault
        // TODO: Migrate to find_by_id_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let existing = state
            .fault_repo
            .find_by_id(fault_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to find fault");
                FaultHandlerError::Database("Failed to find fault".into())
            })?
            .ok_or(FaultHandlerError::FaultNotFound)?;

        // Check if fault can be triaged
        if existing.status != "new" {
            return Err(FaultHandlerError::AlreadyTriaged);
        }

        // Validate priority
        Self::validate_priority(&data.priority)?;

        // Validate category if provided
        if let Some(ref category) = data.category {
            Self::validate_category(category)?;
        }

        let triage_data = TriageFault {
            priority: data.priority,
            category: data.category,
            assigned_to: data.assigned_to,
        };

        let fault = state
            .fault_repo
            .triage(fault_id, context.user_id, triage_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to triage fault");
                FaultHandlerError::Database("Failed to triage fault".into())
            })?;

        tracing::info!(
            fault_id = %fault_id,
            triaged_by = %context.user_id,
            priority = %fault.priority,
            "Fault triaged"
        );

        Ok(fault)
    }

    /// Assign a fault.
    pub async fn assign_fault(
        state: &AppState,
        context: &TenantContext,
        fault_id: Uuid,
        data: AssignFaultData,
    ) -> Result<Fault, FaultHandlerError> {
        let assign_data = AssignFault {
            assigned_to: data.assigned_to,
        };

        let fault = state
            .fault_repo
            .assign(fault_id, context.user_id, assign_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to assign fault");
                FaultHandlerError::Database("Failed to assign fault".into())
            })?;

        tracing::info!(
            fault_id = %fault_id,
            assigned_to = %data.assigned_to,
            assigned_by = %context.user_id,
            "Fault assigned"
        );

        Ok(fault)
    }

    /// Update fault status (Story 4.4).
    pub async fn update_status(
        state: &AppState,
        context: &TenantContext,
        fault_id: Uuid,
        data: UpdateStatusData,
    ) -> Result<Fault, FaultHandlerError> {
        // Validate status
        Self::validate_status(&data.status)?;

        let status_data = UpdateFaultStatus {
            status: data.status,
            note: data.note,
            scheduled_date: data.scheduled_date,
            estimated_completion: data.estimated_completion,
        };

        // TODO: Migrate to update_status_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let fault = state
            .fault_repo
            .update_status(fault_id, context.user_id, status_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to update status");
                FaultHandlerError::Database("Failed to update status".into())
            })?;

        tracing::info!(
            fault_id = %fault_id,
            new_status = %fault.status,
            updated_by = %context.user_id,
            "Fault status updated"
        );

        Ok(fault)
    }

    /// Resolve a fault (Story 4.4).
    pub async fn resolve_fault(
        state: &AppState,
        context: &TenantContext,
        fault_id: Uuid,
        data: ResolveFaultData,
    ) -> Result<Fault, FaultHandlerError> {
        // Validate resolution notes
        if data.resolution_notes.trim().is_empty() {
            return Err(FaultHandlerError::InvalidInput(
                "Resolution notes are required".into(),
            ));
        }

        let resolve_data = ResolveFault {
            resolution_notes: data.resolution_notes,
        };

        let fault = state
            .fault_repo
            .resolve(fault_id, context.user_id, resolve_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to resolve fault");
                FaultHandlerError::Database("Failed to resolve fault".into())
            })?;

        tracing::info!(
            fault_id = %fault_id,
            resolved_by = %context.user_id,
            "Fault resolved"
        );

        Ok(fault)
    }

    /// Confirm fault resolution (Story 4.6).
    pub async fn confirm_fault(
        state: &AppState,
        context: &TenantContext,
        fault_id: Uuid,
        data: ConfirmFaultData,
    ) -> Result<Fault, FaultHandlerError> {
        // Validate rating if provided
        if let Some(rating) = data.rating {
            if !(1..=5).contains(&rating) {
                return Err(FaultHandlerError::InvalidInput(
                    "Rating must be between 1 and 5".into(),
                ));
            }
        }

        let confirm_data = ConfirmFault {
            rating: data.rating,
            feedback: data.feedback,
        };

        let fault = state
            .fault_repo
            .confirm(fault_id, context.user_id, confirm_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to confirm fault");
                FaultHandlerError::Database("Failed to confirm fault".into())
            })?;

        tracing::info!(
            fault_id = %fault_id,
            confirmed_by = %context.user_id,
            rating = ?data.rating,
            "Fault resolution confirmed"
        );

        Ok(fault)
    }

    /// Reopen a fault (Story 4.6).
    pub async fn reopen_fault(
        state: &AppState,
        context: &TenantContext,
        fault_id: Uuid,
        data: ReopenFaultData,
    ) -> Result<Fault, FaultHandlerError> {
        // Validate reason
        if data.reason.trim().is_empty() {
            return Err(FaultHandlerError::InvalidInput(
                "Reason for reopening is required".into(),
            ));
        }

        let reopen_data = ReopenFault {
            reason: data.reason,
        };

        let fault = state
            .fault_repo
            .reopen(fault_id, context.user_id, reopen_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to reopen fault");
                FaultHandlerError::Database("Failed to reopen fault".into())
            })?;

        tracing::info!(
            fault_id = %fault_id,
            reopened_by = %context.user_id,
            "Fault reopened"
        );

        Ok(fault)
    }

    // ========================================================================
    // Comments and Notes
    // ========================================================================

    /// List timeline entries (comments and notes).
    pub async fn list_timeline(
        state: &AppState,
        context: &TenantContext,
        fault_id: Uuid,
    ) -> Result<Vec<FaultTimelineEntryWithUser>, FaultHandlerError> {
        let is_manager = context.role.is_manager();

        let entries = state
            .fault_repo
            .list_timeline(fault_id, is_manager)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to list timeline");
                FaultHandlerError::Database("Failed to list timeline".into())
            })?;

        Ok(entries)
    }

    /// Add a comment to a fault.
    pub async fn add_comment(
        state: &AppState,
        context: &TenantContext,
        fault_id: Uuid,
        data: AddCommentData,
    ) -> Result<(), FaultHandlerError> {
        // Validate comment
        if data.note.trim().is_empty() {
            return Err(FaultHandlerError::InvalidInput(
                "Comment cannot be empty".into(),
            ));
        }

        let comment_data = AddFaultComment {
            note: data.note,
            is_internal: data.is_internal,
        };

        state
            .fault_repo
            .add_comment(fault_id, context.user_id, comment_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to add comment");
                FaultHandlerError::Database("Failed to add comment".into())
            })?;

        tracing::info!(
            fault_id = %fault_id,
            user_id = %context.user_id,
            is_internal = data.is_internal,
            "Comment added"
        );

        Ok(())
    }

    /// Add a work note to a fault.
    pub async fn add_work_note(
        state: &AppState,
        context: &TenantContext,
        fault_id: Uuid,
        data: AddWorkNoteData,
    ) -> Result<(), FaultHandlerError> {
        // Validate note
        if data.note.trim().is_empty() {
            return Err(FaultHandlerError::InvalidInput(
                "Work note cannot be empty".into(),
            ));
        }

        let note_data = AddWorkNote { note: data.note };

        state
            .fault_repo
            .add_work_note(fault_id, context.user_id, note_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to add work note");
                FaultHandlerError::Database("Failed to add work note".into())
            })?;

        tracing::info!(
            fault_id = %fault_id,
            user_id = %context.user_id,
            "Work note added"
        );

        Ok(())
    }

    // ========================================================================
    // Attachments
    // ========================================================================

    /// List attachments for a fault.
    pub async fn list_attachments(
        state: &AppState,
        fault_id: Uuid,
    ) -> Result<Vec<FaultAttachment>, FaultHandlerError> {
        let attachments = state
            .fault_repo
            .list_attachments(fault_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to list attachments");
                FaultHandlerError::Database("Failed to list attachments".into())
            })?;

        Ok(attachments)
    }

    /// Add an attachment to a fault.
    pub async fn add_attachment(
        state: &AppState,
        context: &TenantContext,
        fault_id: Uuid,
        data: AddAttachmentData,
    ) -> Result<FaultAttachment, FaultHandlerError> {
        // Validate required fields
        if data.filename.trim().is_empty() {
            return Err(FaultHandlerError::InvalidInput(
                "Filename is required".into(),
            ));
        }

        if data.storage_url.trim().is_empty() {
            return Err(FaultHandlerError::InvalidInput(
                "Storage URL is required".into(),
            ));
        }

        let attachment_data = CreateFaultAttachment {
            fault_id,
            filename: data.filename.clone(),
            original_filename: data.original_filename,
            content_type: data.content_type,
            size_bytes: data.size_bytes,
            storage_url: data.storage_url,
            thumbnail_url: data.thumbnail_url,
            uploaded_by: context.user_id,
            description: data.description,
            width: data.width,
            height: data.height,
        };

        let attachment = state
            .fault_repo
            .add_attachment(attachment_data)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to add attachment");
                FaultHandlerError::Database("Failed to add attachment".into())
            })?;

        tracing::info!(
            fault_id = %fault_id,
            attachment_id = %attachment.id,
            filename = %data.filename,
            "Attachment added"
        );

        Ok(attachment)
    }

    /// Delete an attachment.
    pub async fn delete_attachment(
        state: &AppState,
        attachment_id: Uuid,
    ) -> Result<(), FaultHandlerError> {
        state
            .fault_repo
            .delete_attachment(attachment_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to delete attachment");
                FaultHandlerError::Database("Failed to delete attachment".into())
            })?;

        tracing::info!(attachment_id = %attachment_id, "Attachment deleted");

        Ok(())
    }

    // ========================================================================
    // AI Suggestions
    // ========================================================================

    /// Get AI suggestion for a fault (Story 4.2).
    pub async fn get_ai_suggestion(
        state: &AppState,
        fault_id: Uuid,
    ) -> Result<AiSuggestionResult, FaultHandlerError> {
        // Get fault to analyze
        // TODO: Migrate to find_by_id_rls when route handlers pass RLS connection
        #[allow(deprecated)]
        let fault = state
            .fault_repo
            .find_by_id(fault_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to get fault");
                FaultHandlerError::Database("Failed to get fault".into())
            })?
            .ok_or(FaultHandlerError::FaultNotFound)?;

        // Simple keyword-based suggestion (real ML in Phase 3)
        let description_lower = fault.description.to_lowercase();
        let title_lower = fault.title.to_lowercase();
        let combined = format!("{} {}", title_lower, description_lower);

        let (category, confidence) = Self::categorize_fault(&combined);
        let priority = Self::suggest_priority(&combined);

        // Update fault with AI suggestion
        let _ = state
            .fault_repo
            .update_ai_suggestion(fault_id, &category, priority.as_deref(), confidence)
            .await;

        Ok(AiSuggestionResult {
            category,
            confidence,
            priority,
        })
    }

    /// Categorize fault based on text content.
    fn categorize_fault(text: &str) -> (String, f64) {
        if text.contains("water")
            || text.contains("pipe")
            || text.contains("leak")
            || text.contains("faucet")
            || text.contains("drain")
            || text.contains("toilet")
        {
            ("plumbing".to_string(), 0.85)
        } else if text.contains("electric")
            || text.contains("power")
            || text.contains("outlet")
            || text.contains("light")
            || text.contains("switch")
            || text.contains("wire")
        {
            ("electrical".to_string(), 0.82)
        } else if text.contains("heat")
            || text.contains("cold")
            || text.contains("radiator")
            || text.contains("thermostat")
            || text.contains("boiler")
            || text.contains("furnace")
        {
            ("heating".to_string(), 0.80)
        } else if text.contains("crack")
            || text.contains("wall")
            || text.contains("foundation")
            || text.contains("ceiling")
            || text.contains("floor")
            || text.contains("structural")
        {
            ("structural".to_string(), 0.75)
        } else if text.contains("roof")
            || text.contains("window")
            || text.contains("door")
            || text.contains("facade")
            || text.contains("balcony")
            || text.contains("exterior")
        {
            ("exterior".to_string(), 0.78)
        } else if text.contains("elevator") || text.contains("lift") {
            ("elevator".to_string(), 0.90)
        } else if text.contains("hallway")
            || text.contains("lobby")
            || text.contains("staircase")
            || text.contains("common")
            || text.contains("garage")
            || text.contains("parking")
        {
            ("common_area".to_string(), 0.70)
        } else if text.contains("security")
            || text.contains("lock")
            || text.contains("key")
            || text.contains("intercom")
            || text.contains("camera")
        {
            ("security".to_string(), 0.75)
        } else if text.contains("clean")
            || text.contains("trash")
            || text.contains("garbage")
            || text.contains("dirty")
        {
            ("cleaning".to_string(), 0.72)
        } else {
            ("other".to_string(), 0.50)
        }
    }

    /// Suggest priority based on text content.
    fn suggest_priority(text: &str) -> Option<String> {
        if text.contains("urgent")
            || text.contains("emergency")
            || text.contains("dangerous")
            || text.contains("flood")
            || text.contains("fire")
        {
            Some("urgent".to_string())
        } else if text.contains("broken") || text.contains("not working") {
            Some("high".to_string())
        } else {
            None
        }
    }

    // ========================================================================
    // Statistics
    // ========================================================================

    /// Get fault statistics (Story 4.7).
    pub async fn get_statistics(
        state: &AppState,
        context: &TenantContext,
        building_id: Option<Uuid>,
    ) -> Result<FaultStatistics, FaultHandlerError> {
        let statistics = state
            .fault_repo
            .get_statistics(context.tenant_id, building_id)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to get statistics");
                FaultHandlerError::Database("Failed to get statistics".into())
            })?;

        Ok(statistics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        let err = FaultHandlerError::FaultNotFound;
        let response: ErrorResponse = err.into();
        assert_eq!(response.code, "NOT_FOUND");
    }

    #[test]
    fn test_categorize_fault_plumbing() {
        let (category, confidence) = FaultHandler::categorize_fault("water leak in bathroom");
        assert_eq!(category, "plumbing");
        assert!(confidence > 0.8);
    }

    #[test]
    fn test_categorize_fault_electrical() {
        let (category, confidence) = FaultHandler::categorize_fault("power outlet not working");
        assert_eq!(category, "electrical");
        assert!(confidence > 0.8);
    }

    #[test]
    fn test_suggest_priority_urgent() {
        let priority = FaultHandler::suggest_priority("emergency water flood");
        assert_eq!(priority, Some("urgent".to_string()));
    }

    #[test]
    fn test_suggest_priority_high() {
        let priority = FaultHandler::suggest_priority("broken window");
        assert_eq!(priority, Some("high".to_string()));
    }

    #[test]
    fn test_suggest_priority_none() {
        let priority = FaultHandler::suggest_priority("minor paint issue");
        assert_eq!(priority, None);
    }
}
