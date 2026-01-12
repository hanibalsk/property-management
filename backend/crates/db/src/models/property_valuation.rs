//! Property Valuation models for Epic 138: Automated Property Valuation Model (AVM).
//!
//! This module provides types for property valuation, including AVM algorithms,
//! comparable sales analysis, market analytics, and automated appraisal tools.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// Enums
// ============================================================================

/// Type of valuation model/algorithm
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "valuation_model_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ValuationModelType {
    /// Hedonic pricing model
    Hedonic,
    /// Comparable sales approach
    #[default]
    ComparableSales,
    /// Income capitalization approach
    IncomeApproach,
    /// Replacement cost approach
    CostApproach,
    /// Hybrid/ensemble model
    Hybrid,
    /// ML-based model
    MachineLearning,
    /// Deep learning model
    NeuralNetwork,
}

/// Status of a valuation
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "valuation_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ValuationStatus {
    #[default]
    Draft,
    PendingReview,
    InProgress,
    Completed,
    Approved,
    Rejected,
    Expired,
}

/// Confidence level for valuations
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "valuation_confidence", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ValuationConfidence {
    VeryLow,
    Low,
    #[default]
    Medium,
    High,
    VeryHigh,
}

/// Property condition rating
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "property_condition", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PropertyCondition {
    Excellent,
    VeryGood,
    Good,
    #[default]
    Average,
    Fair,
    Poor,
    VeryPoor,
}

/// Market trend direction
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "market_trend", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MarketTrend {
    StrongGrowth,
    ModerateGrowth,
    #[default]
    Stable,
    ModerateDecline,
    StrongDecline,
}

/// Adjustment type for comparable analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "adjustment_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AdjustmentType {
    Location,
    Size,
    Age,
    Condition,
    Features,
    MarketTime,
    Financing,
    SaleType,
    LotSize,
    View,
    Basement,
    Garage,
    Pool,
    Renovation,
    Other,
}

// ============================================================================
// Valuation Model Entity
// ============================================================================

/// AVM model configuration
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PropertyValuationModel {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub model_type: ValuationModelType,
    pub model_config: serde_json::Value,
    pub feature_weights: Option<serde_json::Value>,
    pub r_squared: Option<Decimal>,
    pub mean_absolute_error: Option<Decimal>,
    pub mean_percentage_error: Option<Decimal>,
    pub version: i32,
    pub training_sample_size: Option<i32>,
    pub last_trained_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub is_default: bool,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create a new valuation model
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateValuationModel {
    pub name: String,
    pub description: Option<String>,
    pub model_type: ValuationModelType,
    #[serde(default)]
    pub model_config: serde_json::Value,
    pub feature_weights: Option<serde_json::Value>,
    #[serde(default)]
    pub is_default: bool,
}

/// Update a valuation model
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateValuationModel {
    pub name: Option<String>,
    pub description: Option<String>,
    pub model_config: Option<serde_json::Value>,
    pub feature_weights: Option<serde_json::Value>,
    pub r_squared: Option<f64>,
    pub mean_absolute_error: Option<f64>,
    pub mean_percentage_error: Option<f64>,
    pub training_sample_size: Option<i32>,
    pub is_active: Option<bool>,
    pub is_default: Option<bool>,
}

// ============================================================================
// Property Valuation Entity
// ============================================================================

/// Property valuation
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PropertyValuation {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub property_id: Uuid,
    pub building_id: Option<Uuid>,
    pub model_id: Option<Uuid>,
    pub valuation_date: NaiveDate,
    pub effective_date: NaiveDate,
    pub expiration_date: Option<NaiveDate>,
    pub status: ValuationStatus,
    pub estimated_value: Decimal,
    pub value_range_low: Option<Decimal>,
    pub value_range_high: Option<Decimal>,
    pub confidence_level: ValuationConfidence,
    pub confidence_score: Option<Decimal>,
    pub price_per_sqm: Option<Decimal>,
    pub price_per_sqft: Option<Decimal>,
    pub market_value: Option<Decimal>,
    pub insurance_value: Option<Decimal>,
    pub tax_assessed_value: Option<Decimal>,
    pub replacement_cost: Option<Decimal>,
    pub land_value: Option<Decimal>,
    pub improvement_value: Option<Decimal>,
    pub property_condition: Option<PropertyCondition>,
    pub effective_age: Option<i32>,
    pub remaining_economic_life: Option<i32>,
    pub market_trend: Option<MarketTrend>,
    pub days_on_market_estimate: Option<i32>,
    pub absorption_rate: Option<Decimal>,
    pub property_data: Option<serde_json::Value>,
    pub market_data: Option<serde_json::Value>,
    pub methodology_notes: Option<String>,
    pub assumptions: Option<String>,
    pub limiting_conditions: Option<String>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub review_notes: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create a new property valuation
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePropertyValuation {
    pub property_id: Uuid,
    pub building_id: Option<Uuid>,
    pub model_id: Option<Uuid>,
    pub valuation_date: NaiveDate,
    pub effective_date: NaiveDate,
    pub expiration_date: Option<NaiveDate>,
    pub estimated_value: f64,
    pub value_range_low: Option<f64>,
    pub value_range_high: Option<f64>,
    #[serde(default)]
    pub confidence_level: ValuationConfidence,
    pub confidence_score: Option<f64>,
    pub price_per_sqm: Option<f64>,
    pub property_condition: Option<PropertyCondition>,
    pub effective_age: Option<i32>,
    pub market_trend: Option<MarketTrend>,
    pub methodology_notes: Option<String>,
    pub assumptions: Option<String>,
}

/// Update a property valuation
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdatePropertyValuation {
    pub status: Option<ValuationStatus>,
    pub estimated_value: Option<f64>,
    pub value_range_low: Option<f64>,
    pub value_range_high: Option<f64>,
    pub confidence_level: Option<ValuationConfidence>,
    pub confidence_score: Option<f64>,
    pub market_value: Option<f64>,
    pub insurance_value: Option<f64>,
    pub tax_assessed_value: Option<f64>,
    pub property_condition: Option<PropertyCondition>,
    pub market_trend: Option<MarketTrend>,
    pub methodology_notes: Option<String>,
    pub assumptions: Option<String>,
    pub limiting_conditions: Option<String>,
    pub review_notes: Option<String>,
}

// ============================================================================
// Comparable Sales Entity
// ============================================================================

/// Comparable sale for valuation
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ValuationComparable {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub valuation_id: Option<Uuid>,
    pub comparable_property_id: Option<Uuid>,
    pub external_address: Option<String>,
    pub external_city: Option<String>,
    pub external_postal_code: Option<String>,
    pub external_country: Option<String>,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub sale_date: NaiveDate,
    pub sale_price: Decimal,
    pub sale_price_per_sqm: Option<Decimal>,
    pub property_type: Option<String>,
    pub total_area_sqm: Option<Decimal>,
    pub lot_size_sqm: Option<Decimal>,
    pub year_built: Option<i32>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<Decimal>,
    pub floors: Option<i32>,
    pub parking_spaces: Option<i32>,
    pub condition: Option<PropertyCondition>,
    pub distance_km: Option<Decimal>,
    pub similarity_score: Option<Decimal>,
    pub weight: Option<Decimal>,
    pub gross_adjustment_percent: Option<Decimal>,
    pub net_adjustment_percent: Option<Decimal>,
    pub adjusted_price: Option<Decimal>,
    pub data_source: Option<String>,
    pub source_reference: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create a comparable sale
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateComparable {
    pub valuation_id: Option<Uuid>,
    pub comparable_property_id: Option<Uuid>,
    pub external_address: Option<String>,
    pub external_city: Option<String>,
    pub external_postal_code: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub sale_date: NaiveDate,
    pub sale_price: f64,
    pub property_type: Option<String>,
    pub total_area_sqm: Option<f64>,
    pub lot_size_sqm: Option<f64>,
    pub year_built: Option<i32>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<f64>,
    pub condition: Option<PropertyCondition>,
    pub distance_km: Option<f64>,
    pub data_source: Option<String>,
}

/// Update a comparable sale
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateComparable {
    pub sale_price: Option<f64>,
    pub sale_price_per_sqm: Option<f64>,
    pub similarity_score: Option<f64>,
    pub weight: Option<f64>,
    pub gross_adjustment_percent: Option<f64>,
    pub net_adjustment_percent: Option<f64>,
    pub adjusted_price: Option<f64>,
    pub is_verified: Option<bool>,
}

// ============================================================================
// Comparable Adjustment Entity
// ============================================================================

/// Adjustment applied to a comparable sale
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ComparableAdjustment {
    pub id: Uuid,
    pub comparable_id: Uuid,
    pub adjustment_type: AdjustmentType,
    pub adjustment_name: String,
    pub subject_value: Option<String>,
    pub comparable_value: Option<String>,
    pub adjustment_amount: Decimal,
    pub adjustment_percent: Option<Decimal>,
    pub justification: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Create a comparable adjustment
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateAdjustment {
    pub comparable_id: Uuid,
    pub adjustment_type: AdjustmentType,
    pub adjustment_name: String,
    pub subject_value: Option<String>,
    pub comparable_value: Option<String>,
    pub adjustment_amount: f64,
    pub adjustment_percent: Option<f64>,
    pub justification: Option<String>,
}

// ============================================================================
// Market Data Entity
// ============================================================================

/// Regional/neighborhood market statistics
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ValuationMarketData {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub region: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub neighborhood: Option<String>,
    pub postal_code: Option<String>,
    pub property_type: Option<String>,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub median_price: Option<Decimal>,
    pub average_price: Option<Decimal>,
    pub price_per_sqm_median: Option<Decimal>,
    pub price_per_sqm_average: Option<Decimal>,
    pub price_per_sqm_min: Option<Decimal>,
    pub price_per_sqm_max: Option<Decimal>,
    pub total_transactions: Option<i32>,
    pub total_volume: Option<Decimal>,
    pub avg_days_on_market: Option<Decimal>,
    pub median_days_on_market: Option<i32>,
    pub price_change_percent: Option<Decimal>,
    pub price_change_yoy: Option<Decimal>,
    pub price_change_mom: Option<Decimal>,
    pub market_trend: Option<MarketTrend>,
    pub active_listings: Option<i32>,
    pub new_listings: Option<i32>,
    pub pending_sales: Option<i32>,
    pub months_of_supply: Option<Decimal>,
    pub absorption_rate: Option<Decimal>,
    pub list_to_sale_ratio: Option<Decimal>,
    pub avg_rent_per_sqm: Option<Decimal>,
    pub rent_yield_percent: Option<Decimal>,
    pub vacancy_rate: Option<Decimal>,
    pub data_source: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create market data entry
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateMarketData {
    pub region: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub neighborhood: Option<String>,
    pub postal_code: Option<String>,
    pub property_type: Option<String>,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub median_price: Option<f64>,
    pub average_price: Option<f64>,
    pub price_per_sqm_median: Option<f64>,
    pub price_per_sqm_average: Option<f64>,
    pub total_transactions: Option<i32>,
    pub price_change_percent: Option<f64>,
    pub market_trend: Option<MarketTrend>,
    pub data_source: Option<String>,
}

/// Update market data entry
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateMarketData {
    pub median_price: Option<f64>,
    pub average_price: Option<f64>,
    pub price_per_sqm_median: Option<f64>,
    pub price_per_sqm_average: Option<f64>,
    pub total_transactions: Option<i32>,
    pub price_change_percent: Option<f64>,
    pub price_change_yoy: Option<f64>,
    pub market_trend: Option<MarketTrend>,
    pub active_listings: Option<i32>,
    pub months_of_supply: Option<f64>,
    pub is_verified: Option<bool>,
}

// ============================================================================
// Value History Entity
// ============================================================================

/// Historical property value record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PropertyValueHistory {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub property_id: Uuid,
    pub valuation_id: Option<Uuid>,
    pub record_date: NaiveDate,
    pub estimated_value: Decimal,
    pub price_per_sqm: Option<Decimal>,
    pub confidence_level: Option<ValuationConfidence>,
    pub previous_value: Option<Decimal>,
    pub value_change: Option<Decimal>,
    pub value_change_percent: Option<Decimal>,
    pub value_source: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Create value history entry
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateValueHistory {
    pub property_id: Uuid,
    pub valuation_id: Option<Uuid>,
    pub record_date: NaiveDate,
    pub estimated_value: f64,
    pub price_per_sqm: Option<f64>,
    pub confidence_level: Option<ValuationConfidence>,
    pub previous_value: Option<f64>,
    pub value_source: Option<String>,
}

// ============================================================================
// Valuation Request Entity
// ============================================================================

/// User-initiated valuation request
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ValuationRequest {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub property_id: Uuid,
    pub request_type: String,
    pub purpose: Option<String>,
    pub priority: i32,
    pub status: ValuationStatus,
    pub requested_date: NaiveDate,
    pub due_date: Option<NaiveDate>,
    pub completed_date: Option<NaiveDate>,
    pub valuation_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub assigned_at: Option<DateTime<Utc>>,
    pub requester_notes: Option<String>,
    pub internal_notes: Option<String>,
    pub requested_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create a valuation request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateValuationRequest {
    pub property_id: Uuid,
    #[serde(default = "default_request_type")]
    pub request_type: String,
    pub purpose: Option<String>,
    #[serde(default = "default_priority")]
    pub priority: i32,
    pub due_date: Option<NaiveDate>,
    pub requester_notes: Option<String>,
}

fn default_request_type() -> String {
    "standard".to_string()
}

fn default_priority() -> i32 {
    5
}

/// Update a valuation request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateValuationRequest {
    pub status: Option<ValuationStatus>,
    pub priority: Option<i32>,
    pub due_date: Option<NaiveDate>,
    pub assigned_to: Option<Uuid>,
    pub valuation_id: Option<Uuid>,
    pub internal_notes: Option<String>,
}

// ============================================================================
// Property Features Entity
// ============================================================================

/// Property features for hedonic model inputs
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PropertyValuationFeatures {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub property_id: Uuid,
    pub recorded_date: NaiveDate,
    pub total_area_sqm: Option<Decimal>,
    pub living_area_sqm: Option<Decimal>,
    pub lot_size_sqm: Option<Decimal>,
    pub year_built: Option<i32>,
    pub year_renovated: Option<i32>,
    pub floors: Option<i32>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<Decimal>,
    pub half_baths: Option<i32>,
    pub construction_quality: Option<i32>,
    pub interior_quality: Option<i32>,
    pub exterior_quality: Option<i32>,
    pub features: Option<serde_json::Value>,
    pub has_garage: bool,
    pub garage_spaces: i32,
    pub has_pool: bool,
    pub has_basement: bool,
    pub basement_finished: bool,
    pub has_fireplace: bool,
    pub has_central_ac: bool,
    pub has_central_heat: bool,
    pub walk_score: Option<i32>,
    pub transit_score: Option<i32>,
    pub bike_score: Option<i32>,
    pub school_rating: Option<Decimal>,
    pub view_quality: Option<i32>,
    pub lot_shape: Option<String>,
    pub topography: Option<String>,
    pub condition: Option<PropertyCondition>,
    pub condition_score: Option<i32>,
    pub assessed_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create property features
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePropertyFeatures {
    pub property_id: Uuid,
    pub total_area_sqm: Option<f64>,
    pub living_area_sqm: Option<f64>,
    pub lot_size_sqm: Option<f64>,
    pub year_built: Option<i32>,
    pub year_renovated: Option<i32>,
    pub floors: Option<i32>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<f64>,
    pub construction_quality: Option<i32>,
    pub interior_quality: Option<i32>,
    pub exterior_quality: Option<i32>,
    pub features: Option<serde_json::Value>,
    #[serde(default)]
    pub has_garage: bool,
    #[serde(default)]
    pub garage_spaces: i32,
    #[serde(default)]
    pub has_pool: bool,
    #[serde(default)]
    pub has_basement: bool,
    pub condition: Option<PropertyCondition>,
    pub condition_score: Option<i32>,
}

/// Update property features
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdatePropertyFeatures {
    pub total_area_sqm: Option<f64>,
    pub living_area_sqm: Option<f64>,
    pub year_renovated: Option<i32>,
    pub construction_quality: Option<i32>,
    pub interior_quality: Option<i32>,
    pub exterior_quality: Option<i32>,
    pub features: Option<serde_json::Value>,
    pub has_garage: Option<bool>,
    pub garage_spaces: Option<i32>,
    pub has_pool: Option<bool>,
    pub condition: Option<PropertyCondition>,
    pub condition_score: Option<i32>,
}

// ============================================================================
// Valuation Report Entity
// ============================================================================

/// Generated valuation report
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ValuationReport {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub valuation_id: Uuid,
    pub report_type: String,
    pub report_number: Option<String>,
    pub title: Option<String>,
    pub executive_summary: Option<String>,
    pub full_report_content: Option<serde_json::Value>,
    pub file_path: Option<String>,
    pub file_name: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub file_mime_type: Option<String>,
    pub is_draft: bool,
    pub is_signed: bool,
    pub signed_by: Option<Uuid>,
    pub signed_at: Option<DateTime<Utc>>,
    pub generated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create a valuation report
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateValuationReport {
    pub valuation_id: Uuid,
    #[serde(default = "default_report_type")]
    pub report_type: String,
    pub report_number: Option<String>,
    pub title: Option<String>,
    pub executive_summary: Option<String>,
    pub full_report_content: Option<serde_json::Value>,
}

fn default_report_type() -> String {
    "summary".to_string()
}

/// Update a valuation report
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateValuationReport {
    pub title: Option<String>,
    pub executive_summary: Option<String>,
    pub full_report_content: Option<serde_json::Value>,
    pub file_path: Option<String>,
    pub file_name: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub is_draft: Option<bool>,
}

// ============================================================================
// Audit Log Entity
// ============================================================================

/// Audit log for valuation activities
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ValuationAuditLog {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub valuation_id: Option<Uuid>,
    pub model_id: Option<Uuid>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub reason: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub performed_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Create an audit log entry
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateValuationAuditLog {
    pub valuation_id: Option<Uuid>,
    pub model_id: Option<Uuid>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub reason: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

// ============================================================================
// Dashboard and Summary Types
// ============================================================================

/// Valuation dashboard summary
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ValuationDashboard {
    pub total_valuations: i64,
    pub pending_valuations: i64,
    pub completed_valuations: i64,
    pub average_confidence: Option<f64>,
    pub total_portfolio_value: Option<f64>,
    pub valuations_this_month: i64,
    pub expiring_soon: i64,
    pub pending_requests: i64,
}

/// Property value trend
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PropertyValueTrend {
    pub property_id: Uuid,
    pub property_name: Option<String>,
    pub current_value: Option<f64>,
    pub previous_value: Option<f64>,
    pub change_percent: Option<f64>,
    pub trend: MarketTrend,
    pub history: Vec<ValueHistoryPoint>,
}

/// Value history data point
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ValueHistoryPoint {
    pub date: NaiveDate,
    pub value: f64,
    pub source: Option<String>,
}

/// Market analysis summary
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MarketAnalysisSummary {
    pub location: String,
    pub property_type: Option<String>,
    pub median_price: Option<f64>,
    pub price_per_sqm: Option<f64>,
    pub trend: MarketTrend,
    pub price_change_yoy: Option<f64>,
    pub days_on_market: Option<i32>,
    pub total_transactions: Option<i32>,
}

/// Valuation with related data
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ValuationWithDetails {
    #[serde(flatten)]
    pub valuation: PropertyValuation,
    pub comparables: Vec<ValuationComparable>,
    pub adjustments: Vec<ComparableAdjustment>,
    pub model: Option<PropertyValuationModel>,
}
