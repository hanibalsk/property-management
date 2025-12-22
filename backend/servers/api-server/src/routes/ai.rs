//! AI routes (Epic 13: AI Assistant & Automation).
//!
//! Handles AI chat, sentiment analysis, equipment/maintenance, and workflows.

use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use common::errors::ErrorResponse;
use db::models::{
    CreateChatSession, CreateEquipment, CreateMaintenance, CreateWorkflow, CreateWorkflowAction,
    EquipmentQuery, ExecutionQuery, ProvideFeedback, SendChatMessage, SentimentTrendQuery,
    TriggerWorkflow, UpdateEquipment, UpdateMaintenance, UpdateSentimentThresholds, UpdateWorkflow,
    WorkflowQuery,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// Response Types
// ============================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

// ============================================================================
// Query Types
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Default, utoipa::IntoParams)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default, utoipa::IntoParams)]
pub struct AlertsQuery {
    pub acknowledged: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// ============================================================================
// AI Chat Router (Story 13.1)
// ============================================================================

pub fn ai_chat_router() -> Router<AppState> {
    Router::new()
        .route("/sessions", post(create_session))
        .route("/sessions", get(list_sessions))
        .route("/sessions/{session_id}", get(get_session))
        .route("/sessions/{session_id}", delete(delete_session))
        .route("/sessions/{session_id}/messages", get(list_messages))
        .route("/sessions/{session_id}/messages", post(send_message))
        .route("/messages/{message_id}/feedback", post(provide_feedback))
        .route("/escalated", get(list_escalated))
}

#[utoipa::path(
    post,
    path = "/api/v1/ai/chat/sessions",
    request_body = CreateChatSession,
    responses(
        (status = 201, description = "Session created"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "AI Chat"
)]
async fn create_session(
    State(state): State<AppState>,
    Json(req): Json<CreateChatSession>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<ErrorResponse>)> {
    match state.ai_chat_repo.create_session(req).await {
        Ok(session) => Ok((StatusCode::CREATED, Json(serde_json::json!(session)))),
        Err(e) => {
            tracing::error!("Failed to create session: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to create session",
                )),
            ))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/ai/chat/sessions",
    params(PaginationQuery),
    responses(
        (status = 200, description = "Sessions list"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "AI Chat"
)]
async fn list_sessions(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let user_id = Uuid::nil();

    match state
        .ai_chat_repo
        .list_user_sessions(
            user_id,
            query.limit.unwrap_or(50),
            query.offset.unwrap_or(0),
        )
        .await
    {
        Ok(sessions) => Ok(Json(serde_json::json!({ "sessions": sessions }))),
        Err(e) => {
            tracing::error!("Failed to list sessions: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to list sessions",
                )),
            ))
        }
    }
}

async fn get_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match state.ai_chat_repo.find_session_by_id(session_id).await {
        Ok(Some(session)) => Ok(Json(serde_json::json!(session))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Session not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to get session: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to get session",
                )),
            ))
        }
    }
}

async fn delete_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    match state.ai_chat_repo.delete_session(session_id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Session not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to delete session: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to delete session",
                )),
            ))
        }
    }
}

async fn list_messages(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match state
        .ai_chat_repo
        .list_session_messages(
            session_id,
            query.limit.unwrap_or(100),
            query.offset.unwrap_or(0),
        )
        .await
    {
        Ok(messages) => Ok(Json(serde_json::json!({ "messages": messages }))),
        Err(e) => {
            tracing::error!("Failed to list messages: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to list messages",
                )),
            ))
        }
    }
}

async fn send_message(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    Json(req): Json<SendChatMessage>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<ErrorResponse>)> {
    // Add user message
    let user_msg = state
        .ai_chat_repo
        .add_message(
            session_id,
            "user",
            &req.content,
            None,
            vec![],
            false,
            None,
            None,
            None,
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to add user message: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to send message",
                )),
            )
        })?;

    // TODO: Process with AI and add assistant response
    // For now, return a placeholder response
    let assistant_msg = state
        .ai_chat_repo
        .add_message(
            session_id,
            "assistant",
            "I'm the AI assistant. This is a placeholder response. Real AI integration coming soon!",
            Some(0.95),
            vec![],
            false,
            None,
            Some(100),
            Some(50),
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to add assistant message: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to process message")),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "user_message": user_msg,
            "assistant_message": assistant_msg
        })),
    ))
}

async fn provide_feedback(
    State(state): State<AppState>,
    Path(message_id): Path<Uuid>,
    Json(req): Json<ProvideFeedback>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<ErrorResponse>)> {
    let mut feedback = req;
    feedback.message_id = message_id;

    match state.ai_chat_repo.add_feedback(feedback).await {
        Ok(fb) => Ok((StatusCode::CREATED, Json(serde_json::json!(fb)))),
        Err(e) => {
            tracing::error!("Failed to add feedback: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to add feedback",
                )),
            ))
        }
    }
}

async fn list_escalated(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let org_id = Uuid::nil();

    match state
        .ai_chat_repo
        .list_escalated_messages(org_id, query.limit.unwrap_or(50), query.offset.unwrap_or(0))
        .await
    {
        Ok(messages) => Ok(Json(serde_json::json!({ "messages": messages }))),
        Err(e) => {
            tracing::error!("Failed to list escalated: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to list escalated",
                )),
            ))
        }
    }
}

// ============================================================================
// Sentiment Router (Story 13.2)
// ============================================================================

pub fn sentiment_router() -> Router<AppState> {
    Router::new()
        .route("/trends", get(get_trends))
        .route("/alerts", get(list_alerts))
        .route("/alerts/{alert_id}/acknowledge", post(acknowledge_alert))
        .route("/thresholds", get(get_thresholds))
        .route("/thresholds", put(update_thresholds))
        .route("/dashboard", get(get_dashboard))
}

async fn get_trends(
    State(state): State<AppState>,
    Query(query): Query<SentimentTrendQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let org_id = Uuid::nil();

    match state.sentiment_repo.list_trends(org_id, query).await {
        Ok(trends) => Ok(Json(serde_json::json!({ "trends": trends }))),
        Err(e) => {
            tracing::error!("Failed to get trends: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to get trends")),
            ))
        }
    }
}

async fn list_alerts(
    State(state): State<AppState>,
    Query(query): Query<AlertsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let org_id = Uuid::nil();

    match state
        .sentiment_repo
        .list_alerts(
            org_id,
            query.acknowledged,
            query.limit.unwrap_or(50),
            query.offset.unwrap_or(0),
        )
        .await
    {
        Ok(alerts) => Ok(Json(serde_json::json!({ "alerts": alerts }))),
        Err(e) => {
            tracing::error!("Failed to list alerts: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to list alerts",
                )),
            ))
        }
    }
}

async fn acknowledge_alert(
    State(state): State<AppState>,
    Path(alert_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let user_id = Uuid::nil();

    match state
        .sentiment_repo
        .acknowledge_alert(alert_id, user_id)
        .await
    {
        Ok(alert) => Ok(Json(serde_json::json!(alert))),
        Err(e) => {
            tracing::error!("Failed to acknowledge alert: {}", e);
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

async fn get_thresholds(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let org_id = Uuid::nil();

    match state.sentiment_repo.get_thresholds(org_id).await {
        Ok(thresholds) => Ok(Json(serde_json::json!(thresholds))),
        Err(e) => {
            tracing::error!("Failed to get thresholds: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to get thresholds",
                )),
            ))
        }
    }
}

async fn update_thresholds(
    State(state): State<AppState>,
    Json(req): Json<UpdateSentimentThresholds>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let org_id = Uuid::nil();

    match state.sentiment_repo.update_thresholds(org_id, req).await {
        Ok(thresholds) => Ok(Json(serde_json::json!(thresholds))),
        Err(e) => {
            tracing::error!("Failed to update thresholds: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to update")),
            ))
        }
    }
}

async fn get_dashboard(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let org_id = Uuid::nil();
    let today = chrono::Utc::now().date_naive();
    let thirty_days_ago = today - chrono::Duration::days(30);

    let org_avg = state
        .sentiment_repo
        .get_org_average_sentiment(org_id, thirty_days_ago, today)
        .await
        .unwrap_or(0.0);

    let trends = state
        .sentiment_repo
        .list_trends(org_id, SentimentTrendQuery::default())
        .await
        .unwrap_or_default();

    let alerts = state
        .sentiment_repo
        .list_alerts(org_id, Some(false), 5, 0)
        .await
        .unwrap_or_default();

    Ok(Json(serde_json::json!({
        "organization_avg": org_avg,
        "trends": trends,
        "recent_alerts": alerts
    })))
}

// ============================================================================
// Equipment Router (Story 13.3)
// ============================================================================

pub fn equipment_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_equipment))
        .route("/", get(list_equipment))
        .route("/{id}", get(get_equipment))
        .route("/{id}", put(update_equipment))
        .route("/{id}", delete(delete_equipment))
        .route("/{id}/maintenance", get(list_maintenance))
        .route("/{id}/maintenance", post(create_maintenance))
        .route("/maintenance/{id}", put(update_maintenance))
        .route("/predictions", get(list_predictions))
        .route(
            "/predictions/{id}/acknowledge",
            post(acknowledge_prediction),
        )
        .route("/needing-maintenance", get(list_needing_maintenance))
}

async fn create_equipment(
    State(state): State<AppState>,
    Json(req): Json<CreateEquipment>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<ErrorResponse>)> {
    match state.equipment_repo.create(req).await {
        Ok(equipment) => Ok((StatusCode::CREATED, Json(serde_json::json!(equipment)))),
        Err(e) => {
            tracing::error!("Failed to create equipment: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to create")),
            ))
        }
    }
}

async fn list_equipment(
    State(state): State<AppState>,
    Query(query): Query<EquipmentQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let org_id = Uuid::nil();

    match state.equipment_repo.list(org_id, query).await {
        Ok(equipment) => Ok(Json(serde_json::json!({ "equipment": equipment }))),
        Err(e) => {
            tracing::error!("Failed to list equipment: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to list")),
            ))
        }
    }
}

async fn get_equipment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match state.equipment_repo.find_by_id(id).await {
        Ok(Some(equipment)) => Ok(Json(serde_json::json!(equipment))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Equipment not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to get equipment: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to get")),
            ))
        }
    }
}

async fn update_equipment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateEquipment>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match state.equipment_repo.update(id, req).await {
        Ok(equipment) => Ok(Json(serde_json::json!(equipment))),
        Err(e) => {
            tracing::error!("Failed to update equipment: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to update")),
            ))
        }
    }
}

async fn delete_equipment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    match state.equipment_repo.delete(id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Equipment not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to delete equipment: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to delete")),
            ))
        }
    }
}

async fn list_maintenance(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match state
        .equipment_repo
        .list_maintenance(id, query.limit.unwrap_or(50), query.offset.unwrap_or(0))
        .await
    {
        Ok(records) => Ok(Json(serde_json::json!({ "maintenance": records }))),
        Err(e) => {
            tracing::error!("Failed to list maintenance: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to list")),
            ))
        }
    }
}

async fn create_maintenance(
    State(state): State<AppState>,
    Path(_id): Path<Uuid>,
    Json(req): Json<CreateMaintenance>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<ErrorResponse>)> {
    match state.equipment_repo.create_maintenance(req).await {
        Ok(record) => Ok((StatusCode::CREATED, Json(serde_json::json!(record)))),
        Err(e) => {
            tracing::error!("Failed to create maintenance: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to create")),
            ))
        }
    }
}

async fn update_maintenance(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateMaintenance>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match state.equipment_repo.update_maintenance(id, req).await {
        Ok(record) => Ok(Json(serde_json::json!(record))),
        Err(e) => {
            tracing::error!("Failed to update maintenance: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to update")),
            ))
        }
    }
}

async fn list_predictions(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let org_id = Uuid::nil();

    match state
        .equipment_repo
        .list_high_risk_predictions(org_id, 50.0, query.limit.unwrap_or(20))
        .await
    {
        Ok(predictions) => Ok(Json(serde_json::json!({ "predictions": predictions }))),
        Err(e) => {
            tracing::error!("Failed to list predictions: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to list")),
            ))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcknowledgePredictionRequest {
    pub action_taken: Option<String>,
}

async fn acknowledge_prediction(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<AcknowledgePredictionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let user_id = Uuid::nil();

    match state
        .equipment_repo
        .acknowledge_prediction(id, user_id, req.action_taken.as_deref())
        .await
    {
        Ok(prediction) => Ok(Json(serde_json::json!(prediction))),
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

#[derive(Debug, Serialize, Deserialize, Default, utoipa::IntoParams)]
pub struct MaintenanceDueQuery {
    pub days_ahead: Option<i32>,
    pub limit: Option<i64>,
}

async fn list_needing_maintenance(
    State(state): State<AppState>,
    Query(query): Query<MaintenanceDueQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let org_id = Uuid::nil();

    match state
        .equipment_repo
        .list_needing_maintenance(
            org_id,
            query.days_ahead.unwrap_or(30),
            query.limit.unwrap_or(20),
        )
        .await
    {
        Ok(equipment) => Ok(Json(serde_json::json!({ "equipment": equipment }))),
        Err(e) => {
            tracing::error!("Failed to list: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to list")),
            ))
        }
    }
}

// ============================================================================
// Workflow Router (Story 13.6 & 13.7)
// ============================================================================

pub fn workflow_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_workflow))
        .route("/", get(list_workflows))
        .route("/{id}", get(get_workflow))
        .route("/{id}", put(update_workflow))
        .route("/{id}", delete(delete_workflow))
        .route("/{id}/actions", get(list_actions))
        .route("/{id}/actions", post(add_action))
        .route("/actions/{action_id}", delete(delete_action))
        .route("/{id}/trigger", post(trigger_workflow))
        .route("/executions", get(list_executions))
        .route("/executions/{id}", get(get_execution))
        .route("/executions/{id}/steps", get(list_execution_steps))
}

async fn create_workflow(
    State(state): State<AppState>,
    Json(req): Json<CreateWorkflow>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<ErrorResponse>)> {
    match state.workflow_repo.create(req).await {
        Ok(workflow) => Ok((StatusCode::CREATED, Json(serde_json::json!(workflow)))),
        Err(e) => {
            tracing::error!("Failed to create workflow: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to create")),
            ))
        }
    }
}

async fn list_workflows(
    State(state): State<AppState>,
    Query(query): Query<WorkflowQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let org_id = Uuid::nil();

    match state.workflow_repo.list(org_id, query).await {
        Ok(workflows) => Ok(Json(serde_json::json!({ "workflows": workflows }))),
        Err(e) => {
            tracing::error!("Failed to list workflows: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to list")),
            ))
        }
    }
}

async fn get_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match state.workflow_repo.find_by_id(id).await {
        Ok(Some(workflow)) => {
            let actions = state
                .workflow_repo
                .list_actions(id)
                .await
                .unwrap_or_default();
            Ok(Json(serde_json::json!({
                "workflow": workflow,
                "actions": actions
            })))
        }
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Workflow not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to get workflow: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to get")),
            ))
        }
    }
}

async fn update_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateWorkflow>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match state.workflow_repo.update(id, req).await {
        Ok(workflow) => Ok(Json(serde_json::json!(workflow))),
        Err(e) => {
            tracing::error!("Failed to update workflow: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to update")),
            ))
        }
    }
}

async fn delete_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    match state.workflow_repo.delete(id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Workflow not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to delete workflow: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to delete")),
            ))
        }
    }
}

async fn list_actions(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match state.workflow_repo.list_actions(id).await {
        Ok(actions) => Ok(Json(serde_json::json!({ "actions": actions }))),
        Err(e) => {
            tracing::error!("Failed to list actions: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to list")),
            ))
        }
    }
}

async fn add_action(
    State(state): State<AppState>,
    Path(_id): Path<Uuid>,
    Json(req): Json<CreateWorkflowAction>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<ErrorResponse>)> {
    match state.workflow_repo.add_action(req).await {
        Ok(action) => Ok((StatusCode::CREATED, Json(serde_json::json!(action)))),
        Err(e) => {
            tracing::error!("Failed to add action: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to add")),
            ))
        }
    }
}

async fn delete_action(
    State(state): State<AppState>,
    Path(action_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    match state.workflow_repo.delete_action(action_id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Action not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to delete action: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to delete")),
            ))
        }
    }
}

async fn trigger_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(mut req): Json<TriggerWorkflow>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<ErrorResponse>)> {
    req.workflow_id = id;

    match state.workflow_repo.create_execution(req).await {
        Ok(execution) => {
            // TODO: Actually execute the workflow asynchronously
            Ok((StatusCode::CREATED, Json(serde_json::json!(execution))))
        }
        Err(e) => {
            tracing::error!("Failed to trigger workflow: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to trigger")),
            ))
        }
    }
}

async fn list_executions(
    State(state): State<AppState>,
    Query(query): Query<ExecutionQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Get from auth context
    let org_id = Uuid::nil();

    match state.workflow_repo.list_executions(org_id, query).await {
        Ok(executions) => Ok(Json(serde_json::json!({ "executions": executions }))),
        Err(e) => {
            tracing::error!("Failed to list executions: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to list")),
            ))
        }
    }
}

async fn get_execution(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match state.workflow_repo.find_execution_by_id(id).await {
        Ok(Some(execution)) => {
            let steps = state
                .workflow_repo
                .list_execution_steps(id)
                .await
                .unwrap_or_default();
            Ok(Json(serde_json::json!({
                "execution": execution,
                "steps": steps
            })))
        }
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Execution not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to get execution: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to get")),
            ))
        }
    }
}

async fn list_execution_steps(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match state.workflow_repo.list_execution_steps(id).await {
        Ok(steps) => Ok(Json(serde_json::json!({ "steps": steps }))),
        Err(e) => {
            tracing::error!("Failed to list steps: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to list")),
            ))
        }
    }
}
