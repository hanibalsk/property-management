//! Owner analytics models (Epic 74: Owner Investment Analytics).
//! Stub file - full implementation pending.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PropertyValuation {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub valuation_date: NaiveDate,
    pub value_low: Decimal,
    pub value_high: Decimal,
    pub estimated_value: Decimal,
    pub valuation_method: String,
    pub confidence_score: Decimal,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreatePropertyValuation {
    pub unit_id: Uuid,
    pub estimated_value: Decimal,
    pub valuation_method: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PropertyValuationWithComparables {
    pub valuation: PropertyValuation,
    pub comparables: Vec<ComparableProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ComparableProperty {
    pub id: Uuid,
    pub valuation_id: Uuid,
    pub address: String,
    pub sale_price: Decimal,
    pub size_sqm: i32,
    pub rooms: i32,
    pub distance_km: Decimal,
    pub similarity_score: Decimal,
    pub source: String,
    pub sold_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AddComparableProperty {
    pub address: String,
    pub sale_price: Decimal,
    pub size_sqm: i32,
    pub rooms: i32,
    pub distance_km: Decimal,
    pub similarity_score: Decimal,
    pub source: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CalculateROIRequest {
    pub unit_id: Uuid,
    pub total_investment: Decimal,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PropertyROI {
    pub unit_id: Uuid,
    pub total_investment: Decimal,
    pub total_returns: Decimal,
    pub roi_percentage: Decimal,
    pub annualized_roi: Decimal,
    pub period_months: i32,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ROIDashboard {
    pub property_roi: PropertyROI,
    pub cash_flow: CashFlowSummary,
    pub value_trend: ValueTrendSummary,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CashFlowSummary {
    pub total_income: Decimal,
    pub total_expenses: Decimal,
    pub net_cash_flow: Decimal,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ValueTrendSummary {
    pub current_value: Decimal,
    pub trend_percentage: Decimal,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CashFlowBreakdown {
    pub unit_id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub income: CashFlowIncome,
    pub expenses: CashFlowExpenses,
    pub net_cash_flow: Decimal,
    pub cumulative_cash_flow: Decimal,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CashFlowIncome {
    pub rental_income: Decimal,
    pub other_fees: Decimal,
    pub total: Decimal,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CashFlowExpenses {
    pub maintenance: Decimal,
    pub utilities: Decimal,
    pub insurance: Decimal,
    pub property_taxes: Decimal,
    pub management_fees: Decimal,
    pub other: Decimal,
    pub total: Decimal,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct MonthlyCashFlow {
    pub month: NaiveDate,
    pub income: Decimal,
    pub expenses: Decimal,
    pub net: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ExpenseAutoApprovalRule {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub owner_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub max_amount_per_expense: Decimal,
    pub max_monthly_total: Decimal,
    pub allowed_categories: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateAutoApprovalRule {
    pub unit_id: Option<Uuid>,
    pub max_amount_per_expense: Option<Decimal>,
    pub max_monthly_total: Option<Decimal>,
    pub allowed_categories: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateAutoApprovalRule {
    pub unit_id: Option<Uuid>,
    pub max_amount_per_expense: Option<Decimal>,
    pub max_monthly_total: Option<Decimal>,
    pub allowed_categories: Option<Vec<String>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ExpenseApprovalRequest {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub amount: Decimal,
    pub category: String,
    pub description: String,
    pub status: String,
    pub auto_approval_rule_id: Option<Uuid>,
    pub review_notes: Option<String>,
    pub submitted_by: Uuid,
    pub reviewed_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct SubmitExpenseForApproval {
    pub unit_id: Uuid,
    pub amount: Decimal,
    pub category: String,
    pub description: String,
}

/// Decision status for expense approval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "expense_approval_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ExpenseApprovalDecision {
    Approved,
    Rejected,
    NeedsInfo,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ReviewExpenseRequest {
    pub decision: ExpenseApprovalDecision,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ExpenseApprovalResponse {
    pub request: ExpenseApprovalRequest,
    pub auto_approved: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ExpenseApprovalStatus {
    pub pending: i64,
    pub approved: i64,
    pub rejected: i64,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ExpenseRequestsQuery {
    pub property_id: Option<Uuid>,
    pub status: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ListExpenseRequestsResponse {
    pub requests: Vec<ExpenseApprovalRequest>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PortfolioProperty {
    pub unit_id: Uuid,
    pub address: String,
    pub current_value: Decimal,
    pub roi: Decimal,
    pub net_income: Decimal,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PortfolioSummary {
    pub properties: Vec<PortfolioProperty>,
    pub total_value: Decimal,
    pub total_roi: Decimal,
    pub total_net_income: Decimal,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct OwnerPropertiesQuery {
    pub owner_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct PortfolioComparisonRequest {
    pub unit_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PropertyComparison {
    pub unit_id: Uuid,
    pub value: Decimal,
    pub roi: Decimal,
    pub net_income: Decimal,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ComparisonMetrics {
    pub properties: Vec<PropertyComparison>,
    pub best_roi_unit: Uuid,
    pub highest_value_unit: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PropertyValueHistory {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub value: Decimal,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ValueHistoryQuery {
    pub unit_id: Uuid,
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ValueTrendAnalysis {
    pub unit_id: Uuid,
    pub history: Vec<PropertyValueHistory>,
    pub trend_percent: Decimal,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ValuationMethod {
    pub code: String,
    pub name: String,
    pub description: String,
}
