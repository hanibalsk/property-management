//! Portfolio Performance Analytics repository (Epic 144).
//! Provides database operations for portfolio configuration, income/expense tracking,
//! financial metrics calculation, benchmarking, and dashboard analytics.

use crate::models::portfolio_performance::*;
use crate::DbPool;
use chrono::{Datelike, NaiveDate};
use common::errors::AppError;
use rust_decimal::Decimal;
use sqlx::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct PortfolioPerformanceRepository {
    pool: DbPool,
}

impl PortfolioPerformanceRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // STORY 144.1: PORTFOLIO CONFIGURATION
    // =========================================================================

    /// Create a new performance portfolio.
    pub async fn create_portfolio(
        &self,
        org_id: Uuid,
        req: CreatePerformancePortfolio,
        created_by: Uuid,
    ) -> Result<PerformancePortfolio, AppError> {
        let portfolio = sqlx::query_as::<_, PerformancePortfolio>(
            r#"
            INSERT INTO performance_portfolios (
                organization_id, name, description, target_return_pct,
                target_exit_year, investment_strategy, currency, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, organization_id, name, description, target_return_pct,
                      target_exit_year, investment_strategy, total_invested,
                      total_current_value, total_equity, total_debt, property_count,
                      currency, is_active, created_at, updated_at, created_by
            "#,
        )
        .bind(org_id)
        .bind(&req.name)
        .bind(&req.description)
        .bind(req.target_return_pct)
        .bind(req.target_exit_year)
        .bind(&req.investment_strategy)
        .bind(&req.currency)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(portfolio)
    }

    /// Get a portfolio by ID.
    pub async fn get_portfolio(
        &self,
        id: Uuid,
        org_id: Uuid,
    ) -> Result<Option<PerformancePortfolio>, AppError> {
        let portfolio = sqlx::query_as::<_, PerformancePortfolio>(
            r#"
            SELECT id, organization_id, name, description, target_return_pct,
                   target_exit_year, investment_strategy, total_invested,
                   total_current_value, total_equity, total_debt, property_count,
                   currency, is_active, created_at, updated_at, created_by
            FROM performance_portfolios
            WHERE id = $1 AND organization_id = $2
            "#,
        )
        .bind(id)
        .bind(org_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(portfolio)
    }

    /// List all portfolios for an organization.
    pub async fn list_portfolios(
        &self,
        org_id: Uuid,
        active_only: bool,
    ) -> Result<Vec<PerformancePortfolio>, AppError> {
        let portfolios = sqlx::query_as::<_, PerformancePortfolio>(
            r#"
            SELECT id, organization_id, name, description, target_return_pct,
                   target_exit_year, investment_strategy, total_invested,
                   total_current_value, total_equity, total_debt, property_count,
                   currency, is_active, created_at, updated_at, created_by
            FROM performance_portfolios
            WHERE organization_id = $1 AND ($2 = false OR is_active = true)
            ORDER BY name
            "#,
        )
        .bind(org_id)
        .bind(active_only)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(portfolios)
    }

    /// Update a portfolio.
    pub async fn update_portfolio(
        &self,
        id: Uuid,
        org_id: Uuid,
        req: UpdatePerformancePortfolio,
    ) -> Result<Option<PerformancePortfolio>, AppError> {
        let portfolio = sqlx::query_as::<_, PerformancePortfolio>(
            r#"
            UPDATE performance_portfolios
            SET name = COALESCE($3, name),
                description = COALESCE($4, description),
                target_return_pct = COALESCE($5, target_return_pct),
                target_exit_year = COALESCE($6, target_exit_year),
                investment_strategy = COALESCE($7, investment_strategy),
                is_active = COALESCE($8, is_active),
                updated_at = NOW()
            WHERE id = $1 AND organization_id = $2
            RETURNING id, organization_id, name, description, target_return_pct,
                      target_exit_year, investment_strategy, total_invested,
                      total_current_value, total_equity, total_debt, property_count,
                      currency, is_active, created_at, updated_at, created_by
            "#,
        )
        .bind(id)
        .bind(org_id)
        .bind(&req.name)
        .bind(&req.description)
        .bind(req.target_return_pct)
        .bind(req.target_exit_year)
        .bind(&req.investment_strategy)
        .bind(req.is_active)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(portfolio)
    }

    /// Delete a portfolio.
    pub async fn delete_portfolio(&self, id: Uuid, org_id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query(
            "DELETE FROM performance_portfolios WHERE id = $1 AND organization_id = $2",
        )
        .bind(id)
        .bind(org_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    /// Add a property to a portfolio.
    pub async fn add_property(
        &self,
        portfolio_id: Uuid,
        req: CreatePortfolioProperty,
    ) -> Result<PortfolioProperty, AppError> {
        let total_acquisition =
            req.acquisition_price + req.acquisition_costs.unwrap_or(Decimal::ZERO);

        // Calculate loan maturity date if loan details provided
        let loan_maturity = req.loan_start_date.map(|start| {
            let years = req.loan_term_years.unwrap_or(30);
            NaiveDate::from_ymd_opt(start.year() + years, start.month(), start.day())
                .unwrap_or(start)
        });

        // Calculate current equity
        let current_value = req.current_value.unwrap_or(req.acquisition_price);
        let current_loan = req.loan_amount.unwrap_or(Decimal::ZERO);
        let current_equity = current_value - current_loan;

        let property = sqlx::query_as::<_, PortfolioProperty>(
            r#"
            INSERT INTO portfolio_properties_perf (
                portfolio_id, building_id, property_name,
                acquisition_date, acquisition_price, acquisition_costs, total_acquisition_cost,
                financing_type, down_payment, loan_amount, interest_rate, loan_term_years,
                monthly_payment, loan_start_date, loan_maturity_date,
                ownership_percentage, current_value, current_loan_balance, current_equity,
                currency, notes
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8::financing_type, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)
            RETURNING id, portfolio_id, building_id, property_name,
                      acquisition_date, acquisition_price, acquisition_costs, total_acquisition_cost,
                      financing_type, down_payment, loan_amount, interest_rate, loan_term_years,
                      monthly_payment, loan_start_date, loan_maturity_date,
                      ownership_percentage, current_value, current_loan_balance, current_equity,
                      currency, notes, created_at, updated_at
            "#,
        )
        .bind(portfolio_id)
        .bind(req.building_id)
        .bind(&req.property_name)
        .bind(req.acquisition_date)
        .bind(req.acquisition_price)
        .bind(req.acquisition_costs)
        .bind(total_acquisition)
        .bind(&req.financing_type)
        .bind(req.down_payment)
        .bind(req.loan_amount)
        .bind(req.interest_rate)
        .bind(req.loan_term_years)
        .bind(req.monthly_payment)
        .bind(req.loan_start_date)
        .bind(loan_maturity)
        .bind(req.ownership_percentage)
        .bind(current_value)
        .bind(current_loan)
        .bind(current_equity)
        .bind(&req.currency)
        .bind(&req.notes)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        // Update portfolio totals
        self.update_portfolio_totals(portfolio_id).await?;

        Ok(property)
    }

    /// List properties in a portfolio.
    pub async fn list_properties(
        &self,
        portfolio_id: Uuid,
    ) -> Result<Vec<PortfolioProperty>, AppError> {
        let properties = sqlx::query_as::<_, PortfolioProperty>(
            r#"
            SELECT id, portfolio_id, building_id, property_name,
                   acquisition_date, acquisition_price, acquisition_costs, total_acquisition_cost,
                   financing_type, down_payment, loan_amount, interest_rate, loan_term_years,
                   monthly_payment, loan_start_date, loan_maturity_date,
                   ownership_percentage, current_value, current_loan_balance, current_equity,
                   currency, notes, created_at, updated_at
            FROM portfolio_properties_perf
            WHERE portfolio_id = $1
            ORDER BY acquisition_date DESC
            "#,
        )
        .bind(portfolio_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(properties)
    }

    /// Get a property by ID.
    pub async fn get_property(
        &self,
        id: Uuid,
        portfolio_id: Uuid,
    ) -> Result<Option<PortfolioProperty>, AppError> {
        let property = sqlx::query_as::<_, PortfolioProperty>(
            r#"
            SELECT id, portfolio_id, building_id, property_name,
                   acquisition_date, acquisition_price, acquisition_costs, total_acquisition_cost,
                   financing_type, down_payment, loan_amount, interest_rate, loan_term_years,
                   monthly_payment, loan_start_date, loan_maturity_date,
                   ownership_percentage, current_value, current_loan_balance, current_equity,
                   currency, notes, created_at, updated_at
            FROM portfolio_properties_perf
            WHERE id = $1 AND portfolio_id = $2
            "#,
        )
        .bind(id)
        .bind(portfolio_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(property)
    }

    /// Update a property.
    pub async fn update_property(
        &self,
        id: Uuid,
        portfolio_id: Uuid,
        req: UpdatePortfolioProperty,
    ) -> Result<Option<PortfolioProperty>, AppError> {
        let property = sqlx::query_as::<_, PortfolioProperty>(
            r#"
            UPDATE portfolio_properties_perf
            SET property_name = COALESCE($3, property_name),
                financing_type = COALESCE($4::financing_type, financing_type),
                loan_amount = COALESCE($5, loan_amount),
                interest_rate = COALESCE($6, interest_rate),
                monthly_payment = COALESCE($7, monthly_payment),
                current_value = COALESCE($8, current_value),
                current_loan_balance = COALESCE($9, current_loan_balance),
                current_equity = COALESCE($8, current_value) - COALESCE($9, current_loan_balance, loan_amount, 0),
                ownership_percentage = COALESCE($10, ownership_percentage),
                notes = COALESCE($11, notes),
                updated_at = NOW()
            WHERE id = $1 AND portfolio_id = $2
            RETURNING id, portfolio_id, building_id, property_name,
                      acquisition_date, acquisition_price, acquisition_costs, total_acquisition_cost,
                      financing_type, down_payment, loan_amount, interest_rate, loan_term_years,
                      monthly_payment, loan_start_date, loan_maturity_date,
                      ownership_percentage, current_value, current_loan_balance, current_equity,
                      currency, notes, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(portfolio_id)
        .bind(&req.property_name)
        .bind(&req.financing_type)
        .bind(req.loan_amount)
        .bind(req.interest_rate)
        .bind(req.monthly_payment)
        .bind(req.current_value)
        .bind(req.current_loan_balance)
        .bind(req.ownership_percentage)
        .bind(&req.notes)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if property.is_some() {
            self.update_portfolio_totals(portfolio_id).await?;
        }

        Ok(property)
    }

    /// Remove a property from a portfolio.
    pub async fn remove_property(&self, id: Uuid, portfolio_id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query(
            "DELETE FROM portfolio_properties_perf WHERE id = $1 AND portfolio_id = $2",
        )
        .bind(id)
        .bind(portfolio_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() > 0 {
            self.update_portfolio_totals(portfolio_id).await?;
        }

        Ok(result.rows_affected() > 0)
    }

    /// Update portfolio summary totals.
    async fn update_portfolio_totals(&self, portfolio_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE performance_portfolios p
            SET total_invested = (
                    SELECT COALESCE(SUM(total_acquisition_cost), 0)
                    FROM portfolio_properties_perf WHERE portfolio_id = p.id
                ),
                total_current_value = (
                    SELECT COALESCE(SUM(current_value), 0)
                    FROM portfolio_properties_perf WHERE portfolio_id = p.id
                ),
                total_equity = (
                    SELECT COALESCE(SUM(current_equity), 0)
                    FROM portfolio_properties_perf WHERE portfolio_id = p.id
                ),
                total_debt = (
                    SELECT COALESCE(SUM(current_loan_balance), 0)
                    FROM portfolio_properties_perf WHERE portfolio_id = p.id
                ),
                property_count = (
                    SELECT COUNT(*)::INTEGER
                    FROM portfolio_properties_perf WHERE portfolio_id = p.id
                ),
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(portfolio_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    // =========================================================================
    // STORY 144.2: INCOME & EXPENSE TRACKING
    // =========================================================================

    /// Create a transaction.
    pub async fn create_transaction(
        &self,
        portfolio_id: Uuid,
        req: CreatePropertyTransaction,
        created_by: Uuid,
    ) -> Result<PropertyTransaction, AppError> {
        let transaction = sqlx::query_as::<_, PropertyTransaction>(
            r#"
            INSERT INTO property_transactions (
                portfolio_id, property_id, transaction_type, category,
                amount, currency, transaction_date, period_start, period_end,
                description, vendor_name, reference_number, document_id,
                is_recurring, recurrence_frequency, created_by
            )
            VALUES ($1, $2, $3::transaction_type_portfolio, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING id, portfolio_id, property_id, transaction_type, category,
                      amount, currency, transaction_date, period_start, period_end,
                      description, vendor_name, reference_number, document_id,
                      is_recurring, recurrence_frequency, created_at, updated_at, created_by
            "#,
        )
        .bind(portfolio_id)
        .bind(req.property_id)
        .bind(&req.transaction_type)
        .bind(&req.category)
        .bind(req.amount)
        .bind(&req.currency)
        .bind(req.transaction_date)
        .bind(req.period_start)
        .bind(req.period_end)
        .bind(&req.description)
        .bind(&req.vendor_name)
        .bind(&req.reference_number)
        .bind(req.document_id)
        .bind(req.is_recurring)
        .bind(&req.recurrence_frequency)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(transaction)
    }

    /// List transactions.
    pub async fn list_transactions(
        &self,
        portfolio_id: Uuid,
        query: TransactionQuery,
    ) -> Result<Vec<PropertyTransaction>, AppError> {
        let transactions = sqlx::query_as::<_, PropertyTransaction>(
            r#"
            SELECT id, portfolio_id, property_id, transaction_type, category,
                   amount, currency, transaction_date, period_start, period_end,
                   description, vendor_name, reference_number, document_id,
                   is_recurring, recurrence_frequency, created_at, updated_at, created_by
            FROM property_transactions
            WHERE portfolio_id = $1
              AND ($2::UUID IS NULL OR property_id = $2)
              AND ($3::transaction_type_portfolio IS NULL OR transaction_type = $3)
              AND ($4::DATE IS NULL OR transaction_date >= $4)
              AND ($5::DATE IS NULL OR transaction_date <= $5)
              AND ($6::TEXT IS NULL OR category = $6)
            ORDER BY transaction_date DESC
            LIMIT 500
            "#,
        )
        .bind(portfolio_id)
        .bind(query.property_id)
        .bind(&query.transaction_type)
        .bind(query.start_date)
        .bind(query.end_date)
        .bind(&query.category)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(transactions)
    }

    /// Get a transaction by ID.
    pub async fn get_transaction(
        &self,
        id: Uuid,
        portfolio_id: Uuid,
    ) -> Result<Option<PropertyTransaction>, AppError> {
        let transaction = sqlx::query_as::<_, PropertyTransaction>(
            r#"
            SELECT id, portfolio_id, property_id, transaction_type, category,
                   amount, currency, transaction_date, period_start, period_end,
                   description, vendor_name, reference_number, document_id,
                   is_recurring, recurrence_frequency, created_at, updated_at, created_by
            FROM property_transactions
            WHERE id = $1 AND portfolio_id = $2
            "#,
        )
        .bind(id)
        .bind(portfolio_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(transaction)
    }

    /// Update a transaction.
    pub async fn update_transaction(
        &self,
        id: Uuid,
        portfolio_id: Uuid,
        req: UpdatePropertyTransaction,
    ) -> Result<Option<PropertyTransaction>, AppError> {
        let transaction = sqlx::query_as::<_, PropertyTransaction>(
            r#"
            UPDATE property_transactions
            SET transaction_type = COALESCE($3::transaction_type_portfolio, transaction_type),
                category = COALESCE($4, category),
                amount = COALESCE($5, amount),
                transaction_date = COALESCE($6, transaction_date),
                description = COALESCE($7, description),
                vendor_name = COALESCE($8, vendor_name),
                reference_number = COALESCE($9, reference_number),
                updated_at = NOW()
            WHERE id = $1 AND portfolio_id = $2
            RETURNING id, portfolio_id, property_id, transaction_type, category,
                      amount, currency, transaction_date, period_start, period_end,
                      description, vendor_name, reference_number, document_id,
                      is_recurring, recurrence_frequency, created_at, updated_at, created_by
            "#,
        )
        .bind(id)
        .bind(portfolio_id)
        .bind(&req.transaction_type)
        .bind(&req.category)
        .bind(req.amount)
        .bind(req.transaction_date)
        .bind(&req.description)
        .bind(&req.vendor_name)
        .bind(&req.reference_number)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(transaction)
    }

    /// Delete a transaction.
    pub async fn delete_transaction(&self, id: Uuid, portfolio_id: Uuid) -> Result<bool, AppError> {
        let result =
            sqlx::query("DELETE FROM property_transactions WHERE id = $1 AND portfolio_id = $2")
                .bind(id)
                .bind(portfolio_id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    /// Upsert cash flow for a property/period.
    pub async fn upsert_cash_flow(
        &self,
        portfolio_id: Uuid,
        req: UpsertPropertyCashFlow,
    ) -> Result<PropertyCashFlow, AppError> {
        let other_income = req.other_income.unwrap_or(Decimal::ZERO);
        let total_income = req.gross_rental_income + other_income;
        let mortgage_payment = req.mortgage_payment.unwrap_or(Decimal::ZERO);
        let capital_expenditures = req.capital_expenditures.unwrap_or(Decimal::ZERO);
        let total_expenses = req.operating_expenses + mortgage_payment + capital_expenditures;
        let noi = total_income - req.operating_expenses;
        let cash_flow_before_debt = noi;
        let cash_flow_after_debt = noi - mortgage_payment;

        let cash_flow = sqlx::query_as::<_, PropertyCashFlow>(
            r#"
            INSERT INTO property_cash_flows (
                portfolio_id, property_id, period_year, period_month,
                gross_rental_income, other_income, total_income,
                operating_expenses, mortgage_payment, capital_expenditures, total_expenses,
                net_operating_income, cash_flow_before_debt, cash_flow_after_debt,
                vacancy_rate, vacancy_cost, currency
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            ON CONFLICT (portfolio_id, property_id, period_year, period_month)
            DO UPDATE SET
                gross_rental_income = EXCLUDED.gross_rental_income,
                other_income = EXCLUDED.other_income,
                total_income = EXCLUDED.total_income,
                operating_expenses = EXCLUDED.operating_expenses,
                mortgage_payment = EXCLUDED.mortgage_payment,
                capital_expenditures = EXCLUDED.capital_expenditures,
                total_expenses = EXCLUDED.total_expenses,
                net_operating_income = EXCLUDED.net_operating_income,
                cash_flow_before_debt = EXCLUDED.cash_flow_before_debt,
                cash_flow_after_debt = EXCLUDED.cash_flow_after_debt,
                vacancy_rate = EXCLUDED.vacancy_rate,
                vacancy_cost = EXCLUDED.vacancy_cost,
                updated_at = NOW()
            RETURNING id, portfolio_id, property_id, period_year, period_month,
                      gross_rental_income, other_income, total_income,
                      operating_expenses, mortgage_payment, capital_expenditures, total_expenses,
                      net_operating_income, cash_flow_before_debt, cash_flow_after_debt,
                      vacancy_rate, vacancy_cost, currency, created_at, updated_at
            "#,
        )
        .bind(portfolio_id)
        .bind(req.property_id)
        .bind(req.period_year)
        .bind(req.period_month)
        .bind(req.gross_rental_income)
        .bind(other_income)
        .bind(total_income)
        .bind(req.operating_expenses)
        .bind(mortgage_payment)
        .bind(capital_expenditures)
        .bind(total_expenses)
        .bind(noi)
        .bind(cash_flow_before_debt)
        .bind(cash_flow_after_debt)
        .bind(req.vacancy_rate)
        .bind(req.vacancy_cost)
        .bind(&req.currency)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(cash_flow)
    }

    /// Get cash flows for a property.
    pub async fn get_cash_flows(
        &self,
        portfolio_id: Uuid,
        property_id: Option<Uuid>,
        start_year: Option<i32>,
        start_month: Option<i32>,
        end_year: Option<i32>,
        end_month: Option<i32>,
    ) -> Result<Vec<PropertyCashFlow>, AppError> {
        let cash_flows = sqlx::query_as::<_, PropertyCashFlow>(
            r#"
            SELECT id, portfolio_id, property_id, period_year, period_month,
                   gross_rental_income, other_income, total_income,
                   operating_expenses, mortgage_payment, capital_expenditures, total_expenses,
                   net_operating_income, cash_flow_before_debt, cash_flow_after_debt,
                   vacancy_rate, vacancy_cost, currency, created_at, updated_at
            FROM property_cash_flows
            WHERE portfolio_id = $1
              AND ($2::UUID IS NULL OR property_id = $2)
              AND ($3::INTEGER IS NULL OR period_year > $3 OR (period_year = $3 AND period_month >= $4))
              AND ($5::INTEGER IS NULL OR period_year < $5 OR (period_year = $5 AND period_month <= $6))
            ORDER BY period_year DESC, period_month DESC
            "#,
        )
        .bind(portfolio_id)
        .bind(property_id)
        .bind(start_year)
        .bind(start_month.unwrap_or(1))
        .bind(end_year)
        .bind(end_month.unwrap_or(12))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(cash_flows)
    }

    // =========================================================================
    // STORY 144.3: ROI & FINANCIAL METRICS CALCULATOR
    // =========================================================================

    /// Calculate and store financial metrics for a property or portfolio.
    pub async fn calculate_metrics(
        &self,
        portfolio_id: Uuid,
        req: CalculateMetricsRequest,
    ) -> Result<FinancialMetrics, AppError> {
        // Get income/expense data from cash flows
        let (gross_income, other_income, operating_expenses, total_debt_service, vacancy_loss) =
            self.get_income_expenses_for_period(
                portfolio_id,
                req.property_id,
                req.period_start,
                req.period_end,
            )
            .await?;

        let effective_gross_income = gross_income + other_income.unwrap_or(Decimal::ZERO)
            - vacancy_loss.unwrap_or(Decimal::ZERO);
        let noi = effective_gross_income - operating_expenses;

        // Get property/portfolio values
        let (property_value, total_investment, total_equity, annual_debt_service) = self
            .get_investment_values(portfolio_id, req.property_id, &req)
            .await?;

        // Calculate metrics
        let cap_rate = if property_value > Decimal::ZERO {
            Some((noi / property_value) * Decimal::from(100))
        } else {
            None
        };

        let annual_cash_flow = noi - annual_debt_service.unwrap_or(Decimal::ZERO);
        let cash_on_cash = if total_equity > Decimal::ZERO {
            Some((annual_cash_flow / total_equity) * Decimal::from(100))
        } else {
            None
        };

        let dscr = if annual_debt_service.is_some() && annual_debt_service.unwrap() > Decimal::ZERO
        {
            Some(noi / annual_debt_service.unwrap())
        } else {
            None
        };

        let grm = if gross_income > Decimal::ZERO {
            Some(property_value / gross_income)
        } else {
            None
        };

        // Equity multiple and IRR require historical data - placeholder for now
        let equity_multiple = if total_investment > Decimal::ZERO {
            Some((property_value + annual_cash_flow) / total_investment)
        } else {
            None
        };

        let metrics = sqlx::query_as::<_, FinancialMetrics>(
            r#"
            INSERT INTO financial_metrics (
                portfolio_id, property_id, period_type, period_start, period_end,
                gross_income, effective_gross_income, vacancy_loss, other_income,
                operating_expenses, total_debt_service, net_operating_income,
                cap_rate, cash_on_cash_return, gross_rent_multiplier,
                irr, equity_multiple, dscr, npv,
                property_value, total_investment, total_equity, annual_debt_service,
                currency
            )
            VALUES ($1, $2, $3::metric_period, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24)
            ON CONFLICT (portfolio_id, property_id, period_type, period_start, period_end)
            DO UPDATE SET
                gross_income = EXCLUDED.gross_income,
                effective_gross_income = EXCLUDED.effective_gross_income,
                vacancy_loss = EXCLUDED.vacancy_loss,
                operating_expenses = EXCLUDED.operating_expenses,
                total_debt_service = EXCLUDED.total_debt_service,
                net_operating_income = EXCLUDED.net_operating_income,
                cap_rate = EXCLUDED.cap_rate,
                cash_on_cash_return = EXCLUDED.cash_on_cash_return,
                gross_rent_multiplier = EXCLUDED.gross_rent_multiplier,
                equity_multiple = EXCLUDED.equity_multiple,
                dscr = EXCLUDED.dscr,
                property_value = EXCLUDED.property_value,
                total_investment = EXCLUDED.total_investment,
                total_equity = EXCLUDED.total_equity,
                annual_debt_service = EXCLUDED.annual_debt_service,
                calculated_at = NOW()
            RETURNING id, portfolio_id, property_id, period_type, period_start, period_end,
                      gross_income, effective_gross_income, vacancy_loss, other_income,
                      operating_expenses, total_debt_service, net_operating_income,
                      cap_rate, cash_on_cash_return, gross_rent_multiplier,
                      irr, npv, equity_multiple, dscr,
                      property_value, total_investment, total_equity, annual_debt_service,
                      currency, notes, calculated_at, created_at
            "#,
        )
        .bind(portfolio_id)
        .bind(req.property_id)
        .bind(&req.period_type)
        .bind(req.period_start)
        .bind(req.period_end)
        .bind(gross_income)
        .bind(effective_gross_income)
        .bind(vacancy_loss)
        .bind(other_income)
        .bind(operating_expenses)
        .bind(total_debt_service)
        .bind(noi)
        .bind(cap_rate)
        .bind(cash_on_cash)
        .bind(grm)
        .bind(None::<Decimal>) // IRR
        .bind(equity_multiple)
        .bind(dscr)
        .bind(None::<Decimal>) // NPV
        .bind(property_value)
        .bind(total_investment)
        .bind(total_equity)
        .bind(annual_debt_service)
        .bind("EUR")
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(metrics)
    }

    /// Get income/expenses for a period.
    async fn get_income_expenses_for_period(
        &self,
        portfolio_id: Uuid,
        property_id: Option<Uuid>,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<
        (
            Decimal,
            Option<Decimal>,
            Decimal,
            Option<Decimal>,
            Option<Decimal>,
        ),
        AppError,
    > {
        let row = sqlx::query(
            r#"
            SELECT
                COALESCE(SUM(gross_rental_income), 0) as gross_income,
                SUM(other_income) as other_income,
                COALESCE(SUM(operating_expenses), 0) as operating_expenses,
                SUM(mortgage_payment) as debt_service,
                SUM(vacancy_cost) as vacancy_loss
            FROM property_cash_flows
            WHERE portfolio_id = $1
              AND ($2::UUID IS NULL OR property_id = $2)
              AND (period_year * 100 + period_month) >= ($3 * 100 + $4)
              AND (period_year * 100 + period_month) <= ($5 * 100 + $6)
            "#,
        )
        .bind(portfolio_id)
        .bind(property_id)
        .bind(start_date.year())
        .bind(start_date.month() as i32)
        .bind(end_date.year())
        .bind(end_date.month() as i32)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok((
            row.get("gross_income"),
            row.get("other_income"),
            row.get("operating_expenses"),
            row.get("debt_service"),
            row.get("vacancy_loss"),
        ))
    }

    /// Get investment values.
    async fn get_investment_values(
        &self,
        portfolio_id: Uuid,
        property_id: Option<Uuid>,
        req: &CalculateMetricsRequest,
    ) -> Result<(Decimal, Decimal, Decimal, Option<Decimal>), AppError> {
        if property_id.is_some() {
            let row = sqlx::query(
                r#"
                SELECT
                    COALESCE(current_value, 0) as property_value,
                    COALESCE(total_acquisition_cost, acquisition_price) as total_investment,
                    COALESCE(current_equity, 0) as total_equity,
                    (COALESCE(monthly_payment, 0) * 12) as annual_debt_service
                FROM portfolio_properties_perf
                WHERE portfolio_id = $1 AND id = $2
                "#,
            )
            .bind(portfolio_id)
            .bind(property_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

            Ok((
                req.property_value
                    .unwrap_or_else(|| row.get("property_value")),
                req.total_investment
                    .unwrap_or_else(|| row.get("total_investment")),
                row.get("total_equity"),
                row.get("annual_debt_service"),
            ))
        } else {
            let row = sqlx::query(
                r#"
                SELECT
                    COALESCE(SUM(current_value), 0) as property_value,
                    COALESCE(SUM(total_acquisition_cost), 0) as total_investment,
                    COALESCE(SUM(current_equity), 0) as total_equity,
                    SUM(COALESCE(monthly_payment, 0) * 12) as annual_debt_service
                FROM portfolio_properties_perf
                WHERE portfolio_id = $1
                "#,
            )
            .bind(portfolio_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

            Ok((
                req.property_value
                    .unwrap_or_else(|| row.get("property_value")),
                req.total_investment
                    .unwrap_or_else(|| row.get("total_investment")),
                row.get("total_equity"),
                row.get("annual_debt_service"),
            ))
        }
    }

    /// Get latest metrics for a property or portfolio.
    pub async fn get_latest_metrics(
        &self,
        portfolio_id: Uuid,
        property_id: Option<Uuid>,
    ) -> Result<Option<FinancialMetrics>, AppError> {
        let metrics = sqlx::query_as::<_, FinancialMetrics>(
            r#"
            SELECT id, portfolio_id, property_id, period_type, period_start, period_end,
                   gross_income, effective_gross_income, vacancy_loss, other_income,
                   operating_expenses, total_debt_service, net_operating_income,
                   cap_rate, cash_on_cash_return, gross_rent_multiplier,
                   irr, npv, equity_multiple, dscr,
                   property_value, total_investment, total_equity, annual_debt_service,
                   currency, notes, calculated_at, created_at
            FROM financial_metrics
            WHERE portfolio_id = $1
              AND ($2::UUID IS NULL OR property_id = $2)
            ORDER BY period_end DESC
            LIMIT 1
            "#,
        )
        .bind(portfolio_id)
        .bind(property_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(metrics)
    }

    /// Get portfolio metrics summary.
    pub async fn get_portfolio_metrics_summary(
        &self,
        portfolio_id: Uuid,
        period_start: NaiveDate,
        period_end: NaiveDate,
    ) -> Result<PortfolioMetricsSummary, AppError> {
        let portfolio = self
            .get_portfolio(portfolio_id, Uuid::nil())
            .await?
            .ok_or_else(|| AppError::NotFound("Portfolio not found".into()))?;

        // Get property-level metrics
        let property_metrics = sqlx::query_as::<
            _,
            (
                Uuid,
                Option<String>,
                Decimal,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
            ),
        >(
            r#"
            SELECT pp.id, pp.property_name,
                   COALESCE(fm.net_operating_income, 0) as noi,
                   fm.cap_rate, fm.cash_on_cash_return, fm.irr, fm.dscr, fm.equity_multiple
            FROM portfolio_properties_perf pp
            LEFT JOIN financial_metrics fm ON fm.property_id = pp.id
                AND fm.period_start = $2 AND fm.period_end = $3
            WHERE pp.portfolio_id = $1
            "#,
        )
        .bind(portfolio_id)
        .bind(period_start)
        .bind(period_end)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        let mut props: Vec<MetricsSummary> = vec![];
        let mut total_noi = Decimal::ZERO;

        for (id, name, noi, cap_rate, coc, irr, dscr, em) in property_metrics {
            total_noi += noi;
            props.push(MetricsSummary {
                property_id: Some(id),
                property_name: name,
                noi,
                cap_rate,
                cash_on_cash: coc,
                irr,
                dscr,
                equity_multiple: em,
                period: format!("{} to {}", period_start, period_end),
                currency: portfolio.currency.clone(),
            });
        }

        // Calculate portfolio-level metrics
        let total_value = portfolio.total_current_value.unwrap_or(Decimal::ZERO);
        let total_equity = portfolio.total_equity.unwrap_or(Decimal::ZERO);
        let total_debt = portfolio.total_debt.unwrap_or(Decimal::ZERO);

        let weighted_cap_rate = if total_value > Decimal::ZERO {
            Some((total_noi / total_value) * Decimal::from(100))
        } else {
            None
        };

        let leverage_ratio = if total_value > Decimal::ZERO {
            Some((total_debt / total_value) * Decimal::from(100))
        } else {
            None
        };

        Ok(PortfolioMetricsSummary {
            portfolio_id,
            portfolio_name: portfolio.name,
            period_start,
            period_end,
            total_noi,
            weighted_avg_cap_rate: weighted_cap_rate,
            portfolio_cash_on_cash: None,
            portfolio_irr: None,
            portfolio_dscr: None,
            portfolio_equity_multiple: None,
            total_value,
            total_equity,
            total_debt,
            leverage_ratio,
            property_count: portfolio.property_count.unwrap_or(0),
            currency: portfolio.currency,
            property_metrics: props,
        })
    }

    // =========================================================================
    // STORY 144.4: PERFORMANCE BENCHMARKING
    // =========================================================================

    /// Create a market benchmark.
    pub async fn create_benchmark(
        &self,
        org_id: Uuid,
        req: CreateMarketBenchmark,
    ) -> Result<MarketBenchmark, AppError> {
        let benchmark = sqlx::query_as::<_, MarketBenchmark>(
            r#"
            INSERT INTO market_benchmarks (
                organization_id, name, description, source, source_name, source_url, source_date,
                property_type, region, market, period_year, period_quarter,
                avg_cap_rate, avg_cash_on_cash, avg_noi_per_unit, avg_price_per_unit,
                avg_price_per_sqm, avg_occupancy, avg_rent_growth, avg_expense_ratio,
                avg_irr, avg_equity_multiple, currency
            )
            VALUES ($1, $2, $3, $4::benchmark_source, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)
            RETURNING id, organization_id, name, description, source, source_name, source_url, source_date,
                      property_type, region, market, period_year, period_quarter,
                      avg_cap_rate, avg_cash_on_cash, avg_noi_per_unit, avg_price_per_unit,
                      avg_price_per_sqm, avg_occupancy, avg_rent_growth, avg_expense_ratio,
                      avg_irr, avg_equity_multiple, currency, is_active, created_at, updated_at
            "#,
        )
        .bind(org_id)
        .bind(&req.name)
        .bind(&req.description)
        .bind(&req.source)
        .bind(&req.source_name)
        .bind(&req.source_url)
        .bind(req.source_date)
        .bind(&req.property_type)
        .bind(&req.region)
        .bind(&req.market)
        .bind(req.period_year)
        .bind(req.period_quarter)
        .bind(req.avg_cap_rate)
        .bind(req.avg_cash_on_cash)
        .bind(req.avg_noi_per_unit)
        .bind(req.avg_price_per_unit)
        .bind(req.avg_price_per_sqm)
        .bind(req.avg_occupancy)
        .bind(req.avg_rent_growth)
        .bind(req.avg_expense_ratio)
        .bind(req.avg_irr)
        .bind(req.avg_equity_multiple)
        .bind(&req.currency)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(benchmark)
    }

    /// Get a benchmark by ID.
    pub async fn get_benchmark(
        &self,
        id: Uuid,
        org_id: Uuid,
    ) -> Result<Option<MarketBenchmark>, AppError> {
        let benchmark = sqlx::query_as::<_, MarketBenchmark>(
            r#"
            SELECT id, organization_id, name, description, source, source_name, source_url, source_date,
                   property_type, region, market, period_year, period_quarter,
                   avg_cap_rate, avg_cash_on_cash, avg_noi_per_unit, avg_price_per_unit,
                   avg_price_per_sqm, avg_occupancy, avg_rent_growth, avg_expense_ratio,
                   avg_irr, avg_equity_multiple, currency, is_active, created_at, updated_at
            FROM market_benchmarks
            WHERE id = $1 AND organization_id = $2
            "#,
        )
        .bind(id)
        .bind(org_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(benchmark)
    }

    /// List benchmarks.
    pub async fn list_benchmarks(
        &self,
        org_id: Uuid,
        active_only: bool,
    ) -> Result<Vec<MarketBenchmark>, AppError> {
        let benchmarks = sqlx::query_as::<_, MarketBenchmark>(
            r#"
            SELECT id, organization_id, name, description, source, source_name, source_url, source_date,
                   property_type, region, market, period_year, period_quarter,
                   avg_cap_rate, avg_cash_on_cash, avg_noi_per_unit, avg_price_per_unit,
                   avg_price_per_sqm, avg_occupancy, avg_rent_growth, avg_expense_ratio,
                   avg_irr, avg_equity_multiple, currency, is_active, created_at, updated_at
            FROM market_benchmarks
            WHERE organization_id = $1 AND ($2 = false OR is_active = true)
            ORDER BY period_year DESC, name
            "#,
        )
        .bind(org_id)
        .bind(active_only)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(benchmarks)
    }

    /// Update a benchmark.
    pub async fn update_benchmark(
        &self,
        id: Uuid,
        org_id: Uuid,
        req: UpdateMarketBenchmark,
    ) -> Result<Option<MarketBenchmark>, AppError> {
        let benchmark = sqlx::query_as::<_, MarketBenchmark>(
            r#"
            UPDATE market_benchmarks
            SET name = COALESCE($3, name),
                description = COALESCE($4, description),
                source_date = COALESCE($5, source_date),
                avg_cap_rate = COALESCE($6, avg_cap_rate),
                avg_cash_on_cash = COALESCE($7, avg_cash_on_cash),
                avg_noi_per_unit = COALESCE($8, avg_noi_per_unit),
                avg_occupancy = COALESCE($9, avg_occupancy),
                avg_irr = COALESCE($10, avg_irr),
                is_active = COALESCE($11, is_active),
                updated_at = NOW()
            WHERE id = $1 AND organization_id = $2
            RETURNING id, organization_id, name, description, source, source_name, source_url, source_date,
                      property_type, region, market, period_year, period_quarter,
                      avg_cap_rate, avg_cash_on_cash, avg_noi_per_unit, avg_price_per_unit,
                      avg_price_per_sqm, avg_occupancy, avg_rent_growth, avg_expense_ratio,
                      avg_irr, avg_equity_multiple, currency, is_active, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(org_id)
        .bind(&req.name)
        .bind(&req.description)
        .bind(req.source_date)
        .bind(req.avg_cap_rate)
        .bind(req.avg_cash_on_cash)
        .bind(req.avg_noi_per_unit)
        .bind(req.avg_occupancy)
        .bind(req.avg_irr)
        .bind(req.is_active)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(benchmark)
    }

    /// Delete a benchmark.
    pub async fn delete_benchmark(&self, id: Uuid, org_id: Uuid) -> Result<bool, AppError> {
        let result =
            sqlx::query("DELETE FROM market_benchmarks WHERE id = $1 AND organization_id = $2")
                .bind(id)
                .bind(org_id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    /// Create a benchmark comparison.
    pub async fn create_comparison(
        &self,
        portfolio_id: Uuid,
        req: CreateBenchmarkComparison,
    ) -> Result<BenchmarkComparison, AppError> {
        // Get actual values from metrics
        let metrics = self
            .get_latest_metrics(portfolio_id, req.property_id)
            .await?;

        let (actual_cap_rate, actual_coc, actual_noi, actual_irr, actual_em) = metrics
            .map(|m| {
                (
                    m.cap_rate,
                    m.cash_on_cash_return,
                    Some(m.net_operating_income),
                    m.irr,
                    m.equity_multiple,
                )
            })
            .unwrap_or((None, None, None, None, None));

        // Calculate variances (would need benchmark values)
        let comparison = sqlx::query_as::<_, BenchmarkComparison>(
            r#"
            INSERT INTO benchmark_comparisons (
                portfolio_id, benchmark_id, property_id, comparison_date,
                actual_cap_rate, actual_cash_on_cash, actual_noi_per_unit,
                actual_occupancy, actual_irr, actual_equity_multiple
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, portfolio_id, benchmark_id, property_id, comparison_date,
                      actual_cap_rate, actual_cash_on_cash, actual_noi_per_unit,
                      actual_occupancy, actual_irr, actual_equity_multiple,
                      cap_rate_variance, cash_on_cash_variance, noi_variance_pct,
                      occupancy_variance, irr_variance,
                      cap_rate_percentile, cash_on_cash_percentile, overall_percentile,
                      performance_score, performance_rating, summary, created_at
            "#,
        )
        .bind(portfolio_id)
        .bind(req.benchmark_id)
        .bind(req.property_id)
        .bind(req.comparison_date)
        .bind(actual_cap_rate)
        .bind(actual_coc)
        .bind(actual_noi)
        .bind(None::<Decimal>) // actual_occupancy
        .bind(actual_irr)
        .bind(actual_em)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(comparison)
    }

    /// Get comparison by ID.
    pub async fn get_comparison(
        &self,
        id: Uuid,
        portfolio_id: Uuid,
    ) -> Result<Option<BenchmarkComparison>, AppError> {
        let comparison = sqlx::query_as::<_, BenchmarkComparison>(
            r#"
            SELECT id, portfolio_id, benchmark_id, property_id, comparison_date,
                   actual_cap_rate, actual_cash_on_cash, actual_noi_per_unit,
                   actual_occupancy, actual_irr, actual_equity_multiple,
                   cap_rate_variance, cash_on_cash_variance, noi_variance_pct,
                   occupancy_variance, irr_variance,
                   cap_rate_percentile, cash_on_cash_percentile, overall_percentile,
                   performance_score, performance_rating, summary, created_at
            FROM benchmark_comparisons
            WHERE id = $1 AND portfolio_id = $2
            "#,
        )
        .bind(id)
        .bind(portfolio_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(comparison)
    }

    /// List comparisons for a portfolio.
    pub async fn list_comparisons(
        &self,
        portfolio_id: Uuid,
    ) -> Result<Vec<BenchmarkComparison>, AppError> {
        let comparisons = sqlx::query_as::<_, BenchmarkComparison>(
            r#"
            SELECT id, portfolio_id, benchmark_id, property_id, comparison_date,
                   actual_cap_rate, actual_cash_on_cash, actual_noi_per_unit,
                   actual_occupancy, actual_irr, actual_equity_multiple,
                   cap_rate_variance, cash_on_cash_variance, noi_variance_pct,
                   occupancy_variance, irr_variance,
                   cap_rate_percentile, cash_on_cash_percentile, overall_percentile,
                   performance_score, performance_rating, summary, created_at
            FROM benchmark_comparisons
            WHERE portfolio_id = $1
            ORDER BY comparison_date DESC
            "#,
        )
        .bind(portfolio_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(comparisons)
    }

    // =========================================================================
    // STORY 144.5: PORTFOLIO ANALYTICS DASHBOARD
    // =========================================================================

    /// Get dashboard summary.
    pub async fn get_dashboard_summary(
        &self,
        portfolio_id: Uuid,
        as_of_date: NaiveDate,
    ) -> Result<DashboardSummary, AppError> {
        let portfolio = self
            .get_portfolio(portfolio_id, Uuid::nil())
            .await?
            .ok_or_else(|| AppError::NotFound("Portfolio not found".into()))?;

        let total_value = portfolio.total_current_value.unwrap_or(Decimal::ZERO);
        let total_equity = portfolio.total_equity.unwrap_or(Decimal::ZERO);
        let total_debt = portfolio.total_debt.unwrap_or(Decimal::ZERO);

        let debt_to_equity = if total_equity > Decimal::ZERO {
            Some(total_debt / total_equity)
        } else {
            None
        };

        let ltv_ratio = if total_value > Decimal::ZERO {
            Some((total_debt / total_value) * Decimal::from(100))
        } else {
            None
        };

        // Get YTD cash flow
        let year = as_of_date.year();
        let (ytd_noi, ytd_cash_flow) = sqlx::query_as::<_, (Decimal, Decimal)>(
            r#"
            SELECT
                COALESCE(SUM(net_operating_income), 0) as ytd_noi,
                COALESCE(SUM(cash_flow_after_debt), 0) as ytd_cash_flow
            FROM property_cash_flows
            WHERE portfolio_id = $1 AND period_year = $2
            "#,
        )
        .bind(portfolio_id)
        .bind(year)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        let ytd_return = if total_equity > Decimal::ZERO {
            Some((ytd_cash_flow / total_equity) * Decimal::from(100))
        } else {
            None
        };

        Ok(DashboardSummary {
            total_portfolio_value: total_value,
            total_equity,
            total_debt,
            debt_to_equity_ratio: debt_to_equity,
            ltv_ratio,
            ytd_noi,
            ytd_cash_flow,
            ytd_return_pct: ytd_return,
            property_count: portfolio.property_count.unwrap_or(0),
            total_units: None,
            occupied_units: None,
            occupancy_rate: None,
            currency: portfolio.currency,
            as_of_date,
        })
    }

    /// Get property performance cards.
    pub async fn get_property_performance_cards(
        &self,
        portfolio_id: Uuid,
    ) -> Result<Vec<PropertyPerformanceCard>, AppError> {
        let cards = sqlx::query_as::<_, (Uuid, Option<String>, Decimal, Decimal, Option<Decimal>, Decimal, Option<Decimal>, Option<Decimal>, Option<Decimal>, Option<Decimal>, String)>(
            r#"
            SELECT pp.id, pp.property_name,
                   COALESCE(pp.current_value, 0) as current_value,
                   COALESCE(pp.current_equity, 0) as equity,
                   CASE WHEN pp.current_value > 0
                        THEN (pp.current_loan_balance / pp.current_value) * 100
                        ELSE NULL END as ltv,
                   COALESCE(fm.net_operating_income, 0) as noi,
                   fm.cap_rate,
                   fm.cash_on_cash_return,
                   fm.dscr,
                   (SELECT cash_flow_after_debt FROM property_cash_flows pcf
                    WHERE pcf.property_id = pp.id
                    ORDER BY period_year DESC, period_month DESC LIMIT 1) as monthly_cash_flow,
                   pp.currency
            FROM portfolio_properties_perf pp
            LEFT JOIN financial_metrics fm ON fm.property_id = pp.id
                AND fm.period_end = (SELECT MAX(period_end) FROM financial_metrics WHERE property_id = pp.id)
            WHERE pp.portfolio_id = $1
            "#,
        )
        .bind(portfolio_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        let result = cards
            .into_iter()
            .map(
                |(id, name, value, equity, ltv, noi, cap_rate, coc, dscr, mcf, currency)| {
                    let status = match cap_rate {
                        Some(cr) if cr >= Decimal::from(8) => "excellent",
                        Some(cr) if cr >= Decimal::from(6) => "good",
                        Some(cr) if cr >= Decimal::from(4) => "fair",
                        Some(_) => "needs_attention",
                        None => "no_data",
                    };

                    PropertyPerformanceCard {
                        property_id: id,
                        property_name: name.unwrap_or_else(|| "Unnamed Property".to_string()),
                        building_address: None,
                        current_value: value,
                        equity,
                        ltv,
                        noi,
                        cap_rate,
                        cash_on_cash: coc,
                        dscr,
                        occupancy_rate: None,
                        monthly_cash_flow: mcf,
                        vs_benchmark_pct: None,
                        performance_status: status.to_string(),
                        currency,
                    }
                },
            )
            .collect();

        Ok(result)
    }

    /// Get cash flow trend.
    pub async fn get_cash_flow_trend(
        &self,
        portfolio_id: Uuid,
        months: i32,
    ) -> Result<Vec<CashFlowTrendPoint>, AppError> {
        let trend = sqlx::query_as::<
            _,
            (
                i32,
                i32,
                Decimal,
                Decimal,
                Decimal,
                Option<Decimal>,
                Decimal,
            ),
        >(
            r#"
            SELECT period_year, period_month,
                   SUM(total_income) as gross_income,
                   SUM(operating_expenses) as operating_expenses,
                   SUM(net_operating_income) as noi,
                   SUM(mortgage_payment) as debt_service,
                   SUM(cash_flow_after_debt) as net_cash_flow
            FROM property_cash_flows
            WHERE portfolio_id = $1
            GROUP BY period_year, period_month
            ORDER BY period_year DESC, period_month DESC
            LIMIT $2
            "#,
        )
        .bind(portfolio_id)
        .bind(months)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        let result = trend
            .into_iter()
            .map(|(year, month, gross, expenses, noi, debt, net)| {
                let date = NaiveDate::from_ymd_opt(year, month as u32, 1).unwrap_or_default();
                CashFlowTrendPoint {
                    period: format!("{}-{:02}", year, month),
                    period_date: date,
                    gross_income: gross,
                    operating_expenses: expenses,
                    noi,
                    debt_service: debt,
                    net_cash_flow: net,
                }
            })
            .collect();

        Ok(result)
    }

    // =========================================================================
    // ALERTS
    // =========================================================================

    /// Create a performance alert.
    pub async fn create_alert(
        &self,
        portfolio_id: Uuid,
        req: CreatePerformanceAlert,
    ) -> Result<PerformanceAlert, AppError> {
        let alert = sqlx::query_as::<_, PerformanceAlert>(
            r#"
            INSERT INTO performance_alerts (
                portfolio_id, property_id, alert_type, severity,
                title, message, metric_name, current_value, threshold_value
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, portfolio_id, property_id, alert_type, severity,
                      title, message, metric_name, current_value, threshold_value,
                      is_read, is_resolved, created_at
            "#,
        )
        .bind(portfolio_id)
        .bind(req.property_id)
        .bind(&req.alert_type)
        .bind(&req.severity)
        .bind(&req.title)
        .bind(&req.message)
        .bind(&req.metric_name)
        .bind(req.current_value)
        .bind(req.threshold_value)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(alert)
    }

    /// List alerts for a portfolio.
    pub async fn list_alerts(
        &self,
        portfolio_id: Uuid,
        unread_only: bool,
        limit: i32,
    ) -> Result<Vec<PerformanceAlert>, AppError> {
        let alerts = sqlx::query_as::<_, PerformanceAlert>(
            r#"
            SELECT id, portfolio_id, property_id, alert_type, severity,
                   title, message, metric_name, current_value, threshold_value,
                   is_read, is_resolved, created_at
            FROM performance_alerts
            WHERE portfolio_id = $1 AND ($2 = false OR is_read = false)
            ORDER BY created_at DESC
            LIMIT $3
            "#,
        )
        .bind(portfolio_id)
        .bind(unread_only)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(alerts)
    }

    /// Mark alert as read.
    pub async fn mark_alert_read(&self, id: Uuid, portfolio_id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query(
            "UPDATE performance_alerts SET is_read = true WHERE id = $1 AND portfolio_id = $2",
        )
        .bind(id)
        .bind(portfolio_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    /// Resolve an alert.
    pub async fn resolve_alert(&self, id: Uuid, portfolio_id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query(
            "UPDATE performance_alerts SET is_resolved = true WHERE id = $1 AND portfolio_id = $2",
        )
        .bind(id)
        .bind(portfolio_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }
}
