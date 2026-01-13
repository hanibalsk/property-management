//! Portfolio Performance Analytics models (Epic 144).
//! Enables property investors to track and analyze portfolio performance with ROI calculations
//! and market comparisons.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// =============================================================================
// ENUMS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema, sqlx::Type)]
#[sqlx(type_name = "financing_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum FinancingType {
    Cash,
    Mortgage,
    Commercial,
    PrivateLending,
    Partnership,
    Syndication,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema, sqlx::Type)]
#[sqlx(type_name = "transaction_type_portfolio", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PortfolioTransactionType {
    RentalIncome,
    OtherIncome,
    OperatingExpense,
    MortgagePayment,
    CapitalExpenditure,
    TaxPayment,
    Insurance,
    PropertyManagement,
    Maintenance,
    Utilities,
    VacancyCost,
    LeasingCost,
    LegalProfessional,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema, sqlx::Type)]
#[sqlx(type_name = "benchmark_source", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkSource {
    Industry,
    Regional,
    PropertyType,
    Custom,
    NcreifOdce,
    MsciIpd,
    NarreitIndex,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema, sqlx::Type)]
#[sqlx(type_name = "metric_period", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MetricPeriod {
    Monthly,
    Quarterly,
    Annual,
    Ytd,
    SinceInception,
}

// =============================================================================
// STORY 144.1: PORTFOLIO CONFIGURATION
// =============================================================================

/// Investment portfolio with enhanced configuration including acquisition and financing details.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PerformancePortfolio {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,

    // Investment goals
    pub target_return_pct: Option<Decimal>,
    pub target_exit_year: Option<i32>,
    pub investment_strategy: Option<String>,

    // Summary metrics (computed)
    pub total_invested: Option<Decimal>,
    pub total_current_value: Option<Decimal>,
    pub total_equity: Option<Decimal>,
    pub total_debt: Option<Decimal>,
    pub property_count: Option<i32>,

    pub currency: String,
    pub is_active: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreatePerformancePortfolio {
    pub name: String,
    pub description: Option<String>,
    pub target_return_pct: Option<Decimal>,
    pub target_exit_year: Option<i32>,
    pub investment_strategy: Option<String>,
    #[serde(default = "default_currency")]
    pub currency: String,
}

fn default_currency() -> String {
    "EUR".to_string()
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdatePerformancePortfolio {
    pub name: Option<String>,
    pub description: Option<String>,
    pub target_return_pct: Option<Decimal>,
    pub target_exit_year: Option<i32>,
    pub investment_strategy: Option<String>,
    pub is_active: Option<bool>,
}

/// Property within a portfolio with acquisition and financing details.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PortfolioProperty {
    pub id: Uuid,
    pub portfolio_id: Uuid,
    pub building_id: Uuid,
    pub property_name: Option<String>,

    // Acquisition details
    pub acquisition_date: NaiveDate,
    pub acquisition_price: Decimal,
    pub acquisition_costs: Option<Decimal>,
    pub total_acquisition_cost: Option<Decimal>,

    // Financing details
    pub financing_type: FinancingType,
    pub down_payment: Option<Decimal>,
    pub loan_amount: Option<Decimal>,
    pub interest_rate: Option<Decimal>,
    pub loan_term_years: Option<i32>,
    pub monthly_payment: Option<Decimal>,
    pub loan_start_date: Option<NaiveDate>,
    pub loan_maturity_date: Option<NaiveDate>,

    // Ownership
    pub ownership_percentage: Decimal,

    // Current values
    pub current_value: Option<Decimal>,
    pub current_loan_balance: Option<Decimal>,
    pub current_equity: Option<Decimal>,

    pub currency: String,
    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreatePortfolioProperty {
    pub building_id: Uuid,
    pub property_name: Option<String>,

    pub acquisition_date: NaiveDate,
    pub acquisition_price: Decimal,
    pub acquisition_costs: Option<Decimal>,

    #[serde(default = "default_financing_type")]
    pub financing_type: FinancingType,
    pub down_payment: Option<Decimal>,
    pub loan_amount: Option<Decimal>,
    pub interest_rate: Option<Decimal>,
    pub loan_term_years: Option<i32>,
    pub monthly_payment: Option<Decimal>,
    pub loan_start_date: Option<NaiveDate>,

    #[serde(default = "default_ownership")]
    pub ownership_percentage: Decimal,

    pub current_value: Option<Decimal>,
    #[serde(default = "default_currency")]
    pub currency: String,
    pub notes: Option<String>,
}

fn default_financing_type() -> FinancingType {
    FinancingType::Mortgage
}

fn default_ownership() -> Decimal {
    Decimal::from(100)
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdatePortfolioProperty {
    pub property_name: Option<String>,
    pub financing_type: Option<FinancingType>,
    pub loan_amount: Option<Decimal>,
    pub interest_rate: Option<Decimal>,
    pub monthly_payment: Option<Decimal>,
    pub current_value: Option<Decimal>,
    pub current_loan_balance: Option<Decimal>,
    pub ownership_percentage: Option<Decimal>,
    pub notes: Option<String>,
}

// =============================================================================
// STORY 144.2: INCOME & EXPENSE TRACKING
// =============================================================================

/// Financial transaction for a property in a portfolio.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PropertyTransaction {
    pub id: Uuid,
    pub portfolio_id: Uuid,
    pub property_id: Uuid,
    pub transaction_type: PortfolioTransactionType,
    pub category: Option<String>,

    pub amount: Decimal,
    pub currency: String,

    pub transaction_date: NaiveDate,
    pub period_start: Option<NaiveDate>,
    pub period_end: Option<NaiveDate>,

    pub description: Option<String>,
    pub vendor_name: Option<String>,
    pub reference_number: Option<String>,
    pub document_id: Option<Uuid>,

    pub is_recurring: bool,
    pub recurrence_frequency: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreatePropertyTransaction {
    pub property_id: Uuid,
    pub transaction_type: PortfolioTransactionType,
    pub category: Option<String>,
    pub amount: Decimal,
    #[serde(default = "default_currency")]
    pub currency: String,
    pub transaction_date: NaiveDate,
    pub period_start: Option<NaiveDate>,
    pub period_end: Option<NaiveDate>,
    pub description: Option<String>,
    pub vendor_name: Option<String>,
    pub reference_number: Option<String>,
    pub document_id: Option<Uuid>,
    #[serde(default)]
    pub is_recurring: bool,
    pub recurrence_frequency: Option<String>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdatePropertyTransaction {
    pub transaction_type: Option<PortfolioTransactionType>,
    pub category: Option<String>,
    pub amount: Option<Decimal>,
    pub transaction_date: Option<NaiveDate>,
    pub description: Option<String>,
    pub vendor_name: Option<String>,
    pub reference_number: Option<String>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct TransactionQuery {
    pub property_id: Option<Uuid>,
    pub transaction_type: Option<PortfolioTransactionType>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub category: Option<String>,
}

/// Monthly cash flow summary for a property.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PropertyCashFlow {
    pub id: Uuid,
    pub portfolio_id: Uuid,
    pub property_id: Uuid,
    pub period_year: i32,
    pub period_month: i32,

    pub gross_rental_income: Decimal,
    pub other_income: Decimal,
    pub total_income: Decimal,

    pub operating_expenses: Decimal,
    pub mortgage_payment: Decimal,
    pub capital_expenditures: Decimal,
    pub total_expenses: Decimal,

    pub net_operating_income: Decimal,
    pub cash_flow_before_debt: Decimal,
    pub cash_flow_after_debt: Decimal,

    pub vacancy_rate: Option<Decimal>,
    pub vacancy_cost: Option<Decimal>,

    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpsertPropertyCashFlow {
    pub property_id: Uuid,
    pub period_year: i32,
    pub period_month: i32,
    pub gross_rental_income: Decimal,
    pub other_income: Option<Decimal>,
    pub operating_expenses: Decimal,
    pub mortgage_payment: Option<Decimal>,
    pub capital_expenditures: Option<Decimal>,
    pub vacancy_rate: Option<Decimal>,
    pub vacancy_cost: Option<Decimal>,
    #[serde(default = "default_currency")]
    pub currency: String,
}

// =============================================================================
// STORY 144.3: ROI & FINANCIAL METRICS CALCULATOR
// =============================================================================

/// Calculated financial metrics for a property or portfolio.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct FinancialMetrics {
    pub id: Uuid,
    pub portfolio_id: Uuid,
    pub property_id: Option<Uuid>,
    pub period_type: MetricPeriod,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,

    // Income metrics
    pub gross_income: Decimal,
    pub effective_gross_income: Decimal,
    pub vacancy_loss: Option<Decimal>,
    pub other_income: Option<Decimal>,

    // Expense metrics
    pub operating_expenses: Decimal,
    pub total_debt_service: Option<Decimal>,

    // Core metrics
    pub net_operating_income: Decimal,
    pub cap_rate: Option<Decimal>,
    pub cash_on_cash_return: Option<Decimal>,
    pub gross_rent_multiplier: Option<Decimal>,

    // Advanced metrics
    pub irr: Option<Decimal>,
    pub npv: Option<Decimal>,
    pub equity_multiple: Option<Decimal>,
    pub dscr: Option<Decimal>,

    // Values used for calculations
    pub property_value: Option<Decimal>,
    pub total_investment: Option<Decimal>,
    pub total_equity: Option<Decimal>,
    pub annual_debt_service: Option<Decimal>,

    pub currency: String,
    pub notes: Option<String>,

    pub calculated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CalculateMetricsRequest {
    pub property_id: Option<Uuid>,
    pub period_type: MetricPeriod,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,

    // Override values if not using calculated
    pub property_value: Option<Decimal>,
    pub total_investment: Option<Decimal>,
    pub discount_rate: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct MetricsSummary {
    pub property_id: Option<Uuid>,
    pub property_name: Option<String>,

    pub noi: Decimal,
    pub cap_rate: Option<Decimal>,
    pub cash_on_cash: Option<Decimal>,
    pub irr: Option<Decimal>,
    pub dscr: Option<Decimal>,
    pub equity_multiple: Option<Decimal>,

    pub period: String,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PortfolioMetricsSummary {
    pub portfolio_id: Uuid,
    pub portfolio_name: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,

    // Aggregated metrics
    pub total_noi: Decimal,
    pub weighted_avg_cap_rate: Option<Decimal>,
    pub portfolio_cash_on_cash: Option<Decimal>,
    pub portfolio_irr: Option<Decimal>,
    pub portfolio_dscr: Option<Decimal>,
    pub portfolio_equity_multiple: Option<Decimal>,

    // Portfolio totals
    pub total_value: Decimal,
    pub total_equity: Decimal,
    pub total_debt: Decimal,
    pub leverage_ratio: Option<Decimal>,

    pub property_count: i32,
    pub currency: String,

    pub property_metrics: Vec<MetricsSummary>,
}

// =============================================================================
// STORY 144.4: PERFORMANCE BENCHMARKING
// =============================================================================

/// Market benchmark for comparison.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct MarketBenchmark {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,

    pub source: BenchmarkSource,
    pub source_name: Option<String>,
    pub source_url: Option<String>,
    pub source_date: Option<NaiveDate>,

    pub property_type: Option<String>,
    pub region: Option<String>,
    pub market: Option<String>,

    pub period_year: i32,
    pub period_quarter: Option<i32>,

    // Benchmark values
    pub avg_cap_rate: Option<Decimal>,
    pub avg_cash_on_cash: Option<Decimal>,
    pub avg_noi_per_unit: Option<Decimal>,
    pub avg_price_per_unit: Option<Decimal>,
    pub avg_price_per_sqm: Option<Decimal>,
    pub avg_occupancy: Option<Decimal>,
    pub avg_rent_growth: Option<Decimal>,
    pub avg_expense_ratio: Option<Decimal>,
    pub avg_irr: Option<Decimal>,
    pub avg_equity_multiple: Option<Decimal>,

    pub currency: String,
    pub is_active: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateMarketBenchmark {
    pub name: String,
    pub description: Option<String>,
    #[serde(default = "default_benchmark_source")]
    pub source: BenchmarkSource,
    pub source_name: Option<String>,
    pub source_url: Option<String>,
    pub source_date: Option<NaiveDate>,
    pub property_type: Option<String>,
    pub region: Option<String>,
    pub market: Option<String>,
    pub period_year: i32,
    pub period_quarter: Option<i32>,
    pub avg_cap_rate: Option<Decimal>,
    pub avg_cash_on_cash: Option<Decimal>,
    pub avg_noi_per_unit: Option<Decimal>,
    pub avg_price_per_unit: Option<Decimal>,
    pub avg_price_per_sqm: Option<Decimal>,
    pub avg_occupancy: Option<Decimal>,
    pub avg_rent_growth: Option<Decimal>,
    pub avg_expense_ratio: Option<Decimal>,
    pub avg_irr: Option<Decimal>,
    pub avg_equity_multiple: Option<Decimal>,
    #[serde(default = "default_currency")]
    pub currency: String,
}

fn default_benchmark_source() -> BenchmarkSource {
    BenchmarkSource::Industry
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateMarketBenchmark {
    pub name: Option<String>,
    pub description: Option<String>,
    pub source_date: Option<NaiveDate>,
    pub avg_cap_rate: Option<Decimal>,
    pub avg_cash_on_cash: Option<Decimal>,
    pub avg_noi_per_unit: Option<Decimal>,
    pub avg_occupancy: Option<Decimal>,
    pub avg_irr: Option<Decimal>,
    pub is_active: Option<bool>,
}

/// Performance comparison against benchmarks.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct BenchmarkComparison {
    pub id: Uuid,
    pub portfolio_id: Uuid,
    pub benchmark_id: Uuid,
    pub property_id: Option<Uuid>,

    pub comparison_date: NaiveDate,

    // Actual values
    pub actual_cap_rate: Option<Decimal>,
    pub actual_cash_on_cash: Option<Decimal>,
    pub actual_noi_per_unit: Option<Decimal>,
    pub actual_occupancy: Option<Decimal>,
    pub actual_irr: Option<Decimal>,
    pub actual_equity_multiple: Option<Decimal>,

    // Variance from benchmark (percentage points)
    pub cap_rate_variance: Option<Decimal>,
    pub cash_on_cash_variance: Option<Decimal>,
    pub noi_variance_pct: Option<Decimal>,
    pub occupancy_variance: Option<Decimal>,
    pub irr_variance: Option<Decimal>,

    // Percentile ranking (0-100)
    pub cap_rate_percentile: Option<i32>,
    pub cash_on_cash_percentile: Option<i32>,
    pub overall_percentile: Option<i32>,

    pub performance_score: Option<Decimal>,
    pub performance_rating: Option<String>,
    pub summary: Option<String>,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateBenchmarkComparison {
    pub benchmark_id: Uuid,
    pub property_id: Option<Uuid>,
    pub comparison_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct BenchmarkComparisonSummary {
    pub benchmark_name: String,
    pub benchmark_source: BenchmarkSource,
    pub comparison_date: NaiveDate,

    pub metrics: Vec<MetricComparison>,

    pub overall_performance: String,
    pub overall_percentile: Option<i32>,
    pub performance_score: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct MetricComparison {
    pub metric_name: String,
    pub actual_value: Option<Decimal>,
    pub benchmark_value: Option<Decimal>,
    pub variance: Option<Decimal>,
    pub variance_pct: Option<Decimal>,
    pub percentile: Option<i32>,
    pub status: String, // above_benchmark, at_benchmark, below_benchmark
}

// =============================================================================
// STORY 144.5: PORTFOLIO ANALYTICS DASHBOARD
// =============================================================================

/// Comprehensive portfolio analytics dashboard data.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PortfolioAnalyticsDashboard144 {
    pub portfolio: PerformancePortfolio,
    pub summary: DashboardSummary,
    pub metrics: PortfolioMetricsSummary,
    pub benchmark_comparison: Option<BenchmarkComparisonSummary>,
    pub property_performance: Vec<PropertyPerformanceCard>,
    pub cash_flow_trend: Vec<CashFlowTrendPoint>,
    pub value_trend: Vec<ValueTrendPoint>,
    pub alerts: Vec<PerformanceAlert>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DashboardSummary {
    pub total_portfolio_value: Decimal,
    pub total_equity: Decimal,
    pub total_debt: Decimal,
    pub debt_to_equity_ratio: Option<Decimal>,
    pub ltv_ratio: Option<Decimal>,

    pub ytd_noi: Decimal,
    pub ytd_cash_flow: Decimal,
    pub ytd_return_pct: Option<Decimal>,

    pub property_count: i32,
    pub total_units: Option<i32>,
    pub occupied_units: Option<i32>,
    pub occupancy_rate: Option<Decimal>,

    pub currency: String,
    pub as_of_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PropertyPerformanceCard {
    pub property_id: Uuid,
    pub property_name: String,
    pub building_address: Option<String>,

    pub current_value: Decimal,
    pub equity: Decimal,
    pub ltv: Option<Decimal>,

    pub noi: Decimal,
    pub cap_rate: Option<Decimal>,
    pub cash_on_cash: Option<Decimal>,
    pub dscr: Option<Decimal>,

    pub occupancy_rate: Option<Decimal>,
    pub monthly_cash_flow: Option<Decimal>,

    pub vs_benchmark_pct: Option<Decimal>,
    pub performance_status: String,

    pub currency: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CashFlowTrendPoint {
    pub period: String,
    pub period_date: NaiveDate,
    pub gross_income: Decimal,
    pub operating_expenses: Decimal,
    pub noi: Decimal,
    pub debt_service: Option<Decimal>,
    pub net_cash_flow: Decimal,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ValueTrendPoint {
    pub period: String,
    pub period_date: NaiveDate,
    pub portfolio_value: Decimal,
    pub total_equity: Decimal,
    pub total_debt: Decimal,
    pub appreciation_pct: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PerformanceAlert {
    pub id: Uuid,
    pub portfolio_id: Uuid,
    pub property_id: Option<Uuid>,
    pub alert_type: String,
    pub severity: String,
    pub title: String,
    pub message: String,
    pub metric_name: Option<String>,
    pub current_value: Option<Decimal>,
    pub threshold_value: Option<Decimal>,
    pub is_read: bool,
    pub is_resolved: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreatePerformanceAlert {
    pub property_id: Option<Uuid>,
    pub alert_type: String,
    pub severity: String,
    pub title: String,
    pub message: String,
    pub metric_name: Option<String>,
    pub current_value: Option<Decimal>,
    pub threshold_value: Option<Decimal>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct DashboardQuery {
    pub as_of_date: Option<NaiveDate>,
    pub period_months: Option<i32>,
    pub include_benchmark: Option<bool>,
    pub benchmark_id: Option<Uuid>,
}

/// Export request for portfolio reports.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ExportPortfolioReport {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub include_properties: bool,
    pub include_transactions: bool,
    pub include_metrics: bool,
    pub include_benchmark: bool,
    pub format: String, // pdf, xlsx, csv
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ExportPortfolioReportResponse {
    pub document_id: Uuid,
    pub filename: String,
    pub format: String,
    pub generated_at: DateTime<Utc>,
}
