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
    State(_state): State<AppState>,
) -> Result<Json<SavedSearchesResponse>, (axum::http::StatusCode, String)> {
    // TODO: Extract user_id from authentication context when auth middleware is implemented.
    // Returns UNAUTHORIZED until proper auth is in place to prevent data leakage.
    Err((
        axum::http::StatusCode::UNAUTHORIZED,
        "Authentication required".to_string(),
    ))
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
    State(_state): State<AppState>,
    Json(_data): Json<CreateSavedSearch>,
) -> Result<Json<SavedSearch>, (axum::http::StatusCode, String)> {
    // TODO: Extract user_id from authentication context when auth middleware is implemented.
    // Returns UNAUTHORIZED until proper auth is in place to prevent data leakage.
    Err((
        axum::http::StatusCode::UNAUTHORIZED,
        "Authentication required".to_string(),
    ))
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
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<SavedSearch>, (axum::http::StatusCode, String)> {
    // TODO: Extract user_id from authentication context when auth middleware is implemented.
    // Returns UNAUTHORIZED until proper auth is in place to prevent data leakage.
    Err((
        axum::http::StatusCode::UNAUTHORIZED,
        "Authentication required".to_string(),
    ))
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
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
    Json(_data): Json<UpdateSavedSearch>,
) -> Result<Json<SavedSearch>, (axum::http::StatusCode, String)> {
    // TODO: Extract user_id from authentication context when auth middleware is implemented.
    // Returns UNAUTHORIZED until proper auth is in place to prevent data leakage.
    Err((
        axum::http::StatusCode::UNAUTHORIZED,
        "Authentication required".to_string(),
    ))
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
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<axum::http::StatusCode, (axum::http::StatusCode, String)> {
    // TODO: Extract user_id from authentication context when auth middleware is implemented.
    // Returns UNAUTHORIZED until proper auth is in place to prevent data leakage.
    Err((
        axum::http::StatusCode::UNAUTHORIZED,
        "Authentication required".to_string(),
    ))
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
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<RunSavedSearchResponse>, (axum::http::StatusCode, String)> {
    // TODO: Extract user_id from authentication context when auth middleware is implemented.
    // Returns UNAUTHORIZED until proper auth is in place to prevent data leakage.
    Err((
        axum::http::StatusCode::UNAUTHORIZED,
        "Authentication required".to_string(),
    ))
}
