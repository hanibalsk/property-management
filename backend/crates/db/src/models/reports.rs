//! Report models (Epic 55: Advanced Reporting & Analytics).
//!
//! Types for report generation, analytics, and trend analysis.

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// Common Report Types
// ============================================================================

/// Monthly count data point for trend analysis (with year).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct ReportMonthlyCount {
    pub year: i32,
    pub month: i32,
    pub count: i64,
}

/// Monthly average data point for trend analysis.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct MonthlyAverage {
    pub year: i32,
    pub month: i32,
    pub average: f64,
}

/// Date range for reports.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DateRange {
    pub from: NaiveDate,
    pub to: NaiveDate,
}

// ============================================================================
// Story 55.1: Fault Statistics Report Types
// ============================================================================

/// Fault trends over time.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FaultTrends {
    pub monthly_counts: Vec<ReportMonthlyCount>,
    pub resolution_time_trend: Vec<MonthlyAverage>,
    pub category_trend: Vec<CategoryTrend>,
}

/// Category trend over time.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CategoryTrend {
    pub category: String,
    pub monthly_counts: Vec<ReportMonthlyCount>,
}

// ============================================================================
// Story 55.2: Voting Participation Report Types
// ============================================================================

/// Voting participation summary.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VotingParticipationSummary {
    pub total_votes: i64,
    pub votes_with_quorum: i64,
    pub votes_without_quorum: i64,
    pub average_participation_rate: f64,
    pub total_eligible_voters: i64,
    pub total_responses: i64,
}

/// Individual vote participation detail.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct VoteParticipationDetail {
    pub vote_id: Uuid,
    pub title: String,
    pub status: String,
    pub start_at: Option<String>,
    pub end_at: String,
    pub eligible_count: i64,
    pub response_count: i64,
    pub participation_rate: f64,
    pub quorum_required: Option<i32>,
    pub quorum_reached: bool,
}

// ============================================================================
// Story 55.3: Occupancy Report Types
// ============================================================================

/// Occupancy summary.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct OccupancySummary {
    pub total_units: i64,
    pub occupied_units: i64,
    pub vacant_units: i64,
    pub occupancy_rate: f64,
    pub total_person_months: i64,
    pub average_occupants_per_unit: f64,
}

/// Unit-level occupancy data.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UnitOccupancy {
    pub unit_id: Uuid,
    pub unit_designation: String,
    pub person_months: Vec<MonthlyPersonCount>,
    pub total_person_months: i64,
    pub average_occupants: f64,
}

/// Monthly person count.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct MonthlyPersonCount {
    pub year: i32,
    pub month: i32,
    pub count: i32,
}

/// Occupancy trends.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OccupancyTrends {
    pub monthly_total: Vec<ReportMonthlyCount>,
    pub year_over_year_comparison: Option<YearComparison>,
}

/// Year-over-year comparison.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct YearComparison {
    pub current_year: i32,
    pub previous_year: i32,
    pub current_total: i64,
    pub previous_total: i64,
    pub change_percentage: f64,
}

/// Occupancy report data from repository.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OccupancyReportData {
    pub summary: OccupancySummary,
    pub by_unit: Vec<UnitOccupancy>,
    pub monthly_totals: Vec<ReportMonthlyCount>,
}

// ============================================================================
// Story 55.4: Consumption Report Types
// ============================================================================

/// Consumption summary.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct ConsumptionSummary {
    pub total_consumption: Decimal,
    pub total_cost: Decimal,
    pub meter_count: i64,
    pub average_consumption_per_unit: Decimal,
}

/// Consumption by utility type.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UtilityTypeConsumption {
    pub utility_type: String,
    pub total_consumption: Decimal,
    pub unit: String,
    pub total_cost: Decimal,
    pub meter_count: i64,
    pub monthly_data: Vec<MonthlyConsumption>,
}

/// Monthly consumption data point.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct MonthlyConsumption {
    pub year: i32,
    pub month: i32,
    pub consumption: Decimal,
    pub cost: Decimal,
}

/// Unit-level consumption.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UnitConsumption {
    pub unit_id: Uuid,
    pub unit_designation: String,
    pub utility_type: String,
    pub total_consumption: Decimal,
    pub average_monthly: Decimal,
    pub is_above_average: bool,
    pub deviation_percentage: f64,
}

/// Consumption anomaly detected.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct ConsumptionAnomaly {
    pub unit_id: Uuid,
    pub unit_designation: String,
    pub meter_id: Uuid,
    pub utility_type: String,
    pub reading_date: NaiveDate,
    pub consumption: Decimal,
    pub expected_consumption: Decimal,
    pub deviation_percentage: f64,
    pub severity: String,
}

/// Consumption report data from repository.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConsumptionReportData {
    pub summary: ConsumptionSummary,
    pub by_utility_type: Vec<UtilityTypeConsumption>,
    pub by_unit: Vec<UnitConsumption>,
}

// ============================================================================
// Story 55.5: Export Report Types
// ============================================================================

/// Export report request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExportReportRequest {
    pub report_type: String,
    pub format: String,
    pub building_id: Option<Uuid>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub params: Option<serde_json::Value>,
}

/// Export report response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExportReportResponse {
    pub download_url: String,
    pub format: String,
    pub expires_at: String,
}
