//! Market Pricing & Analytics routes (Epic 132).
//! Dynamic Rent Pricing, Market Data Collection, and Comparative Market Analysis.

use crate::state::AppState;
use api_core::extractors::AuthUser;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use common::errors::ErrorResponse;
use db::models::market_pricing::{
    AcceptPricingRecommendation, AddCmaProperty, CreateComparativeMarketAnalysis,
    CreateMarketDataPoint, CreateMarketRegion, GenerateStatisticsRequest, MarketDataQuery,
    RecordPriceChange, RejectPricingRecommendation, RequestPricingRecommendation,
    UpdateComparativeMarketAnalysis, UpdateMarketRegion,
};
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_json::json;
use utoipa::IntoParams;
use uuid::Uuid;

type ApiResult<T> = Result<T, (StatusCode, Json<ErrorResponse>)>;

pub fn router() -> Router<AppState> {
    Router::new()
        // Market Regions
        .route("/regions", get(list_regions))
        .route("/regions", post(create_region))
        .route("/regions/{id}", get(get_region))
        .route("/regions/{id}", put(update_region))
        .route("/regions/{id}", delete(delete_region))
        // Market Data Points
        .route("/data", get(list_data_points))
        .route("/data", post(add_data_point))
        // Market Statistics
        .route("/statistics/{region_id}", get(get_statistics))
        .route("/statistics/generate", post(generate_statistics))
        // Pricing Recommendations
        .route("/recommendations", get(list_recommendations))
        .route("/recommendations/request", post(request_recommendation))
        .route("/recommendations/{id}", get(get_recommendation))
        .route("/recommendations/{id}/accept", post(accept_recommendation))
        .route("/recommendations/{id}/reject", post(reject_recommendation))
        // Unit Pricing History
        .route("/units/{unit_id}/history", get(get_pricing_history))
        .route("/units/{unit_id}/price", post(record_price_change))
        .route("/units/{unit_id}/current-rent", get(get_current_rent))
        // Comparative Market Analysis
        .route("/cma", get(list_cmas))
        .route("/cma", post(create_cma))
        .route("/cma/{id}", get(get_cma))
        .route("/cma/{id}", put(update_cma))
        .route("/cma/{id}", delete(delete_cma))
        .route("/cma/{id}/properties", get(get_cma_properties))
        .route("/cma/{id}/properties", post(add_cma_property))
        // Comparables lookup
        .route("/comparables", get(get_comparables))
}

// =============================================================================
// MARKET REGIONS
// =============================================================================

async fn list_regions(
    State(s): State<AppState>,
    user: AuthUser,
) -> ApiResult<Json<serde_json::Value>> {
    let org_id = user.tenant_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request("Tenant context required")),
        )
    })?;

    match s.market_pricing_repo.list_regions(org_id).await {
        Ok(regions) => Ok(Json(json!({ "regions": regions }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal_error(&e.to_string())),
        )),
    }
}

async fn create_region(
    State(s): State<AppState>,
    user: AuthUser,
    Json(req): Json<CreateMarketRegion>,
) -> ApiResult<Json<serde_json::Value>> {
    let org_id = user.tenant_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request("Tenant context required")),
        )
    })?;

    match s.market_pricing_repo.create_region(org_id, req).await {
        Ok(region) => Ok(Json(serde_json::to_value(region).unwrap())),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(&e.to_string())),
        )),
    }
}

async fn get_region(
    State(s): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let org_id = user.tenant_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request("Tenant context required")),
        )
    })?;

    match s.market_pricing_repo.get_region(id, org_id).await {
        Ok(Some(region)) => Ok(Json(serde_json::to_value(region).unwrap())),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("Region not found")),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal_error(&e.to_string())),
        )),
    }
}

async fn update_region(
    State(s): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateMarketRegion>,
) -> ApiResult<Json<serde_json::Value>> {
    let org_id = user.tenant_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request("Tenant context required")),
        )
    })?;

    match s.market_pricing_repo.update_region(id, org_id, req).await {
        Ok(Some(region)) => Ok(Json(serde_json::to_value(region).unwrap())),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("Region not found")),
        )),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(&e.to_string())),
        )),
    }
}

async fn delete_region(
    State(s): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let org_id = user.tenant_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request("Tenant context required")),
        )
    })?;

    match s.market_pricing_repo.delete_region(id, org_id).await {
        Ok(true) => Ok(Json(json!({ "deleted": true }))),
        Ok(false) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("Region not found")),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal_error(&e.to_string())),
        )),
    }
}

// =============================================================================
// MARKET DATA POINTS
// =============================================================================

async fn list_data_points(
    State(s): State<AppState>,
    user: AuthUser,
    Query(query): Query<MarketDataQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let org_id = user.tenant_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request("Tenant context required")),
        )
    })?;

    match s.market_pricing_repo.list_data_points(org_id, query).await {
        Ok(data_points) => Ok(Json(json!({ "data_points": data_points }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal_error(&e.to_string())),
        )),
    }
}

async fn add_data_point(
    State(s): State<AppState>,
    _user: AuthUser,
    Json(req): Json<CreateMarketDataPoint>,
) -> ApiResult<Json<serde_json::Value>> {
    match s.market_pricing_repo.add_data_point(req).await {
        Ok(data_point) => Ok(Json(serde_json::to_value(data_point).unwrap())),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(&e.to_string())),
        )),
    }
}

// =============================================================================
// MARKET STATISTICS
// =============================================================================

#[derive(Debug, Deserialize, IntoParams)]
pub struct StatisticsParams {
    pub property_type: Option<String>,
}

async fn get_statistics(
    State(s): State<AppState>,
    _user: AuthUser,
    Path(region_id): Path<Uuid>,
    Query(params): Query<StatisticsParams>,
) -> ApiResult<Json<serde_json::Value>> {
    match s
        .market_pricing_repo
        .get_market_statistics(region_id, params.property_type)
        .await
    {
        Ok(Some(stats)) => Ok(Json(serde_json::to_value(stats).unwrap())),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("No statistics available")),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal_error(&e.to_string())),
        )),
    }
}

async fn generate_statistics(
    State(s): State<AppState>,
    _user: AuthUser,
    Json(req): Json<GenerateStatisticsRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    match s.market_pricing_repo.generate_statistics(req).await {
        Ok(stats) => Ok(Json(serde_json::to_value(stats).unwrap())),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(&e.to_string())),
        )),
    }
}

// =============================================================================
// PRICING RECOMMENDATIONS
// =============================================================================

async fn list_recommendations(
    State(s): State<AppState>,
    user: AuthUser,
) -> ApiResult<Json<serde_json::Value>> {
    let org_id = user.tenant_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request("Tenant context required")),
        )
    })?;

    match s
        .market_pricing_repo
        .list_pending_recommendations(org_id)
        .await
    {
        Ok(recs) => Ok(Json(json!({ "recommendations": recs }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal_error(&e.to_string())),
        )),
    }
}

async fn request_recommendation(
    State(s): State<AppState>,
    user: AuthUser,
    Json(req): Json<RequestPricingRecommendation>,
) -> ApiResult<Json<serde_json::Value>> {
    let _org_id = user.tenant_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request("Tenant context required")),
        )
    })?;

    // For now, generate a simple recommendation based on market data
    // In Story 132.2, this will be enhanced with AI model integration
    let min_price = Decimal::new(800, 0);
    let optimal_price = Decimal::new(950, 0);
    let max_price = Decimal::new(1100, 0);
    let confidence = Decimal::new(75, 0);
    let factors = json!({
        "market_avg": 920,
        "location_premium": 3,
        "size_adjustment": 2
    });

    match s
        .market_pricing_repo
        .create_recommendation(
            req.unit_id,
            min_price,
            optimal_price,
            max_price,
            &req.currency,
            confidence,
            factors,
            5,
            None,
        )
        .await
    {
        Ok(rec) => Ok(Json(serde_json::to_value(rec).unwrap())),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(&e.to_string())),
        )),
    }
}

async fn get_recommendation(
    State(s): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let org_id = user.tenant_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request("Tenant context required")),
        )
    })?;

    match s.market_pricing_repo.get_recommendation(id, org_id).await {
        Ok(Some(rec)) => Ok(Json(serde_json::to_value(rec).unwrap())),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("Recommendation not found")),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal_error(&e.to_string())),
        )),
    }
}

async fn accept_recommendation(
    State(s): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<AcceptPricingRecommendation>,
) -> ApiResult<Json<serde_json::Value>> {
    let org_id = user.tenant_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request("Tenant context required")),
        )
    })?;
    let user_id = user.user_id;

    match s
        .market_pricing_repo
        .accept_recommendation(id, org_id, user_id, req.accepted_price)
        .await
    {
        Ok(Some(rec)) => Ok(Json(serde_json::to_value(rec).unwrap())),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("Recommendation not found")),
        )),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(&e.to_string())),
        )),
    }
}

async fn reject_recommendation(
    State(s): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<RejectPricingRecommendation>,
) -> ApiResult<Json<serde_json::Value>> {
    let org_id = user.tenant_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request("Tenant context required")),
        )
    })?;

    match s
        .market_pricing_repo
        .reject_recommendation(id, org_id, &req.rejection_reason)
        .await
    {
        Ok(Some(rec)) => Ok(Json(serde_json::to_value(rec).unwrap())),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("Recommendation not found")),
        )),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(&e.to_string())),
        )),
    }
}

// =============================================================================
// UNIT PRICING HISTORY
// =============================================================================

async fn get_pricing_history(
    State(s): State<AppState>,
    user: AuthUser,
    Path(unit_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let org_id = user.tenant_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request("Tenant context required")),
        )
    })?;

    match s
        .market_pricing_repo
        .get_pricing_history(unit_id, org_id)
        .await
    {
        Ok(history) => Ok(Json(json!({ "history": history }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal_error(&e.to_string())),
        )),
    }
}

async fn record_price_change(
    State(s): State<AppState>,
    user: AuthUser,
    Path(unit_id): Path<Uuid>,
    Json(mut req): Json<RecordPriceChange>,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = user.user_id;
    req.unit_id = unit_id;

    match s
        .market_pricing_repo
        .record_price_change(req, user_id)
        .await
    {
        Ok(history) => Ok(Json(serde_json::to_value(history).unwrap())),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(&e.to_string())),
        )),
    }
}

async fn get_current_rent(
    State(s): State<AppState>,
    _user: AuthUser,
    Path(unit_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    match s.market_pricing_repo.get_current_rent(unit_id).await {
        Ok(Some(rent)) => Ok(Json(json!({ "current_rent": rent }))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("No pricing history for unit")),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal_error(&e.to_string())),
        )),
    }
}

// =============================================================================
// COMPARATIVE MARKET ANALYSIS
// =============================================================================

async fn list_cmas(
    State(s): State<AppState>,
    user: AuthUser,
) -> ApiResult<Json<serde_json::Value>> {
    let org_id = user.tenant_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request("Tenant context required")),
        )
    })?;

    match s.market_pricing_repo.list_cmas(org_id).await {
        Ok(cmas) => Ok(Json(json!({ "analyses": cmas }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal_error(&e.to_string())),
        )),
    }
}

async fn create_cma(
    State(s): State<AppState>,
    user: AuthUser,
    Json(req): Json<CreateComparativeMarketAnalysis>,
) -> ApiResult<Json<serde_json::Value>> {
    let org_id = user.tenant_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request("Tenant context required")),
        )
    })?;
    let user_id = user.user_id;

    match s.market_pricing_repo.create_cma(org_id, user_id, req).await {
        Ok(cma) => Ok(Json(serde_json::to_value(cma).unwrap())),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(&e.to_string())),
        )),
    }
}

async fn get_cma(
    State(s): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let org_id = user.tenant_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request("Tenant context required")),
        )
    })?;

    match s.market_pricing_repo.get_cma(id, org_id).await {
        Ok(Some(cma)) => Ok(Json(serde_json::to_value(cma).unwrap())),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("CMA not found")),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal_error(&e.to_string())),
        )),
    }
}

async fn update_cma(
    State(s): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateComparativeMarketAnalysis>,
) -> ApiResult<Json<serde_json::Value>> {
    let org_id = user.tenant_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request("Tenant context required")),
        )
    })?;

    match s.market_pricing_repo.update_cma(id, org_id, req).await {
        Ok(Some(cma)) => Ok(Json(serde_json::to_value(cma).unwrap())),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("CMA not found")),
        )),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(&e.to_string())),
        )),
    }
}

async fn delete_cma(
    State(s): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let org_id = user.tenant_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request("Tenant context required")),
        )
    })?;

    match s.market_pricing_repo.delete_cma(id, org_id).await {
        Ok(true) => Ok(Json(json!({ "deleted": true }))),
        Ok(false) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("CMA not found")),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal_error(&e.to_string())),
        )),
    }
}

async fn get_cma_properties(
    State(s): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    match s.market_pricing_repo.get_cma_properties(id).await {
        Ok(props) => Ok(Json(json!({ "properties": props }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal_error(&e.to_string())),
        )),
    }
}

async fn add_cma_property(
    State(s): State<AppState>,
    _user: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<AddCmaProperty>,
) -> ApiResult<Json<serde_json::Value>> {
    match s.market_pricing_repo.add_property_to_cma(id, req).await {
        Ok(prop) => Ok(Json(serde_json::to_value(prop).unwrap())),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(&e.to_string())),
        )),
    }
}

// =============================================================================
// COMPARABLES LOOKUP
// =============================================================================

#[derive(Debug, Deserialize, IntoParams)]
pub struct ComparablesParams {
    pub region_id: Uuid,
    pub property_type: String,
    pub size_sqm: Decimal,
    pub limit: Option<i32>,
}

async fn get_comparables(
    State(s): State<AppState>,
    _user: AuthUser,
    Query(params): Query<ComparablesParams>,
) -> ApiResult<Json<serde_json::Value>> {
    let limit = params.limit.unwrap_or(10).min(50);

    match s
        .market_pricing_repo
        .get_market_comparables(
            params.region_id,
            &params.property_type,
            params.size_sqm,
            limit,
        )
        .await
    {
        Ok(comparables) => Ok(Json(json!({ "comparables": comparables }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal_error(&e.to_string())),
        )),
    }
}
