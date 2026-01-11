-- Epic 132: Dynamic Rent Pricing & Market Analytics
-- Creates tables for market data collection, pricing recommendations, and comparative analysis.

-- =============================================================================
-- PRICING SOURCE ENUM
-- =============================================================================

DO $$ BEGIN
    CREATE TYPE pricing_source AS ENUM ('manual', 'api', 'scraper', 'import');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE pricing_recommendation_status AS ENUM ('pending', 'accepted', 'rejected', 'expired');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- =============================================================================
-- MARKET DATA REGIONS (Story 132.1)
-- =============================================================================

CREATE TABLE IF NOT EXISTS market_regions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    country_code CHAR(2) NOT NULL,
    city VARCHAR(255) NOT NULL,
    postal_codes TEXT[] DEFAULT '{}',
    center_lat DECIMAL(10, 7),
    center_lng DECIMAL(10, 7),
    radius_km DECIMAL(10, 2) NOT NULL DEFAULT 5.0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_market_regions_org ON market_regions(organization_id);
CREATE INDEX IF NOT EXISTS idx_market_regions_country ON market_regions(country_code);
CREATE INDEX IF NOT EXISTS idx_market_regions_city ON market_regions(city);

-- =============================================================================
-- MARKET DATA POINTS (Story 132.1)
-- Stores collected market rental data for regions
-- =============================================================================

CREATE TABLE IF NOT EXISTS market_data_points (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    region_id UUID NOT NULL REFERENCES market_regions(id) ON DELETE CASCADE,
    collected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    source pricing_source NOT NULL DEFAULT 'manual',
    source_reference VARCHAR(500),

    -- Property characteristics
    property_type VARCHAR(100) NOT NULL,
    size_sqm DECIMAL(10, 2) NOT NULL,
    rooms INTEGER,
    bathrooms INTEGER,
    floor INTEGER,
    has_parking BOOLEAN DEFAULT false,
    has_balcony BOOLEAN DEFAULT false,
    has_elevator BOOLEAN DEFAULT false,
    year_built INTEGER,

    -- Pricing data
    monthly_rent DECIMAL(15, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',
    price_per_sqm DECIMAL(10, 2) GENERATED ALWAYS AS (monthly_rent / NULLIF(size_sqm, 0)) STORED,

    -- Location
    latitude DECIMAL(10, 7),
    longitude DECIMAL(10, 7),
    postal_code VARCHAR(20),
    district VARCHAR(255),

    -- Metadata
    listing_date DATE,
    days_on_market INTEGER,
    is_furnished BOOLEAN DEFAULT false,
    amenities JSONB DEFAULT '[]',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_market_data_points_region ON market_data_points(region_id);
CREATE INDEX IF NOT EXISTS idx_market_data_points_collected ON market_data_points(collected_at DESC);
CREATE INDEX IF NOT EXISTS idx_market_data_points_type ON market_data_points(property_type);
CREATE INDEX IF NOT EXISTS idx_market_data_points_size ON market_data_points(size_sqm);
CREATE INDEX IF NOT EXISTS idx_market_data_points_rent ON market_data_points(monthly_rent);
CREATE INDEX IF NOT EXISTS idx_market_data_points_price_sqm ON market_data_points(price_per_sqm);

-- =============================================================================
-- MARKET STATISTICS (Story 132.1)
-- Aggregated statistics per region, updated weekly
-- =============================================================================

CREATE TABLE IF NOT EXISTS market_statistics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    region_id UUID NOT NULL REFERENCES market_regions(id) ON DELETE CASCADE,
    property_type VARCHAR(100) NOT NULL,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,

    -- Rent statistics
    avg_rent DECIMAL(15, 2) NOT NULL,
    median_rent DECIMAL(15, 2) NOT NULL,
    min_rent DECIMAL(15, 2) NOT NULL,
    max_rent DECIMAL(15, 2) NOT NULL,

    -- Price per sqm statistics
    avg_price_per_sqm DECIMAL(10, 2) NOT NULL,
    median_price_per_sqm DECIMAL(10, 2) NOT NULL,

    -- Vacancy and market health
    vacancy_rate DECIMAL(5, 2),
    avg_days_on_market DECIMAL(10, 2),
    sample_size INTEGER NOT NULL,

    -- Trend indicators
    rent_change_pct DECIMAL(5, 2),
    rent_change_vs_last_year DECIMAL(5, 2),

    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (region_id, property_type, period_start, period_end)
);

CREATE INDEX IF NOT EXISTS idx_market_statistics_region ON market_statistics(region_id);
CREATE INDEX IF NOT EXISTS idx_market_statistics_period ON market_statistics(period_start, period_end);
CREATE INDEX IF NOT EXISTS idx_market_statistics_type ON market_statistics(property_type);

-- =============================================================================
-- PRICING RECOMMENDATIONS (Story 132.2)
-- AI-generated pricing suggestions for units
-- =============================================================================

CREATE TABLE IF NOT EXISTS pricing_recommendations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    generated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Price range
    min_price DECIMAL(15, 2) NOT NULL,
    optimal_price DECIMAL(15, 2) NOT NULL,
    max_price DECIMAL(15, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',

    -- Confidence and status
    confidence_score DECIMAL(5, 2) NOT NULL DEFAULT 0,
    status pricing_recommendation_status NOT NULL DEFAULT 'pending',
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '30 days'),

    -- Factors used in recommendation
    factors JSONB NOT NULL DEFAULT '{}',

    -- Comparables used
    comparables_count INTEGER NOT NULL DEFAULT 0,
    market_stats_id UUID REFERENCES market_statistics(id) ON DELETE SET NULL,

    -- User actions
    accepted_price DECIMAL(15, 2),
    accepted_at TIMESTAMPTZ,
    accepted_by UUID REFERENCES users(id) ON DELETE SET NULL,
    rejection_reason TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_pricing_recommendations_unit ON pricing_recommendations(unit_id);
CREATE INDEX IF NOT EXISTS idx_pricing_recommendations_status ON pricing_recommendations(status);
CREATE INDEX IF NOT EXISTS idx_pricing_recommendations_generated ON pricing_recommendations(generated_at DESC);
CREATE INDEX IF NOT EXISTS idx_pricing_recommendations_expires ON pricing_recommendations(expires_at) WHERE status = 'pending';

-- =============================================================================
-- UNIT PRICING HISTORY (Story 132.3)
-- Tracks actual rent prices set for units over time
-- =============================================================================

CREATE TABLE IF NOT EXISTS unit_pricing_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    effective_date DATE NOT NULL,
    end_date DATE,

    monthly_rent DECIMAL(15, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',

    -- Source of price change
    recommendation_id UUID REFERENCES pricing_recommendations(id) ON DELETE SET NULL,
    change_reason TEXT,
    changed_by UUID REFERENCES users(id) ON DELETE SET NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_unit_pricing_history_unit ON unit_pricing_history(unit_id);
CREATE INDEX IF NOT EXISTS idx_unit_pricing_history_effective ON unit_pricing_history(effective_date DESC);
CREATE INDEX IF NOT EXISTS idx_unit_pricing_history_active ON unit_pricing_history(unit_id, effective_date DESC) WHERE end_date IS NULL;

-- =============================================================================
-- COMPARATIVE MARKET ANALYSIS (Story 132.4)
-- Saved CMA reports for investors
-- =============================================================================

CREATE TABLE IF NOT EXISTS comparative_market_analyses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    name VARCHAR(255) NOT NULL,

    -- Analysis scope
    region_id UUID REFERENCES market_regions(id) ON DELETE SET NULL,
    property_type VARCHAR(100),

    -- Results
    avg_price_per_sqm DECIMAL(10, 2),
    avg_rental_yield DECIMAL(5, 2),
    appreciation_trend DECIMAL(5, 2),

    -- Report data
    analysis_data JSONB NOT NULL DEFAULT '{}',
    properties_compared UUID[] DEFAULT '{}',

    -- Status
    is_archived BOOLEAN NOT NULL DEFAULT false,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_cma_org ON comparative_market_analyses(organization_id);
CREATE INDEX IF NOT EXISTS idx_cma_created_by ON comparative_market_analyses(created_by);
CREATE INDEX IF NOT EXISTS idx_cma_region ON comparative_market_analyses(region_id) WHERE region_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_cma_archived ON comparative_market_analyses(is_archived);

-- =============================================================================
-- CMA PROPERTY COMPARISONS (Story 132.4)
-- Individual properties in a CMA report
-- =============================================================================

CREATE TABLE IF NOT EXISTS cma_property_comparisons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cma_id UUID NOT NULL REFERENCES comparative_market_analyses(id) ON DELETE CASCADE,
    unit_id UUID REFERENCES units(id) ON DELETE SET NULL,

    -- Property details (may be external)
    address TEXT NOT NULL,
    property_type VARCHAR(100) NOT NULL,
    size_sqm DECIMAL(10, 2) NOT NULL,
    rooms INTEGER,
    year_built INTEGER,

    -- Pricing
    monthly_rent DECIMAL(15, 2),
    sale_price DECIMAL(15, 2),
    price_per_sqm DECIMAL(10, 2),
    rental_yield DECIMAL(5, 2),
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',

    -- Comparison metrics
    distance_km DECIMAL(10, 2),
    similarity_score DECIMAL(5, 2),
    notes TEXT,

    -- Source
    source VARCHAR(100) NOT NULL DEFAULT 'manual',
    source_url TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_cma_comparisons_cma ON cma_property_comparisons(cma_id);
CREATE INDEX IF NOT EXISTS idx_cma_comparisons_unit ON cma_property_comparisons(unit_id) WHERE unit_id IS NOT NULL;

-- =============================================================================
-- TRIGGERS
-- =============================================================================

CREATE OR REPLACE FUNCTION update_market_pricing_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_market_regions_updated_at ON market_regions;
CREATE TRIGGER trigger_market_regions_updated_at
    BEFORE UPDATE ON market_regions
    FOR EACH ROW
    EXECUTE FUNCTION update_market_pricing_updated_at();

DROP TRIGGER IF EXISTS trigger_pricing_recommendations_updated_at ON pricing_recommendations;
CREATE TRIGGER trigger_pricing_recommendations_updated_at
    BEFORE UPDATE ON pricing_recommendations
    FOR EACH ROW
    EXECUTE FUNCTION update_market_pricing_updated_at();

DROP TRIGGER IF EXISTS trigger_cma_updated_at ON comparative_market_analyses;
CREATE TRIGGER trigger_cma_updated_at
    BEFORE UPDATE ON comparative_market_analyses
    FOR EACH ROW
    EXECUTE FUNCTION update_market_pricing_updated_at();

-- =============================================================================
-- ROW LEVEL SECURITY
-- =============================================================================

ALTER TABLE market_regions ENABLE ROW LEVEL SECURITY;
ALTER TABLE market_data_points ENABLE ROW LEVEL SECURITY;
ALTER TABLE market_statistics ENABLE ROW LEVEL SECURITY;
ALTER TABLE pricing_recommendations ENABLE ROW LEVEL SECURITY;
ALTER TABLE unit_pricing_history ENABLE ROW LEVEL SECURITY;
ALTER TABLE comparative_market_analyses ENABLE ROW LEVEL SECURITY;
ALTER TABLE cma_property_comparisons ENABLE ROW LEVEL SECURITY;

-- Market regions policy
CREATE POLICY market_regions_tenant_isolation ON market_regions
    FOR ALL
    USING (
        is_super_admin()
        OR organization_id = get_current_org_id()
    );

-- Market data points policy (via region)
CREATE POLICY market_data_points_tenant_isolation ON market_data_points
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM market_regions mr
            WHERE mr.id = market_data_points.region_id
            AND mr.organization_id = get_current_org_id()
        )
    );

-- Market statistics policy (via region)
CREATE POLICY market_statistics_tenant_isolation ON market_statistics
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM market_regions mr
            WHERE mr.id = market_statistics.region_id
            AND mr.organization_id = get_current_org_id()
        )
    );

-- Pricing recommendations policy (via unit)
CREATE POLICY pricing_recommendations_tenant_isolation ON pricing_recommendations
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM units u
            JOIN buildings b ON b.id = u.building_id
            WHERE u.id = pricing_recommendations.unit_id
            AND b.organization_id = get_current_org_id()
        )
    );

-- Unit pricing history policy (via unit)
CREATE POLICY unit_pricing_history_tenant_isolation ON unit_pricing_history
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM units u
            JOIN buildings b ON b.id = u.building_id
            WHERE u.id = unit_pricing_history.unit_id
            AND b.organization_id = get_current_org_id()
        )
    );

-- CMA policy
CREATE POLICY cma_tenant_isolation ON comparative_market_analyses
    FOR ALL
    USING (
        is_super_admin()
        OR organization_id = get_current_org_id()
    );

-- CMA comparisons policy (via CMA)
CREATE POLICY cma_comparisons_tenant_isolation ON cma_property_comparisons
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM comparative_market_analyses cma
            WHERE cma.id = cma_property_comparisons.cma_id
            AND cma.organization_id = get_current_org_id()
        )
    );

-- =============================================================================
-- COMMENTS
-- =============================================================================

COMMENT ON TABLE market_regions IS 'Geographic regions for market data collection';
COMMENT ON TABLE market_data_points IS 'Individual market rental data points collected from various sources';
COMMENT ON TABLE market_statistics IS 'Aggregated market statistics per region and property type';
COMMENT ON TABLE pricing_recommendations IS 'AI-generated pricing recommendations for units';
COMMENT ON TABLE unit_pricing_history IS 'Historical record of actual rent prices for units';
COMMENT ON TABLE comparative_market_analyses IS 'Saved comparative market analysis reports';
COMMENT ON TABLE cma_property_comparisons IS 'Properties compared in a CMA report';
