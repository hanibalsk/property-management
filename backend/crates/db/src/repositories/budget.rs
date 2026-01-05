//! Budget repository for Epic 24.
//!
//! Provides CRUD operations for budgets, budget items, capital plans, reserve funds, and forecasts.
//!
//! # RLS Integration
//!
//! This repository supports two usage patterns:
//!
//! 1. **RLS-aware** (recommended): Use methods with `_rls` suffix that accept an executor
//!    with RLS context already set (e.g., from `RlsConnection`).
//!
//! 2. **Legacy**: Use methods without suffix that use the internal pool. These do NOT
//!    enforce RLS and should be migrated to the RLS-aware pattern.
//!
//! ## Example
//!
//! ```rust,ignore
//! async fn create_budget(
//!     mut rls: RlsConnection,
//!     State(state): State<AppState>,
//!     Json(data): Json<CreateBudgetRequest>,
//! ) -> Result<Json<Budget>> {
//!     let budget = state.budget_repo.create_budget_rls(rls.conn(), org_id, user_id, data).await?;
//!     rls.release().await;
//!     Ok(Json(budget))
//! }
//! ```

use chrono::Datelike;

use crate::models::{
    budget_status, AcknowledgeVarianceAlert, Budget, BudgetActual, BudgetCategory, BudgetDashboard,
    BudgetItem, BudgetQuery, BudgetSummary, BudgetVarianceAlert, CapitalPlan, CapitalPlanQuery,
    CategoryVariance, CreateBudget, CreateBudgetCategory, CreateBudgetItem, CreateCapitalPlan,
    CreateFinancialForecast, CreateReserveFund, FinancialForecast, ForecastQuery,
    RecordBudgetActual, RecordReserveTransaction, ReserveFund, ReserveFundProjection,
    ReserveFundTransaction, UpdateBudget, UpdateBudgetCategory, UpdateBudgetItem,
    UpdateCapitalPlan, UpdateFinancialForecast, UpdateReserveFund, YearlyCapitalSummary,
};
use rust_decimal::Decimal;
use sqlx::{Executor, PgPool, Postgres};
use uuid::Uuid;

/// Repository for budget and financial planning operations.
#[derive(Clone)]
pub struct BudgetRepository {
    pool: PgPool,
}

impl BudgetRepository {
    /// Create a new budget repository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ===========================================
    // RLS-aware Budget Operations (recommended)
    // ===========================================

    /// Create a new budget with RLS context.
    ///
    /// Use this method with an `RlsConnection` to ensure RLS policies are enforced.
    pub async fn create_budget_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        user_id: Uuid,
        data: CreateBudget,
    ) -> Result<Budget, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            INSERT INTO budgets (organization_id, building_id, fiscal_year, name, notes, created_by)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(data.building_id)
        .bind(data.fiscal_year)
        .bind(&data.name)
        .bind(&data.notes)
        .bind(user_id)
        .fetch_one(executor)
        .await
    }

    /// Find budget by ID with RLS context.
    pub async fn find_budget_by_id_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<Option<Budget>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            SELECT * FROM budgets
            WHERE id = $1 AND organization_id = $2
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .fetch_optional(executor)
        .await
    }

    /// List budgets with filters and RLS context.
    pub async fn list_budgets_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        query: BudgetQuery,
    ) -> Result<Vec<Budget>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let limit = query.limit.unwrap_or(50);
        let offset = query.offset.unwrap_or(0);

        sqlx::query_as(
            r#"
            SELECT * FROM budgets
            WHERE organization_id = $1
              AND ($2::uuid IS NULL OR building_id = $2)
              AND ($3::integer IS NULL OR fiscal_year = $3)
              AND ($4::text IS NULL OR status = $4)
            ORDER BY fiscal_year DESC, created_at DESC
            LIMIT $5 OFFSET $6
            "#,
        )
        .bind(organization_id)
        .bind(query.building_id)
        .bind(query.fiscal_year)
        .bind(&query.status)
        .bind(limit)
        .bind(offset)
        .fetch_all(executor)
        .await
    }

    /// Update a budget with RLS context.
    pub async fn update_budget_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
        data: UpdateBudget,
    ) -> Result<Option<Budget>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE budgets
            SET name = COALESCE($3, name),
                notes = COALESCE($4, notes),
                updated_at = NOW()
            WHERE id = $1 AND organization_id = $2
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .bind(&data.name)
        .bind(&data.notes)
        .fetch_optional(executor)
        .await
    }

    /// Submit budget for approval with RLS context.
    pub async fn submit_budget_for_approval_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<Option<Budget>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE budgets
            SET status = $3, updated_at = NOW()
            WHERE id = $1 AND organization_id = $2 AND status = 'draft'
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .bind(budget_status::PENDING_APPROVAL)
        .fetch_optional(executor)
        .await
    }

    /// Approve a budget with RLS context.
    pub async fn approve_budget_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
        approved_by: Uuid,
    ) -> Result<Option<Budget>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE budgets
            SET status = $3, approved_by = $4, approved_at = NOW(), updated_at = NOW()
            WHERE id = $1 AND organization_id = $2 AND status = 'pending_approval'
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .bind(budget_status::APPROVED)
        .bind(approved_by)
        .fetch_optional(executor)
        .await
    }

    /// Activate a budget with RLS context.
    pub async fn activate_budget_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<Option<Budget>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE budgets
            SET status = $3, updated_at = NOW()
            WHERE id = $1 AND organization_id = $2 AND status = 'approved'
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .bind(budget_status::ACTIVE)
        .fetch_optional(executor)
        .await
    }

    /// Close a budget with RLS context.
    pub async fn close_budget_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<Option<Budget>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE budgets
            SET status = $3, updated_at = NOW()
            WHERE id = $1 AND organization_id = $2 AND status = 'active'
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .bind(budget_status::CLOSED)
        .fetch_optional(executor)
        .await
    }

    /// Delete a draft budget with RLS context.
    pub async fn delete_budget_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<bool, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let result = sqlx::query(
            r#"
            DELETE FROM budgets
            WHERE id = $1 AND organization_id = $2 AND status = 'draft'
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .execute(executor)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // ===========================================
    // RLS-aware Budget Category Operations
    // ===========================================

    /// Create a budget category with RLS context.
    pub async fn create_category_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        data: CreateBudgetCategory,
    ) -> Result<BudgetCategory, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            INSERT INTO budget_categories (organization_id, name, description, parent_id, sort_order)
            VALUES ($1, $2, $3, $4, COALESCE($5, 0))
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.parent_id)
        .bind(data.sort_order)
        .fetch_one(executor)
        .await
    }

    /// List categories for an organization with RLS context.
    pub async fn list_categories_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
    ) -> Result<Vec<BudgetCategory>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            SELECT * FROM budget_categories
            WHERE organization_id = $1
            ORDER BY sort_order, name
            "#,
        )
        .bind(organization_id)
        .fetch_all(executor)
        .await
    }

    /// Update a category with RLS context.
    pub async fn update_category_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
        data: UpdateBudgetCategory,
    ) -> Result<Option<BudgetCategory>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE budget_categories
            SET name = COALESCE($3, name),
                description = COALESCE($4, description),
                sort_order = COALESCE($5, sort_order)
            WHERE id = $1 AND organization_id = $2 AND is_system = false
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.sort_order)
        .fetch_optional(executor)
        .await
    }

    /// Delete a category with RLS context.
    pub async fn delete_category_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<bool, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let result = sqlx::query(
            r#"
            DELETE FROM budget_categories
            WHERE id = $1 AND organization_id = $2 AND is_system = false
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .execute(executor)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // ===========================================
    // RLS-aware Budget Item Operations
    // ===========================================

    /// Add an item to a budget with RLS context.
    pub async fn add_budget_item_rls<'e, E>(
        &self,
        executor: E,
        budget_id: Uuid,
        data: CreateBudgetItem,
    ) -> Result<BudgetItem, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            INSERT INTO budget_items (budget_id, category_id, name, description, budgeted_amount, notes)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(budget_id)
        .bind(data.category_id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.budgeted_amount)
        .bind(&data.notes)
        .fetch_one(executor)
        .await
    }

    /// List items for a budget with RLS context.
    pub async fn list_budget_items_rls<'e, E>(
        &self,
        executor: E,
        budget_id: Uuid,
    ) -> Result<Vec<BudgetItem>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            SELECT bi.* FROM budget_items bi
            JOIN budget_categories bc ON bc.id = bi.category_id
            WHERE bi.budget_id = $1
            ORDER BY bc.sort_order, bc.name, bi.name
            "#,
        )
        .bind(budget_id)
        .fetch_all(executor)
        .await
    }

    /// Update a budget item with RLS context.
    pub async fn update_budget_item_rls<'e, E>(
        &self,
        executor: E,
        id: Uuid,
        data: UpdateBudgetItem,
    ) -> Result<Option<BudgetItem>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE budget_items
            SET name = COALESCE($2, name),
                description = COALESCE($3, description),
                budgeted_amount = COALESCE($4, budgeted_amount),
                notes = COALESCE($5, notes),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.budgeted_amount)
        .bind(&data.notes)
        .fetch_optional(executor)
        .await
    }

    /// Delete a budget item with RLS context.
    pub async fn delete_budget_item_rls<'e, E>(
        &self,
        executor: E,
        id: Uuid,
    ) -> Result<bool, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let result = sqlx::query("DELETE FROM budget_items WHERE id = $1")
            .bind(id)
            .execute(executor)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // ===========================================
    // RLS-aware Budget Actuals Operations
    // ===========================================

    /// Record an actual expense against a budget item with RLS context.
    pub async fn record_actual_rls<'e, E>(
        &self,
        executor: E,
        budget_item_id: Uuid,
        user_id: Uuid,
        data: RecordBudgetActual,
    ) -> Result<BudgetActual, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            INSERT INTO budget_actuals (budget_item_id, transaction_id, amount, description, transaction_date, recorded_by)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(budget_item_id)
        .bind(data.transaction_id)
        .bind(data.amount)
        .bind(&data.description)
        .bind(data.transaction_date)
        .bind(user_id)
        .fetch_one(executor)
        .await
    }

    /// List actuals for a budget item with RLS context.
    pub async fn list_actuals_rls<'e, E>(
        &self,
        executor: E,
        budget_item_id: Uuid,
    ) -> Result<Vec<BudgetActual>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            SELECT * FROM budget_actuals
            WHERE budget_item_id = $1
            ORDER BY transaction_date DESC, created_at DESC
            "#,
        )
        .bind(budget_item_id)
        .fetch_all(executor)
        .await
    }

    // ===========================================
    // RLS-aware Capital Plan Operations
    // ===========================================

    /// Create a capital plan with RLS context.
    pub async fn create_capital_plan_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        user_id: Uuid,
        data: CreateCapitalPlan,
    ) -> Result<CapitalPlan, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            INSERT INTO capital_plans (
                organization_id, building_id, name, description, estimated_cost,
                funding_source, target_year, target_quarter, priority, notes, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, COALESCE($9, 'medium'), $10, $11)
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(data.building_id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.estimated_cost)
        .bind(&data.funding_source)
        .bind(data.target_year)
        .bind(data.target_quarter)
        .bind(&data.priority)
        .bind(&data.notes)
        .bind(user_id)
        .fetch_one(executor)
        .await
    }

    /// Find capital plan by ID with RLS context.
    pub async fn find_capital_plan_by_id_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<Option<CapitalPlan>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            SELECT * FROM capital_plans
            WHERE id = $1 AND organization_id = $2
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .fetch_optional(executor)
        .await
    }

    /// List capital plans with filters and RLS context.
    pub async fn list_capital_plans_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        query: CapitalPlanQuery,
    ) -> Result<Vec<CapitalPlan>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let limit = query.limit.unwrap_or(50);
        let offset = query.offset.unwrap_or(0);

        sqlx::query_as(
            r#"
            SELECT * FROM capital_plans
            WHERE organization_id = $1
              AND ($2::uuid IS NULL OR building_id = $2)
              AND ($3::integer IS NULL OR target_year = $3)
              AND ($4::text IS NULL OR status = $4)
              AND ($5::text IS NULL OR priority = $5)
            ORDER BY target_year, target_quarter NULLS LAST, priority DESC
            LIMIT $6 OFFSET $7
            "#,
        )
        .bind(organization_id)
        .bind(query.building_id)
        .bind(query.target_year)
        .bind(&query.status)
        .bind(&query.priority)
        .bind(limit)
        .bind(offset)
        .fetch_all(executor)
        .await
    }

    /// Update a capital plan with RLS context.
    pub async fn update_capital_plan_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
        data: UpdateCapitalPlan,
    ) -> Result<Option<CapitalPlan>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE capital_plans
            SET name = COALESCE($3, name),
                description = COALESCE($4, description),
                estimated_cost = COALESCE($5, estimated_cost),
                actual_cost = COALESCE($6, actual_cost),
                funding_source = COALESCE($7, funding_source),
                target_year = COALESCE($8, target_year),
                target_quarter = COALESCE($9, target_quarter),
                priority = COALESCE($10, priority),
                start_date = COALESCE($11, start_date),
                completion_date = COALESCE($12, completion_date),
                notes = COALESCE($13, notes),
                updated_at = NOW()
            WHERE id = $1 AND organization_id = $2
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.estimated_cost)
        .bind(data.actual_cost)
        .bind(&data.funding_source)
        .bind(data.target_year)
        .bind(data.target_quarter)
        .bind(&data.priority)
        .bind(data.start_date)
        .bind(data.completion_date)
        .bind(&data.notes)
        .fetch_optional(executor)
        .await
    }

    /// Start a capital plan with RLS context.
    pub async fn start_capital_plan_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<Option<CapitalPlan>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE capital_plans
            SET status = 'in_progress', start_date = CURRENT_DATE, updated_at = NOW()
            WHERE id = $1 AND organization_id = $2 AND status IN ('planned', 'approved')
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .fetch_optional(executor)
        .await
    }

    /// Complete a capital plan with RLS context.
    pub async fn complete_capital_plan_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
        actual_cost: Decimal,
    ) -> Result<Option<CapitalPlan>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE capital_plans
            SET status = 'completed', actual_cost = $3, completion_date = CURRENT_DATE, updated_at = NOW()
            WHERE id = $1 AND organization_id = $2 AND status = 'in_progress'
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .bind(actual_cost)
        .fetch_optional(executor)
        .await
    }

    /// Delete a capital plan with RLS context.
    pub async fn delete_capital_plan_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<bool, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let result = sqlx::query(
            r#"
            DELETE FROM capital_plans
            WHERE id = $1 AND organization_id = $2 AND status = 'planned'
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .execute(executor)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // ===========================================
    // RLS-aware Reserve Fund Operations
    // ===========================================

    /// Create a reserve fund with RLS context.
    pub async fn create_reserve_fund_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        data: CreateReserveFund,
    ) -> Result<ReserveFund, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            INSERT INTO reserve_funds (organization_id, building_id, name, target_balance, annual_contribution, notes)
            VALUES ($1, $2, COALESCE($3, 'General Reserve'), $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(data.building_id)
        .bind(&data.name)
        .bind(data.target_balance)
        .bind(data.annual_contribution)
        .bind(&data.notes)
        .fetch_one(executor)
        .await
    }

    /// Find reserve fund by ID with RLS context.
    pub async fn find_reserve_fund_by_id_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<Option<ReserveFund>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            SELECT * FROM reserve_funds
            WHERE id = $1 AND organization_id = $2
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .fetch_optional(executor)
        .await
    }

    /// List reserve funds with RLS context.
    pub async fn list_reserve_funds_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        building_id: Option<Uuid>,
    ) -> Result<Vec<ReserveFund>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            SELECT * FROM reserve_funds
            WHERE organization_id = $1
              AND ($2::uuid IS NULL OR building_id = $2)
            ORDER BY name
            "#,
        )
        .bind(organization_id)
        .bind(building_id)
        .fetch_all(executor)
        .await
    }

    /// Update a reserve fund with RLS context.
    pub async fn update_reserve_fund_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
        data: UpdateReserveFund,
    ) -> Result<Option<ReserveFund>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE reserve_funds
            SET name = COALESCE($3, name),
                target_balance = COALESCE($4, target_balance),
                annual_contribution = COALESCE($5, annual_contribution),
                notes = COALESCE($6, notes),
                updated_at = NOW()
            WHERE id = $1 AND organization_id = $2
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .bind(&data.name)
        .bind(data.target_balance)
        .bind(data.annual_contribution)
        .bind(&data.notes)
        .fetch_optional(executor)
        .await
    }

    /// Record a reserve fund transaction with RLS context.
    ///
    /// Note: This method requires the reserve fund to be fetched first to calculate balance.
    /// For full RLS support, fetch the fund using find_reserve_fund_by_id_rls first.
    pub async fn record_reserve_transaction_rls<'e, E>(
        &self,
        executor: E,
        reserve_fund_id: Uuid,
        user_id: Uuid,
        current_balance: Decimal,
        data: RecordReserveTransaction,
    ) -> Result<ReserveFundTransaction, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let balance_after = match data.transaction_type.as_str() {
            "contribution" | "interest" => current_balance + data.amount,
            "withdrawal" => current_balance - data.amount,
            "adjustment" => current_balance + data.amount,
            _ => current_balance + data.amount,
        };

        sqlx::query_as(
            r#"
            INSERT INTO reserve_fund_transactions (
                reserve_fund_id, transaction_type, amount, description,
                reference_type, reference_id, balance_after, transaction_date, recorded_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(reserve_fund_id)
        .bind(&data.transaction_type)
        .bind(data.amount)
        .bind(&data.description)
        .bind(&data.reference_type)
        .bind(data.reference_id)
        .bind(balance_after)
        .bind(data.transaction_date)
        .bind(user_id)
        .fetch_one(executor)
        .await
    }

    /// List reserve fund transactions with RLS context.
    pub async fn list_reserve_transactions_rls<'e, E>(
        &self,
        executor: E,
        reserve_fund_id: Uuid,
    ) -> Result<Vec<ReserveFundTransaction>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            SELECT * FROM reserve_fund_transactions
            WHERE reserve_fund_id = $1
            ORDER BY transaction_date DESC, created_at DESC
            "#,
        )
        .bind(reserve_fund_id)
        .fetch_all(executor)
        .await
    }

    // ===========================================
    // RLS-aware Financial Forecast Operations
    // ===========================================

    /// Create a financial forecast with RLS context.
    pub async fn create_forecast_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        user_id: Uuid,
        data: CreateFinancialForecast,
    ) -> Result<FinancialForecast, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            INSERT INTO financial_forecasts (
                organization_id, building_id, name, forecast_type, start_year, end_year,
                inflation_rate, parameters, notes, created_by
            )
            VALUES ($1, $2, $3, COALESCE($4, 'expense'), $5, $6, COALESCE($7, 3.00), COALESCE($8, '{}'), $9, $10)
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(data.building_id)
        .bind(&data.name)
        .bind(&data.forecast_type)
        .bind(data.start_year)
        .bind(data.end_year)
        .bind(data.inflation_rate)
        .bind(&data.parameters)
        .bind(&data.notes)
        .bind(user_id)
        .fetch_one(executor)
        .await
    }

    /// Find forecast by ID with RLS context.
    pub async fn find_forecast_by_id_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<Option<FinancialForecast>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            SELECT * FROM financial_forecasts
            WHERE id = $1 AND organization_id = $2
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .fetch_optional(executor)
        .await
    }

    /// List forecasts with RLS context.
    pub async fn list_forecasts_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        query: ForecastQuery,
    ) -> Result<Vec<FinancialForecast>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let limit = query.limit.unwrap_or(50);
        let offset = query.offset.unwrap_or(0);

        sqlx::query_as(
            r#"
            SELECT * FROM financial_forecasts
            WHERE organization_id = $1
              AND ($2::uuid IS NULL OR building_id = $2)
              AND ($3::text IS NULL OR forecast_type = $3)
            ORDER BY created_at DESC
            LIMIT $4 OFFSET $5
            "#,
        )
        .bind(organization_id)
        .bind(query.building_id)
        .bind(&query.forecast_type)
        .bind(limit)
        .bind(offset)
        .fetch_all(executor)
        .await
    }

    /// Update a forecast with RLS context.
    pub async fn update_forecast_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
        data: UpdateFinancialForecast,
    ) -> Result<Option<FinancialForecast>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE financial_forecasts
            SET name = COALESCE($3, name),
                inflation_rate = COALESCE($4, inflation_rate),
                parameters = COALESCE($5, parameters),
                forecast_data = COALESCE($6, forecast_data),
                notes = COALESCE($7, notes),
                updated_at = NOW()
            WHERE id = $1 AND organization_id = $2
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .bind(&data.name)
        .bind(data.inflation_rate)
        .bind(&data.parameters)
        .bind(&data.forecast_data)
        .bind(&data.notes)
        .fetch_optional(executor)
        .await
    }

    /// Delete a forecast with RLS context.
    pub async fn delete_forecast_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<bool, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let result = sqlx::query(
            r#"
            DELETE FROM financial_forecasts
            WHERE id = $1 AND organization_id = $2
            "#,
        )
        .bind(id)
        .bind(organization_id)
        .execute(executor)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // ===========================================
    // RLS-aware Variance Alert Operations
    // ===========================================

    /// List pending variance alerts for a budget with RLS context.
    pub async fn list_variance_alerts_rls<'e, E>(
        &self,
        executor: E,
        budget_id: Uuid,
        acknowledged: Option<bool>,
    ) -> Result<Vec<BudgetVarianceAlert>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            SELECT bva.* FROM budget_variance_alerts bva
            JOIN budget_items bi ON bi.id = bva.budget_item_id
            WHERE bi.budget_id = $1
              AND ($2::boolean IS NULL OR bva.is_acknowledged = $2)
            ORDER BY bva.created_at DESC
            "#,
        )
        .bind(budget_id)
        .bind(acknowledged)
        .fetch_all(executor)
        .await
    }

    /// Acknowledge a variance alert with RLS context.
    pub async fn acknowledge_alert_rls<'e, E>(
        &self,
        executor: E,
        id: Uuid,
        user_id: Uuid,
        data: AcknowledgeVarianceAlert,
    ) -> Result<Option<BudgetVarianceAlert>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let _ = data;
        sqlx::query_as(
            r#"
            UPDATE budget_variance_alerts
            SET is_acknowledged = true, acknowledged_by = $2, acknowledged_at = NOW()
            WHERE id = $1 AND is_acknowledged = false
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(executor)
        .await
    }

    // ===========================================
    // RLS-aware Statistics & Reporting
    // ===========================================

    /// Get budget summary with RLS context.
    pub async fn get_budget_summary_rls<'e, E>(
        &self,
        executor: E,
        budget_id: Uuid,
    ) -> Result<BudgetSummary, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let result: (Decimal, Decimal, Decimal, Decimal, i64, i64) = sqlx::query_as(
            r#"
            SELECT
                COALESCE(SUM(budgeted_amount), 0) as total_budgeted,
                COALESCE(SUM(actual_amount), 0) as total_actual,
                COALESCE(SUM(variance_amount), 0) as total_variance,
                CASE WHEN SUM(budgeted_amount) = 0 THEN 0
                     ELSE ROUND((SUM(actual_amount) - SUM(budgeted_amount)) / SUM(budgeted_amount) * 100, 2)
                END as variance_percent,
                COUNT(*) FILTER (WHERE variance_amount > 0) as items_over_budget,
                COUNT(*) FILTER (WHERE variance_amount < 0) as items_under_budget
            FROM budget_items
            WHERE budget_id = $1
            "#,
        )
        .bind(budget_id)
        .fetch_one(executor)
        .await?;

        Ok(BudgetSummary {
            total_budgeted: result.0,
            total_actual: result.1,
            total_variance: result.2,
            variance_percent: result.3,
            items_over_budget: result.4,
            items_under_budget: result.5,
        })
    }

    /// Get variance by category with RLS context.
    pub async fn get_category_variance_rls<'e, E>(
        &self,
        executor: E,
        budget_id: Uuid,
    ) -> Result<Vec<CategoryVariance>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            SELECT
                bc.id as category_id,
                bc.name as category_name,
                COALESCE(SUM(bi.budgeted_amount), 0) as budgeted_amount,
                COALESCE(SUM(bi.actual_amount), 0) as actual_amount,
                COALESCE(SUM(bi.variance_amount), 0) as variance_amount,
                CASE WHEN SUM(bi.budgeted_amount) = 0 THEN 0
                     ELSE ROUND((SUM(bi.actual_amount) - SUM(bi.budgeted_amount)) / SUM(bi.budgeted_amount) * 100, 2)
                END as variance_percent
            FROM budget_categories bc
            LEFT JOIN budget_items bi ON bi.category_id = bc.id AND bi.budget_id = $1
            WHERE bc.organization_id = (SELECT organization_id FROM budgets WHERE id = $1)
            GROUP BY bc.id, bc.name
            ORDER BY bc.sort_order, bc.name
            "#,
        )
        .bind(budget_id)
        .fetch_all(executor)
        .await
    }

    /// Get yearly capital plan summary with RLS context.
    pub async fn get_yearly_capital_summary_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
    ) -> Result<Vec<YearlyCapitalSummary>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            SELECT
                target_year,
                COALESCE(SUM(estimated_cost), 0) as total_estimated,
                COALESCE(SUM(actual_cost), 0) as total_actual,
                COUNT(*) as plan_count
            FROM capital_plans
            WHERE organization_id = $1
            GROUP BY target_year
            ORDER BY target_year
            "#,
        )
        .bind(organization_id)
        .fetch_all(executor)
        .await
    }

    /// Get budget dashboard with RLS context.
    ///
    /// Note: This method makes multiple queries and cannot use a single executor.
    /// For full RLS support, call individual RLS methods separately.
    pub async fn get_dashboard_rls<'e, E>(
        &self,
        executor: E,
        organization_id: Uuid,
        building_id: Option<Uuid>,
    ) -> Result<(Option<Budget>, Decimal), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        // Get active budget and reserve balance in a single query
        // Note: Full dashboard requires multiple queries; for RLS use, call individual methods
        let active_budget: Option<Budget> = sqlx::query_as(
            r#"
            SELECT * FROM budgets
            WHERE organization_id = $1
              AND ($2::uuid IS NULL OR building_id = $2)
              AND status = 'active'
            ORDER BY fiscal_year DESC
            LIMIT 1
            "#,
        )
        .bind(organization_id)
        .bind(building_id)
        .fetch_optional(executor)
        .await?;

        // Reserve balance would need a separate executor call
        // Return default for now; callers should use list_reserve_funds_rls for full data
        Ok((active_budget, Decimal::ZERO))
    }

    // ===========================================
    // Legacy Budget Operations (deprecated)
    // ===========================================

    /// Create a new budget.
    ///
    /// **Deprecated**: Use `create_budget_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use create_budget_rls with RlsConnection instead"
    )]
    pub async fn create_budget(
        &self,
        organization_id: Uuid,
        user_id: Uuid,
        data: CreateBudget,
    ) -> Result<Budget, sqlx::Error> {
        self.create_budget_rls(&self.pool, organization_id, user_id, data)
            .await
    }

    /// Find budget by ID.
    ///
    /// **Deprecated**: Use `find_budget_by_id_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use find_budget_by_id_rls with RlsConnection instead"
    )]
    pub async fn find_budget_by_id(
        &self,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<Option<Budget>, sqlx::Error> {
        self.find_budget_by_id_rls(&self.pool, organization_id, id)
            .await
    }

    /// List budgets with filters.
    ///
    /// **Deprecated**: Use `list_budgets_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use list_budgets_rls with RlsConnection instead"
    )]
    pub async fn list_budgets(
        &self,
        organization_id: Uuid,
        query: BudgetQuery,
    ) -> Result<Vec<Budget>, sqlx::Error> {
        self.list_budgets_rls(&self.pool, organization_id, query)
            .await
    }

    /// Update a budget.
    ///
    /// **Deprecated**: Use `update_budget_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use update_budget_rls with RlsConnection instead"
    )]
    pub async fn update_budget(
        &self,
        organization_id: Uuid,
        id: Uuid,
        data: UpdateBudget,
    ) -> Result<Option<Budget>, sqlx::Error> {
        self.update_budget_rls(&self.pool, organization_id, id, data)
            .await
    }

    /// Submit budget for approval.
    ///
    /// **Deprecated**: Use `submit_budget_for_approval_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use submit_budget_for_approval_rls with RlsConnection instead"
    )]
    pub async fn submit_budget_for_approval(
        &self,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<Option<Budget>, sqlx::Error> {
        self.submit_budget_for_approval_rls(&self.pool, organization_id, id)
            .await
    }

    /// Approve a budget.
    ///
    /// **Deprecated**: Use `approve_budget_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use approve_budget_rls with RlsConnection instead"
    )]
    pub async fn approve_budget(
        &self,
        organization_id: Uuid,
        id: Uuid,
        approved_by: Uuid,
    ) -> Result<Option<Budget>, sqlx::Error> {
        self.approve_budget_rls(&self.pool, organization_id, id, approved_by)
            .await
    }

    /// Activate a budget.
    ///
    /// **Deprecated**: Use `activate_budget_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use activate_budget_rls with RlsConnection instead"
    )]
    pub async fn activate_budget(
        &self,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<Option<Budget>, sqlx::Error> {
        self.activate_budget_rls(&self.pool, organization_id, id)
            .await
    }

    /// Close a budget.
    ///
    /// **Deprecated**: Use `close_budget_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use close_budget_rls with RlsConnection instead"
    )]
    pub async fn close_budget(
        &self,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<Option<Budget>, sqlx::Error> {
        self.close_budget_rls(&self.pool, organization_id, id).await
    }

    /// Delete a draft budget.
    ///
    /// **Deprecated**: Use `delete_budget_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use delete_budget_rls with RlsConnection instead"
    )]
    pub async fn delete_budget(
        &self,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        self.delete_budget_rls(&self.pool, organization_id, id)
            .await
    }

    // ===========================================
    // Legacy Budget Category Operations (deprecated)
    // ===========================================

    /// Create a budget category.
    ///
    /// **Deprecated**: Use `create_category_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use create_category_rls with RlsConnection instead"
    )]
    pub async fn create_category(
        &self,
        organization_id: Uuid,
        data: CreateBudgetCategory,
    ) -> Result<BudgetCategory, sqlx::Error> {
        self.create_category_rls(&self.pool, organization_id, data)
            .await
    }

    /// List categories for an organization.
    ///
    /// **Deprecated**: Use `list_categories_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use list_categories_rls with RlsConnection instead"
    )]
    pub async fn list_categories(
        &self,
        organization_id: Uuid,
    ) -> Result<Vec<BudgetCategory>, sqlx::Error> {
        self.list_categories_rls(&self.pool, organization_id).await
    }

    /// Update a category.
    ///
    /// **Deprecated**: Use `update_category_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use update_category_rls with RlsConnection instead"
    )]
    pub async fn update_category(
        &self,
        organization_id: Uuid,
        id: Uuid,
        data: UpdateBudgetCategory,
    ) -> Result<Option<BudgetCategory>, sqlx::Error> {
        self.update_category_rls(&self.pool, organization_id, id, data)
            .await
    }

    /// Delete a category.
    ///
    /// **Deprecated**: Use `delete_category_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use delete_category_rls with RlsConnection instead"
    )]
    pub async fn delete_category(
        &self,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        self.delete_category_rls(&self.pool, organization_id, id)
            .await
    }

    // ===========================================
    // Legacy Budget Item Operations (deprecated)
    // ===========================================

    /// Add an item to a budget.
    ///
    /// **Deprecated**: Use `add_budget_item_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use add_budget_item_rls with RlsConnection instead"
    )]
    pub async fn add_budget_item(
        &self,
        budget_id: Uuid,
        data: CreateBudgetItem,
    ) -> Result<BudgetItem, sqlx::Error> {
        self.add_budget_item_rls(&self.pool, budget_id, data).await
    }

    /// List items for a budget.
    ///
    /// **Deprecated**: Use `list_budget_items_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use list_budget_items_rls with RlsConnection instead"
    )]
    pub async fn list_budget_items(&self, budget_id: Uuid) -> Result<Vec<BudgetItem>, sqlx::Error> {
        self.list_budget_items_rls(&self.pool, budget_id).await
    }

    /// Update a budget item.
    ///
    /// **Deprecated**: Use `update_budget_item_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use update_budget_item_rls with RlsConnection instead"
    )]
    pub async fn update_budget_item(
        &self,
        id: Uuid,
        data: UpdateBudgetItem,
    ) -> Result<Option<BudgetItem>, sqlx::Error> {
        self.update_budget_item_rls(&self.pool, id, data).await
    }

    /// Delete a budget item.
    ///
    /// **Deprecated**: Use `delete_budget_item_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use delete_budget_item_rls with RlsConnection instead"
    )]
    pub async fn delete_budget_item(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        self.delete_budget_item_rls(&self.pool, id).await
    }

    // ===========================================
    // Legacy Budget Actuals Operations (deprecated)
    // ===========================================

    /// Record an actual expense against a budget item.
    ///
    /// **Deprecated**: Use `record_actual_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use record_actual_rls with RlsConnection instead"
    )]
    pub async fn record_actual(
        &self,
        budget_item_id: Uuid,
        user_id: Uuid,
        data: RecordBudgetActual,
    ) -> Result<BudgetActual, sqlx::Error> {
        self.record_actual_rls(&self.pool, budget_item_id, user_id, data)
            .await
    }

    /// List actuals for a budget item.
    ///
    /// **Deprecated**: Use `list_actuals_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use list_actuals_rls with RlsConnection instead"
    )]
    pub async fn list_actuals(
        &self,
        budget_item_id: Uuid,
    ) -> Result<Vec<BudgetActual>, sqlx::Error> {
        self.list_actuals_rls(&self.pool, budget_item_id).await
    }

    // ===========================================
    // Legacy Capital Plan Operations (deprecated)
    // ===========================================

    /// Create a capital plan.
    ///
    /// **Deprecated**: Use `create_capital_plan_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use create_capital_plan_rls with RlsConnection instead"
    )]
    pub async fn create_capital_plan(
        &self,
        organization_id: Uuid,
        user_id: Uuid,
        data: CreateCapitalPlan,
    ) -> Result<CapitalPlan, sqlx::Error> {
        self.create_capital_plan_rls(&self.pool, organization_id, user_id, data)
            .await
    }

    /// Find capital plan by ID.
    ///
    /// **Deprecated**: Use `find_capital_plan_by_id_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use find_capital_plan_by_id_rls with RlsConnection instead"
    )]
    pub async fn find_capital_plan_by_id(
        &self,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<Option<CapitalPlan>, sqlx::Error> {
        self.find_capital_plan_by_id_rls(&self.pool, organization_id, id)
            .await
    }

    /// List capital plans with filters.
    ///
    /// **Deprecated**: Use `list_capital_plans_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use list_capital_plans_rls with RlsConnection instead"
    )]
    pub async fn list_capital_plans(
        &self,
        organization_id: Uuid,
        query: CapitalPlanQuery,
    ) -> Result<Vec<CapitalPlan>, sqlx::Error> {
        self.list_capital_plans_rls(&self.pool, organization_id, query)
            .await
    }

    /// Update a capital plan.
    ///
    /// **Deprecated**: Use `update_capital_plan_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use update_capital_plan_rls with RlsConnection instead"
    )]
    pub async fn update_capital_plan(
        &self,
        organization_id: Uuid,
        id: Uuid,
        data: UpdateCapitalPlan,
    ) -> Result<Option<CapitalPlan>, sqlx::Error> {
        self.update_capital_plan_rls(&self.pool, organization_id, id, data)
            .await
    }

    /// Start a capital plan.
    ///
    /// **Deprecated**: Use `start_capital_plan_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use start_capital_plan_rls with RlsConnection instead"
    )]
    pub async fn start_capital_plan(
        &self,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<Option<CapitalPlan>, sqlx::Error> {
        self.start_capital_plan_rls(&self.pool, organization_id, id)
            .await
    }

    /// Complete a capital plan.
    ///
    /// **Deprecated**: Use `complete_capital_plan_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use complete_capital_plan_rls with RlsConnection instead"
    )]
    pub async fn complete_capital_plan(
        &self,
        organization_id: Uuid,
        id: Uuid,
        actual_cost: Decimal,
    ) -> Result<Option<CapitalPlan>, sqlx::Error> {
        self.complete_capital_plan_rls(&self.pool, organization_id, id, actual_cost)
            .await
    }

    /// Delete a capital plan.
    ///
    /// **Deprecated**: Use `delete_capital_plan_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use delete_capital_plan_rls with RlsConnection instead"
    )]
    pub async fn delete_capital_plan(
        &self,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        self.delete_capital_plan_rls(&self.pool, organization_id, id)
            .await
    }

    // ===========================================
    // Legacy Reserve Fund Operations (deprecated)
    // ===========================================

    /// Create a reserve fund.
    ///
    /// **Deprecated**: Use `create_reserve_fund_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use create_reserve_fund_rls with RlsConnection instead"
    )]
    pub async fn create_reserve_fund(
        &self,
        organization_id: Uuid,
        data: CreateReserveFund,
    ) -> Result<ReserveFund, sqlx::Error> {
        self.create_reserve_fund_rls(&self.pool, organization_id, data)
            .await
    }

    /// Find reserve fund by ID.
    ///
    /// **Deprecated**: Use `find_reserve_fund_by_id_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use find_reserve_fund_by_id_rls with RlsConnection instead"
    )]
    pub async fn find_reserve_fund_by_id(
        &self,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<Option<ReserveFund>, sqlx::Error> {
        self.find_reserve_fund_by_id_rls(&self.pool, organization_id, id)
            .await
    }

    /// List reserve funds.
    ///
    /// **Deprecated**: Use `list_reserve_funds_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use list_reserve_funds_rls with RlsConnection instead"
    )]
    pub async fn list_reserve_funds(
        &self,
        organization_id: Uuid,
        building_id: Option<Uuid>,
    ) -> Result<Vec<ReserveFund>, sqlx::Error> {
        self.list_reserve_funds_rls(&self.pool, organization_id, building_id)
            .await
    }

    /// Update a reserve fund.
    ///
    /// **Deprecated**: Use `update_reserve_fund_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use update_reserve_fund_rls with RlsConnection instead"
    )]
    pub async fn update_reserve_fund(
        &self,
        organization_id: Uuid,
        id: Uuid,
        data: UpdateReserveFund,
    ) -> Result<Option<ReserveFund>, sqlx::Error> {
        self.update_reserve_fund_rls(&self.pool, organization_id, id, data)
            .await
    }

    /// Record a reserve fund transaction.
    ///
    /// **Deprecated**: Use `record_reserve_transaction_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use record_reserve_transaction_rls with RlsConnection instead"
    )]
    #[allow(deprecated)]
    pub async fn record_reserve_transaction(
        &self,
        reserve_fund_id: Uuid,
        user_id: Uuid,
        data: RecordReserveTransaction,
    ) -> Result<ReserveFundTransaction, sqlx::Error> {
        // Get current balance using deprecated method (internal use)
        let fund: ReserveFund = sqlx::query_as("SELECT * FROM reserve_funds WHERE id = $1")
            .bind(reserve_fund_id)
            .fetch_one(&self.pool)
            .await?;

        self.record_reserve_transaction_rls(
            &self.pool,
            reserve_fund_id,
            user_id,
            fund.current_balance,
            data,
        )
        .await
    }

    /// List reserve fund transactions.
    ///
    /// **Deprecated**: Use `list_reserve_transactions_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use list_reserve_transactions_rls with RlsConnection instead"
    )]
    pub async fn list_reserve_transactions(
        &self,
        reserve_fund_id: Uuid,
    ) -> Result<Vec<ReserveFundTransaction>, sqlx::Error> {
        self.list_reserve_transactions_rls(&self.pool, reserve_fund_id)
            .await
    }

    // ===========================================
    // Legacy Financial Forecast Operations (deprecated)
    // ===========================================

    /// Create a financial forecast.
    ///
    /// **Deprecated**: Use `create_forecast_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use create_forecast_rls with RlsConnection instead"
    )]
    pub async fn create_forecast(
        &self,
        organization_id: Uuid,
        user_id: Uuid,
        data: CreateFinancialForecast,
    ) -> Result<FinancialForecast, sqlx::Error> {
        self.create_forecast_rls(&self.pool, organization_id, user_id, data)
            .await
    }

    /// Find forecast by ID.
    ///
    /// **Deprecated**: Use `find_forecast_by_id_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use find_forecast_by_id_rls with RlsConnection instead"
    )]
    pub async fn find_forecast_by_id(
        &self,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<Option<FinancialForecast>, sqlx::Error> {
        self.find_forecast_by_id_rls(&self.pool, organization_id, id)
            .await
    }

    /// List forecasts.
    ///
    /// **Deprecated**: Use `list_forecasts_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use list_forecasts_rls with RlsConnection instead"
    )]
    pub async fn list_forecasts(
        &self,
        organization_id: Uuid,
        query: ForecastQuery,
    ) -> Result<Vec<FinancialForecast>, sqlx::Error> {
        self.list_forecasts_rls(&self.pool, organization_id, query)
            .await
    }

    /// Update a forecast.
    ///
    /// **Deprecated**: Use `update_forecast_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use update_forecast_rls with RlsConnection instead"
    )]
    pub async fn update_forecast(
        &self,
        organization_id: Uuid,
        id: Uuid,
        data: UpdateFinancialForecast,
    ) -> Result<Option<FinancialForecast>, sqlx::Error> {
        self.update_forecast_rls(&self.pool, organization_id, id, data)
            .await
    }

    /// Delete a forecast.
    ///
    /// **Deprecated**: Use `delete_forecast_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use delete_forecast_rls with RlsConnection instead"
    )]
    pub async fn delete_forecast(
        &self,
        organization_id: Uuid,
        id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        self.delete_forecast_rls(&self.pool, organization_id, id)
            .await
    }

    // ===========================================
    // Legacy Variance Alert Operations (deprecated)
    // ===========================================

    /// List pending variance alerts for a budget.
    ///
    /// **Deprecated**: Use `list_variance_alerts_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use list_variance_alerts_rls with RlsConnection instead"
    )]
    pub async fn list_variance_alerts(
        &self,
        budget_id: Uuid,
        acknowledged: Option<bool>,
    ) -> Result<Vec<BudgetVarianceAlert>, sqlx::Error> {
        self.list_variance_alerts_rls(&self.pool, budget_id, acknowledged)
            .await
    }

    /// Acknowledge a variance alert.
    ///
    /// **Deprecated**: Use `acknowledge_alert_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use acknowledge_alert_rls with RlsConnection instead"
    )]
    pub async fn acknowledge_alert(
        &self,
        id: Uuid,
        user_id: Uuid,
        data: AcknowledgeVarianceAlert,
    ) -> Result<Option<BudgetVarianceAlert>, sqlx::Error> {
        self.acknowledge_alert_rls(&self.pool, id, user_id, data)
            .await
    }

    // ===========================================
    // Legacy Statistics & Reporting (deprecated)
    // ===========================================

    /// Get budget summary.
    ///
    /// **Deprecated**: Use `get_budget_summary_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use get_budget_summary_rls with RlsConnection instead"
    )]
    pub async fn get_budget_summary(&self, budget_id: Uuid) -> Result<BudgetSummary, sqlx::Error> {
        self.get_budget_summary_rls(&self.pool, budget_id).await
    }

    /// Get variance by category.
    ///
    /// **Deprecated**: Use `get_category_variance_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use get_category_variance_rls with RlsConnection instead"
    )]
    pub async fn get_category_variance(
        &self,
        budget_id: Uuid,
    ) -> Result<Vec<CategoryVariance>, sqlx::Error> {
        self.get_category_variance_rls(&self.pool, budget_id).await
    }

    /// Get yearly capital plan summary.
    ///
    /// **Deprecated**: Use `get_yearly_capital_summary_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use get_yearly_capital_summary_rls with RlsConnection instead"
    )]
    pub async fn get_yearly_capital_summary(
        &self,
        organization_id: Uuid,
    ) -> Result<Vec<YearlyCapitalSummary>, sqlx::Error> {
        self.get_yearly_capital_summary_rls(&self.pool, organization_id)
            .await
    }

    /// Generate reserve fund projection.
    ///
    /// Note: This method requires multiple queries and uses the pool directly.
    /// For RLS support, fetch data using individual RLS methods and compute projection in caller.
    #[allow(deprecated)]
    pub async fn generate_reserve_projection(
        &self,
        reserve_fund_id: Uuid,
        years: i32,
    ) -> Result<Vec<ReserveFundProjection>, sqlx::Error> {
        let fund: ReserveFund = sqlx::query_as("SELECT * FROM reserve_funds WHERE id = $1")
            .bind(reserve_fund_id)
            .fetch_one(&self.pool)
            .await?;

        // Get planned capital withdrawals
        let org_id = fund.organization_id;
        let building_id = fund.building_id;

        let plans: Vec<CapitalPlan> = sqlx::query_as(
            r#"
            SELECT * FROM capital_plans
            WHERE organization_id = $1
              AND ($2::uuid IS NULL OR building_id = $2)
              AND funding_source = 'reserve_fund'
              AND status NOT IN ('completed', 'cancelled')
            ORDER BY target_year
            "#,
        )
        .bind(org_id)
        .bind(building_id)
        .fetch_all(&self.pool)
        .await?;

        let current_year = chrono::Utc::now().year();
        let mut projections = Vec::new();
        let mut balance = fund.current_balance;

        for year_offset in 0..years {
            let year = current_year + year_offset;
            let starting_balance = balance;
            let contributions = fund.annual_contribution;

            let planned_withdrawals: Decimal = plans
                .iter()
                .filter(|p| p.target_year == year)
                .map(|p| p.estimated_cost)
                .sum();

            balance = starting_balance + contributions - planned_withdrawals;

            projections.push(ReserveFundProjection {
                year,
                starting_balance,
                contributions,
                planned_withdrawals,
                ending_balance: balance,
            });
        }

        Ok(projections)
    }

    /// Get budget dashboard.
    ///
    /// **Deprecated**: Use individual RLS methods to build dashboard data instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use individual RLS methods to build dashboard data"
    )]
    #[allow(deprecated)]
    pub async fn get_dashboard(
        &self,
        organization_id: Uuid,
        building_id: Option<Uuid>,
    ) -> Result<BudgetDashboard, sqlx::Error> {
        // Get active budget
        let active_budget: Option<Budget> = sqlx::query_as(
            r#"
            SELECT * FROM budgets
            WHERE organization_id = $1
              AND ($2::uuid IS NULL OR building_id = $2)
              AND status = 'active'
            ORDER BY fiscal_year DESC
            LIMIT 1
            "#,
        )
        .bind(organization_id)
        .bind(building_id)
        .fetch_optional(&self.pool)
        .await?;

        let summary = if let Some(ref budget) = active_budget {
            Some(self.get_budget_summary(budget.id).await?)
        } else {
            None
        };

        let categories = if let Some(ref budget) = active_budget {
            self.get_category_variance(budget.id).await?
        } else {
            Vec::new()
        };

        // Count pending alerts
        let pending_alerts: (i64,) = if let Some(ref budget) = active_budget {
            sqlx::query_as(
                r#"
                SELECT COUNT(*) FROM budget_variance_alerts bva
                JOIN budget_items bi ON bi.id = bva.budget_item_id
                WHERE bi.budget_id = $1 AND bva.is_acknowledged = false
                "#,
            )
            .bind(budget.id)
            .fetch_one(&self.pool)
            .await?
        } else {
            (0,)
        };

        // Get total reserve balance
        let reserve_balance: (Decimal,) = sqlx::query_as(
            r#"
            SELECT COALESCE(SUM(current_balance), 0)
            FROM reserve_funds
            WHERE organization_id = $1
              AND ($2::uuid IS NULL OR building_id = $2)
            "#,
        )
        .bind(organization_id)
        .bind(building_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(BudgetDashboard {
            active_budget,
            summary,
            categories,
            pending_alerts: pending_alerts.0,
            reserve_balance: reserve_balance.0,
        })
    }
}
