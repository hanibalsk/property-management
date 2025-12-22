//! Subscription and billing repository (Epic 26).

use crate::models::{
    CancelSubscriptionRequest, ChangePlanRequest, CouponRedemption, CreateOrganizationSubscription,
    CreateSubscriptionCoupon, CreateSubscriptionEvent, CreateSubscriptionPaymentMethod,
    CreateSubscriptionPlan, CreateUsageRecord, InvoiceLineItem, InvoiceQueryParams,
    InvoiceWithDetails, OrganizationSubscription, PlanSubscriptionCount, SubscriptionCoupon,
    SubscriptionEvent, SubscriptionInvoice, SubscriptionPaymentMethod, SubscriptionPlan,
    SubscriptionStatistics, SubscriptionWithPlan, UpdateOrganizationSubscription,
    UpdateSubscriptionCoupon, UpdateSubscriptionPlan, UsageRecord, UsageSummary,
};
use chrono::{Days, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;
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

    // ==================== Subscription Plans CRUD ====================

    /// Create a new subscription plan.
    pub async fn create_plan(
        &self,
        data: CreateSubscriptionPlan,
    ) -> Result<SubscriptionPlan, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    /// Find a subscription plan by ID.
    pub async fn find_plan_by_id(&self, id: Uuid) -> Result<Option<SubscriptionPlan>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM subscription_plans WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    /// Find a subscription plan by name.
    pub async fn find_plan_by_name(
        &self,
        name: &str,
    ) -> Result<Option<SubscriptionPlan>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM subscription_plans WHERE name = $1")
            .bind(name)
            .fetch_optional(&self.pool)
            .await
    }

    /// List all subscription plans.
    pub async fn list_plans(
        &self,
        active_only: bool,
    ) -> Result<Vec<SubscriptionPlan>, sqlx::Error> {
        if active_only {
            sqlx::query_as(
                "SELECT * FROM subscription_plans WHERE is_active = true ORDER BY sort_order, monthly_price",
            )
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as("SELECT * FROM subscription_plans ORDER BY sort_order, monthly_price")
                .fetch_all(&self.pool)
                .await
        }
    }

    /// List public subscription plans (for display to customers).
    pub async fn list_public_plans(&self) -> Result<Vec<SubscriptionPlan>, sqlx::Error> {
        sqlx::query_as(
            "SELECT * FROM subscription_plans WHERE is_active = true AND is_public = true ORDER BY sort_order, monthly_price",
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Update a subscription plan.
    pub async fn update_plan(
        &self,
        id: Uuid,
        data: UpdateSubscriptionPlan,
    ) -> Result<SubscriptionPlan, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    /// Delete a subscription plan.
    pub async fn delete_plan(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM subscription_plans WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // ==================== Organization Subscriptions CRUD ====================

    /// Create a new organization subscription.
    pub async fn create_subscription(
        &self,
        org_id: Uuid,
        data: CreateOrganizationSubscription,
    ) -> Result<OrganizationSubscription, sqlx::Error> {
        // Get plan for trial days calculation
        let plan = self.find_plan_by_id(data.plan_id).await?;

        let billing_cycle = data.billing_cycle.unwrap_or_else(|| "monthly".to_string());
        let now = Utc::now();

        // Calculate period end based on billing cycle
        let period_end = if billing_cycle == "annual" {
            now.checked_add_days(Days::new(365)).unwrap_or(now)
        } else {
            now.checked_add_days(Days::new(30)).unwrap_or(now)
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
        .fetch_one(&self.pool)
        .await
    }

    /// Find an organization's subscription.
    pub async fn find_subscription_by_org(
        &self,
        org_id: Uuid,
    ) -> Result<Option<OrganizationSubscription>, sqlx::Error> {
        sqlx::query_as(
            "SELECT * FROM organization_subscriptions WHERE organization_id = $1 AND status NOT IN ('cancelled', 'expired') ORDER BY created_at DESC LIMIT 1",
        )
        .bind(org_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Find a subscription by ID.
    pub async fn find_subscription_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<OrganizationSubscription>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM organization_subscriptions WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    /// Get subscription with plan details.
    pub async fn get_subscription_with_plan(
        &self,
        org_id: Uuid,
    ) -> Result<Option<SubscriptionWithPlan>, sqlx::Error> {
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
        .fetch_optional(&self.pool)
        .await
    }

    /// Update an organization subscription.
    pub async fn update_subscription(
        &self,
        id: Uuid,
        data: UpdateOrganizationSubscription,
    ) -> Result<OrganizationSubscription, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    /// Change subscription plan.
    pub async fn change_plan(
        &self,
        id: Uuid,
        data: ChangePlanRequest,
    ) -> Result<OrganizationSubscription, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    /// Cancel a subscription.
    pub async fn cancel_subscription(
        &self,
        id: Uuid,
        data: CancelSubscriptionRequest,
    ) -> Result<OrganizationSubscription, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    /// Reactivate a cancelled subscription.
    pub async fn reactivate_subscription(
        &self,
        id: Uuid,
    ) -> Result<OrganizationSubscription, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    /// List all subscriptions (platform admin).
    pub async fn list_all_subscriptions(
        &self,
        status: Option<&str>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<SubscriptionWithPlan>, sqlx::Error> {
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
        .fetch_all(&self.pool)
        .await
    }

    // ==================== Payment Methods CRUD ====================

    /// Create a payment method.
    pub async fn create_payment_method(
        &self,
        org_id: Uuid,
        data: CreateSubscriptionPaymentMethod,
    ) -> Result<SubscriptionPaymentMethod, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    /// List payment methods for an organization.
    pub async fn list_payment_methods(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<SubscriptionPaymentMethod>, sqlx::Error> {
        sqlx::query_as(
            "SELECT * FROM payment_methods WHERE organization_id = $1 ORDER BY is_default DESC, created_at DESC",
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Set default payment method.
    pub async fn set_default_payment_method(
        &self,
        org_id: Uuid,
        payment_method_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        // Reset all to non-default
        sqlx::query("UPDATE payment_methods SET is_default = false WHERE organization_id = $1")
            .bind(org_id)
            .execute(&self.pool)
            .await?;

        // Set the specified one as default
        sqlx::query(
            "UPDATE payment_methods SET is_default = true WHERE id = $1 AND organization_id = $2",
        )
        .bind(payment_method_id)
        .bind(org_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Delete a payment method.
    pub async fn delete_payment_method(&self, id: Uuid, org_id: Uuid) -> Result<bool, sqlx::Error> {
        let result =
            sqlx::query("DELETE FROM payment_methods WHERE id = $1 AND organization_id = $2")
                .bind(id)
                .bind(org_id)
                .execute(&self.pool)
                .await?;
        Ok(result.rows_affected() > 0)
    }

    // ==================== Invoices ====================

    /// Create an invoice.
    #[allow(clippy::too_many_arguments)]
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
        // Generate invoice number
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM subscription_invoices")
            .fetch_one(&self.pool)
            .await?;
        let invoice_number = format!("INV-{:08}", count.0 + 1);

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
        .fetch_one(&self.pool)
        .await
    }

    /// Find an invoice by ID.
    pub async fn find_invoice_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<SubscriptionInvoice>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM subscription_invoices WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    /// List invoices for an organization.
    pub async fn list_invoices(
        &self,
        org_id: Uuid,
        query: InvoiceQueryParams,
    ) -> Result<Vec<SubscriptionInvoice>, sqlx::Error> {
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
        .fetch_all(&self.pool)
        .await
    }

    /// List invoices with details (platform admin).
    pub async fn list_all_invoices(
        &self,
        query: InvoiceQueryParams,
    ) -> Result<Vec<InvoiceWithDetails>, sqlx::Error> {
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
        .fetch_all(&self.pool)
        .await
    }

    /// Mark invoice as paid.
    pub async fn mark_invoice_paid(
        &self,
        id: Uuid,
        payment_method_id: Option<Uuid>,
    ) -> Result<SubscriptionInvoice, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    /// Void an invoice.
    pub async fn void_invoice(&self, id: Uuid) -> Result<SubscriptionInvoice, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    /// Add line items to an invoice.
    #[allow(clippy::too_many_arguments)]
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
        .fetch_one(&self.pool)
        .await
    }

    /// Get line items for an invoice.
    pub async fn get_invoice_line_items(
        &self,
        invoice_id: Uuid,
    ) -> Result<Vec<InvoiceLineItem>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM invoice_line_items WHERE invoice_id = $1 ORDER BY created_at")
            .bind(invoice_id)
            .fetch_all(&self.pool)
            .await
    }

    // ==================== Usage Records ====================

    /// Record a usage metric.
    pub async fn record_usage(
        &self,
        org_id: Uuid,
        subscription_id: Option<Uuid>,
        data: CreateUsageRecord,
    ) -> Result<UsageRecord, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    /// Get usage summary for an organization.
    pub async fn get_usage_summary(
        &self,
        org_id: Uuid,
        period_start: chrono::DateTime<Utc>,
        period_end: chrono::DateTime<Utc>,
    ) -> Result<Vec<UsageSummary>, sqlx::Error> {
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
        .fetch_all(&self.pool)
        .await
    }

    /// Get current usage counts for an organization.
    pub async fn get_current_usage(
        &self,
        org_id: Uuid,
    ) -> Result<(i64, i64, i64, i64), sqlx::Error> {
        // Get building count
        let buildings: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM buildings WHERE organization_id = $1")
                .bind(org_id)
                .fetch_one(&self.pool)
                .await?;

        // Get unit count
        let units: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM units u JOIN buildings b ON u.building_id = b.id WHERE b.organization_id = $1",
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        // Get user count
        let users: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM organization_members WHERE organization_id = $1")
                .bind(org_id)
                .fetch_one(&self.pool)
                .await?;

        // Storage would require summing document sizes - using 0 as placeholder
        let storage = 0i64;

        Ok((buildings.0, units.0, users.0, storage))
    }

    // ==================== Subscription Events ====================

    /// Log a subscription event.
    pub async fn log_event(
        &self,
        org_id: Uuid,
        subscription_id: Option<Uuid>,
        actor_id: Option<Uuid>,
        data: CreateSubscriptionEvent,
    ) -> Result<SubscriptionEvent, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    /// Get subscription events.
    pub async fn get_events(
        &self,
        org_id: Uuid,
        limit: i32,
    ) -> Result<Vec<SubscriptionEvent>, sqlx::Error> {
        sqlx::query_as(
            "SELECT * FROM subscription_events WHERE organization_id = $1 ORDER BY created_at DESC LIMIT $2",
        )
        .bind(org_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    // ==================== Coupons ====================

    /// Create a coupon.
    pub async fn create_coupon(
        &self,
        data: CreateSubscriptionCoupon,
    ) -> Result<SubscriptionCoupon, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    /// Find a coupon by code.
    pub async fn find_coupon_by_code(
        &self,
        code: &str,
    ) -> Result<Option<SubscriptionCoupon>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM subscription_coupons WHERE code = $1 AND is_active = true")
            .bind(code)
            .fetch_optional(&self.pool)
            .await
    }

    /// List all coupons.
    pub async fn list_coupons(
        &self,
        active_only: bool,
    ) -> Result<Vec<SubscriptionCoupon>, sqlx::Error> {
        if active_only {
            sqlx::query_as(
                "SELECT * FROM subscription_coupons WHERE is_active = true ORDER BY created_at DESC",
            )
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as("SELECT * FROM subscription_coupons ORDER BY created_at DESC")
                .fetch_all(&self.pool)
                .await
        }
    }

    /// Update a coupon.
    pub async fn update_coupon(
        &self,
        id: Uuid,
        data: UpdateSubscriptionCoupon,
    ) -> Result<SubscriptionCoupon, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    /// Redeem a coupon.
    pub async fn redeem_coupon(
        &self,
        coupon_id: Uuid,
        org_id: Uuid,
        subscription_id: Option<Uuid>,
        user_id: Uuid,
    ) -> Result<CouponRedemption, sqlx::Error> {
        // Increment redemption count
        sqlx::query(
            "UPDATE subscription_coupons SET redemption_count = COALESCE(redemption_count, 0) + 1 WHERE id = $1",
        )
        .bind(coupon_id)
        .execute(&self.pool)
        .await?;

        sqlx::query_as(
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
        .fetch_one(&self.pool)
        .await
    }

    // ==================== Statistics ====================

    /// Get subscription statistics.
    pub async fn get_statistics(&self) -> Result<SubscriptionStatistics, sqlx::Error> {
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM organization_subscriptions")
            .fetch_one(&self.pool)
            .await?;

        let active: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM organization_subscriptions WHERE status = 'active'",
        )
        .fetch_one(&self.pool)
        .await?;

        let trial: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM organization_subscriptions WHERE status = 'trialing'",
        )
        .fetch_one(&self.pool)
        .await?;

        let cancelled: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM organization_subscriptions WHERE status = 'cancelled'",
        )
        .fetch_one(&self.pool)
        .await?;

        // Calculate MRR and ARR
        let mrr: (Option<Decimal>,) = sqlx::query_as(
            r#"
            SELECT SUM(
                CASE WHEN s.billing_cycle = 'annual'
                    THEN p.annual_price / 12
                    ELSE p.monthly_price
                END
            )
            FROM organization_subscriptions s
            JOIN subscription_plans p ON p.id = s.plan_id
            WHERE s.status = 'active'
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        let monthly_recurring_revenue = mrr.0.unwrap_or(Decimal::ZERO);
        let annual_recurring_revenue = monthly_recurring_revenue * Decimal::from(12);

        // Get counts by plan
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
            total_subscriptions: total.0,
            active_subscriptions: active.0,
            trial_subscriptions: trial.0,
            cancelled_subscriptions: cancelled.0,
            monthly_recurring_revenue,
            annual_recurring_revenue,
            by_plan,
        })
    }
}
