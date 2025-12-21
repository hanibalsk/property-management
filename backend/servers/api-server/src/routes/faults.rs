//! Fault routes (Epic 4: Fault Reporting & Resolution).

use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use common::errors::ErrorResponse;
use db::models::{
    fault_category, fault_priority, fault_status, AddFaultComment, AddWorkNote, AiSuggestion,
    AssignFault, ConfirmFault, CreateFault, CreateFaultAttachment, Fault, FaultAttachment,
    FaultListQuery, FaultStatistics, FaultSummary, FaultTimelineEntryWithUser, FaultWithDetails,
    ReopenFault, ResolveFault, TriageFault, UpdateFault, UpdateFaultStatus,
};
use serde::{Deserialize, Serialize};
use sqlx::Error as SqlxError;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// Response Types
// ============================================================================

/// Response for fault creation.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateFaultResponse {
    pub id: Uuid,
    pub message: String,
}

/// Response for fault list.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FaultListResponse {
    pub faults: Vec<FaultSummary>,
    pub count: usize,
}

/// Response for fault details.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FaultDetailResponse {
    pub fault: FaultWithDetails,
    pub timeline: Vec<FaultTimelineEntryWithUser>,
    pub attachments: Vec<FaultAttachment>,
}

/// Response for generic fault action.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FaultActionResponse {
    pub message: String,
    pub fault: Fault,
}

/// Response for timeline list.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TimelineResponse {
    pub entries: Vec<FaultTimelineEntryWithUser>,
}

/// Response for attachments list.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AttachmentsResponse {
    pub attachments: Vec<FaultAttachment>,
}

/// Response for AI suggestion.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AiSuggestionResponse {
    pub suggestion: AiSuggestion,
}

/// Response for statistics.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StatisticsResponse {
    pub statistics: FaultStatistics,
}

// ============================================================================
// Request Types
// ============================================================================

/// Request for creating a fault.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateFaultRequest {
    pub building_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub location_description: Option<String>,
    pub category: String,
    pub priority: Option<String>,
    pub idempotency_key: Option<String>,
}

/// Request for updating a fault.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateFaultRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub location_description: Option<String>,
    pub category: Option<String>,
}

/// Request for triaging a fault.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TriageFaultRequest {
    pub priority: String,
    pub category: Option<String>,
    pub assigned_to: Option<Uuid>,
}

/// Request for updating fault status.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateStatusRequest {
    pub status: String,
    pub note: Option<String>,
    pub scheduled_date: Option<chrono::NaiveDate>,
    pub estimated_completion: Option<chrono::NaiveDate>,
}

/// Request for resolving a fault.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResolveFaultRequest {
    pub resolution_notes: String,
}

/// Request for confirming fault resolution.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConfirmFaultRequest {
    pub rating: Option<i32>,
    pub feedback: Option<String>,
}

/// Request for reopening a fault.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReopenFaultRequest {
    pub reason: String,
}

/// Request for assigning a fault.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AssignFaultRequest {
    pub assigned_to: Uuid,
}

/// Request for adding a comment.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddCommentRequest {
    pub note: String,
    #[serde(default)]
    pub is_internal: bool,
}

/// Request for adding a work note.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddWorkNoteRequest {
    pub note: String,
}

/// Request for adding an attachment.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddAttachmentRequest {
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

/// Query for listing faults.
#[derive(Debug, Serialize, Deserialize, ToSchema, Default, utoipa::IntoParams)]
pub struct ListFaultsQuery {
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

/// Query for statistics.
#[derive(Debug, Serialize, Deserialize, ToSchema, Default, utoipa::IntoParams)]
pub struct StatisticsQuery {
    pub building_id: Option<Uuid>,
}

// ============================================================================
// Router
// ============================================================================

/// Create faults router.
pub fn router() -> Router<AppState> {
    Router::new()
        // CRUD
        .route("/", post(create_fault))
        .route("/", get(list_faults))
        .route("/my", get(list_my_faults))
        .route("/{id}", get(get_fault))
        .route("/{id}", put(update_fault))
        // Workflow
        .route("/{id}/triage", post(triage_fault))
        .route("/{id}/assign", post(assign_fault))
        .route("/{id}/status", put(update_status))
        .route("/{id}/resolve", post(resolve_fault))
        .route("/{id}/confirm", post(confirm_fault))
        .route("/{id}/reopen", post(reopen_fault))
        // Comments & Notes
        .route("/{id}/comments", get(list_comments))
        .route("/{id}/comments", post(add_comment))
        .route("/{id}/work-notes", post(add_work_note))
        // Attachments
        .route("/{id}/attachments", get(list_attachments))
        .route("/{id}/attachments", post(add_attachment))
        .route(
            "/{id}/attachments/{attachment_id}",
            delete(delete_attachment),
        )
        // AI
        .route("/{id}/suggest", post(get_ai_suggestion))
        // Statistics
        .route("/statistics", get(get_statistics))
}

// ============================================================================
// Handlers
// ============================================================================

/// Create a new fault (Story 4.1).
#[utoipa::path(
    post,
    path = "/api/v1/faults",
    request_body = CreateFaultRequest,
    responses(
        (status = 201, description = "Fault created", body = CreateFaultResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "Faults"
)]
async fn create_fault(
    State(state): State<AppState>,
    Json(req): Json<CreateFaultRequest>,
) -> Result<(StatusCode, Json<CreateFaultResponse>), (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let reporter_id = Uuid::nil();
    let org_id = Uuid::nil();

    // Check for idempotency
    if let Some(ref key) = req.idempotency_key {
        if let Ok(Some(existing)) = state.fault_repo.find_by_idempotency_key(key).await {
            return Ok((
                StatusCode::OK,
                Json(CreateFaultResponse {
                    id: existing.id,
                    message: "Fault already exists (idempotent)".to_string(),
                }),
            ));
        }
    }

    // Validate category
    if !fault_category::ALL.contains(&req.category.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new("BAD_REQUEST", "Invalid category")),
        ));
    }

    // Validate priority if provided
    if let Some(ref priority) = req.priority {
        if !fault_priority::ALL.contains(&priority.as_str()) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new("BAD_REQUEST", "Invalid priority")),
            ));
        }
    }

    let data = CreateFault {
        organization_id: org_id,
        building_id: req.building_id,
        unit_id: req.unit_id,
        reporter_id,
        title: req.title,
        description: req.description,
        location_description: req.location_description,
        category: req.category,
        priority: req.priority,
        idempotency_key: req.idempotency_key,
    };

    match state.fault_repo.create(data).await {
        Ok(fault) => Ok((
            StatusCode::CREATED,
            Json(CreateFaultResponse {
                id: fault.id,
                message: "Fault created successfully".to_string(),
            }),
        )),
        Err(e) => {
            tracing::error!("Failed to create fault: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to create fault",
                )),
            ))
        }
    }
}

/// List faults with filters (Story 4.3).
#[utoipa::path(
    get,
    path = "/api/v1/faults",
    params(ListFaultsQuery),
    responses(
        (status = 200, description = "Fault list", body = FaultListResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "Faults"
)]
async fn list_faults(
    State(state): State<AppState>,
    Query(query): Query<ListFaultsQuery>,
) -> Result<Json<FaultListResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let org_id = Uuid::nil();

    let list_query = FaultListQuery {
        building_id: query.building_id,
        unit_id: query.unit_id,
        status: query
            .status
            .map(|s| s.split(',').map(String::from).collect()),
        priority: query
            .priority
            .map(|s| s.split(',').map(String::from).collect()),
        category: query
            .category
            .map(|s| s.split(',').map(String::from).collect()),
        assigned_to: query.assigned_to,
        reporter_id: None,
        search: query.search,
        from_date: query.from_date,
        to_date: query.to_date,
        limit: query.limit,
        offset: query.offset,
        sort_by: query.sort_by,
        sort_order: query.sort_order,
    };

    match state.fault_repo.list(org_id, list_query).await {
        Ok(faults) => {
            let count = faults.len();
            Ok(Json(FaultListResponse { faults, count }))
        }
        Err(e) => {
            tracing::error!("Failed to list faults: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to list faults",
                )),
            ))
        }
    }
}

/// List my faults (Story 4.5).
#[utoipa::path(
    get,
    path = "/api/v1/faults/my",
    responses(
        (status = 200, description = "My fault list", body = FaultListResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "Faults"
)]
async fn list_my_faults(
    State(state): State<AppState>,
    Query(query): Query<ListFaultsQuery>,
) -> Result<Json<FaultListResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let user_id = Uuid::nil();

    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    match state
        .fault_repo
        .list_by_reporter(user_id, limit, offset)
        .await
    {
        Ok(faults) => {
            let count = faults.len();
            Ok(Json(FaultListResponse { faults, count }))
        }
        Err(e) => {
            tracing::error!("Failed to list my faults: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to list faults",
                )),
            ))
        }
    }
}

/// Get fault details.
#[utoipa::path(
    get,
    path = "/api/v1/faults/{id}",
    params(
        ("id" = Uuid, Path, description = "Fault ID")
    ),
    responses(
        (status = 200, description = "Fault details", body = FaultDetailResponse),
        (status = 404, description = "Fault not found", body = ErrorResponse),
    ),
    tag = "Faults"
)]
async fn get_fault(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<FaultDetailResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get user from auth context for internal visibility check
    let is_manager = true; // Placeholder

    let fault = match state.fault_repo.find_by_id_with_details(id).await {
        Ok(Some(f)) => f,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Fault not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to get fault: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to get fault")),
            ));
        }
    };

    let timeline = state
        .fault_repo
        .list_timeline(id, is_manager)
        .await
        .unwrap_or_default();

    let attachments = state
        .fault_repo
        .list_attachments(id)
        .await
        .unwrap_or_default();

    Ok(Json(FaultDetailResponse {
        fault,
        timeline,
        attachments,
    }))
}

/// Update fault details.
#[utoipa::path(
    put,
    path = "/api/v1/faults/{id}",
    params(
        ("id" = Uuid, Path, description = "Fault ID")
    ),
    request_body = UpdateFaultRequest,
    responses(
        (status = 200, description = "Fault updated", body = FaultActionResponse),
        (status = 400, description = "Cannot update", body = ErrorResponse),
        (status = 404, description = "Fault not found", body = ErrorResponse),
    ),
    tag = "Faults"
)]
async fn update_fault(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateFaultRequest>,
) -> Result<Json<FaultActionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Check fault exists and can be edited
    let existing = match state.fault_repo.find_by_id(id).await {
        Ok(Some(f)) => f,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Fault not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find fault: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to find fault")),
            ));
        }
    };

    if !existing.can_reporter_edit() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "BAD_REQUEST",
                "Fault cannot be edited after triage",
            )),
        ));
    }

    let data = UpdateFault {
        title: req.title,
        description: req.description,
        location_description: req.location_description,
        category: req.category,
    };

    match state.fault_repo.update(id, data).await {
        Ok(fault) => Ok(Json(FaultActionResponse {
            message: "Fault updated".to_string(),
            fault,
        })),
        Err(e) => {
            tracing::error!("Failed to update fault: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to update fault",
                )),
            ))
        }
    }
}

/// Triage a fault (Story 4.3).
#[utoipa::path(
    post,
    path = "/api/v1/faults/{id}/triage",
    params(
        ("id" = Uuid, Path, description = "Fault ID")
    ),
    request_body = TriageFaultRequest,
    responses(
        (status = 200, description = "Fault triaged", body = FaultActionResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 404, description = "Fault not found", body = ErrorResponse),
    ),
    tag = "Faults"
)]
async fn triage_fault(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<TriageFaultRequest>,
) -> Result<Json<FaultActionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let user_id = Uuid::nil();

    // Validate priority
    if !fault_priority::ALL.contains(&req.priority.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new("BAD_REQUEST", "Invalid priority")),
        ));
    }

    let data = TriageFault {
        priority: req.priority,
        category: req.category,
        assigned_to: req.assigned_to,
    };

    match state.fault_repo.triage(id, user_id, data).await {
        Ok(fault) => Ok(Json(FaultActionResponse {
            message: "Fault triaged".to_string(),
            fault,
        })),
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Fault not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to triage fault: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to triage fault",
                )),
            ))
        }
    }
}

/// Assign a fault.
#[utoipa::path(
    post,
    path = "/api/v1/faults/{id}/assign",
    params(
        ("id" = Uuid, Path, description = "Fault ID")
    ),
    request_body = AssignFaultRequest,
    responses(
        (status = 200, description = "Fault assigned", body = FaultActionResponse),
        (status = 404, description = "Fault not found", body = ErrorResponse),
    ),
    tag = "Faults"
)]
async fn assign_fault(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<AssignFaultRequest>,
) -> Result<Json<FaultActionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let user_id = Uuid::nil();

    let data = AssignFault {
        assigned_to: req.assigned_to,
    };

    match state.fault_repo.assign(id, user_id, data).await {
        Ok(fault) => Ok(Json(FaultActionResponse {
            message: "Fault assigned".to_string(),
            fault,
        })),
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Fault not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to assign fault: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to assign fault",
                )),
            ))
        }
    }
}

/// Update fault status (Story 4.4).
#[utoipa::path(
    put,
    path = "/api/v1/faults/{id}/status",
    params(
        ("id" = Uuid, Path, description = "Fault ID")
    ),
    request_body = UpdateStatusRequest,
    responses(
        (status = 200, description = "Status updated", body = FaultActionResponse),
        (status = 400, description = "Invalid status", body = ErrorResponse),
        (status = 404, description = "Fault not found", body = ErrorResponse),
    ),
    tag = "Faults"
)]
async fn update_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateStatusRequest>,
) -> Result<Json<FaultActionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let user_id = Uuid::nil();

    // Validate status
    if !fault_status::ALL.contains(&req.status.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new("BAD_REQUEST", "Invalid status")),
        ));
    }

    let data = UpdateFaultStatus {
        status: req.status,
        note: req.note,
        scheduled_date: req.scheduled_date,
        estimated_completion: req.estimated_completion,
    };

    match state.fault_repo.update_status(id, user_id, data).await {
        Ok(fault) => Ok(Json(FaultActionResponse {
            message: "Status updated".to_string(),
            fault,
        })),
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Fault not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to update status: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to update status",
                )),
            ))
        }
    }
}

/// Resolve a fault (Story 4.4).
#[utoipa::path(
    post,
    path = "/api/v1/faults/{id}/resolve",
    params(
        ("id" = Uuid, Path, description = "Fault ID")
    ),
    request_body = ResolveFaultRequest,
    responses(
        (status = 200, description = "Fault resolved", body = FaultActionResponse),
        (status = 404, description = "Fault not found", body = ErrorResponse),
    ),
    tag = "Faults"
)]
async fn resolve_fault(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ResolveFaultRequest>,
) -> Result<Json<FaultActionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let user_id = Uuid::nil();

    let data = ResolveFault {
        resolution_notes: req.resolution_notes,
    };

    match state.fault_repo.resolve(id, user_id, data).await {
        Ok(fault) => Ok(Json(FaultActionResponse {
            message: "Fault resolved".to_string(),
            fault,
        })),
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Fault not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to resolve fault: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to resolve fault",
                )),
            ))
        }
    }
}

/// Confirm fault resolution (Story 4.6).
#[utoipa::path(
    post,
    path = "/api/v1/faults/{id}/confirm",
    params(
        ("id" = Uuid, Path, description = "Fault ID")
    ),
    request_body = ConfirmFaultRequest,
    responses(
        (status = 200, description = "Resolution confirmed", body = FaultActionResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 404, description = "Fault not found", body = ErrorResponse),
    ),
    tag = "Faults"
)]
async fn confirm_fault(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ConfirmFaultRequest>,
) -> Result<Json<FaultActionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let user_id = Uuid::nil();

    // Validate rating
    if let Some(rating) = req.rating {
        if !(1..=5).contains(&rating) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(
                    "BAD_REQUEST",
                    "Rating must be between 1 and 5",
                )),
            ));
        }
    }

    let data = ConfirmFault {
        rating: req.rating,
        feedback: req.feedback,
    };

    match state.fault_repo.confirm(id, user_id, data).await {
        Ok(fault) => Ok(Json(FaultActionResponse {
            message: "Resolution confirmed".to_string(),
            fault,
        })),
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Fault not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to confirm fault: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to confirm resolution",
                )),
            ))
        }
    }
}

/// Reopen a fault (Story 4.6).
#[utoipa::path(
    post,
    path = "/api/v1/faults/{id}/reopen",
    params(
        ("id" = Uuid, Path, description = "Fault ID")
    ),
    request_body = ReopenFaultRequest,
    responses(
        (status = 200, description = "Fault reopened", body = FaultActionResponse),
        (status = 404, description = "Fault not found", body = ErrorResponse),
    ),
    tag = "Faults"
)]
async fn reopen_fault(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ReopenFaultRequest>,
) -> Result<Json<FaultActionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let user_id = Uuid::nil();

    let data = ReopenFault { reason: req.reason };

    match state.fault_repo.reopen(id, user_id, data).await {
        Ok(fault) => Ok(Json(FaultActionResponse {
            message: "Fault reopened".to_string(),
            fault,
        })),
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Fault not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to reopen fault: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to reopen fault",
                )),
            ))
        }
    }
}

/// List comments for a fault.
#[utoipa::path(
    get,
    path = "/api/v1/faults/{id}/comments",
    params(
        ("id" = Uuid, Path, description = "Fault ID")
    ),
    responses(
        (status = 200, description = "Timeline entries", body = TimelineResponse),
    ),
    tag = "Faults"
)]
async fn list_comments(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<TimelineResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Determine if user is manager for internal visibility
    let is_manager = true;

    match state.fault_repo.list_timeline(id, is_manager).await {
        Ok(entries) => Ok(Json(TimelineResponse { entries })),
        Err(e) => {
            tracing::error!("Failed to list timeline: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to list comments",
                )),
            ))
        }
    }
}

/// Add a comment to a fault.
#[utoipa::path(
    post,
    path = "/api/v1/faults/{id}/comments",
    params(
        ("id" = Uuid, Path, description = "Fault ID")
    ),
    request_body = AddCommentRequest,
    responses(
        (status = 201, description = "Comment added"),
        (status = 404, description = "Fault not found", body = ErrorResponse),
    ),
    tag = "Faults"
)]
async fn add_comment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<AddCommentRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let user_id = Uuid::nil();

    let data = AddFaultComment {
        note: req.note,
        is_internal: req.is_internal,
    };

    match state.fault_repo.add_comment(id, user_id, data).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            tracing::error!("Failed to add comment: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to add comment",
                )),
            ))
        }
    }
}

/// Add a work note to a fault.
#[utoipa::path(
    post,
    path = "/api/v1/faults/{id}/work-notes",
    params(
        ("id" = Uuid, Path, description = "Fault ID")
    ),
    request_body = AddWorkNoteRequest,
    responses(
        (status = 201, description = "Work note added"),
        (status = 404, description = "Fault not found", body = ErrorResponse),
    ),
    tag = "Faults"
)]
async fn add_work_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<AddWorkNoteRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let user_id = Uuid::nil();

    let data = AddWorkNote { note: req.note };

    match state.fault_repo.add_work_note(id, user_id, data).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            tracing::error!("Failed to add work note: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to add work note",
                )),
            ))
        }
    }
}

/// List attachments for a fault.
#[utoipa::path(
    get,
    path = "/api/v1/faults/{id}/attachments",
    params(
        ("id" = Uuid, Path, description = "Fault ID")
    ),
    responses(
        (status = 200, description = "Attachments list", body = AttachmentsResponse),
    ),
    tag = "Faults"
)]
async fn list_attachments(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AttachmentsResponse>, (StatusCode, Json<ErrorResponse>)> {
    match state.fault_repo.list_attachments(id).await {
        Ok(attachments) => Ok(Json(AttachmentsResponse { attachments })),
        Err(e) => {
            tracing::error!("Failed to list attachments: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to list attachments",
                )),
            ))
        }
    }
}

/// Add an attachment to a fault.
#[utoipa::path(
    post,
    path = "/api/v1/faults/{id}/attachments",
    params(
        ("id" = Uuid, Path, description = "Fault ID")
    ),
    request_body = AddAttachmentRequest,
    responses(
        (status = 201, description = "Attachment added", body = FaultAttachment),
        (status = 404, description = "Fault not found", body = ErrorResponse),
    ),
    tag = "Faults"
)]
async fn add_attachment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<AddAttachmentRequest>,
) -> Result<(StatusCode, Json<FaultAttachment>), (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let user_id = Uuid::nil();

    let data = CreateFaultAttachment {
        fault_id: id,
        filename: req.filename,
        original_filename: req.original_filename,
        content_type: req.content_type,
        size_bytes: req.size_bytes,
        storage_url: req.storage_url,
        thumbnail_url: req.thumbnail_url,
        uploaded_by: user_id,
        description: req.description,
        width: req.width,
        height: req.height,
    };

    match state.fault_repo.add_attachment(data).await {
        Ok(attachment) => Ok((StatusCode::CREATED, Json(attachment))),
        Err(e) => {
            tracing::error!("Failed to add attachment: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to add attachment",
                )),
            ))
        }
    }
}

/// Delete an attachment.
#[utoipa::path(
    delete,
    path = "/api/v1/faults/{id}/attachments/{attachment_id}",
    params(
        ("id" = Uuid, Path, description = "Fault ID"),
        ("attachment_id" = Uuid, Path, description = "Attachment ID")
    ),
    responses(
        (status = 204, description = "Attachment deleted"),
        (status = 404, description = "Attachment not found", body = ErrorResponse),
    ),
    tag = "Faults"
)]
async fn delete_attachment(
    State(state): State<AppState>,
    Path((_id, attachment_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    match state.fault_repo.delete_attachment(attachment_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            tracing::error!("Failed to delete attachment: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to delete attachment",
                )),
            ))
        }
    }
}

/// Get AI suggestion for a fault (Story 4.2).
#[utoipa::path(
    post,
    path = "/api/v1/faults/{id}/suggest",
    params(
        ("id" = Uuid, Path, description = "Fault ID")
    ),
    responses(
        (status = 200, description = "AI suggestion", body = AiSuggestionResponse),
        (status = 404, description = "Fault not found", body = ErrorResponse),
    ),
    tag = "Faults"
)]
async fn get_ai_suggestion(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AiSuggestionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Get fault to analyze
    let fault = match state.fault_repo.find_by_id(id).await {
        Ok(Some(f)) => f,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Fault not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to get fault: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to get fault")),
            ));
        }
    };

    // Simple keyword-based suggestion (real ML in Phase 3)
    let description_lower = fault.description.to_lowercase();
    let title_lower = fault.title.to_lowercase();
    let combined = format!("{} {}", title_lower, description_lower);

    let (category, confidence) = if combined.contains("water")
        || combined.contains("pipe")
        || combined.contains("leak")
        || combined.contains("faucet")
        || combined.contains("drain")
        || combined.contains("toilet")
    {
        ("plumbing", 0.85)
    } else if combined.contains("electric")
        || combined.contains("power")
        || combined.contains("outlet")
        || combined.contains("light")
        || combined.contains("switch")
        || combined.contains("wire")
    {
        ("electrical", 0.82)
    } else if combined.contains("heat")
        || combined.contains("cold")
        || combined.contains("radiator")
        || combined.contains("thermostat")
        || combined.contains("boiler")
        || combined.contains("furnace")
    {
        ("heating", 0.80)
    } else if combined.contains("crack")
        || combined.contains("wall")
        || combined.contains("foundation")
        || combined.contains("ceiling")
        || combined.contains("floor")
        || combined.contains("structural")
    {
        ("structural", 0.75)
    } else if combined.contains("roof")
        || combined.contains("window")
        || combined.contains("door")
        || combined.contains("facade")
        || combined.contains("balcony")
        || combined.contains("exterior")
    {
        ("exterior", 0.78)
    } else if combined.contains("elevator") || combined.contains("lift") {
        ("elevator", 0.90)
    } else if combined.contains("hallway")
        || combined.contains("lobby")
        || combined.contains("staircase")
        || combined.contains("common")
        || combined.contains("garage")
        || combined.contains("parking")
    {
        ("common_area", 0.70)
    } else if combined.contains("security")
        || combined.contains("lock")
        || combined.contains("key")
        || combined.contains("intercom")
        || combined.contains("camera")
    {
        ("security", 0.75)
    } else if combined.contains("clean")
        || combined.contains("trash")
        || combined.contains("garbage")
        || combined.contains("dirty")
    {
        ("cleaning", 0.72)
    } else {
        ("other", 0.50)
    };

    // Determine priority based on keywords
    let priority = if combined.contains("urgent")
        || combined.contains("emergency")
        || combined.contains("dangerous")
        || combined.contains("flood")
        || combined.contains("fire")
    {
        Some("urgent".to_string())
    } else if combined.contains("broken") || combined.contains("not working") {
        Some("high".to_string())
    } else {
        None
    };

    // Update fault with AI suggestion
    let _ = state
        .fault_repo
        .update_ai_suggestion(id, category, priority.as_deref(), confidence)
        .await;

    Ok(Json(AiSuggestionResponse {
        suggestion: AiSuggestion {
            category: category.to_string(),
            confidence,
            priority,
        },
    }))
}

/// Get fault statistics (Story 4.7).
#[utoipa::path(
    get,
    path = "/api/v1/faults/statistics",
    params(StatisticsQuery),
    responses(
        (status = 200, description = "Fault statistics", body = StatisticsResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "Faults"
)]
async fn get_statistics(
    State(state): State<AppState>,
    Query(query): Query<StatisticsQuery>,
) -> Result<Json<StatisticsResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let org_id = Uuid::nil();

    match state
        .fault_repo
        .get_statistics(org_id, query.building_id)
        .await
    {
        Ok(statistics) => Ok(Json(StatisticsResponse { statistics })),
        Err(e) => {
            tracing::error!("Failed to get statistics: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to get statistics",
                )),
            ))
        }
    }
}
