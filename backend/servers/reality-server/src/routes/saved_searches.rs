//! Saved searches routes (Story 16.3).

use crate::state::AppState;
use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use db::models::{CreateSavedSearch, SavedSearch, SavedSearchesResponse, UpdateSavedSearch};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Create saved searches router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_saved_searches))
        .route("/", post(create_saved_search))
        .route("/:id", get(get_saved_search))
        .route("/:id", put(update_saved_search))
        .route("/:id", delete(delete_saved_search))
        .route("/:id/run", post(run_saved_search))
}

/// Run saved search response.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RunSavedSearchResponse {
    pub count: i64,
    pub listings: Vec<db::models::PublicListingSummary>,
}

/// List user's saved searches.
#[utoipa::path(
    get,
    path = "/api/v1/saved-searches",
    tag = "SavedSearches",
    responses(
        (status = 200, description = "List of saved searches", body = SavedSearchesResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_saved_searches(
    State(state): State<AppState>,
) -> Result<Json<SavedSearchesResponse>, (axum::http::StatusCode, String)> {
    // TODO: Get user from auth context
    let user_id = Uuid::nil(); // Placeholder

    let searches = state
        .portal_repo
        .get_saved_searches(user_id)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get saved searches: {}", e),
            )
        })?;

    let total = searches.len() as i64;

    Ok(Json(SavedSearchesResponse { searches, total }))
}

/// Create a saved search.
#[utoipa::path(
    post,
    path = "/api/v1/saved-searches",
    tag = "SavedSearches",
    request_body = CreateSavedSearch,
    responses(
        (status = 201, description = "Saved search created", body = SavedSearch),
        (status = 400, description = "Max searches reached"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn create_saved_search(
    State(state): State<AppState>,
    Json(data): Json<CreateSavedSearch>,
) -> Result<Json<SavedSearch>, (axum::http::StatusCode, String)> {
    // TODO: Get user from auth context
    let user_id = Uuid::nil(); // Placeholder

    // Check saved searches limit (max 20)
    let count = state
        .portal_repo
        .count_saved_searches(user_id)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to count saved searches: {}", e),
            )
        })?;

    if count >= 20 {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            "Maximum saved searches limit (20) reached".to_string(),
        ));
    }

    let search = state
        .portal_repo
        .create_saved_search(user_id, data)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create saved search: {}", e),
            )
        })?;

    Ok(Json(search))
}

/// Get a saved search by ID.
#[utoipa::path(
    get,
    path = "/api/v1/saved-searches/{id}",
    tag = "SavedSearches",
    params(("id" = Uuid, Path, description = "Saved search ID")),
    responses(
        (status = 200, description = "Saved search", body = SavedSearch),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Saved search not found")
    )
)]
pub async fn get_saved_search(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<SavedSearch>, (axum::http::StatusCode, String)> {
    // TODO: Get user from auth context
    let user_id = Uuid::nil(); // Placeholder

    let search = state
        .portal_repo
        .get_saved_search(id, user_id)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get saved search: {}", e),
            )
        })?
        .ok_or_else(|| {
            (
                axum::http::StatusCode::NOT_FOUND,
                "Saved search not found".to_string(),
            )
        })?;

    Ok(Json(search))
}

/// Update a saved search.
#[utoipa::path(
    put,
    path = "/api/v1/saved-searches/{id}",
    tag = "SavedSearches",
    params(("id" = Uuid, Path, description = "Saved search ID")),
    request_body = UpdateSavedSearch,
    responses(
        (status = 200, description = "Saved search updated", body = SavedSearch),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Saved search not found")
    )
)]
pub async fn update_saved_search(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(data): Json<UpdateSavedSearch>,
) -> Result<Json<SavedSearch>, (axum::http::StatusCode, String)> {
    // TODO: Get user from auth context
    let user_id = Uuid::nil(); // Placeholder

    let search = state
        .portal_repo
        .update_saved_search(id, user_id, data)
        .await
        .map_err(|e| {
            if e.to_string().contains("no rows") {
                (
                    axum::http::StatusCode::NOT_FOUND,
                    "Saved search not found".to_string(),
                )
            } else {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to update saved search: {}", e),
                )
            }
        })?;

    Ok(Json(search))
}

/// Delete a saved search.
#[utoipa::path(
    delete,
    path = "/api/v1/saved-searches/{id}",
    tag = "SavedSearches",
    params(("id" = Uuid, Path, description = "Saved search ID")),
    responses(
        (status = 204, description = "Saved search deleted"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Saved search not found")
    )
)]
pub async fn delete_saved_search(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, (axum::http::StatusCode, String)> {
    // TODO: Get user from auth context
    let user_id = Uuid::nil(); // Placeholder

    let deleted = state
        .portal_repo
        .delete_saved_search(id, user_id)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to delete saved search: {}", e),
            )
        })?;

    if !deleted {
        return Err((
            axum::http::StatusCode::NOT_FOUND,
            "Saved search not found".to_string(),
        ));
    }

    Ok(axum::http::StatusCode::NO_CONTENT)
}

/// Run a saved search to get matching listings.
#[utoipa::path(
    post,
    path = "/api/v1/saved-searches/{id}/run",
    tag = "SavedSearches",
    params(("id" = Uuid, Path, description = "Saved search ID")),
    responses(
        (status = 200, description = "Search results", body = RunSavedSearchResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Saved search not found")
    )
)]
pub async fn run_saved_search(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<RunSavedSearchResponse>, (axum::http::StatusCode, String)> {
    // TODO: Get user from auth context
    let user_id = Uuid::nil(); // Placeholder

    // Get the saved search
    let search = state
        .portal_repo
        .get_saved_search(id, user_id)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get saved search: {}", e),
            )
        })?
        .ok_or_else(|| {
            (
                axum::http::StatusCode::NOT_FOUND,
                "Saved search not found".to_string(),
            )
        })?;

    // Parse criteria from JSONB
    let criteria: db::models::SearchCriteria = serde_json::from_value(search.criteria.clone())
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse search criteria: {}", e),
            )
        })?;

    // Convert to PublicListingQuery
    let query = db::models::PublicListingQuery {
        q: criteria.q.clone(),
        property_type: criteria.property_type.clone(),
        transaction_type: criteria.transaction_type.clone(),
        price_min: criteria.price_min,
        price_max: criteria.price_max,
        area_min: criteria.area_min,
        area_max: criteria.area_max,
        rooms_min: criteria.rooms_min,
        rooms_max: criteria.rooms_max,
        city: criteria.city.clone(),
        country: criteria.country.clone(),
        page: Some(1),
        limit: Some(50),
        sort: Some("date_desc".to_string()),
    };

    // Search listings
    let listings = state
        .portal_repo
        .search_listings(&query)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to search listings: {}", e),
            )
        })?;

    let count = state
        .portal_repo
        .count_listings(&query)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to count listings: {}", e),
            )
        })?;

    // Update match count
    let _ = state.portal_repo.update_match_count(id, count as i32).await;

    Ok(Json(RunSavedSearchResponse { count, listings }))
}
