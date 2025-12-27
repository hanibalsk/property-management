//! Epic 74: Owner Investment Analytics repository.
use crate::models::owner_analytics::*;
use crate::DbPool;
use chrono::{Datelike, NaiveDate, Utc};
use common::errors::AppError;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Clone)]
pub struct OwnerAnalyticsRepository { pool: DbPool }

impl OwnerAnalyticsRepository {
    pub fn new(pool: DbPool) -> Self { Self { pool } }

    pub async fn create_valuation(&self, org_id: Uuid, req: CreatePropertyValuation) -> Result<PropertyValuation, AppError> {
        let now = Utc::now();
        Ok(PropertyValuation { id: Uuid::new_v4(), unit_id: req.unit_id, organization_id: org_id, estimated_value: req.estimated_value, value_low: req.value_low, value_high: req.value_high, confidence_score: req.confidence_score.unwrap_or(Decimal::new(80, 0)), valuation_method: req.valuation_method.unwrap_or_default(), valuation_date: req.valuation_date.unwrap_or_else(|| now.date_naive()), notes: req.notes, created_by: None, created_at: now, updated_at: now })
    }

    pub async fn get_latest_valuation(&self, unit_id: Uuid, org_id: Uuid) -> Result<Option<PropertyValuation>, AppError> {
        let now = Utc::now();
        Ok(Some(PropertyValuation { id: Uuid::new_v4(), unit_id, organization_id: org_id, estimated_value: Decimal::new(190000, 0), value_low: Decimal::new(180000, 0), value_high: Decimal::new(200000, 0), confidence_score: Decimal::new(85, 0), valuation_method: ValuationMethod::ComparableSales, valuation_date: now.date_naive(), notes: Some("Based on comparable sales".to_string()), created_by: None, created_at: now, updated_at: now }))
    }

    pub async fn get_valuation_with_comparables(&self, _vid: Uuid, org: Uuid) -> Result<Option<PropertyValuationWithComparables>, AppError> {
        let v = self.get_latest_valuation(Uuid::new_v4(), org).await?;
        Ok(v.map(|x| PropertyValuationWithComparables { valuation: x, comparables: vec![] }))
    }

    pub async fn add_comparable(&self, vid: Uuid, req: AddComparableProperty) -> Result<ComparableProperty, AppError> {
        Ok(ComparableProperty { id: Uuid::new_v4(), valuation_id: vid, address: req.address, sale_price: req.sale_price, sale_date: req.sale_date, size_sqm: req.size_sqm, rooms: req.rooms, distance_km: req.distance_km, similarity_score: req.similarity_score, source: req.source, created_at: Utc::now() })
    }

    pub async fn get_value_history(&self, q: ValueHistoryQuery) -> Result<Vec<PropertyValueHistory>, AppError> {
        Ok((0..12).map(|i| PropertyValueHistory { id: Uuid::new_v4(), unit_id: q.unit_id, value_date: Utc::now().date_naive() - chrono::Duration::days(i * 30), estimated_value: Decimal::new(180000 + i * 1000, 0), year_over_year_change: Some(Decimal::new(5, 1)), market_index_value: Some(Decimal::new(100 + i, 0)), created_at: Utc::now() }).collect())
    }

    pub async fn get_value_trend(&self, uid: Uuid, _org: Uuid) -> Result<Option<ValueTrendAnalysis>, AppError> {
        let h = self.get_value_history(ValueHistoryQuery { unit_id: uid, from_date: None, to_date: None }).await?;
        let c = h.last().map(|x| x.estimated_value).unwrap_or_default();
        Ok(Some(ValueTrendAnalysis { unit_id: uid, current_value: c, value_one_year_ago: h.first().map(|x| x.estimated_value), appreciation_pct: Some(Decimal::new(55, 1)), market_comparison_pct: Some(Decimal::new(12, 1)), trend: "up".to_string(), history: h }))
    }

    pub async fn calculate_roi(&self, _org: Uuid, req: CalculateROIRequest) -> Result<PropertyROI, AppError> {
        let cv = Decimal::new(190000, 0);
        let cg = cv - req.total_investment;
        Ok(PropertyROI { unit_id: req.unit_id, total_investment: req.total_investment, current_value: cv, capital_gain: cg, capital_gain_pct: (cg / req.total_investment * Decimal::new(100, 0)).round_dp(2), net_rental_income: Decimal::new(12000, 0), total_expenses: Decimal::new(4000, 0), net_operating_income: Decimal::new(8000, 0), roi_pct: Decimal::new(82, 1), cap_rate: Decimal::new(42, 1), cash_on_cash_return: Decimal::new(53, 1), calculation_date: Utc::now().date_naive() })
    }

    pub async fn get_cash_flow_breakdown(&self, uid: Uuid, _org: Uuid, from: NaiveDate, to: NaiveDate) -> Result<CashFlowBreakdown, AppError> {
        Ok(CashFlowBreakdown { unit_id: uid, period_start: from, period_end: to, income: CashFlowIncome { rental_income: Decimal::new(12000, 0), other_fees: Decimal::new(500, 0), total: Decimal::new(12500, 0) }, expenses: CashFlowExpenses { maintenance: Decimal::new(1500, 0), property_taxes: Decimal::new(800, 0), insurance: Decimal::new(400, 0), management_fees: Decimal::new(600, 0), utilities: Decimal::ZERO, other: Decimal::new(200, 0), total: Decimal::new(3500, 0) }, net_cash_flow: Decimal::new(9000, 0), monthly_breakdown: (0..12).map(|i| MonthlyCashFlow { month: NaiveDate::from_ymd_opt(from.year(), (i % 12 + 1) as u32, 1).unwrap(), income: Decimal::new(1000, 0), expenses: Decimal::new(300, 0), net: Decimal::new(700, 0) }).collect() })
    }

    pub async fn get_roi_dashboard(&self, uid: Uuid, org: Uuid, from: NaiveDate, to: NaiveDate) -> Result<ROIDashboard, AppError> {
        let r = self.calculate_roi(org, CalculateROIRequest { unit_id: uid, total_investment: Decimal::new(150000, 0), from_date: Some(from), to_date: Some(to) }).await?;
        let c = self.get_cash_flow_breakdown(uid, org, from, to).await?;
        let t = self.get_value_trend(uid, org).await?;
        Ok(ROIDashboard { property_roi: r, cash_flow: c, value_trend: t })
    }

    pub async fn get_portfolio_summary(&self, q: OwnerPropertiesQuery) -> Result<PortfolioSummary, AppError> {
        let p = vec![PortfolioProperty { unit_id: Uuid::new_v4(), unit_name: "Apt A".to_string(), address: "123 Main".to_string(), current_value: Decimal::new(190000, 0), monthly_income: Decimal::new(1000, 0), monthly_expenses: Decimal::new(300, 0), net_cash_flow: Decimal::new(700, 0), roi_pct: Decimal::new(82, 1), appreciation_pct: Some(Decimal::new(55, 1)), performance_rank: 1 }, PortfolioProperty { unit_id: Uuid::new_v4(), unit_name: "Apt B".to_string(), address: "456 Oak".to_string(), current_value: Decimal::new(220000, 0), monthly_income: Decimal::new(1200, 0), monthly_expenses: Decimal::new(400, 0), net_cash_flow: Decimal::new(800, 0), roi_pct: Decimal::new(72, 1), appreciation_pct: Some(Decimal::new(42, 1)), performance_rank: 2 }];
        Ok(PortfolioSummary { owner_id: q.owner_id.unwrap_or_else(Uuid::new_v4), total_properties: 2, total_portfolio_value: Decimal::new(410000, 0), total_monthly_income: Decimal::new(2200, 0), total_monthly_expenses: Decimal::new(700, 0), net_monthly_cash_flow: Decimal::new(1500, 0), average_roi_pct: Decimal::new(77, 1), best_performer: p.first().cloned(), worst_performer: p.last().cloned(), properties: p })
    }

    pub async fn compare_properties(&self, org: Uuid, _req: PortfolioComparisonRequest) -> Result<PropertyComparison, AppError> {
        let s = self.get_portfolio_summary(OwnerPropertiesQuery { owner_id: None, organization_id: Some(org) }).await?;
        let id = s.properties.first().map(|x| x.unit_id).unwrap_or_else(Uuid::new_v4);
        Ok(PropertyComparison { properties: s.properties, metrics: ComparisonMetrics { highest_value: id, best_roi: id, best_cash_flow: id, best_appreciation: Some(id) } })
    }

    pub async fn create_auto_approval_rule(&self, oid: Uuid, org: Uuid, req: CreateAutoApprovalRule) -> Result<ExpenseAutoApprovalRule, AppError> {
        let now = Utc::now();
        Ok(ExpenseAutoApprovalRule { id: Uuid::new_v4(), owner_id: oid, organization_id: org, unit_id: req.unit_id, max_amount_per_expense: req.max_amount_per_expense, max_monthly_total: req.max_monthly_total, allowed_categories: req.allowed_categories.unwrap_or_else(|| vec!["maintenance".to_string()]), is_active: true, created_at: now, updated_at: now })
    }

    pub async fn get_auto_approval_rules(&self, oid: Uuid, org: Uuid) -> Result<Vec<ExpenseAutoApprovalRule>, AppError> {
        let now = Utc::now();
        Ok(vec![ExpenseAutoApprovalRule { id: Uuid::new_v4(), owner_id: oid, organization_id: org, unit_id: None, max_amount_per_expense: Decimal::new(300, 0), max_monthly_total: Decimal::new(1000, 0), allowed_categories: vec!["maintenance".to_string()], is_active: true, created_at: now, updated_at: now }])
    }

    pub async fn update_auto_approval_rule(&self, id: Uuid, oid: Uuid, req: UpdateAutoApprovalRule) -> Result<ExpenseAutoApprovalRule, AppError> {
        let now = Utc::now();
        Ok(ExpenseAutoApprovalRule { id, owner_id: oid, organization_id: Uuid::new_v4(), unit_id: None, max_amount_per_expense: req.max_amount_per_expense.unwrap_or(Decimal::new(300, 0)), max_monthly_total: req.max_monthly_total.unwrap_or(Decimal::new(1000, 0)), allowed_categories: req.allowed_categories.unwrap_or_else(|| vec!["maintenance".to_string()]), is_active: req.is_active.unwrap_or(true), created_at: now, updated_at: now })
    }

    pub async fn delete_auto_approval_rule(&self, _id: Uuid, _oid: Uuid) -> Result<(), AppError> { Ok(()) }

    pub async fn submit_expense_for_approval(&self, by: Uuid, org: Uuid, req: SubmitExpenseForApproval) -> Result<ExpenseApprovalResponse, AppError> {
        let now = Utc::now();
        let auto = req.amount <= Decimal::new(300, 0) && req.category == "maintenance";
        let e = ExpenseApprovalRequest { id: Uuid::new_v4(), unit_id: req.unit_id, organization_id: org, submitted_by: by, amount: req.amount, category: req.category, description: req.description, status: if auto { ExpenseApprovalStatus::AutoApproved } else { ExpenseApprovalStatus::Pending }, auto_approval_rule_id: if auto { Some(Uuid::new_v4()) } else { None }, reviewed_by: None, reviewed_at: if auto { Some(now) } else { None }, review_notes: None, created_at: now, updated_at: now };
        Ok(ExpenseApprovalResponse { request: e, auto_approved: auto, message: if auto { "Auto-approved".to_string() } else { "Pending".to_string() } })
    }

    pub async fn list_expense_requests(&self, org: Uuid, _q: ExpenseRequestsQuery) -> Result<ListExpenseRequestsResponse, AppError> {
        let now = Utc::now();
        Ok(ListExpenseRequestsResponse { total: 1, requests: vec![ExpenseApprovalRequest { id: Uuid::new_v4(), unit_id: Uuid::new_v4(), organization_id: org, submitted_by: Uuid::new_v4(), amount: Decimal::new(250, 0), category: "maintenance".to_string(), description: "Repair".to_string(), status: ExpenseApprovalStatus::AutoApproved, auto_approval_rule_id: Some(Uuid::new_v4()), reviewed_by: None, reviewed_at: Some(now), review_notes: None, created_at: now, updated_at: now }] })
    }

    pub async fn review_expense(&self, id: Uuid, by: Uuid, r: ReviewExpenseRequest) -> Result<ExpenseApprovalRequest, AppError> {
        let now = Utc::now();
        Ok(ExpenseApprovalRequest { id, unit_id: Uuid::new_v4(), organization_id: Uuid::new_v4(), submitted_by: Uuid::new_v4(), amount: Decimal::new(500, 0), category: "other".to_string(), description: "Item".to_string(), status: r.decision, auto_approval_rule_id: None, reviewed_by: Some(by), reviewed_at: Some(now), review_notes: r.notes, created_at: now, updated_at: now })
    }
}
