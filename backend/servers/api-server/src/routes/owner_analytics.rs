//! Owner Investment Analytics routes (Epic 74).
use api_core::extractors::TenantContext;
use axum::{extract::{Path, Query, State}, http::StatusCode, routing::{delete, get, post, put}, Json, Router};
use chrono::NaiveDate;
use common::errors::ErrorResponse;
use db::models::owner_analytics::{AddComparableProperty, CalculateROIRequest, CreateAutoApprovalRule, CreatePropertyValuation, ExpenseRequestsQuery, OwnerPropertiesQuery, PortfolioComparisonRequest, ReviewExpenseRequest, SubmitExpenseForApproval, UpdateAutoApprovalRule, ValueHistoryQuery};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use crate::state::AppState;

type ApiResult<T> = Result<T, (StatusCode, Json<ErrorResponse>)>;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/units/{unit_id}/valuation", get(get_unit_valuation))
        .route("/units/{unit_id}/valuation", post(create_valuation))
        .route("/valuations/{valuation_id}", get(get_valuation_with_comparables))
        .route("/valuations/{valuation_id}/comparables", post(add_comparable))
        .route("/units/{unit_id}/value-history", get(get_value_history))
        .route("/units/{unit_id}/value-trend", get(get_value_trend))
        .route("/units/{unit_id}/roi", post(calculate_roi))
        .route("/units/{unit_id}/cash-flow", get(get_cash_flow_breakdown))
        .route("/units/{unit_id}/roi-dashboard", get(get_roi_dashboard))
        .route("/portfolio", get(get_portfolio_summary))
        .route("/portfolio/compare", post(compare_properties))
        .route("/expense-rules", get(list_auto_approval_rules))
        .route("/expense-rules", post(create_auto_approval_rule))
        .route("/expense-rules/{id}", put(update_auto_approval_rule))
        .route("/expense-rules/{id}", delete(delete_auto_approval_rule))
        .route("/expenses/submit", post(submit_expense))
        .route("/expenses", get(list_expense_requests))
        .route("/expenses/{id}/review", post(review_expense))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ValueHistoryParams { pub from_date: Option<NaiveDate>, pub to_date: Option<NaiveDate> }

#[derive(Debug, Deserialize, IntoParams)]
pub struct CashFlowParams { pub from_date: NaiveDate, pub to_date: NaiveDate }

#[derive(Debug, Deserialize, IntoParams)]
pub struct ROIDashboardParams { pub from_date: NaiveDate, pub to_date: NaiveDate }

#[derive(Debug, Deserialize, IntoParams)]
pub struct PortfolioParams { pub owner_id: Option<Uuid> }

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateValuationRequest { pub organization_id: Uuid, #[serde(flatten)] pub data: CreatePropertyValuation }

#[derive(Debug, Deserialize, ToSchema)]
pub struct CalculateROIWithOrg { pub organization_id: Uuid, #[serde(flatten)] pub data: CalculateROIRequest }

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRuleRequest { pub organization_id: Uuid, #[serde(flatten)] pub data: CreateAutoApprovalRule }

#[derive(Debug, Deserialize, ToSchema)]
pub struct SubmitExpenseRequest { pub organization_id: Uuid, #[serde(flatten)] pub data: SubmitExpenseForApproval }

async fn get_unit_valuation(State(s): State<AppState>, ctx: TenantContext, Path(uid): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    let org = ctx.organization_id.ok_or_else(|| (StatusCode::BAD_REQUEST, Json(ErrorResponse::message("Organization context required"))))?;
    match s.owner_analytics.get_latest_valuation(uid, org).await {
        Ok(Some(v)) => Ok(Json(serde_json::to_value(v).unwrap())),
        Ok(None) => Err((StatusCode::NOT_FOUND, Json(ErrorResponse::message("Not found")))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse::message(e.to_string())))),
    }
}

async fn create_valuation(State(s): State<AppState>, _ctx: TenantContext, Path(_uid): Path<Uuid>, Json(r): Json<CreateValuationRequest>) -> ApiResult<Json<serde_json::Value>> {
    match s.owner_analytics.create_valuation(r.organization_id, r.data).await {
        Ok(v) => Ok(Json(serde_json::to_value(v).unwrap())),
        Err(e) => Err((StatusCode::BAD_REQUEST, Json(ErrorResponse::message(e.to_string())))),
    }
}

async fn get_valuation_with_comparables(State(s): State<AppState>, ctx: TenantContext, Path(vid): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    let org = ctx.organization_id.ok_or_else(|| (StatusCode::BAD_REQUEST, Json(ErrorResponse::message("Organization context required"))))?;
    match s.owner_analytics.get_valuation_with_comparables(vid, org).await {
        Ok(Some(v)) => Ok(Json(serde_json::to_value(v).unwrap())),
        Ok(None) => Err((StatusCode::NOT_FOUND, Json(ErrorResponse::message("Not found")))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse::message(e.to_string())))),
    }
}

async fn add_comparable(State(s): State<AppState>, _ctx: TenantContext, Path(vid): Path<Uuid>, Json(r): Json<AddComparableProperty>) -> ApiResult<Json<serde_json::Value>> {
    match s.owner_analytics.add_comparable(vid, r).await {
        Ok(c) => Ok(Json(serde_json::to_value(c).unwrap())),
        Err(e) => Err((StatusCode::BAD_REQUEST, Json(ErrorResponse::message(e.to_string())))),
    }
}

async fn get_value_history(State(s): State<AppState>, _ctx: TenantContext, Path(uid): Path<Uuid>, Query(p): Query<ValueHistoryParams>) -> ApiResult<Json<serde_json::Value>> {
    match s.owner_analytics.get_value_history(ValueHistoryQuery { unit_id: uid, from_date: p.from_date, to_date: p.to_date }).await {
        Ok(h) => Ok(Json(serde_json::to_value(h).unwrap())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse::message(e.to_string())))),
    }
}

async fn get_value_trend(State(s): State<AppState>, ctx: TenantContext, Path(uid): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    let org = ctx.organization_id.ok_or_else(|| (StatusCode::BAD_REQUEST, Json(ErrorResponse::message("Organization context required"))))?;
    match s.owner_analytics.get_value_trend(uid, org).await {
        Ok(Some(t)) => Ok(Json(serde_json::to_value(t).unwrap())),
        Ok(None) => Err((StatusCode::NOT_FOUND, Json(ErrorResponse::message("Not found")))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse::message(e.to_string())))),
    }
}

async fn calculate_roi(State(s): State<AppState>, _ctx: TenantContext, Path(_uid): Path<Uuid>, Json(r): Json<CalculateROIWithOrg>) -> ApiResult<Json<serde_json::Value>> {
    match s.owner_analytics.calculate_roi(r.organization_id, r.data).await {
        Ok(roi) => Ok(Json(serde_json::to_value(roi).unwrap())),
        Err(e) => Err((StatusCode::BAD_REQUEST, Json(ErrorResponse::message(e.to_string())))),
    }
}

async fn get_cash_flow_breakdown(State(s): State<AppState>, ctx: TenantContext, Path(uid): Path<Uuid>, Query(p): Query<CashFlowParams>) -> ApiResult<Json<serde_json::Value>> {
    let org = ctx.organization_id.ok_or_else(|| (StatusCode::BAD_REQUEST, Json(ErrorResponse::message("Organization context required"))))?;
    match s.owner_analytics.get_cash_flow_breakdown(uid, org, p.from_date, p.to_date).await {
        Ok(cf) => Ok(Json(serde_json::to_value(cf).unwrap())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse::message(e.to_string())))),
    }
}

async fn get_roi_dashboard(State(s): State<AppState>, ctx: TenantContext, Path(uid): Path<Uuid>, Query(p): Query<ROIDashboardParams>) -> ApiResult<Json<serde_json::Value>> {
    let org = ctx.organization_id.ok_or_else(|| (StatusCode::BAD_REQUEST, Json(ErrorResponse::message("Organization context required"))))?;
    match s.owner_analytics.get_roi_dashboard(uid, org, p.from_date, p.to_date).await {
        Ok(d) => Ok(Json(serde_json::to_value(d).unwrap())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse::message(e.to_string())))),
    }
}

async fn get_portfolio_summary(State(s): State<AppState>, ctx: TenantContext, Query(p): Query<PortfolioParams>) -> ApiResult<Json<serde_json::Value>> {
    match s.owner_analytics.get_portfolio_summary(OwnerPropertiesQuery { owner_id: p.owner_id, organization_id: ctx.organization_id }).await {
        Ok(ps) => Ok(Json(serde_json::to_value(ps).unwrap())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse::message(e.to_string())))),
    }
}

async fn compare_properties(State(s): State<AppState>, ctx: TenantContext, Json(r): Json<PortfolioComparisonRequest>) -> ApiResult<Json<serde_json::Value>> {
    let org = ctx.organization_id.ok_or_else(|| (StatusCode::BAD_REQUEST, Json(ErrorResponse::message("Organization context required"))))?;
    match s.owner_analytics.compare_properties(org, r).await {
        Ok(c) => Ok(Json(serde_json::to_value(c).unwrap())),
        Err(e) => Err((StatusCode::BAD_REQUEST, Json(ErrorResponse::message(e.to_string())))),
    }
}

async fn list_auto_approval_rules(State(s): State<AppState>, ctx: TenantContext) -> ApiResult<Json<serde_json::Value>> {
    let org = ctx.organization_id.ok_or_else(|| (StatusCode::BAD_REQUEST, Json(ErrorResponse::message("Organization context required"))))?;
    match s.owner_analytics.get_auto_approval_rules(ctx.user_id, org).await {
        Ok(r) => Ok(Json(serde_json::to_value(r).unwrap())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse::message(e.to_string())))),
    }
}

async fn create_auto_approval_rule(State(s): State<AppState>, ctx: TenantContext, Json(r): Json<CreateRuleRequest>) -> ApiResult<Json<serde_json::Value>> {
    match s.owner_analytics.create_auto_approval_rule(ctx.user_id, r.organization_id, r.data).await {
        Ok(rule) => Ok(Json(serde_json::to_value(rule).unwrap())),
        Err(e) => Err((StatusCode::BAD_REQUEST, Json(ErrorResponse::message(e.to_string())))),
    }
}

async fn update_auto_approval_rule(State(s): State<AppState>, ctx: TenantContext, Path(id): Path<Uuid>, Json(r): Json<UpdateAutoApprovalRule>) -> ApiResult<Json<serde_json::Value>> {
    match s.owner_analytics.update_auto_approval_rule(id, ctx.user_id, r).await {
        Ok(rule) => Ok(Json(serde_json::to_value(rule).unwrap())),
        Err(e) => Err((StatusCode::BAD_REQUEST, Json(ErrorResponse::message(e.to_string())))),
    }
}

async fn delete_auto_approval_rule(State(s): State<AppState>, ctx: TenantContext, Path(id): Path<Uuid>) -> ApiResult<StatusCode> {
    match s.owner_analytics.delete_auto_approval_rule(id, ctx.user_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((StatusCode::NOT_FOUND, Json(ErrorResponse::message(e.to_string())))),
    }
}

async fn submit_expense(State(s): State<AppState>, ctx: TenantContext, Json(r): Json<SubmitExpenseRequest>) -> ApiResult<Json<serde_json::Value>> {
    match s.owner_analytics.submit_expense_for_approval(ctx.user_id, r.organization_id, r.data).await {
        Ok(resp) => Ok(Json(serde_json::to_value(resp).unwrap())),
        Err(e) => Err((StatusCode::BAD_REQUEST, Json(ErrorResponse::message(e.to_string())))),
    }
}

async fn list_expense_requests(State(s): State<AppState>, ctx: TenantContext, Query(q): Query<ExpenseRequestsQuery>) -> ApiResult<Json<serde_json::Value>> {
    let org = ctx.organization_id.ok_or_else(|| (StatusCode::BAD_REQUEST, Json(ErrorResponse::message("Organization context required"))))?;
    match s.owner_analytics.list_expense_requests(org, q).await {
        Ok(r) => Ok(Json(serde_json::to_value(r).unwrap())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse::message(e.to_string())))),
    }
}

async fn review_expense(State(s): State<AppState>, ctx: TenantContext, Path(id): Path<Uuid>, Json(r): Json<ReviewExpenseRequest>) -> ApiResult<Json<serde_json::Value>> {
    match s.owner_analytics.review_expense(id, ctx.user_id, r).await {
        Ok(e) => Ok(Json(serde_json::to_value(e).unwrap())),
        Err(e) => Err((StatusCode::BAD_REQUEST, Json(ErrorResponse::message(e.to_string())))),
    }
}
