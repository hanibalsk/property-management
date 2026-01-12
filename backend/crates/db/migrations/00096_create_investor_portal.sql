-- Epic 139: Investor Portal & ROI Reporting
-- Provides investment tracking, ROI calculations, and investor dashboard features

-- =============================================================================
-- ENUMS
-- =============================================================================

CREATE TYPE investor_type AS ENUM (
    'individual',
    'institutional',
    'reit',
    'family_office',
    'syndicate',
    'fund'
);

CREATE TYPE investment_status AS ENUM (
    'active',
    'exited',
    'pending',
    'on_hold'
);

CREATE TYPE roi_period AS ENUM (
    'monthly',
    'quarterly',
    'annual',
    'ytd',
    'itd'
);

CREATE TYPE distribution_type AS ENUM (
    'cash_dividend',
    'reinvestment',
    'return_of_capital',
    'capital_gain',
    'interest'
);

CREATE TYPE report_type AS ENUM (
    'performance',
    'tax_summary',
    'distribution_history',
    'portfolio_overview',
    'capital_account'
);

-- =============================================================================
-- INVESTOR PROFILES
-- =============================================================================

CREATE TABLE investor_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,

    -- Profile info
    display_name VARCHAR(200) NOT NULL,
    investor_type investor_type NOT NULL DEFAULT 'individual',
    tax_id VARCHAR(50),
    tax_country VARCHAR(3),

    -- Contact
    email VARCHAR(255),
    phone VARCHAR(50),
    address_line1 VARCHAR(255),
    address_line2 VARCHAR(255),
    city VARCHAR(100),
    state VARCHAR(100),
    postal_code VARCHAR(20),
    country VARCHAR(3),

    -- Preferences
    preferred_currency VARCHAR(3) DEFAULT 'EUR',
    distribution_preference distribution_type DEFAULT 'cash_dividend',
    report_frequency VARCHAR(20) DEFAULT 'quarterly',

    -- KYC/AML
    kyc_verified BOOLEAN DEFAULT FALSE,
    kyc_verified_at TIMESTAMPTZ,
    kyc_document_ids UUID[],
    accredited_investor BOOLEAN DEFAULT FALSE,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id)
);

CREATE INDEX idx_investor_profiles_org ON investor_profiles(organization_id);
CREATE INDEX idx_investor_profiles_user ON investor_profiles(user_id);
CREATE INDEX idx_investor_profiles_type ON investor_profiles(investor_type);

-- =============================================================================
-- INVESTMENT PORTFOLIOS
-- =============================================================================

CREATE TABLE investment_portfolios (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    investor_id UUID NOT NULL REFERENCES investor_profiles(id) ON DELETE CASCADE,

    -- Portfolio info
    name VARCHAR(200) NOT NULL,
    description TEXT,
    status investment_status NOT NULL DEFAULT 'active',

    -- Investment details
    initial_investment DECIMAL(18, 2) NOT NULL,
    current_value DECIMAL(18, 2),
    total_contributions DECIMAL(18, 2) DEFAULT 0,
    total_distributions DECIMAL(18, 2) DEFAULT 0,
    ownership_percentage DECIMAL(8, 4),
    currency VARCHAR(3) DEFAULT 'EUR',

    -- Dates
    investment_date DATE NOT NULL,
    exit_date DATE,
    target_exit_date DATE,

    -- Performance
    irr DECIMAL(8, 4),
    multiple DECIMAL(8, 4),

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id)
);

CREATE INDEX idx_portfolios_org ON investment_portfolios(organization_id);
CREATE INDEX idx_portfolios_investor ON investment_portfolios(investor_id);
CREATE INDEX idx_portfolios_status ON investment_portfolios(status);

-- =============================================================================
-- PORTFOLIO PROPERTIES (Investment in specific properties)
-- =============================================================================

CREATE TABLE portfolio_properties (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    portfolio_id UUID NOT NULL REFERENCES investment_portfolios(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,

    -- Investment details
    investment_amount DECIMAL(18, 2) NOT NULL,
    ownership_share DECIMAL(8, 4) NOT NULL,
    acquisition_date DATE NOT NULL,
    acquisition_cost DECIMAL(18, 2),

    -- Current values
    current_value DECIMAL(18, 2),
    appraised_value DECIMAL(18, 2),
    appraised_at DATE,

    -- Income
    rental_income_share DECIMAL(18, 2) DEFAULT 0,
    operating_expenses_share DECIMAL(18, 2) DEFAULT 0,
    net_income_share DECIMAL(18, 2) DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_portfolio_properties_portfolio ON portfolio_properties(portfolio_id);
CREATE INDEX idx_portfolio_properties_building ON portfolio_properties(building_id);

-- =============================================================================
-- ROI CALCULATIONS
-- =============================================================================

CREATE TABLE roi_calculations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    portfolio_id UUID NOT NULL REFERENCES investment_portfolios(id) ON DELETE CASCADE,

    -- Period
    period_type roi_period NOT NULL,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,

    -- Values
    beginning_value DECIMAL(18, 2) NOT NULL,
    ending_value DECIMAL(18, 2) NOT NULL,
    contributions DECIMAL(18, 2) DEFAULT 0,
    distributions DECIMAL(18, 2) DEFAULT 0,

    -- Returns
    gross_return DECIMAL(18, 2),
    net_return DECIMAL(18, 2),
    return_percentage DECIMAL(8, 4),
    annualized_return DECIMAL(8, 4),

    -- Income breakdown
    rental_income DECIMAL(18, 2) DEFAULT 0,
    other_income DECIMAL(18, 2) DEFAULT 0,
    operating_expenses DECIMAL(18, 2) DEFAULT 0,
    capital_expenditures DECIMAL(18, 2) DEFAULT 0,

    -- Appreciation
    unrealized_gain DECIMAL(18, 2) DEFAULT 0,
    realized_gain DECIMAL(18, 2) DEFAULT 0,

    -- Risk metrics
    volatility DECIMAL(8, 4),
    sharpe_ratio DECIMAL(8, 4),

    -- Benchmarks
    benchmark_return DECIMAL(8, 4),
    alpha DECIMAL(8, 4),

    -- Timestamps
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_roi_calculations_org ON roi_calculations(organization_id);
CREATE INDEX idx_roi_calculations_portfolio ON roi_calculations(portfolio_id);
CREATE INDEX idx_roi_calculations_period ON roi_calculations(period_type, period_start, period_end);

-- =============================================================================
-- DISTRIBUTIONS
-- =============================================================================

CREATE TABLE investor_distributions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    portfolio_id UUID NOT NULL REFERENCES investment_portfolios(id) ON DELETE CASCADE,
    investor_id UUID NOT NULL REFERENCES investor_profiles(id) ON DELETE CASCADE,

    -- Distribution details
    distribution_type distribution_type NOT NULL,
    amount DECIMAL(18, 2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'EUR',

    -- Tax info
    gross_amount DECIMAL(18, 2),
    withholding_tax DECIMAL(18, 2) DEFAULT 0,
    net_amount DECIMAL(18, 2),
    tax_year INTEGER,

    -- Processing
    scheduled_date DATE NOT NULL,
    paid_date DATE,
    payment_method VARCHAR(50),
    payment_reference VARCHAR(100),

    -- Status
    status VARCHAR(50) DEFAULT 'scheduled',

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id)
);

CREATE INDEX idx_distributions_org ON investor_distributions(organization_id);
CREATE INDEX idx_distributions_portfolio ON investor_distributions(portfolio_id);
CREATE INDEX idx_distributions_investor ON investor_distributions(investor_id);
CREATE INDEX idx_distributions_date ON investor_distributions(scheduled_date);

-- =============================================================================
-- INVESTOR REPORTS
-- =============================================================================

CREATE TABLE investor_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    investor_id UUID REFERENCES investor_profiles(id) ON DELETE SET NULL,
    portfolio_id UUID REFERENCES investment_portfolios(id) ON DELETE SET NULL,

    -- Report info
    report_type report_type NOT NULL,
    title VARCHAR(200) NOT NULL,
    description TEXT,

    -- Period
    period_start DATE,
    period_end DATE,

    -- Content
    report_data JSONB NOT NULL,

    -- Files
    pdf_document_id UUID,
    excel_document_id UUID,

    -- Status
    status VARCHAR(50) DEFAULT 'draft',
    published_at TIMESTAMPTZ,
    sent_to_investor BOOLEAN DEFAULT FALSE,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id)
);

CREATE INDEX idx_investor_reports_org ON investor_reports(organization_id);
CREATE INDEX idx_reports_investor ON investor_reports(investor_id);
CREATE INDEX idx_reports_portfolio ON investor_reports(portfolio_id);
CREATE INDEX idx_reports_type ON investor_reports(report_type);

-- =============================================================================
-- CAPITAL CALLS
-- =============================================================================

CREATE TABLE capital_calls (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    portfolio_id UUID NOT NULL REFERENCES investment_portfolios(id) ON DELETE CASCADE,
    investor_id UUID NOT NULL REFERENCES investor_profiles(id) ON DELETE CASCADE,

    -- Call details
    call_number INTEGER NOT NULL,
    amount DECIMAL(18, 2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'EUR',
    purpose VARCHAR(500),

    -- Dates
    call_date DATE NOT NULL,
    due_date DATE NOT NULL,
    funded_date DATE,

    -- Status
    status VARCHAR(50) DEFAULT 'pending',
    funded_amount DECIMAL(18, 2) DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id)
);

CREATE INDEX idx_capital_calls_org ON capital_calls(organization_id);
CREATE INDEX idx_capital_calls_portfolio ON capital_calls(portfolio_id);
CREATE INDEX idx_capital_calls_investor ON capital_calls(investor_id);
CREATE INDEX idx_capital_calls_status ON capital_calls(status);

-- =============================================================================
-- INVESTOR DASHBOARD METRICS
-- =============================================================================

CREATE TABLE investor_dashboard_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    investor_id UUID NOT NULL REFERENCES investor_profiles(id) ON DELETE CASCADE,

    -- Snapshot date
    metric_date DATE NOT NULL,

    -- Portfolio summary
    total_invested DECIMAL(18, 2) NOT NULL DEFAULT 0,
    total_value DECIMAL(18, 2) NOT NULL DEFAULT 0,
    total_distributions DECIMAL(18, 2) DEFAULT 0,
    total_return DECIMAL(18, 2) DEFAULT 0,

    -- Performance metrics
    ytd_return DECIMAL(8, 4),
    itd_return DECIMAL(8, 4),
    irr DECIMAL(8, 4),
    cash_on_cash DECIMAL(8, 4),
    equity_multiple DECIMAL(8, 4),

    -- Allocation
    property_count INTEGER DEFAULT 0,
    portfolio_count INTEGER DEFAULT 0,

    -- Income metrics
    monthly_income DECIMAL(18, 2) DEFAULT 0,
    annual_income DECIMAL(18, 2) DEFAULT 0,
    yield_percentage DECIMAL(8, 4),

    -- Timestamps
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_dashboard_metrics_org ON investor_dashboard_metrics(organization_id);
CREATE INDEX idx_dashboard_metrics_investor ON investor_dashboard_metrics(investor_id);
CREATE INDEX idx_dashboard_metrics_date ON investor_dashboard_metrics(metric_date);
CREATE UNIQUE INDEX idx_dashboard_metrics_unique ON investor_dashboard_metrics(investor_id, metric_date);

-- =============================================================================
-- ROW-LEVEL SECURITY
-- =============================================================================

ALTER TABLE investor_profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE investment_portfolios ENABLE ROW LEVEL SECURITY;
ALTER TABLE portfolio_properties ENABLE ROW LEVEL SECURITY;
ALTER TABLE roi_calculations ENABLE ROW LEVEL SECURITY;
ALTER TABLE investor_distributions ENABLE ROW LEVEL SECURITY;
ALTER TABLE investor_reports ENABLE ROW LEVEL SECURITY;
ALTER TABLE capital_calls ENABLE ROW LEVEL SECURITY;
ALTER TABLE investor_dashboard_metrics ENABLE ROW LEVEL SECURITY;

-- Investor profiles policies
CREATE POLICY investor_profiles_tenant_isolation ON investor_profiles
    USING (organization_id = current_setting('app.current_tenant_id', true)::uuid);

-- Investment portfolios policies
CREATE POLICY investment_portfolios_tenant_isolation ON investment_portfolios
    USING (organization_id = current_setting('app.current_tenant_id', true)::uuid);

-- Portfolio properties policies
CREATE POLICY portfolio_properties_tenant_isolation ON portfolio_properties
    USING (portfolio_id IN (
        SELECT id FROM investment_portfolios
        WHERE organization_id = current_setting('app.current_tenant_id', true)::uuid
    ));

-- ROI calculations policies
CREATE POLICY roi_calculations_tenant_isolation ON roi_calculations
    USING (organization_id = current_setting('app.current_tenant_id', true)::uuid);

-- Investor distributions policies
CREATE POLICY investor_distributions_tenant_isolation ON investor_distributions
    USING (organization_id = current_setting('app.current_tenant_id', true)::uuid);

-- Investor reports policies
CREATE POLICY investor_reports_tenant_isolation ON investor_reports
    USING (organization_id = current_setting('app.current_tenant_id', true)::uuid);

-- Capital calls policies
CREATE POLICY capital_calls_tenant_isolation ON capital_calls
    USING (organization_id = current_setting('app.current_tenant_id', true)::uuid);

-- Dashboard metrics policies
CREATE POLICY investor_dashboard_metrics_tenant_isolation ON investor_dashboard_metrics
    USING (organization_id = current_setting('app.current_tenant_id', true)::uuid);

-- =============================================================================
-- TRIGGERS
-- =============================================================================

CREATE TRIGGER update_investor_profiles_updated_at
    BEFORE UPDATE ON investor_profiles
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_investment_portfolios_updated_at
    BEFORE UPDATE ON investment_portfolios
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_portfolio_properties_updated_at
    BEFORE UPDATE ON portfolio_properties
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_investor_distributions_updated_at
    BEFORE UPDATE ON investor_distributions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_investor_reports_updated_at
    BEFORE UPDATE ON investor_reports
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_capital_calls_updated_at
    BEFORE UPDATE ON capital_calls
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
