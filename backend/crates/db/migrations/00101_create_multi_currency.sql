-- Epic 145: Multi-Currency & Cross-Border Support
-- Story 145.1: Multi-Currency Configuration
-- Story 145.2: Exchange Rate Management
-- Story 145.3: Cross-Currency Transactions
-- Story 145.4: Cross-Border Lease Management
-- Story 145.5: Consolidated Multi-Currency Reporting

-- =============================================================================
-- ENUM TYPES
-- =============================================================================

-- Supported currencies (EUR, CZK, CHF, GBP, PLN and more)
CREATE TYPE supported_currency AS ENUM (
    'EUR',  -- Euro (base for most EU)
    'CZK',  -- Czech Koruna
    'CHF',  -- Swiss Franc
    'GBP',  -- British Pound
    'PLN',  -- Polish Zloty
    'USD',  -- US Dollar
    'HUF',  -- Hungarian Forint
    'RON',  -- Romanian Leu
    'BGN',  -- Bulgarian Lev
    'HRK',  -- Croatian Kuna (legacy, now EUR)
    'SEK',  -- Swedish Krona
    'DKK',  -- Danish Krone
    'NOK'   -- Norwegian Krone
);

-- Exchange rate source
CREATE TYPE exchange_rate_source AS ENUM (
    'ecb',      -- European Central Bank
    'xe',       -- XE.com
    'manual',   -- Manual override
    'api'       -- Custom API integration
);

-- Transaction conversion status
CREATE TYPE conversion_status AS ENUM (
    'pending',
    'converted',
    'failed',
    'manual'
);

-- Cross-border compliance status
CREATE TYPE compliance_status AS ENUM (
    'compliant',
    'pending_review',
    'non_compliant',
    'exempt'
);

-- Country codes (ISO 3166-1 alpha-2 for EU/EEA focus)
CREATE TYPE country_code AS ENUM (
    'SK',  -- Slovakia
    'CZ',  -- Czech Republic
    'AT',  -- Austria
    'DE',  -- Germany
    'PL',  -- Poland
    'HU',  -- Hungary
    'CH',  -- Switzerland
    'GB',  -- United Kingdom
    'FR',  -- France
    'IT',  -- Italy
    'ES',  -- Spain
    'NL',  -- Netherlands
    'BE',  -- Belgium
    'PT',  -- Portugal
    'IE',  -- Ireland
    'RO',  -- Romania
    'BG',  -- Bulgaria
    'HR',  -- Croatia
    'SI',  -- Slovenia
    'LU',  -- Luxembourg
    'SE',  -- Sweden
    'DK',  -- Denmark
    'NO',  -- Norway
    'FI'   -- Finland
);

-- =============================================================================
-- STORY 145.1: MULTI-CURRENCY CONFIGURATION
-- =============================================================================

-- Organization currency configuration
CREATE TABLE organization_currency_config (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Base currency for the organization
    base_currency supported_currency NOT NULL DEFAULT 'EUR',

    -- Enabled additional currencies
    enabled_currencies supported_currency[] NOT NULL DEFAULT ARRAY['EUR']::supported_currency[],

    -- Default display settings
    display_currency supported_currency,  -- Null = use base currency
    show_original_amount BOOLEAN NOT NULL DEFAULT true,
    decimal_places INTEGER NOT NULL DEFAULT 2,

    -- Exchange rate settings
    exchange_rate_source exchange_rate_source NOT NULL DEFAULT 'ecb',
    auto_update_rates BOOLEAN NOT NULL DEFAULT true,
    update_frequency_hours INTEGER NOT NULL DEFAULT 24,
    last_rate_update TIMESTAMPTZ,

    -- Rounding settings
    rounding_mode VARCHAR(20) NOT NULL DEFAULT 'half_up',  -- half_up, half_down, ceil, floor

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id),

    CONSTRAINT unique_org_currency_config UNIQUE (organization_id)
);

-- Property-level currency settings (override organization defaults)
CREATE TABLE property_currency_config (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Property currency (override for this property)
    default_currency supported_currency NOT NULL,

    -- Country for tax/compliance
    country country_code NOT NULL,

    -- Tax settings
    vat_rate DECIMAL(5, 2),
    vat_registration_number VARCHAR(50),
    local_tax_id VARCHAR(50),

    -- Compliance flags
    requires_local_reporting BOOLEAN NOT NULL DEFAULT false,
    local_accounting_format VARCHAR(50),  -- e.g., 'pohoda', 'money_s3', 'datev'

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_property_currency UNIQUE (building_id)
);

-- =============================================================================
-- STORY 145.2: EXCHANGE RATE MANAGEMENT
-- =============================================================================

-- Historical exchange rates
CREATE TABLE exchange_rates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Currency pair
    from_currency supported_currency NOT NULL,
    to_currency supported_currency NOT NULL,

    -- Rate information
    rate DECIMAL(18, 8) NOT NULL,
    inverse_rate DECIMAL(18, 8) NOT NULL,

    -- Rate date and source
    rate_date DATE NOT NULL,
    source exchange_rate_source NOT NULL,
    source_reference VARCHAR(100),  -- e.g., ECB reference number

    -- For manual overrides
    is_override BOOLEAN NOT NULL DEFAULT false,
    override_reason TEXT,
    overridden_by UUID REFERENCES users(id),

    -- Validity period for the rate
    valid_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    valid_until TIMESTAMPTZ,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Unique constraint per currency pair per date
    CONSTRAINT unique_exchange_rate UNIQUE (from_currency, to_currency, rate_date, source)
);

-- Exchange rate fetch log
CREATE TABLE exchange_rate_fetch_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE,

    -- Fetch details
    source exchange_rate_source NOT NULL,
    fetch_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Result
    success BOOLEAN NOT NULL,
    rates_fetched INTEGER DEFAULT 0,
    error_message TEXT,

    -- Response data (for debugging)
    response_data JSONB,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- STORY 145.3: CROSS-CURRENCY TRANSACTIONS
-- =============================================================================

-- Multi-currency transaction record
CREATE TABLE multi_currency_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE SET NULL,

    -- Reference to original transaction (invoice, payment, etc.)
    source_type VARCHAR(50) NOT NULL,  -- 'invoice', 'payment', 'lease_payment', 'expense'
    source_id UUID NOT NULL,

    -- Original amount
    original_currency supported_currency NOT NULL,
    original_amount DECIMAL(15, 2) NOT NULL,

    -- Converted amount (to organization base currency)
    base_currency supported_currency NOT NULL,
    converted_amount DECIMAL(15, 2) NOT NULL,

    -- Exchange rate used
    exchange_rate DECIMAL(18, 8) NOT NULL,
    exchange_rate_id UUID REFERENCES exchange_rates(id),
    rate_date DATE NOT NULL,

    -- Conversion details
    conversion_status conversion_status NOT NULL DEFAULT 'converted',
    conversion_timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Manual override
    is_rate_override BOOLEAN NOT NULL DEFAULT false,
    override_rate DECIMAL(18, 8),
    override_reason TEXT,
    overridden_by UUID REFERENCES users(id),

    -- Realized gain/loss (for payments at different rate than invoice)
    realized_gain_loss DECIMAL(15, 2) DEFAULT 0,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Currency conversion audit log
CREATE TABLE currency_conversion_audit (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transaction_id UUID NOT NULL REFERENCES multi_currency_transactions(id) ON DELETE CASCADE,

    -- Action
    action VARCHAR(50) NOT NULL,  -- 'created', 'rate_updated', 'override', 'revaluation'

    -- Before/after values
    previous_rate DECIMAL(18, 8),
    new_rate DECIMAL(18, 8),
    previous_amount DECIMAL(15, 2),
    new_amount DECIMAL(15, 2),

    -- Who/when
    performed_by UUID REFERENCES users(id),
    performed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Notes
    notes TEXT
);

-- =============================================================================
-- STORY 145.4: CROSS-BORDER LEASE MANAGEMENT
-- =============================================================================

-- Cross-border lease configuration
CREATE TABLE cross_border_leases (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lease_id UUID NOT NULL,  -- Reference to main lease table
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Property location
    property_country country_code NOT NULL,
    property_currency supported_currency NOT NULL,

    -- Tenant information
    tenant_country country_code,
    tenant_tax_id VARCHAR(50),
    tenant_vat_number VARCHAR(50),

    -- Lease currency settings
    lease_currency supported_currency NOT NULL,
    payment_currency supported_currency NOT NULL,  -- May differ from lease currency

    -- Conversion rules
    convert_at_invoice_date BOOLEAN NOT NULL DEFAULT true,
    convert_at_payment_date BOOLEAN NOT NULL DEFAULT false,
    fixed_exchange_rate DECIMAL(18, 8),  -- If not using market rate
    rate_lock_date DATE,

    -- Tax handling
    local_vat_applicable BOOLEAN NOT NULL DEFAULT true,
    vat_rate DECIMAL(5, 2),
    reverse_charge_vat BOOLEAN NOT NULL DEFAULT false,  -- For B2B cross-border
    withholding_tax_rate DECIMAL(5, 2),

    -- Compliance
    compliance_status compliance_status NOT NULL DEFAULT 'pending_review',
    compliance_notes TEXT,
    last_compliance_check TIMESTAMPTZ,

    -- Country-specific clauses
    local_clauses JSONB,  -- Country-specific lease clauses
    governing_law country_code,
    jurisdiction country_code,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_cross_border_lease UNIQUE (lease_id)
);

-- Cross-border compliance requirements
CREATE TABLE cross_border_compliance_requirements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Country this applies to
    country country_code NOT NULL,

    -- Requirement details
    requirement_type VARCHAR(100) NOT NULL,  -- 'registration', 'reporting', 'documentation'
    requirement_name VARCHAR(200) NOT NULL,
    description TEXT,

    -- Thresholds
    threshold_amount DECIMAL(15, 2),
    threshold_currency supported_currency,

    -- Reporting
    reporting_frequency VARCHAR(50),  -- 'monthly', 'quarterly', 'annual'
    reporting_deadline_days INTEGER,

    -- Document requirements
    required_documents JSONB,

    -- Active/inactive
    is_active BOOLEAN NOT NULL DEFAULT true,
    effective_from DATE NOT NULL,
    effective_until DATE,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- STORY 145.5: CONSOLIDATED MULTI-CURRENCY REPORTING
-- =============================================================================

-- Multi-currency report configuration
CREATE TABLE multi_currency_report_config (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Report settings
    name VARCHAR(200) NOT NULL,
    description TEXT,

    -- Currency display
    report_currency supported_currency NOT NULL,  -- Currency for report totals
    show_original_currencies BOOLEAN NOT NULL DEFAULT true,
    show_conversion_details BOOLEAN NOT NULL DEFAULT true,

    -- Exchange rate for report
    rate_date_type VARCHAR(50) NOT NULL DEFAULT 'end_of_period',  -- 'end_of_period', 'average', 'specific_date'
    specific_rate_date DATE,

    -- Grouping
    group_by_currency BOOLEAN NOT NULL DEFAULT true,
    group_by_country BOOLEAN NOT NULL DEFAULT false,
    group_by_property BOOLEAN NOT NULL DEFAULT true,

    -- Saved report
    is_saved BOOLEAN NOT NULL DEFAULT false,
    is_default BOOLEAN NOT NULL DEFAULT false,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id)
);

-- Multi-currency report snapshots (for historical comparison)
CREATE TABLE multi_currency_report_snapshots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    config_id UUID REFERENCES multi_currency_report_config(id) ON DELETE SET NULL,

    -- Report period
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,

    -- Report currency
    report_currency supported_currency NOT NULL,

    -- Summary data
    total_revenue DECIMAL(15, 2) NOT NULL,
    total_expenses DECIMAL(15, 2) NOT NULL,
    net_income DECIMAL(15, 2) NOT NULL,

    -- Currency breakdown
    currency_breakdown JSONB NOT NULL,  -- {currency: {revenue, expenses, net, exchange_rate}}

    -- Exchange rate impact
    exchange_rate_impact DECIMAL(15, 2),  -- Gain/loss due to exchange rate changes
    unrealized_fx_gain_loss DECIMAL(15, 2),
    realized_fx_gain_loss DECIMAL(15, 2),

    -- Country breakdown
    country_breakdown JSONB,  -- {country: {revenue, expenses, properties}}

    -- Property breakdown
    property_breakdown JSONB,  -- {building_id: {currency, revenue, expenses}}

    -- Exchange rates used
    rates_used JSONB NOT NULL,  -- {currency: rate}
    rate_date DATE NOT NULL,

    -- Metadata
    generated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    generated_by UUID REFERENCES users(id)
);

-- Currency exposure analysis
CREATE TABLE currency_exposure_analysis (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Analysis date
    analysis_date DATE NOT NULL,

    -- Currency
    currency supported_currency NOT NULL,

    -- Exposure amounts
    receivables_amount DECIMAL(15, 2) NOT NULL DEFAULT 0,
    payables_amount DECIMAL(15, 2) NOT NULL DEFAULT 0,
    net_exposure DECIMAL(15, 2) NOT NULL DEFAULT 0,

    -- Asset exposure (property values in this currency)
    asset_value DECIMAL(15, 2) NOT NULL DEFAULT 0,

    -- Revenue/expense exposure
    projected_revenue DECIMAL(15, 2) NOT NULL DEFAULT 0,
    projected_expenses DECIMAL(15, 2) NOT NULL DEFAULT 0,

    -- Risk metrics
    value_at_risk DECIMAL(15, 2),  -- VaR calculation
    expected_shortfall DECIMAL(15, 2),

    -- Hedging (if applicable)
    hedged_amount DECIMAL(15, 2) DEFAULT 0,
    hedge_effectiveness DECIMAL(5, 2),

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_exposure_analysis UNIQUE (organization_id, currency, analysis_date)
);

-- =============================================================================
-- INDEXES
-- =============================================================================

-- Organization currency config
CREATE INDEX idx_org_currency_config_org ON organization_currency_config(organization_id);

-- Property currency config
CREATE INDEX idx_property_currency_org ON property_currency_config(organization_id);
CREATE INDEX idx_property_currency_building ON property_currency_config(building_id);
CREATE INDEX idx_property_currency_country ON property_currency_config(country);

-- Exchange rates
CREATE INDEX idx_exchange_rates_pair ON exchange_rates(from_currency, to_currency);
CREATE INDEX idx_exchange_rates_date ON exchange_rates(rate_date);
CREATE INDEX idx_exchange_rates_source ON exchange_rates(source);
CREATE INDEX idx_exchange_rates_valid ON exchange_rates(valid_from, valid_until);

-- Exchange rate fetch log
CREATE INDEX idx_rate_fetch_log_org ON exchange_rate_fetch_log(organization_id);
CREATE INDEX idx_rate_fetch_log_time ON exchange_rate_fetch_log(fetch_time);

-- Multi-currency transactions
CREATE INDEX idx_multi_currency_tx_org ON multi_currency_transactions(organization_id);
CREATE INDEX idx_multi_currency_tx_building ON multi_currency_transactions(building_id);
CREATE INDEX idx_multi_currency_tx_source ON multi_currency_transactions(source_type, source_id);
CREATE INDEX idx_multi_currency_tx_currency ON multi_currency_transactions(original_currency, base_currency);
CREATE INDEX idx_multi_currency_tx_date ON multi_currency_transactions(rate_date);

-- Currency conversion audit
CREATE INDEX idx_currency_audit_tx ON currency_conversion_audit(transaction_id);
CREATE INDEX idx_currency_audit_time ON currency_conversion_audit(performed_at);

-- Cross-border leases
CREATE INDEX idx_cross_border_lease_org ON cross_border_leases(organization_id);
CREATE INDEX idx_cross_border_lease_country ON cross_border_leases(property_country);
CREATE INDEX idx_cross_border_lease_compliance ON cross_border_leases(compliance_status);

-- Compliance requirements
CREATE INDEX idx_compliance_req_country ON cross_border_compliance_requirements(country);
CREATE INDEX idx_compliance_req_active ON cross_border_compliance_requirements(is_active);

-- Report config
CREATE INDEX idx_report_config_org ON multi_currency_report_config(organization_id);
CREATE INDEX idx_report_config_default ON multi_currency_report_config(organization_id, is_default);

-- Report snapshots
CREATE INDEX idx_report_snapshot_org ON multi_currency_report_snapshots(organization_id);
CREATE INDEX idx_report_snapshot_period ON multi_currency_report_snapshots(period_start, period_end);

-- Currency exposure
CREATE INDEX idx_currency_exposure_org ON currency_exposure_analysis(organization_id);
CREATE INDEX idx_currency_exposure_date ON currency_exposure_analysis(analysis_date);
CREATE INDEX idx_currency_exposure_currency ON currency_exposure_analysis(currency);

-- =============================================================================
-- ROW LEVEL SECURITY
-- =============================================================================

ALTER TABLE organization_currency_config ENABLE ROW LEVEL SECURITY;
ALTER TABLE property_currency_config ENABLE ROW LEVEL SECURITY;
ALTER TABLE exchange_rates ENABLE ROW LEVEL SECURITY;
ALTER TABLE exchange_rate_fetch_log ENABLE ROW LEVEL SECURITY;
ALTER TABLE multi_currency_transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE currency_conversion_audit ENABLE ROW LEVEL SECURITY;
ALTER TABLE cross_border_leases ENABLE ROW LEVEL SECURITY;
ALTER TABLE multi_currency_report_config ENABLE ROW LEVEL SECURITY;
ALTER TABLE multi_currency_report_snapshots ENABLE ROW LEVEL SECURITY;
ALTER TABLE currency_exposure_analysis ENABLE ROW LEVEL SECURITY;

-- RLS Policies
CREATE POLICY "Tenant isolation for organization_currency_config"
    ON organization_currency_config FOR ALL
    USING (organization_id = current_setting('app.current_tenant')::UUID);

CREATE POLICY "Tenant isolation for property_currency_config"
    ON property_currency_config FOR ALL
    USING (organization_id = current_setting('app.current_tenant')::UUID);

-- Exchange rates are global (read-only for tenants)
CREATE POLICY "Public read for exchange_rates"
    ON exchange_rates FOR SELECT
    USING (true);

CREATE POLICY "Tenant isolation for exchange_rate_fetch_log"
    ON exchange_rate_fetch_log FOR ALL
    USING (organization_id IS NULL OR organization_id = current_setting('app.current_tenant')::UUID);

CREATE POLICY "Tenant isolation for multi_currency_transactions"
    ON multi_currency_transactions FOR ALL
    USING (organization_id = current_setting('app.current_tenant')::UUID);

CREATE POLICY "Tenant isolation for currency_conversion_audit"
    ON currency_conversion_audit FOR ALL
    USING (transaction_id IN (
        SELECT id FROM multi_currency_transactions
        WHERE organization_id = current_setting('app.current_tenant')::UUID
    ));

CREATE POLICY "Tenant isolation for cross_border_leases"
    ON cross_border_leases FOR ALL
    USING (organization_id = current_setting('app.current_tenant')::UUID);

-- Compliance requirements are global (read-only)
ALTER TABLE cross_border_compliance_requirements DISABLE ROW LEVEL SECURITY;

CREATE POLICY "Tenant isolation for multi_currency_report_config"
    ON multi_currency_report_config FOR ALL
    USING (organization_id = current_setting('app.current_tenant')::UUID);

CREATE POLICY "Tenant isolation for multi_currency_report_snapshots"
    ON multi_currency_report_snapshots FOR ALL
    USING (organization_id = current_setting('app.current_tenant')::UUID);

CREATE POLICY "Tenant isolation for currency_exposure_analysis"
    ON currency_exposure_analysis FOR ALL
    USING (organization_id = current_setting('app.current_tenant')::UUID);

-- =============================================================================
-- SEED DATA: Initial Exchange Rates (ECB rates as of reference date)
-- =============================================================================

-- Insert some baseline EUR exchange rates
INSERT INTO exchange_rates (from_currency, to_currency, rate, inverse_rate, rate_date, source, source_reference)
VALUES
    ('EUR', 'CZK', 25.085, 0.039864, '2026-01-13', 'ecb', 'ECB-2026-01-13'),
    ('EUR', 'CHF', 0.9420, 1.061571, '2026-01-13', 'ecb', 'ECB-2026-01-13'),
    ('EUR', 'GBP', 0.8420, 1.187648, '2026-01-13', 'ecb', 'ECB-2026-01-13'),
    ('EUR', 'PLN', 4.2850, 0.233372, '2026-01-13', 'ecb', 'ECB-2026-01-13'),
    ('EUR', 'USD', 1.0920, 0.915751, '2026-01-13', 'ecb', 'ECB-2026-01-13'),
    ('EUR', 'HUF', 408.50, 0.002448, '2026-01-13', 'ecb', 'ECB-2026-01-13'),
    ('EUR', 'RON', 4.9750, 0.201005, '2026-01-13', 'ecb', 'ECB-2026-01-13'),
    ('EUR', 'BGN', 1.9558, 0.511300, '2026-01-13', 'ecb', 'ECB-2026-01-13'),
    ('EUR', 'SEK', 11.3250, 0.088300, '2026-01-13', 'ecb', 'ECB-2026-01-13'),
    ('EUR', 'DKK', 7.4580, 0.134084, '2026-01-13', 'ecb', 'ECB-2026-01-13'),
    ('EUR', 'NOK', 11.7850, 0.084856, '2026-01-13', 'ecb', 'ECB-2026-01-13');

-- Seed cross-border compliance requirements for major EU countries
INSERT INTO cross_border_compliance_requirements
    (country, requirement_type, requirement_name, description, threshold_amount, threshold_currency,
     reporting_frequency, reporting_deadline_days, is_active, effective_from)
VALUES
    ('SK', 'registration', 'Slovak Tax Registration', 'Registration with Slovak tax authority for rental income', 0, 'EUR', 'annual', 30, true, '2024-01-01'),
    ('CZ', 'registration', 'Czech Tax Registration', 'Registration with Czech tax authority for rental income', 0, 'CZK', 'annual', 30, true, '2024-01-01'),
    ('DE', 'registration', 'German Tax Registration', 'Anmeldung beim Finanzamt for rental income', 0, 'EUR', 'annual', 60, true, '2024-01-01'),
    ('AT', 'registration', 'Austrian Tax Registration', 'Registration with Finanzamt for rental income', 0, 'EUR', 'annual', 30, true, '2024-01-01'),
    ('PL', 'registration', 'Polish Tax Registration', 'Registration with Polish tax office (Urzad Skarbowy)', 0, 'PLN', 'annual', 30, true, '2024-01-01'),
    ('SK', 'reporting', 'Slovak VAT Reporting', 'VAT reporting for properties over threshold', 50000, 'EUR', 'quarterly', 25, true, '2024-01-01'),
    ('CZ', 'reporting', 'Czech VAT Reporting', 'DPH reporting for properties over threshold', 2000000, 'CZK', 'monthly', 25, true, '2024-01-01'),
    ('GB', 'documentation', 'UK Non-Resident Landlord', 'NRLS registration for non-UK landlords', 0, 'GBP', NULL, NULL, true, '2024-01-01');
