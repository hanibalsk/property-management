//! Favorites routes - save and manage favorite listings (Story 16.2).

use crate::state::AppState;
use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use db::models::{AddFavorite, FavoriteWithListing, FavoritesResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Create favorites router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_favorites))
        .route("/:listing_id", post(add_favorite))
        .route("/:listing_id", delete(remove_favorite))
        .route("/:listing_id/check", get(check_favorite))
}

/// Check favorite response.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CheckFavoriteResponse {
    pub is_favorited: bool,
}

/// List user's favorites.
#[utoipa::path(
    get,
    path = "/api/v1/favorites",
    tag = "Favorites",
    responses(
        (status = 200, description = "List of favorites", body = FavoritesResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_favorites(
    State(_state): State<AppState>,
) -> Result<Json<FavoritesResponse>, (axum::http::StatusCode, String)> {
    // TODO: Extract user_id from authentication context when auth middleware is implemented.
    // Returns UNAUTHORIZED until proper auth is in place to prevent data leakage.
    Err((
        axum::http::StatusCode::UNAUTHORIZED,
        "Authentication required".to_string(),
    ))
}

/// Add listing to favorites.
#[utoipa::path(
    post,
    path = "/api/v1/favorites/{listing_id}",
    tag = "Favorites",
    params(("listing_id" = Uuid, Path, description = "Listing ID")),
    request_body = AddFavorite,
    responses(
        (status = 201, description = "Added to favorites", body = FavoriteWithListing),
        (status = 400, description = "Max favorites reached"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Listing not found")
    )
)]
pub async fn add_favorite(
    State(_state): State<AppState>,
    Path(_listing_id): Path<Uuid>,
    Json(_data): Json<AddFavorite>,
) -> Result<Json<FavoriteWithListing>, (axum::http::StatusCode, String)> {
    // TODO: Extract user_id from authentication context when auth middleware is implemented.
    // Returns UNAUTHORIZED until proper auth is in place to prevent data leakage.
    Err((
        axum::http::StatusCode::UNAUTHORIZED,
        "Authentication required".to_string(),
    ))
}

/// Remove listing from favorites.
#[utoipa::path(
    delete,
    path = "/api/v1/favorites/{listing_id}",
    tag = "Favorites",
    params(("listing_id" = Uuid, Path, description = "Listing ID")),
    responses(
        (status = 204, description = "Removed from favorites"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Favorite not found")
    )
)]
pub async fn remove_favorite(
    State(_state): State<AppState>,
    Path(_listing_id): Path<Uuid>,
) -> Result<axum::http::StatusCode, (axum::http::StatusCode, String)> {
    // TODO: Extract user_id from authentication context when auth middleware is implemented.
    // Returns UNAUTHORIZED until proper auth is in place to prevent data leakage.
    Err((
        axum::http::StatusCode::UNAUTHORIZED,
        "Authentication required".to_string(),
    ))
}

/// Check if listing is favorited.
#[utoipa::path(
    get,
    path = "/api/v1/favorites/{listing_id}/check",
    tag = "Favorites",
    params(("listing_id" = Uuid, Path, description = "Listing ID")),
    responses(
        (status = 200, description = "Favorite status", body = CheckFavoriteResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn check_favorite(
    State(_state): State<AppState>,
    Path(_listing_id): Path<Uuid>,
) -> Result<Json<CheckFavoriteResponse>, (axum::http::StatusCode, String)> {
    // TODO: Extract user_id from authentication context when auth middleware is implemented.
    // Returns UNAUTHORIZED until proper auth is in place to prevent data leakage.
    Err((
        axum::http::StatusCode::UNAUTHORIZED,
        "Authentication required".to_string(),
    ))
}
