-- Epic 140: Multi-Property Portfolio Analytics
-- Provides cross-property analytics, benchmarking, and trend analysis

-- Enum types for portfolio analytics
CREATE TYPE benchmark_category AS ENUM (
    'occupancy',
    'rent_collection',
    'maintenance_costs',
    'tenant_satisfaction',
    'energy_efficiency',
    'property_value',
    'noi',
    'cap_rate'
);

CREATE TYPE aggregation_period AS ENUM (
    'daily',
    'weekly',
    'monthly',
    'quarterly',
    'annual'
);

CREATE TYPE comparison_scope AS ENUM (
    'portfolio',
    'region',
    'property_type',
    'market',
    'industry'
);

-- Portfolio benchmark definitions
CREATE TABLE portfolio_benchmarks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Benchmark details
    name VARCHAR(200) NOT NULL,
    description TEXT,
    category benchmark_category NOT NULL,
    comparison_scope comparison_scope NOT NULL DEFAULT 'portfolio',

    -- Thresholds
    target_value DECIMAL(15, 4),
    warning_threshold DECIMAL(15, 4),
    critical_threshold DECIMAL(15, 4),

    -- Configuration
    is_higher_better BOOLEAN NOT NULL DEFAULT true,
    is_active BOOLEAN NOT NULL DEFAULT true,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id)
);

-- Property performance metrics (aggregated data)
CREATE TABLE property_performance_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,

    -- Period
    period_type aggregation_period NOT NULL,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,

    -- Occupancy metrics
    occupancy_rate DECIMAL(5, 2),
    total_units INTEGER,
    occupied_units INTEGER,
    vacant_units INTEGER,

    -- Financial metrics
    gross_potential_rent DECIMAL(15, 2),
    collected_rent DECIMAL(15, 2),
    rent_collection_rate DECIMAL(5, 2),
    gross_revenue DECIMAL(15, 2),
    operating_expenses DECIMAL(15, 2),
    net_operating_income DECIMAL(15, 2),
    noi_margin DECIMAL(5, 2),

    -- Per-unit metrics
    revenue_per_unit DECIMAL(10, 2),
    expense_per_unit DECIMAL(10, 2),
    noi_per_unit DECIMAL(10, 2),

    -- Maintenance metrics
    maintenance_requests INTEGER,
    completed_requests INTEGER,
    avg_resolution_time_hours DECIMAL(10, 2),
    maintenance_cost DECIMAL(15, 2),

    -- Tenant metrics
    new_leases INTEGER,
    renewals INTEGER,
    move_outs INTEGER,
    tenant_turnover_rate DECIMAL(5, 2),
    avg_tenant_tenure_months DECIMAL(10, 2),

    -- Energy metrics
    energy_consumption_kwh DECIMAL(15, 2),
    water_consumption_liters DECIMAL(15, 2),
    energy_cost DECIMAL(10, 2),
    energy_cost_per_sqm DECIMAL(10, 4),

    -- Valuation metrics
    property_value DECIMAL(15, 2),
    cap_rate DECIMAL(5, 4),
    price_per_sqm DECIMAL(10, 2),

    -- Calculated
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Unique constraint per property per period
    CONSTRAINT unique_property_period UNIQUE (building_id, period_type, period_start)
);

-- Portfolio-wide aggregated metrics
CREATE TABLE portfolio_aggregated_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Period
    period_type aggregation_period NOT NULL,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,

    -- Portfolio summary
    total_properties INTEGER,
    total_units INTEGER,
    total_sqm DECIMAL(15, 2),

    -- Weighted averages
    avg_occupancy_rate DECIMAL(5, 2),
    avg_rent_collection_rate DECIMAL(5, 2),
    avg_noi_margin DECIMAL(5, 2),
    avg_cap_rate DECIMAL(5, 4),
    avg_tenant_turnover_rate DECIMAL(5, 2),

    -- Totals
    total_gross_revenue DECIMAL(15, 2),
    total_operating_expenses DECIMAL(15, 2),
    total_noi DECIMAL(15, 2),
    total_property_value DECIMAL(15, 2),

    -- Per-unit portfolio averages
    portfolio_revenue_per_unit DECIMAL(10, 2),
    portfolio_expense_per_unit DECIMAL(10, 2),
    portfolio_noi_per_unit DECIMAL(10, 2),

    -- Trend indicators (vs previous period)
    occupancy_trend DECIMAL(5, 2),
    revenue_trend DECIMAL(5, 2),
    noi_trend DECIMAL(5, 2),

    -- Calculated
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Unique constraint per organization per period
    CONSTRAINT unique_org_period UNIQUE (organization_id, period_type, period_start)
);

-- Property comparisons (for ranking and benchmarking)
CREATE TABLE property_comparisons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    benchmark_id UUID NOT NULL REFERENCES portfolio_benchmarks(id) ON DELETE CASCADE,

    -- Period
    period_type aggregation_period NOT NULL,
    period_start DATE NOT NULL,

    -- Comparison data
    actual_value DECIMAL(15, 4),
    target_value DECIMAL(15, 4),
    portfolio_avg DECIMAL(15, 4),

    -- Rankings
    rank_in_portfolio INTEGER,
    percentile DECIMAL(5, 2),

    -- Performance indicators
    variance_from_target DECIMAL(15, 4),
    variance_percentage DECIMAL(10, 4),
    status VARCHAR(50), -- 'exceeds', 'meets', 'warning', 'critical'

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Unique constraint
    CONSTRAINT unique_property_benchmark_period UNIQUE (building_id, benchmark_id, period_start)
);

-- Portfolio trend analysis
CREATE TABLE portfolio_trends (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Metric identification
    metric_name VARCHAR(100) NOT NULL,
    category benchmark_category NOT NULL,

    -- Period range
    trend_start DATE NOT NULL,
    trend_end DATE NOT NULL,
    period_count INTEGER NOT NULL,

    -- Trend statistics
    start_value DECIMAL(15, 4),
    end_value DECIMAL(15, 4),
    min_value DECIMAL(15, 4),
    max_value DECIMAL(15, 4),
    avg_value DECIMAL(15, 4),

    -- Trend direction and strength
    trend_direction VARCHAR(20), -- 'increasing', 'decreasing', 'stable'
    trend_percentage DECIMAL(10, 4),
    slope DECIMAL(15, 6),
    r_squared DECIMAL(5, 4),

    -- Seasonality
    has_seasonality BOOLEAN DEFAULT false,
    seasonal_pattern JSONB,

    -- Forecasting
    forecast_next_period DECIMAL(15, 4),
    forecast_confidence DECIMAL(5, 2),

    -- Data points (for visualization)
    data_points JSONB, -- Array of {date, value} pairs

    -- Metadata
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Alert rules for portfolio metrics
CREATE TABLE portfolio_alert_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    benchmark_id UUID REFERENCES portfolio_benchmarks(id) ON DELETE CASCADE,

    -- Rule configuration
    name VARCHAR(200) NOT NULL,
    description TEXT,
    metric_name VARCHAR(100) NOT NULL,

    -- Conditions
    condition_type VARCHAR(50) NOT NULL, -- 'threshold', 'trend', 'anomaly'
    threshold_value DECIMAL(15, 4),
    threshold_operator VARCHAR(20), -- 'gt', 'lt', 'gte', 'lte', 'eq'

    -- Trend conditions
    trend_direction VARCHAR(20),
    trend_periods INTEGER,
    trend_percentage DECIMAL(10, 4),

    -- Alert settings
    severity VARCHAR(20) NOT NULL DEFAULT 'warning', -- 'info', 'warning', 'critical'
    notification_channels JSONB, -- ['email', 'push', 'sms']
    recipients JSONB, -- Array of user IDs or roles

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    last_triggered_at TIMESTAMPTZ,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id)
);

-- Triggered alerts history
CREATE TABLE portfolio_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    rule_id UUID REFERENCES portfolio_alert_rules(id) ON DELETE SET NULL,
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,

    -- Alert details
    alert_type VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL,
    title VARCHAR(200) NOT NULL,
    message TEXT,

    -- Metric data
    metric_name VARCHAR(100),
    metric_value DECIMAL(15, 4),
    threshold_value DECIMAL(15, 4),

    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'active', -- 'active', 'acknowledged', 'resolved', 'dismissed'
    acknowledged_at TIMESTAMPTZ,
    acknowledged_by UUID REFERENCES users(id),
    resolved_at TIMESTAMPTZ,
    resolved_by UUID REFERENCES users(id),

    -- Metadata
    triggered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX idx_portfolio_benchmarks_org ON portfolio_benchmarks(organization_id);
CREATE INDEX idx_portfolio_benchmarks_category ON portfolio_benchmarks(category);

CREATE INDEX idx_property_perf_org ON property_performance_metrics(organization_id);
CREATE INDEX idx_property_perf_building ON property_performance_metrics(building_id);
CREATE INDEX idx_property_perf_period ON property_performance_metrics(period_type, period_start);

CREATE INDEX idx_portfolio_agg_org ON portfolio_aggregated_metrics(organization_id);
CREATE INDEX idx_portfolio_agg_period ON portfolio_aggregated_metrics(period_type, period_start);

CREATE INDEX idx_property_comp_building ON property_comparisons(building_id);
CREATE INDEX idx_property_comp_benchmark ON property_comparisons(benchmark_id);
CREATE INDEX idx_property_comp_period ON property_comparisons(period_start);

CREATE INDEX idx_portfolio_trends_org ON portfolio_trends(organization_id);
CREATE INDEX idx_portfolio_trends_category ON portfolio_trends(category);

CREATE INDEX idx_portfolio_alert_rules_org ON portfolio_alert_rules(organization_id);
CREATE INDEX idx_portfolio_alerts_org ON portfolio_alerts(organization_id);
CREATE INDEX idx_portfolio_alerts_status ON portfolio_alerts(status);

-- Enable RLS
ALTER TABLE portfolio_benchmarks ENABLE ROW LEVEL SECURITY;
ALTER TABLE property_performance_metrics ENABLE ROW LEVEL SECURITY;
ALTER TABLE portfolio_aggregated_metrics ENABLE ROW LEVEL SECURITY;
ALTER TABLE property_comparisons ENABLE ROW LEVEL SECURITY;
ALTER TABLE portfolio_trends ENABLE ROW LEVEL SECURITY;
ALTER TABLE portfolio_alert_rules ENABLE ROW LEVEL SECURITY;
ALTER TABLE portfolio_alerts ENABLE ROW LEVEL SECURITY;

-- RLS Policies
CREATE POLICY "Tenant isolation for portfolio_benchmarks"
    ON portfolio_benchmarks FOR ALL
    USING (organization_id = current_setting('app.current_tenant')::UUID);

CREATE POLICY "Tenant isolation for property_performance_metrics"
    ON property_performance_metrics FOR ALL
    USING (organization_id = current_setting('app.current_tenant')::UUID);

CREATE POLICY "Tenant isolation for portfolio_aggregated_metrics"
    ON portfolio_aggregated_metrics FOR ALL
    USING (organization_id = current_setting('app.current_tenant')::UUID);

CREATE POLICY "Tenant isolation for property_comparisons"
    ON property_comparisons FOR ALL
    USING (organization_id = current_setting('app.current_tenant')::UUID);

CREATE POLICY "Tenant isolation for portfolio_trends"
    ON portfolio_trends FOR ALL
    USING (organization_id = current_setting('app.current_tenant')::UUID);

CREATE POLICY "Tenant isolation for portfolio_alert_rules"
    ON portfolio_alert_rules FOR ALL
    USING (organization_id = current_setting('app.current_tenant')::UUID);

CREATE POLICY "Tenant isolation for portfolio_alerts"
    ON portfolio_alerts FOR ALL
    USING (organization_id = current_setting('app.current_tenant')::UUID);
