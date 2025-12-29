//! Public listing routes - search and view (Story 16.1).

use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use db::models::{PublicListingQuery, PublicListingSummary};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

/// Create listings router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(search))
        .route("/:id", get(get_listing))
        .route("/suggestions", get(get_suggestions))
}

/// Listing search request (maps to PublicListingQuery).
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct ListingSearchRequest {
    /// Search query (address, city, description)
    pub q: Option<String>,
    /// Property type (apartment, house, land, commercial)
    pub property_type: Option<String>,
    /// Transaction type (sale, rent)
    pub transaction_type: Option<String>,
    /// Minimum price
    pub price_min: Option<i64>,
    /// Maximum price
    pub price_max: Option<i64>,
    /// Minimum area (m2)
    pub area_min: Option<i32>,
    /// Maximum area (m2)
    pub area_max: Option<i32>,
    /// Minimum rooms
    pub rooms_min: Option<i32>,
    /// Maximum rooms
    pub rooms_max: Option<i32>,
    /// City
    pub city: Option<String>,
    /// Country code (SK, CZ, etc.)
    pub country: Option<String>,
    /// Page number
    pub page: Option<i32>,
    /// Page size
    pub limit: Option<i32>,
    /// Sort by (price_asc, price_desc, date_desc, area_asc)
    pub sort: Option<String>,
}

impl From<ListingSearchRequest> for PublicListingQuery {
    fn from(req: ListingSearchRequest) -> Self {
        Self {
            q: req.q,
            property_type: req.property_type,
            transaction_type: req.transaction_type,
            price_min: req.price_min,
            price_max: req.price_max,
            area_min: req.area_min,
            area_max: req.area_max,
            rooms_min: req.rooms_min,
            rooms_max: req.rooms_max,
            city: req.city,
            country: req.country,
            page: req.page,
            limit: req.limit,
            sort: req.sort,
        }
    }
}

/// Listing search response.
#[derive(Debug, Serialize, ToSchema)]
pub struct ListingSearchResponse {
    /// List of listings
    pub listings: Vec<ListingSummary>,
    /// Total count
    pub total: i64,
    /// Current page
    pub page: i32,
    /// Page size
    pub limit: i32,
}

/// Listing summary for search results.
#[derive(Debug, Serialize, ToSchema)]
pub struct ListingSummary {
    /// Listing ID
    pub id: Uuid,
    /// Title
    pub title: String,
    /// Short description
    pub description: Option<String>,
    /// Price
    pub price: i64,
    /// Currency
    pub currency: String,
    /// Area in m2
    pub area: Option<i32>,
    /// Number of rooms
    pub rooms: Option<i32>,
    /// City
    pub city: String,
    /// Main photo URL
    pub photo_url: Option<String>,
    /// Property type
    pub property_type: String,
    /// Transaction type
    pub transaction_type: String,
    /// Published date
    pub published_at: String,
}

impl From<PublicListingSummary> for ListingSummary {
    fn from(summary: PublicListingSummary) -> Self {
        Self {
            id: summary.id,
            title: summary.title,
            description: summary.description,
            price: summary.price,
            currency: summary.currency,
            area: summary.size_sqm,
            rooms: summary.rooms,
            city: summary.city,
            photo_url: summary.photo_url,
            property_type: summary.property_type,
            transaction_type: summary.transaction_type,
            published_at: summary.published_at.to_rfc3339(),
        }
    }
}

/// Full listing detail.
#[derive(Debug, Serialize, ToSchema)]
pub struct ListingDetail {
    /// Listing ID
    pub id: Uuid,
    /// Title
    pub title: String,
    /// Full description
    pub description: Option<String>,
    /// Price
    pub price: i64,
    /// Currency
    pub currency: String,
    /// Area in m2
    pub area: Option<i32>,
    /// Number of rooms
    pub rooms: Option<i32>,
    /// Number of bathrooms
    pub bathrooms: Option<i32>,
    /// Floor number
    pub floor: Option<i32>,
    /// Total floors in building
    pub total_floors: Option<i32>,
    /// Address
    pub address: String,
    /// City
    pub city: String,
    /// Country
    pub country: String,
    /// Latitude
    pub latitude: Option<f64>,
    /// Longitude
    pub longitude: Option<f64>,
    /// Property type
    pub property_type: String,
    /// Transaction type
    pub transaction_type: String,
    /// Photo URLs
    pub photos: Vec<String>,
    /// Features (parking, balcony, etc.)
    pub features: Vec<String>,
    /// Published date
    pub published_at: String,
    /// View count
    pub view_count: i64,
}

/// Search suggestions response.
#[derive(Debug, Serialize, ToSchema)]
pub struct SuggestionsResponse {
    /// Nearby cities
    pub cities: Vec<String>,
    /// Popular searches
    pub popular_searches: Vec<String>,
}

/// Search listings.
#[utoipa::path(
    get,
    path = "/api/v1/listings",
    tag = "Listings",
    params(ListingSearchRequest),
    responses(
        (status = 200, description = "Search results", body = ListingSearchResponse)
    )
)]
pub async fn search(
    State(state): State<AppState>,
    Query(req): Query<ListingSearchRequest>,
) -> Result<Json<ListingSearchResponse>, (axum::http::StatusCode, String)> {
    let page = req.page.unwrap_or(1);
    let limit = req.limit.unwrap_or(20);
    let query: PublicListingQuery = req.into();

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

    // Count total
    let total = state
        .portal_repo
        .count_listings(&query)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to count listings: {}", e),
            )
        })?;

    // Convert to response types
    let listings: Vec<ListingSummary> = listings.into_iter().map(Into::into).collect();

    Ok(Json(ListingSearchResponse {
        listings,
        total,
        page,
        limit,
    }))
}

/// Get listing detail.
#[utoipa::path(
    get,
    path = "/api/v1/listings/{id}",
    tag = "Listings",
    params(
        ("id" = Uuid, Path, description = "Listing ID")
    ),
    responses(
        (status = 200, description = "Listing detail", body = ListingDetail),
        (status = 404, description = "Listing not found")
    )
)]
pub async fn get_listing(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ListingDetail>, (axum::http::StatusCode, String)> {
    tracing::info!(%id, "Get listing detail");

    // Query the database directly for the specific listing by ID
    let listing = state.portal_repo.get_listing_by_id(id).await.map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch listing: {}", e),
        )
    })?;

    match listing {
        Some(l) => {
            // Track the view
            let _ = state.reality_portal_repo.track_view(id, "website").await;

            Ok(Json(ListingDetail {
                id: l.id,
                title: l.title,
                description: l.description,
                price: l.price,
                currency: l.currency,
                area: l.size_sqm,
                rooms: l.rooms,
                bathrooms: None,        // Would need additional query
                floor: None,            // Would need additional query
                total_floors: None,     // Would need additional query
                address: String::new(), // Would need additional query
                city: l.city,
                country: String::new(), // Would need additional query
                latitude: None,
                longitude: None,
                property_type: l.property_type,
                transaction_type: l.transaction_type,
                photos: l.photo_url.map(|url| vec![url]).unwrap_or_default(),
                features: vec![],
                published_at: l.published_at.to_rfc3339(),
                view_count: 0, // Would need analytics query
            }))
        }
        None => Err((
            axum::http::StatusCode::NOT_FOUND,
            "Listing not found".to_string(),
        )),
    }
}

/// Get search suggestions.
#[utoipa::path(
    get,
    path = "/api/v1/listings/suggestions",
    tag = "Listings",
    params(
        ("city" = Option<String>, Query, description = "Current city for nearby suggestions")
    ),
    responses(
        (status = 200, description = "Search suggestions", body = SuggestionsResponse)
    )
)]
pub async fn get_suggestions(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<SuggestionsResponse>, (axum::http::StatusCode, String)> {
    let city = params
        .get("city")
        .map(|s| s.as_str())
        .unwrap_or("Bratislava");

    let cities = state
        .portal_repo
        .get_nearby_cities(city, 10)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get cities: {}", e),
            )
        })?;

    Ok(Json(SuggestionsResponse {
        cities,
        popular_searches: vec![
            "2-izbový byt Bratislava".to_string(),
            "Dom Košice".to_string(),
            "Pozemok Žilina".to_string(),
        ],
    }))
}
