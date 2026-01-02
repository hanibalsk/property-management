-- Epic 74: Owner Investment Analytics
-- Creates tables for property valuations, cash flow, ROI tracking, and expense approvals.

-- =============================================================================
-- EXPENSE APPROVAL ENUM
-- =============================================================================

DO $$ BEGIN
    CREATE TYPE expense_approval_status AS ENUM ('approved', 'rejected', 'needs_info');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- =============================================================================
-- PROPERTY VALUATIONS (Story 74.1)
-- =============================================================================

CREATE TABLE IF NOT EXISTS property_valuations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    valuation_date DATE NOT NULL DEFAULT CURRENT_DATE,
    value_low DECIMAL(15, 2) NOT NULL,
    value_high DECIMAL(15, 2) NOT NULL,
    estimated_value DECIMAL(15, 2) NOT NULL,
    valuation_method VARCHAR(50) NOT NULL,
    confidence_score DECIMAL(5, 2) NOT NULL DEFAULT 0,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_property_valuations_unit ON property_valuations(unit_id);
CREATE INDEX IF NOT EXISTS idx_property_valuations_date ON property_valuations(valuation_date DESC);

CREATE TABLE IF NOT EXISTS comparable_properties (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    valuation_id UUID NOT NULL REFERENCES property_valuations(id) ON DELETE CASCADE,
    address TEXT NOT NULL,
    sale_price DECIMAL(15, 2) NOT NULL,
    sold_date DATE,
    size_sqm INTEGER NOT NULL,
    rooms INTEGER NOT NULL,
    distance_km DECIMAL(10, 2) NOT NULL,
    similarity_score DECIMAL(5, 2) NOT NULL DEFAULT 0,
    source VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_comparable_properties_valuation ON comparable_properties(valuation_id);

CREATE TABLE IF NOT EXISTS property_value_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    value DECIMAL(15, 2) NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_property_value_history_unit ON property_value_history(unit_id);
CREATE INDEX IF NOT EXISTS idx_property_value_history_date ON property_value_history(recorded_at DESC);

-- =============================================================================
-- EXPENSE AUTO-APPROVAL (Story 74.4)
-- =============================================================================

CREATE TABLE IF NOT EXISTS expense_auto_approval_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    unit_id UUID REFERENCES units(id) ON DELETE CASCADE,
    max_amount_per_expense DECIMAL(12, 2) NOT NULL DEFAULT 300.00,
    max_monthly_total DECIMAL(12, 2) NOT NULL DEFAULT 1000.00,
    allowed_categories TEXT[] NOT NULL DEFAULT ARRAY['maintenance'],
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_expense_auto_approval_rules_org ON expense_auto_approval_rules(organization_id);
CREATE INDEX IF NOT EXISTS idx_expense_auto_approval_rules_owner ON expense_auto_approval_rules(owner_id);
CREATE INDEX IF NOT EXISTS idx_expense_auto_approval_rules_unit ON expense_auto_approval_rules(unit_id) WHERE unit_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS expense_approval_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    submitted_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    amount DECIMAL(12, 2) NOT NULL,
    category VARCHAR(100) NOT NULL,
    description TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    auto_approval_rule_id UUID REFERENCES expense_auto_approval_rules(id) ON DELETE SET NULL,
    reviewed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    review_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_expense_approval_requests_unit ON expense_approval_requests(unit_id);
CREATE INDEX IF NOT EXISTS idx_expense_approval_requests_status ON expense_approval_requests(status);
CREATE INDEX IF NOT EXISTS idx_expense_approval_requests_submitted_by ON expense_approval_requests(submitted_by);

-- =============================================================================
-- TRIGGERS
-- =============================================================================

CREATE OR REPLACE FUNCTION update_owner_analytics_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_expense_auto_approval_rules_updated_at ON expense_auto_approval_rules;
CREATE TRIGGER trigger_expense_auto_approval_rules_updated_at
    BEFORE UPDATE ON expense_auto_approval_rules
    FOR EACH ROW
    EXECUTE FUNCTION update_owner_analytics_updated_at();

DROP TRIGGER IF EXISTS trigger_expense_approval_requests_updated_at ON expense_approval_requests;
CREATE TRIGGER trigger_expense_approval_requests_updated_at
    BEFORE UPDATE ON expense_approval_requests
    FOR EACH ROW
    EXECUTE FUNCTION update_owner_analytics_updated_at();

-- =============================================================================
-- COMMENTS
-- =============================================================================

COMMENT ON TABLE property_valuations IS 'Property valuation estimates with confidence scores';
COMMENT ON TABLE comparable_properties IS 'Comparable property sales used for valuation';
COMMENT ON TABLE property_value_history IS 'Historical property value tracking';
COMMENT ON TABLE expense_auto_approval_rules IS 'Rules for automatic expense approval by owners';
COMMENT ON TABLE expense_approval_requests IS 'Expense requests requiring owner approval';
