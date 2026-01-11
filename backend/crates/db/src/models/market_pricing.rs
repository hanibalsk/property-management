//! Market pricing models (Epic 132: Dynamic Rent Pricing & Market Analytics).
//! Provides AI-powered pricing recommendations and comparative market analysis.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// =============================================================================
// ENUMS
// =============================================================================

pub mod pricing_source {
    pub const MANUAL: &str = "manual";
    pub const API: &str = "api";
    pub const SCRAPER: &str = "scraper";
    pub const IMPORT: &str = "import";
}

pub mod pricing_recommendation_status {
    pub const PENDING: &str = "pending";
    pub const ACCEPTED: &str = "accepted";
    pub const REJECTED: &str = "rejected";
    pub const EXPIRED: &str = "expired";
}

// =============================================================================
// MARKET REGIONS (Story 132.1)
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct MarketRegion {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub country_code: String,
    pub city: String,
    pub postal_codes: Vec<String>,
    pub center_lat: Option<Decimal>,
    pub center_lng: Option<Decimal>,
    pub radius_km: Decimal,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateMarketRegion {
    pub name: String,
    pub country_code: String,
    pub city: String,
    #[serde(default)]
    pub postal_codes: Vec<String>,
    pub center_lat: Option<Decimal>,
    pub center_lng: Option<Decimal>,
    #[serde(default = "default_radius")]
    pub radius_km: Decimal,
}

fn default_radius() -> Decimal {
    Decimal::new(5, 0)
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateMarketRegion {
    pub name: Option<String>,
    pub postal_codes: Option<Vec<String>>,
    pub center_lat: Option<Decimal>,
    pub center_lng: Option<Decimal>,
    pub radius_km: Option<Decimal>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct MarketRegionSummary {
    pub id: Uuid,
    pub name: String,
    pub country_code: String,
    pub city: String,
    pub is_active: bool,
    pub data_points_count: i64,
    pub last_data_collected: Option<DateTime<Utc>>,
}

// =============================================================================
// MARKET DATA POINTS (Story 132.1)
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct MarketDataPoint {
    pub id: Uuid,
    pub region_id: Uuid,
    pub collected_at: DateTime<Utc>,
    pub source: String,
    pub source_reference: Option<String>,

    // Property characteristics
    pub property_type: String,
    pub size_sqm: Decimal,
    pub rooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub floor: Option<i32>,
    pub has_parking: Option<bool>,
    pub has_balcony: Option<bool>,
    pub has_elevator: Option<bool>,
    pub year_built: Option<i32>,

    // Pricing data
    pub monthly_rent: Decimal,
    pub currency: String,
    pub price_per_sqm: Option<Decimal>,

    // Location
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub postal_code: Option<String>,
    pub district: Option<String>,

    // Metadata
    pub listing_date: Option<NaiveDate>,
    pub days_on_market: Option<i32>,
    pub is_furnished: Option<bool>,
    pub amenities: Option<JsonValue>,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateMarketDataPoint {
    pub region_id: Uuid,
    #[serde(default = "default_source")]
    pub source: String,
    pub source_reference: Option<String>,

    pub property_type: String,
    pub size_sqm: Decimal,
    pub rooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub floor: Option<i32>,
    pub has_parking: Option<bool>,
    pub has_balcony: Option<bool>,
    pub has_elevator: Option<bool>,
    pub year_built: Option<i32>,

    pub monthly_rent: Decimal,
    #[serde(default = "default_currency")]
    pub currency: String,

    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub postal_code: Option<String>,
    pub district: Option<String>,

    pub listing_date: Option<NaiveDate>,
    pub days_on_market: Option<i32>,
    pub is_furnished: Option<bool>,
    pub amenities: Option<JsonValue>,
}

fn default_source() -> String {
    pricing_source::MANUAL.to_string()
}

fn default_currency() -> String {
    "EUR".to_string()
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct MarketDataQuery {
    pub region_id: Option<Uuid>,
    pub property_type: Option<String>,
    pub min_size_sqm: Option<Decimal>,
    pub max_size_sqm: Option<Decimal>,
    pub min_rent: Option<Decimal>,
    pub max_rent: Option<Decimal>,
    pub rooms: Option<i32>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

// =============================================================================
// MARKET STATISTICS (Story 132.1)
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct MarketStatistics {
    pub id: Uuid,
    pub region_id: Uuid,
    pub property_type: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,

    pub avg_rent: Decimal,
    pub median_rent: Decimal,
    pub min_rent: Decimal,
    pub max_rent: Decimal,

    pub avg_price_per_sqm: Decimal,
    pub median_price_per_sqm: Decimal,

    pub vacancy_rate: Option<Decimal>,
    pub avg_days_on_market: Option<Decimal>,
    pub sample_size: i32,

    pub rent_change_pct: Option<Decimal>,
    pub rent_change_vs_last_year: Option<Decimal>,

    pub currency: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct MarketStatisticsSummary {
    pub region_id: Uuid,
    pub region_name: String,
    pub property_type: String,
    pub avg_rent: Decimal,
    pub avg_price_per_sqm: Decimal,
    pub vacancy_rate: Option<Decimal>,
    pub sample_size: i32,
    pub period: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct GenerateStatisticsRequest {
    pub region_id: Uuid,
    pub property_type: Option<String>,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
}

// =============================================================================
// PRICING RECOMMENDATIONS (Story 132.2)
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PricingRecommendation {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub generated_at: DateTime<Utc>,

    pub min_price: Decimal,
    pub optimal_price: Decimal,
    pub max_price: Decimal,
    pub currency: String,

    pub confidence_score: Decimal,
    pub status: String,
    pub expires_at: DateTime<Utc>,

    pub factors: JsonValue,

    pub comparables_count: i32,
    pub market_stats_id: Option<Uuid>,

    pub accepted_price: Option<Decimal>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub accepted_by: Option<Uuid>,
    pub rejection_reason: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct RequestPricingRecommendation {
    pub unit_id: Uuid,
    #[serde(default = "default_currency")]
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PricingRecommendationWithDetails {
    pub recommendation: PricingRecommendation,
    pub factors_explanation: Vec<PricingFactor>,
    pub comparables: Vec<MarketComparable>,
    pub market_stats: Option<MarketStatisticsSummary>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PricingFactor {
    pub name: String,
    pub impact: Decimal,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct MarketComparable {
    pub address: String,
    pub property_type: String,
    pub size_sqm: Decimal,
    pub monthly_rent: Decimal,
    pub price_per_sqm: Option<Decimal>,
    pub distance_km: Decimal,
    pub similarity_score: Decimal,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AcceptPricingRecommendation {
    pub accepted_price: Decimal,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct RejectPricingRecommendation {
    pub rejection_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PricingRecommendationSummary {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub optimal_price: Decimal,
    pub confidence_score: Decimal,
    pub status: String,
    pub generated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

// =============================================================================
// UNIT PRICING HISTORY (Story 132.3)
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UnitPricingHistory {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub monthly_rent: Decimal,
    pub currency: String,
    pub recommendation_id: Option<Uuid>,
    pub change_reason: Option<String>,
    pub changed_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct RecordPriceChange {
    pub unit_id: Uuid,
    pub effective_date: NaiveDate,
    pub monthly_rent: Decimal,
    #[serde(default = "default_currency")]
    pub currency: String,
    pub recommendation_id: Option<Uuid>,
    pub change_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PricingHistoryEntry {
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub monthly_rent: Decimal,
    pub change_reason: Option<String>,
    pub was_ai_recommended: bool,
}

// =============================================================================
// COMPARATIVE MARKET ANALYSIS (Story 132.4)
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ComparativeMarketAnalysis {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub created_by: Uuid,
    pub name: String,

    pub region_id: Option<Uuid>,
    pub property_type: Option<String>,

    pub avg_price_per_sqm: Option<Decimal>,
    pub avg_rental_yield: Option<Decimal>,
    pub appreciation_trend: Option<Decimal>,

    pub analysis_data: JsonValue,
    pub properties_compared: Vec<Uuid>,

    pub is_archived: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateComparativeMarketAnalysis {
    pub name: String,
    pub region_id: Option<Uuid>,
    pub property_type: Option<String>,
    pub properties_to_compare: Vec<Uuid>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateComparativeMarketAnalysis {
    pub name: Option<String>,
    pub is_archived: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmaSummary {
    pub id: Uuid,
    pub name: String,
    pub property_type: Option<String>,
    pub avg_price_per_sqm: Option<Decimal>,
    pub avg_rental_yield: Option<Decimal>,
    pub properties_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmaPropertyComparison {
    pub id: Uuid,
    pub cma_id: Uuid,
    pub unit_id: Option<Uuid>,

    pub address: String,
    pub property_type: String,
    pub size_sqm: Decimal,
    pub rooms: Option<i32>,
    pub year_built: Option<i32>,

    pub monthly_rent: Option<Decimal>,
    pub sale_price: Option<Decimal>,
    pub price_per_sqm: Option<Decimal>,
    pub rental_yield: Option<Decimal>,
    pub currency: String,

    pub distance_km: Option<Decimal>,
    pub similarity_score: Option<Decimal>,
    pub notes: Option<String>,

    pub source: String,
    pub source_url: Option<String>,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AddCmaProperty {
    pub unit_id: Option<Uuid>,
    pub address: String,
    pub property_type: String,
    pub size_sqm: Decimal,
    pub rooms: Option<i32>,
    pub year_built: Option<i32>,
    pub monthly_rent: Option<Decimal>,
    pub sale_price: Option<Decimal>,
    #[serde(default = "default_currency")]
    pub currency: String,
    pub distance_km: Option<Decimal>,
    pub notes: Option<String>,
    #[serde(default = "default_source")]
    pub source: String,
    pub source_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CmaWithProperties {
    pub analysis: ComparativeMarketAnalysis,
    pub properties: Vec<CmaPropertyComparison>,
    pub summary: CmaAnalysisSummary,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CmaAnalysisSummary {
    pub total_properties: i32,
    pub avg_size_sqm: Decimal,
    pub avg_rent: Option<Decimal>,
    pub avg_price_per_sqm: Option<Decimal>,
    pub price_range: Option<PriceRange>,
    pub rental_yield_range: Option<YieldRange>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PriceRange {
    pub min: Decimal,
    pub max: Decimal,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct YieldRange {
    pub min: Decimal,
    pub max: Decimal,
}

// =============================================================================
// DASHBOARD (Story 132.3)
// =============================================================================

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PricingDashboard {
    pub portfolio_summary: PortfolioPricingSummary,
    pub market_trends: Vec<MarketTrendPoint>,
    pub units_with_recommendations: Vec<UnitRecommendationSummary>,
    pub vacancy_trends: Vec<VacancyTrendPoint>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PortfolioPricingSummary {
    pub total_units: i32,
    pub avg_rent: Decimal,
    pub market_avg_rent: Decimal,
    pub portfolio_vs_market_pct: Decimal,
    pub units_below_market: i32,
    pub units_above_market: i32,
    pub potential_revenue_increase: Decimal,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct MarketTrendPoint {
    pub date: NaiveDate,
    pub avg_rent: Decimal,
    pub avg_price_per_sqm: Decimal,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct UnitRecommendationSummary {
    pub unit_id: Uuid,
    pub unit_name: String,
    pub current_rent: Decimal,
    pub recommended_rent: Decimal,
    pub difference_pct: Decimal,
    pub confidence_score: Decimal,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct VacancyTrendPoint {
    pub date: NaiveDate,
    pub vacancy_rate: Decimal,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct PricingDashboardQuery {
    pub region_id: Option<Uuid>,
    pub building_id: Option<Uuid>,
    pub property_type: Option<String>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

// =============================================================================
// EXPORT
// =============================================================================

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ExportPricingDataRequest {
    pub region_id: Option<Uuid>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub format: String, // csv, xlsx
}
