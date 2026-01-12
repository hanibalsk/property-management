//! Portfolio Analytics models for Epic 140: Multi-Property Portfolio Analytics.
//!
//! Provides cross-property analytics, benchmarking, and trend analysis.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// =============================================================================
// ENUMS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema, PartialEq)]
#[sqlx(type_name = "benchmark_category", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkCategory {
    Occupancy,
    RentCollection,
    MaintenanceCosts,
    TenantSatisfaction,
    EnergyEfficiency,
    PropertyValue,
    Noi,
    CapRate,
}

impl AsRef<str> for BenchmarkCategory {
    fn as_ref(&self) -> &str {
        match self {
            Self::Occupancy => "occupancy",
            Self::RentCollection => "rent_collection",
            Self::MaintenanceCosts => "maintenance_costs",
            Self::TenantSatisfaction => "tenant_satisfaction",
            Self::EnergyEfficiency => "energy_efficiency",
            Self::PropertyValue => "property_value",
            Self::Noi => "noi",
            Self::CapRate => "cap_rate",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema, PartialEq)]
#[sqlx(type_name = "aggregation_period", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AggregationPeriod {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annual,
}

impl AsRef<str> for AggregationPeriod {
    fn as_ref(&self) -> &str {
        match self {
            Self::Daily => "daily",
            Self::Weekly => "weekly",
            Self::Monthly => "monthly",
            Self::Quarterly => "quarterly",
            Self::Annual => "annual",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema, PartialEq)]
#[sqlx(type_name = "comparison_scope", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ComparisonScope {
    Portfolio,
    Region,
    PropertyType,
    Market,
    Industry,
}

impl AsRef<str> for ComparisonScope {
    fn as_ref(&self) -> &str {
        match self {
            Self::Portfolio => "portfolio",
            Self::Region => "region",
            Self::PropertyType => "property_type",
            Self::Market => "market",
            Self::Industry => "industry",
        }
    }
}

// =============================================================================
// PORTFOLIO BENCHMARKS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PortfolioBenchmark {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: BenchmarkCategory,
    pub comparison_scope: ComparisonScope,
    pub target_value: Option<Decimal>,
    pub warning_threshold: Option<Decimal>,
    pub critical_threshold: Option<Decimal>,
    pub is_higher_better: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreatePortfolioBenchmark {
    pub name: String,
    pub description: Option<String>,
    pub category: BenchmarkCategory,
    pub comparison_scope: Option<ComparisonScope>,
    pub target_value: Option<Decimal>,
    pub warning_threshold: Option<Decimal>,
    pub critical_threshold: Option<Decimal>,
    pub is_higher_better: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdatePortfolioBenchmark {
    pub name: Option<String>,
    pub description: Option<String>,
    pub target_value: Option<Decimal>,
    pub warning_threshold: Option<Decimal>,
    pub critical_threshold: Option<Decimal>,
    pub is_higher_better: Option<bool>,
    pub is_active: Option<bool>,
}

// =============================================================================
// PROPERTY PERFORMANCE METRICS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PropertyPerformanceMetrics {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Uuid,
    pub period_type: AggregationPeriod,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,

    // Occupancy
    pub occupancy_rate: Option<Decimal>,
    pub total_units: Option<i32>,
    pub occupied_units: Option<i32>,
    pub vacant_units: Option<i32>,

    // Financial
    pub gross_potential_rent: Option<Decimal>,
    pub collected_rent: Option<Decimal>,
    pub rent_collection_rate: Option<Decimal>,
    pub gross_revenue: Option<Decimal>,
    pub operating_expenses: Option<Decimal>,
    pub net_operating_income: Option<Decimal>,
    pub noi_margin: Option<Decimal>,

    // Per-unit
    pub revenue_per_unit: Option<Decimal>,
    pub expense_per_unit: Option<Decimal>,
    pub noi_per_unit: Option<Decimal>,

    // Maintenance
    pub maintenance_requests: Option<i32>,
    pub completed_requests: Option<i32>,
    pub avg_resolution_time_hours: Option<Decimal>,
    pub maintenance_cost: Option<Decimal>,

    // Tenant
    pub new_leases: Option<i32>,
    pub renewals: Option<i32>,
    pub move_outs: Option<i32>,
    pub tenant_turnover_rate: Option<Decimal>,
    pub avg_tenant_tenure_months: Option<Decimal>,

    // Energy
    pub energy_consumption_kwh: Option<Decimal>,
    pub water_consumption_liters: Option<Decimal>,
    pub energy_cost: Option<Decimal>,
    pub energy_cost_per_sqm: Option<Decimal>,

    // Valuation
    pub property_value: Option<Decimal>,
    pub cap_rate: Option<Decimal>,
    pub price_per_sqm: Option<Decimal>,

    pub calculated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreatePropertyMetrics {
    pub building_id: Uuid,
    pub period_type: AggregationPeriod,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,

    // Occupancy
    pub occupancy_rate: Option<Decimal>,
    pub total_units: Option<i32>,
    pub occupied_units: Option<i32>,

    // Financial
    pub gross_potential_rent: Option<Decimal>,
    pub collected_rent: Option<Decimal>,
    pub gross_revenue: Option<Decimal>,
    pub operating_expenses: Option<Decimal>,

    // Maintenance
    pub maintenance_requests: Option<i32>,
    pub completed_requests: Option<i32>,
    pub avg_resolution_time_hours: Option<Decimal>,
    pub maintenance_cost: Option<Decimal>,

    // Tenant
    pub new_leases: Option<i32>,
    pub renewals: Option<i32>,
    pub move_outs: Option<i32>,

    // Energy
    pub energy_consumption_kwh: Option<Decimal>,
    pub water_consumption_liters: Option<Decimal>,
    pub energy_cost: Option<Decimal>,

    // Valuation
    pub property_value: Option<Decimal>,
}

// =============================================================================
// PORTFOLIO AGGREGATED METRICS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PortfolioAggregatedMetrics {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub period_type: AggregationPeriod,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,

    // Summary
    pub total_properties: Option<i32>,
    pub total_units: Option<i32>,
    pub total_sqm: Option<Decimal>,

    // Averages
    pub avg_occupancy_rate: Option<Decimal>,
    pub avg_rent_collection_rate: Option<Decimal>,
    pub avg_noi_margin: Option<Decimal>,
    pub avg_cap_rate: Option<Decimal>,
    pub avg_tenant_turnover_rate: Option<Decimal>,

    // Totals
    pub total_gross_revenue: Option<Decimal>,
    pub total_operating_expenses: Option<Decimal>,
    pub total_noi: Option<Decimal>,
    pub total_property_value: Option<Decimal>,

    // Per-unit
    pub portfolio_revenue_per_unit: Option<Decimal>,
    pub portfolio_expense_per_unit: Option<Decimal>,
    pub portfolio_noi_per_unit: Option<Decimal>,

    // Trends
    pub occupancy_trend: Option<Decimal>,
    pub revenue_trend: Option<Decimal>,
    pub noi_trend: Option<Decimal>,

    pub calculated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// =============================================================================
// PROPERTY COMPARISONS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PropertyComparison {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Uuid,
    pub benchmark_id: Uuid,
    pub period_type: AggregationPeriod,
    pub period_start: NaiveDate,

    pub actual_value: Option<Decimal>,
    pub target_value: Option<Decimal>,
    pub portfolio_avg: Option<Decimal>,

    pub rank_in_portfolio: Option<i32>,
    pub percentile: Option<Decimal>,

    pub variance_from_target: Option<Decimal>,
    pub variance_percentage: Option<Decimal>,
    pub status: Option<String>,

    pub created_at: DateTime<Utc>,
}

// =============================================================================
// PORTFOLIO TRENDS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PortfolioTrend {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub metric_name: String,
    pub category: BenchmarkCategory,

    pub trend_start: NaiveDate,
    pub trend_end: NaiveDate,
    pub period_count: i32,

    pub start_value: Option<Decimal>,
    pub end_value: Option<Decimal>,
    pub min_value: Option<Decimal>,
    pub max_value: Option<Decimal>,
    pub avg_value: Option<Decimal>,

    pub trend_direction: Option<String>,
    pub trend_percentage: Option<Decimal>,
    pub slope: Option<Decimal>,
    pub r_squared: Option<Decimal>,

    pub has_seasonality: Option<bool>,
    pub seasonal_pattern: Option<serde_json::Value>,

    pub forecast_next_period: Option<Decimal>,
    pub forecast_confidence: Option<Decimal>,

    pub data_points: Option<serde_json::Value>,

    pub calculated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct TrendQuery {
    pub metric_name: Option<String>,
    pub category: Option<BenchmarkCategory>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

// =============================================================================
// PORTFOLIO ALERT RULES
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PortfolioAlertRule {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub benchmark_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub metric_name: String,

    pub condition_type: String,
    pub threshold_value: Option<Decimal>,
    pub threshold_operator: Option<String>,

    pub trend_direction: Option<String>,
    pub trend_periods: Option<i32>,
    pub trend_percentage: Option<Decimal>,

    pub severity: String,
    pub notification_channels: Option<serde_json::Value>,
    pub recipients: Option<serde_json::Value>,

    pub is_active: bool,
    pub last_triggered_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateAlertRule {
    pub benchmark_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub metric_name: String,
    pub condition_type: String,
    pub threshold_value: Option<Decimal>,
    pub threshold_operator: Option<String>,
    pub severity: Option<String>,
    pub notification_channels: Option<serde_json::Value>,
    pub recipients: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateAlertRule {
    pub name: Option<String>,
    pub description: Option<String>,
    pub threshold_value: Option<Decimal>,
    pub threshold_operator: Option<String>,
    pub severity: Option<String>,
    pub notification_channels: Option<serde_json::Value>,
    pub recipients: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

// =============================================================================
// PORTFOLIO ALERTS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PortfolioAlert {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub rule_id: Option<Uuid>,
    pub building_id: Option<Uuid>,

    pub alert_type: String,
    pub severity: String,
    pub title: String,
    pub message: Option<String>,

    pub metric_name: Option<String>,
    pub metric_value: Option<Decimal>,
    pub threshold_value: Option<Decimal>,

    pub status: String,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub acknowledged_by: Option<Uuid>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<Uuid>,

    pub triggered_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AlertQuery {
    pub status: Option<String>,
    pub severity: Option<String>,
    pub building_id: Option<Uuid>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

// =============================================================================
// SUMMARY & DASHBOARD TYPES
// =============================================================================

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PortfolioOverview {
    pub total_properties: i32,
    pub total_units: i32,
    pub total_value: Decimal,
    pub avg_occupancy: Decimal,
    pub avg_noi_margin: Decimal,
    pub total_noi: Decimal,
    pub active_alerts: i32,
    pub top_performers: Vec<PropertyRanking>,
    pub underperformers: Vec<PropertyRanking>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PropertyRanking {
    pub building_id: Uuid,
    pub building_name: String,
    pub metric_name: String,
    pub value: Decimal,
    pub rank: i32,
    pub trend: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct MetricsComparison {
    pub metric_name: String,
    pub properties: Vec<PropertyMetricValue>,
    pub portfolio_avg: Decimal,
    pub benchmark_target: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PropertyMetricValue {
    pub building_id: Uuid,
    pub building_name: String,
    pub value: Decimal,
    pub variance_from_avg: Decimal,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TrendAnalysis {
    pub metric_name: String,
    pub category: BenchmarkCategory,
    pub direction: String,
    pub change_percentage: Decimal,
    pub forecast: Option<Decimal>,
    pub confidence: Option<Decimal>,
    pub data_points: Vec<TrendDataPoint>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TrendDataPoint {
    pub date: NaiveDate,
    pub value: Decimal,
}
