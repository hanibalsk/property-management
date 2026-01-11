//! Investor Portal & ROI Reporting models (Epic 139).
//! Provides investment tracking, ROI calculations, and investor dashboard features.

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema, sqlx::Type)]
#[sqlx(type_name = "investor_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum InvestorType {
    Individual,
    Institutional,
    Reit,
    FamilyOffice,
    Syndicate,
    Fund,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema, sqlx::Type)]
#[sqlx(type_name = "investment_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum InvestmentStatus {
    Active,
    Exited,
    Pending,
    OnHold,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema, sqlx::Type)]
#[sqlx(type_name = "roi_period", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RoiPeriod {
    Monthly,
    Quarterly,
    Annual,
    Ytd,
    Itd,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema, sqlx::Type)]
#[sqlx(type_name = "distribution_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DistributionType {
    CashDividend,
    Reinvestment,
    ReturnOfCapital,
    CapitalGain,
    Interest,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema, sqlx::Type)]
#[sqlx(type_name = "report_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum InvestorReportType {
    Performance,
    TaxSummary,
    DistributionHistory,
    PortfolioOverview,
    CapitalAccount,
}

// =============================================================================
// INVESTOR PROFILES
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InvestorProfile {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Option<Uuid>,
    pub display_name: String,
    pub investor_type: InvestorType,
    pub tax_id: Option<String>,
    pub tax_country: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub preferred_currency: Option<String>,
    pub distribution_preference: Option<DistributionType>,
    pub report_frequency: Option<String>,
    pub kyc_verified: bool,
    pub kyc_verified_at: Option<DateTime<Utc>>,
    pub kyc_document_ids: Option<Vec<Uuid>>,
    pub accredited_investor: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateInvestorProfile {
    pub user_id: Option<Uuid>,
    pub display_name: String,
    #[serde(default)]
    pub investor_type: Option<InvestorType>,
    pub tax_id: Option<String>,
    pub tax_country: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub preferred_currency: Option<String>,
    pub distribution_preference: Option<DistributionType>,
    pub report_frequency: Option<String>,
    pub accredited_investor: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateInvestorProfile {
    pub display_name: Option<String>,
    pub investor_type: Option<InvestorType>,
    pub tax_id: Option<String>,
    pub tax_country: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub preferred_currency: Option<String>,
    pub distribution_preference: Option<DistributionType>,
    pub report_frequency: Option<String>,
    pub kyc_verified: Option<bool>,
    pub accredited_investor: Option<bool>,
}

// =============================================================================
// INVESTMENT PORTFOLIOS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InvestmentPortfolio {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub investor_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: InvestmentStatus,
    pub initial_investment: Decimal,
    pub current_value: Option<Decimal>,
    pub total_contributions: Option<Decimal>,
    pub total_distributions: Option<Decimal>,
    pub ownership_percentage: Option<Decimal>,
    pub currency: Option<String>,
    pub investment_date: NaiveDate,
    pub exit_date: Option<NaiveDate>,
    pub target_exit_date: Option<NaiveDate>,
    pub irr: Option<Decimal>,
    pub multiple: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateInvestmentPortfolio {
    pub investor_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub initial_investment: Decimal,
    pub ownership_percentage: Option<Decimal>,
    pub currency: Option<String>,
    pub investment_date: NaiveDate,
    pub target_exit_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateInvestmentPortfolio {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<InvestmentStatus>,
    pub current_value: Option<Decimal>,
    pub ownership_percentage: Option<Decimal>,
    pub exit_date: Option<NaiveDate>,
    pub target_exit_date: Option<NaiveDate>,
    pub irr: Option<Decimal>,
    pub multiple: Option<Decimal>,
}

// =============================================================================
// PORTFOLIO PROPERTIES
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InvestorPortfolioProperty {
    pub id: Uuid,
    pub portfolio_id: Uuid,
    pub building_id: Uuid,
    pub investment_amount: Decimal,
    pub ownership_share: Decimal,
    pub acquisition_date: NaiveDate,
    pub acquisition_cost: Option<Decimal>,
    pub current_value: Option<Decimal>,
    pub appraised_value: Option<Decimal>,
    pub appraised_at: Option<NaiveDate>,
    pub rental_income_share: Option<Decimal>,
    pub operating_expenses_share: Option<Decimal>,
    pub net_income_share: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateInvestorPortfolioProperty {
    pub building_id: Uuid,
    pub investment_amount: Decimal,
    pub ownership_share: Decimal,
    pub acquisition_date: NaiveDate,
    pub acquisition_cost: Option<Decimal>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateInvestorPortfolioProperty {
    pub investment_amount: Option<Decimal>,
    pub ownership_share: Option<Decimal>,
    pub current_value: Option<Decimal>,
    pub appraised_value: Option<Decimal>,
    pub appraised_at: Option<NaiveDate>,
    pub rental_income_share: Option<Decimal>,
    pub operating_expenses_share: Option<Decimal>,
    pub net_income_share: Option<Decimal>,
}

// =============================================================================
// ROI CALCULATIONS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct RoiCalculation {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub portfolio_id: Uuid,
    pub period_type: RoiPeriod,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub beginning_value: Decimal,
    pub ending_value: Decimal,
    pub contributions: Option<Decimal>,
    pub distributions: Option<Decimal>,
    pub gross_return: Option<Decimal>,
    pub net_return: Option<Decimal>,
    pub return_percentage: Option<Decimal>,
    pub annualized_return: Option<Decimal>,
    pub rental_income: Option<Decimal>,
    pub other_income: Option<Decimal>,
    pub operating_expenses: Option<Decimal>,
    pub capital_expenditures: Option<Decimal>,
    pub unrealized_gain: Option<Decimal>,
    pub realized_gain: Option<Decimal>,
    pub volatility: Option<Decimal>,
    pub sharpe_ratio: Option<Decimal>,
    pub benchmark_return: Option<Decimal>,
    pub alpha: Option<Decimal>,
    pub calculated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateRoiCalculation {
    pub portfolio_id: Uuid,
    pub period_type: RoiPeriod,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub beginning_value: Decimal,
    pub ending_value: Decimal,
    pub contributions: Option<Decimal>,
    pub distributions: Option<Decimal>,
    pub rental_income: Option<Decimal>,
    pub other_income: Option<Decimal>,
    pub operating_expenses: Option<Decimal>,
    pub capital_expenditures: Option<Decimal>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct RoiCalculationQuery {
    pub portfolio_id: Option<Uuid>,
    pub period_type: Option<RoiPeriod>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

// =============================================================================
// DISTRIBUTIONS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InvestorDistribution {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub portfolio_id: Uuid,
    pub investor_id: Uuid,
    pub distribution_type: DistributionType,
    pub amount: Decimal,
    pub currency: Option<String>,
    pub gross_amount: Option<Decimal>,
    pub withholding_tax: Option<Decimal>,
    pub net_amount: Option<Decimal>,
    pub tax_year: Option<i32>,
    pub scheduled_date: NaiveDate,
    pub paid_date: Option<NaiveDate>,
    pub payment_method: Option<String>,
    pub payment_reference: Option<String>,
    pub status: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateDistribution {
    pub portfolio_id: Uuid,
    pub investor_id: Uuid,
    pub distribution_type: DistributionType,
    pub amount: Decimal,
    pub currency: Option<String>,
    pub gross_amount: Option<Decimal>,
    pub withholding_tax: Option<Decimal>,
    pub tax_year: Option<i32>,
    pub scheduled_date: NaiveDate,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateDistribution {
    pub amount: Option<Decimal>,
    pub gross_amount: Option<Decimal>,
    pub withholding_tax: Option<Decimal>,
    pub net_amount: Option<Decimal>,
    pub scheduled_date: Option<NaiveDate>,
    pub paid_date: Option<NaiveDate>,
    pub payment_method: Option<String>,
    pub payment_reference: Option<String>,
    pub status: Option<String>,
}

// =============================================================================
// INVESTOR REPORTS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InvestorReport {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub investor_id: Option<Uuid>,
    pub portfolio_id: Option<Uuid>,
    pub report_type: InvestorReportType,
    pub title: String,
    pub description: Option<String>,
    pub period_start: Option<NaiveDate>,
    pub period_end: Option<NaiveDate>,
    pub report_data: JsonValue,
    pub pdf_document_id: Option<Uuid>,
    pub excel_document_id: Option<Uuid>,
    pub status: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub sent_to_investor: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateInvestorReport {
    pub investor_id: Option<Uuid>,
    pub portfolio_id: Option<Uuid>,
    pub report_type: InvestorReportType,
    pub title: String,
    pub description: Option<String>,
    pub period_start: Option<NaiveDate>,
    pub period_end: Option<NaiveDate>,
    pub report_data: JsonValue,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateInvestorReport {
    pub title: Option<String>,
    pub description: Option<String>,
    pub report_data: Option<JsonValue>,
    pub status: Option<String>,
    pub pdf_document_id: Option<Uuid>,
    pub excel_document_id: Option<Uuid>,
}

// =============================================================================
// CAPITAL CALLS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CapitalCall {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub portfolio_id: Uuid,
    pub investor_id: Uuid,
    pub call_number: i32,
    pub amount: Decimal,
    pub currency: Option<String>,
    pub purpose: Option<String>,
    pub call_date: NaiveDate,
    pub due_date: NaiveDate,
    pub funded_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub funded_amount: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateCapitalCall {
    pub portfolio_id: Uuid,
    pub investor_id: Uuid,
    pub call_number: i32,
    pub amount: Decimal,
    pub currency: Option<String>,
    pub purpose: Option<String>,
    pub call_date: NaiveDate,
    pub due_date: NaiveDate,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateCapitalCall {
    pub amount: Option<Decimal>,
    pub purpose: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub funded_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub funded_amount: Option<Decimal>,
}

// =============================================================================
// DASHBOARD METRICS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InvestorDashboardMetrics {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub investor_id: Uuid,
    pub metric_date: NaiveDate,
    pub total_invested: Decimal,
    pub total_value: Decimal,
    pub total_distributions: Option<Decimal>,
    pub total_return: Option<Decimal>,
    pub ytd_return: Option<Decimal>,
    pub itd_return: Option<Decimal>,
    pub irr: Option<Decimal>,
    pub cash_on_cash: Option<Decimal>,
    pub equity_multiple: Option<Decimal>,
    pub property_count: Option<i32>,
    pub portfolio_count: Option<i32>,
    pub monthly_income: Option<Decimal>,
    pub annual_income: Option<Decimal>,
    pub yield_percentage: Option<Decimal>,
    pub calculated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateDashboardMetrics {
    pub investor_id: Uuid,
    pub metric_date: NaiveDate,
    pub total_invested: Decimal,
    pub total_value: Decimal,
    pub total_distributions: Option<Decimal>,
    pub ytd_return: Option<Decimal>,
    pub itd_return: Option<Decimal>,
    pub irr: Option<Decimal>,
    pub cash_on_cash: Option<Decimal>,
    pub equity_multiple: Option<Decimal>,
    pub property_count: Option<i32>,
    pub portfolio_count: Option<i32>,
    pub monthly_income: Option<Decimal>,
    pub annual_income: Option<Decimal>,
}

// =============================================================================
// SUMMARY TYPES
// =============================================================================

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct InvestorSummary {
    pub id: Uuid,
    pub display_name: String,
    pub investor_type: InvestorType,
    pub portfolio_count: i64,
    pub total_invested: Decimal,
    pub total_value: Decimal,
    pub overall_return: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct InvestorPortfolioSummary {
    pub id: Uuid,
    pub name: String,
    pub status: InvestmentStatus,
    pub initial_investment: Decimal,
    pub current_value: Option<Decimal>,
    pub irr: Option<Decimal>,
    pub property_count: i64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct InvestorPortalDashboard {
    pub investor_id: Uuid,
    pub investor_name: String,
    pub total_invested: Decimal,
    pub total_current_value: Decimal,
    pub total_distributions: Decimal,
    pub overall_irr: Option<Decimal>,
    pub equity_multiple: Option<Decimal>,
    pub ytd_return: Option<Decimal>,
    pub portfolio_count: i32,
    pub property_count: i32,
    pub pending_capital_calls: i32,
    pub recent_distributions: Vec<InvestorDistribution>,
    pub portfolio_summaries: Vec<InvestorPortfolioSummary>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct RoiSummary {
    pub period_type: RoiPeriod,
    pub return_percentage: Option<Decimal>,
    pub gross_return: Option<Decimal>,
    pub net_return: Option<Decimal>,
    pub benchmark_return: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PortfolioWithDetails {
    pub portfolio: InvestmentPortfolio,
    pub investor: InvestorProfile,
    pub properties: Vec<InvestorPortfolioProperty>,
    pub recent_roi: Option<RoiCalculation>,
}
