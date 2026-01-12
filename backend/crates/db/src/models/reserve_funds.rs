//! Reserve Fund Management models for Epic 141.
//!
//! Provides models for HOA/Condo reserve fund management including:
//! - Reserve funds and fund types
//! - Contribution schedules
//! - Fund transactions
//! - Investment policies
//! - Fund projections (reserve studies)
//! - Component tracking

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Fund type enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "fund_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum FundType {
    Operating,
    Reserve,
    Emergency,
    SpecialAssessment,
    CapitalImprovement,
    Insurance,
    Legal,
    Custom,
}

/// Contribution frequency enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "contribution_frequency", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ContributionFrequency {
    Monthly,
    Quarterly,
    SemiAnnually,
    Annually,
    OneTime,
}

/// Investment risk level enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "investment_risk_level", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum InvestmentRiskLevel {
    Conservative,
    Moderate,
    Balanced,
    Growth,
    Aggressive,
}

/// Fund transaction type enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "fund_transaction_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum FundTransactionType {
    Contribution,
    Withdrawal,
    Transfer,
    Interest,
    Dividend,
    Fee,
    Adjustment,
    OpeningBalance,
}

/// Reserve fund entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReserveFund {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub fund_type: FundType,
    pub current_balance: Decimal,
    pub target_balance: Option<Decimal>,
    pub minimum_balance: Option<Decimal>,
    pub currency: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

/// Fund contribution schedule.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FundContributionSchedule {
    pub id: Uuid,
    pub fund_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub amount: Decimal,
    pub frequency: ContributionFrequency,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub next_due_date: Option<NaiveDate>,
    pub is_active: bool,
    pub auto_collect: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Fund transaction.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FundTransaction {
    pub id: Uuid,
    pub fund_id: Uuid,
    pub transaction_type: FundTransactionType,
    pub amount: Decimal,
    pub balance_after: Decimal,
    pub description: Option<String>,
    pub reference_number: Option<String>,
    pub contribution_schedule_id: Option<Uuid>,
    pub transfer_to_fund_id: Option<Uuid>,
    pub requires_approval: bool,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub transaction_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

/// Fund investment policy.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FundInvestmentPolicy {
    pub id: Uuid,
    pub fund_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub risk_level: InvestmentRiskLevel,
    pub cash_allocation_pct: Decimal,
    pub bonds_allocation_pct: Decimal,
    pub money_market_allocation_pct: Decimal,
    pub other_allocation_pct: Decimal,
    pub max_single_investment: Option<Decimal>,
    pub min_liquidity_pct: Option<Decimal>,
    pub is_active: bool,
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
}

/// Fund projection (reserve study).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FundProjection {
    pub id: Uuid,
    pub fund_id: Uuid,
    pub study_name: String,
    pub study_date: NaiveDate,
    pub projection_years: i32,
    pub annual_inflation_rate: Decimal,
    pub annual_interest_rate: Decimal,
    pub starting_balance: Decimal,
    pub recommended_annual_contribution: Option<Decimal>,
    pub funding_status_pct: Option<Decimal>,
    pub is_current: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub prepared_by: Option<String>,
}

/// Fund projection line item.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FundProjectionItem {
    pub id: Uuid,
    pub projection_id: Uuid,
    pub projection_year: i32,
    pub fiscal_year: i32,
    pub contributions: Decimal,
    pub interest_income: Decimal,
    pub planned_expenditures: Decimal,
    pub beginning_balance: Decimal,
    pub ending_balance: Decimal,
    pub expenditure_details: Option<serde_json::Value>,
}

/// Fund component (e.g., roof, HVAC).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FundComponent {
    pub id: Uuid,
    pub fund_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub current_replacement_cost: Option<Decimal>,
    pub useful_life_years: Option<i32>,
    pub remaining_life_years: Option<i32>,
    pub condition_rating: Option<i32>,
    pub last_inspection_date: Option<NaiveDate>,
    pub next_replacement_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Fund alert.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FundAlert {
    pub id: Uuid,
    pub fund_id: Uuid,
    pub alert_type: String,
    pub severity: String,
    pub title: String,
    pub message: String,
    pub threshold_value: Option<Decimal>,
    pub current_value: Option<Decimal>,
    pub is_active: bool,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub acknowledged_by: Option<Uuid>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// DTOs
// ============================================================================

/// Create reserve fund request.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateReserveFund {
    pub building_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub fund_type: FundType,
    pub target_balance: Option<Decimal>,
    pub minimum_balance: Option<Decimal>,
    pub currency: Option<String>,
}

/// Update reserve fund request.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateReserveFund {
    pub name: Option<String>,
    pub description: Option<String>,
    pub fund_type: Option<FundType>,
    pub target_balance: Option<Decimal>,
    pub minimum_balance: Option<Decimal>,
    pub is_active: Option<bool>,
}

/// Create contribution schedule request.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateContributionSchedule {
    pub name: String,
    pub description: Option<String>,
    pub amount: Decimal,
    pub frequency: ContributionFrequency,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub auto_collect: Option<bool>,
}

/// Update contribution schedule request.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateContributionSchedule {
    pub name: Option<String>,
    pub description: Option<String>,
    pub amount: Option<Decimal>,
    pub frequency: Option<ContributionFrequency>,
    pub end_date: Option<NaiveDate>,
    pub is_active: Option<bool>,
    pub auto_collect: Option<bool>,
}

/// Record fund transaction request.
#[derive(Debug, Clone, Deserialize)]
pub struct RecordFundTransaction {
    pub transaction_type: FundTransactionType,
    pub amount: Decimal,
    pub description: Option<String>,
    pub reference_number: Option<String>,
    pub contribution_schedule_id: Option<Uuid>,
    pub transfer_to_fund_id: Option<Uuid>,
    pub requires_approval: Option<bool>,
}

/// Create investment policy request.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateInvestmentPolicy {
    pub name: String,
    pub description: Option<String>,
    pub risk_level: InvestmentRiskLevel,
    pub cash_allocation_pct: Decimal,
    pub bonds_allocation_pct: Decimal,
    pub money_market_allocation_pct: Decimal,
    pub other_allocation_pct: Decimal,
    pub max_single_investment: Option<Decimal>,
    pub min_liquidity_pct: Option<Decimal>,
    pub effective_date: NaiveDate,
}

/// Create fund projection request.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateFundProjection {
    pub study_name: String,
    pub study_date: NaiveDate,
    pub projection_years: i32,
    pub annual_inflation_rate: Option<Decimal>,
    pub annual_interest_rate: Option<Decimal>,
    pub recommended_annual_contribution: Option<Decimal>,
    pub prepared_by: Option<String>,
}

/// Create projection item request.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateProjectionItem {
    pub projection_year: i32,
    pub fiscal_year: i32,
    pub contributions: Decimal,
    pub interest_income: Decimal,
    pub planned_expenditures: Decimal,
    pub beginning_balance: Decimal,
    pub ending_balance: Decimal,
    pub expenditure_details: Option<serde_json::Value>,
}

/// Create fund component request.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateFundComponent {
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub current_replacement_cost: Option<Decimal>,
    pub useful_life_years: Option<i32>,
    pub remaining_life_years: Option<i32>,
    pub condition_rating: Option<i32>,
    pub last_inspection_date: Option<NaiveDate>,
    pub next_replacement_date: Option<NaiveDate>,
}

/// Update fund component request.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateFundComponent {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub current_replacement_cost: Option<Decimal>,
    pub useful_life_years: Option<i32>,
    pub remaining_life_years: Option<i32>,
    pub condition_rating: Option<i32>,
    pub last_inspection_date: Option<NaiveDate>,
    pub next_replacement_date: Option<NaiveDate>,
}

/// Acknowledge fund alert request.
#[derive(Debug, Clone, Deserialize)]
pub struct AcknowledgeFundAlert {
    pub notes: Option<String>,
}

/// Resolve fund alert request.
#[derive(Debug, Clone, Deserialize)]
pub struct ResolveFundAlert {
    pub notes: Option<String>,
}

// ============================================================================
// Response DTOs
// ============================================================================

/// Fund summary with calculated fields.
#[derive(Debug, Clone, Serialize)]
pub struct FundSummary {
    pub id: Uuid,
    pub name: String,
    pub fund_type: FundType,
    pub current_balance: Decimal,
    pub target_balance: Option<Decimal>,
    pub funding_percentage: Option<Decimal>,
    pub is_below_minimum: bool,
    pub upcoming_contributions: Decimal,
    pub active_alerts_count: i64,
}

/// Fund dashboard data.
#[derive(Debug, Clone, Serialize)]
pub struct FundDashboard {
    pub total_fund_balance: Decimal,
    pub total_target_balance: Decimal,
    pub overall_funding_percentage: Decimal,
    pub funds_below_minimum: i64,
    pub active_alerts: i64,
    pub funds: Vec<FundSummary>,
}

/// Transaction history query.
#[derive(Debug, Clone, Deserialize)]
pub struct TransactionQuery {
    pub fund_id: Option<Uuid>,
    pub transaction_type: Option<FundTransactionType>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Fund health report.
#[derive(Debug, Clone, Serialize)]
pub struct FundHealthReport {
    pub fund_id: Uuid,
    pub fund_name: String,
    pub current_balance: Decimal,
    pub target_balance: Option<Decimal>,
    pub funding_status_pct: Option<Decimal>,
    pub health_score: i32,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
}

/// Reserve study summary.
#[derive(Debug, Clone, Serialize)]
pub struct ReserveStudySummary {
    pub projection_id: Uuid,
    pub study_name: String,
    pub study_date: NaiveDate,
    pub starting_balance: Decimal,
    pub projected_ending_balance: Decimal,
    pub total_contributions: Decimal,
    pub total_expenditures: Decimal,
    pub funding_status_pct: Option<Decimal>,
    pub is_current: bool,
}

/// Component replacement schedule.
#[derive(Debug, Clone, Serialize)]
pub struct ComponentReplacementSchedule {
    pub component_id: Uuid,
    pub component_name: String,
    pub category: Option<String>,
    pub replacement_cost: Option<Decimal>,
    pub remaining_years: Option<i32>,
    pub next_replacement_date: Option<NaiveDate>,
    pub condition_rating: Option<i32>,
}

/// Fund transfer request.
#[derive(Debug, Clone, Deserialize)]
pub struct FundTransferRequest {
    pub from_fund_id: Uuid,
    pub to_fund_id: Uuid,
    pub amount: Decimal,
    pub description: Option<String>,
}
