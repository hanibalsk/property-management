//! Subscription and billing models (Epic 26).

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Subscription status constants.
pub mod subscription_status {
    pub const ACTIVE: &str = "active";
    pub const TRIALING: &str = "trialing";
    pub const PAST_DUE: &str = "past_due";
    pub const CANCELLED: &str = "cancelled";
    pub const PAUSED: &str = "paused";
    pub const INCOMPLETE: &str = "incomplete";
    pub const ALL: &[&str] = &[ACTIVE, TRIALING, PAST_DUE, CANCELLED, PAUSED, INCOMPLETE];
}

/// Billing cycle constants.
pub mod billing_cycle {
    pub const MONTHLY: &str = "monthly";
    pub const ANNUAL: &str = "annual";
    pub const ALL: &[&str] = &[MONTHLY, ANNUAL];
}

/// Payment method type constants.
pub mod payment_method_type {
    pub const CARD: &str = "card";
    pub const BANK_TRANSFER: &str = "bank_transfer";
    pub const SEPA_DEBIT: &str = "sepa_debit";
    pub const INVOICE: &str = "invoice";
    pub const ALL: &[&str] = &[CARD, BANK_TRANSFER, SEPA_DEBIT, INVOICE];
}

/// Subscription invoice status constants.
pub mod subscription_invoice_status {
    pub const DRAFT: &str = "draft";
    pub const OPEN: &str = "open";
    pub const PAID: &str = "paid";
    pub const UNCOLLECTIBLE: &str = "uncollectible";
    pub const VOID: &str = "void";
    pub const ALL: &[&str] = &[DRAFT, OPEN, PAID, UNCOLLECTIBLE, VOID];
}

/// Invoice line item type constants.
pub mod line_item_type {
    pub const SUBSCRIPTION: &str = "subscription";
    pub const ADDON: &str = "addon";
    pub const OVERAGE: &str = "overage";
    pub const CREDIT: &str = "credit";
    pub const DISCOUNT: &str = "discount";
    pub const ALL: &[&str] = &[SUBSCRIPTION, ADDON, OVERAGE, CREDIT, DISCOUNT];
}

/// Usage metric type constants.
pub mod metric_type {
    pub const STORAGE_GB: &str = "storage_gb";
    pub const API_CALLS: &str = "api_calls";
    pub const EMAIL_SENT: &str = "email_sent";
    pub const SMS_SENT: &str = "sms_sent";
    pub const BUILDINGS: &str = "buildings";
    pub const UNITS: &str = "units";
    pub const USERS: &str = "users";
    pub const ALL: &[&str] = &[
        STORAGE_GB, API_CALLS, EMAIL_SENT, SMS_SENT, BUILDINGS, UNITS, USERS,
    ];
}

/// Coupon discount type constants.
pub mod discount_type {
    pub const PERCENTAGE: &str = "percentage";
    pub const FIXED_AMOUNT: &str = "fixed_amount";
    pub const ALL: &[&str] = &[PERCENTAGE, FIXED_AMOUNT];
}

/// Coupon duration constants.
pub mod coupon_duration {
    pub const ONCE: &str = "once";
    pub const REPEATING: &str = "repeating";
    pub const FOREVER: &str = "forever";
    pub const ALL: &[&str] = &[ONCE, REPEATING, FOREVER];
}

// ==================== Subscription Plan ====================

/// Subscription plan entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SubscriptionPlan {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,

    // Pricing
    pub monthly_price: Decimal,
    pub annual_price: Option<Decimal>,
    pub currency: String,

    // Limits
    pub max_buildings: Option<i32>,
    pub max_units: Option<i32>,
    pub max_users: Option<i32>,
    pub max_storage_gb: Option<i32>,

    // Features
    pub features: Option<serde_json::Value>,

    // Status
    pub is_active: Option<bool>,
    pub is_public: Option<bool>,
    pub trial_days: Option<i32>,

    // Ordering
    pub sort_order: Option<i32>,
    pub metadata: Option<serde_json::Value>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create subscription plan request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSubscriptionPlan {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub monthly_price: Decimal,
    pub annual_price: Option<Decimal>,
    pub currency: Option<String>,
    pub max_buildings: Option<i32>,
    pub max_units: Option<i32>,
    pub max_users: Option<i32>,
    pub max_storage_gb: Option<i32>,
    pub features: Option<serde_json::Value>,
    pub trial_days: Option<i32>,
    pub sort_order: Option<i32>,
    pub metadata: Option<serde_json::Value>,
}

/// Update subscription plan request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSubscriptionPlan {
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub monthly_price: Option<Decimal>,
    pub annual_price: Option<Decimal>,
    pub currency: Option<String>,
    pub max_buildings: Option<i32>,
    pub max_units: Option<i32>,
    pub max_users: Option<i32>,
    pub max_storage_gb: Option<i32>,
    pub features: Option<serde_json::Value>,
    pub is_active: Option<bool>,
    pub is_public: Option<bool>,
    pub trial_days: Option<i32>,
    pub sort_order: Option<i32>,
    pub metadata: Option<serde_json::Value>,
}

// ==================== Organization Subscription ====================

/// Organization subscription entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct OrganizationSubscription {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub plan_id: Uuid,

    // Status
    pub status: String,
    pub billing_cycle: String,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,

    // Trial
    pub trial_start: Option<DateTime<Utc>>,
    pub trial_end: Option<DateTime<Utc>>,
    pub is_trial: Option<bool>,

    // Cancellation
    pub cancel_at_period_end: Option<bool>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancellation_reason: Option<String>,

    // Payment
    pub payment_method_id: Option<Uuid>,
    pub last_payment_at: Option<DateTime<Utc>>,
    pub next_payment_at: Option<DateTime<Utc>>,

    // Usage
    pub current_buildings: Option<i32>,
    pub current_units: Option<i32>,
    pub current_users: Option<i32>,
    pub current_storage_bytes: Option<i64>,

    // External
    pub stripe_subscription_id: Option<String>,
    pub stripe_customer_id: Option<String>,

    pub metadata: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create organization subscription request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateOrganizationSubscription {
    pub plan_id: Uuid,
    pub billing_cycle: Option<String>,
    pub start_trial: Option<bool>,
    pub payment_method_id: Option<Uuid>,
    pub coupon_code: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Update organization subscription request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateOrganizationSubscription {
    pub billing_cycle: Option<String>,
    pub payment_method_id: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
}

/// Change plan request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ChangePlanRequest {
    pub new_plan_id: Uuid,
    pub billing_cycle: Option<String>,
    pub prorate: Option<bool>,
}

/// Cancel subscription request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CancelSubscriptionRequest {
    pub cancel_at_period_end: Option<bool>,
    pub cancellation_reason: Option<String>,
}

// ==================== Payment Method ====================

/// Payment method entity for subscriptions.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SubscriptionPaymentMethod {
    pub id: Uuid,
    pub organization_id: Uuid,

    // Type
    pub method_type: String,

    // Card info
    pub card_brand: Option<String>,
    pub card_last_four: Option<String>,
    pub card_exp_month: Option<i32>,
    pub card_exp_year: Option<i32>,

    // Bank info
    pub bank_name: Option<String>,
    pub bank_last_four: Option<String>,

    // Status
    pub is_default: Option<bool>,
    pub is_verified: Option<bool>,

    // External
    pub stripe_payment_method_id: Option<String>,

    pub billing_address: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create payment method request for subscriptions.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSubscriptionPaymentMethod {
    pub method_type: String,
    pub stripe_payment_method_id: Option<String>,
    pub is_default: Option<bool>,
    pub billing_address: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

// ==================== Invoice ====================

/// Subscription invoice entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SubscriptionInvoice {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub subscription_id: Option<Uuid>,

    // Details
    pub invoice_number: String,
    pub invoice_date: NaiveDate,
    pub due_date: NaiveDate,

    // Period
    pub period_start: Option<DateTime<Utc>>,
    pub period_end: Option<DateTime<Utc>>,

    // Amounts
    pub subtotal: Decimal,
    pub tax_amount: Option<Decimal>,
    pub discount_amount: Option<Decimal>,
    pub total_amount: Decimal,
    pub currency: String,

    // Status
    pub status: String,

    // Payment
    pub paid_at: Option<DateTime<Utc>>,
    pub payment_method_id: Option<Uuid>,
    pub payment_intent_id: Option<String>,

    // Line items
    pub line_items: Option<serde_json::Value>,

    // PDF
    pub pdf_url: Option<String>,

    // External
    pub stripe_invoice_id: Option<String>,

    pub billing_details: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Invoice query parameters.
#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct InvoiceQueryParams {
    pub status: Option<String>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

// ==================== Invoice Line Item ====================

/// Invoice line item entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InvoiceLineItem {
    pub id: Uuid,
    pub invoice_id: Uuid,

    // Details
    pub description: String,
    pub quantity: Option<Decimal>,
    pub unit_price: Decimal,
    pub amount: Decimal,

    // Type
    pub item_type: String,

    // Period
    pub period_start: Option<DateTime<Utc>>,
    pub period_end: Option<DateTime<Utc>>,

    // Reference
    pub plan_id: Option<Uuid>,

    pub created_at: Option<DateTime<Utc>>,
}

// ==================== Usage Record ====================

/// Usage record entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UsageRecord {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub subscription_id: Option<Uuid>,

    // Metric
    pub metric_type: String,
    pub quantity: Decimal,
    pub unit: Option<String>,

    // Time
    pub recorded_at: Option<DateTime<Utc>>,
    pub period_start: Option<DateTime<Utc>>,
    pub period_end: Option<DateTime<Utc>>,

    // Billing
    pub is_billed: Option<bool>,
    pub invoice_id: Option<Uuid>,

    pub metadata: Option<serde_json::Value>,
}

/// Create usage record request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUsageRecord {
    pub metric_type: String,
    pub quantity: Decimal,
    pub unit: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Usage summary.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UsageSummary {
    pub metric_type: String,
    pub total_quantity: Decimal,
    pub unit: Option<String>,
    pub record_count: i64,
}

// ==================== Subscription Event ====================

/// Subscription event entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SubscriptionEvent {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub subscription_id: Option<Uuid>,

    // Event
    pub event_type: String,
    pub description: Option<String>,

    // Actor
    pub actor_id: Option<Uuid>,
    pub actor_type: Option<String>,

    // Data
    pub previous_data: Option<serde_json::Value>,
    pub new_data: Option<serde_json::Value>,

    // Webhook
    pub webhook_id: Option<String>,

    pub created_at: Option<DateTime<Utc>>,
}

/// Create subscription event request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSubscriptionEvent {
    pub event_type: String,
    pub description: Option<String>,
    pub previous_data: Option<serde_json::Value>,
    pub new_data: Option<serde_json::Value>,
    pub webhook_id: Option<String>,
}

// ==================== Coupon ====================

/// Subscription coupon entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SubscriptionCoupon {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,

    // Discount
    pub discount_type: String,
    pub discount_value: Decimal,
    pub currency: Option<String>,

    // Duration
    pub duration: String,
    pub duration_months: Option<i32>,

    // Limits
    pub max_redemptions: Option<i32>,
    pub redemption_count: Option<i32>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,

    // Restrictions
    pub applicable_plans: Option<Vec<Uuid>>,
    pub min_amount: Option<Decimal>,

    // Status
    pub is_active: Option<bool>,

    // External
    pub stripe_coupon_id: Option<String>,

    pub metadata: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Create coupon request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSubscriptionCoupon {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: Decimal,
    pub currency: Option<String>,
    pub duration: Option<String>,
    pub duration_months: Option<i32>,
    pub max_redemptions: Option<i32>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub applicable_plans: Option<Vec<Uuid>>,
    pub min_amount: Option<Decimal>,
    pub metadata: Option<serde_json::Value>,
}

/// Update coupon request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSubscriptionCoupon {
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_redemptions: Option<i32>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub applicable_plans: Option<Vec<Uuid>>,
    pub min_amount: Option<Decimal>,
    pub is_active: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}

// ==================== Coupon Redemption ====================

/// Coupon redemption entity.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CouponRedemption {
    pub id: Uuid,
    pub coupon_id: Uuid,
    pub organization_id: Uuid,
    pub subscription_id: Option<Uuid>,

    // Details
    pub redeemed_at: Option<DateTime<Utc>>,
    pub redeemed_by: Option<Uuid>,

    // Discount
    pub discount_amount: Option<Decimal>,

    // Status
    pub is_active: Option<bool>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Redeem coupon request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct RedeemCouponRequest {
    pub coupon_code: String,
}

// ==================== Analytics ====================

/// Subscription statistics.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SubscriptionStatistics {
    pub total_subscriptions: i64,
    pub active_subscriptions: i64,
    pub trial_subscriptions: i64,
    pub cancelled_subscriptions: i64,
    pub monthly_recurring_revenue: Decimal,
    pub annual_recurring_revenue: Decimal,
    pub by_plan: Vec<PlanSubscriptionCount>,
}

/// Plan subscription count.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PlanSubscriptionCount {
    pub plan_id: Uuid,
    pub plan_name: String,
    pub count: i64,
}

/// Subscription with plan details.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SubscriptionWithPlan {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub status: String,
    pub billing_cycle: String,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub is_trial: Option<bool>,
    pub cancel_at_period_end: Option<bool>,
    pub plan_id: Uuid,
    pub plan_name: String,
    pub plan_display_name: String,
    pub monthly_price: Decimal,
    pub annual_price: Option<Decimal>,
}

/// Invoice with organization details.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InvoiceWithDetails {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub invoice_number: String,
    pub invoice_date: NaiveDate,
    pub due_date: NaiveDate,
    pub subtotal: Decimal,
    pub tax_amount: Option<Decimal>,
    pub total_amount: Decimal,
    pub currency: String,
    pub status: String,
    pub paid_at: Option<DateTime<Utc>>,
}
