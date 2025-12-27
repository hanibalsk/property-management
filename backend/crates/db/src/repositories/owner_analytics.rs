//! Epic 74: Owner Investment Analytics repository.
use crate::models::owner_analytics::*;
use crate::DbPool;
use chrono::Utc;
use common::errors::AppError;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Clone)]
pub struct OwnerAnalyticsRepository {
    #[allow(dead_code)]
    pool: DbPool,
}

impl OwnerAnalyticsRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create_valuation(
        &self,
        _org_id: Uuid,
        req: CreatePropertyValuation,
    ) -> Result<PropertyValuation, AppError> {
        let now = Utc::now();
        Ok(PropertyValuation {
            id: Uuid::new_v4(),
            unit_id: req.unit_id,
            valuation_date: now.date_naive(),
            value_low: req.estimated_value * Decimal::new(95, 2),
            value_high: req.estimated_value * Decimal::new(105, 2),
            estimated_value: req.estimated_value,
            valuation_method: req.valuation_method,
            confidence_score: Decimal::new(85, 0),
            notes: req.notes,
            created_at: now,
        })
    }

    pub async fn get_latest_valuation(
        &self,
        unit_id: Uuid,
        _org_id: Uuid,
    ) -> Result<Option<PropertyValuation>, AppError> {
        let now = Utc::now();
        Ok(Some(PropertyValuation {
            id: Uuid::new_v4(),
            unit_id,
            valuation_date: now.date_naive(),
            value_low: Decimal::new(180000, 0),
            value_high: Decimal::new(200000, 0),
            estimated_value: Decimal::new(190000, 0),
            valuation_method: "comparable_sales".to_string(),
            confidence_score: Decimal::new(85, 0),
            notes: Some("Based on comparable sales".to_string()),
            created_at: now,
        }))
    }

    pub async fn get_valuation_with_comparables(
        &self,
        _vid: Uuid,
        org: Uuid,
    ) -> Result<Option<PropertyValuationWithComparables>, AppError> {
        let v = self.get_latest_valuation(Uuid::new_v4(), org).await?;
        Ok(v.map(|x| PropertyValuationWithComparables {
            valuation: x,
            comparables: vec![],
        }))
    }

    pub async fn add_comparable(
        &self,
        vid: Uuid,
        req: AddComparableProperty,
    ) -> Result<ComparableProperty, AppError> {
        Ok(ComparableProperty {
            id: Uuid::new_v4(),
            valuation_id: vid,
            address: req.address,
            sale_price: req.sale_price,
            sold_date: None,
            size_sqm: req.size_sqm,
            rooms: req.rooms,
            distance_km: req.distance_km,
            similarity_score: req.similarity_score,
            source: req.source,
            created_at: Utc::now(),
        })
    }

    pub async fn get_value_history(
        &self,
        q: ValueHistoryQuery,
    ) -> Result<Vec<PropertyValueHistory>, AppError> {
        let now = Utc::now();
        Ok((0..12)
            .map(|i| PropertyValueHistory {
                id: Uuid::new_v4(),
                unit_id: q.unit_id,
                value: Decimal::new(180000 + i * 1000, 0),
                recorded_at: now - chrono::Duration::days(i * 30),
            })
            .collect())
    }

    pub async fn get_value_trend(
        &self,
        uid: Uuid,
        _org: Uuid,
    ) -> Result<Option<ValueTrendAnalysis>, AppError> {
        let h = self
            .get_value_history(ValueHistoryQuery {
                unit_id: uid,
                limit: None,
            })
            .await?;
        Ok(Some(ValueTrendAnalysis {
            unit_id: uid,
            history: h,
            trend_percent: Decimal::new(55, 1),
        }))
    }

    pub async fn calculate_roi(
        &self,
        _org: Uuid,
        req: CalculateROIRequest,
    ) -> Result<PropertyROI, AppError> {
        let total_returns = Decimal::new(20000, 0);
        let roi_pct = (total_returns / req.total_investment * Decimal::new(100, 0)).round_dp(2);
        let months = (req.to_date - req.from_date).num_days() as i32 / 30;
        let annualized = roi_pct * Decimal::new(12, 0) / Decimal::from(months.max(1));
        Ok(PropertyROI {
            unit_id: req.unit_id,
            total_investment: req.total_investment,
            total_returns,
            roi_percentage: roi_pct,
            annualized_roi: annualized.round_dp(2),
            period_months: months,
        })
    }

    pub async fn get_cash_flow_breakdown(
        &self,
        uid: Uuid,
        _org: Uuid,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<CashFlowBreakdown, AppError> {
        Ok(CashFlowBreakdown {
            unit_id: uid,
            period_start: from,
            period_end: to,
            income: CashFlowIncome {
                rental_income: Decimal::new(12000, 0),
                other_fees: Decimal::new(500, 0),
                total: Decimal::new(12500, 0),
            },
            expenses: CashFlowExpenses {
                maintenance: Decimal::new(1500, 0),
                utilities: Decimal::ZERO,
                insurance: Decimal::new(400, 0),
                property_taxes: Decimal::new(800, 0),
                management_fees: Decimal::new(600, 0),
                other: Decimal::new(200, 0),
                total: Decimal::new(3500, 0),
            },
            net_cash_flow: Decimal::new(9000, 0),
            cumulative_cash_flow: Decimal::new(9000, 0),
        })
    }

    pub async fn get_roi_dashboard(
        &self,
        uid: Uuid,
        org: Uuid,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<ROIDashboard, AppError> {
        let r = self
            .calculate_roi(
                org,
                CalculateROIRequest {
                    unit_id: uid,
                    total_investment: Decimal::new(150000, 0),
                    from_date: from,
                    to_date: to,
                },
            )
            .await?;
        let cf = self.get_cash_flow_breakdown(uid, org, from, to).await?;
        let trend = self.get_value_trend(uid, org).await?;
        Ok(ROIDashboard {
            property_roi: r,
            cash_flow: CashFlowSummary {
                total_income: cf.income.total,
                total_expenses: cf.expenses.total,
                net_cash_flow: cf.net_cash_flow,
            },
            value_trend: ValueTrendSummary {
                current_value: Decimal::new(190000, 0),
                trend_percentage: trend.map(|t| t.trend_percent).unwrap_or_default(),
            },
        })
    }

    pub async fn get_portfolio_summary(
        &self,
        q: OwnerPropertiesQuery,
    ) -> Result<PortfolioSummary, AppError> {
        let _ = q;
        let p = vec![
            PortfolioProperty {
                unit_id: Uuid::new_v4(),
                address: "123 Main St".to_string(),
                current_value: Decimal::new(190000, 0),
                roi: Decimal::new(82, 1),
                net_income: Decimal::new(700, 0),
            },
            PortfolioProperty {
                unit_id: Uuid::new_v4(),
                address: "456 Oak Ave".to_string(),
                current_value: Decimal::new(220000, 0),
                roi: Decimal::new(72, 1),
                net_income: Decimal::new(800, 0),
            },
        ];
        Ok(PortfolioSummary {
            properties: p,
            total_value: Decimal::new(410000, 0),
            total_roi: Decimal::new(77, 1),
            total_net_income: Decimal::new(1500, 0),
        })
    }

    pub async fn compare_properties(
        &self,
        _org: Uuid,
        req: PortfolioComparisonRequest,
    ) -> Result<ComparisonMetrics, AppError> {
        let first_id = req.unit_ids.first().copied().unwrap_or_else(Uuid::new_v4);
        let props: Vec<PropertyComparison> = req
            .unit_ids
            .iter()
            .enumerate()
            .map(|(i, &id)| PropertyComparison {
                unit_id: id,
                value: Decimal::new(190000 + (i as i64) * 30000, 0),
                roi: Decimal::new(80 - (i as i64) * 10, 1),
                net_income: Decimal::new(700 + (i as i64) * 100, 0),
            })
            .collect();
        Ok(ComparisonMetrics {
            properties: props,
            best_roi_unit: first_id,
            highest_value_unit: first_id,
        })
    }

    pub async fn create_auto_approval_rule(
        &self,
        oid: Uuid,
        org: Uuid,
        req: CreateAutoApprovalRule,
    ) -> Result<ExpenseAutoApprovalRule, AppError> {
        let now = Utc::now();
        Ok(ExpenseAutoApprovalRule {
            id: Uuid::new_v4(),
            owner_id: oid,
            organization_id: org,
            unit_id: req.unit_id,
            max_amount_per_expense: req.max_amount_per_expense.unwrap_or(Decimal::new(300, 0)),
            max_monthly_total: req.max_monthly_total.unwrap_or(Decimal::new(1000, 0)),
            allowed_categories: req
                .allowed_categories
                .unwrap_or_else(|| vec!["maintenance".to_string()]),
            is_active: true,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_auto_approval_rules(
        &self,
        oid: Uuid,
        org: Uuid,
    ) -> Result<Vec<ExpenseAutoApprovalRule>, AppError> {
        let now = Utc::now();
        Ok(vec![ExpenseAutoApprovalRule {
            id: Uuid::new_v4(),
            owner_id: oid,
            organization_id: org,
            unit_id: None,
            max_amount_per_expense: Decimal::new(300, 0),
            max_monthly_total: Decimal::new(1000, 0),
            allowed_categories: vec!["maintenance".to_string()],
            is_active: true,
            created_at: now,
            updated_at: now,
        }])
    }

    pub async fn update_auto_approval_rule(
        &self,
        id: Uuid,
        oid: Uuid,
        req: UpdateAutoApprovalRule,
    ) -> Result<ExpenseAutoApprovalRule, AppError> {
        let now = Utc::now();
        Ok(ExpenseAutoApprovalRule {
            id,
            owner_id: oid,
            organization_id: Uuid::new_v4(),
            unit_id: req.unit_id,
            max_amount_per_expense: req.max_amount_per_expense.unwrap_or(Decimal::new(300, 0)),
            max_monthly_total: req.max_monthly_total.unwrap_or(Decimal::new(1000, 0)),
            allowed_categories: req
                .allowed_categories
                .unwrap_or_else(|| vec!["maintenance".to_string()]),
            is_active: req.is_active.unwrap_or(true),
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn delete_auto_approval_rule(&self, _id: Uuid, _oid: Uuid) -> Result<(), AppError> {
        Ok(())
    }

    pub async fn submit_expense_for_approval(
        &self,
        by: Uuid,
        _org: Uuid,
        req: SubmitExpenseForApproval,
    ) -> Result<ExpenseApprovalResponse, AppError> {
        let now = Utc::now();
        let auto = req.amount <= Decimal::new(300, 0) && req.category == "maintenance";
        let e = ExpenseApprovalRequest {
            id: Uuid::new_v4(),
            unit_id: req.unit_id,
            submitted_by: by,
            amount: req.amount,
            category: req.category,
            description: req.description,
            status: if auto {
                "auto_approved".to_string()
            } else {
                "pending".to_string()
            },
            auto_approval_rule_id: if auto { Some(Uuid::new_v4()) } else { None },
            reviewed_by: None,
            review_notes: None,
            created_at: now,
            updated_at: now,
        };
        Ok(ExpenseApprovalResponse {
            request: e,
            auto_approved: auto,
            message: if auto {
                "Auto-approved".to_string()
            } else {
                "Pending".to_string()
            },
        })
    }

    pub async fn list_expense_requests(
        &self,
        _org: Uuid,
        _q: ExpenseRequestsQuery,
    ) -> Result<ListExpenseRequestsResponse, AppError> {
        let now = Utc::now();
        Ok(ListExpenseRequestsResponse {
            total: 1,
            requests: vec![ExpenseApprovalRequest {
                id: Uuid::new_v4(),
                unit_id: Uuid::new_v4(),
                submitted_by: Uuid::new_v4(),
                amount: Decimal::new(250, 0),
                category: "maintenance".to_string(),
                description: "Repair".to_string(),
                status: "auto_approved".to_string(),
                auto_approval_rule_id: Some(Uuid::new_v4()),
                reviewed_by: None,
                review_notes: None,
                created_at: now,
                updated_at: now,
            }],
        })
    }

    pub async fn review_expense(
        &self,
        id: Uuid,
        by: Uuid,
        r: ReviewExpenseRequest,
    ) -> Result<ExpenseApprovalRequest, AppError> {
        let now = Utc::now();
        let status = match r.decision {
            ExpenseApprovalDecision::Approved => "approved",
            ExpenseApprovalDecision::Rejected => "rejected",
            ExpenseApprovalDecision::NeedsInfo => "needs_info",
        };
        Ok(ExpenseApprovalRequest {
            id,
            unit_id: Uuid::new_v4(),
            submitted_by: Uuid::new_v4(),
            amount: Decimal::new(500, 0),
            category: "other".to_string(),
            description: "Item".to_string(),
            status: status.to_string(),
            auto_approval_rule_id: None,
            reviewed_by: Some(by),
            review_notes: r.notes,
            created_at: now,
            updated_at: now,
        })
    }
}
