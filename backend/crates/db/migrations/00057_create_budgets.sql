-- Epic 24: Budget & Financial Planning
-- Migration: 00057_create_budgets.sql
-- Creates tables for budgets, budget items, capital plans, and forecasts.

-- ===========================================
-- Budgets Table
-- ===========================================
CREATE TABLE IF NOT EXISTS budgets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    building_id UUID,
    fiscal_year INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'draft',
    total_amount DECIMAL(15, 2) NOT NULL DEFAULT 0,
    notes TEXT,
    approved_by UUID,
    approved_at TIMESTAMPTZ,
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(organization_id, building_id, fiscal_year)
);

COMMENT ON TABLE budgets IS 'Annual budgets for buildings or organizations';
COMMENT ON COLUMN budgets.status IS 'draft, pending_approval, approved, active, closed';

-- ===========================================
-- Budget Categories Table
-- ===========================================
CREATE TABLE IF NOT EXISTS budget_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    parent_id UUID REFERENCES budget_categories(id),
    sort_order INTEGER DEFAULT 0,
    is_system BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(organization_id, name, parent_id)
);

COMMENT ON TABLE budget_categories IS 'Budget category definitions with hierarchy support';

-- ===========================================
-- Budget Items Table
-- ===========================================
CREATE TABLE IF NOT EXISTS budget_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    budget_id UUID NOT NULL REFERENCES budgets(id) ON DELETE CASCADE,
    category_id UUID NOT NULL REFERENCES budget_categories(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    budgeted_amount DECIMAL(15, 2) NOT NULL DEFAULT 0,
    actual_amount DECIMAL(15, 2) NOT NULL DEFAULT 0,
    variance_amount DECIMAL(15, 2) GENERATED ALWAYS AS (actual_amount - budgeted_amount) STORED,
    variance_percent DECIMAL(8, 2) GENERATED ALWAYS AS (
        CASE WHEN budgeted_amount = 0 THEN 0
        ELSE ROUND(((actual_amount - budgeted_amount) / budgeted_amount) * 100, 2)
        END
    ) STORED,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE budget_items IS 'Individual budget line items';

-- ===========================================
-- Budget Actuals Table (for tracking expenses against budget)
-- ===========================================
CREATE TABLE IF NOT EXISTS budget_actuals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    budget_item_id UUID NOT NULL REFERENCES budget_items(id) ON DELETE CASCADE,
    transaction_id UUID, -- Link to financial transaction if available
    amount DECIMAL(15, 2) NOT NULL,
    description TEXT,
    transaction_date DATE NOT NULL,
    recorded_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE budget_actuals IS 'Actual expenses recorded against budget items';

-- ===========================================
-- Capital Plans Table
-- ===========================================
CREATE TABLE IF NOT EXISTS capital_plans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    building_id UUID,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    estimated_cost DECIMAL(15, 2) NOT NULL,
    actual_cost DECIMAL(15, 2),
    funding_source VARCHAR(100) NOT NULL,
    target_year INTEGER NOT NULL,
    target_quarter INTEGER,
    priority VARCHAR(50) DEFAULT 'medium',
    status VARCHAR(50) NOT NULL DEFAULT 'planned',
    start_date DATE,
    completion_date DATE,
    notes TEXT,
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE capital_plans IS 'Capital improvement plans and projects';
COMMENT ON COLUMN capital_plans.funding_source IS 'reserve_fund, special_assessment, loan, grant, operating_budget, other';
COMMENT ON COLUMN capital_plans.priority IS 'low, medium, high, critical';
COMMENT ON COLUMN capital_plans.status IS 'planned, approved, in_progress, completed, cancelled, deferred';

-- ===========================================
-- Reserve Fund Table
-- ===========================================
CREATE TABLE IF NOT EXISTS reserve_funds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    building_id UUID,
    name VARCHAR(255) NOT NULL DEFAULT 'General Reserve',
    current_balance DECIMAL(15, 2) NOT NULL DEFAULT 0,
    target_balance DECIMAL(15, 2),
    annual_contribution DECIMAL(15, 2) NOT NULL DEFAULT 0,
    last_contribution_date DATE,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(organization_id, building_id, name)
);

COMMENT ON TABLE reserve_funds IS 'Reserve fund tracking for buildings';

-- ===========================================
-- Reserve Fund Transactions Table
-- ===========================================
CREATE TABLE IF NOT EXISTS reserve_fund_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reserve_fund_id UUID NOT NULL REFERENCES reserve_funds(id) ON DELETE CASCADE,
    transaction_type VARCHAR(50) NOT NULL,
    amount DECIMAL(15, 2) NOT NULL,
    description TEXT,
    reference_type VARCHAR(100),
    reference_id UUID,
    balance_after DECIMAL(15, 2) NOT NULL,
    transaction_date DATE NOT NULL,
    recorded_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE reserve_fund_transactions IS 'Reserve fund deposits and withdrawals';
COMMENT ON COLUMN reserve_fund_transactions.transaction_type IS 'contribution, withdrawal, interest, adjustment';

-- ===========================================
-- Financial Forecasts Table
-- ===========================================
CREATE TABLE IF NOT EXISTS financial_forecasts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    building_id UUID,
    name VARCHAR(255) NOT NULL,
    forecast_type VARCHAR(50) NOT NULL DEFAULT 'expense',
    start_year INTEGER NOT NULL,
    end_year INTEGER NOT NULL,
    inflation_rate DECIMAL(5, 2) NOT NULL DEFAULT 3.00,
    parameters JSONB NOT NULL DEFAULT '{}',
    forecast_data JSONB NOT NULL DEFAULT '[]',
    notes TEXT,
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE financial_forecasts IS 'Multi-year financial forecasts';
COMMENT ON COLUMN financial_forecasts.forecast_type IS 'expense, revenue, reserve, combined';

-- ===========================================
-- Budget Variance Alerts Table
-- ===========================================
CREATE TABLE IF NOT EXISTS budget_variance_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    budget_item_id UUID NOT NULL REFERENCES budget_items(id) ON DELETE CASCADE,
    alert_type VARCHAR(50) NOT NULL,
    threshold_percent DECIMAL(5, 2) NOT NULL,
    current_variance_percent DECIMAL(8, 2) NOT NULL,
    message TEXT NOT NULL,
    is_acknowledged BOOLEAN DEFAULT FALSE,
    acknowledged_by UUID,
    acknowledged_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE budget_variance_alerts IS 'Alerts triggered when budget variance exceeds threshold';
COMMENT ON COLUMN budget_variance_alerts.alert_type IS 'warning, critical, exceeded';

-- ===========================================
-- Indexes
-- ===========================================
CREATE INDEX idx_budgets_organization ON budgets(organization_id);
CREATE INDEX idx_budgets_building ON budgets(building_id) WHERE building_id IS NOT NULL;
CREATE INDEX idx_budgets_year ON budgets(fiscal_year);
CREATE INDEX idx_budgets_status ON budgets(status);

CREATE INDEX idx_budget_categories_organization ON budget_categories(organization_id);
CREATE INDEX idx_budget_categories_parent ON budget_categories(parent_id) WHERE parent_id IS NOT NULL;

CREATE INDEX idx_budget_items_budget ON budget_items(budget_id);
CREATE INDEX idx_budget_items_category ON budget_items(category_id);

CREATE INDEX idx_budget_actuals_item ON budget_actuals(budget_item_id);
CREATE INDEX idx_budget_actuals_date ON budget_actuals(transaction_date);

CREATE INDEX idx_capital_plans_organization ON capital_plans(organization_id);
CREATE INDEX idx_capital_plans_building ON capital_plans(building_id) WHERE building_id IS NOT NULL;
CREATE INDEX idx_capital_plans_year ON capital_plans(target_year);
CREATE INDEX idx_capital_plans_status ON capital_plans(status);

CREATE INDEX idx_reserve_funds_organization ON reserve_funds(organization_id);
CREATE INDEX idx_reserve_funds_building ON reserve_funds(building_id) WHERE building_id IS NOT NULL;

CREATE INDEX idx_reserve_transactions_fund ON reserve_fund_transactions(reserve_fund_id);
CREATE INDEX idx_reserve_transactions_date ON reserve_fund_transactions(transaction_date);

CREATE INDEX idx_financial_forecasts_organization ON financial_forecasts(organization_id);
CREATE INDEX idx_financial_forecasts_building ON financial_forecasts(building_id) WHERE building_id IS NOT NULL;

CREATE INDEX idx_budget_alerts_item ON budget_variance_alerts(budget_item_id);
CREATE INDEX idx_budget_alerts_acknowledged ON budget_variance_alerts(is_acknowledged);

-- ===========================================
-- RLS Policies
-- ===========================================
ALTER TABLE budgets ENABLE ROW LEVEL SECURITY;
ALTER TABLE budget_categories ENABLE ROW LEVEL SECURITY;
ALTER TABLE budget_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE budget_actuals ENABLE ROW LEVEL SECURITY;
ALTER TABLE capital_plans ENABLE ROW LEVEL SECURITY;
ALTER TABLE reserve_funds ENABLE ROW LEVEL SECURITY;
ALTER TABLE reserve_fund_transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE financial_forecasts ENABLE ROW LEVEL SECURITY;
ALTER TABLE budget_variance_alerts ENABLE ROW LEVEL SECURITY;

-- Budgets policies
CREATE POLICY budgets_tenant_isolation ON budgets
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY budget_categories_tenant_isolation ON budget_categories
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY budget_items_tenant_isolation ON budget_items
    USING (budget_id IN (
        SELECT id FROM budgets
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

CREATE POLICY budget_actuals_tenant_isolation ON budget_actuals
    USING (budget_item_id IN (
        SELECT bi.id FROM budget_items bi
        JOIN budgets b ON b.id = bi.budget_id
        WHERE b.organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

CREATE POLICY capital_plans_tenant_isolation ON capital_plans
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY reserve_funds_tenant_isolation ON reserve_funds
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY reserve_transactions_tenant_isolation ON reserve_fund_transactions
    USING (reserve_fund_id IN (
        SELECT id FROM reserve_funds
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

CREATE POLICY financial_forecasts_tenant_isolation ON financial_forecasts
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY budget_alerts_tenant_isolation ON budget_variance_alerts
    USING (budget_item_id IN (
        SELECT bi.id FROM budget_items bi
        JOIN budgets b ON b.id = bi.budget_id
        WHERE b.organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

-- ===========================================
-- Triggers
-- ===========================================
CREATE OR REPLACE FUNCTION update_budget_total()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE budgets
    SET total_amount = (
        SELECT COALESCE(SUM(budgeted_amount), 0)
        FROM budget_items
        WHERE budget_id = NEW.budget_id
    ),
    updated_at = NOW()
    WHERE id = NEW.budget_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_update_budget_total
AFTER INSERT OR UPDATE OR DELETE ON budget_items
FOR EACH ROW EXECUTE FUNCTION update_budget_total();

CREATE OR REPLACE FUNCTION update_budget_item_actuals()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE budget_items
    SET actual_amount = (
        SELECT COALESCE(SUM(amount), 0)
        FROM budget_actuals
        WHERE budget_item_id = NEW.budget_item_id
    ),
    updated_at = NOW()
    WHERE id = NEW.budget_item_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_update_budget_item_actuals
AFTER INSERT OR UPDATE OR DELETE ON budget_actuals
FOR EACH ROW EXECUTE FUNCTION update_budget_item_actuals();

CREATE OR REPLACE FUNCTION update_reserve_fund_balance()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE reserve_funds
    SET current_balance = NEW.balance_after,
        last_contribution_date = CASE
            WHEN NEW.transaction_type = 'contribution' THEN NEW.transaction_date
            ELSE last_contribution_date
        END,
        updated_at = NOW()
    WHERE id = NEW.reserve_fund_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_update_reserve_fund_balance
AFTER INSERT ON reserve_fund_transactions
FOR EACH ROW EXECUTE FUNCTION update_reserve_fund_balance();

-- Updated_at triggers
CREATE TRIGGER set_budgets_updated_at
    BEFORE UPDATE ON budgets
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_budget_items_updated_at
    BEFORE UPDATE ON budget_items
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_capital_plans_updated_at
    BEFORE UPDATE ON capital_plans
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_reserve_funds_updated_at
    BEFORE UPDATE ON reserve_funds
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_financial_forecasts_updated_at
    BEFORE UPDATE ON financial_forecasts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ===========================================
-- Seed Default Budget Categories
-- ===========================================
-- Note: These will be created per organization when they set up budgets
-- Common categories for reference:
-- - Maintenance & Repairs
-- - Utilities (Electric, Gas, Water, Sewer)
-- - Insurance
-- - Management Fees
-- - Reserve Fund Contributions
-- - Administrative
-- - Landscaping & Grounds
-- - Security
-- - Cleaning & Janitorial
-- - Legal & Professional Fees
-- - Taxes & Assessments
-- - Capital Improvements
-- - Other
