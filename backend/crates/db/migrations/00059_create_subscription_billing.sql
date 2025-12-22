-- Epic 26: Platform Subscription & Billing
-- Migration: 00059_create_subscription_billing.sql
-- Creates tables for subscription plans, organization subscriptions, and billing.

-- ===========================================
-- Subscription Plans Table
-- ===========================================
CREATE TABLE IF NOT EXISTS subscription_plans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    description TEXT,

    -- Pricing
    monthly_price DECIMAL(12, 2) NOT NULL,
    annual_price DECIMAL(12, 2),
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',

    -- Features and limits
    max_buildings INTEGER,
    max_units INTEGER,
    max_users INTEGER,
    max_storage_gb INTEGER,
    features JSONB DEFAULT '[]',

    -- Status
    is_active BOOLEAN DEFAULT TRUE,
    is_public BOOLEAN DEFAULT TRUE,
    trial_days INTEGER DEFAULT 14,

    -- Metadata
    sort_order INTEGER DEFAULT 0,
    metadata JSONB DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE subscription_plans IS 'Available subscription plans for the platform';
COMMENT ON COLUMN subscription_plans.features IS 'JSON array of feature flags enabled for this plan';

-- ===========================================
-- Organization Subscriptions Table
-- ===========================================
CREATE TABLE IF NOT EXISTS organization_subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL UNIQUE,
    plan_id UUID NOT NULL REFERENCES subscription_plans(id),

    -- Subscription period
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    billing_cycle VARCHAR(20) NOT NULL DEFAULT 'monthly',
    current_period_start TIMESTAMPTZ NOT NULL,
    current_period_end TIMESTAMPTZ NOT NULL,

    -- Trial
    trial_start TIMESTAMPTZ,
    trial_end TIMESTAMPTZ,
    is_trial BOOLEAN DEFAULT FALSE,

    -- Cancellation
    cancel_at_period_end BOOLEAN DEFAULT FALSE,
    cancelled_at TIMESTAMPTZ,
    cancellation_reason TEXT,

    -- Payment
    payment_method_id UUID,
    last_payment_at TIMESTAMPTZ,
    next_payment_at TIMESTAMPTZ,

    -- Usage tracking
    current_buildings INTEGER DEFAULT 0,
    current_units INTEGER DEFAULT 0,
    current_users INTEGER DEFAULT 0,
    current_storage_bytes BIGINT DEFAULT 0,

    -- External references
    stripe_subscription_id VARCHAR(255),
    stripe_customer_id VARCHAR(255),

    -- Metadata
    metadata JSONB DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE organization_subscriptions IS 'Organization subscription status and billing info';
COMMENT ON COLUMN organization_subscriptions.status IS 'active, trialing, past_due, cancelled, paused, incomplete';
COMMENT ON COLUMN organization_subscriptions.billing_cycle IS 'monthly, annual';

-- ===========================================
-- Payment Methods Table
-- ===========================================
CREATE TABLE IF NOT EXISTS payment_methods (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,

    -- Method type
    method_type VARCHAR(50) NOT NULL,

    -- Card details (masked)
    card_brand VARCHAR(50),
    card_last_four VARCHAR(4),
    card_exp_month INTEGER,
    card_exp_year INTEGER,

    -- Bank details (masked)
    bank_name VARCHAR(255),
    bank_last_four VARCHAR(4),

    -- Status
    is_default BOOLEAN DEFAULT FALSE,
    is_verified BOOLEAN DEFAULT FALSE,

    -- External references
    stripe_payment_method_id VARCHAR(255),

    -- Metadata
    billing_address JSONB,
    metadata JSONB DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE payment_methods IS 'Stored payment methods for organizations';
COMMENT ON COLUMN payment_methods.method_type IS 'card, bank_transfer, sepa_debit, invoice';

-- ===========================================
-- Invoices Table
-- ===========================================
CREATE TABLE IF NOT EXISTS subscription_invoices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    subscription_id UUID REFERENCES organization_subscriptions(id),

    -- Invoice details
    invoice_number VARCHAR(50) NOT NULL UNIQUE,
    invoice_date DATE NOT NULL,
    due_date DATE NOT NULL,

    -- Period covered
    period_start TIMESTAMPTZ,
    period_end TIMESTAMPTZ,

    -- Amounts
    subtotal DECIMAL(12, 2) NOT NULL,
    tax_amount DECIMAL(12, 2) DEFAULT 0,
    discount_amount DECIMAL(12, 2) DEFAULT 0,
    total_amount DECIMAL(12, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',

    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'draft',

    -- Payment
    paid_at TIMESTAMPTZ,
    payment_method_id UUID REFERENCES payment_methods(id),
    payment_intent_id VARCHAR(255),

    -- Line items
    line_items JSONB DEFAULT '[]',

    -- PDF
    pdf_url VARCHAR(1000),

    -- External references
    stripe_invoice_id VARCHAR(255),

    -- Metadata
    billing_details JSONB,
    metadata JSONB DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE subscription_invoices IS 'Invoices for subscription payments';
COMMENT ON COLUMN subscription_invoices.status IS 'draft, open, paid, uncollectible, void';

-- ===========================================
-- Invoice Line Items Table (for detail)
-- ===========================================
CREATE TABLE IF NOT EXISTS invoice_line_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    invoice_id UUID NOT NULL REFERENCES subscription_invoices(id) ON DELETE CASCADE,

    -- Item details
    description VARCHAR(500) NOT NULL,
    quantity DECIMAL(12, 4) DEFAULT 1,
    unit_price DECIMAL(12, 2) NOT NULL,
    amount DECIMAL(12, 2) NOT NULL,

    -- Type
    item_type VARCHAR(50) NOT NULL DEFAULT 'subscription',

    -- Period
    period_start TIMESTAMPTZ,
    period_end TIMESTAMPTZ,

    -- References
    plan_id UUID REFERENCES subscription_plans(id),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE invoice_line_items IS 'Individual line items on invoices';
COMMENT ON COLUMN invoice_line_items.item_type IS 'subscription, addon, overage, credit, discount';

-- ===========================================
-- Usage Records Table
-- ===========================================
CREATE TABLE IF NOT EXISTS usage_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    subscription_id UUID REFERENCES organization_subscriptions(id),

    -- Usage metric
    metric_type VARCHAR(100) NOT NULL,
    quantity DECIMAL(12, 4) NOT NULL,
    unit VARCHAR(50),

    -- Period
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    period_start TIMESTAMPTZ,
    period_end TIMESTAMPTZ,

    -- Billing
    is_billed BOOLEAN DEFAULT FALSE,
    invoice_id UUID REFERENCES subscription_invoices(id),

    -- Metadata
    metadata JSONB DEFAULT '{}'
);

COMMENT ON TABLE usage_records IS 'Usage-based billing records';
COMMENT ON COLUMN usage_records.metric_type IS 'storage_gb, api_calls, email_sent, sms_sent, buildings, units, users';

-- ===========================================
-- Subscription Events Table (Audit)
-- ===========================================
CREATE TABLE IF NOT EXISTS subscription_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    subscription_id UUID REFERENCES organization_subscriptions(id),

    -- Event details
    event_type VARCHAR(100) NOT NULL,
    description TEXT,

    -- Actor
    actor_id UUID,
    actor_type VARCHAR(50),

    -- Data
    previous_data JSONB,
    new_data JSONB,

    -- Webhook
    webhook_id VARCHAR(255),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE subscription_events IS 'Audit trail for subscription changes';
COMMENT ON COLUMN subscription_events.event_type IS 'created, upgraded, downgraded, renewed, cancelled, payment_succeeded, payment_failed, trial_ending, etc.';

-- ===========================================
-- Coupons Table
-- ===========================================
CREATE TABLE IF NOT EXISTS subscription_coupons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    description TEXT,

    -- Discount
    discount_type VARCHAR(20) NOT NULL,
    discount_value DECIMAL(12, 2) NOT NULL,
    currency VARCHAR(3),

    -- Duration
    duration VARCHAR(20) NOT NULL DEFAULT 'once',
    duration_months INTEGER,

    -- Limits
    max_redemptions INTEGER,
    redemption_count INTEGER DEFAULT 0,
    valid_from TIMESTAMPTZ,
    valid_until TIMESTAMPTZ,

    -- Restrictions
    applicable_plans UUID[],
    min_amount DECIMAL(12, 2),

    -- Status
    is_active BOOLEAN DEFAULT TRUE,

    -- External
    stripe_coupon_id VARCHAR(255),

    -- Metadata
    metadata JSONB DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE subscription_coupons IS 'Discount coupons for subscriptions';
COMMENT ON COLUMN subscription_coupons.discount_type IS 'percentage, fixed_amount';
COMMENT ON COLUMN subscription_coupons.duration IS 'once, repeating, forever';

-- ===========================================
-- Coupon Redemptions Table
-- ===========================================
CREATE TABLE IF NOT EXISTS coupon_redemptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    coupon_id UUID NOT NULL REFERENCES subscription_coupons(id),
    organization_id UUID NOT NULL,
    subscription_id UUID REFERENCES organization_subscriptions(id),

    -- Redemption details
    redeemed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    redeemed_by UUID,

    -- Discount applied
    discount_amount DECIMAL(12, 2),

    -- Status
    is_active BOOLEAN DEFAULT TRUE,
    expires_at TIMESTAMPTZ
);

COMMENT ON TABLE coupon_redemptions IS 'Coupon redemption history';

-- ===========================================
-- Indexes
-- ===========================================
CREATE INDEX idx_subscription_plans_active ON subscription_plans(is_active, is_public);
CREATE INDEX idx_subscription_plans_sort ON subscription_plans(sort_order);

CREATE INDEX idx_org_subscriptions_org ON organization_subscriptions(organization_id);
CREATE INDEX idx_org_subscriptions_status ON organization_subscriptions(status);
CREATE INDEX idx_org_subscriptions_plan ON organization_subscriptions(plan_id);
CREATE INDEX idx_org_subscriptions_period ON organization_subscriptions(current_period_end);
CREATE INDEX idx_org_subscriptions_stripe ON organization_subscriptions(stripe_subscription_id) WHERE stripe_subscription_id IS NOT NULL;

CREATE INDEX idx_payment_methods_org ON payment_methods(organization_id);
CREATE INDEX idx_payment_methods_default ON payment_methods(organization_id, is_default) WHERE is_default = TRUE;
CREATE INDEX idx_payment_methods_stripe ON payment_methods(stripe_payment_method_id) WHERE stripe_payment_method_id IS NOT NULL;

CREATE INDEX idx_sub_invoices_org ON subscription_invoices(organization_id);
CREATE INDEX idx_sub_invoices_subscription ON subscription_invoices(subscription_id);
CREATE INDEX idx_sub_invoices_status ON subscription_invoices(status);
CREATE INDEX idx_sub_invoices_due ON subscription_invoices(due_date) WHERE status NOT IN ('paid', 'void');
CREATE INDEX idx_sub_invoices_stripe ON subscription_invoices(stripe_invoice_id) WHERE stripe_invoice_id IS NOT NULL;

CREATE INDEX idx_invoice_lines_invoice ON invoice_line_items(invoice_id);

CREATE INDEX idx_usage_records_org ON usage_records(organization_id);
CREATE INDEX idx_usage_records_subscription ON usage_records(subscription_id);
CREATE INDEX idx_usage_records_metric ON usage_records(metric_type);
CREATE INDEX idx_usage_records_date ON usage_records(recorded_at);
CREATE INDEX idx_usage_records_unbilled ON usage_records(is_billed) WHERE is_billed = FALSE;

CREATE INDEX idx_sub_events_org ON subscription_events(organization_id);
CREATE INDEX idx_sub_events_subscription ON subscription_events(subscription_id);
CREATE INDEX idx_sub_events_type ON subscription_events(event_type);
CREATE INDEX idx_sub_events_date ON subscription_events(created_at);

CREATE INDEX idx_coupons_code ON subscription_coupons(code);
CREATE INDEX idx_coupons_active ON subscription_coupons(is_active, valid_until);

CREATE INDEX idx_coupon_redemptions_coupon ON coupon_redemptions(coupon_id);
CREATE INDEX idx_coupon_redemptions_org ON coupon_redemptions(organization_id);

-- ===========================================
-- RLS Policies
-- ===========================================
ALTER TABLE subscription_plans ENABLE ROW LEVEL SECURITY;
ALTER TABLE organization_subscriptions ENABLE ROW LEVEL SECURITY;
ALTER TABLE payment_methods ENABLE ROW LEVEL SECURITY;
ALTER TABLE subscription_invoices ENABLE ROW LEVEL SECURITY;
ALTER TABLE invoice_line_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE usage_records ENABLE ROW LEVEL SECURITY;
ALTER TABLE subscription_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE subscription_coupons ENABLE ROW LEVEL SECURITY;
ALTER TABLE coupon_redemptions ENABLE ROW LEVEL SECURITY;

-- Plans are public read
CREATE POLICY subscription_plans_read ON subscription_plans
    FOR SELECT USING (is_public = TRUE OR is_active = TRUE);

-- Organization subscriptions - tenant isolation
CREATE POLICY org_subscriptions_tenant_isolation ON organization_subscriptions
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY payment_methods_tenant_isolation ON payment_methods
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY sub_invoices_tenant_isolation ON subscription_invoices
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY invoice_lines_tenant_isolation ON invoice_line_items
    USING (invoice_id IN (
        SELECT id FROM subscription_invoices
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

CREATE POLICY usage_records_tenant_isolation ON usage_records
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY sub_events_tenant_isolation ON subscription_events
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY coupons_read ON subscription_coupons
    FOR SELECT USING (is_active = TRUE);

CREATE POLICY coupon_redemptions_tenant_isolation ON coupon_redemptions
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

-- ===========================================
-- Triggers
-- ===========================================
CREATE TRIGGER set_subscription_plans_updated_at
    BEFORE UPDATE ON subscription_plans
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_org_subscriptions_updated_at
    BEFORE UPDATE ON organization_subscriptions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_payment_methods_updated_at
    BEFORE UPDATE ON payment_methods
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_sub_invoices_updated_at
    BEFORE UPDATE ON subscription_invoices
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_coupons_updated_at
    BEFORE UPDATE ON subscription_coupons
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ===========================================
-- Insert default plans
-- ===========================================
INSERT INTO subscription_plans (name, display_name, description, monthly_price, annual_price, currency, max_buildings, max_units, max_users, max_storage_gb, features, trial_days, sort_order)
VALUES
    ('free', 'Free', 'Perfect for getting started', 0.00, 0.00, 'EUR', 1, 10, 3, 1, '["basic_features"]', 0, 1),
    ('starter', 'Starter', 'For small property managers', 29.00, 290.00, 'EUR', 5, 50, 10, 5, '["basic_features", "reports", "email_notifications"]', 14, 2),
    ('professional', 'Professional', 'For growing businesses', 79.00, 790.00, 'EUR', 20, 200, 50, 25, '["basic_features", "reports", "email_notifications", "api_access", "custom_branding"]', 14, 3),
    ('enterprise', 'Enterprise', 'For large organizations', 199.00, 1990.00, 'EUR', NULL, NULL, NULL, 100, '["basic_features", "reports", "email_notifications", "api_access", "custom_branding", "sso", "priority_support", "sla"]', 30, 4)
ON CONFLICT DO NOTHING;
