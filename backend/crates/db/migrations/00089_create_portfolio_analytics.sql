-- Epic 140: Multi-Property Portfolio Analytics
-- Creates tables for portfolio benchmarking, property performance metrics, and trend analysis.

-- =============================================================================
-- ENUMS
-- =============================================================================

DO $$ BEGIN
    CREATE TYPE benchmark_category AS ENUM ('occupancy', 'revenue', 'expense', 'noi', 'cap_rate', 'maintenance', 'tenant_satisfaction');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE aggregation_period AS ENUM ('daily', 'weekly', 'monthly', 'quarterly', 'yearly');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE comparison_scope AS ENUM ('portfolio', 'building', 'unit_type', 'region', 'market');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- =============================================================================
-- PORTFOLIO BENCHMARKS (Story 140.1)
-- Industry and custom benchmarks for comparison
-- =============================================================================

CREATE TABLE IF NOT EXISTS portfolio_benchmarks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    category benchmark_category NOT NULL,

    -- Target values
    target_value DECIMAL(15, 4) NOT NULL,
    min_acceptable DECIMAL(15, 4),
    max_acceptable DECIMAL(15, 4),

    -- Scope
    scope comparison_scope NOT NULL DEFAULT 'portfolio',
    property_type VARCHAR(100),
    region VARCHAR(255),

    -- Source
    is_industry_standard BOOLEAN NOT NULL DEFAULT false,
    source_name VARCHAR(255),
    source_year INTEGER,

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_portfolio_benchmarks_org ON portfolio_benchmarks(organization_id);
CREATE INDEX IF NOT EXISTS idx_portfolio_benchmarks_category ON portfolio_benchmarks(category);
CREATE INDEX IF NOT EXISTS idx_portfolio_benchmarks_active ON portfolio_benchmarks(is_active) WHERE is_active = true;

-- =============================================================================
-- PROPERTY PERFORMANCE METRICS (Story 140.2)
-- Individual property performance tracking
-- =============================================================================

CREATE TABLE IF NOT EXISTS property_performance_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    period_type aggregation_period NOT NULL DEFAULT 'monthly',

    -- Occupancy metrics
    total_units INTEGER NOT NULL DEFAULT 0,
    occupied_units INTEGER NOT NULL DEFAULT 0,
    occupancy_rate DECIMAL(5, 2) GENERATED ALWAYS AS (
        CASE WHEN total_units > 0
        THEN (occupied_units::DECIMAL / total_units) * 100
        ELSE 0 END
    ) STORED,
    average_lease_term_months DECIMAL(5, 1),
    tenant_turnover_rate DECIMAL(5, 2),

    -- Financial metrics (in currency)
    gross_rental_income DECIMAL(15, 2) NOT NULL DEFAULT 0,
    other_income DECIMAL(15, 2) DEFAULT 0,
    total_revenue DECIMAL(15, 2) GENERATED ALWAYS AS (
        COALESCE(gross_rental_income, 0) + COALESCE(other_income, 0)
    ) STORED,
    operating_expenses DECIMAL(15, 2) DEFAULT 0,
    net_operating_income DECIMAL(15, 2) GENERATED ALWAYS AS (
        COALESCE(gross_rental_income, 0) + COALESCE(other_income, 0) - COALESCE(operating_expenses, 0)
    ) STORED,
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',

    -- Per unit metrics
    revenue_per_unit DECIMAL(15, 2),
    expense_per_unit DECIMAL(15, 2),

    -- Efficiency metrics
    expense_ratio DECIMAL(5, 2),
    collection_rate DECIMAL(5, 2),

    -- Maintenance metrics
    maintenance_requests INTEGER DEFAULT 0,
    avg_resolution_time_hours DECIMAL(10, 2),
    maintenance_cost DECIMAL(15, 2) DEFAULT 0,

    -- Tenant satisfaction
    tenant_satisfaction_score DECIMAL(3, 1),
    complaints_count INTEGER DEFAULT 0,

    -- Property value
    estimated_value DECIMAL(15, 2),
    cap_rate DECIMAL(5, 2),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (building_id, period_start, period_end, period_type)
);

CREATE INDEX IF NOT EXISTS idx_property_metrics_building ON property_performance_metrics(building_id);
CREATE INDEX IF NOT EXISTS idx_property_metrics_period ON property_performance_metrics(period_start, period_end);
CREATE INDEX IF NOT EXISTS idx_property_metrics_type ON property_performance_metrics(period_type);

-- =============================================================================
-- PORTFOLIO AGGREGATED METRICS (Story 140.3)
-- Organization-wide portfolio performance
-- =============================================================================

CREATE TABLE IF NOT EXISTS portfolio_aggregated_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    period_type aggregation_period NOT NULL DEFAULT 'monthly',

    -- Portfolio composition
    total_buildings INTEGER NOT NULL DEFAULT 0,
    total_units INTEGER NOT NULL DEFAULT 0,
    total_sqm DECIMAL(15, 2),

    -- Occupancy
    occupied_units INTEGER NOT NULL DEFAULT 0,
    portfolio_occupancy_rate DECIMAL(5, 2),

    -- Financial aggregates
    total_revenue DECIMAL(15, 2) NOT NULL DEFAULT 0,
    total_expenses DECIMAL(15, 2) DEFAULT 0,
    total_noi DECIMAL(15, 2),
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',

    -- Portfolio averages
    avg_rent_per_unit DECIMAL(15, 2),
    avg_rent_per_sqm DECIMAL(15, 2),
    avg_expense_per_unit DECIMAL(15, 2),
    avg_cap_rate DECIMAL(5, 2),

    -- Performance indicators
    revenue_growth_pct DECIMAL(5, 2),
    expense_growth_pct DECIMAL(5, 2),
    noi_growth_pct DECIMAL(5, 2),

    -- Portfolio value
    estimated_portfolio_value DECIMAL(18, 2),

    -- Diversification metrics
    buildings_by_type JSONB DEFAULT '{}',
    revenue_by_region JSONB DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (organization_id, period_start, period_end, period_type)
);

CREATE INDEX IF NOT EXISTS idx_portfolio_metrics_org ON portfolio_aggregated_metrics(organization_id);
CREATE INDEX IF NOT EXISTS idx_portfolio_metrics_period ON portfolio_aggregated_metrics(period_start, period_end);

-- =============================================================================
-- PROPERTY COMPARISONS (Story 140.4)
-- Side-by-side property comparison data
-- =============================================================================

CREATE TABLE IF NOT EXISTS property_comparisons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,

    -- Properties being compared
    building_ids UUID[] NOT NULL DEFAULT '{}',

    -- Comparison parameters
    comparison_period_start DATE NOT NULL,
    comparison_period_end DATE NOT NULL,
    metrics_to_compare TEXT[] DEFAULT '{}',

    -- Results cached
    comparison_results JSONB DEFAULT '{}',
    rankings JSONB DEFAULT '{}',

    -- Status
    is_saved BOOLEAN NOT NULL DEFAULT false,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_property_comparisons_org ON property_comparisons(organization_id);
CREATE INDEX IF NOT EXISTS idx_property_comparisons_saved ON property_comparisons(is_saved) WHERE is_saved = true;

-- =============================================================================
-- PORTFOLIO TRENDS (Story 140.5)
-- Historical trend data for analytics
-- =============================================================================

CREATE TABLE IF NOT EXISTS portfolio_trends (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,
    metric_name VARCHAR(100) NOT NULL,

    -- Time series data
    recorded_at DATE NOT NULL,
    value DECIMAL(15, 4) NOT NULL,
    previous_value DECIMAL(15, 4),
    change_pct DECIMAL(8, 4),

    -- Context
    period_type aggregation_period NOT NULL DEFAULT 'monthly',
    currency VARCHAR(3),
    notes TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (organization_id, building_id, metric_name, recorded_at, period_type)
);

CREATE INDEX IF NOT EXISTS idx_portfolio_trends_org ON portfolio_trends(organization_id);
CREATE INDEX IF NOT EXISTS idx_portfolio_trends_building ON portfolio_trends(building_id) WHERE building_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_portfolio_trends_metric ON portfolio_trends(metric_name);
CREATE INDEX IF NOT EXISTS idx_portfolio_trends_date ON portfolio_trends(recorded_at DESC);

-- =============================================================================
-- PORTFOLIO ALERT RULES (Story 140.6)
-- Configurable alerts for KPI thresholds
-- =============================================================================

CREATE TABLE IF NOT EXISTS portfolio_alert_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,

    -- Metric to monitor
    metric_name VARCHAR(100) NOT NULL,
    category benchmark_category NOT NULL,

    -- Threshold configuration
    operator VARCHAR(10) NOT NULL DEFAULT '<', -- <, >, <=, >=, =, !=
    threshold_value DECIMAL(15, 4) NOT NULL,

    -- Scope
    scope comparison_scope NOT NULL DEFAULT 'portfolio',
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,

    -- Notification settings
    notify_roles TEXT[] DEFAULT '{}',
    notify_users UUID[] DEFAULT '{}',
    notification_frequency VARCHAR(50) DEFAULT 'immediate', -- immediate, daily, weekly

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    last_triggered_at TIMESTAMPTZ,
    trigger_count INTEGER NOT NULL DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_alert_rules_org ON portfolio_alert_rules(organization_id);
CREATE INDEX IF NOT EXISTS idx_alert_rules_active ON portfolio_alert_rules(is_active) WHERE is_active = true;
CREATE INDEX IF NOT EXISTS idx_alert_rules_building ON portfolio_alert_rules(building_id) WHERE building_id IS NOT NULL;

-- =============================================================================
-- PORTFOLIO ALERTS (Story 140.6)
-- Alert instances when thresholds are breached
-- =============================================================================

CREATE TABLE IF NOT EXISTS portfolio_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rule_id UUID NOT NULL REFERENCES portfolio_alert_rules(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,

    -- Alert details
    metric_name VARCHAR(100) NOT NULL,
    current_value DECIMAL(15, 4) NOT NULL,
    threshold_value DECIMAL(15, 4) NOT NULL,
    deviation_pct DECIMAL(8, 4),

    -- Severity based on deviation
    severity VARCHAR(20) NOT NULL DEFAULT 'warning', -- info, warning, critical

    -- Message
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,

    -- Status
    is_read BOOLEAN NOT NULL DEFAULT false,
    is_resolved BOOLEAN NOT NULL DEFAULT false,
    resolved_at TIMESTAMPTZ,
    resolved_by UUID REFERENCES users(id) ON DELETE SET NULL,
    resolution_notes TEXT,

    -- Notification tracking
    notifications_sent JSONB DEFAULT '[]',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_alerts_rule ON portfolio_alerts(rule_id);
CREATE INDEX IF NOT EXISTS idx_alerts_org ON portfolio_alerts(organization_id);
CREATE INDEX IF NOT EXISTS idx_alerts_unread ON portfolio_alerts(is_read) WHERE is_read = false;
CREATE INDEX IF NOT EXISTS idx_alerts_unresolved ON portfolio_alerts(is_resolved) WHERE is_resolved = false;
CREATE INDEX IF NOT EXISTS idx_alerts_created ON portfolio_alerts(created_at DESC);

-- =============================================================================
-- TRIGGERS
-- =============================================================================

CREATE OR REPLACE FUNCTION update_portfolio_analytics_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_portfolio_benchmarks_updated_at ON portfolio_benchmarks;
CREATE TRIGGER trigger_portfolio_benchmarks_updated_at
    BEFORE UPDATE ON portfolio_benchmarks
    FOR EACH ROW
    EXECUTE FUNCTION update_portfolio_analytics_updated_at();

DROP TRIGGER IF EXISTS trigger_property_comparisons_updated_at ON property_comparisons;
CREATE TRIGGER trigger_property_comparisons_updated_at
    BEFORE UPDATE ON property_comparisons
    FOR EACH ROW
    EXECUTE FUNCTION update_portfolio_analytics_updated_at();

DROP TRIGGER IF EXISTS trigger_alert_rules_updated_at ON portfolio_alert_rules;
CREATE TRIGGER trigger_alert_rules_updated_at
    BEFORE UPDATE ON portfolio_alert_rules
    FOR EACH ROW
    EXECUTE FUNCTION update_portfolio_analytics_updated_at();

-- =============================================================================
-- ROW LEVEL SECURITY
-- =============================================================================

ALTER TABLE portfolio_benchmarks ENABLE ROW LEVEL SECURITY;
ALTER TABLE property_performance_metrics ENABLE ROW LEVEL SECURITY;
ALTER TABLE portfolio_aggregated_metrics ENABLE ROW LEVEL SECURITY;
ALTER TABLE property_comparisons ENABLE ROW LEVEL SECURITY;
ALTER TABLE portfolio_trends ENABLE ROW LEVEL SECURITY;
ALTER TABLE portfolio_alert_rules ENABLE ROW LEVEL SECURITY;
ALTER TABLE portfolio_alerts ENABLE ROW LEVEL SECURITY;

-- Portfolio benchmarks policy
CREATE POLICY portfolio_benchmarks_tenant_isolation ON portfolio_benchmarks
    FOR ALL
    USING (
        is_super_admin()
        OR organization_id = get_current_org_id()
    )
    WITH CHECK (
        is_super_admin()
        OR organization_id = get_current_org_id()
    );

-- Property performance metrics policy (via building)
CREATE POLICY property_metrics_tenant_isolation ON property_performance_metrics
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM buildings b
            WHERE b.id = property_performance_metrics.building_id
            AND b.organization_id = get_current_org_id()
        )
    )
    WITH CHECK (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM buildings b
            WHERE b.id = property_performance_metrics.building_id
            AND b.organization_id = get_current_org_id()
        )
    );

-- Portfolio aggregated metrics policy
CREATE POLICY portfolio_metrics_tenant_isolation ON portfolio_aggregated_metrics
    FOR ALL
    USING (
        is_super_admin()
        OR organization_id = get_current_org_id()
    )
    WITH CHECK (
        is_super_admin()
        OR organization_id = get_current_org_id()
    );

-- Property comparisons policy
CREATE POLICY property_comparisons_tenant_isolation ON property_comparisons
    FOR ALL
    USING (
        is_super_admin()
        OR organization_id = get_current_org_id()
    )
    WITH CHECK (
        is_super_admin()
        OR organization_id = get_current_org_id()
    );

-- Portfolio trends policy
CREATE POLICY portfolio_trends_tenant_isolation ON portfolio_trends
    FOR ALL
    USING (
        is_super_admin()
        OR organization_id = get_current_org_id()
    )
    WITH CHECK (
        is_super_admin()
        OR organization_id = get_current_org_id()
    );

-- Alert rules policy
CREATE POLICY alert_rules_tenant_isolation ON portfolio_alert_rules
    FOR ALL
    USING (
        is_super_admin()
        OR organization_id = get_current_org_id()
    )
    WITH CHECK (
        is_super_admin()
        OR organization_id = get_current_org_id()
    );

-- Alerts policy
CREATE POLICY alerts_tenant_isolation ON portfolio_alerts
    FOR ALL
    USING (
        is_super_admin()
        OR organization_id = get_current_org_id()
    )
    WITH CHECK (
        is_super_admin()
        OR organization_id = get_current_org_id()
    );

-- =============================================================================
-- COMMENTS
-- =============================================================================

COMMENT ON TABLE portfolio_benchmarks IS 'Industry and custom benchmarks for portfolio performance comparison';
COMMENT ON TABLE property_performance_metrics IS 'Individual building performance metrics tracked over time';
COMMENT ON TABLE portfolio_aggregated_metrics IS 'Organization-wide portfolio aggregated performance data';
COMMENT ON TABLE property_comparisons IS 'Saved property comparison configurations and results';
COMMENT ON TABLE portfolio_trends IS 'Historical trend data for portfolio analytics';
COMMENT ON TABLE portfolio_alert_rules IS 'Configurable alert rules for KPI threshold monitoring';
COMMENT ON TABLE portfolio_alerts IS 'Alert instances triggered when thresholds are breached';
