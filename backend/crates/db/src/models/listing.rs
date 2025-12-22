//! Listing model (Epic 15) - Real estate listing management.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Listing status enum.
pub mod listing_status {
    pub const DRAFT: &str = "draft";
    pub const ACTIVE: &str = "active";
    pub const PAUSED: &str = "paused";
    pub const SOLD: &str = "sold";
    pub const RENTED: &str = "rented";
    pub const ARCHIVED: &str = "archived";
}

/// Transaction type enum.
pub mod transaction_type {
    pub const SALE: &str = "sale";
    pub const RENT: &str = "rent";
}

/// Property type enum for listings (maps from unit_type).
pub mod property_type {
    pub const APARTMENT: &str = "apartment";
    pub const HOUSE: &str = "house";
    pub const COMMERCIAL: &str = "commercial";
    pub const LAND: &str = "land";
    pub const PARKING: &str = "parking";
    pub const STORAGE: &str = "storage";
    pub const OTHER: &str = "other";
}

/// Currency enum.
pub mod currency {
    pub const EUR: &str = "EUR";
    pub const CZK: &str = "CZK";
}

/// Listing entity from database.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Listing {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub created_by: Uuid,

    // Listing status
    pub status: String,
    pub transaction_type: String,

    // Property details (can override unit data)
    pub title: String,
    pub description: Option<String>,
    pub property_type: String,
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub size_sqm: Option<Decimal>,
    pub rooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub floor: Option<i32>,
    pub total_floors: Option<i32>,

    // Address (copied from unit/building or manually entered)
    pub street: String,
    pub city: String,
    pub postal_code: String,
    pub country: String,

    // Location coordinates (optional)
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub latitude: Option<Decimal>,
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub longitude: Option<Decimal>,

    // Pricing
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub price: Decimal,
    pub currency: String,
    pub is_negotiable: bool,

    // Features (stored as JSON array)
    pub features: serde_json::Value,

    // Timestamps
    pub published_at: Option<DateTime<Utc>>,
    pub sold_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Listing {
    /// Check if listing is in draft status.
    pub fn is_draft(&self) -> bool {
        self.status == listing_status::DRAFT
    }

    /// Check if listing is active (visible to public).
    pub fn is_active(&self) -> bool {
        self.status == listing_status::ACTIVE
    }

    /// Check if listing can be edited.
    pub fn can_edit(&self) -> bool {
        matches!(
            self.status.as_str(),
            listing_status::DRAFT | listing_status::ACTIVE | listing_status::PAUSED
        )
    }

    /// Check if listing can be published.
    pub fn can_publish(&self) -> bool {
        self.status == listing_status::DRAFT || self.status == listing_status::PAUSED
    }

    /// Get features as a list of strings.
    pub fn feature_list(&self) -> Vec<String> {
        self.features
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// Summary view of a listing (for list views).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ListingSummary {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub title: String,
    pub property_type: String,
    pub transaction_type: String,
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub price: Decimal,
    pub currency: String,
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub size_sqm: Option<Decimal>,
    pub rooms: Option<i32>,
    pub city: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    #[sqlx(default)]
    pub photo_url: Option<String>,
}

/// Listing with full details including photos.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ListingWithDetails {
    #[serde(flatten)]
    pub listing: Listing,
    pub photos: Vec<ListingPhoto>,
    pub syndications: Vec<ListingSyndication>,
}

/// Data for creating a new listing.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateListing {
    pub unit_id: Option<Uuid>,
    pub transaction_type: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(default = "default_property_type")]
    pub property_type: String,
    pub size_sqm: Option<Decimal>,
    pub rooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub floor: Option<i32>,
    pub total_floors: Option<i32>,
    pub street: String,
    pub city: String,
    pub postal_code: String,
    #[serde(default = "default_country")]
    pub country: String,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub price: Decimal,
    #[serde(default = "default_currency")]
    pub currency: String,
    #[serde(default)]
    pub is_negotiable: bool,
    #[serde(default)]
    pub features: Vec<String>,
}

fn default_property_type() -> String {
    property_type::APARTMENT.to_string()
}

fn default_country() -> String {
    "SK".to_string()
}

fn default_currency() -> String {
    currency::EUR.to_string()
}

/// Data for creating a listing from unit data (pre-populated).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateListingFromUnit {
    pub unit_id: Uuid,
    pub transaction_type: String,
    pub price: Decimal,
    #[serde(default = "default_currency")]
    pub currency: String,
    #[serde(default)]
    pub is_negotiable: bool,
    // Optional overrides (if not provided, use unit/building data)
    pub title: Option<String>,
    pub description: Option<String>,
    pub rooms: Option<i32>,
    pub bathrooms: Option<i32>,
    #[serde(default)]
    pub features: Vec<String>,
}

/// Data for updating a listing.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct UpdateListing {
    pub title: Option<String>,
    pub description: Option<String>,
    pub property_type: Option<String>,
    pub size_sqm: Option<Decimal>,
    pub rooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub floor: Option<i32>,
    pub total_floors: Option<i32>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub price: Option<Decimal>,
    pub currency: Option<String>,
    pub is_negotiable: Option<bool>,
    pub features: Option<Vec<String>>,
}

/// Data for updating listing status.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateListingStatus {
    pub status: String,
}

/// Query parameters for listing search.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, utoipa::IntoParams)]
pub struct ListingListQuery {
    pub status: Option<String>,
    pub transaction_type: Option<String>,
    pub property_type: Option<String>,
    pub city: Option<String>,
    pub price_min: Option<Decimal>,
    pub price_max: Option<Decimal>,
    pub rooms_min: Option<i32>,
    pub rooms_max: Option<i32>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

// ============================================
// Listing Photos
// ============================================

/// Listing photo entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ListingPhoto {
    pub id: Uuid,
    pub listing_id: Uuid,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub medium_url: Option<String>,
    pub display_order: i32,
    pub alt_text: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Data for adding a photo to a listing.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateListingPhoto {
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub medium_url: Option<String>,
    pub display_order: Option<i32>,
    pub alt_text: Option<String>,
}

/// Data for reordering photos.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReorderPhotos {
    pub photo_ids: Vec<Uuid>,
}

// ============================================
// Listing Syndication (Multi-Portal Publishing)
// ============================================

/// Syndication status enum.
pub mod syndication_status {
    pub const PENDING: &str = "pending";
    pub const SYNCED: &str = "synced";
    pub const FAILED: &str = "failed";
    pub const REMOVED: &str = "removed";
}

/// Portal enum.
pub mod portal {
    pub const REALITY_PORTAL: &str = "reality_portal";
    pub const SREALITY: &str = "sreality";
    pub const BEZREALITKY: &str = "bezrealitky";
    pub const NEHNUTELNOSTI: &str = "nehnutelnosti";
}

/// Listing syndication entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ListingSyndication {
    pub id: Uuid,
    pub listing_id: Uuid,
    pub portal: String,
    pub external_id: Option<String>,
    pub status: String,
    pub last_error: Option<String>,
    pub synced_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Data for creating a syndication.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateSyndication {
    pub portals: Vec<String>,
}

/// Syndication result for a single portal.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyndicationResult {
    pub portal: String,
    pub success: bool,
    pub external_id: Option<String>,
    pub error: Option<String>,
}

/// Response for publish operation.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PublishListingResponse {
    pub listing_id: Uuid,
    pub status: String,
    pub syndication_results: Vec<SyndicationResult>,
}

// ============================================
// Listing Statistics
// ============================================

/// Listing statistics for dashboard.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ListingStatistics {
    pub total_listings: i64,
    pub active_listings: i64,
    pub draft_listings: i64,
    pub sold_listings: i64,
    pub rented_listings: i64,
    pub by_property_type: Vec<PropertyTypeCount>,
}

/// Count by property type.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct PropertyTypeCount {
    pub property_type: String,
    pub count: i64,
}
