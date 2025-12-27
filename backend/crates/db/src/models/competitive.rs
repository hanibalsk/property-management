//! Competitive feature models (Epic 70) - Virtual tours, dynamic pricing, neighborhood insights, comparables.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================
// Story 70.1: Virtual Tour Integration
// ============================================

/// Tour type enum.
pub mod tour_type {
    pub const PHOTO_360: &str = "photo_360";
    pub const MATTERPORT: &str = "matterport";
    pub const VIDEO: &str = "video";
    pub const EXTERNAL_EMBED: &str = "external_embed";
}

/// Virtual tour entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct VirtualTour {
    pub id: Uuid,
    pub listing_id: Uuid,
    pub tour_type: String,
    pub title: Option<String>,
    pub description: Option<String>,

    // For 360 photos
    pub photo_url: Option<String>,

    // For Matterport or external embeds
    pub embed_url: Option<String>,
    pub external_id: Option<String>,

    // For video tours
    pub video_url: Option<String>,
    pub thumbnail_url: Option<String>,

    // Ordering and display
    pub display_order: i32,
    pub is_featured: bool,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Tour hotspot for interactive navigation.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct TourHotspot {
    pub id: Uuid,
    pub tour_id: Uuid,
    pub label: String,
    pub description: Option<String>,
    /// Position in tour (x, y coordinates as percentage)
    pub position_x: Decimal,
    pub position_y: Decimal,
    /// Link to another tour or action
    pub link_to_tour_id: Option<Uuid>,
    pub action_type: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Data for creating a virtual tour.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateVirtualTour {
    pub tour_type: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub photo_url: Option<String>,
    pub embed_url: Option<String>,
    pub external_id: Option<String>,
    pub video_url: Option<String>,
    pub thumbnail_url: Option<String>,
    #[serde(default)]
    pub is_featured: bool,
}

/// Data for updating a virtual tour.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct UpdateVirtualTour {
    pub title: Option<String>,
    pub description: Option<String>,
    pub photo_url: Option<String>,
    pub embed_url: Option<String>,
    pub video_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub is_featured: Option<bool>,
}

/// Data for creating a hotspot.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateTourHotspot {
    pub label: String,
    pub description: Option<String>,
    pub position_x: Decimal,
    pub position_y: Decimal,
    pub link_to_tour_id: Option<Uuid>,
    pub action_type: Option<String>,
}

/// Data for reordering tours.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReorderTours {
    pub tour_ids: Vec<Uuid>,
}

/// Virtual tour with hotspots.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VirtualTourWithHotspots {
    #[serde(flatten)]
    pub tour: VirtualTour,
    pub hotspots: Vec<TourHotspot>,
}

// ============================================
// Story 70.2: Dynamic Pricing Suggestions
// ============================================

/// Confidence level for pricing suggestions.
pub mod confidence_level {
    pub const HIGH: &str = "high";
    pub const MEDIUM: &str = "medium";
    pub const LOW: &str = "low";
}

/// Pricing suggestion entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct PricingSuggestion {
    pub id: Uuid,
    pub listing_id: Uuid,

    // Suggested price range
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub suggested_price_low: Decimal,
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub suggested_price_mid: Decimal,
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub suggested_price_high: Decimal,
    pub currency: String,

    // Confidence and reasoning
    pub confidence_level: String,
    pub confidence_score: Option<i32>, // 0-100

    // Analysis factors
    pub comparables_count: i32,
    pub market_trend: String, // "rising", "stable", "falling"
    pub seasonal_adjustment: Option<Decimal>,

    // Timestamps
    pub calculated_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
}

/// Factor contributing to price suggestion.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct PricingFactor {
    pub id: Uuid,
    pub suggestion_id: Uuid,
    pub factor_type: String,
    pub factor_name: String,
    pub impact: Decimal, // Positive or negative percentage impact
    pub explanation: String,
}

/// Price history for trend analysis.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct PriceHistory {
    pub id: Uuid,
    pub listing_id: Option<Uuid>,
    pub property_type: String,
    pub city: String,
    pub postal_code: Option<String>,
    pub transaction_type: String,
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub price: Decimal,
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub price_per_sqm: Option<Decimal>,
    pub currency: String,
    pub recorded_at: DateTime<Utc>,
}

/// Request for pricing analysis.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PricingAnalysisRequest {
    pub listing_id: Uuid,
    #[serde(default)]
    pub force_refresh: bool,
}

/// Pricing analysis response with full details.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PricingAnalysisResponse {
    pub suggestion: PricingSuggestion,
    pub factors: Vec<PricingFactor>,
    pub price_history: Vec<PriceHistory>,
    pub comparables_used: Vec<ComparablePropertySummary>,
}

// ============================================
// Story 70.3: Neighborhood Insights
// ============================================

/// Neighborhood insights entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct NeighborhoodInsights {
    pub id: Uuid,
    pub listing_id: Option<Uuid>,
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub latitude: Decimal,
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub longitude: Decimal,

    // Scores (0-100)
    pub walk_score: Option<i32>,
    pub transit_score: Option<i32>,
    pub bike_score: Option<i32>,

    // Demographics (optional)
    pub population: Option<i64>,
    pub median_age: Option<Decimal>,
    pub median_income: Option<Decimal>,

    // Safety (optional, if data available)
    pub crime_index: Option<i32>,
    pub safety_rating: Option<String>,

    // Data source attribution
    pub data_sources: serde_json::Value,

    // Timestamps
    pub fetched_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
}

/// Nearby amenity entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct NearbyAmenity {
    pub id: Uuid,
    pub insights_id: Uuid,
    pub category: String, // school, shop, transit, restaurant, healthcare, park
    pub name: String,
    pub address: Option<String>,
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub distance_meters: Decimal,
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub latitude: Decimal,
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub longitude: Decimal,
    pub rating: Option<Decimal>,
    pub details: Option<serde_json::Value>,
}

/// Amenity category enum.
pub mod amenity_category {
    pub const SCHOOL: &str = "school";
    pub const KINDERGARTEN: &str = "kindergarten";
    pub const UNIVERSITY: &str = "university";
    pub const SUPERMARKET: &str = "supermarket";
    pub const SHOP: &str = "shop";
    pub const RESTAURANT: &str = "restaurant";
    pub const CAFE: &str = "cafe";
    pub const TRANSIT_STOP: &str = "transit_stop";
    pub const TRAIN_STATION: &str = "train_station";
    pub const HEALTHCARE: &str = "healthcare";
    pub const PHARMACY: &str = "pharmacy";
    pub const PARK: &str = "park";
    pub const GYM: &str = "gym";
    pub const BANK: &str = "bank";
}

/// Request for neighborhood insights.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NeighborhoodInsightsRequest {
    pub listing_id: Option<Uuid>,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    #[serde(default)]
    pub force_refresh: bool,
}

/// Full neighborhood insights response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NeighborhoodInsightsResponse {
    pub insights: NeighborhoodInsights,
    pub amenities: Vec<NearbyAmenity>,
    pub amenities_by_category: std::collections::HashMap<String, Vec<NearbyAmenity>>,
}

// ============================================
// Story 70.4: Comparable Sales/Rentals
// ============================================

/// Comparable property entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ComparableProperty {
    pub id: Uuid,
    pub source_listing_id: Uuid,
    pub comparable_listing_id: Option<Uuid>, // If from our system
    pub external_id: Option<String>,         // If from external source

    // Property details
    pub property_type: String,
    pub transaction_type: String,
    pub street: Option<String>,
    pub city: String,
    pub postal_code: Option<String>,

    // Size and features
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub size_sqm: Decimal,
    pub rooms: Option<i32>,
    pub floor: Option<i32>,

    // Pricing
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub price: Decimal,
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub price_per_sqm: Decimal,
    pub currency: String,

    // Location
    #[sqlx(try_from = "rust_decimal::Decimal")]
    pub distance_meters: Decimal,

    // Similarity score (0-100)
    pub similarity_score: i32,

    // Transaction date
    pub transaction_date: Option<DateTime<Utc>>,
    pub is_active: bool,

    // Timestamps
    pub found_at: DateTime<Utc>,
}

/// Summary of comparable for responses.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ComparablePropertySummary {
    pub id: Uuid,
    pub property_type: String,
    pub city: String,
    pub size_sqm: Decimal,
    pub rooms: Option<i32>,
    pub price: Decimal,
    pub price_per_sqm: Decimal,
    pub currency: String,
    pub distance_meters: Decimal,
    pub similarity_score: i32,
    pub transaction_date: Option<DateTime<Utc>>,
    pub is_active: bool,
}

impl From<ComparableProperty> for ComparablePropertySummary {
    fn from(cp: ComparableProperty) -> Self {
        Self {
            id: cp.id,
            property_type: cp.property_type,
            city: cp.city,
            size_sqm: cp.size_sqm,
            rooms: cp.rooms,
            price: cp.price,
            price_per_sqm: cp.price_per_sqm,
            currency: cp.currency,
            distance_meters: cp.distance_meters,
            similarity_score: cp.similarity_score,
            transaction_date: cp.transaction_date,
            is_active: cp.is_active,
        }
    }
}

/// Comparison table entry for side-by-side view.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ComparisonTableEntry {
    pub feature: String,
    pub source_value: String,
    pub comparable_values: Vec<String>,
}

/// Request for comparables.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, utoipa::IntoParams)]
pub struct ComparablesRequest {
    /// Maximum distance in meters (default 2000)
    pub max_distance_meters: Option<i32>,
    /// Maximum number of comparables to return (default 10)
    pub limit: Option<i32>,
    /// Minimum similarity score (0-100, default 50)
    pub min_similarity_score: Option<i32>,
    /// Include only active listings
    pub active_only: Option<bool>,
}

impl Default for ComparablesRequest {
    fn default() -> Self {
        Self {
            max_distance_meters: Some(2000),
            limit: Some(10),
            min_similarity_score: Some(50),
            active_only: Some(false),
        }
    }
}

/// Full comparables response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ComparablesResponse {
    pub listing_id: Uuid,
    pub comparables: Vec<ComparablePropertySummary>,
    pub comparison_table: Vec<ComparisonTableEntry>,
    pub average_price_per_sqm: Decimal,
    pub price_range: PriceRange,
}

/// Price range statistics.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PriceRange {
    pub min: Decimal,
    pub max: Decimal,
    pub median: Decimal,
    pub currency: String,
}

// ============================================
// Combined Competitive Features Response
// ============================================

/// Complete competitive analysis for a listing.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompetitiveAnalysis {
    pub listing_id: Uuid,
    pub virtual_tours: Vec<VirtualTourWithHotspots>,
    pub pricing_analysis: Option<PricingAnalysisResponse>,
    pub neighborhood: Option<NeighborhoodInsightsResponse>,
    pub comparables: Option<ComparablesResponse>,
    pub generated_at: DateTime<Utc>,
}

/// Competitive features status for a listing.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompetitiveFeaturesStatus {
    pub listing_id: Uuid,
    pub has_virtual_tours: bool,
    pub virtual_tour_count: i32,
    pub has_pricing_analysis: bool,
    pub pricing_analysis_valid: bool,
    pub has_neighborhood_insights: bool,
    pub neighborhood_insights_valid: bool,
    pub has_comparables: bool,
    pub comparables_count: i32,
}
