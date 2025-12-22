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
    State(state): State<AppState>,
) -> Result<Json<FavoritesResponse>, (axum::http::StatusCode, String)> {
    // TODO: Get user from auth context
    let user_id = Uuid::nil(); // Placeholder

    let favorites = state
        .portal_repo
        .get_favorites(user_id)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get favorites: {}", e),
            )
        })?;

    let total = favorites.len() as i64;

    Ok(Json(FavoritesResponse { favorites, total }))
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
    State(state): State<AppState>,
    Path(listing_id): Path<Uuid>,
    Json(data): Json<AddFavorite>,
) -> Result<Json<FavoriteWithListing>, (axum::http::StatusCode, String)> {
    // TODO: Get user from auth context
    let user_id = Uuid::nil(); // Placeholder

    // Check favorites limit (max 100)
    let count = state
        .portal_repo
        .count_favorites(user_id)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to count favorites: {}", e),
            )
        })?;

    if count >= 100 {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            "Maximum favorites limit (100) reached".to_string(),
        ));
    }

    // Add to favorites
    let favorite = state
        .portal_repo
        .add_favorite(user_id, listing_id, data.notes)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to add favorite: {}", e),
            )
        })?;

    // Return with listing details
    let favorites = state
        .portal_repo
        .get_favorites(user_id)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get favorites: {}", e),
            )
        })?;

    let fav_with_listing = favorites
        .into_iter()
        .find(|f| f.id == favorite.id)
        .ok_or_else(|| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Favorite not found".to_string(),
            )
        })?;

    Ok(Json(fav_with_listing))
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
    State(state): State<AppState>,
    Path(listing_id): Path<Uuid>,
) -> Result<axum::http::StatusCode, (axum::http::StatusCode, String)> {
    // TODO: Get user from auth context
    let user_id = Uuid::nil(); // Placeholder

    let removed = state
        .portal_repo
        .remove_favorite(user_id, listing_id)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to remove favorite: {}", e),
            )
        })?;

    if !removed {
        return Err((
            axum::http::StatusCode::NOT_FOUND,
            "Favorite not found".to_string(),
        ));
    }

    Ok(axum::http::StatusCode::NO_CONTENT)
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
    State(state): State<AppState>,
    Path(listing_id): Path<Uuid>,
) -> Result<Json<CheckFavoriteResponse>, (axum::http::StatusCode, String)> {
    // TODO: Get user from auth context
    let user_id = Uuid::nil(); // Placeholder

    let is_favorited = state
        .portal_repo
        .is_favorited(user_id, listing_id)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to check favorite: {}", e),
            )
        })?;

    Ok(Json(CheckFavoriteResponse { is_favorited }))
}
