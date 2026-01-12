-- Epic 141: Reserve Fund Management
-- Migration: Create tables for HOA/Condo reserve fund management

-- Enum for fund types
CREATE TYPE fund_type AS ENUM (
    'operating',
    'reserve',
    'emergency',
    'special_assessment',
    'capital_improvement',
    'insurance',
    'legal',
    'custom'
);

-- Enum for contribution frequency
CREATE TYPE contribution_frequency AS ENUM (
    'monthly',
    'quarterly',
    'semi_annually',
    'annually',
    'one_time'
);

-- Enum for investment risk levels
CREATE TYPE investment_risk_level AS ENUM (
    'conservative',
    'moderate',
    'balanced',
    'growth',
    'aggressive'
);

-- Enum for transaction types
CREATE TYPE fund_transaction_type AS ENUM (
    'contribution',
    'withdrawal',
    'transfer',
    'interest',
    'dividend',
    'fee',
    'adjustment',
    'opening_balance'
);

-- Main reserve funds table
CREATE TABLE reserve_funds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE SET NULL,

    -- Fund details
    name VARCHAR(200) NOT NULL,
    description TEXT,
    fund_type fund_type NOT NULL DEFAULT 'reserve',

    -- Financial details
    current_balance DECIMAL(15, 2) NOT NULL DEFAULT 0,
    target_balance DECIMAL(15, 2),
    minimum_balance DECIMAL(15, 2),
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id),

    CONSTRAINT reserve_funds_balance_check CHECK (current_balance >= 0),
    CONSTRAINT reserve_funds_target_check CHECK (target_balance IS NULL OR target_balance >= 0),
    CONSTRAINT reserve_funds_minimum_check CHECK (minimum_balance IS NULL OR minimum_balance >= 0)
);

-- Fund contribution schedules
CREATE TABLE fund_contribution_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fund_id UUID NOT NULL REFERENCES reserve_funds(id) ON DELETE CASCADE,

    -- Schedule details
    name VARCHAR(200) NOT NULL,
    description TEXT,

    -- Contribution details
    amount DECIMAL(15, 2) NOT NULL,
    frequency contribution_frequency NOT NULL,

    -- Timing
    start_date DATE NOT NULL,
    end_date DATE,
    next_due_date DATE,

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    auto_collect BOOLEAN NOT NULL DEFAULT false,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT contribution_amount_positive CHECK (amount > 0)
);

-- Fund transactions
CREATE TABLE fund_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fund_id UUID NOT NULL REFERENCES reserve_funds(id) ON DELETE CASCADE,

    -- Transaction details
    transaction_type fund_transaction_type NOT NULL,
    amount DECIMAL(15, 2) NOT NULL,
    balance_after DECIMAL(15, 2) NOT NULL,

    -- Description
    description TEXT,
    reference_number VARCHAR(100),

    -- Related entities
    contribution_schedule_id UUID REFERENCES fund_contribution_schedules(id) ON DELETE SET NULL,
    transfer_to_fund_id UUID REFERENCES reserve_funds(id) ON DELETE SET NULL,

    -- Approval (for withdrawals)
    requires_approval BOOLEAN NOT NULL DEFAULT false,
    approved_by UUID REFERENCES users(id),
    approved_at TIMESTAMPTZ,

    -- Metadata
    transaction_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id)
);

-- Fund investment policies
CREATE TABLE fund_investment_policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fund_id UUID NOT NULL REFERENCES reserve_funds(id) ON DELETE CASCADE,

    -- Policy details
    name VARCHAR(200) NOT NULL,
    description TEXT,

    -- Risk profile
    risk_level investment_risk_level NOT NULL DEFAULT 'conservative',

    -- Asset allocation (percentages)
    cash_allocation_pct DECIMAL(5, 2) NOT NULL DEFAULT 100,
    bonds_allocation_pct DECIMAL(5, 2) NOT NULL DEFAULT 0,
    money_market_allocation_pct DECIMAL(5, 2) NOT NULL DEFAULT 0,
    other_allocation_pct DECIMAL(5, 2) NOT NULL DEFAULT 0,

    -- Constraints
    max_single_investment DECIMAL(15, 2),
    min_liquidity_pct DECIMAL(5, 2),

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    effective_date DATE NOT NULL,
    end_date DATE,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    approved_by UUID REFERENCES users(id),
    approved_at TIMESTAMPTZ,

    CONSTRAINT allocation_total_check CHECK (
        cash_allocation_pct + bonds_allocation_pct +
        money_market_allocation_pct + other_allocation_pct = 100
    ),
    CONSTRAINT allocation_range_check CHECK (
        cash_allocation_pct BETWEEN 0 AND 100 AND
        bonds_allocation_pct BETWEEN 0 AND 100 AND
        money_market_allocation_pct BETWEEN 0 AND 100 AND
        other_allocation_pct BETWEEN 0 AND 100
    )
);

-- Fund projections (reserve studies)
CREATE TABLE fund_projections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fund_id UUID NOT NULL REFERENCES reserve_funds(id) ON DELETE CASCADE,

    -- Study details
    study_name VARCHAR(200) NOT NULL,
    study_date DATE NOT NULL,
    projection_years INT NOT NULL DEFAULT 30,

    -- Assumptions
    annual_inflation_rate DECIMAL(5, 2) NOT NULL DEFAULT 2.5,
    annual_interest_rate DECIMAL(5, 2) NOT NULL DEFAULT 1.5,

    -- Current state
    starting_balance DECIMAL(15, 2) NOT NULL,

    -- Recommendations
    recommended_annual_contribution DECIMAL(15, 2),
    funding_status_pct DECIMAL(5, 2),

    -- Status
    is_current BOOLEAN NOT NULL DEFAULT true,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    prepared_by VARCHAR(200),

    CONSTRAINT projection_years_check CHECK (projection_years BETWEEN 1 AND 50)
);

-- Fund projection line items (year-by-year breakdown)
CREATE TABLE fund_projection_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    projection_id UUID NOT NULL REFERENCES fund_projections(id) ON DELETE CASCADE,

    -- Year details
    projection_year INT NOT NULL,
    fiscal_year INT NOT NULL,

    -- Inflows
    contributions DECIMAL(15, 2) NOT NULL DEFAULT 0,
    interest_income DECIMAL(15, 2) NOT NULL DEFAULT 0,

    -- Outflows
    planned_expenditures DECIMAL(15, 2) NOT NULL DEFAULT 0,

    -- Balances
    beginning_balance DECIMAL(15, 2) NOT NULL,
    ending_balance DECIMAL(15, 2) NOT NULL,

    -- Component details (JSON for flexibility)
    expenditure_details JSONB,

    CONSTRAINT projection_year_check CHECK (projection_year >= 0)
);

-- Fund component tracking (e.g., roof, HVAC, elevator)
CREATE TABLE fund_components (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fund_id UUID NOT NULL REFERENCES reserve_funds(id) ON DELETE CASCADE,

    -- Component details
    name VARCHAR(200) NOT NULL,
    description TEXT,
    category VARCHAR(100),

    -- Financial
    current_replacement_cost DECIMAL(15, 2),
    useful_life_years INT,
    remaining_life_years INT,

    -- Status
    condition_rating INT CHECK (condition_rating BETWEEN 1 AND 10),
    last_inspection_date DATE,
    next_replacement_date DATE,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Fund alerts
CREATE TABLE fund_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fund_id UUID NOT NULL REFERENCES reserve_funds(id) ON DELETE CASCADE,

    -- Alert details
    alert_type VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL DEFAULT 'warning',
    title VARCHAR(200) NOT NULL,
    message TEXT NOT NULL,

    -- Thresholds
    threshold_value DECIMAL(15, 2),
    current_value DECIMAL(15, 2),

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    acknowledged_at TIMESTAMPTZ,
    acknowledged_by UUID REFERENCES users(id),
    resolved_at TIMESTAMPTZ,
    resolved_by UUID REFERENCES users(id),

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_reserve_funds_org ON reserve_funds(organization_id);
CREATE INDEX idx_reserve_funds_building ON reserve_funds(building_id);
CREATE INDEX idx_reserve_funds_type ON reserve_funds(fund_type);
CREATE INDEX idx_reserve_funds_active ON reserve_funds(is_active) WHERE is_active = true;

CREATE INDEX idx_contribution_schedules_fund ON fund_contribution_schedules(fund_id);
CREATE INDEX idx_contribution_schedules_next_due ON fund_contribution_schedules(next_due_date)
    WHERE is_active = true;

CREATE INDEX idx_fund_transactions_fund ON fund_transactions(fund_id);
CREATE INDEX idx_fund_transactions_date ON fund_transactions(transaction_date);
CREATE INDEX idx_fund_transactions_type ON fund_transactions(transaction_type);

CREATE INDEX idx_fund_projections_fund ON fund_projections(fund_id);
CREATE INDEX idx_fund_projections_current ON fund_projections(fund_id) WHERE is_current = true;

CREATE INDEX idx_fund_projection_items_projection ON fund_projection_items(projection_id);

CREATE INDEX idx_fund_components_fund ON fund_components(fund_id);
CREATE INDEX idx_fund_components_replacement ON fund_components(next_replacement_date);

CREATE INDEX idx_fund_alerts_fund ON fund_alerts(fund_id);
CREATE INDEX idx_fund_alerts_active ON fund_alerts(fund_id) WHERE is_active = true;

-- Row Level Security
ALTER TABLE reserve_funds ENABLE ROW LEVEL SECURITY;
ALTER TABLE fund_contribution_schedules ENABLE ROW LEVEL SECURITY;
ALTER TABLE fund_transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE fund_investment_policies ENABLE ROW LEVEL SECURITY;
ALTER TABLE fund_projections ENABLE ROW LEVEL SECURITY;
ALTER TABLE fund_projection_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE fund_components ENABLE ROW LEVEL SECURITY;
ALTER TABLE fund_alerts ENABLE ROW LEVEL SECURITY;

-- RLS Policies for reserve_funds
CREATE POLICY reserve_funds_tenant_isolation ON reserve_funds
    FOR ALL USING (
        organization_id = NULLIF(current_setting('app.current_organization_id', true), '')::UUID
    );

-- RLS Policies for fund_contribution_schedules
CREATE POLICY contribution_schedules_tenant_isolation ON fund_contribution_schedules
    FOR ALL USING (
        fund_id IN (
            SELECT id FROM reserve_funds
            WHERE organization_id = NULLIF(current_setting('app.current_organization_id', true), '')::UUID
        )
    );

-- RLS Policies for fund_transactions
CREATE POLICY fund_transactions_tenant_isolation ON fund_transactions
    FOR ALL USING (
        fund_id IN (
            SELECT id FROM reserve_funds
            WHERE organization_id = NULLIF(current_setting('app.current_organization_id', true), '')::UUID
        )
    );

-- RLS Policies for fund_investment_policies
CREATE POLICY investment_policies_tenant_isolation ON fund_investment_policies
    FOR ALL USING (
        fund_id IN (
            SELECT id FROM reserve_funds
            WHERE organization_id = NULLIF(current_setting('app.current_organization_id', true), '')::UUID
        )
    );

-- RLS Policies for fund_projections
CREATE POLICY fund_projections_tenant_isolation ON fund_projections
    FOR ALL USING (
        fund_id IN (
            SELECT id FROM reserve_funds
            WHERE organization_id = NULLIF(current_setting('app.current_organization_id', true), '')::UUID
        )
    );

-- RLS Policies for fund_projection_items
CREATE POLICY projection_items_tenant_isolation ON fund_projection_items
    FOR ALL USING (
        projection_id IN (
            SELECT fp.id FROM fund_projections fp
            JOIN reserve_funds rf ON rf.id = fp.fund_id
            WHERE rf.organization_id = NULLIF(current_setting('app.current_organization_id', true), '')::UUID
        )
    );

-- RLS Policies for fund_components
CREATE POLICY fund_components_tenant_isolation ON fund_components
    FOR ALL USING (
        fund_id IN (
            SELECT id FROM reserve_funds
            WHERE organization_id = NULLIF(current_setting('app.current_organization_id', true), '')::UUID
        )
    );

-- RLS Policies for fund_alerts
CREATE POLICY fund_alerts_tenant_isolation ON fund_alerts
    FOR ALL USING (
        fund_id IN (
            SELECT id FROM reserve_funds
            WHERE organization_id = NULLIF(current_setting('app.current_organization_id', true), '')::UUID
        )
    );

-- Triggers for updated_at
CREATE TRIGGER update_reserve_funds_updated_at
    BEFORE UPDATE ON reserve_funds
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_contribution_schedules_updated_at
    BEFORE UPDATE ON fund_contribution_schedules
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_investment_policies_updated_at
    BEFORE UPDATE ON fund_investment_policies
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_fund_projections_updated_at
    BEFORE UPDATE ON fund_projections
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_fund_components_updated_at
    BEFORE UPDATE ON fund_components
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
