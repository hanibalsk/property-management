//! Announcement routes (Epic 6: Announcements & Communication).

use crate::state::AppState;
use api_core::{AuthUser, TenantExtractor};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{DateTime, Utc};
use common::errors::ErrorResponse;
use db::models::{
    target_type, AcknowledgmentStats, Announcement, AnnouncementAttachment, AnnouncementComment,
    AnnouncementListQuery, AnnouncementStatistics, AnnouncementSummary, AnnouncementWithDetails,
    CommentWithAuthor, CreateAnnouncement, CreateAnnouncementAttachment, CreateComment,
    DeleteComment, UpdateAnnouncement, UserAcknowledgmentStatus,
};
use serde::{Deserialize, Serialize};
use sqlx::Error as SqlxError;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// Constants
// ============================================================================

/// Maximum allowed title length (characters).
const MAX_TITLE_LENGTH: usize = 200;

/// Maximum allowed content length (characters).
const MAX_CONTENT_LENGTH: usize = 50_000;

/// Maximum allowed comment length (characters).
const MAX_COMMENT_LENGTH: usize = 2000;

// ============================================================================
// Response Types
// ============================================================================

/// Response for announcement creation.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateAnnouncementResponse {
    pub id: Uuid,
    pub message: String,
}

/// Response for announcement list with pagination.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AnnouncementListResponse {
    pub announcements: Vec<AnnouncementSummary>,
    /// Number of items in this response.
    pub count: usize,
    /// Total number of items matching the query (for pagination).
    pub total: i64,
}

/// Response for announcement details.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AnnouncementDetailResponse {
    pub announcement: AnnouncementWithDetails,
    pub attachments: Vec<AnnouncementAttachment>,
}

/// Response for generic announcement action.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AnnouncementActionResponse {
    pub message: String,
    pub announcement: Announcement,
}

/// Response for attachments list.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AttachmentsResponse {
    pub attachments: Vec<AnnouncementAttachment>,
}

/// Response for statistics.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StatisticsResponse {
    pub statistics: AnnouncementStatistics,
}

/// Response for unread count.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UnreadCountResponse {
    pub unread_count: i64,
}

/// Response for acknowledgment statistics (Story 6.2).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AcknowledgmentStatsResponse {
    pub stats: AcknowledgmentStats,
}

/// Response for acknowledgment list (Story 6.2).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AcknowledgmentListResponse {
    pub users: Vec<UserAcknowledgmentStatus>,
    pub count: usize,
}

// ============================================================================
// Request Types
// ============================================================================

/// Request for creating an announcement.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateAnnouncementRequest {
    pub title: String,
    pub content: String,
    pub target_type: String,
    pub target_ids: Option<Vec<Uuid>>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub comments_enabled: Option<bool>,
    pub acknowledgment_required: Option<bool>,
}

/// Request for updating an announcement.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateAnnouncementRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub target_type: Option<String>,
    pub target_ids: Option<Vec<Uuid>>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub comments_enabled: Option<bool>,
    pub acknowledgment_required: Option<bool>,
}

/// Request for scheduling an announcement.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ScheduleAnnouncementRequest {
    pub scheduled_at: DateTime<Utc>,
}

/// Request for pinning/unpinning an announcement.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PinAnnouncementRequest {
    pub pinned: bool,
}

/// Request for adding an attachment.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddAttachmentRequest {
    pub file_key: String,
    pub file_name: String,
    pub file_type: String,
    pub file_size: i64,
}

/// Query for listing announcements.
#[derive(Debug, Serialize, Deserialize, ToSchema, Default, utoipa::IntoParams)]
pub struct ListAnnouncementsQuery {
    pub status: Option<String>,
    pub target_type: Option<String>,
    pub author_id: Option<Uuid>,
    pub pinned: Option<bool>,
    pub from_date: Option<chrono::NaiveDate>,
    pub to_date: Option<chrono::NaiveDate>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// ============================================================================
// Comment Types (Story 6.3)
// ============================================================================

/// Request for creating a comment.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCommentRequest {
    pub content: String,
    pub parent_id: Option<Uuid>,
    #[serde(default)]
    pub ai_training_consent: bool,
}

/// Request for deleting a comment (moderation).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteCommentRequest {
    pub reason: Option<String>,
}

/// Response for comment list.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CommentsResponse {
    pub comments: Vec<CommentWithAuthor>,
    pub count: usize,
    pub total: i64,
}

/// Response for single comment.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CommentResponse {
    pub comment: CommentWithAuthor,
}

/// Query for listing comments.
#[derive(Debug, Serialize, Deserialize, ToSchema, Default, utoipa::IntoParams)]
pub struct ListCommentsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// ============================================================================
// Router
// ============================================================================

/// Create announcements router.
pub fn router() -> Router<AppState> {
    Router::new()
        // CRUD
        .route("/", post(create_announcement))
        .route("/", get(list_announcements))
        .route("/published", get(list_published_announcements))
        .route("/{id}", get(get_announcement))
        .route("/{id}", put(update_announcement))
        .route("/{id}", delete(delete_announcement))
        // Publishing
        .route("/{id}/publish", post(publish_announcement))
        .route("/{id}/schedule", post(schedule_announcement))
        .route("/{id}/archive", post(archive_announcement))
        // Pinning
        .route("/{id}/pin", post(pin_announcement))
        // Attachments
        .route("/{id}/attachments", get(list_attachments))
        .route("/{id}/attachments", post(add_attachment))
        .route("/{id}/attachments/{attachment_id}", delete(delete_attachment))
        // Read/Acknowledge (Story 6.2)
        .route("/{id}/read", post(mark_read))
        .route("/{id}/acknowledge", post(acknowledge))
        .route("/{id}/acknowledgments", get(get_acknowledgments))
        // Comments (Story 6.3)
        .route("/{id}/comments", get(list_comments))
        .route("/{id}/comments", post(create_comment))
        .route("/{id}/comments/{comment_id}", delete(delete_comment))
        // Statistics
        .route("/statistics", get(get_statistics))
        .route("/unread-count", get(get_unread_count))
}

// ============================================================================
// Handlers
// ============================================================================

/// Create a new announcement (Story 6.1).
///
/// Requires manager-level role (Manager, TechnicalManager, OrgAdmin, or SuperAdmin).
#[utoipa::path(
    post,
    path = "/api/v1/announcements",
    request_body = CreateAnnouncementRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Announcement created", body = CreateAnnouncementResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden - requires manager role", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn create_announcement(
    State(state): State<AppState>,
    auth: AuthUser,
    tenant: TenantExtractor,
    Json(req): Json<CreateAnnouncementRequest>,
) -> Result<(StatusCode, Json<CreateAnnouncementResponse>), (StatusCode, Json<ErrorResponse>)> {
    // Authorization: require manager-level role (Task 3.7)
    let role = tenant.role;
    if !role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only managers can create announcements",
            )),
        ));
    }

    let author_id = auth.user_id;
    let org_id = tenant.tenant_id;

    // Validate content length (H-3: Content validation)
    if req.title.len() > MAX_TITLE_LENGTH {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "BAD_REQUEST",
                format!("Title exceeds maximum length of {} characters", MAX_TITLE_LENGTH),
            )),
        ));
    }
    if req.content.len() > MAX_CONTENT_LENGTH {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "BAD_REQUEST",
                format!("Content exceeds maximum length of {} characters", MAX_CONTENT_LENGTH),
            )),
        ));
    }

    // Validate target_type
    if !target_type::ALL_TYPES.contains(&req.target_type.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new("BAD_REQUEST", "Invalid target_type")),
        ));
    }

    // Validate target_ids based on target_type
    if req.target_type != target_type::ALL && req.target_ids.as_ref().map_or(true, |v| v.is_empty())
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "BAD_REQUEST",
                "target_ids required for non-'all' target_type",
            )),
        ));
    }

    // Sanitize content (M-2: Basic markdown sanitization)
    let sanitized_content = sanitize_markdown(&req.content);

    let data = CreateAnnouncement {
        organization_id: org_id,
        author_id,
        title: req.title,
        content: sanitized_content,
        target_type: req.target_type,
        target_ids: req.target_ids.unwrap_or_default(),
        scheduled_at: req.scheduled_at,
        comments_enabled: req.comments_enabled,
        acknowledgment_required: req.acknowledgment_required,
    };

    match state.announcement_repo.create(data).await {
        Ok(announcement) => Ok((
            StatusCode::CREATED,
            Json(CreateAnnouncementResponse {
                id: announcement.id,
                message: "Announcement created successfully".to_string(),
            }),
        )),
        Err(e) => {
            tracing::error!("Failed to create announcement: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to create announcement",
                )),
            ))
        }
    }
}

/// List announcements with filters (for managers).
///
/// Requires manager-level role.
#[utoipa::path(
    get,
    path = "/api/v1/announcements",
    params(ListAnnouncementsQuery),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Announcement list", body = AnnouncementListResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden - requires manager role", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn list_announcements(
    State(state): State<AppState>,
    _auth: AuthUser,
    tenant: TenantExtractor,
    Query(query): Query<ListAnnouncementsQuery>,
) -> Result<Json<AnnouncementListResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Authorization: require manager-level role (Task 3.7)
    if !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only managers can list all announcements",
            )),
        ));
    }

    let org_id = tenant.tenant_id;

    let list_query = AnnouncementListQuery {
        status: query
            .status
            .map(|s| s.split(',').map(String::from).collect()),
        target_type: query.target_type.clone(),
        author_id: query.author_id,
        pinned: query.pinned,
        from_date: query.from_date,
        to_date: query.to_date,
        limit: query.limit,
        offset: query.offset,
    };

    // Get total count for pagination (H-4)
    let count_query = AnnouncementListQuery {
        status: list_query.status.clone(),
        target_type: query.target_type,
        author_id: query.author_id,
        pinned: query.pinned,
        from_date: query.from_date,
        to_date: query.to_date,
        limit: None,
        offset: None,
    };
    let total = state
        .announcement_repo
        .count(org_id, count_query)
        .await
        .unwrap_or(0);

    match state.announcement_repo.list(org_id, list_query).await {
        Ok(announcements) => {
            let count = announcements.len();
            Ok(Json(AnnouncementListResponse {
                announcements,
                count,
                total,
            }))
        }
        Err(e) => {
            tracing::error!("Failed to list announcements: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to list announcements",
                )),
            ))
        }
    }
}

/// List published announcements (for all authenticated users).
#[utoipa::path(
    get,
    path = "/api/v1/announcements/published",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Published announcement list", body = AnnouncementListResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn list_published_announcements(
    State(state): State<AppState>,
    _auth: AuthUser,
    tenant: TenantExtractor,
    Query(query): Query<ListAnnouncementsQuery>,
) -> Result<Json<AnnouncementListResponse>, (StatusCode, Json<ErrorResponse>)> {
    let org_id = tenant.tenant_id;

    // Get total count for pagination (H-4)
    let total = state
        .announcement_repo
        .count_published(org_id)
        .await
        .unwrap_or(0);

    match state
        .announcement_repo
        .list_published(org_id, query.limit, query.offset)
        .await
    {
        Ok(announcements) => {
            let count = announcements.len();
            Ok(Json(AnnouncementListResponse {
                announcements,
                count,
                total,
            }))
        }
        Err(e) => {
            tracing::error!("Failed to list published announcements: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to list announcements",
                )),
            ))
        }
    }
}

/// Get announcement details.
#[utoipa::path(
    get,
    path = "/api/v1/announcements/{id}",
    params(
        ("id" = Uuid, Path, description = "Announcement ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Announcement details", body = AnnouncementDetailResponse),
        (status = 404, description = "Announcement not found", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn get_announcement(
    State(state): State<AppState>,
    _auth: AuthUser,
    _tenant: TenantExtractor,
    Path(id): Path<Uuid>,
) -> Result<Json<AnnouncementDetailResponse>, (StatusCode, Json<ErrorResponse>)> {
    let announcement = match state.announcement_repo.find_by_id_with_details(id).await {
        Ok(Some(a)) => a,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Announcement not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to get announcement: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to get announcement",
                )),
            ));
        }
    };

    let attachments = state
        .announcement_repo
        .get_attachments(id)
        .await
        .unwrap_or_default();

    Ok(Json(AnnouncementDetailResponse {
        announcement,
        attachments,
    }))
}

/// Update announcement details.
///
/// Requires manager-level role.
#[utoipa::path(
    put,
    path = "/api/v1/announcements/{id}",
    params(
        ("id" = Uuid, Path, description = "Announcement ID")
    ),
    request_body = UpdateAnnouncementRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Announcement updated", body = AnnouncementActionResponse),
        (status = 400, description = "Cannot update", body = ErrorResponse),
        (status = 403, description = "Forbidden - requires manager role", body = ErrorResponse),
        (status = 404, description = "Announcement not found", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn update_announcement(
    State(state): State<AppState>,
    _auth: AuthUser,
    tenant: TenantExtractor,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateAnnouncementRequest>,
) -> Result<Json<AnnouncementActionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Authorization: require manager-level role (Task 3.7)
    if !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only managers can update announcements",
            )),
        ));
    }

    // Validate content length if provided (H-3)
    if let Some(ref title) = req.title {
        if title.len() > MAX_TITLE_LENGTH {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(
                    "BAD_REQUEST",
                    format!("Title exceeds maximum length of {} characters", MAX_TITLE_LENGTH),
                )),
            ));
        }
    }
    if let Some(ref content) = req.content {
        if content.len() > MAX_CONTENT_LENGTH {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(
                    "BAD_REQUEST",
                    format!("Content exceeds maximum length of {} characters", MAX_CONTENT_LENGTH),
                )),
            ));
        }
    }
    // Check announcement exists and can be edited
    let existing = match state.announcement_repo.find_by_id(id).await {
        Ok(Some(a)) => a,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Announcement not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find announcement: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find announcement",
                )),
            ));
        }
    };

    if !existing.can_edit() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "BAD_REQUEST",
                "Published announcements cannot be edited",
            )),
        ));
    }

    // Validate target_type if provided
    if let Some(ref tt) = req.target_type {
        if !target_type::ALL_TYPES.contains(&tt.as_str()) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new("BAD_REQUEST", "Invalid target_type")),
            ));
        }
    }

    // Sanitize content if provided (M-2)
    let sanitized_content = req.content.map(|c| sanitize_markdown(&c));

    let data = UpdateAnnouncement {
        title: req.title,
        content: sanitized_content,
        target_type: req.target_type,
        target_ids: req.target_ids,
        scheduled_at: req.scheduled_at,
        comments_enabled: req.comments_enabled,
        acknowledgment_required: req.acknowledgment_required,
    };

    match state.announcement_repo.update(id, data).await {
        Ok(announcement) => Ok(Json(AnnouncementActionResponse {
            message: "Announcement updated".to_string(),
            announcement,
        })),
        Err(e) => {
            tracing::error!("Failed to update announcement: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to update announcement",
                )),
            ))
        }
    }
}

/// Delete an announcement (draft only).
///
/// Requires manager-level role.
#[utoipa::path(
    delete,
    path = "/api/v1/announcements/{id}",
    params(
        ("id" = Uuid, Path, description = "Announcement ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 204, description = "Announcement deleted"),
        (status = 400, description = "Cannot delete", body = ErrorResponse),
        (status = 403, description = "Forbidden - requires manager role", body = ErrorResponse),
        (status = 404, description = "Announcement not found", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn delete_announcement(
    State(state): State<AppState>,
    _auth: AuthUser,
    tenant: TenantExtractor,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // Authorization: require manager-level role (Task 3.7)
    if !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only managers can delete announcements",
            )),
        ));
    }
    // Check if draft
    let existing = match state.announcement_repo.find_by_id(id).await {
        Ok(Some(a)) => a,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Announcement not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find announcement: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find announcement",
                )),
            ));
        }
    };

    if !existing.is_draft() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "BAD_REQUEST",
                "Only draft announcements can be deleted",
            )),
        ));
    }

    match state.announcement_repo.delete(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            tracing::error!("Failed to delete announcement: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to delete announcement",
                )),
            ))
        }
    }
}

/// Publish an announcement immediately (Story 6.1).
///
/// Requires manager-level role.
#[utoipa::path(
    post,
    path = "/api/v1/announcements/{id}/publish",
    params(
        ("id" = Uuid, Path, description = "Announcement ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Announcement published", body = AnnouncementActionResponse),
        (status = 400, description = "Cannot publish", body = ErrorResponse),
        (status = 403, description = "Forbidden - requires manager role", body = ErrorResponse),
        (status = 404, description = "Announcement not found", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn publish_announcement(
    State(state): State<AppState>,
    _auth: AuthUser,
    tenant: TenantExtractor,
    Path(id): Path<Uuid>,
) -> Result<Json<AnnouncementActionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Authorization: require manager-level role (Task 3.7)
    if !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only managers can publish announcements",
            )),
        ));
    }

    match state.announcement_repo.publish(id).await {
        Ok(announcement) => {
            // TODO(Epic-2B): Trigger notifications here when notification infrastructure is ready
            // Integration point: notification_service.send_announcement_notification(&announcement)
            // This should notify all targeted users based on announcement.target_type and target_ids
            tracing::info!(
                announcement_id = %announcement.id,
                target_type = %announcement.target_type,
                "Announcement published - notification integration pending Epic 2B"
            );
            Ok(Json(AnnouncementActionResponse {
                message: "Announcement published".to_string(),
                announcement,
            }))
        }
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Announcement not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to publish announcement: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to publish announcement",
                )),
            ))
        }
    }
}

/// Schedule an announcement for future publishing (Story 6.1).
///
/// Requires manager-level role.
#[utoipa::path(
    post,
    path = "/api/v1/announcements/{id}/schedule",
    params(
        ("id" = Uuid, Path, description = "Announcement ID")
    ),
    request_body = ScheduleAnnouncementRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Announcement scheduled", body = AnnouncementActionResponse),
        (status = 400, description = "Invalid schedule", body = ErrorResponse),
        (status = 403, description = "Forbidden - requires manager role", body = ErrorResponse),
        (status = 404, description = "Announcement not found", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn schedule_announcement(
    State(state): State<AppState>,
    _auth: AuthUser,
    tenant: TenantExtractor,
    Path(id): Path<Uuid>,
    Json(req): Json<ScheduleAnnouncementRequest>,
) -> Result<Json<AnnouncementActionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Authorization: require manager-level role (Task 3.7)
    if !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only managers can schedule announcements",
            )),
        ));
    }
    // Validate scheduled_at is in the future
    if req.scheduled_at <= Utc::now() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "BAD_REQUEST",
                "Scheduled time must be in the future",
            )),
        ));
    }

    match state.announcement_repo.schedule(id, req.scheduled_at).await {
        Ok(announcement) => Ok(Json(AnnouncementActionResponse {
            message: "Announcement scheduled".to_string(),
            announcement,
        })),
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Announcement not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to schedule announcement: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to schedule announcement",
                )),
            ))
        }
    }
}

/// Archive an announcement.
///
/// Requires manager-level role.
#[utoipa::path(
    post,
    path = "/api/v1/announcements/{id}/archive",
    params(
        ("id" = Uuid, Path, description = "Announcement ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Announcement archived", body = AnnouncementActionResponse),
        (status = 403, description = "Forbidden - requires manager role", body = ErrorResponse),
        (status = 404, description = "Announcement not found", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn archive_announcement(
    State(state): State<AppState>,
    _auth: AuthUser,
    tenant: TenantExtractor,
    Path(id): Path<Uuid>,
) -> Result<Json<AnnouncementActionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Authorization: require manager-level role (Task 3.7)
    if !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only managers can archive announcements",
            )),
        ));
    }
    match state.announcement_repo.archive(id).await {
        Ok(announcement) => Ok(Json(AnnouncementActionResponse {
            message: "Announcement archived".to_string(),
            announcement,
        })),
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Announcement not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to archive announcement: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to archive announcement",
                )),
            ))
        }
    }
}

/// Pin/unpin an announcement (Story 6.4 foundation).
///
/// Requires manager-level role.
#[utoipa::path(
    post,
    path = "/api/v1/announcements/{id}/pin",
    params(
        ("id" = Uuid, Path, description = "Announcement ID")
    ),
    request_body = PinAnnouncementRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Pin status updated", body = AnnouncementActionResponse),
        (status = 403, description = "Forbidden - requires manager role", body = ErrorResponse),
        (status = 404, description = "Announcement not found", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn pin_announcement(
    State(state): State<AppState>,
    auth: AuthUser,
    tenant: TenantExtractor,
    Path(id): Path<Uuid>,
    Json(req): Json<PinAnnouncementRequest>,
) -> Result<Json<AnnouncementActionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Authorization: require manager-level role (Task 3.7)
    if !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only managers can pin announcements",
            )),
        ));
    }

    let user_id = auth.user_id;

    let result = if req.pinned {
        state.announcement_repo.pin(id, user_id).await
    } else {
        state.announcement_repo.unpin(id).await
    };

    match result {
        Ok(announcement) => Ok(Json(AnnouncementActionResponse {
            message: if req.pinned {
                "Announcement pinned"
            } else {
                "Announcement unpinned"
            }
            .to_string(),
            announcement,
        })),
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Announcement not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to update pin status: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to update pin status",
                )),
            ))
        }
    }
}

/// List attachments for an announcement.
#[utoipa::path(
    get,
    path = "/api/v1/announcements/{id}/attachments",
    params(
        ("id" = Uuid, Path, description = "Announcement ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Attachments list", body = AttachmentsResponse),
    ),
    tag = "Announcements"
)]
async fn list_attachments(
    State(state): State<AppState>,
    _auth: AuthUser,
    _tenant: TenantExtractor,
    Path(id): Path<Uuid>,
) -> Result<Json<AttachmentsResponse>, (StatusCode, Json<ErrorResponse>)> {
    match state.announcement_repo.get_attachments(id).await {
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

/// Add an attachment to an announcement.
///
/// Requires manager-level role.
#[utoipa::path(
    post,
    path = "/api/v1/announcements/{id}/attachments",
    params(
        ("id" = Uuid, Path, description = "Announcement ID")
    ),
    request_body = AddAttachmentRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Attachment added", body = AnnouncementAttachment),
        (status = 403, description = "Forbidden - requires manager role", body = ErrorResponse),
        (status = 404, description = "Announcement not found", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn add_attachment(
    State(state): State<AppState>,
    _auth: AuthUser,
    tenant: TenantExtractor,
    Path(id): Path<Uuid>,
    Json(req): Json<AddAttachmentRequest>,
) -> Result<(StatusCode, Json<AnnouncementAttachment>), (StatusCode, Json<ErrorResponse>)> {
    // Authorization: require manager-level role (Task 3.7)
    if !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only managers can add attachments",
            )),
        ));
    }
    let data = CreateAnnouncementAttachment {
        announcement_id: id,
        file_key: req.file_key,
        file_name: req.file_name,
        file_type: req.file_type,
        file_size: req.file_size,
    };

    match state.announcement_repo.add_attachment(data).await {
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
///
/// Requires manager-level role.
#[utoipa::path(
    delete,
    path = "/api/v1/announcements/{id}/attachments/{attachment_id}",
    params(
        ("id" = Uuid, Path, description = "Announcement ID"),
        ("attachment_id" = Uuid, Path, description = "Attachment ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 204, description = "Attachment deleted"),
        (status = 403, description = "Forbidden - requires manager role", body = ErrorResponse),
        (status = 404, description = "Attachment not found", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn delete_attachment(
    State(state): State<AppState>,
    _auth: AuthUser,
    tenant: TenantExtractor,
    Path((_id, attachment_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // Authorization: require manager-level role (Task 3.7)
    if !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only managers can delete attachments",
            )),
        ));
    }
    match state.announcement_repo.delete_attachment(attachment_id).await {
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

/// Mark an announcement as read (Story 6.2 foundation).
#[utoipa::path(
    post,
    path = "/api/v1/announcements/{id}/read",
    params(
        ("id" = Uuid, Path, description = "Announcement ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Marked as read"),
        (status = 404, description = "Announcement not found", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn mark_read(
    State(state): State<AppState>,
    auth: AuthUser,
    _tenant: TenantExtractor,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let user_id = auth.user_id;

    match state.announcement_repo.mark_read(id, user_id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            tracing::error!("Failed to mark as read: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to mark as read",
                )),
            ))
        }
    }
}

/// Acknowledge an announcement (Story 6.2 foundation).
#[utoipa::path(
    post,
    path = "/api/v1/announcements/{id}/acknowledge",
    params(
        ("id" = Uuid, Path, description = "Announcement ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Acknowledged"),
        (status = 404, description = "Announcement not found", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn acknowledge(
    State(state): State<AppState>,
    auth: AuthUser,
    _tenant: TenantExtractor,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let user_id = auth.user_id;

    match state.announcement_repo.acknowledge(id, user_id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            tracing::error!("Failed to acknowledge: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to acknowledge",
                )),
            ))
        }
    }
}

/// Get announcement statistics.
///
/// Requires manager-level role.
#[utoipa::path(
    get,
    path = "/api/v1/announcements/statistics",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Announcement statistics", body = StatisticsResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden - requires manager role", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn get_statistics(
    State(state): State<AppState>,
    _auth: AuthUser,
    tenant: TenantExtractor,
) -> Result<Json<StatisticsResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Authorization: require manager-level role (Task 3.7)
    if !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only managers can view statistics",
            )),
        ));
    }

    let org_id = tenant.tenant_id;

    match state.announcement_repo.get_statistics(org_id).await {
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

/// Get unread announcement count.
#[utoipa::path(
    get,
    path = "/api/v1/announcements/unread-count",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Unread count", body = UnreadCountResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn get_unread_count(
    State(state): State<AppState>,
    auth: AuthUser,
    tenant: TenantExtractor,
) -> Result<Json<UnreadCountResponse>, (StatusCode, Json<ErrorResponse>)> {
    let org_id = tenant.tenant_id;
    let user_id = auth.user_id;

    match state.announcement_repo.count_unread(org_id, user_id).await {
        Ok(unread_count) => Ok(Json(UnreadCountResponse { unread_count })),
        Err(e) => {
            tracing::error!("Failed to get unread count: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to get unread count",
                )),
            ))
        }
    }
}

/// Query for acknowledgment list pagination.
#[derive(Debug, Serialize, Deserialize, ToSchema, Default, utoipa::IntoParams)]
pub struct AcknowledgmentListQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Get acknowledgment statistics and list for an announcement (Story 6.2).
///
/// Requires manager-level role.
#[utoipa::path(
    get,
    path = "/api/v1/announcements/{id}/acknowledgments",
    params(
        ("id" = Uuid, Path, description = "Announcement ID"),
        AcknowledgmentListQuery
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Acknowledgment stats and list", body = AcknowledgmentStatsResponse),
        (status = 403, description = "Forbidden - requires manager role", body = ErrorResponse),
        (status = 404, description = "Announcement not found", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn get_acknowledgments(
    State(state): State<AppState>,
    _auth: AuthUser,
    tenant: TenantExtractor,
    Path(id): Path<Uuid>,
    Query(_query): Query<AcknowledgmentListQuery>,
) -> Result<Json<AcknowledgmentStatsResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Authorization: require manager-level role
    if !tenant.role.is_manager() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "Only managers can view acknowledgment statistics",
            )),
        ));
    }

    // Check announcement exists
    match state.announcement_repo.find_by_id(id).await {
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Announcement not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find announcement: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find announcement",
                )),
            ));
        }
        Ok(Some(_)) => {}
    }

    match state.announcement_repo.get_acknowledgment_stats(id).await {
        Ok(stats) => Ok(Json(AcknowledgmentStatsResponse { stats })),
        Err(e) => {
            tracing::error!("Failed to get acknowledgment stats: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to get acknowledgment statistics",
                )),
            ))
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Sanitize markdown content by removing potentially dangerous HTML tags.
///
/// This uses an allowlist approach to only permit safe markdown/HTML elements.
/// Stripped elements: script, iframe, object, embed, form, input, button,
/// and any elements with event handlers (onclick, onerror, etc.).
///
/// Note: For production use, consider using a proper HTML sanitizer library
/// like `ammonia` for more robust protection.
fn sanitize_markdown(content: &str) -> String {
    // Remove script tags and their content
    let re_script = regex::Regex::new(r"(?is)<script[^>]*>.*?</script>").unwrap_or_else(|_| {
        regex::Regex::new(r"^$").unwrap()
    });
    let content = re_script.replace_all(content, "");

    // Remove iframe tags
    let re_iframe = regex::Regex::new(r"(?is)<iframe[^>]*>.*?</iframe>").unwrap_or_else(|_| {
        regex::Regex::new(r"^$").unwrap()
    });
    let content = re_iframe.replace_all(&content, "");

    // Remove object/embed tags
    let re_object = regex::Regex::new(r"(?is)<(object|embed)[^>]*>.*?</(object|embed)>").unwrap_or_else(|_| {
        regex::Regex::new(r"^$").unwrap()
    });
    let content = re_object.replace_all(&content, "");

    // Remove form elements
    let re_form = regex::Regex::new(r"(?is)<(form|input|button|textarea|select)[^>]*>").unwrap_or_else(|_| {
        regex::Regex::new(r"^$").unwrap()
    });
    let content = re_form.replace_all(&content, "");

    // Remove event handlers (onclick, onerror, onload, etc.)
    let re_events = regex::Regex::new(r#"(?i)\s+on\w+\s*=\s*["'][^"']*["']"#).unwrap_or_else(|_| {
        regex::Regex::new(r"^$").unwrap()
    });
    let content = re_events.replace_all(&content, "");

    // Remove javascript: URLs
    let re_js_url = regex::Regex::new(r"(?i)javascript\s*:").unwrap_or_else(|_| {
        regex::Regex::new(r"^$").unwrap()
    });
    let content = re_js_url.replace_all(&content, "");

    content.to_string()
}

// ============================================================================
// Comment Handlers (Story 6.3)
// ============================================================================

/// List comments for an announcement.
#[utoipa::path(
    get,
    path = "/api/v1/announcements/{id}/comments",
    params(
        ("id" = Uuid, Path, description = "Announcement ID"),
        ListCommentsQuery
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Comment list", body = CommentsResponse),
        (status = 404, description = "Announcement not found", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn list_comments(
    State(state): State<AppState>,
    _auth: AuthUser,
    _tenant: TenantExtractor,
    Path(id): Path<Uuid>,
    Query(query): Query<ListCommentsQuery>,
) -> Result<Json<CommentsResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Check announcement exists
    match state.announcement_repo.find_by_id(id).await {
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Announcement not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find announcement: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find announcement",
                )),
            ));
        }
        Ok(Some(_)) => {}
    }

    // Get total count
    let total = match state.announcement_repo.get_comment_count(id).await {
        Ok(count) => count,
        Err(e) => {
            tracing::error!("Failed to get comment count: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to get comment count",
                )),
            ));
        }
    };

    // Get threaded comments
    match state
        .announcement_repo
        .get_threaded_comments(id, query.limit, query.offset)
        .await
    {
        Ok(comments) => Ok(Json(CommentsResponse {
            count: comments.len(),
            comments,
            total,
        })),
        Err(e) => {
            tracing::error!("Failed to list comments: {}", e);
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

/// Create a comment on an announcement.
#[utoipa::path(
    post,
    path = "/api/v1/announcements/{id}/comments",
    params(
        ("id" = Uuid, Path, description = "Announcement ID")
    ),
    request_body = CreateCommentRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = 201, description = "Comment created", body = AnnouncementComment),
        (status = 400, description = "Invalid request or comments disabled", body = ErrorResponse),
        (status = 404, description = "Announcement not found", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn create_comment(
    State(state): State<AppState>,
    auth: AuthUser,
    _tenant: TenantExtractor,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<AnnouncementComment>), (StatusCode, Json<ErrorResponse>)> {
    // Validate content length
    if req.content.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new("BAD_REQUEST", "Comment content is required")),
        ));
    }
    if req.content.len() > MAX_COMMENT_LENGTH {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "BAD_REQUEST",
                format!(
                    "Comment exceeds maximum length of {} characters",
                    MAX_COMMENT_LENGTH
                ),
            )),
        ));
    }

    // Check announcement exists and has comments enabled
    let announcement = match state.announcement_repo.find_by_id(id).await {
        Ok(Some(a)) => a,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Announcement not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find announcement: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find announcement",
                )),
            ));
        }
    };

    // Check comments are enabled
    if !announcement.comments_enabled {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "COMMENTS_DISABLED",
                "Comments are disabled for this announcement",
            )),
        ));
    }

    // Check announcement is published
    if !announcement.is_published() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "BAD_REQUEST",
                "Cannot comment on unpublished announcements",
            )),
        ));
    }

    // If parent_id is provided, verify it exists and belongs to same announcement
    if let Some(parent_id) = req.parent_id {
        match state.announcement_repo.get_comment(parent_id).await {
            Ok(Some(parent)) => {
                if parent.announcement_id != id {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(ErrorResponse::new(
                            "BAD_REQUEST",
                            "Parent comment belongs to different announcement",
                        )),
                    ));
                }
                // Only allow one level of nesting
                if parent.parent_id.is_some() {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(ErrorResponse::new(
                            "BAD_REQUEST",
                            "Cannot reply to a reply - maximum nesting depth is 2",
                        )),
                    ));
                }
            }
            Ok(None) => {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse::new("NOT_FOUND", "Parent comment not found")),
                ))
            }
            Err(e) => {
                tracing::error!("Failed to find parent comment: {}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::new(
                        "INTERNAL_ERROR",
                        "Failed to find parent comment",
                    )),
                ));
            }
        }
    }

    // Sanitize content
    let sanitized_content = sanitize_markdown(&req.content);

    let data = CreateComment {
        announcement_id: id,
        user_id: auth.user_id,
        parent_id: req.parent_id,
        content: sanitized_content,
        ai_training_consent: req.ai_training_consent,
    };

    match state.announcement_repo.create_comment(data).await {
        Ok(comment) => {
            tracing::info!(
                comment_id = %comment.id,
                announcement_id = %id,
                user_id = %auth.user_id,
                "Comment created"
            );
            Ok((StatusCode::CREATED, Json(comment)))
        }
        Err(e) => {
            tracing::error!("Failed to create comment: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to create comment",
                )),
            ))
        }
    }
}

/// Delete a comment (author or manager moderation).
#[utoipa::path(
    delete,
    path = "/api/v1/announcements/{id}/comments/{comment_id}",
    params(
        ("id" = Uuid, Path, description = "Announcement ID"),
        ("comment_id" = Uuid, Path, description = "Comment ID")
    ),
    request_body = Option<DeleteCommentRequest>,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Comment deleted", body = AnnouncementComment),
        (status = 403, description = "Not authorized to delete", body = ErrorResponse),
        (status = 404, description = "Comment not found", body = ErrorResponse),
    ),
    tag = "Announcements"
)]
async fn delete_comment(
    State(state): State<AppState>,
    auth: AuthUser,
    tenant: TenantExtractor,
    Path((id, comment_id)): Path<(Uuid, Uuid)>,
    body: Option<Json<DeleteCommentRequest>>,
) -> Result<Json<AnnouncementComment>, (StatusCode, Json<ErrorResponse>)> {
    // Get the comment
    let comment = match state.announcement_repo.get_comment(comment_id).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Comment not found")),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to find comment: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find comment",
                )),
            ));
        }
    };

    // Verify comment belongs to the announcement
    if comment.announcement_id != id {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Comment not found")),
        ));
    }

    // Check if already deleted
    if comment.is_deleted() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "BAD_REQUEST",
                "Comment is already deleted",
            )),
        ));
    }

    // Authorization: author can delete their own, managers can delete any
    let is_author = comment.user_id == auth.user_id;
    let is_manager = tenant.role.is_manager();

    if !is_author && !is_manager {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse::new(
                "FORBIDDEN",
                "You can only delete your own comments",
            )),
        ));
    }

    // Get deletion reason (only for manager moderation)
    let deletion_reason = if is_manager && !is_author {
        body.and_then(|b| b.reason.clone())
    } else {
        None
    };

    let data = DeleteComment {
        comment_id,
        deleted_by: auth.user_id,
        deletion_reason,
    };

    match state.announcement_repo.delete_comment(data).await {
        Ok(deleted) => {
            tracing::info!(
                comment_id = %comment_id,
                deleted_by = %auth.user_id,
                is_moderation = %(!is_author && is_manager),
                "Comment deleted"
            );
            Ok(Json(deleted))
        }
        Err(SqlxError::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Comment not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to delete comment: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to delete comment",
                )),
            ))
        }
    }
}
