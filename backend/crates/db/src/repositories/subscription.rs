//! Subscription and billing repository (Epic 26).
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
//! async fn create_subscription(
//!     mut rls: RlsConnection,
//!     State(state): State<AppState>,
//!     Json(data): Json<CreateOrganizationSubscription>,
//! ) -> Result<Json<OrganizationSubscription>> {
//!     let subscription = state.subscription_repo.create_subscription_rls(
//!         rls.conn(), org_id, data
//!     ).await?;
//!     rls.release().await;
//!     Ok(Json(subscription))
//! }
//! ```

use crate::models::{
    CancelSubscriptionRequest, ChangePlanRequest, CouponRedemption, CreateOrganizationSubscription,
    CreateSubscriptionCoupon, CreateSubscriptionEvent, CreateSubscriptionPaymentMethod,
    CreateSubscriptionPlan, CreateUsageRecord, InvoiceLineItem, InvoiceQueryParams,
    InvoiceWithDetails, OrganizationSubscription, PlanSubscriptionCount, SubscriptionCoupon,
    SubscriptionEvent, SubscriptionInvoice, SubscriptionPaymentMethod, SubscriptionPlan,
    SubscriptionStatistics, SubscriptionWithPlan, UpdateOrganizationSubscription,
    UpdateSubscriptionCoupon, UpdateSubscriptionPlan, UsageRecord, UsageSummary,
};
use chrono::{Days, Months, Utc};
use rust_decimal::Decimal;
use sqlx::{Executor, PgPool, Postgres};
use uuid::Uuid;

/// Repository for subscription and billing operations.
#[derive(Clone)]
pub struct SubscriptionRepository {
    pool: PgPool,
}

impl SubscriptionRepository {
    /// Create a new SubscriptionRepository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // RLS-aware methods (recommended)
    // ========================================================================

    // ==================== Subscription Plans CRUD ====================

    /// Create a new subscription plan with RLS context.
    pub async fn create_plan_rls<'e, E>(
        &self,
        executor: E,
        data: CreateSubscriptionPlan,
    ) -> Result<SubscriptionPlan, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            INSERT INTO subscription_plans
                (name, display_name, description, monthly_price, annual_price, currency,
                 max_buildings, max_units, max_users, max_storage_gb, features, trial_days,
                 sort_order, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING *
            "#,
        )
        .bind(&data.name)
        .bind(&data.display_name)
        .bind(&data.description)
        .bind(data.monthly_price)
        .bind(data.annual_price)
        .bind(data.currency.unwrap_or_else(|| "EUR".to_string()))
        .bind(data.max_buildings)
        .bind(data.max_units)
        .bind(data.max_users)
        .bind(data.max_storage_gb)
        .bind(&data.features)
        .bind(data.trial_days)
        .bind(data.sort_order)
        .bind(&data.metadata)
        .fetch_one(executor)
        .await
    }

    /// Find a subscription plan by ID with RLS context.
    pub async fn find_plan_by_id_rls<'e, E>(
        &self,
        executor: E,
        id: Uuid,
    ) -> Result<Option<SubscriptionPlan>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as("SELECT * FROM subscription_plans WHERE id = $1")
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    /// Find a subscription plan by name with RLS context.
    pub async fn find_plan_by_name_rls<'e, E>(
        &self,
        executor: E,
        name: &str,
    ) -> Result<Option<SubscriptionPlan>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as("SELECT * FROM subscription_plans WHERE name = $1")
            .bind(name)
            .fetch_optional(executor)
            .await
    }

    /// List all subscription plans with RLS context.
    pub async fn list_plans_rls<'e, E>(
        &self,
        executor: E,
        active_only: bool,
    ) -> Result<Vec<SubscriptionPlan>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        if active_only {
            sqlx::query_as(
                "SELECT * FROM subscription_plans WHERE is_active = true ORDER BY sort_order, monthly_price",
            )
            .fetch_all(executor)
            .await
        } else {
            sqlx::query_as("SELECT * FROM subscription_plans ORDER BY sort_order, monthly_price")
                .fetch_all(executor)
                .await
        }
    }

    /// List public subscription plans with RLS context.
    pub async fn list_public_plans_rls<'e, E>(
        &self,
        executor: E,
    ) -> Result<Vec<SubscriptionPlan>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            "SELECT * FROM subscription_plans WHERE is_active = true AND is_public = true ORDER BY sort_order, monthly_price",
        )
        .fetch_all(executor)
        .await
    }

    /// Update a subscription plan with RLS context.
    pub async fn update_plan_rls<'e, E>(
        &self,
        executor: E,
        id: Uuid,
        data: UpdateSubscriptionPlan,
    ) -> Result<SubscriptionPlan, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE subscription_plans SET
                display_name = COALESCE($2, display_name),
                description = COALESCE($3, description),
                monthly_price = COALESCE($4, monthly_price),
                annual_price = COALESCE($5, annual_price),
                currency = COALESCE($6, currency),
                max_buildings = COALESCE($7, max_buildings),
                max_units = COALESCE($8, max_units),
                max_users = COALESCE($9, max_users),
                max_storage_gb = COALESCE($10, max_storage_gb),
                features = COALESCE($11, features),
                is_active = COALESCE($12, is_active),
                is_public = COALESCE($13, is_public),
                trial_days = COALESCE($14, trial_days),
                sort_order = COALESCE($15, sort_order),
                metadata = COALESCE($16, metadata),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.display_name)
        .bind(&data.description)
        .bind(data.monthly_price)
        .bind(data.annual_price)
        .bind(&data.currency)
        .bind(data.max_buildings)
        .bind(data.max_units)
        .bind(data.max_users)
        .bind(data.max_storage_gb)
        .bind(&data.features)
        .bind(data.is_active)
        .bind(data.is_public)
        .bind(data.trial_days)
        .bind(data.sort_order)
        .bind(&data.metadata)
        .fetch_one(executor)
        .await
    }

    /// Delete a subscription plan with RLS context.
    pub async fn delete_plan_rls<'e, E>(&self, executor: E, id: Uuid) -> Result<bool, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let result = sqlx::query("DELETE FROM subscription_plans WHERE id = $1")
            .bind(id)
            .execute(executor)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // ==================== Organization Subscriptions CRUD ====================

    /// Create a new organization subscription with RLS context.
    #[allow(deprecated)]
    pub async fn create_subscription_rls<'e, E>(
        &self,
        executor: E,
        org_id: Uuid,
        data: CreateOrganizationSubscription,
    ) -> Result<OrganizationSubscription, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        // Get plan for trial days calculation - use legacy method since we need a separate query
        let plan = self.find_plan_by_id(data.plan_id).await?;

        let billing_cycle = data.billing_cycle.unwrap_or_else(|| "monthly".to_string());
        let now = Utc::now();

        // Calculate period end based on billing cycle using calendar months/years
        let period_end = if billing_cycle == "annual" {
            now.checked_add_months(Months::new(12)).unwrap_or(now)
        } else {
            now.checked_add_months(Months::new(1)).unwrap_or(now)
        };

        // Calculate trial dates if starting trial
        let (trial_start, trial_end, is_trial, status) = if data.start_trial.unwrap_or(false)
            && plan.as_ref().is_some_and(|p| p.trial_days.unwrap_or(0) > 0)
        {
            let trial_days = plan
                .as_ref()
                .map(|p| p.trial_days.unwrap_or(14))
                .unwrap_or(14);
            let trial_end = now
                .checked_add_days(Days::new(trial_days as u64))
                .unwrap_or(now);
            (Some(now), Some(trial_end), Some(true), "trialing")
        } else {
            (None, None, Some(false), "active")
        };

        sqlx::query_as(
            r#"
            INSERT INTO organization_subscriptions
                (organization_id, plan_id, status, billing_cycle, current_period_start,
                 current_period_end, trial_start, trial_end, is_trial, payment_method_id, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#,
        )
        .bind(org_id)
        .bind(data.plan_id)
        .bind(status)
        .bind(&billing_cycle)
        .bind(now)
        .bind(period_end)
        .bind(trial_start)
        .bind(trial_end)
        .bind(is_trial)
        .bind(data.payment_method_id)
        .bind(&data.metadata)
        .fetch_one(executor)
        .await
    }

    /// Find an organization's subscription with RLS context.
    pub async fn find_subscription_by_org_rls<'e, E>(
        &self,
        executor: E,
        org_id: Uuid,
    ) -> Result<Option<OrganizationSubscription>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            "SELECT * FROM organization_subscriptions WHERE organization_id = $1 AND status NOT IN ('cancelled', 'expired') ORDER BY created_at DESC LIMIT 1",
        )
        .bind(org_id)
        .fetch_optional(executor)
        .await
    }

    /// Find a subscription by ID with RLS context.
    pub async fn find_subscription_by_id_rls<'e, E>(
        &self,
        executor: E,
        id: Uuid,
    ) -> Result<Option<OrganizationSubscription>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as("SELECT * FROM organization_subscriptions WHERE id = $1")
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    /// Get subscription with plan details with RLS context.
    pub async fn get_subscription_with_plan_rls<'e, E>(
        &self,
        executor: E,
        org_id: Uuid,
    ) -> Result<Option<SubscriptionWithPlan>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            SELECT
                s.id, s.organization_id, s.status, s.billing_cycle,
                s.current_period_start, s.current_period_end, s.is_trial, s.cancel_at_period_end,
                p.id as plan_id, p.name as plan_name, p.display_name as plan_display_name,
                p.monthly_price, p.annual_price
            FROM organization_subscriptions s
            JOIN subscription_plans p ON p.id = s.plan_id
            WHERE s.organization_id = $1 AND s.status NOT IN ('cancelled', 'expired')
            ORDER BY s.created_at DESC
            LIMIT 1
            "#,
        )
        .bind(org_id)
        .fetch_optional(executor)
        .await
    }

    /// Update an organization subscription with RLS context.
    pub async fn update_subscription_rls<'e, E>(
        &self,
        executor: E,
        id: Uuid,
        data: UpdateOrganizationSubscription,
    ) -> Result<OrganizationSubscription, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE organization_subscriptions SET
                billing_cycle = COALESCE($2, billing_cycle),
                payment_method_id = COALESCE($3, payment_method_id),
                metadata = COALESCE($4, metadata),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.billing_cycle)
        .bind(data.payment_method_id)
        .bind(&data.metadata)
        .fetch_one(executor)
        .await
    }

    /// Change subscription plan with RLS context.
    pub async fn change_plan_rls<'e, E>(
        &self,
        executor: E,
        id: Uuid,
        data: ChangePlanRequest,
    ) -> Result<OrganizationSubscription, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE organization_subscriptions SET
                plan_id = $2,
                billing_cycle = COALESCE($3, billing_cycle),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(data.new_plan_id)
        .bind(&data.billing_cycle)
        .fetch_one(executor)
        .await
    }

    /// Cancel a subscription with RLS context.
    pub async fn cancel_subscription_rls<'e, E>(
        &self,
        executor: E,
        id: Uuid,
        data: CancelSubscriptionRequest,
    ) -> Result<OrganizationSubscription, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let cancel_at_period_end = data.cancel_at_period_end.unwrap_or(true);
        let status = if cancel_at_period_end {
            "active"
        } else {
            "cancelled"
        };

        sqlx::query_as(
            r#"
            UPDATE organization_subscriptions SET
                status = $2,
                cancel_at_period_end = $3,
                cancelled_at = NOW(),
                cancellation_reason = $4,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(cancel_at_period_end)
        .bind(&data.cancellation_reason)
        .fetch_one(executor)
        .await
    }

    /// Reactivate a cancelled subscription with RLS context.
    pub async fn reactivate_subscription_rls<'e, E>(
        &self,
        executor: E,
        id: Uuid,
    ) -> Result<OrganizationSubscription, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE organization_subscriptions SET
                status = 'active',
                cancel_at_period_end = false,
                cancelled_at = NULL,
                cancellation_reason = NULL,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(executor)
        .await
    }

    /// List all subscriptions with RLS context (platform admin).
    pub async fn list_all_subscriptions_rls<'e, E>(
        &self,
        executor: E,
        status: Option<&str>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<SubscriptionWithPlan>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            SELECT
                s.id, s.organization_id, s.status, s.billing_cycle,
                s.current_period_start, s.current_period_end, s.is_trial, s.cancel_at_period_end,
                p.id as plan_id, p.name as plan_name, p.display_name as plan_display_name,
                p.monthly_price, p.annual_price
            FROM organization_subscriptions s
            JOIN subscription_plans p ON p.id = s.plan_id
            WHERE ($1::text IS NULL OR s.status = $1)
            ORDER BY s.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(status)
        .bind(limit)
        .bind(offset)
        .fetch_all(executor)
        .await
    }

    // ==================== Payment Methods CRUD ====================

    /// Create a payment method with RLS context.
    pub async fn create_payment_method_rls<'e, E>(
        &self,
        executor: E,
        org_id: Uuid,
        data: CreateSubscriptionPaymentMethod,
    ) -> Result<SubscriptionPaymentMethod, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            INSERT INTO payment_methods
                (organization_id, method_type, stripe_payment_method_id, is_default,
                 billing_address, metadata)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(org_id)
        .bind(&data.method_type)
        .bind(&data.stripe_payment_method_id)
        .bind(data.is_default.unwrap_or(false))
        .bind(&data.billing_address)
        .bind(&data.metadata)
        .fetch_one(executor)
        .await
    }

    /// List payment methods for an organization with RLS context.
    pub async fn list_payment_methods_rls<'e, E>(
        &self,
        executor: E,
        org_id: Uuid,
    ) -> Result<Vec<SubscriptionPaymentMethod>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            "SELECT * FROM payment_methods WHERE organization_id = $1 ORDER BY is_default DESC, created_at DESC",
        )
        .bind(org_id)
        .fetch_all(executor)
        .await
    }

    /// Delete a payment method with RLS context.
    pub async fn delete_payment_method_rls<'e, E>(
        &self,
        executor: E,
        id: Uuid,
        org_id: Uuid,
    ) -> Result<bool, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let result =
            sqlx::query("DELETE FROM payment_methods WHERE id = $1 AND organization_id = $2")
                .bind(id)
                .bind(org_id)
                .execute(executor)
                .await?;
        Ok(result.rows_affected() > 0)
    }

    // ==================== Invoices ====================

    /// Create an invoice with RLS context.
    ///
    /// Uses a database sequence for atomic invoice number generation to prevent
    /// race conditions and duplicate invoice numbers under concurrent requests.
    #[allow(clippy::too_many_arguments)]
    #[allow(deprecated)]
    pub async fn create_invoice_rls<'e, E>(
        &self,
        executor: E,
        org_id: Uuid,
        subscription_id: Option<Uuid>,
        subtotal: Decimal,
        tax_amount: Option<Decimal>,
        total_amount: Decimal,
        currency: &str,
        due_date: chrono::NaiveDate,
    ) -> Result<SubscriptionInvoice, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        // Generate invoice number atomically using database sequence
        // This prevents race conditions where concurrent requests could get the same number
        // Note: We use the pool for sequence generation as sequences are not affected by RLS
        let seq: (i64,) = sqlx::query_as("SELECT nextval('invoice_number_seq')")
            .fetch_one(&self.pool)
            .await
            .unwrap_or((
                // Fallback: use count + timestamp if sequence doesn't exist
                chrono::Utc::now().timestamp_millis() % 100_000_000,
            ));
        let invoice_number = format!("INV-{:08}", seq.0);

        sqlx::query_as(
            r#"
            INSERT INTO subscription_invoices
                (organization_id, subscription_id, invoice_number, invoice_date, due_date,
                 subtotal, tax_amount, total_amount, currency, status)
            VALUES ($1, $2, $3, CURRENT_DATE, $4, $5, $6, $7, $8, 'open')
            RETURNING *
            "#,
        )
        .bind(org_id)
        .bind(subscription_id)
        .bind(&invoice_number)
        .bind(due_date)
        .bind(subtotal)
        .bind(tax_amount)
        .bind(total_amount)
        .bind(currency)
        .fetch_one(executor)
        .await
    }

    /// Find an invoice by ID with RLS context.
    pub async fn find_invoice_by_id_rls<'e, E>(
        &self,
        executor: E,
        id: Uuid,
    ) -> Result<Option<SubscriptionInvoice>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as("SELECT * FROM subscription_invoices WHERE id = $1")
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    /// List invoices for an organization with RLS context.
    pub async fn list_invoices_rls<'e, E>(
        &self,
        executor: E,
        org_id: Uuid,
        query: InvoiceQueryParams,
    ) -> Result<Vec<SubscriptionInvoice>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            SELECT * FROM subscription_invoices
            WHERE organization_id = $1
            AND ($2::text IS NULL OR status = $2)
            AND ($3::date IS NULL OR invoice_date >= $3)
            AND ($4::date IS NULL OR invoice_date <= $4)
            ORDER BY invoice_date DESC
            LIMIT $5 OFFSET $6
            "#,
        )
        .bind(org_id)
        .bind(&query.status)
        .bind(query.from_date)
        .bind(query.to_date)
        .bind(query.limit.unwrap_or(50))
        .bind(query.offset.unwrap_or(0))
        .fetch_all(executor)
        .await
    }

    /// List invoices with details with RLS context (platform admin).
    pub async fn list_all_invoices_rls<'e, E>(
        &self,
        executor: E,
        query: InvoiceQueryParams,
    ) -> Result<Vec<InvoiceWithDetails>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            SELECT
                id, organization_id, invoice_number, invoice_date, due_date,
                subtotal, tax_amount, total_amount, currency, status, paid_at
            FROM subscription_invoices
            WHERE ($1::text IS NULL OR status = $1)
            AND ($2::date IS NULL OR invoice_date >= $2)
            AND ($3::date IS NULL OR invoice_date <= $3)
            ORDER BY invoice_date DESC
            LIMIT $4 OFFSET $5
            "#,
        )
        .bind(&query.status)
        .bind(query.from_date)
        .bind(query.to_date)
        .bind(query.limit.unwrap_or(50))
        .bind(query.offset.unwrap_or(0))
        .fetch_all(executor)
        .await
    }

    /// Mark invoice as paid with RLS context.
    pub async fn mark_invoice_paid_rls<'e, E>(
        &self,
        executor: E,
        id: Uuid,
        payment_method_id: Option<Uuid>,
    ) -> Result<SubscriptionInvoice, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE subscription_invoices SET
                status = 'paid',
                paid_at = NOW(),
                payment_method_id = $2,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(payment_method_id)
        .fetch_one(executor)
        .await
    }

    /// Void an invoice with RLS context.
    pub async fn void_invoice_rls<'e, E>(
        &self,
        executor: E,
        id: Uuid,
    ) -> Result<SubscriptionInvoice, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE subscription_invoices SET
                status = 'void',
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(executor)
        .await
    }

    /// Add line items to an invoice with RLS context.
    #[allow(clippy::too_many_arguments)]
    pub async fn add_invoice_line_item_rls<'e, E>(
        &self,
        executor: E,
        invoice_id: Uuid,
        description: &str,
        quantity: Option<Decimal>,
        unit_price: Decimal,
        amount: Decimal,
        item_type: &str,
        plan_id: Option<Uuid>,
    ) -> Result<InvoiceLineItem, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            INSERT INTO invoice_line_items
                (invoice_id, description, quantity, unit_price, amount, item_type, plan_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(invoice_id)
        .bind(description)
        .bind(quantity)
        .bind(unit_price)
        .bind(amount)
        .bind(item_type)
        .bind(plan_id)
        .fetch_one(executor)
        .await
    }

    /// Get line items for an invoice with RLS context.
    pub async fn get_invoice_line_items_rls<'e, E>(
        &self,
        executor: E,
        invoice_id: Uuid,
    ) -> Result<Vec<InvoiceLineItem>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as("SELECT * FROM invoice_line_items WHERE invoice_id = $1 ORDER BY created_at")
            .bind(invoice_id)
            .fetch_all(executor)
            .await
    }

    // ==================== Usage Records ====================

    /// Record a usage metric with RLS context.
    pub async fn record_usage_rls<'e, E>(
        &self,
        executor: E,
        org_id: Uuid,
        subscription_id: Option<Uuid>,
        data: CreateUsageRecord,
    ) -> Result<UsageRecord, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            INSERT INTO usage_records
                (organization_id, subscription_id, metric_type, quantity, unit, metadata)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(org_id)
        .bind(subscription_id)
        .bind(&data.metric_type)
        .bind(data.quantity)
        .bind(&data.unit)
        .bind(&data.metadata)
        .fetch_one(executor)
        .await
    }

    /// Get usage summary for an organization with RLS context.
    pub async fn get_usage_summary_rls<'e, E>(
        &self,
        executor: E,
        org_id: Uuid,
        period_start: chrono::DateTime<Utc>,
        period_end: chrono::DateTime<Utc>,
    ) -> Result<Vec<UsageSummary>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            SELECT
                metric_type,
                SUM(quantity) as total_quantity,
                MAX(unit) as unit,
                COUNT(*) as record_count
            FROM usage_records
            WHERE organization_id = $1
            AND recorded_at >= $2 AND recorded_at < $3
            GROUP BY metric_type
            "#,
        )
        .bind(org_id)
        .bind(period_start)
        .bind(period_end)
        .fetch_all(executor)
        .await
    }

    /// Get current usage counts for an organization with RLS context.
    pub async fn get_current_usage_rls<'e, E>(
        &self,
        executor: E,
        org_id: Uuid,
    ) -> Result<(i64, i64, i64, i64), sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        // Combined query to get all counts in one round-trip
        let counts: (i64, i64, i64) = sqlx::query_as(
            r#"
            SELECT
                (SELECT COUNT(*) FROM buildings WHERE organization_id = $1) as building_count,
                (SELECT COUNT(*) FROM units u JOIN buildings b ON u.building_id = b.id WHERE b.organization_id = $1) as unit_count,
                (SELECT COUNT(*) FROM organization_members WHERE organization_id = $1) as user_count
            "#,
        )
        .bind(org_id)
        .fetch_one(executor)
        .await?;

        // Storage would require summing document sizes - using 0 as placeholder
        let storage = 0i64;

        Ok((counts.0, counts.1, counts.2, storage))
    }

    // ==================== Subscription Events ====================

    /// Log a subscription event with RLS context.
    pub async fn log_event_rls<'e, E>(
        &self,
        executor: E,
        org_id: Uuid,
        subscription_id: Option<Uuid>,
        actor_id: Option<Uuid>,
        data: CreateSubscriptionEvent,
    ) -> Result<SubscriptionEvent, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            INSERT INTO subscription_events
                (organization_id, subscription_id, event_type, description, actor_id,
                 previous_data, new_data, webhook_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(org_id)
        .bind(subscription_id)
        .bind(&data.event_type)
        .bind(&data.description)
        .bind(actor_id)
        .bind(&data.previous_data)
        .bind(&data.new_data)
        .bind(&data.webhook_id)
        .fetch_one(executor)
        .await
    }

    /// Get subscription events with RLS context.
    pub async fn get_events_rls<'e, E>(
        &self,
        executor: E,
        org_id: Uuid,
        limit: i32,
    ) -> Result<Vec<SubscriptionEvent>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            "SELECT * FROM subscription_events WHERE organization_id = $1 ORDER BY created_at DESC LIMIT $2",
        )
        .bind(org_id)
        .bind(limit)
        .fetch_all(executor)
        .await
    }

    // ==================== Coupons ====================

    /// Create a coupon with RLS context.
    pub async fn create_coupon_rls<'e, E>(
        &self,
        executor: E,
        data: CreateSubscriptionCoupon,
    ) -> Result<SubscriptionCoupon, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            INSERT INTO subscription_coupons
                (code, name, description, discount_type, discount_value, currency, duration,
                 duration_months, max_redemptions, valid_from, valid_until, applicable_plans,
                 min_amount, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING *
            "#,
        )
        .bind(&data.code)
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.discount_type)
        .bind(data.discount_value)
        .bind(&data.currency)
        .bind(data.duration.unwrap_or_else(|| "once".to_string()))
        .bind(data.duration_months)
        .bind(data.max_redemptions)
        .bind(data.valid_from)
        .bind(data.valid_until)
        .bind(&data.applicable_plans)
        .bind(data.min_amount)
        .bind(&data.metadata)
        .fetch_one(executor)
        .await
    }

    /// Find a coupon by code with RLS context.
    pub async fn find_coupon_by_code_rls<'e, E>(
        &self,
        executor: E,
        code: &str,
    ) -> Result<Option<SubscriptionCoupon>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as("SELECT * FROM subscription_coupons WHERE code = $1 AND is_active = true")
            .bind(code)
            .fetch_optional(executor)
            .await
    }

    /// List all coupons with RLS context.
    pub async fn list_coupons_rls<'e, E>(
        &self,
        executor: E,
        active_only: bool,
    ) -> Result<Vec<SubscriptionCoupon>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        if active_only {
            sqlx::query_as(
                "SELECT * FROM subscription_coupons WHERE is_active = true ORDER BY created_at DESC",
            )
            .fetch_all(executor)
            .await
        } else {
            sqlx::query_as("SELECT * FROM subscription_coupons ORDER BY created_at DESC")
                .fetch_all(executor)
                .await
        }
    }

    /// Update a coupon with RLS context.
    pub async fn update_coupon_rls<'e, E>(
        &self,
        executor: E,
        id: Uuid,
        data: UpdateSubscriptionCoupon,
    ) -> Result<SubscriptionCoupon, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as(
            r#"
            UPDATE subscription_coupons SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                max_redemptions = COALESCE($4, max_redemptions),
                valid_from = COALESCE($5, valid_from),
                valid_until = COALESCE($6, valid_until),
                applicable_plans = COALESCE($7, applicable_plans),
                min_amount = COALESCE($8, min_amount),
                is_active = COALESCE($9, is_active),
                metadata = COALESCE($10, metadata),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.max_redemptions)
        .bind(data.valid_from)
        .bind(data.valid_until)
        .bind(&data.applicable_plans)
        .bind(data.min_amount)
        .bind(data.is_active)
        .bind(&data.metadata)
        .fetch_one(executor)
        .await
    }

    // ==================== Statistics ====================

    /// Get subscription statistics with RLS context.
    pub async fn get_statistics_rls<'e, E>(
        &self,
        executor: E,
    ) -> Result<SubscriptionStatistics, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        // Combined query to get all stats in one round-trip
        let stats: (i64, i64, i64, i64, Option<Decimal>) = sqlx::query_as(
            r#"
            SELECT
                (SELECT COUNT(*) FROM organization_subscriptions) as total,
                (SELECT COUNT(*) FROM organization_subscriptions WHERE status = 'active') as active,
                (SELECT COUNT(*) FROM organization_subscriptions WHERE status = 'trialing') as trial,
                (SELECT COUNT(*) FROM organization_subscriptions WHERE status = 'cancelled') as cancelled,
                (SELECT SUM(
                    CASE WHEN s.billing_cycle = 'annual'
                        THEN p.annual_price / 12
                        ELSE p.monthly_price
                    END
                )
                FROM organization_subscriptions s
                JOIN subscription_plans p ON p.id = s.plan_id
                WHERE s.status = 'active') as mrr
            "#,
        )
        .fetch_one(executor)
        .await?;

        let monthly_recurring_revenue = stats.4.unwrap_or(Decimal::ZERO);
        let annual_recurring_revenue = monthly_recurring_revenue * Decimal::from(12);

        // Get counts by plan - this requires a separate query
        let by_plan: Vec<PlanSubscriptionCount> = sqlx::query_as(
            r#"
            SELECT p.id as plan_id, p.name as plan_name, COUNT(s.id) as count
            FROM subscription_plans p
            LEFT JOIN organization_subscriptions s ON s.plan_id = p.id AND s.status = 'active'
            GROUP BY p.id, p.name
            ORDER BY count DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(SubscriptionStatistics {
            total_subscriptions: stats.0,
            active_subscriptions: stats.1,
            trial_subscriptions: stats.2,
            cancelled_subscriptions: stats.3,
            monthly_recurring_revenue,
            annual_recurring_revenue,
            by_plan,
        })
    }

    // ========================================================================
    // Legacy methods (use pool directly - migrate to RLS versions)
    // ========================================================================

    // ==================== Subscription Plans CRUD ====================

    /// Create a new subscription plan.
    ///
    /// **Deprecated**: Use `create_plan_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use create_plan_rls with RlsConnection instead"
    )]
    pub async fn create_plan(
        &self,
        data: CreateSubscriptionPlan,
    ) -> Result<SubscriptionPlan, sqlx::Error> {
        self.create_plan_rls(&self.pool, data).await
    }

    /// Find a subscription plan by ID.
    ///
    /// **Deprecated**: Use `find_plan_by_id_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use find_plan_by_id_rls with RlsConnection instead"
    )]
    pub async fn find_plan_by_id(&self, id: Uuid) -> Result<Option<SubscriptionPlan>, sqlx::Error> {
        self.find_plan_by_id_rls(&self.pool, id).await
    }

    /// Find a subscription plan by name.
    ///
    /// **Deprecated**: Use `find_plan_by_name_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use find_plan_by_name_rls with RlsConnection instead"
    )]
    pub async fn find_plan_by_name(
        &self,
        name: &str,
    ) -> Result<Option<SubscriptionPlan>, sqlx::Error> {
        self.find_plan_by_name_rls(&self.pool, name).await
    }

    /// List all subscription plans.
    ///
    /// **Deprecated**: Use `list_plans_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use list_plans_rls with RlsConnection instead"
    )]
    pub async fn list_plans(
        &self,
        active_only: bool,
    ) -> Result<Vec<SubscriptionPlan>, sqlx::Error> {
        self.list_plans_rls(&self.pool, active_only).await
    }

    /// List public subscription plans (for display to customers).
    ///
    /// **Deprecated**: Use `list_public_plans_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use list_public_plans_rls with RlsConnection instead"
    )]
    pub async fn list_public_plans(&self) -> Result<Vec<SubscriptionPlan>, sqlx::Error> {
        self.list_public_plans_rls(&self.pool).await
    }

    /// Update a subscription plan.
    ///
    /// **Deprecated**: Use `update_plan_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use update_plan_rls with RlsConnection instead"
    )]
    pub async fn update_plan(
        &self,
        id: Uuid,
        data: UpdateSubscriptionPlan,
    ) -> Result<SubscriptionPlan, sqlx::Error> {
        self.update_plan_rls(&self.pool, id, data).await
    }

    /// Delete a subscription plan.
    ///
    /// **Deprecated**: Use `delete_plan_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use delete_plan_rls with RlsConnection instead"
    )]
    pub async fn delete_plan(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        self.delete_plan_rls(&self.pool, id).await
    }

    // ==================== Organization Subscriptions CRUD ====================

    /// Create a new organization subscription.
    ///
    /// **Deprecated**: Use `create_subscription_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use create_subscription_rls with RlsConnection instead"
    )]
    #[allow(deprecated)]
    pub async fn create_subscription(
        &self,
        org_id: Uuid,
        data: CreateOrganizationSubscription,
    ) -> Result<OrganizationSubscription, sqlx::Error> {
        self.create_subscription_rls(&self.pool, org_id, data).await
    }

    /// Find an organization's subscription.
    ///
    /// **Deprecated**: Use `find_subscription_by_org_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use find_subscription_by_org_rls with RlsConnection instead"
    )]
    pub async fn find_subscription_by_org(
        &self,
        org_id: Uuid,
    ) -> Result<Option<OrganizationSubscription>, sqlx::Error> {
        self.find_subscription_by_org_rls(&self.pool, org_id).await
    }

    /// Find a subscription by ID.
    ///
    /// **Deprecated**: Use `find_subscription_by_id_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use find_subscription_by_id_rls with RlsConnection instead"
    )]
    pub async fn find_subscription_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<OrganizationSubscription>, sqlx::Error> {
        self.find_subscription_by_id_rls(&self.pool, id).await
    }

    /// Get subscription with plan details.
    ///
    /// **Deprecated**: Use `get_subscription_with_plan_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use get_subscription_with_plan_rls with RlsConnection instead"
    )]
    pub async fn get_subscription_with_plan(
        &self,
        org_id: Uuid,
    ) -> Result<Option<SubscriptionWithPlan>, sqlx::Error> {
        self.get_subscription_with_plan_rls(&self.pool, org_id)
            .await
    }

    /// Update an organization subscription.
    ///
    /// **Deprecated**: Use `update_subscription_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use update_subscription_rls with RlsConnection instead"
    )]
    pub async fn update_subscription(
        &self,
        id: Uuid,
        data: UpdateOrganizationSubscription,
    ) -> Result<OrganizationSubscription, sqlx::Error> {
        self.update_subscription_rls(&self.pool, id, data).await
    }

    /// Change subscription plan.
    ///
    /// **Deprecated**: Use `change_plan_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use change_plan_rls with RlsConnection instead"
    )]
    pub async fn change_plan(
        &self,
        id: Uuid,
        data: ChangePlanRequest,
    ) -> Result<OrganizationSubscription, sqlx::Error> {
        self.change_plan_rls(&self.pool, id, data).await
    }

    /// Cancel a subscription.
    ///
    /// **Deprecated**: Use `cancel_subscription_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use cancel_subscription_rls with RlsConnection instead"
    )]
    pub async fn cancel_subscription(
        &self,
        id: Uuid,
        data: CancelSubscriptionRequest,
    ) -> Result<OrganizationSubscription, sqlx::Error> {
        self.cancel_subscription_rls(&self.pool, id, data).await
    }

    /// Reactivate a cancelled subscription.
    ///
    /// **Deprecated**: Use `reactivate_subscription_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use reactivate_subscription_rls with RlsConnection instead"
    )]
    pub async fn reactivate_subscription(
        &self,
        id: Uuid,
    ) -> Result<OrganizationSubscription, sqlx::Error> {
        self.reactivate_subscription_rls(&self.pool, id).await
    }

    /// List all subscriptions (platform admin).
    ///
    /// **Deprecated**: Use `list_all_subscriptions_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use list_all_subscriptions_rls with RlsConnection instead"
    )]
    pub async fn list_all_subscriptions(
        &self,
        status: Option<&str>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<SubscriptionWithPlan>, sqlx::Error> {
        self.list_all_subscriptions_rls(&self.pool, status, limit, offset)
            .await
    }

    // ==================== Payment Methods CRUD ====================

    /// Create a payment method.
    ///
    /// **Deprecated**: Use `create_payment_method_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use create_payment_method_rls with RlsConnection instead"
    )]
    pub async fn create_payment_method(
        &self,
        org_id: Uuid,
        data: CreateSubscriptionPaymentMethod,
    ) -> Result<SubscriptionPaymentMethod, sqlx::Error> {
        self.create_payment_method_rls(&self.pool, org_id, data)
            .await
    }

    /// List payment methods for an organization.
    ///
    /// **Deprecated**: Use `list_payment_methods_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use list_payment_methods_rls with RlsConnection instead"
    )]
    pub async fn list_payment_methods(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<SubscriptionPaymentMethod>, sqlx::Error> {
        self.list_payment_methods_rls(&self.pool, org_id).await
    }

    /// Set default payment method.
    ///
    /// Uses a transaction to ensure atomicity - prevents race conditions where
    /// concurrent requests could result in multiple default payment methods.
    ///
    /// Note: This method uses transactions internally and cannot be easily
    /// converted to an RLS-aware pattern. Consider using a stored procedure
    /// or handling the transaction at a higher level.
    pub async fn set_default_payment_method(
        &self,
        org_id: Uuid,
        payment_method_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Reset all to non-default
        sqlx::query("UPDATE payment_methods SET is_default = false WHERE organization_id = $1")
            .bind(org_id)
            .execute(&mut *tx)
            .await?;

        // Set the specified one as default
        sqlx::query(
            "UPDATE payment_methods SET is_default = true WHERE id = $1 AND organization_id = $2",
        )
        .bind(payment_method_id)
        .bind(org_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    /// Delete a payment method.
    ///
    /// **Deprecated**: Use `delete_payment_method_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use delete_payment_method_rls with RlsConnection instead"
    )]
    pub async fn delete_payment_method(&self, id: Uuid, org_id: Uuid) -> Result<bool, sqlx::Error> {
        self.delete_payment_method_rls(&self.pool, id, org_id).await
    }

    // ==================== Invoices ====================

    /// Create an invoice.
    ///
    /// Uses a database sequence for atomic invoice number generation to prevent
    /// race conditions and duplicate invoice numbers under concurrent requests.
    ///
    /// **Deprecated**: Use `create_invoice_rls` with an RLS-enabled connection instead.
    #[allow(clippy::too_many_arguments)]
    #[deprecated(
        since = "0.2.276",
        note = "Use create_invoice_rls with RlsConnection instead"
    )]
    #[allow(deprecated)]
    pub async fn create_invoice(
        &self,
        org_id: Uuid,
        subscription_id: Option<Uuid>,
        subtotal: Decimal,
        tax_amount: Option<Decimal>,
        total_amount: Decimal,
        currency: &str,
        due_date: chrono::NaiveDate,
    ) -> Result<SubscriptionInvoice, sqlx::Error> {
        self.create_invoice_rls(
            &self.pool,
            org_id,
            subscription_id,
            subtotal,
            tax_amount,
            total_amount,
            currency,
            due_date,
        )
        .await
    }

    /// Find an invoice by ID.
    ///
    /// **Deprecated**: Use `find_invoice_by_id_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use find_invoice_by_id_rls with RlsConnection instead"
    )]
    pub async fn find_invoice_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<SubscriptionInvoice>, sqlx::Error> {
        self.find_invoice_by_id_rls(&self.pool, id).await
    }

    /// List invoices for an organization.
    ///
    /// **Deprecated**: Use `list_invoices_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use list_invoices_rls with RlsConnection instead"
    )]
    pub async fn list_invoices(
        &self,
        org_id: Uuid,
        query: InvoiceQueryParams,
    ) -> Result<Vec<SubscriptionInvoice>, sqlx::Error> {
        self.list_invoices_rls(&self.pool, org_id, query).await
    }

    /// List invoices with details (platform admin).
    ///
    /// **Deprecated**: Use `list_all_invoices_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use list_all_invoices_rls with RlsConnection instead"
    )]
    pub async fn list_all_invoices(
        &self,
        query: InvoiceQueryParams,
    ) -> Result<Vec<InvoiceWithDetails>, sqlx::Error> {
        self.list_all_invoices_rls(&self.pool, query).await
    }

    /// Mark invoice as paid.
    ///
    /// **Deprecated**: Use `mark_invoice_paid_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use mark_invoice_paid_rls with RlsConnection instead"
    )]
    pub async fn mark_invoice_paid(
        &self,
        id: Uuid,
        payment_method_id: Option<Uuid>,
    ) -> Result<SubscriptionInvoice, sqlx::Error> {
        self.mark_invoice_paid_rls(&self.pool, id, payment_method_id)
            .await
    }

    /// Void an invoice.
    ///
    /// **Deprecated**: Use `void_invoice_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use void_invoice_rls with RlsConnection instead"
    )]
    pub async fn void_invoice(&self, id: Uuid) -> Result<SubscriptionInvoice, sqlx::Error> {
        self.void_invoice_rls(&self.pool, id).await
    }

    /// Add line items to an invoice.
    ///
    /// **Deprecated**: Use `add_invoice_line_item_rls` with an RLS-enabled connection instead.
    #[allow(clippy::too_many_arguments)]
    #[deprecated(
        since = "0.2.276",
        note = "Use add_invoice_line_item_rls with RlsConnection instead"
    )]
    pub async fn add_invoice_line_item(
        &self,
        invoice_id: Uuid,
        description: &str,
        quantity: Option<Decimal>,
        unit_price: Decimal,
        amount: Decimal,
        item_type: &str,
        plan_id: Option<Uuid>,
    ) -> Result<InvoiceLineItem, sqlx::Error> {
        self.add_invoice_line_item_rls(
            &self.pool,
            invoice_id,
            description,
            quantity,
            unit_price,
            amount,
            item_type,
            plan_id,
        )
        .await
    }

    /// Get line items for an invoice.
    ///
    /// **Deprecated**: Use `get_invoice_line_items_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use get_invoice_line_items_rls with RlsConnection instead"
    )]
    pub async fn get_invoice_line_items(
        &self,
        invoice_id: Uuid,
    ) -> Result<Vec<InvoiceLineItem>, sqlx::Error> {
        self.get_invoice_line_items_rls(&self.pool, invoice_id)
            .await
    }

    // ==================== Usage Records ====================

    /// Record a usage metric.
    ///
    /// **Deprecated**: Use `record_usage_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use record_usage_rls with RlsConnection instead"
    )]
    pub async fn record_usage(
        &self,
        org_id: Uuid,
        subscription_id: Option<Uuid>,
        data: CreateUsageRecord,
    ) -> Result<UsageRecord, sqlx::Error> {
        self.record_usage_rls(&self.pool, org_id, subscription_id, data)
            .await
    }

    /// Get usage summary for an organization.
    ///
    /// **Deprecated**: Use `get_usage_summary_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use get_usage_summary_rls with RlsConnection instead"
    )]
    pub async fn get_usage_summary(
        &self,
        org_id: Uuid,
        period_start: chrono::DateTime<Utc>,
        period_end: chrono::DateTime<Utc>,
    ) -> Result<Vec<UsageSummary>, sqlx::Error> {
        self.get_usage_summary_rls(&self.pool, org_id, period_start, period_end)
            .await
    }

    /// Get current usage counts for an organization.
    ///
    /// **Deprecated**: Use `get_current_usage_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use get_current_usage_rls with RlsConnection instead"
    )]
    pub async fn get_current_usage(
        &self,
        org_id: Uuid,
    ) -> Result<(i64, i64, i64, i64), sqlx::Error> {
        self.get_current_usage_rls(&self.pool, org_id).await
    }

    // ==================== Subscription Events ====================

    /// Log a subscription event.
    ///
    /// **Deprecated**: Use `log_event_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use log_event_rls with RlsConnection instead"
    )]
    pub async fn log_event(
        &self,
        org_id: Uuid,
        subscription_id: Option<Uuid>,
        actor_id: Option<Uuid>,
        data: CreateSubscriptionEvent,
    ) -> Result<SubscriptionEvent, sqlx::Error> {
        self.log_event_rls(&self.pool, org_id, subscription_id, actor_id, data)
            .await
    }

    /// Get subscription events.
    ///
    /// **Deprecated**: Use `get_events_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use get_events_rls with RlsConnection instead"
    )]
    pub async fn get_events(
        &self,
        org_id: Uuid,
        limit: i32,
    ) -> Result<Vec<SubscriptionEvent>, sqlx::Error> {
        self.get_events_rls(&self.pool, org_id, limit).await
    }

    // ==================== Coupons ====================

    /// Create a coupon.
    ///
    /// **Deprecated**: Use `create_coupon_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use create_coupon_rls with RlsConnection instead"
    )]
    pub async fn create_coupon(
        &self,
        data: CreateSubscriptionCoupon,
    ) -> Result<SubscriptionCoupon, sqlx::Error> {
        self.create_coupon_rls(&self.pool, data).await
    }

    /// Find a coupon by code.
    ///
    /// **Deprecated**: Use `find_coupon_by_code_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use find_coupon_by_code_rls with RlsConnection instead"
    )]
    pub async fn find_coupon_by_code(
        &self,
        code: &str,
    ) -> Result<Option<SubscriptionCoupon>, sqlx::Error> {
        self.find_coupon_by_code_rls(&self.pool, code).await
    }

    /// List all coupons.
    ///
    /// **Deprecated**: Use `list_coupons_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use list_coupons_rls with RlsConnection instead"
    )]
    pub async fn list_coupons(
        &self,
        active_only: bool,
    ) -> Result<Vec<SubscriptionCoupon>, sqlx::Error> {
        self.list_coupons_rls(&self.pool, active_only).await
    }

    /// Update a coupon.
    ///
    /// **Deprecated**: Use `update_coupon_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use update_coupon_rls with RlsConnection instead"
    )]
    pub async fn update_coupon(
        &self,
        id: Uuid,
        data: UpdateSubscriptionCoupon,
    ) -> Result<SubscriptionCoupon, sqlx::Error> {
        self.update_coupon_rls(&self.pool, id, data).await
    }

    /// Redeem a coupon.
    ///
    /// Uses a transaction with validation to prevent race conditions and over-redemption.
    /// Checks max_redemptions before incrementing count and inserting redemption record.
    ///
    /// Note: This method uses transactions internally and cannot be easily
    /// converted to an RLS-aware pattern. Consider using a stored procedure
    /// or handling the transaction at a higher level.
    pub async fn redeem_coupon(
        &self,
        coupon_id: Uuid,
        org_id: Uuid,
        subscription_id: Option<Uuid>,
        user_id: Uuid,
    ) -> Result<CouponRedemption, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Check if coupon exists and has remaining redemptions (with row lock)
        let coupon: Option<(i32, Option<i32>)> = sqlx::query_as(
            "SELECT COALESCE(redemption_count, 0), max_redemptions FROM subscription_coupons WHERE id = $1 FOR UPDATE",
        )
        .bind(coupon_id)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some((current_count, max_redemptions)) = coupon {
            if let Some(max) = max_redemptions {
                if current_count >= max {
                    tx.rollback().await?;
                    return Err(sqlx::Error::RowNotFound); // Coupon exhausted
                }
            }
        } else {
            tx.rollback().await?;
            return Err(sqlx::Error::RowNotFound); // Coupon not found
        }

        // Insert redemption record first (this validates FK constraints)
        let redemption: CouponRedemption = sqlx::query_as(
            r#"
            INSERT INTO coupon_redemptions
                (coupon_id, organization_id, subscription_id, redeemed_by)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(coupon_id)
        .bind(org_id)
        .bind(subscription_id)
        .bind(user_id)
        .fetch_one(&mut *tx)
        .await?;

        // Only increment count after successful redemption insert
        sqlx::query(
            "UPDATE subscription_coupons SET redemption_count = COALESCE(redemption_count, 0) + 1 WHERE id = $1",
        )
        .bind(coupon_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(redemption)
    }

    // ==================== Statistics ====================

    /// Get subscription statistics.
    ///
    /// **Deprecated**: Use `get_statistics_rls` with an RLS-enabled connection instead.
    #[deprecated(
        since = "0.2.276",
        note = "Use get_statistics_rls with RlsConnection instead"
    )]
    #[allow(deprecated)]
    pub async fn get_statistics(&self) -> Result<SubscriptionStatistics, sqlx::Error> {
        self.get_statistics_rls(&self.pool).await
    }
}
