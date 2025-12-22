//! Public listing routes - search and view.

use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

/// Create listings router.
pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(search))
        .route("/:id", get(get_listing))
}

/// Listing search request.
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
    /// Number of rooms
    pub rooms: Option<i32>,
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
    pub description: String,
    /// Price
    pub price: i64,
    /// Currency
    pub currency: String,
    /// Area in m2
    pub area: i32,
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

/// Full listing detail.
#[derive(Debug, Serialize, ToSchema)]
pub struct ListingDetail {
    /// Listing ID
    pub id: Uuid,
    /// Title
    pub title: String,
    /// Full description
    pub description: String,
    /// Price
    pub price: i64,
    /// Currency
    pub currency: String,
    /// Area in m2
    pub area: i32,
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
pub async fn search(Query(req): Query<ListingSearchRequest>) -> Json<ListingSearchResponse> {
    // TODO: Implement actual search
    tracing::info!(?req, "Listing search");

    Json(ListingSearchResponse {
        listings: vec![],
        total: 0,
        page: req.page.unwrap_or(1),
        limit: req.limit.unwrap_or(20),
    })
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
pub async fn get_listing(Path(id): Path<Uuid>) -> Json<ListingDetail> {
    // TODO: Implement actual listing retrieval
    tracing::info!(%id, "Get listing detail");

    Json(ListingDetail {
        id,
        title: "Sample Listing".to_string(),
        description: "A beautiful property".to_string(),
        price: 150000,
        currency: "EUR".to_string(),
        area: 75,
        rooms: Some(3),
        bathrooms: Some(1),
        floor: Some(2),
        total_floors: Some(5),
        address: "Sample Street 123".to_string(),
        city: "Bratislava".to_string(),
        country: "SK".to_string(),
        latitude: Some(48.1486),
        longitude: Some(17.1077),
        property_type: "apartment".to_string(),
        transaction_type: "sale".to_string(),
        photos: vec![],
        features: vec!["parking".to_string(), "balcony".to_string()],
        published_at: "2024-01-01T00:00:00Z".to_string(),
        view_count: 0,
    })
}
