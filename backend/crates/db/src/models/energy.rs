//! Energy & Sustainability Tracking models (Epic 65).
//!
//! Provides types for energy performance certificates, carbon emissions,
//! sustainability scores, and utility benchmarking.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ============================================================================
// ENUMS
// ============================================================================

/// Energy performance rating (A-G scale).
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "energy_rating", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EnergyRating {
    A,
    B,
    C,
    #[default]
    D,
    E,
    F,
    G,
}

/// Carbon emission source type.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "emission_source_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EmissionSourceType {
    #[default]
    Electricity,
    Gas,
    Heating,
    Cooling,
    Water,
    Waste,
    Transport,
    Other,
}

/// Heating type for sustainability scoring.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "heating_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum HeatingType {
    #[default]
    Gas,
    Electric,
    HeatPump,
    DistrictHeating,
    Oil,
    Biomass,
    Solar,
    Geothermal,
    None,
}

/// Insulation rating for sustainability scoring.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "insulation_rating", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum InsulationRating {
    Excellent,
    Good,
    #[default]
    Average,
    Poor,
    None,
}

/// Benchmark metric type.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "benchmark_metric_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkMetricType {
    #[default]
    ElectricityPerSqm,
    GasPerSqm,
    WaterPerPerson,
    CarbonPerSqm,
    TotalEnergy,
    RenewablePercentage,
}

/// Alert severity for benchmarking.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "benchmark_alert_severity", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkAlertSeverity {
    #[default]
    Info,
    Warning,
    Critical,
}

// ============================================================================
// ENERGY PERFORMANCE CERTIFICATES (Story 65.1)
// ============================================================================

/// Energy Performance Certificate entity.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct EnergyPerformanceCertificate {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub unit_id: Uuid,
    pub rating: EnergyRating,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_energy_kwh: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_co2_kg: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_energy_kwh_per_sqm: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Uuid>,
}

/// Request to create an Energy Performance Certificate.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateEnergyPerformanceCertificate {
    pub unit_id: Uuid,
    pub rating: EnergyRating,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_energy_kwh: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_co2_kg: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_energy_kwh_per_sqm: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Request to update an Energy Performance Certificate.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateEnergyPerformanceCertificate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<EnergyRating>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_energy_kwh: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_co2_kg: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_energy_kwh_per_sqm: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// EPC summary for building overview.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct EpcSummary {
    pub total_units: i64,
    pub units_with_epc: i64,
    pub rating_distribution: Vec<RatingCount>,
    pub expiring_soon: i64,
    pub expired: i64,
}

/// Count per rating.
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct RatingCount {
    pub rating: EnergyRating,
    pub count: i64,
}

// ============================================================================
// CARBON EMISSIONS (Story 65.2)
// ============================================================================

/// Carbon emission record.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct CarbonEmission {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Uuid,
    pub year: i32,
    pub month: i32,
    pub source_type: EmissionSourceType,
    pub co2_kg: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_kwh: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Uuid>,
}

/// Request to record carbon emission.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateCarbonEmission {
    pub building_id: Uuid,
    pub year: i32,
    pub month: i32,
    pub source_type: EmissionSourceType,
    pub co2_kg: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_kwh: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Carbon dashboard response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CarbonDashboard {
    pub building_id: Uuid,
    pub building_name: String,
    pub total_co2_kg: Decimal,
    pub total_co2_kg_previous_year: Decimal,
    pub change_percentage: Decimal,
    pub monthly_emissions: Vec<MonthlyEmission>,
    pub emissions_by_source: Vec<SourceEmission>,
    pub co2_per_sqm: Decimal,
    pub target_co2_kg: Option<Decimal>,
    pub on_track: bool,
}

/// Monthly emission data point.
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct MonthlyEmission {
    pub year: i32,
    pub month: i32,
    pub co2_kg: Decimal,
}

/// Emission by source type.
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct SourceEmission {
    pub source_type: EmissionSourceType,
    pub co2_kg: Decimal,
    pub percentage: Decimal,
}

/// Carbon export request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CarbonExportRequest {
    pub year: i32,
    #[serde(default)]
    pub include_monthly_breakdown: bool,
    #[serde(default)]
    pub include_source_breakdown: bool,
}

/// Carbon target for a building.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct CarbonTarget {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Uuid,
    pub year: i32,
    pub target_co2_kg: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_co2_kg: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Request to set carbon target.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateCarbonTarget {
    pub building_id: Uuid,
    pub year: i32,
    pub target_co2_kg: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_co2_kg: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

// ============================================================================
// SUSTAINABILITY SCORES (Story 65.3)
// ============================================================================

/// Sustainability score for a listing.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct SustainabilityScore {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub listing_id: Uuid,
    pub score: i32, // 1-100
    pub has_solar: bool,
    pub has_heat_pump: bool,
    pub insulation_rating: InsulationRating,
    pub heating_type: HeatingType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_ev_charging: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_rainwater_harvesting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_smart_thermostat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_led_lighting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_double_glazing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewable_energy_percentage: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_energy_cost_estimate: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_rating: Option<EnergyRating>,
    pub calculated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Request to create/update sustainability score.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateSustainabilityScore {
    pub listing_id: Uuid,
    #[serde(default)]
    pub has_solar: bool,
    #[serde(default)]
    pub has_heat_pump: bool,
    #[serde(default)]
    pub insulation_rating: InsulationRating,
    #[serde(default)]
    pub heating_type: HeatingType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_ev_charging: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_rainwater_harvesting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_smart_thermostat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_led_lighting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_double_glazing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewable_energy_percentage: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_energy_cost_estimate: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_rating: Option<EnergyRating>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Update sustainability score request.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateSustainabilityScore {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_solar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_heat_pump: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insulation_rating: Option<InsulationRating>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heating_type: Option<HeatingType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_ev_charging: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_rainwater_harvesting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_smart_thermostat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_led_lighting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_double_glazing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewable_energy_percentage: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_energy_cost_estimate: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_rating: Option<EnergyRating>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Sustainability features filter for listing search.
#[derive(Debug, Clone, Default, Deserialize, utoipa::IntoParams)]
pub struct SustainabilityFilter {
    /// Minimum sustainability score (1-100)
    pub min_score: Option<i32>,
    /// Has solar panels
    pub has_solar: Option<bool>,
    /// Has heat pump
    pub has_heat_pump: Option<bool>,
    /// Has EV charging
    pub has_ev_charging: Option<bool>,
    /// Minimum energy rating
    pub min_energy_rating: Option<EnergyRating>,
}

// ============================================================================
// UTILITY BENCHMARKING (Story 65.4)
// ============================================================================

/// Building benchmark record.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct BuildingBenchmark {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Uuid,
    pub metric_type: BenchmarkMetricType,
    pub value: Decimal,
    pub percentile: i32, // 0-100
    pub comparable_buildings_count: i32,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_type: Option<String>,
    pub calculated_at: DateTime<Utc>,
}

/// Request to calculate benchmark.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CalculateBenchmark {
    pub building_id: Uuid,
    pub metric_type: BenchmarkMetricType,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_type: Option<String>,
}

/// Benchmark alert.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct BenchmarkAlert {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Uuid,
    pub metric_type: BenchmarkMetricType,
    pub current_value: Decimal,
    pub benchmark_value: Decimal,
    pub deviation_percentage: Decimal,
    pub severity: BenchmarkAlertSeverity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub is_resolved: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Request to create benchmark alert.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateBenchmarkAlert {
    pub building_id: Uuid,
    pub metric_type: BenchmarkMetricType,
    pub current_value: Decimal,
    pub benchmark_value: Decimal,
    pub deviation_percentage: Decimal,
    pub severity: BenchmarkAlertSeverity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Benchmark dashboard response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct BenchmarkDashboard {
    pub building_id: Uuid,
    pub building_name: String,
    pub benchmarks: Vec<BuildingBenchmark>,
    pub alerts: Vec<BenchmarkAlert>,
    pub overall_percentile: i32,
    pub metrics_above_average: i32,
    pub metrics_below_average: i32,
    pub improvement_suggestions: Vec<String>,
}

/// Benchmark query parameters.
#[derive(Debug, Clone, Default, Deserialize, utoipa::IntoParams)]
pub struct BenchmarkQuery {
    /// Period start
    pub period_start: Option<NaiveDate>,
    /// Period end
    pub period_end: Option<NaiveDate>,
    /// Metric type filter
    pub metric_type: Option<BenchmarkMetricType>,
}

/// Benchmark alerts query parameters.
#[derive(Debug, Clone, Default, Deserialize, utoipa::IntoParams)]
pub struct BenchmarkAlertsQuery {
    /// Show only unresolved alerts
    #[serde(default = "default_true")]
    pub unresolved_only: bool,
    /// Severity filter
    pub severity: Option<BenchmarkAlertSeverity>,
    /// Limit
    #[serde(default = "default_limit")]
    pub limit: i64,
    /// Offset
    #[serde(default)]
    pub offset: i64,
}

fn default_true() -> bool {
    true
}

fn default_limit() -> i64 {
    50
}

// ============================================================================
// RESPONSES
// ============================================================================

/// List EPCs response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ListEpcsResponse {
    pub epcs: Vec<EnergyPerformanceCertificate>,
    pub total: i64,
}

/// List emissions response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ListEmissionsResponse {
    pub emissions: Vec<CarbonEmission>,
    pub total: i64,
}

/// List benchmarks response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ListBenchmarksResponse {
    pub benchmarks: Vec<BuildingBenchmark>,
    pub total: i64,
}

/// List benchmark alerts response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ListBenchmarkAlertsResponse {
    pub alerts: Vec<BenchmarkAlert>,
    pub total: i64,
}
