//! Portfolio Analytics routes (Epic 140).
//! Provides cross-property analytics, benchmarking, and trend analysis.

use crate::state::AppState;
use api_core::extractors::AuthUser;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::NaiveDate;
use common::errors::ErrorResponse;
use db::models::portfolio_analytics::{
    AggregationPeriod, AlertQuery, BenchmarkCategory, CreateAlertRule, CreatePortfolioBenchmark,
    CreatePropertyMetrics, TrendQuery, UpdateAlertRule, UpdatePortfolioBenchmark,
};
use serde::Deserialize;
use uuid::Uuid;

type ApiResult<T> = Result<T, (StatusCode, Json<ErrorResponse>)>;

fn get_org_id(auth: &AuthUser) -> Result<Uuid, (StatusCode, Json<ErrorResponse>)> {
    auth.tenant_id.ok_or((
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse::bad_request("No organization context")),
    ))
}

pub fn router() -> Router<AppState> {
    Router::new()
        // Benchmarks
        .route("/benchmarks", get(list_benchmarks).post(create_benchmark))
        .route(
            "/benchmarks/:benchmark_id",
            get(get_benchmark)
                .put(update_benchmark)
                .delete(delete_benchmark),
        )
        // Property Metrics
        .route("/properties/metrics", post(upsert_property_metrics))
        .route(
            "/properties/:building_id/metrics",
            get(get_property_metrics),
        )
        .route(
            "/properties/:building_id/metrics/history",
            get(list_property_metrics_history),
        )
        .route("/properties/metrics/all", get(list_all_property_metrics))
        // Portfolio Metrics
        .route("/metrics", get(get_portfolio_metrics))
        .route("/metrics/history", get(list_portfolio_metrics_history))
        .route("/metrics/calculate", post(calculate_portfolio_metrics))
        // Alert Rules
        .route(
            "/alert-rules",
            get(list_alert_rules).post(create_alert_rule),
        )
        .route(
            "/alert-rules/:rule_id",
            put(update_alert_rule).delete(delete_alert_rule),
        )
        // Alerts
        .route("/alerts", get(list_alerts))
        .route("/alerts/:alert_id/acknowledge", post(acknowledge_alert))
        .route("/alerts/:alert_id/resolve", post(resolve_alert))
        .route("/alerts/count", get(get_active_alerts_count))
        // Trends
        .route("/trends", get(list_trends))
}

// =============================================================================
// QUERY PARAMETERS
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct BenchmarkQueryParams {
    pub category: Option<BenchmarkCategory>,
}

#[derive(Debug, Deserialize)]
pub struct MetricsQueryParams {
    pub period_type: AggregationPeriod,
    pub period_start: NaiveDate,
}

#[derive(Debug, Deserialize)]
pub struct MetricsHistoryParams {
    pub period_type: AggregationPeriod,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_limit() -> i64 {
    12
}

#[derive(Debug, Deserialize)]
pub struct CalculateMetricsParams {
    pub period_type: AggregationPeriod,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
}

// =============================================================================
// BENCHMARK HANDLERS
// =============================================================================

async fn list_benchmarks(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<BenchmarkQueryParams>,
) -> ApiResult<Json<Vec<db::models::portfolio_analytics::PortfolioBenchmark>>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .list_benchmarks(org_id, params.category)
        .await
        .map(Json)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })
}

async fn create_benchmark(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(data): Json<CreatePortfolioBenchmark>,
) -> ApiResult<Json<db::models::portfolio_analytics::PortfolioBenchmark>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .create_benchmark(org_id, &data, auth.user_id)
        .await
        .map(Json)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })
}

async fn get_benchmark(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(benchmark_id): Path<Uuid>,
) -> ApiResult<Json<db::models::portfolio_analytics::PortfolioBenchmark>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .get_benchmark(benchmark_id, org_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })?
        .map(Json)
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("Benchmark not found")),
        ))
}

async fn update_benchmark(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(benchmark_id): Path<Uuid>,
    Json(data): Json<UpdatePortfolioBenchmark>,
) -> ApiResult<Json<db::models::portfolio_analytics::PortfolioBenchmark>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .update_benchmark(benchmark_id, org_id, &data)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })?
        .map(Json)
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("Benchmark not found")),
        ))
}

async fn delete_benchmark(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(benchmark_id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let org_id = get_org_id(&auth)?;

    let deleted = state
        .portfolio_analytics_repo
        .delete_benchmark(benchmark_id, org_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("Benchmark not found")),
        ))
    }
}

// =============================================================================
// PROPERTY METRICS HANDLERS
// =============================================================================

async fn upsert_property_metrics(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(data): Json<CreatePropertyMetrics>,
) -> ApiResult<Json<db::models::portfolio_analytics::PropertyPerformanceMetrics>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .upsert_property_metrics(org_id, &data)
        .await
        .map(Json)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })
}

async fn get_property_metrics(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(building_id): Path<Uuid>,
    Query(params): Query<MetricsQueryParams>,
) -> ApiResult<Json<db::models::portfolio_analytics::PropertyPerformanceMetrics>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .get_property_metrics(
            org_id,
            building_id,
            &params.period_type,
            params.period_start,
        )
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })?
        .map(Json)
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("Property metrics not found")),
        ))
}

async fn list_property_metrics_history(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(building_id): Path<Uuid>,
    Query(params): Query<MetricsHistoryParams>,
) -> ApiResult<Json<Vec<db::models::portfolio_analytics::PropertyPerformanceMetrics>>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .list_property_metrics_history(org_id, building_id, &params.period_type, params.limit)
        .await
        .map(Json)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })
}

async fn list_all_property_metrics(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<MetricsQueryParams>,
) -> ApiResult<Json<Vec<db::models::portfolio_analytics::PropertyPerformanceMetrics>>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .list_all_property_metrics(org_id, &params.period_type, params.period_start)
        .await
        .map(Json)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })
}

// =============================================================================
// PORTFOLIO METRICS HANDLERS
// =============================================================================

async fn get_portfolio_metrics(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<MetricsQueryParams>,
) -> ApiResult<Json<db::models::portfolio_analytics::PortfolioAggregatedMetrics>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .get_portfolio_metrics(org_id, &params.period_type, params.period_start)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })?
        .map(Json)
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("Portfolio metrics not found")),
        ))
}

async fn list_portfolio_metrics_history(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<MetricsHistoryParams>,
) -> ApiResult<Json<Vec<db::models::portfolio_analytics::PortfolioAggregatedMetrics>>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .list_portfolio_metrics_history(org_id, &params.period_type, params.limit)
        .await
        .map(Json)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })
}

async fn calculate_portfolio_metrics(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<CalculateMetricsParams>,
) -> ApiResult<Json<db::models::portfolio_analytics::PortfolioAggregatedMetrics>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .calculate_portfolio_metrics(
            org_id,
            &params.period_type,
            params.period_start,
            params.period_end,
        )
        .await
        .map(Json)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })
}

// =============================================================================
// ALERT RULE HANDLERS
// =============================================================================

async fn list_alert_rules(
    State(state): State<AppState>,
    auth: AuthUser,
) -> ApiResult<Json<Vec<db::models::portfolio_analytics::PortfolioAlertRule>>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .list_alert_rules(org_id)
        .await
        .map(Json)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })
}

async fn create_alert_rule(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(data): Json<CreateAlertRule>,
) -> ApiResult<Json<db::models::portfolio_analytics::PortfolioAlertRule>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .create_alert_rule(org_id, &data, auth.user_id)
        .await
        .map(Json)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })
}

async fn update_alert_rule(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(rule_id): Path<Uuid>,
    Json(data): Json<UpdateAlertRule>,
) -> ApiResult<Json<db::models::portfolio_analytics::PortfolioAlertRule>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .update_alert_rule(rule_id, org_id, &data)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })?
        .map(Json)
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("Alert rule not found")),
        ))
}

async fn delete_alert_rule(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(rule_id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let org_id = get_org_id(&auth)?;

    let deleted = state
        .portfolio_analytics_repo
        .delete_alert_rule(rule_id, org_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("Alert rule not found")),
        ))
    }
}

// =============================================================================
// ALERT HANDLERS
// =============================================================================

async fn list_alerts(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<AlertQuery>,
) -> ApiResult<Json<Vec<db::models::portfolio_analytics::PortfolioAlert>>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .list_alerts(org_id, &query)
        .await
        .map(Json)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })
}

async fn acknowledge_alert(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(alert_id): Path<Uuid>,
) -> ApiResult<Json<db::models::portfolio_analytics::PortfolioAlert>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .acknowledge_alert(alert_id, org_id, auth.user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })?
        .map(Json)
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("Alert not found")),
        ))
}

async fn resolve_alert(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(alert_id): Path<Uuid>,
) -> ApiResult<Json<db::models::portfolio_analytics::PortfolioAlert>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .resolve_alert(alert_id, org_id, auth.user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })?
        .map(Json)
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("Alert not found")),
        ))
}

#[derive(Debug, serde::Serialize)]
struct AlertCountResponse {
    count: i64,
}

async fn get_active_alerts_count(
    State(state): State<AppState>,
    auth: AuthUser,
) -> ApiResult<Json<AlertCountResponse>> {
    let org_id = get_org_id(&auth)?;

    let count = state
        .portfolio_analytics_repo
        .get_active_alerts_count(org_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })?;

    Ok(Json(AlertCountResponse { count }))
}

// =============================================================================
// TREND HANDLERS
// =============================================================================

async fn list_trends(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<TrendQuery>,
) -> ApiResult<Json<Vec<db::models::portfolio_analytics::PortfolioTrend>>> {
    let org_id = get_org_id(&auth)?;

    state
        .portfolio_analytics_repo
        .list_trends(org_id, &query)
        .await
        .map(Json)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(&e.to_string())),
            )
        })
}
