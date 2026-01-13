-- Epic 144: Portfolio Performance Analytics
-- Enables property investors to track and analyze portfolio performance with ROI calculations
-- and market comparisons.

-- =============================================================================
-- STORY 144.1: PORTFOLIO CONFIGURATION
-- =============================================================================

-- Financing type enum
CREATE TYPE financing_type AS ENUM (
    'cash',
    'mortgage',
    'commercial',
    'private_lending',
    'partnership',
    'syndication',
    'mixed'
);

-- Metric period enum
CREATE TYPE metric_period AS ENUM (
    'monthly',
    'quarterly',
    'annual',
    'ytd',
    'since_inception'
);

-- Benchmark source enum
CREATE TYPE benchmark_source AS ENUM (
    'industry',
    'regional',
    'property_type',
    'custom',
    'ncreif_odce',
    'msci_ipd',
    'narreit_index'
);

-- Performance portfolios table
CREATE TABLE performance_portfolios (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,

    -- Investment goals
    target_return_pct DECIMAL(10,4),
    target_exit_year INTEGER,
    investment_strategy VARCHAR(100),

    -- Summary metrics (computed)
    total_invested DECIMAL(18,2) DEFAULT 0,
    total_current_value DECIMAL(18,2) DEFAULT 0,
    total_equity DECIMAL(18,2) DEFAULT 0,
    total_debt DECIMAL(18,2) DEFAULT 0,
    property_count INTEGER DEFAULT 0,

    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',
    is_active BOOLEAN NOT NULL DEFAULT true,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id),

    CONSTRAINT uq_perf_portfolio_name UNIQUE (organization_id, name)
);

CREATE INDEX idx_perf_portfolios_org ON performance_portfolios(organization_id);
CREATE INDEX idx_perf_portfolios_active ON performance_portfolios(organization_id) WHERE is_active = true;

-- Portfolio properties with acquisition and financing details
CREATE TABLE portfolio_properties_perf (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    portfolio_id UUID NOT NULL REFERENCES performance_portfolios(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    property_name VARCHAR(255),

    -- Acquisition details
    acquisition_date DATE NOT NULL,
    acquisition_price DECIMAL(18,2) NOT NULL,
    acquisition_costs DECIMAL(18,2),
    total_acquisition_cost DECIMAL(18,2),

    -- Financing details
    financing_type financing_type NOT NULL DEFAULT 'mortgage',
    down_payment DECIMAL(18,2),
    loan_amount DECIMAL(18,2),
    interest_rate DECIMAL(8,4),
    loan_term_years INTEGER,
    monthly_payment DECIMAL(18,2),
    loan_start_date DATE,
    loan_maturity_date DATE,

    -- Ownership
    ownership_percentage DECIMAL(8,4) NOT NULL DEFAULT 100.00,

    -- Current values
    current_value DECIMAL(18,2),
    current_loan_balance DECIMAL(18,2),
    current_equity DECIMAL(18,2),

    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',
    notes TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT uq_portfolio_building UNIQUE (portfolio_id, building_id)
);

CREATE INDEX idx_portfolio_props_portfolio ON portfolio_properties_perf(portfolio_id);
CREATE INDEX idx_portfolio_props_building ON portfolio_properties_perf(building_id);

-- =============================================================================
-- STORY 144.2: INCOME & EXPENSE TRACKING
-- =============================================================================

-- Transaction type enum
CREATE TYPE transaction_type_portfolio AS ENUM (
    'rental_income',
    'other_income',
    'operating_expense',
    'mortgage_payment',
    'capital_expenditure',
    'tax_payment',
    'insurance',
    'property_management',
    'maintenance',
    'utilities',
    'vacancy_cost',
    'leasing_cost',
    'legal_professional',
    'other'
);

-- Property transactions
CREATE TABLE property_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    portfolio_id UUID NOT NULL REFERENCES performance_portfolios(id) ON DELETE CASCADE,
    property_id UUID NOT NULL REFERENCES portfolio_properties_perf(id) ON DELETE CASCADE,
    transaction_type transaction_type_portfolio NOT NULL,
    category VARCHAR(100),

    amount DECIMAL(18,2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',

    transaction_date DATE NOT NULL,
    period_start DATE,
    period_end DATE,

    description TEXT,
    vendor_name VARCHAR(255),
    reference_number VARCHAR(100),
    document_id UUID REFERENCES documents(id),

    is_recurring BOOLEAN NOT NULL DEFAULT false,
    recurrence_frequency VARCHAR(50),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id)
);

CREATE INDEX idx_prop_trans_portfolio ON property_transactions(portfolio_id);
CREATE INDEX idx_prop_trans_property ON property_transactions(property_id);
CREATE INDEX idx_prop_trans_date ON property_transactions(transaction_date);
CREATE INDEX idx_prop_trans_type ON property_transactions(transaction_type);

-- Property cash flows (monthly summary)
CREATE TABLE property_cash_flows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    portfolio_id UUID NOT NULL REFERENCES performance_portfolios(id) ON DELETE CASCADE,
    property_id UUID NOT NULL REFERENCES portfolio_properties_perf(id) ON DELETE CASCADE,
    period_year INTEGER NOT NULL,
    period_month INTEGER NOT NULL CHECK (period_month BETWEEN 1 AND 12),

    gross_rental_income DECIMAL(18,2) NOT NULL DEFAULT 0,
    other_income DECIMAL(18,2) NOT NULL DEFAULT 0,
    total_income DECIMAL(18,2) NOT NULL DEFAULT 0,

    operating_expenses DECIMAL(18,2) NOT NULL DEFAULT 0,
    mortgage_payment DECIMAL(18,2) NOT NULL DEFAULT 0,
    capital_expenditures DECIMAL(18,2) NOT NULL DEFAULT 0,
    total_expenses DECIMAL(18,2) NOT NULL DEFAULT 0,

    net_operating_income DECIMAL(18,2) NOT NULL DEFAULT 0,
    cash_flow_before_debt DECIMAL(18,2) NOT NULL DEFAULT 0,
    cash_flow_after_debt DECIMAL(18,2) NOT NULL DEFAULT 0,

    vacancy_rate DECIMAL(6,2),
    vacancy_cost DECIMAL(18,2),

    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT uq_property_cash_flow_period UNIQUE (portfolio_id, property_id, period_year, period_month)
);

CREATE INDEX idx_cash_flows_portfolio ON property_cash_flows(portfolio_id);
CREATE INDEX idx_cash_flows_property ON property_cash_flows(property_id);
CREATE INDEX idx_cash_flows_period ON property_cash_flows(period_year, period_month);

-- =============================================================================
-- STORY 144.3: ROI & FINANCIAL METRICS CALCULATOR
-- =============================================================================

-- Financial metrics
CREATE TABLE financial_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    portfolio_id UUID NOT NULL REFERENCES performance_portfolios(id) ON DELETE CASCADE,
    property_id UUID REFERENCES portfolio_properties_perf(id) ON DELETE CASCADE,
    period_type metric_period NOT NULL,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,

    -- Income metrics
    gross_income DECIMAL(18,2) NOT NULL DEFAULT 0,
    effective_gross_income DECIMAL(18,2) NOT NULL DEFAULT 0,
    vacancy_loss DECIMAL(18,2),
    other_income DECIMAL(18,2),

    -- Expense metrics
    operating_expenses DECIMAL(18,2) NOT NULL DEFAULT 0,
    total_debt_service DECIMAL(18,2),

    -- Core metrics
    net_operating_income DECIMAL(18,2) NOT NULL DEFAULT 0,
    cap_rate DECIMAL(10,4),
    cash_on_cash_return DECIMAL(10,4),
    gross_rent_multiplier DECIMAL(10,4),

    -- Advanced metrics
    irr DECIMAL(10,4),
    npv DECIMAL(18,2),
    equity_multiple DECIMAL(10,4),
    dscr DECIMAL(10,4),

    -- Values used for calculations
    property_value DECIMAL(18,2),
    total_investment DECIMAL(18,2),
    total_equity DECIMAL(18,2),
    annual_debt_service DECIMAL(18,2),

    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',
    notes TEXT,

    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT uq_financial_metrics_period UNIQUE (portfolio_id, property_id, period_type, period_start, period_end)
);

CREATE INDEX idx_fin_metrics_portfolio ON financial_metrics(portfolio_id);
CREATE INDEX idx_fin_metrics_property ON financial_metrics(property_id);
CREATE INDEX idx_fin_metrics_period ON financial_metrics(period_start, period_end);

-- =============================================================================
-- STORY 144.4: PERFORMANCE BENCHMARKING
-- =============================================================================

-- Market benchmarks
CREATE TABLE market_benchmarks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,

    source benchmark_source NOT NULL DEFAULT 'industry',
    source_name VARCHAR(255),
    source_url TEXT,
    source_date DATE,

    property_type VARCHAR(100),
    region VARCHAR(100),
    market VARCHAR(100),

    period_year INTEGER NOT NULL,
    period_quarter INTEGER CHECK (period_quarter BETWEEN 1 AND 4),

    -- Benchmark values
    avg_cap_rate DECIMAL(10,4),
    avg_cash_on_cash DECIMAL(10,4),
    avg_noi_per_unit DECIMAL(18,2),
    avg_price_per_unit DECIMAL(18,2),
    avg_price_per_sqm DECIMAL(18,2),
    avg_occupancy DECIMAL(6,2),
    avg_rent_growth DECIMAL(10,4),
    avg_expense_ratio DECIMAL(10,4),
    avg_irr DECIMAL(10,4),
    avg_equity_multiple DECIMAL(10,4),

    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',
    is_active BOOLEAN NOT NULL DEFAULT true,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_benchmarks_org ON market_benchmarks(organization_id);
CREATE INDEX idx_benchmarks_source ON market_benchmarks(source);
CREATE INDEX idx_benchmarks_period ON market_benchmarks(period_year, period_quarter);

-- Benchmark comparisons
CREATE TABLE benchmark_comparisons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    portfolio_id UUID NOT NULL REFERENCES performance_portfolios(id) ON DELETE CASCADE,
    benchmark_id UUID NOT NULL REFERENCES market_benchmarks(id) ON DELETE CASCADE,
    property_id UUID REFERENCES portfolio_properties_perf(id) ON DELETE CASCADE,

    comparison_date DATE NOT NULL,

    -- Actual values
    actual_cap_rate DECIMAL(10,4),
    actual_cash_on_cash DECIMAL(10,4),
    actual_noi_per_unit DECIMAL(18,2),
    actual_occupancy DECIMAL(6,2),
    actual_irr DECIMAL(10,4),
    actual_equity_multiple DECIMAL(10,4),

    -- Variance from benchmark (percentage points)
    cap_rate_variance DECIMAL(10,4),
    cash_on_cash_variance DECIMAL(10,4),
    noi_variance_pct DECIMAL(10,4),
    occupancy_variance DECIMAL(10,4),
    irr_variance DECIMAL(10,4),

    -- Percentile ranking (0-100)
    cap_rate_percentile INTEGER CHECK (cap_rate_percentile BETWEEN 0 AND 100),
    cash_on_cash_percentile INTEGER CHECK (cash_on_cash_percentile BETWEEN 0 AND 100),
    overall_percentile INTEGER CHECK (overall_percentile BETWEEN 0 AND 100),

    performance_score DECIMAL(6,2),
    performance_rating VARCHAR(50),
    summary TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_bench_comp_portfolio ON benchmark_comparisons(portfolio_id);
CREATE INDEX idx_bench_comp_benchmark ON benchmark_comparisons(benchmark_id);
CREATE INDEX idx_bench_comp_date ON benchmark_comparisons(comparison_date);

-- =============================================================================
-- STORY 144.5: PORTFOLIO ANALYTICS DASHBOARD
-- =============================================================================

-- Performance alerts
CREATE TABLE performance_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    portfolio_id UUID NOT NULL REFERENCES performance_portfolios(id) ON DELETE CASCADE,
    property_id UUID REFERENCES portfolio_properties_perf(id) ON DELETE CASCADE,

    alert_type VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL DEFAULT 'info',

    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,

    metric_name VARCHAR(100),
    current_value DECIMAL(18,4),
    threshold_value DECIMAL(18,4),

    is_read BOOLEAN NOT NULL DEFAULT false,
    is_resolved BOOLEAN NOT NULL DEFAULT false,
    resolved_at TIMESTAMPTZ,
    resolved_by UUID REFERENCES users(id),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_perf_alerts_portfolio ON performance_alerts(portfolio_id);
CREATE INDEX idx_perf_alerts_property ON performance_alerts(property_id);
CREATE INDEX idx_perf_alerts_unread ON performance_alerts(portfolio_id) WHERE is_read = false;
CREATE INDEX idx_perf_alerts_unresolved ON performance_alerts(portfolio_id) WHERE is_resolved = false;

-- =============================================================================
-- TRIGGERS
-- =============================================================================

-- Update portfolio updated_at
CREATE OR REPLACE FUNCTION update_perf_portfolio_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_perf_portfolio_updated
    BEFORE UPDATE ON performance_portfolios
    FOR EACH ROW EXECUTE FUNCTION update_perf_portfolio_timestamp();

CREATE TRIGGER trg_portfolio_props_updated
    BEFORE UPDATE ON portfolio_properties_perf
    FOR EACH ROW EXECUTE FUNCTION update_perf_portfolio_timestamp();

CREATE TRIGGER trg_property_trans_updated
    BEFORE UPDATE ON property_transactions
    FOR EACH ROW EXECUTE FUNCTION update_perf_portfolio_timestamp();

CREATE TRIGGER trg_cash_flows_updated
    BEFORE UPDATE ON property_cash_flows
    FOR EACH ROW EXECUTE FUNCTION update_perf_portfolio_timestamp();

CREATE TRIGGER trg_benchmarks_updated
    BEFORE UPDATE ON market_benchmarks
    FOR EACH ROW EXECUTE FUNCTION update_perf_portfolio_timestamp();

-- =============================================================================
-- ROW LEVEL SECURITY
-- =============================================================================

ALTER TABLE performance_portfolios ENABLE ROW LEVEL SECURITY;
ALTER TABLE portfolio_properties_perf ENABLE ROW LEVEL SECURITY;
ALTER TABLE property_transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE property_cash_flows ENABLE ROW LEVEL SECURITY;
ALTER TABLE financial_metrics ENABLE ROW LEVEL SECURITY;
ALTER TABLE market_benchmarks ENABLE ROW LEVEL SECURITY;
ALTER TABLE benchmark_comparisons ENABLE ROW LEVEL SECURITY;
ALTER TABLE performance_alerts ENABLE ROW LEVEL SECURITY;

-- Policies for performance_portfolios
CREATE POLICY perf_portfolios_select ON performance_portfolios
    FOR SELECT USING (
        organization_id IN (
            SELECT organization_id FROM organization_members
            WHERE user_id = current_setting('app.current_user_id', true)::UUID
        )
    );

CREATE POLICY perf_portfolios_insert ON performance_portfolios
    FOR INSERT WITH CHECK (
        organization_id IN (
            SELECT organization_id FROM organization_members
            WHERE user_id = current_setting('app.current_user_id', true)::UUID
        )
    );

CREATE POLICY perf_portfolios_update ON performance_portfolios
    FOR UPDATE USING (
        organization_id IN (
            SELECT organization_id FROM organization_members
            WHERE user_id = current_setting('app.current_user_id', true)::UUID
        )
    );

CREATE POLICY perf_portfolios_delete ON performance_portfolios
    FOR DELETE USING (
        organization_id IN (
            SELECT organization_id FROM organization_members
            WHERE user_id = current_setting('app.current_user_id', true)::UUID
        )
    );

-- =============================================================================
-- COMMENTS
-- =============================================================================

COMMENT ON TABLE performance_portfolios IS 'Epic 144: Investment portfolios for performance tracking';
COMMENT ON TABLE portfolio_properties_perf IS 'Epic 144: Properties with acquisition and financing details';
COMMENT ON TABLE property_transactions IS 'Epic 144: Income and expense transactions per property';
COMMENT ON TABLE property_cash_flows IS 'Epic 144: Monthly cash flow summaries per property';
COMMENT ON TABLE financial_metrics IS 'Epic 144: Calculated financial metrics (NOI, Cap Rate, IRR, etc.)';
COMMENT ON TABLE market_benchmarks IS 'Epic 144: Market benchmarks for performance comparison';
COMMENT ON TABLE benchmark_comparisons IS 'Epic 144: Portfolio vs benchmark comparisons';
COMMENT ON TABLE performance_alerts IS 'Epic 144: Performance alerts and notifications';
