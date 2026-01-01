//! AI routes (Epic 13: AI Assistant & Automation).
//!
//! Handles AI chat, sentiment analysis, equipment/maintenance, and workflows.
//! Epic 91: Wired to actual LLM providers (OpenAI/Anthropic).

use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    routing::{delete, get, post, put},
    Json, Router,
};
use common::{errors::ErrorResponse, TenantContext};
use db::models::{
    CreateChatSession, CreateEquipment, CreateMaintenance, CreateWorkflow, CreateWorkflowAction,
    EquipmentQuery, ExecutionQuery, ProvideFeedback, SendChatMessage, SentimentTrendQuery,
    TriggerWorkflow, UpdateEquipment, UpdateMaintenance, UpdateSentimentThresholds, UpdateWorkflow,
    WorkflowQuery,
};
use integrations::{ChatCompletionRequest, ChatMessage, ContextChunk};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// Helper Functions
// ============================================================================

/// Extract tenant context from request headers.
fn extract_tenant_context(
    headers: &HeaderMap,
) -> Result<TenantContext, (StatusCode, Json<ErrorResponse>)> {
    let tenant_header = headers
        .get("X-Tenant-Context")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse::new(
                    "MISSING_CONTEXT",
                    "Tenant context required",
                )),
            )
        })?;

    serde_json::from_str(tenant_header).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                "INVALID_CONTEXT",
                "Invalid tenant context format",
            )),
        )
    })
}

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
    headers: HeaderMap,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state
        .ai_chat_repo
        .list_user_sessions(
            tenant.user_id,
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

/// Default system prompt for the AI assistant.
const DEFAULT_SYSTEM_PROMPT: &str = r#"You are a helpful AI assistant for a property management system. You help users with:
- Building and property management questions
- Fault reporting and maintenance inquiries
- Voting and decision-making processes
- Document and announcement management
- Resident and owner questions

Be concise, professional, and helpful. If you're unsure about something or the question requires human expertise, acknowledge this and suggest escalation to building management.

If you don't have enough context to answer a question confidently, say so and ask for clarification."#;

async fn send_message(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(session_id): Path<Uuid>,
    Json(req): Json<SendChatMessage>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<ErrorResponse>)> {
    let start_time = Instant::now();

    // Get tenant context for RAG document search
    let tenant = extract_tenant_context(&headers).ok();

    // Add user message first
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

    // Verify session exists
    let _session = state
        .ai_chat_repo
        .find_session_by_id(session_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to find session: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to find session",
                )),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("NOT_FOUND", "Session not found")),
            )
        })?;

    // Get conversation history for context (last 10 messages)
    let history = state
        .ai_chat_repo
        .list_session_messages(session_id, 10, 0)
        .await
        .unwrap_or_default();

    // Build messages for LLM
    let mut messages: Vec<ChatMessage> = vec![ChatMessage {
        role: "system".to_string(),
        content: DEFAULT_SYSTEM_PROMPT.to_string(),
    }];

    // Add conversation history (skip the message we just added)
    for msg in history.iter().take(history.len().saturating_sub(1)) {
        messages.push(ChatMessage {
            role: msg.role.clone(),
            content: msg.content.clone(),
        });
    }

    // Search for relevant documents if we have tenant context (RAG)
    let mut context_chunks: Vec<ContextChunk> = vec![];
    if let Some(ref tenant_ctx) = tenant {
        if let Ok(docs) = state
            .llm_document_repo
            .search_documents_by_text(tenant_ctx.tenant_id, &req.content, 3)
            .await
        {
            for doc in docs {
                context_chunks.push(ContextChunk {
                    source_id: doc.document_id,
                    source_title: doc
                        .metadata
                        .get("title")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Document")
                        .to_string(),
                    text: doc.chunk_text.clone(),
                    relevance_score: 0.8, // Placeholder - real semantic search would provide this
                });
            }
        }
    }

    // Add context chunks to user message if available
    let user_content = if !context_chunks.is_empty() {
        let context_text: Vec<String> = context_chunks
            .iter()
            .map(|c| format!("[Source: {}]\n{}", c.source_title, c.text))
            .collect();
        format!(
            "Relevant building documents:\n{}\n\nUser question: {}",
            context_text.join("\n---\n"),
            req.content
        )
    } else {
        req.content.clone()
    };

    messages.push(ChatMessage {
        role: "user".to_string(),
        content: user_content,
    });

    // Determine provider and model (default to anthropic/claude-3-haiku for cost efficiency)
    let provider = std::env::var("LLM_PROVIDER").unwrap_or_else(|_| "anthropic".to_string());
    let model = std::env::var("LLM_MODEL").unwrap_or_else(|_| {
        if provider == "openai" {
            "gpt-4o-mini".to_string()
        } else {
            "claude-3-5-haiku-20241022".to_string()
        }
    });

    // Build LLM request
    let llm_request = ChatCompletionRequest {
        model: model.clone(),
        messages,
        temperature: Some(0.7),
        max_tokens: Some(1024),
    };

    // Call LLM
    let llm_result = state.llm_client.chat(&provider, &llm_request).await;

    let latency_ms = start_time.elapsed().as_millis() as i32;

    // Handle response
    let (response_content, confidence, escalated, escalation_reason, tokens_used) = match llm_result
    {
        Ok(response) => {
            let content = response
                .choices
                .first()
                .map(|c| c.message.content.clone())
                .unwrap_or_else(|| "I'm sorry, I couldn't generate a response.".to_string());

            // Simple escalation detection
            let needs_escalation = content.to_lowercase().contains("contact management")
                || content.to_lowercase().contains("escalat")
                || content.to_lowercase().contains("human assistance")
                || content.to_lowercase().contains("cannot answer");

            let escalation_reason = if needs_escalation {
                Some("Response indicates need for human assistance")
            } else {
                None
            };

            // Confidence based on whether we had context and if escalation was suggested
            let confidence = if needs_escalation {
                0.5
            } else if !context_chunks.is_empty() {
                0.9
            } else {
                0.75
            };

            (
                content,
                confidence,
                needs_escalation,
                escalation_reason,
                Some(response.usage.total_tokens),
            )
        }
        Err(e) => {
            tracing::warn!("LLM call failed: {}, falling back to placeholder", e);
            (
                format!(
                    "I'm sorry, I'm having trouble processing your request right now. Error: {}. Please try again later or contact building management for assistance.",
                    e
                ),
                0.3,
                true,
                Some("LLM service unavailable"),
                None,
            )
        }
    };

    // Build sources from context chunks
    let sources: Vec<serde_json::Value> = context_chunks
        .iter()
        .map(|c| {
            serde_json::json!({
                "source_id": c.source_id,
                "title": c.source_title,
                "relevance_score": c.relevance_score
            })
        })
        .collect();

    // Add assistant message
    let assistant_msg = state
        .ai_chat_repo
        .add_message(
            session_id,
            "assistant",
            &response_content,
            Some(confidence),
            sources,
            escalated,
            escalation_reason,
            tokens_used,
            Some(latency_ms),
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to add assistant message: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to process message",
                )),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "user_message": user_msg,
            "assistant_message": assistant_msg,
            "provider": provider,
            "model": model,
            "latency_ms": latency_ms
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
    headers: HeaderMap,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state
        .ai_chat_repo
        .list_escalated_messages(
            tenant.tenant_id,
            query.limit.unwrap_or(50),
            query.offset.unwrap_or(0),
        )
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
    headers: HeaderMap,
    Query(query): Query<SentimentTrendQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state
        .sentiment_repo
        .list_trends(tenant.tenant_id, query)
        .await
    {
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
    headers: HeaderMap,
    Query(query): Query<AlertsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state
        .sentiment_repo
        .list_alerts(
            tenant.tenant_id,
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
    headers: HeaderMap,
    Path(alert_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state
        .sentiment_repo
        .acknowledge_alert(alert_id, tenant.user_id)
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
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state.sentiment_repo.get_thresholds(tenant.tenant_id).await {
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
    headers: HeaderMap,
    Json(req): Json<UpdateSentimentThresholds>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state
        .sentiment_repo
        .update_thresholds(tenant.tenant_id, req)
        .await
    {
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
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;
    let org_id = tenant.tenant_id;
    let today = chrono::Utc::now().date_naive();
    let thirty_days_ago = today - chrono::Duration::days(30);

    let org_avg = state
        .sentiment_repo
        .get_org_average_sentiment(org_id, thirty_days_ago, today)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get org average sentiment: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to get dashboard",
                )),
            )
        })?;

    let trends = state
        .sentiment_repo
        .list_trends(org_id, SentimentTrendQuery::default())
        .await
        .map_err(|e| {
            tracing::error!("Failed to get trends: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to get dashboard",
                )),
            )
        })?;

    let alerts = state
        .sentiment_repo
        .list_alerts(org_id, Some(false), 5, 0)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get alerts: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to get dashboard",
                )),
            )
        })?;

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
    headers: HeaderMap,
    Query(query): Query<EquipmentQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state.equipment_repo.list(tenant.tenant_id, query).await {
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
    headers: HeaderMap,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state
        .equipment_repo
        .list_high_risk_predictions(tenant.tenant_id, 50.0, query.limit.unwrap_or(20))
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
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(req): Json<AcknowledgePredictionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state
        .equipment_repo
        .acknowledge_prediction(id, tenant.user_id, req.action_taken.as_deref())
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
    headers: HeaderMap,
    Query(query): Query<MaintenanceDueQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state
        .equipment_repo
        .list_needing_maintenance(
            tenant.tenant_id,
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
    headers: HeaderMap,
    Query(query): Query<WorkflowQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state.workflow_repo.list(tenant.tenant_id, query).await {
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
            let actions = state.workflow_repo.list_actions(id).await.map_err(|e| {
                tracing::error!("Failed to list actions: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to get")),
                )
            })?;
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
    headers: HeaderMap,
    Query(query): Query<ExecutionQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state
        .workflow_repo
        .list_executions(tenant.tenant_id, query)
        .await
    {
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
                .map_err(|e| {
                    tracing::error!("Failed to list steps: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to get")),
                    )
                })?;
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

// ============================================================================
// Epic 64: Advanced AI & LLM Capabilities
// ============================================================================

// Story 64.1: LLM-Powered Lease Agreement Generation
// Story 64.2: AI Property Listing Description Generator
// Story 64.3: Conversational AI Tenant Support (Enhanced RAG)
// Story 64.4: AI Photo Enhancement for Listings
// Story 64.5: Voice Assistant Integration

use db::models::{
    EnhancePhotoRequest, EnhancedChatRequest, GenerateLeaseRequest,
    GenerateListingDescriptionRequest, LinkVoiceDeviceRequest, UpdateEscalationConfig,
};

/// Router for LLM document generation (Epic 64).
pub fn llm_router() -> Router<AppState> {
    Router::new()
        // Lease generation (Story 64.1)
        .route("/lease/generate", post(generate_lease))
        .route("/lease/templates", get(list_lease_templates))
        .route("/lease/templates/{id}", get(get_lease_template))
        // Listing descriptions (Story 64.2)
        .route("/listing/description", post(generate_listing_description))
        .route(
            "/listing/descriptions/{listing_id}",
            get(list_listing_descriptions),
        )
        .route(
            "/listing/descriptions/{id}/publish",
            post(publish_description),
        )
        // Enhanced chat (Story 64.3)
        .route("/chat/enhanced", post(enhanced_chat))
        .route("/chat/escalation-config", get(get_escalation_config))
        .route("/chat/escalation-config", put(update_escalation_config))
        // Photo enhancement (Story 64.4)
        .route("/photos/enhance", post(enhance_photo))
        .route("/photos/enhance/batch", post(batch_enhance_photos))
        .route("/photos/{id}", get(get_photo_enhancement))
        // Voice assistant (Story 64.5)
        .route("/voice/devices", get(list_voice_devices))
        .route("/voice/devices", post(link_voice_device))
        .route("/voice/devices/{id}", delete(unlink_voice_device))
        .route("/voice/commands/{device_id}", get(list_voice_commands))
        // Statistics
        .route("/statistics", get(get_ai_statistics))
        .route("/requests", get(list_generation_requests))
        .route("/requests/{id}", get(get_generation_request))
}

// ============================================================================
// Story 64.1: Lease Generation Endpoints
// ============================================================================

#[utoipa::path(
    post,
    path = "/api/v1/ai/llm/lease/generate",
    request_body = GenerateLeaseRequest,
    responses(
        (status = 201, description = "Lease agreement generated"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "AI LLM"
)]
async fn generate_lease(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<GenerateLeaseRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    // Create a generation request record
    let input_data = serde_json::to_value(&req).unwrap_or_default();
    let request = state
        .llm_document_repo
        .create_generation_request(
            tenant.tenant_id,
            tenant.user_id,
            "lease_generation",
            "openai",
            "gpt-4",
            input_data,
            req.template_id,
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to create generation request: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to create request",
                )),
            )
        })?;

    // For now, return a placeholder response
    // Real implementation would call the LLM client
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "request_id": request.id,
            "status": request.status,
            "message": "Lease generation request created. Processing..."
        })),
    ))
}

async fn list_lease_templates(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state
        .llm_document_repo
        .list_prompt_templates(Some(tenant.tenant_id), Some("lease_generation"))
        .await
    {
        Ok(templates) => Ok(Json(serde_json::json!({ "templates": templates }))),
        Err(e) => {
            tracing::error!("Failed to list templates: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to list templates",
                )),
            ))
        }
    }
}

async fn get_lease_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match state.llm_document_repo.find_prompt_template(id).await {
        Ok(Some(template)) => Ok(Json(serde_json::json!(template))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Template not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to get template: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to get template",
                )),
            ))
        }
    }
}

// ============================================================================
// Story 64.2: Listing Description Endpoints
// ============================================================================

#[utoipa::path(
    post,
    path = "/api/v1/ai/llm/listing/description",
    request_body = GenerateListingDescriptionRequest,
    responses(
        (status = 201, description = "Description generated"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "AI LLM"
)]
async fn generate_listing_description(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<GenerateListingDescriptionRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    // Create a generation request
    let input_data = serde_json::to_value(&req).unwrap_or_default();
    let request = state
        .llm_document_repo
        .create_generation_request(
            tenant.tenant_id,
            tenant.user_id,
            "listing_description",
            "openai",
            "gpt-4",
            input_data.clone(),
            None,
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to create generation request: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to create request",
                )),
            )
        })?;

    // Placeholder for actual LLM call
    let placeholder_description = format!(
        "Beautiful {} {} in {} with {} rooms. This property offers {} sqm of living space with modern amenities.",
        req.property_type,
        if req.transaction_type == "sale" {
            "for sale"
        } else {
            "for rent"
        },
        req.location.city,
        req.rooms.unwrap_or(0),
        req.size_sqm.unwrap_or(0.0)
    );

    // Store the generated description
    let description = state
        .llm_document_repo
        .create_listing_description(
            tenant.tenant_id,
            req.listing_id,
            tenant.user_id,
            &req.language,
            &placeholder_description,
            input_data,
            None,
            request.id,
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to store description: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to store description",
                )),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": description.id,
            "description": placeholder_description,
            "request_id": request.id
        })),
    ))
}

async fn list_listing_descriptions(
    State(state): State<AppState>,
    Path(listing_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match state
        .llm_document_repo
        .list_listing_descriptions(listing_id)
        .await
    {
        Ok(descriptions) => Ok(Json(serde_json::json!({ "descriptions": descriptions }))),
        Err(e) => {
            tracing::error!("Failed to list descriptions: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to list")),
            ))
        }
    }
}

async fn publish_description(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match state.llm_document_repo.publish_description(id).await {
        Ok(Some(desc)) => Ok(Json(serde_json::json!(desc))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Description not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to publish description: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to publish")),
            ))
        }
    }
}

// ============================================================================
// Story 64.3: Enhanced Chat (RAG) Endpoints
// ============================================================================

#[utoipa::path(
    post,
    path = "/api/v1/ai/llm/chat/enhanced",
    request_body = EnhancedChatRequest,
    responses(
        (status = 200, description = "Chat response with context"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "AI LLM"
)]
async fn enhanced_chat(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<EnhancedChatRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    // Get escalation config
    let config = state
        .llm_document_repo
        .get_escalation_config(tenant.tenant_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get escalation config: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to get config")),
            )
        })?;

    // Placeholder response - real implementation would:
    // 1. Search document embeddings for relevant context
    // 2. Call LLM with context
    // 3. Check confidence against threshold
    let confidence = 0.85; // Placeholder
    let escalated = confidence < config.confidence_threshold;

    let response = serde_json::json!({
        "message_id": Uuid::new_v4(),
        "response": format!("I understand you're asking about: {}. Let me help you with that.", req.message),
        "confidence": confidence,
        "sources": [],
        "escalated": escalated,
        "escalation_reason": if escalated { Some("Low confidence in response") } else { None },
        "language_detected": req.language,
        "tokens_used": 150
    });

    Ok(Json(response))
}

async fn get_escalation_config(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state
        .llm_document_repo
        .get_escalation_config(tenant.tenant_id)
        .await
    {
        Ok(config) => Ok(Json(serde_json::json!(config))),
        Err(e) => {
            tracing::error!("Failed to get config: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to get config")),
            ))
        }
    }
}

async fn update_escalation_config(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<UpdateEscalationConfig>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state
        .llm_document_repo
        .update_escalation_config(tenant.tenant_id, req)
        .await
    {
        Ok(config) => Ok(Json(serde_json::json!(config))),
        Err(e) => {
            tracing::error!("Failed to update config: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to update config",
                )),
            ))
        }
    }
}

// ============================================================================
// Story 64.4: Photo Enhancement Endpoints
// ============================================================================

#[utoipa::path(
    post,
    path = "/api/v1/ai/llm/photos/enhance",
    request_body = EnhancePhotoRequest,
    responses(
        (status = 201, description = "Photo enhancement started"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "AI LLM"
)]
async fn enhance_photo(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<EnhancePhotoRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    let metadata = serde_json::to_value(&req.options).unwrap_or_default();

    let enhancement = state
        .llm_document_repo
        .create_photo_enhancement(
            tenant.tenant_id,
            req.listing_id,
            tenant.user_id,
            &req.photo_url,
            &req.enhancement_type,
            metadata,
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to create enhancement: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to create enhancement",
                )),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": enhancement.id,
            "status": enhancement.status,
            "is_ai_enhanced": true,
            "message": "Photo enhancement started. Check status for completion."
        })),
    ))
}

async fn batch_enhance_photos(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<db::models::BatchEnhancePhotosRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    let mut enhancements = Vec::new();
    for photo_url in &req.photo_urls {
        let enhancement = state
            .llm_document_repo
            .create_photo_enhancement(
                tenant.tenant_id,
                req.listing_id,
                tenant.user_id,
                photo_url,
                &req.enhancement_type,
                serde_json::json!({}),
            )
            .await
            .map_err(|e| {
                tracing::error!("Failed to create enhancement: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::new(
                        "INTERNAL_ERROR",
                        "Failed to create enhancement",
                    )),
                )
            })?;
        enhancements.push(serde_json::json!({
            "id": enhancement.id,
            "status": enhancement.status,
            "original_url": photo_url
        }));
    }

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "batch_id": Uuid::new_v4(),
            "total_photos": req.photo_urls.len(),
            "enhancements": enhancements
        })),
    ))
}

async fn get_photo_enhancement(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match state.llm_document_repo.find_photo_enhancement(id).await {
        Ok(Some(enhancement)) => Ok(Json(serde_json::json!(enhancement))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Enhancement not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to get enhancement: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to get")),
            ))
        }
    }
}

// ============================================================================
// Story 64.5: Voice Assistant Endpoints
// ============================================================================

async fn list_voice_devices(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state
        .llm_document_repo
        .list_user_voice_devices(tenant.user_id)
        .await
    {
        Ok(devices) => Ok(Json(serde_json::json!({ "devices": devices }))),
        Err(e) => {
            tracing::error!("Failed to list devices: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to list")),
            ))
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/ai/llm/voice/devices",
    request_body = LinkVoiceDeviceRequest,
    responses(
        (status = 201, description = "Voice device linked"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    tag = "AI Voice"
)]
async fn link_voice_device(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<LinkVoiceDeviceRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    // Generate a unique device ID: platform prefix + UUID for debugging and uniqueness.
    // Format: "google_assistant_550e8400-e29b-41d4-a716-446655440000"
    // While longer than a plain UUID, the platform prefix aids in debugging and log analysis.
    let device_id = format!("{}_{}", req.platform, Uuid::new_v4());

    // TODO: Phase 2 - Implement OAuth token exchange using auth_code from request.
    // Current implementation (Phase 1) stores device linkage but doesn't handle OAuth tokens.
    // Future: Exchange auth_code for access_token/refresh_token from voice assistant platform.
    let device = state
        .llm_document_repo
        .create_voice_device(
            tenant.tenant_id,
            tenant.user_id,
            req.unit_id,
            &req.platform,
            &device_id,
            req.device_name.as_deref(),
            None, // access_token - TODO: fetch via OAuth in Phase 2
            None, // refresh_token - TODO: fetch via OAuth in Phase 2
            None, // token_expires_at - TODO: set from OAuth response in Phase 2
            serde_json::json!(["check_balance", "report_fault", "check_announcements"]),
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to link device: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "INTERNAL_ERROR",
                    "Failed to link device",
                )),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "device_id": device.id,
            "platform": device.platform,
            "device_name": device.device_name,
            "capabilities": ["check_balance", "report_fault", "check_announcements"],
            "linked_at": device.linked_at
        })),
    ))
}

async fn unlink_voice_device(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    match state.llm_document_repo.deactivate_voice_device(id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Device not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to unlink device: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to unlink")),
            ))
        }
    }
}

async fn list_voice_commands(
    State(state): State<AppState>,
    Path(device_id): Path<Uuid>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match state
        .llm_document_repo
        .list_voice_commands(
            device_id,
            query.limit.unwrap_or(50),
            query.offset.unwrap_or(0),
        )
        .await
    {
        Ok(commands) => Ok(Json(serde_json::json!({ "commands": commands }))),
        Err(e) => {
            tracing::error!("Failed to list commands: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to list")),
            ))
        }
    }
}

// ============================================================================
// Statistics and Requests Endpoints
// ============================================================================

async fn get_ai_statistics(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state
        .llm_document_repo
        .get_usage_statistics(tenant.tenant_id, None, None)
        .await
    {
        Ok(stats) => Ok(Json(serde_json::json!(stats))),
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

#[derive(Debug, Serialize, Deserialize, Default, utoipa::IntoParams)]
pub struct GenerationRequestsQuery {
    pub request_type: Option<String>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

async fn list_generation_requests(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<GenerationRequestsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let tenant = extract_tenant_context(&headers)?;

    match state
        .llm_document_repo
        .list_generation_requests(
            tenant.tenant_id,
            query.request_type.as_deref(),
            query.status.as_deref(),
            query.limit.unwrap_or(50),
            query.offset.unwrap_or(0),
        )
        .await
    {
        Ok(requests) => Ok(Json(serde_json::json!({ "requests": requests }))),
        Err(e) => {
            tracing::error!("Failed to list requests: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to list")),
            ))
        }
    }
}

async fn get_generation_request(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match state.llm_document_repo.find_generation_request(id).await {
        Ok(Some(request)) => Ok(Json(serde_json::json!(request))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Request not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to get request: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("INTERNAL_ERROR", "Failed to get")),
            ))
        }
    }
}
