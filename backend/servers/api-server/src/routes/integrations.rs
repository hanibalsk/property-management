//! External Integrations routes (Epic 61).
//!
//! Routes for calendar sync, accounting exports, e-signatures, video conferencing, and webhooks.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use common::errors::ErrorResponse;
use db::models::{
    AccountingExport, AccountingExportSettings, CalendarConnection, CalendarSyncResult,
    CreateAccountingExport, CreateCalendarConnection, CreateESignatureWorkflow,
    CreateIntegrationCalendarEvent as CreateCalendarEvent, CreateVideoConferenceConnection,
    CreateVideoMeeting, CreateWebhookSubscription, ESignatureWorkflow,
    ESignatureWorkflowWithRecipients, IntegrationCalendarEvent as CalendarEvent,
    IntegrationStatistics, SyncCalendarRequest, TestWebhookRequest, TestWebhookResponse,
    UpdateAccountingExportSettings, UpdateCalendarConnection, UpdateVideoMeeting,
    UpdateWebhookSubscription, VideoConferenceConnection, VideoMeeting, WebhookDeliveryLog,
    WebhookDeliveryQuery, WebhookStatistics, WebhookSubscription,
};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::state::AppState;

/// Create integrations router.
pub fn router() -> Router<AppState> {
    Router::new()
        // Statistics
        .route("/organizations/:org_id/stats", get(get_integration_stats))
        // Calendar routes (Story 61.1)
        .route(
            "/organizations/:org_id/calendars",
            get(list_calendar_connections),
        )
        .route(
            "/organizations/:org_id/calendars",
            post(create_calendar_connection),
        )
        .route("/calendars/:id", get(get_calendar_connection))
        .route("/calendars/:id", put(update_calendar_connection))
        .route("/calendars/:id", delete(delete_calendar_connection))
        .route("/calendars/:id/sync", post(sync_calendar))
        .route("/calendars/:id/events", get(list_calendar_events))
        .route("/calendars/:id/events", post(create_calendar_event))
        // Accounting routes (Story 61.2)
        .route(
            "/organizations/:org_id/accounting/exports",
            get(list_accounting_exports),
        )
        .route(
            "/organizations/:org_id/accounting/exports",
            post(create_accounting_export),
        )
        .route("/accounting/exports/:id", get(get_accounting_export))
        .route(
            "/accounting/exports/:id/download",
            get(download_accounting_export),
        )
        .route(
            "/organizations/:org_id/accounting/settings/:system",
            get(get_accounting_settings),
        )
        .route(
            "/organizations/:org_id/accounting/settings/:system",
            put(update_accounting_settings),
        )
        // E-Signature routes (Story 61.3)
        .route(
            "/organizations/:org_id/esignatures",
            get(list_esignature_workflows),
        )
        .route(
            "/organizations/:org_id/esignatures",
            post(create_esignature_workflow),
        )
        .route("/esignatures/:id", get(get_esignature_workflow))
        .route("/esignatures/:id/send", post(send_esignature_workflow))
        .route("/esignatures/:id/void", post(void_esignature_workflow))
        .route("/esignatures/:id/remind", post(send_esignature_reminder))
        .route("/esignatures/webhook", post(esignature_webhook))
        // Video Conferencing routes (Story 61.4)
        .route(
            "/organizations/:org_id/video/connections",
            get(list_video_connections),
        )
        .route(
            "/organizations/:org_id/video/connections",
            post(create_video_connection),
        )
        .route("/video/connections/:id", delete(delete_video_connection))
        .route(
            "/organizations/:org_id/video/meetings",
            get(list_video_meetings),
        )
        .route(
            "/organizations/:org_id/video/meetings",
            post(create_video_meeting),
        )
        .route("/video/meetings/:id", get(get_video_meeting))
        .route("/video/meetings/:id", put(update_video_meeting))
        .route("/video/meetings/:id", delete(delete_video_meeting))
        .route("/video/meetings/:id/start", post(start_video_meeting))
        // Webhook routes (Story 61.5)
        .route(
            "/organizations/:org_id/webhooks",
            get(list_webhook_subscriptions),
        )
        .route(
            "/organizations/:org_id/webhooks",
            post(create_webhook_subscription),
        )
        .route("/webhooks/:id", get(get_webhook_subscription))
        .route("/webhooks/:id", put(update_webhook_subscription))
        .route("/webhooks/:id", delete(delete_webhook_subscription))
        .route("/webhooks/:id/test", post(test_webhook))
        .route("/webhooks/:id/logs", get(list_webhook_logs))
        .route("/webhooks/:id/stats", get(get_webhook_stats))
}

// ==================== Types ====================

/// Organization ID path parameter.
#[derive(Debug, Deserialize, IntoParams)]
pub struct OrgIdPath {
    pub org_id: Uuid,
}

/// Resource ID path parameter.
#[derive(Debug, Deserialize, IntoParams)]
pub struct ResourceIdPath {
    pub id: Uuid,
}

/// Accounting system path parameter.
#[derive(Debug, Deserialize, IntoParams)]
pub struct AccountingSystemPath {
    pub org_id: Uuid,
    pub system: String,
}

/// Calendar query parameters.
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct CalendarQuery {
    pub user_id: Option<Uuid>,
}

/// Calendar events query parameters.
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct CalendarEventsQuery {
    pub from: Option<chrono::DateTime<chrono::Utc>>,
    pub to: Option<chrono::DateTime<chrono::Utc>>,
}

/// Accounting exports query parameters.
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct AccountingExportQuery {
    pub system_type: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

/// E-signature query parameters.
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ESignatureQuery {
    pub status: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

/// Video meetings query parameters.
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct VideoMeetingQuery {
    pub from: Option<chrono::DateTime<chrono::Utc>>,
    pub status: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

fn default_limit() -> i32 {
    50
}

// ==================== Statistics ====================

/// Get integration statistics for an organization.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/organizations/{org_id}/stats",
    params(OrgIdPath),
    responses(
        (status = 200, description = "Statistics retrieved", body = IntegrationStatistics),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn get_integration_stats(
    State(state): State<AppState>,
    Path(path): Path<OrgIdPath>,
) -> Result<Json<IntegrationStatistics>, (StatusCode, Json<ErrorResponse>)> {
    let stats = state
        .integration_repo
        .get_integration_statistics(path.org_id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to get integration statistics");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to get integration statistics",
                )),
            )
        })?;

    Ok(Json(stats))
}

// ==================== Calendar (Story 61.1) ====================

/// List calendar connections for an organization.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/organizations/{org_id}/calendars",
    params(OrgIdPath, CalendarQuery),
    responses(
        (status = 200, description = "Connections retrieved", body = Vec<CalendarConnection>),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn list_calendar_connections(
    State(state): State<AppState>,
    Path(path): Path<OrgIdPath>,
    Query(query): Query<CalendarQuery>,
) -> Result<Json<Vec<CalendarConnection>>, (StatusCode, Json<ErrorResponse>)> {
    let connections = state
        .integration_repo
        .list_calendar_connections(path.org_id, query.user_id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list calendar connections");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to list calendar connections",
                )),
            )
        })?;

    Ok(Json(connections))
}

/// Create a calendar connection.
#[utoipa::path(
    post,
    path = "/api/v1/integrations/organizations/{org_id}/calendars",
    params(OrgIdPath),
    request_body = CreateCalendarConnection,
    responses(
        (status = 201, description = "Connection created", body = CalendarConnection),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn create_calendar_connection(
    State(_state): State<AppState>,
    Path(_path): Path<OrgIdPath>,
    Json(_data): Json<CreateCalendarConnection>,
) -> Result<(StatusCode, Json<CalendarConnection>), (StatusCode, Json<ErrorResponse>)> {
    // TODO: Extract user_id from authentication context
    Err((
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse::new(
            "UNAUTHORIZED",
            "Authentication required",
        )),
    ))
}

/// Get a calendar connection by ID.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/calendars/{id}",
    params(ResourceIdPath),
    responses(
        (status = 200, description = "Connection retrieved", body = CalendarConnection),
        (status = 404, description = "Connection not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn get_calendar_connection(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
) -> Result<Json<CalendarConnection>, (StatusCode, Json<ErrorResponse>)> {
    let connection = state
        .integration_repo
        .get_calendar_connection(path.id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to get calendar connection");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to get calendar connection",
                )),
            )
        })?;

    match connection {
        Some(c) => Ok(Json(c)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new(
                "NOT_FOUND",
                "Calendar connection not found",
            )),
        )),
    }
}

/// Update a calendar connection.
#[utoipa::path(
    put,
    path = "/api/v1/integrations/calendars/{id}",
    params(ResourceIdPath),
    request_body = UpdateCalendarConnection,
    responses(
        (status = 200, description = "Connection updated", body = CalendarConnection),
        (status = 404, description = "Connection not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn update_calendar_connection(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
    Json(data): Json<UpdateCalendarConnection>,
) -> Result<Json<CalendarConnection>, (StatusCode, Json<ErrorResponse>)> {
    let connection = state
        .integration_repo
        .update_calendar_connection(path.id, data)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to update calendar connection");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to update calendar connection",
                )),
            )
        })?;

    Ok(Json(connection))
}

/// Delete a calendar connection.
#[utoipa::path(
    delete,
    path = "/api/v1/integrations/calendars/{id}",
    params(ResourceIdPath),
    responses(
        (status = 204, description = "Connection deleted"),
        (status = 404, description = "Connection not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn delete_calendar_connection(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let deleted = state
        .integration_repo
        .delete_calendar_connection(path.id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to delete calendar connection");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to delete calendar connection",
                )),
            )
        })?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new(
                "NOT_FOUND",
                "Calendar connection not found",
            )),
        ))
    }
}

/// Sync calendar events.
#[utoipa::path(
    post,
    path = "/api/v1/integrations/calendars/{id}/sync",
    params(ResourceIdPath),
    request_body = SyncCalendarRequest,
    responses(
        (status = 200, description = "Calendar synced", body = CalendarSyncResult),
        (status = 404, description = "Connection not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn sync_calendar(
    State(_state): State<AppState>,
    Path(_path): Path<ResourceIdPath>,
    Json(_data): Json<SyncCalendarRequest>,
) -> Result<Json<CalendarSyncResult>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implement actual calendar sync with external provider
    Ok(Json(CalendarSyncResult {
        events_created: 0,
        events_updated: 0,
        events_deleted: 0,
        errors: vec![],
        synced_at: chrono::Utc::now(),
    }))
}

/// List calendar events.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/calendars/{id}/events",
    params(ResourceIdPath, CalendarEventsQuery),
    responses(
        (status = 200, description = "Events retrieved", body = Vec<CalendarEvent>),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn list_calendar_events(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
    Query(query): Query<CalendarEventsQuery>,
) -> Result<Json<Vec<CalendarEvent>>, (StatusCode, Json<ErrorResponse>)> {
    let events = state
        .integration_repo
        .list_calendar_events(path.id, query.from, query.to)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list calendar events");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to list calendar events",
                )),
            )
        })?;

    Ok(Json(events))
}

/// Create a calendar event.
#[utoipa::path(
    post,
    path = "/api/v1/integrations/calendars/{id}/events",
    params(ResourceIdPath),
    request_body = CreateCalendarEvent,
    responses(
        (status = 201, description = "Event created", body = CalendarEvent),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn create_calendar_event(
    State(state): State<AppState>,
    Path(_path): Path<ResourceIdPath>,
    Json(data): Json<CreateCalendarEvent>,
) -> Result<(StatusCode, Json<CalendarEvent>), (StatusCode, Json<ErrorResponse>)> {
    let event = state
        .integration_repo
        .create_calendar_event(data)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create calendar event");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to create calendar event",
                )),
            )
        })?;

    Ok((StatusCode::CREATED, Json(event)))
}

// ==================== Accounting (Story 61.2) ====================

/// List accounting exports for an organization.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/organizations/{org_id}/accounting/exports",
    params(OrgIdPath, AccountingExportQuery),
    responses(
        (status = 200, description = "Exports retrieved", body = Vec<AccountingExport>),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn list_accounting_exports(
    State(state): State<AppState>,
    Path(path): Path<OrgIdPath>,
    Query(query): Query<AccountingExportQuery>,
) -> Result<Json<Vec<AccountingExport>>, (StatusCode, Json<ErrorResponse>)> {
    let exports = state
        .integration_repo
        .list_accounting_exports(path.org_id, query.system_type.as_deref(), query.limit)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list accounting exports");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to list accounting exports",
                )),
            )
        })?;

    Ok(Json(exports))
}

/// Create an accounting export.
#[utoipa::path(
    post,
    path = "/api/v1/integrations/organizations/{org_id}/accounting/exports",
    params(OrgIdPath),
    request_body = CreateAccountingExport,
    responses(
        (status = 201, description = "Export created", body = AccountingExport),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn create_accounting_export(
    State(_state): State<AppState>,
    Path(_path): Path<OrgIdPath>,
    Json(_data): Json<CreateAccountingExport>,
) -> Result<(StatusCode, Json<AccountingExport>), (StatusCode, Json<ErrorResponse>)> {
    // TODO: Extract user_id from authentication context
    Err((
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse::new(
            "UNAUTHORIZED",
            "Authentication required",
        )),
    ))
}

/// Get an accounting export by ID.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/accounting/exports/{id}",
    params(ResourceIdPath),
    responses(
        (status = 200, description = "Export retrieved", body = AccountingExport),
        (status = 404, description = "Export not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn get_accounting_export(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
) -> Result<Json<AccountingExport>, (StatusCode, Json<ErrorResponse>)> {
    let export = state
        .integration_repo
        .get_accounting_export(path.id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to get accounting export");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to get accounting export",
                )),
            )
        })?;

    match export {
        Some(e) => Ok(Json(e)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new(
                "NOT_FOUND",
                "Accounting export not found",
            )),
        )),
    }
}

/// Download an accounting export file.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/accounting/exports/{id}/download",
    params(ResourceIdPath),
    responses(
        (status = 200, description = "File downloaded"),
        (status = 404, description = "Export not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn download_accounting_export(
    State(_state): State<AppState>,
    Path(_path): Path<ResourceIdPath>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implement file download
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(ErrorResponse::new(
            "NOT_IMPLEMENTED",
            "File download not yet implemented",
        )),
    ))
}

/// Get accounting export settings.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/organizations/{org_id}/accounting/settings/{system}",
    params(AccountingSystemPath),
    responses(
        (status = 200, description = "Settings retrieved", body = AccountingExportSettings),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn get_accounting_settings(
    State(state): State<AppState>,
    Path(path): Path<AccountingSystemPath>,
) -> Result<Json<AccountingExportSettings>, (StatusCode, Json<ErrorResponse>)> {
    let settings = state
        .integration_repo
        .get_accounting_export_settings(path.org_id, &path.system)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to get accounting settings");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to get accounting settings",
                )),
            )
        })?;

    Ok(Json(settings))
}

/// Update accounting export settings.
#[utoipa::path(
    put,
    path = "/api/v1/integrations/organizations/{org_id}/accounting/settings/{system}",
    params(AccountingSystemPath),
    request_body = UpdateAccountingExportSettings,
    responses(
        (status = 200, description = "Settings updated", body = AccountingExportSettings),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn update_accounting_settings(
    State(state): State<AppState>,
    Path(path): Path<AccountingSystemPath>,
    Json(data): Json<UpdateAccountingExportSettings>,
) -> Result<Json<AccountingExportSettings>, (StatusCode, Json<ErrorResponse>)> {
    let settings = state
        .integration_repo
        .update_accounting_export_settings(path.org_id, &path.system, data)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to update accounting settings");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to update accounting settings",
                )),
            )
        })?;

    Ok(Json(settings))
}

// ==================== E-Signature (Story 61.3) ====================

/// List e-signature workflows for an organization.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/organizations/{org_id}/esignatures",
    params(OrgIdPath, ESignatureQuery),
    responses(
        (status = 200, description = "Workflows retrieved", body = Vec<ESignatureWorkflow>),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn list_esignature_workflows(
    State(state): State<AppState>,
    Path(path): Path<OrgIdPath>,
    Query(query): Query<ESignatureQuery>,
) -> Result<Json<Vec<ESignatureWorkflow>>, (StatusCode, Json<ErrorResponse>)> {
    let workflows = state
        .integration_repo
        .list_esignature_workflows(path.org_id, query.status.as_deref(), query.limit)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list e-signature workflows");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to list e-signature workflows",
                )),
            )
        })?;

    Ok(Json(workflows))
}

/// Create an e-signature workflow.
#[utoipa::path(
    post,
    path = "/api/v1/integrations/organizations/{org_id}/esignatures",
    params(OrgIdPath),
    request_body = CreateESignatureWorkflow,
    responses(
        (status = 201, description = "Workflow created", body = ESignatureWorkflowWithRecipients),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn create_esignature_workflow(
    State(_state): State<AppState>,
    Path(_path): Path<OrgIdPath>,
    Json(_data): Json<CreateESignatureWorkflow>,
) -> Result<(StatusCode, Json<ESignatureWorkflowWithRecipients>), (StatusCode, Json<ErrorResponse>)>
{
    // TODO: Extract user_id from authentication context
    Err((
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse::new(
            "UNAUTHORIZED",
            "Authentication required",
        )),
    ))
}

/// Get an e-signature workflow by ID.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/esignatures/{id}",
    params(ResourceIdPath),
    responses(
        (status = 200, description = "Workflow retrieved", body = ESignatureWorkflowWithRecipients),
        (status = 404, description = "Workflow not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn get_esignature_workflow(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
) -> Result<Json<ESignatureWorkflowWithRecipients>, (StatusCode, Json<ErrorResponse>)> {
    let workflow = state
        .integration_repo
        .get_esignature_workflow_with_recipients(path.id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to get e-signature workflow");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to get e-signature workflow",
                )),
            )
        })?;

    match workflow {
        Some(w) => Ok(Json(w)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new(
                "NOT_FOUND",
                "E-signature workflow not found",
            )),
        )),
    }
}

/// Send an e-signature workflow for signing.
#[utoipa::path(
    post,
    path = "/api/v1/integrations/esignatures/{id}/send",
    params(ResourceIdPath),
    responses(
        (status = 200, description = "Workflow sent"),
        (status = 404, description = "Workflow not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn send_esignature_workflow(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
) -> Result<Json<ESignatureWorkflow>, (StatusCode, Json<ErrorResponse>)> {
    let workflow = state
        .integration_repo
        .update_esignature_workflow_status(path.id, "sent")
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to send e-signature workflow");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to send e-signature workflow",
                )),
            )
        })?;

    Ok(Json(workflow))
}

/// Void an e-signature workflow.
#[utoipa::path(
    post,
    path = "/api/v1/integrations/esignatures/{id}/void",
    params(ResourceIdPath),
    responses(
        (status = 200, description = "Workflow voided"),
        (status = 404, description = "Workflow not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn void_esignature_workflow(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
) -> Result<Json<ESignatureWorkflow>, (StatusCode, Json<ErrorResponse>)> {
    let workflow = state
        .integration_repo
        .update_esignature_workflow_status(path.id, "voided")
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to void e-signature workflow");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to void e-signature workflow",
                )),
            )
        })?;

    Ok(Json(workflow))
}

/// Send reminder for e-signature workflow.
#[utoipa::path(
    post,
    path = "/api/v1/integrations/esignatures/{id}/remind",
    params(ResourceIdPath),
    responses(
        (status = 200, description = "Reminder sent"),
        (status = 404, description = "Workflow not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn send_esignature_reminder(
    State(_state): State<AppState>,
    Path(_path): Path<ResourceIdPath>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implement reminder sending
    Ok(StatusCode::OK)
}

/// E-signature webhook endpoint.
#[utoipa::path(
    post,
    path = "/api/v1/integrations/esignatures/webhook",
    responses(
        (status = 200, description = "Webhook processed"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Integrations"
)]
pub async fn esignature_webhook(
    State(_state): State<AppState>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Process e-signature provider webhooks
    Ok(StatusCode::OK)
}

// ==================== Video Conferencing (Story 61.4) ====================

/// List video conference connections.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/organizations/{org_id}/video/connections",
    params(OrgIdPath, CalendarQuery),
    responses(
        (status = 200, description = "Connections retrieved", body = Vec<VideoConferenceConnection>),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn list_video_connections(
    State(state): State<AppState>,
    Path(path): Path<OrgIdPath>,
    Query(query): Query<CalendarQuery>,
) -> Result<Json<Vec<VideoConferenceConnection>>, (StatusCode, Json<ErrorResponse>)> {
    let connections = state
        .integration_repo
        .list_video_conference_connections(path.org_id, query.user_id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list video connections");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to list video connections",
                )),
            )
        })?;

    Ok(Json(connections))
}

/// Create a video conference connection.
#[utoipa::path(
    post,
    path = "/api/v1/integrations/organizations/{org_id}/video/connections",
    params(OrgIdPath),
    request_body = CreateVideoConferenceConnection,
    responses(
        (status = 201, description = "Connection created", body = VideoConferenceConnection),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn create_video_connection(
    State(_state): State<AppState>,
    Path(_path): Path<OrgIdPath>,
    Json(_data): Json<CreateVideoConferenceConnection>,
) -> Result<(StatusCode, Json<VideoConferenceConnection>), (StatusCode, Json<ErrorResponse>)> {
    // TODO: Extract user_id from authentication context
    Err((
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse::new(
            "UNAUTHORIZED",
            "Authentication required",
        )),
    ))
}

/// Delete a video conference connection.
#[utoipa::path(
    delete,
    path = "/api/v1/integrations/video/connections/{id}",
    params(ResourceIdPath),
    responses(
        (status = 204, description = "Connection deleted"),
        (status = 404, description = "Connection not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn delete_video_connection(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let deleted = state
        .integration_repo
        .delete_video_conference_connection(path.id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to delete video connection");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to delete video connection",
                )),
            )
        })?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new(
                "NOT_FOUND",
                "Video connection not found",
            )),
        ))
    }
}

/// List video meetings.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/organizations/{org_id}/video/meetings",
    params(OrgIdPath, VideoMeetingQuery),
    responses(
        (status = 200, description = "Meetings retrieved", body = Vec<VideoMeeting>),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn list_video_meetings(
    State(state): State<AppState>,
    Path(path): Path<OrgIdPath>,
    Query(query): Query<VideoMeetingQuery>,
) -> Result<Json<Vec<VideoMeeting>>, (StatusCode, Json<ErrorResponse>)> {
    let meetings = state
        .integration_repo
        .list_video_meetings(
            path.org_id,
            query.from,
            query.status.as_deref(),
            query.limit,
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list video meetings");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to list video meetings",
                )),
            )
        })?;

    Ok(Json(meetings))
}

/// Create a video meeting.
#[utoipa::path(
    post,
    path = "/api/v1/integrations/organizations/{org_id}/video/meetings",
    params(OrgIdPath),
    request_body = CreateVideoMeeting,
    responses(
        (status = 201, description = "Meeting created", body = VideoMeeting),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn create_video_meeting(
    State(_state): State<AppState>,
    Path(_path): Path<OrgIdPath>,
    Json(_data): Json<CreateVideoMeeting>,
) -> Result<(StatusCode, Json<VideoMeeting>), (StatusCode, Json<ErrorResponse>)> {
    // TODO: Extract user_id from authentication context
    Err((
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse::new(
            "UNAUTHORIZED",
            "Authentication required",
        )),
    ))
}

/// Get a video meeting by ID.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/video/meetings/{id}",
    params(ResourceIdPath),
    responses(
        (status = 200, description = "Meeting retrieved", body = VideoMeeting),
        (status = 404, description = "Meeting not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn get_video_meeting(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
) -> Result<Json<VideoMeeting>, (StatusCode, Json<ErrorResponse>)> {
    let meeting = state
        .integration_repo
        .get_video_meeting(path.id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to get video meeting");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to get video meeting",
                )),
            )
        })?;

    match meeting {
        Some(m) => Ok(Json(m)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Video meeting not found")),
        )),
    }
}

/// Update a video meeting.
#[utoipa::path(
    put,
    path = "/api/v1/integrations/video/meetings/{id}",
    params(ResourceIdPath),
    request_body = UpdateVideoMeeting,
    responses(
        (status = 200, description = "Meeting updated", body = VideoMeeting),
        (status = 404, description = "Meeting not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn update_video_meeting(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
    Json(data): Json<UpdateVideoMeeting>,
) -> Result<Json<VideoMeeting>, (StatusCode, Json<ErrorResponse>)> {
    let meeting = state
        .integration_repo
        .update_video_meeting(path.id, data)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to update video meeting");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to update video meeting",
                )),
            )
        })?;

    Ok(Json(meeting))
}

/// Delete a video meeting.
#[utoipa::path(
    delete,
    path = "/api/v1/integrations/video/meetings/{id}",
    params(ResourceIdPath),
    responses(
        (status = 204, description = "Meeting deleted"),
        (status = 404, description = "Meeting not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn delete_video_meeting(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let deleted = state
        .integration_repo
        .delete_video_meeting(path.id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to delete video meeting");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to delete video meeting",
                )),
            )
        })?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Video meeting not found")),
        ))
    }
}

/// Start a video meeting.
#[utoipa::path(
    post,
    path = "/api/v1/integrations/video/meetings/{id}/start",
    params(ResourceIdPath),
    responses(
        (status = 200, description = "Meeting started", body = VideoMeeting),
        (status = 404, description = "Meeting not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn start_video_meeting(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
) -> Result<Json<VideoMeeting>, (StatusCode, Json<ErrorResponse>)> {
    let meeting = state
        .integration_repo
        .update_video_meeting(
            path.id,
            UpdateVideoMeeting {
                status: Some("started".to_string()),
                ..Default::default()
            },
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to start video meeting");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to start video meeting",
                )),
            )
        })?;

    Ok(Json(meeting))
}

// ==================== Webhooks (Story 61.5) ====================

/// List webhook subscriptions for an organization.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/organizations/{org_id}/webhooks",
    params(OrgIdPath),
    responses(
        (status = 200, description = "Subscriptions retrieved", body = Vec<WebhookSubscription>),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn list_webhook_subscriptions(
    State(state): State<AppState>,
    Path(path): Path<OrgIdPath>,
) -> Result<Json<Vec<WebhookSubscription>>, (StatusCode, Json<ErrorResponse>)> {
    let subscriptions = state
        .integration_repo
        .list_webhook_subscriptions(path.org_id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list webhook subscriptions");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to list webhook subscriptions",
                )),
            )
        })?;

    Ok(Json(subscriptions))
}

/// Create a webhook subscription.
#[utoipa::path(
    post,
    path = "/api/v1/integrations/organizations/{org_id}/webhooks",
    params(OrgIdPath),
    request_body = CreateWebhookSubscription,
    responses(
        (status = 201, description = "Subscription created", body = WebhookSubscription),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn create_webhook_subscription(
    State(_state): State<AppState>,
    Path(_path): Path<OrgIdPath>,
    Json(_data): Json<CreateWebhookSubscription>,
) -> Result<(StatusCode, Json<WebhookSubscription>), (StatusCode, Json<ErrorResponse>)> {
    // TODO: Extract user_id from authentication context
    Err((
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse::new(
            "UNAUTHORIZED",
            "Authentication required",
        )),
    ))
}

/// Get a webhook subscription by ID.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/webhooks/{id}",
    params(ResourceIdPath),
    responses(
        (status = 200, description = "Subscription retrieved", body = WebhookSubscription),
        (status = 404, description = "Subscription not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn get_webhook_subscription(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
) -> Result<Json<WebhookSubscription>, (StatusCode, Json<ErrorResponse>)> {
    let subscription = state
        .integration_repo
        .get_webhook_subscription(path.id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to get webhook subscription");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to get webhook subscription",
                )),
            )
        })?;

    match subscription {
        Some(s) => Ok(Json(s)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new(
                "NOT_FOUND",
                "Webhook subscription not found",
            )),
        )),
    }
}

/// Update a webhook subscription.
#[utoipa::path(
    put,
    path = "/api/v1/integrations/webhooks/{id}",
    params(ResourceIdPath),
    request_body = UpdateWebhookSubscription,
    responses(
        (status = 200, description = "Subscription updated", body = WebhookSubscription),
        (status = 404, description = "Subscription not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn update_webhook_subscription(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
    Json(data): Json<UpdateWebhookSubscription>,
) -> Result<Json<WebhookSubscription>, (StatusCode, Json<ErrorResponse>)> {
    let subscription = state
        .integration_repo
        .update_webhook_subscription(path.id, data)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to update webhook subscription");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to update webhook subscription",
                )),
            )
        })?;

    Ok(Json(subscription))
}

/// Delete a webhook subscription.
#[utoipa::path(
    delete,
    path = "/api/v1/integrations/webhooks/{id}",
    params(ResourceIdPath),
    responses(
        (status = 204, description = "Subscription deleted"),
        (status = 404, description = "Subscription not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn delete_webhook_subscription(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let deleted = state
        .integration_repo
        .delete_webhook_subscription(path.id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to delete webhook subscription");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to delete webhook subscription",
                )),
            )
        })?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new(
                "NOT_FOUND",
                "Webhook subscription not found",
            )),
        ))
    }
}

/// Test a webhook subscription.
#[utoipa::path(
    post,
    path = "/api/v1/integrations/webhooks/{id}/test",
    params(ResourceIdPath),
    request_body = TestWebhookRequest,
    responses(
        (status = 200, description = "Test completed", body = TestWebhookResponse),
        (status = 404, description = "Subscription not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn test_webhook(
    State(_state): State<AppState>,
    Path(_path): Path<ResourceIdPath>,
    Json(_data): Json<TestWebhookRequest>,
) -> Result<Json<TestWebhookResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implement webhook testing
    Ok(Json(TestWebhookResponse {
        success: true,
        status_code: Some(200),
        response_time_ms: Some(150),
        error: None,
    }))
}

/// List webhook delivery logs.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/webhooks/{id}/logs",
    params(ResourceIdPath),
    responses(
        (status = 200, description = "Logs retrieved", body = Vec<WebhookDeliveryLog>),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn list_webhook_logs(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
) -> Result<Json<Vec<WebhookDeliveryLog>>, (StatusCode, Json<ErrorResponse>)> {
    let logs = state
        .integration_repo
        .list_webhook_delivery_logs(WebhookDeliveryQuery {
            subscription_id: Some(path.id),
            event_type: None,
            status: None,
            from_date: None,
            to_date: None,
            limit: Some(100),
            offset: None,
        })
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list webhook logs");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to list webhook logs",
                )),
            )
        })?;

    Ok(Json(logs))
}

/// Get webhook statistics.
#[utoipa::path(
    get,
    path = "/api/v1/integrations/webhooks/{id}/stats",
    params(ResourceIdPath),
    responses(
        (status = 200, description = "Statistics retrieved", body = WebhookStatistics),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Integrations"
)]
pub async fn get_webhook_stats(
    State(state): State<AppState>,
    Path(path): Path<ResourceIdPath>,
) -> Result<Json<WebhookStatistics>, (StatusCode, Json<ErrorResponse>)> {
    let stats = state
        .integration_repo
        .get_webhook_statistics(path.id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to get webhook statistics");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to get webhook statistics",
                )),
            )
        })?;

    Ok(Json(stats))
}
