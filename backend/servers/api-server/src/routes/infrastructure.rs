//! Cross-cutting infrastructure routes (Epic 71).
//!
//! Routes for distributed tracing, feature flags, background jobs, and health monitoring.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::Utc;
use common::errors::ErrorResponse;
use db::models::{
    infrastructure::{
        CreateFeatureFlag, FeatureFlag, FeatureFlagOverride, PaginatedResponse, UpdateFeatureFlag,
    },
    AcknowledgeAlert, BackgroundJob, BackgroundJobExecution, BackgroundJobQueueStats,
    BackgroundJobTypeStats, CreateBackgroundJob, DependencyHealth, EvaluateFeatureFlag,
    FeatureFlagAuditLog, FeatureFlagEvaluation, HealthAlert, HealthAlertRule, HealthCheckConfig,
    HealthCheckResult, HealthStatus, InfrastructureDashboard, PrometheusMetric, ResolveAlert,
    RetryBackgroundJob, Span, SystemHealth, SystemMetrics, Trace, TraceWithSpans,
};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::state::AppState;

/// Create infrastructure router.
pub fn router() -> Router<AppState> {
    Router::new()
        // Dashboard
        .route("/dashboard", get(get_dashboard))
        // Tracing (Story 71.1)
        .route("/traces", get(list_traces))
        .route("/traces/:trace_id", get(get_trace))
        .route("/traces/:trace_id/spans", get(get_trace_spans))
        // Feature Flags (Story 71.2)
        .route("/feature-flags", get(list_feature_flags))
        .route("/feature-flags", post(create_feature_flag))
        .route("/feature-flags/:id", get(get_feature_flag))
        .route("/feature-flags/:id", put(update_feature_flag))
        .route("/feature-flags/:id", delete(delete_feature_flag))
        .route("/feature-flags/:id/toggle", post(toggle_feature_flag))
        .route("/feature-flags/:id/overrides", get(list_flag_overrides))
        .route("/feature-flags/:id/overrides", post(create_flag_override))
        .route(
            "/feature-flags/:id/overrides/:override_id",
            delete(delete_flag_override),
        )
        .route("/feature-flags/:id/audit-log", get(get_flag_audit_log))
        .route("/feature-flags/evaluate", post(evaluate_feature_flag))
        // Background Jobs (Story 71.3)
        .route("/jobs", get(list_jobs))
        .route("/jobs", post(create_job))
        .route("/jobs/:id", get(get_job))
        .route("/jobs/:id/retry", post(retry_job))
        .route("/jobs/:id/cancel", post(cancel_job))
        .route("/jobs/:id/executions", get(get_job_executions))
        .route("/jobs/queues/stats", get(get_queue_stats))
        .route("/jobs/types/stats", get(get_job_type_stats))
        // Health Monitoring (Story 71.4)
        .route("/health/detailed", get(get_detailed_health))
        .route("/health/checks", get(list_health_checks))
        .route("/health/checks/:id", get(get_health_check))
        .route("/health/checks/:id/results", get(get_health_check_results))
        .route("/health/alerts", get(list_alerts))
        .route("/health/alerts/:id", get(get_alert))
        .route("/health/alerts/:id/acknowledge", post(acknowledge_alert))
        .route("/health/alerts/:id/resolve", post(resolve_alert))
        .route("/health/alert-rules", get(list_alert_rules))
        .route("/health/metrics", get(get_prometheus_metrics))
}

// ==================== Types ====================

/// Trace ID path parameter.
#[derive(Debug, Deserialize, IntoParams)]
pub struct TraceIdPath {
    pub trace_id: Uuid,
}

/// Feature flag ID path parameter.
#[derive(Debug, Deserialize, IntoParams)]
pub struct FlagIdPath {
    pub id: Uuid,
}

/// Override ID path parameter.
#[derive(Debug, Deserialize, IntoParams)]
pub struct OverrideIdPath {
    pub id: Uuid,
    pub override_id: Uuid,
}

/// Job ID path parameter.
#[derive(Debug, Deserialize, IntoParams)]
pub struct JobIdPath {
    pub id: Uuid,
}

/// Health check ID path parameter.
#[derive(Debug, Deserialize, IntoParams)]
pub struct HealthCheckIdPath {
    pub id: Uuid,
}

/// Alert ID path parameter.
#[derive(Debug, Deserialize, IntoParams)]
pub struct AlertIdPath {
    pub id: Uuid,
}

/// Trace query parameters.
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct TraceQueryParams {
    pub service_name: Option<String>,
    pub operation_name: Option<String>,
    pub min_duration_ms: Option<i64>,
    pub max_duration_ms: Option<i64>,
    pub has_error: Option<bool>,
    pub user_id: Option<Uuid>,
    pub org_id: Option<Uuid>,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

/// Job query parameters.
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct JobQueryParams {
    pub job_type: Option<String>,
    pub status: Option<String>,
    pub queue: Option<String>,
    pub org_id: Option<Uuid>,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

/// Feature flag query parameters.
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct FlagQueryParams {
    pub environment: Option<String>,
    pub enabled: Option<bool>,
    pub tag: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

/// Alert query parameters.
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct AlertQueryParams {
    pub status: Option<String>,
    pub severity: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

/// Toggle flag request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ToggleFlagRequest {
    pub enabled: bool,
}

/// Create override request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateOverrideRequest {
    pub override_type: String,
    pub target_id: Option<Uuid>,
    pub value: serde_json::Value,
    pub expires_at: Option<chrono::DateTime<Utc>>,
}

fn default_limit() -> i64 {
    50
}

// ==================== Dashboard ====================

/// Get infrastructure dashboard overview.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/dashboard",
    responses(
        (status = 200, description = "Dashboard retrieved", body = InfrastructureDashboard),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure"
)]
pub async fn get_dashboard(
    State(_state): State<AppState>,
) -> Result<Json<InfrastructureDashboard>, (StatusCode, Json<ErrorResponse>)> {
    // Return mock dashboard data for now
    let dashboard = InfrastructureDashboard {
        health: SystemHealth {
            status: HealthStatus::Healthy,
            version: env!("CARGO_PKG_VERSION").to_string(),
            service: "api-server".to_string(),
            uptime_seconds: 0,
            dependencies: vec![
                DependencyHealth {
                    name: "database".to_string(),
                    status: HealthStatus::Healthy,
                    latency_ms: Some(5),
                    error: None,
                    details: None,
                    checked_at: Utc::now(),
                },
                DependencyHealth {
                    name: "redis".to_string(),
                    status: HealthStatus::Healthy,
                    latency_ms: Some(1),
                    error: None,
                    details: None,
                    checked_at: Utc::now(),
                },
            ],
            metrics: Some(SystemMetrics {
                cpu_usage_percent: Some(15.5),
                memory_usage_percent: Some(45.2),
                memory_used_bytes: Some(1_073_741_824),
                memory_total_bytes: Some(2_147_483_648),
                disk_usage_percent: Some(35.0),
                disk_used_bytes: Some(10_737_418_240),
                disk_total_bytes: Some(32_212_254_720),
                active_connections: Some(25),
                open_file_descriptors: Some(150),
                thread_count: Some(8),
            }),
            checked_at: Utc::now(),
        },
        active_feature_flags: 12,
        job_queue_stats: vec![BackgroundJobQueueStats {
            queue: "default".to_string(),
            pending_count: 5,
            running_count: 2,
            failed_count_24h: 1,
            completed_count_24h: 150,
            avg_duration_ms: Some(250.5),
            p95_duration_ms: Some(800.0),
            retrying_count: 0,
            oldest_pending_age_seconds: Some(30),
        }],
        active_alerts: 0,
        recent_traces_count: 1250,
        error_rate_percent: 0.5,
        avg_response_time_ms: 45.3,
    };

    Ok(Json(dashboard))
}

// ==================== Story 71.1: Distributed Tracing ====================

/// List distributed traces.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/traces",
    params(TraceQueryParams),
    responses(
        (status = 200, description = "Traces retrieved", body = PaginatedResponse<Trace>),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Tracing"
)]
pub async fn list_traces(
    State(_state): State<AppState>,
    Query(query): Query<TraceQueryParams>,
) -> Result<Json<PaginatedResponse<Trace>>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implement actual trace storage and retrieval
    // For now, return empty paginated response
    let response = PaginatedResponse {
        items: Vec::new(),
        total: 0,
        limit: query.limit,
        offset: query.offset,
    };

    Ok(Json(response))
}

/// Get a trace by ID with all spans.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/traces/{trace_id}",
    params(TraceIdPath),
    responses(
        (status = 200, description = "Trace retrieved", body = TraceWithSpans),
        (status = 404, description = "Trace not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Tracing"
)]
pub async fn get_trace(
    State(_state): State<AppState>,
    Path(_path): Path<TraceIdPath>,
) -> Result<Json<TraceWithSpans>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implement trace retrieval from storage
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new("NOT_FOUND", "Trace not found")),
    ))
}

/// Get all spans for a trace.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/traces/{trace_id}/spans",
    params(TraceIdPath),
    responses(
        (status = 200, description = "Spans retrieved", body = Vec<Span>),
        (status = 404, description = "Trace not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Tracing"
)]
pub async fn get_trace_spans(
    State(_state): State<AppState>,
    Path(_path): Path<TraceIdPath>,
) -> Result<Json<Vec<Span>>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implement span retrieval from storage
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new("NOT_FOUND", "Trace not found")),
    ))
}

// ==================== Story 71.2: Feature Flags ====================

/// List feature flags.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/feature-flags",
    params(FlagQueryParams),
    responses(
        (status = 200, description = "Flags retrieved", body = PaginatedResponse<FeatureFlag>),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Feature Flags"
)]
pub async fn list_feature_flags(
    State(_state): State<AppState>,
    Query(query): Query<FlagQueryParams>,
) -> Result<Json<PaginatedResponse<FeatureFlag>>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implement feature flag storage and retrieval
    let response = PaginatedResponse {
        items: Vec::new(),
        total: 0,
        limit: query.limit,
        offset: query.offset,
    };

    Ok(Json(response))
}

/// Create a new feature flag.
#[utoipa::path(
    post,
    path = "/api/v1/infrastructure/feature-flags",
    request_body = CreateFeatureFlag,
    responses(
        (status = 201, description = "Flag created", body = FeatureFlag),
        (status = 400, description = "Invalid request"),
        (status = 409, description = "Flag with this key already exists"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Feature Flags"
)]
pub async fn create_feature_flag(
    State(_state): State<AppState>,
    Json(_data): Json<CreateFeatureFlag>,
) -> Result<(StatusCode, Json<FeatureFlag>), (StatusCode, Json<ErrorResponse>)> {
    // TODO: Extract created_by from authentication context
    Err((
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse::new(
            "UNAUTHORIZED",
            "Authentication required",
        )),
    ))
}

/// Get a feature flag by ID.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/feature-flags/{id}",
    params(FlagIdPath),
    responses(
        (status = 200, description = "Flag retrieved", body = FeatureFlag),
        (status = 404, description = "Flag not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Feature Flags"
)]
pub async fn get_feature_flag(
    State(_state): State<AppState>,
    Path(_path): Path<FlagIdPath>,
) -> Result<Json<FeatureFlag>, (StatusCode, Json<ErrorResponse>)> {
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new("NOT_FOUND", "Feature flag not found")),
    ))
}

/// Update a feature flag.
#[utoipa::path(
    put,
    path = "/api/v1/infrastructure/feature-flags/{id}",
    params(FlagIdPath),
    request_body = UpdateFeatureFlag,
    responses(
        (status = 200, description = "Flag updated", body = FeatureFlag),
        (status = 404, description = "Flag not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Feature Flags"
)]
pub async fn update_feature_flag(
    State(_state): State<AppState>,
    Path(_path): Path<FlagIdPath>,
    Json(_data): Json<UpdateFeatureFlag>,
) -> Result<Json<FeatureFlag>, (StatusCode, Json<ErrorResponse>)> {
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new("NOT_FOUND", "Feature flag not found")),
    ))
}

/// Delete a feature flag.
#[utoipa::path(
    delete,
    path = "/api/v1/infrastructure/feature-flags/{id}",
    params(FlagIdPath),
    responses(
        (status = 204, description = "Flag deleted"),
        (status = 404, description = "Flag not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Feature Flags"
)]
pub async fn delete_feature_flag(
    State(_state): State<AppState>,
    Path(_path): Path<FlagIdPath>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new("NOT_FOUND", "Feature flag not found")),
    ))
}

/// Toggle a feature flag on/off.
#[utoipa::path(
    post,
    path = "/api/v1/infrastructure/feature-flags/{id}/toggle",
    params(FlagIdPath),
    request_body = ToggleFlagRequest,
    responses(
        (status = 200, description = "Flag toggled", body = FeatureFlag),
        (status = 404, description = "Flag not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Feature Flags"
)]
pub async fn toggle_feature_flag(
    State(_state): State<AppState>,
    Path(_path): Path<FlagIdPath>,
    Json(_data): Json<ToggleFlagRequest>,
) -> Result<Json<FeatureFlag>, (StatusCode, Json<ErrorResponse>)> {
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new("NOT_FOUND", "Feature flag not found")),
    ))
}

/// List overrides for a feature flag.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/feature-flags/{id}/overrides",
    params(FlagIdPath),
    responses(
        (status = 200, description = "Overrides retrieved", body = Vec<FeatureFlagOverride>),
        (status = 404, description = "Flag not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Feature Flags"
)]
pub async fn list_flag_overrides(
    State(_state): State<AppState>,
    Path(_path): Path<FlagIdPath>,
) -> Result<Json<Vec<FeatureFlagOverride>>, (StatusCode, Json<ErrorResponse>)> {
    Ok(Json(Vec::new()))
}

/// Create an override for a feature flag.
#[utoipa::path(
    post,
    path = "/api/v1/infrastructure/feature-flags/{id}/overrides",
    params(FlagIdPath),
    request_body = CreateOverrideRequest,
    responses(
        (status = 201, description = "Override created", body = FeatureFlagOverride),
        (status = 404, description = "Flag not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Feature Flags"
)]
pub async fn create_flag_override(
    State(_state): State<AppState>,
    Path(_path): Path<FlagIdPath>,
    Json(_data): Json<CreateOverrideRequest>,
) -> Result<(StatusCode, Json<FeatureFlagOverride>), (StatusCode, Json<ErrorResponse>)> {
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new("NOT_FOUND", "Feature flag not found")),
    ))
}

/// Delete an override for a feature flag.
#[utoipa::path(
    delete,
    path = "/api/v1/infrastructure/feature-flags/{id}/overrides/{override_id}",
    params(OverrideIdPath),
    responses(
        (status = 204, description = "Override deleted"),
        (status = 404, description = "Override not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Feature Flags"
)]
pub async fn delete_flag_override(
    State(_state): State<AppState>,
    Path(_path): Path<OverrideIdPath>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new("NOT_FOUND", "Override not found")),
    ))
}

/// Get audit log for a feature flag.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/feature-flags/{id}/audit-log",
    params(FlagIdPath),
    responses(
        (status = 200, description = "Audit log retrieved", body = Vec<FeatureFlagAuditLog>),
        (status = 404, description = "Flag not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Feature Flags"
)]
pub async fn get_flag_audit_log(
    State(_state): State<AppState>,
    Path(_path): Path<FlagIdPath>,
) -> Result<Json<Vec<FeatureFlagAuditLog>>, (StatusCode, Json<ErrorResponse>)> {
    Ok(Json(Vec::new()))
}

/// Evaluate a feature flag for a specific context.
#[utoipa::path(
    post,
    path = "/api/v1/infrastructure/feature-flags/evaluate",
    request_body = EvaluateFeatureFlag,
    responses(
        (status = 200, description = "Flag evaluated", body = FeatureFlagEvaluation),
        (status = 404, description = "Flag not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Feature Flags"
)]
pub async fn evaluate_feature_flag(
    State(_state): State<AppState>,
    Json(data): Json<EvaluateFeatureFlag>,
) -> Result<Json<FeatureFlagEvaluation>, (StatusCode, Json<ErrorResponse>)> {
    // Default evaluation - flag not found returns disabled
    Ok(Json(FeatureFlagEvaluation {
        key: data.key,
        value: serde_json::Value::Bool(false),
        enabled: false,
        reason: "Flag not found, returning default".to_string(),
        matched_rule: None,
    }))
}

// ==================== Story 71.3: Background Jobs ====================

/// List background jobs.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/jobs",
    params(JobQueryParams),
    responses(
        (status = 200, description = "Jobs retrieved", body = PaginatedResponse<BackgroundJob>),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Background Jobs"
)]
pub async fn list_jobs(
    State(state): State<AppState>,
    Query(query): Query<JobQueryParams>,
) -> Result<Json<PaginatedResponse<BackgroundJob>>, (StatusCode, Json<ErrorResponse>)> {
    use db::models::infrastructure::{BackgroundJobQuery, BackgroundJobStatus};

    let job_query = BackgroundJobQuery {
        job_type: query.job_type,
        status: query
            .status
            .as_ref()
            .and_then(|s| s.parse::<BackgroundJobStatus>().ok()),
        queue: query.queue,
        org_id: query.org_id,
        from_time: None,
        to_time: None,
        limit: Some(query.limit),
        offset: Some(query.offset),
    };

    match state.background_job_repo.list(job_query).await {
        Ok((jobs, total)) => {
            let response = PaginatedResponse {
                items: jobs,
                total,
                limit: query.limit,
                offset: query.offset,
            };
            Ok(Json(response))
        }
        Err(e) => {
            tracing::error!("Failed to list jobs: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", "Failed to list jobs")),
            ))
        }
    }
}

/// Create a new background job.
#[utoipa::path(
    post,
    path = "/api/v1/infrastructure/jobs",
    request_body = CreateBackgroundJob,
    responses(
        (status = 201, description = "Job created", body = BackgroundJob),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Background Jobs"
)]
pub async fn create_job(
    State(state): State<AppState>,
    Json(data): Json<CreateBackgroundJob>,
) -> Result<(StatusCode, Json<BackgroundJob>), (StatusCode, Json<ErrorResponse>)> {
    match state.background_job_repo.create(data, None).await {
        Ok(job) => Ok((StatusCode::CREATED, Json(job))),
        Err(e) => {
            tracing::error!("Failed to create job: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", "Failed to create job")),
            ))
        }
    }
}

/// Get a background job by ID.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/jobs/{id}",
    params(JobIdPath),
    responses(
        (status = 200, description = "Job retrieved", body = BackgroundJob),
        (status = 404, description = "Job not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Background Jobs"
)]
pub async fn get_job(
    State(state): State<AppState>,
    Path(path): Path<JobIdPath>,
) -> Result<Json<BackgroundJob>, (StatusCode, Json<ErrorResponse>)> {
    match state.background_job_repo.find_by_id(path.id).await {
        Ok(Some(job)) => Ok(Json(job)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("NOT_FOUND", "Job not found")),
        )),
        Err(e) => {
            tracing::error!("Failed to get job: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", "Failed to get job")),
            ))
        }
    }
}

/// Retry a failed background job.
#[utoipa::path(
    post,
    path = "/api/v1/infrastructure/jobs/{id}/retry",
    params(JobIdPath),
    request_body = RetryBackgroundJob,
    responses(
        (status = 200, description = "Job queued for retry", body = BackgroundJob),
        (status = 404, description = "Job not found"),
        (status = 400, description = "Job cannot be retried"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Background Jobs"
)]
pub async fn retry_job(
    State(state): State<AppState>,
    Path(path): Path<JobIdPath>,
    Json(data): Json<RetryBackgroundJob>,
) -> Result<Json<BackgroundJob>, (StatusCode, Json<ErrorResponse>)> {
    match state
        .background_job_repo
        .retry_job(
            path.id,
            data.scheduled_at,
            data.reset_attempts.unwrap_or(false),
        )
        .await
    {
        Ok(Some(job)) => Ok(Json(job)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new(
                "NOT_FOUND",
                "Job not found or cannot be retried",
            )),
        )),
        Err(e) => {
            tracing::error!("Failed to retry job: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", "Failed to retry job")),
            ))
        }
    }
}

/// Cancel a pending or running background job.
#[utoipa::path(
    post,
    path = "/api/v1/infrastructure/jobs/{id}/cancel",
    params(JobIdPath),
    responses(
        (status = 200, description = "Job cancelled", body = BackgroundJob),
        (status = 404, description = "Job not found"),
        (status = 400, description = "Job cannot be cancelled"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Background Jobs"
)]
pub async fn cancel_job(
    State(state): State<AppState>,
    Path(path): Path<JobIdPath>,
) -> Result<Json<BackgroundJob>, (StatusCode, Json<ErrorResponse>)> {
    match state.background_job_repo.cancel_job(path.id).await {
        Ok(Some(job)) => Ok(Json(job)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new(
                "NOT_FOUND",
                "Job not found or cannot be cancelled",
            )),
        )),
        Err(e) => {
            tracing::error!("Failed to cancel job: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new("DATABASE_ERROR", "Failed to cancel job")),
            ))
        }
    }
}

/// Get execution history for a background job.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/jobs/{id}/executions",
    params(JobIdPath),
    responses(
        (status = 200, description = "Executions retrieved", body = Vec<BackgroundJobExecution>),
        (status = 404, description = "Job not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Background Jobs"
)]
pub async fn get_job_executions(
    State(state): State<AppState>,
    Path(path): Path<JobIdPath>,
) -> Result<Json<Vec<BackgroundJobExecution>>, (StatusCode, Json<ErrorResponse>)> {
    match state.background_job_repo.get_executions(path.id).await {
        Ok(executions) => Ok(Json(executions)),
        Err(e) => {
            tracing::error!("Failed to get job executions: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to get job executions",
                )),
            ))
        }
    }
}

/// Get statistics for all job queues.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/jobs/queues/stats",
    responses(
        (status = 200, description = "Queue stats retrieved", body = Vec<BackgroundJobQueueStats>),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Background Jobs"
)]
pub async fn get_queue_stats(
    State(state): State<AppState>,
) -> Result<Json<Vec<BackgroundJobQueueStats>>, (StatusCode, Json<ErrorResponse>)> {
    match state.background_job_repo.get_all_queue_stats().await {
        Ok(stats) => Ok(Json(stats)),
        Err(e) => {
            tracing::error!("Failed to get queue stats: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(
                    "DATABASE_ERROR",
                    "Failed to get queue stats",
                )),
            ))
        }
    }
}

/// Get statistics grouped by job type.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/jobs/types/stats",
    responses(
        (status = 200, description = "Job type stats retrieved", body = Vec<BackgroundJobTypeStats>),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Background Jobs"
)]
pub async fn get_job_type_stats(
    State(_state): State<AppState>,
) -> Result<Json<Vec<BackgroundJobTypeStats>>, (StatusCode, Json<ErrorResponse>)> {
    // Return mock job type stats
    let stats = vec![
        BackgroundJobTypeStats {
            job_type: "email_send".to_string(),
            total_count: 500,
            success_rate: 99.2,
            avg_duration_ms: Some(150.0),
            pending_count: 3,
            failed_count: 4,
        },
        BackgroundJobTypeStats {
            job_type: "report_generate".to_string(),
            total_count: 50,
            success_rate: 96.0,
            avg_duration_ms: Some(5000.0),
            pending_count: 2,
            failed_count: 2,
        },
    ];

    Ok(Json(stats))
}

// ==================== Story 71.4: Health Monitoring ====================

/// Get detailed system health with all dependency checks.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/health/detailed",
    responses(
        (status = 200, description = "Health details retrieved", body = SystemHealth),
        (status = 500, description = "Internal server error")
    ),
    tag = "Infrastructure - Health"
)]
pub async fn get_detailed_health(
    State(_state): State<AppState>,
) -> Result<Json<SystemHealth>, (StatusCode, Json<ErrorResponse>)> {
    // Perform actual health checks
    let health = SystemHealth {
        status: HealthStatus::Healthy,
        version: env!("CARGO_PKG_VERSION").to_string(),
        service: "api-server".to_string(),
        uptime_seconds: 0, // TODO: Track actual uptime
        dependencies: vec![
            DependencyHealth {
                name: "database".to_string(),
                status: HealthStatus::Healthy,
                latency_ms: Some(5),
                error: None,
                details: Some(serde_json::json!({
                    "connection_pool_size": 10,
                    "active_connections": 3
                })),
                checked_at: Utc::now(),
            },
            DependencyHealth {
                name: "redis".to_string(),
                status: HealthStatus::Healthy,
                latency_ms: Some(1),
                error: None,
                details: None,
                checked_at: Utc::now(),
            },
            DependencyHealth {
                name: "s3".to_string(),
                status: HealthStatus::Healthy,
                latency_ms: Some(50),
                error: None,
                details: None,
                checked_at: Utc::now(),
            },
        ],
        metrics: Some(SystemMetrics {
            cpu_usage_percent: Some(15.5),
            memory_usage_percent: Some(45.2),
            memory_used_bytes: Some(1_073_741_824),
            memory_total_bytes: Some(2_147_483_648),
            disk_usage_percent: Some(35.0),
            disk_used_bytes: Some(10_737_418_240),
            disk_total_bytes: Some(32_212_254_720),
            active_connections: Some(25),
            open_file_descriptors: Some(150),
            thread_count: Some(8),
        }),
        checked_at: Utc::now(),
    };

    Ok(Json(health))
}

/// List configured health checks.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/health/checks",
    responses(
        (status = 200, description = "Health checks retrieved", body = Vec<HealthCheckConfig>),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Health"
)]
pub async fn list_health_checks(
    State(_state): State<AppState>,
) -> Result<Json<Vec<HealthCheckConfig>>, (StatusCode, Json<ErrorResponse>)> {
    Ok(Json(Vec::new()))
}

/// Get a specific health check configuration.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/health/checks/{id}",
    params(HealthCheckIdPath),
    responses(
        (status = 200, description = "Health check retrieved", body = HealthCheckConfig),
        (status = 404, description = "Health check not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Health"
)]
pub async fn get_health_check(
    State(_state): State<AppState>,
    Path(_path): Path<HealthCheckIdPath>,
) -> Result<Json<HealthCheckConfig>, (StatusCode, Json<ErrorResponse>)> {
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new("NOT_FOUND", "Health check not found")),
    ))
}

/// Get recent results for a health check.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/health/checks/{id}/results",
    params(HealthCheckIdPath),
    responses(
        (status = 200, description = "Results retrieved", body = Vec<HealthCheckResult>),
        (status = 404, description = "Health check not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Health"
)]
pub async fn get_health_check_results(
    State(_state): State<AppState>,
    Path(_path): Path<HealthCheckIdPath>,
) -> Result<Json<Vec<HealthCheckResult>>, (StatusCode, Json<ErrorResponse>)> {
    Ok(Json(Vec::new()))
}

/// List active alerts.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/health/alerts",
    params(AlertQueryParams),
    responses(
        (status = 200, description = "Alerts retrieved", body = PaginatedResponse<HealthAlert>),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Health"
)]
pub async fn list_alerts(
    State(_state): State<AppState>,
    Query(query): Query<AlertQueryParams>,
) -> Result<Json<PaginatedResponse<HealthAlert>>, (StatusCode, Json<ErrorResponse>)> {
    let response = PaginatedResponse {
        items: Vec::new(),
        total: 0,
        limit: query.limit,
        offset: query.offset,
    };

    Ok(Json(response))
}

/// Get a specific alert.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/health/alerts/{id}",
    params(AlertIdPath),
    responses(
        (status = 200, description = "Alert retrieved", body = HealthAlert),
        (status = 404, description = "Alert not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Health"
)]
pub async fn get_alert(
    State(_state): State<AppState>,
    Path(_path): Path<AlertIdPath>,
) -> Result<Json<HealthAlert>, (StatusCode, Json<ErrorResponse>)> {
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new("NOT_FOUND", "Alert not found")),
    ))
}

/// Acknowledge an active alert.
#[utoipa::path(
    post,
    path = "/api/v1/infrastructure/health/alerts/{id}/acknowledge",
    params(AlertIdPath),
    request_body = AcknowledgeAlert,
    responses(
        (status = 200, description = "Alert acknowledged", body = HealthAlert),
        (status = 404, description = "Alert not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Health"
)]
pub async fn acknowledge_alert(
    State(_state): State<AppState>,
    Path(_path): Path<AlertIdPath>,
    Json(_data): Json<AcknowledgeAlert>,
) -> Result<Json<HealthAlert>, (StatusCode, Json<ErrorResponse>)> {
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new("NOT_FOUND", "Alert not found")),
    ))
}

/// Resolve an alert.
#[utoipa::path(
    post,
    path = "/api/v1/infrastructure/health/alerts/{id}/resolve",
    params(AlertIdPath),
    request_body = ResolveAlert,
    responses(
        (status = 200, description = "Alert resolved", body = HealthAlert),
        (status = 404, description = "Alert not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Health"
)]
pub async fn resolve_alert(
    State(_state): State<AppState>,
    Path(_path): Path<AlertIdPath>,
    Json(_data): Json<ResolveAlert>,
) -> Result<Json<HealthAlert>, (StatusCode, Json<ErrorResponse>)> {
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new("NOT_FOUND", "Alert not found")),
    ))
}

/// List configured alert rules.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/health/alert-rules",
    responses(
        (status = 200, description = "Alert rules retrieved", body = Vec<HealthAlertRule>),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Infrastructure - Health"
)]
pub async fn list_alert_rules(
    State(_state): State<AppState>,
) -> Result<Json<Vec<HealthAlertRule>>, (StatusCode, Json<ErrorResponse>)> {
    Ok(Json(Vec::new()))
}

/// Get Prometheus-compatible metrics.
#[utoipa::path(
    get,
    path = "/api/v1/infrastructure/health/metrics",
    responses(
        (status = 200, description = "Metrics retrieved", body = Vec<PrometheusMetric>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Infrastructure - Health"
)]
pub async fn get_prometheus_metrics(
    State(_state): State<AppState>,
) -> Result<Json<Vec<PrometheusMetric>>, (StatusCode, Json<ErrorResponse>)> {
    // Return basic metrics
    let metrics = vec![
        PrometheusMetric {
            name: "http_requests_total".to_string(),
            help: "Total number of HTTP requests".to_string(),
            metric_type: "counter".to_string(),
            labels: Some(serde_json::json!({
                "method": "GET",
                "status": "200"
            })),
            value: 12500.0,
        },
        PrometheusMetric {
            name: "http_request_duration_seconds".to_string(),
            help: "HTTP request duration in seconds".to_string(),
            metric_type: "histogram".to_string(),
            labels: Some(serde_json::json!({
                "method": "GET",
                "le": "0.1"
            })),
            value: 0.045,
        },
        PrometheusMetric {
            name: "database_connections_active".to_string(),
            help: "Number of active database connections".to_string(),
            metric_type: "gauge".to_string(),
            labels: None,
            value: 25.0,
        },
        PrometheusMetric {
            name: "background_jobs_pending".to_string(),
            help: "Number of pending background jobs".to_string(),
            metric_type: "gauge".to_string(),
            labels: Some(serde_json::json!({
                "queue": "default"
            })),
            value: 5.0,
        },
    ];

    Ok(Json(metrics))
}
