-- Epic 138: Automated Property Valuation Model (AVM)
-- Migration: 00088_create_property_valuation.sql
-- Description: Property valuation system with AVM algorithms, market analytics,
--              comparable sales analysis, and automated appraisal tools

-- Valuation model types
CREATE TYPE valuation_model_type AS ENUM (
    'hedonic',           -- Hedonic pricing model
    'comparable_sales',  -- Comparable sales approach
    'income_approach',   -- Income capitalization approach
    'cost_approach',     -- Replacement cost approach
    'hybrid',            -- Hybrid/ensemble model
    'machine_learning',  -- ML-based model
    'neural_network'     -- Deep learning model
);

-- Valuation status
CREATE TYPE valuation_status AS ENUM (
    'draft',
    'pending_review',
    'in_progress',
    'completed',
    'approved',
    'rejected',
    'expired'
);

-- Confidence level for valuations
CREATE TYPE valuation_confidence AS ENUM (
    'very_low',
    'low',
    'medium',
    'high',
    'very_high'
);

-- Property condition rating
CREATE TYPE property_condition AS ENUM (
    'excellent',
    'very_good',
    'good',
    'average',
    'fair',
    'poor',
    'very_poor'
);

-- Market trend direction
CREATE TYPE market_trend AS ENUM (
    'strong_growth',
    'moderate_growth',
    'stable',
    'moderate_decline',
    'strong_decline'
);

-- Adjustment type for comparable analysis
CREATE TYPE adjustment_type AS ENUM (
    'location',
    'size',
    'age',
    'condition',
    'features',
    'market_time',
    'financing',
    'sale_type',
    'lot_size',
    'view',
    'basement',
    'garage',
    'pool',
    'renovation',
    'other'
);

-- ===========================================================================
-- Property Valuation Models (AVM configurations)
-- ===========================================================================
CREATE TABLE property_valuation_models (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    model_type valuation_model_type NOT NULL,
    -- Model configuration (weights, parameters, etc.)
    model_config JSONB NOT NULL DEFAULT '{}',
    -- Feature weights for hedonic models
    feature_weights JSONB DEFAULT '{}',
    -- Validation metrics
    r_squared DECIMAL(5, 4),          -- Model fit (0-1)
    mean_absolute_error DECIMAL(12, 2),
    mean_percentage_error DECIMAL(5, 2),
    -- Model version and training info
    version INTEGER NOT NULL DEFAULT 1,
    training_sample_size INTEGER,
    last_trained_at TIMESTAMPTZ,
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_default BOOLEAN NOT NULL DEFAULT false,
    -- Metadata
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_valuation_models_org ON property_valuation_models(organization_id);
CREATE INDEX idx_valuation_models_type ON property_valuation_models(model_type);
CREATE INDEX idx_valuation_models_active ON property_valuation_models(organization_id, is_active) WHERE is_active = true;

-- ===========================================================================
-- Property Valuations (actual valuations)
-- ===========================================================================
CREATE TABLE avm_property_valuations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    property_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE SET NULL,
    model_id UUID REFERENCES property_valuation_models(id) ON DELETE SET NULL,
    -- Valuation details
    valuation_date DATE NOT NULL,
    effective_date DATE NOT NULL,
    expiration_date DATE,
    status valuation_status NOT NULL DEFAULT 'draft',
    -- Estimated values
    estimated_value DECIMAL(14, 2) NOT NULL,
    value_range_low DECIMAL(14, 2),
    value_range_high DECIMAL(14, 2),
    confidence_level valuation_confidence NOT NULL DEFAULT 'medium',
    confidence_score DECIMAL(5, 4),  -- 0-1 score
    -- Price per unit metrics
    price_per_sqm DECIMAL(10, 2),
    price_per_sqft DECIMAL(10, 2),
    -- Market context
    market_value DECIMAL(14, 2),
    insurance_value DECIMAL(14, 2),
    tax_assessed_value DECIMAL(14, 2),
    replacement_cost DECIMAL(14, 2),
    land_value DECIMAL(14, 2),
    improvement_value DECIMAL(14, 2),
    -- Property characteristics at valuation time
    property_condition property_condition,
    effective_age INTEGER,  -- Years
    remaining_economic_life INTEGER,  -- Years
    -- Market analysis
    market_trend market_trend DEFAULT 'stable',
    days_on_market_estimate INTEGER,
    absorption_rate DECIMAL(5, 2),  -- % per month
    -- Input data snapshot
    property_data JSONB DEFAULT '{}',
    market_data JSONB DEFAULT '{}',
    -- Methodology notes
    methodology_notes TEXT,
    assumptions TEXT,
    limiting_conditions TEXT,
    -- Review information
    reviewed_by UUID REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    review_notes TEXT,
    -- Metadata
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_valuations_org ON avm_property_valuations(organization_id);
CREATE INDEX idx_valuations_property ON avm_property_valuations(property_id);
CREATE INDEX idx_valuations_building ON avm_property_valuations(building_id);
CREATE INDEX idx_valuations_date ON avm_property_valuations(valuation_date DESC);
CREATE INDEX idx_valuations_status ON avm_property_valuations(organization_id, status);
CREATE INDEX idx_valuations_value ON avm_property_valuations(estimated_value);

-- ===========================================================================
-- Comparable Sales
-- ===========================================================================
CREATE TABLE valuation_comparables (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    valuation_id UUID REFERENCES avm_property_valuations(id) ON DELETE CASCADE,
    -- Comparable property info (may or may not be in our system)
    comparable_property_id UUID REFERENCES units(id) ON DELETE SET NULL,
    -- External comparable data
    external_address TEXT,
    external_city VARCHAR(255),
    external_postal_code VARCHAR(20),
    external_country VARCHAR(2) DEFAULT 'SK',
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),
    -- Sale information
    sale_date DATE NOT NULL,
    sale_price DECIMAL(14, 2) NOT NULL,
    sale_price_per_sqm DECIMAL(10, 2),
    -- Property characteristics
    property_type VARCHAR(100),
    total_area_sqm DECIMAL(10, 2),
    lot_size_sqm DECIMAL(12, 2),
    year_built INTEGER,
    bedrooms INTEGER,
    bathrooms DECIMAL(3, 1),
    floors INTEGER,
    parking_spaces INTEGER,
    condition property_condition,
    -- Distance and similarity
    distance_km DECIMAL(8, 3),
    similarity_score DECIMAL(5, 4),  -- 0-1
    weight DECIMAL(5, 4),  -- Weight in final valuation
    -- Adjusted value
    gross_adjustment_percent DECIMAL(6, 2),
    net_adjustment_percent DECIMAL(6, 2),
    adjusted_price DECIMAL(14, 2),
    -- Data source
    data_source VARCHAR(100),
    source_reference VARCHAR(255),
    is_verified BOOLEAN DEFAULT false,
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_comparables_org ON valuation_comparables(organization_id);
CREATE INDEX idx_comparables_valuation ON valuation_comparables(valuation_id);
CREATE INDEX idx_comparables_sale_date ON valuation_comparables(sale_date DESC);
CREATE INDEX idx_comparables_location ON valuation_comparables(latitude, longitude);

-- ===========================================================================
-- Comparable Adjustments
-- ===========================================================================
CREATE TABLE comparable_adjustments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    comparable_id UUID NOT NULL REFERENCES valuation_comparables(id) ON DELETE CASCADE,
    adjustment_type adjustment_type NOT NULL,
    adjustment_name VARCHAR(255) NOT NULL,
    -- Adjustment values
    subject_value TEXT,
    comparable_value TEXT,
    adjustment_amount DECIMAL(12, 2) NOT NULL,
    adjustment_percent DECIMAL(6, 2),
    -- Explanation
    justification TEXT,
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_adjustments_comparable ON comparable_adjustments(comparable_id);
CREATE INDEX idx_adjustments_type ON comparable_adjustments(adjustment_type);

-- ===========================================================================
-- Market Data (regional/neighborhood market statistics)
-- ===========================================================================
CREATE TABLE valuation_market_data (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    -- Geographic scope
    region VARCHAR(100),
    city VARCHAR(255),
    district VARCHAR(255),
    neighborhood VARCHAR(255),
    postal_code VARCHAR(20),
    -- Property type scope
    property_type VARCHAR(100),
    -- Time period
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    -- Market statistics
    median_price DECIMAL(14, 2),
    average_price DECIMAL(14, 2),
    price_per_sqm_median DECIMAL(10, 2),
    price_per_sqm_average DECIMAL(10, 2),
    price_per_sqm_min DECIMAL(10, 2),
    price_per_sqm_max DECIMAL(10, 2),
    -- Volume metrics
    total_transactions INTEGER,
    total_volume DECIMAL(16, 2),
    -- Time on market
    avg_days_on_market DECIMAL(8, 2),
    median_days_on_market INTEGER,
    -- Price trends
    price_change_percent DECIMAL(6, 2),
    price_change_yoy DECIMAL(6, 2),  -- Year over year
    price_change_mom DECIMAL(6, 2),  -- Month over month
    market_trend market_trend DEFAULT 'stable',
    -- Supply and demand
    active_listings INTEGER,
    new_listings INTEGER,
    pending_sales INTEGER,
    months_of_supply DECIMAL(5, 2),
    absorption_rate DECIMAL(5, 2),
    list_to_sale_ratio DECIMAL(5, 4),
    -- Rental market (for income approach)
    avg_rent_per_sqm DECIMAL(8, 2),
    rent_yield_percent DECIMAL(5, 2),
    vacancy_rate DECIMAL(5, 2),
    -- Data source
    data_source VARCHAR(100),
    is_verified BOOLEAN DEFAULT false,
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (organization_id, region, city, district, property_type, period_start, period_end)
);

CREATE INDEX idx_market_data_org ON valuation_market_data(organization_id);
CREATE INDEX idx_market_data_location ON valuation_market_data(city, district);
CREATE INDEX idx_market_data_period ON valuation_market_data(period_start, period_end);
CREATE INDEX idx_market_data_property_type ON valuation_market_data(property_type);

-- ===========================================================================
-- Valuation History (track value changes over time)
-- ===========================================================================
CREATE TABLE avm_property_value_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    property_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    valuation_id UUID REFERENCES avm_property_valuations(id) ON DELETE SET NULL,
    -- Value snapshot
    record_date DATE NOT NULL,
    estimated_value DECIMAL(14, 2) NOT NULL,
    price_per_sqm DECIMAL(10, 2),
    confidence_level valuation_confidence,
    -- Change metrics
    previous_value DECIMAL(14, 2),
    value_change DECIMAL(14, 2),
    value_change_percent DECIMAL(6, 2),
    -- Source of value
    value_source VARCHAR(100),  -- 'avm', 'manual', 'appraisal', 'sale'
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_value_history_org ON avm_property_value_history(organization_id);
CREATE INDEX idx_value_history_property ON avm_property_value_history(property_id);
CREATE INDEX idx_value_history_date ON avm_property_value_history(property_id, record_date DESC);

-- ===========================================================================
-- Valuation Requests (user-initiated valuation requests)
-- ===========================================================================
CREATE TABLE valuation_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    property_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    -- Request details
    request_type VARCHAR(50) NOT NULL DEFAULT 'standard',  -- 'standard', 'urgent', 'detailed', 'appraisal'
    purpose VARCHAR(255),  -- 'sale', 'refinance', 'insurance', 'tax_appeal', 'estate', 'other'
    priority INTEGER NOT NULL DEFAULT 5,  -- 1 (highest) to 10 (lowest)
    -- Status tracking
    status valuation_status NOT NULL DEFAULT 'pending_review',
    -- Dates
    requested_date DATE NOT NULL DEFAULT CURRENT_DATE,
    due_date DATE,
    completed_date DATE,
    -- Resulting valuation
    valuation_id UUID REFERENCES avm_property_valuations(id) ON DELETE SET NULL,
    -- Assignment
    assigned_to UUID REFERENCES users(id),
    assigned_at TIMESTAMPTZ,
    -- Notes
    requester_notes TEXT,
    internal_notes TEXT,
    -- Metadata
    requested_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_valuation_requests_org ON valuation_requests(organization_id);
CREATE INDEX idx_valuation_requests_property ON valuation_requests(property_id);
CREATE INDEX idx_valuation_requests_status ON valuation_requests(status);
CREATE INDEX idx_valuation_requests_assigned ON valuation_requests(assigned_to) WHERE assigned_to IS NOT NULL;
CREATE INDEX idx_valuation_requests_due ON valuation_requests(due_date) WHERE status NOT IN ('completed', 'rejected');

-- ===========================================================================
-- Property Features (for hedonic model inputs)
-- ===========================================================================
CREATE TABLE property_valuation_features (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    property_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    -- Feature data (current state)
    recorded_date DATE NOT NULL DEFAULT CURRENT_DATE,
    -- Physical characteristics
    total_area_sqm DECIMAL(10, 2),
    living_area_sqm DECIMAL(10, 2),
    lot_size_sqm DECIMAL(12, 2),
    year_built INTEGER,
    year_renovated INTEGER,
    floors INTEGER,
    bedrooms INTEGER,
    bathrooms DECIMAL(3, 1),
    half_baths INTEGER,
    -- Quality ratings (1-10)
    construction_quality INTEGER,
    interior_quality INTEGER,
    exterior_quality INTEGER,
    -- Features as JSONB for flexibility
    features JSONB DEFAULT '{}',
    -- Amenities flags
    has_garage BOOLEAN DEFAULT false,
    garage_spaces INTEGER DEFAULT 0,
    has_pool BOOLEAN DEFAULT false,
    has_basement BOOLEAN DEFAULT false,
    basement_finished BOOLEAN DEFAULT false,
    has_fireplace BOOLEAN DEFAULT false,
    has_central_ac BOOLEAN DEFAULT false,
    has_central_heat BOOLEAN DEFAULT false,
    -- Location factors
    walk_score INTEGER,
    transit_score INTEGER,
    bike_score INTEGER,
    school_rating DECIMAL(3, 1),
    -- View and lot
    view_quality INTEGER,  -- 1-10
    lot_shape VARCHAR(50),  -- 'regular', 'irregular', 'corner', 'cul-de-sac'
    topography VARCHAR(50),  -- 'level', 'gentle_slope', 'steep', 'rolling'
    -- Condition assessment
    condition property_condition,
    condition_score INTEGER,  -- 1-100
    -- Metadata
    assessed_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_valuation_features_org ON property_valuation_features(organization_id);
CREATE INDEX idx_valuation_features_property ON property_valuation_features(property_id);
CREATE UNIQUE INDEX idx_valuation_features_current ON property_valuation_features(property_id, recorded_date);

-- ===========================================================================
-- Valuation Reports
-- ===========================================================================
CREATE TABLE valuation_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    valuation_id UUID NOT NULL REFERENCES avm_property_valuations(id) ON DELETE CASCADE,
    -- Report info
    report_type VARCHAR(100) NOT NULL DEFAULT 'summary',  -- 'summary', 'detailed', 'appraisal', 'comparative'
    report_number VARCHAR(100),
    title VARCHAR(255),
    -- Content
    executive_summary TEXT,
    full_report_content JSONB,
    -- File attachment
    file_path TEXT,
    file_name VARCHAR(255),
    file_size_bytes BIGINT,
    file_mime_type VARCHAR(100),
    -- Status
    is_draft BOOLEAN NOT NULL DEFAULT true,
    is_signed BOOLEAN NOT NULL DEFAULT false,
    signed_by UUID REFERENCES users(id),
    signed_at TIMESTAMPTZ,
    -- Metadata
    generated_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_valuation_reports_org ON valuation_reports(organization_id);
CREATE INDEX idx_valuation_reports_valuation ON valuation_reports(valuation_id);
CREATE INDEX idx_valuation_reports_type ON valuation_reports(report_type);

-- ===========================================================================
-- Valuation Audit Log
-- ===========================================================================
CREATE TABLE valuation_audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    valuation_id UUID REFERENCES avm_property_valuations(id) ON DELETE SET NULL,
    model_id UUID REFERENCES property_valuation_models(id) ON DELETE SET NULL,
    -- Action info
    action VARCHAR(100) NOT NULL,
    entity_type VARCHAR(100) NOT NULL,
    entity_id UUID NOT NULL,
    -- Change details
    old_values JSONB,
    new_values JSONB,
    -- Context
    reason TEXT,
    ip_address VARCHAR(45),
    user_agent TEXT,
    -- Metadata
    performed_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_valuation_audit_org ON valuation_audit_logs(organization_id);
CREATE INDEX idx_valuation_audit_valuation ON valuation_audit_logs(valuation_id);
CREATE INDEX idx_valuation_audit_entity ON valuation_audit_logs(entity_type, entity_id);
CREATE INDEX idx_valuation_audit_date ON valuation_audit_logs(created_at DESC);

-- ===========================================================================
-- Row Level Security Policies
-- ===========================================================================
ALTER TABLE property_valuation_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE avm_property_valuations ENABLE ROW LEVEL SECURITY;
ALTER TABLE valuation_comparables ENABLE ROW LEVEL SECURITY;
ALTER TABLE comparable_adjustments ENABLE ROW LEVEL SECURITY;
ALTER TABLE valuation_market_data ENABLE ROW LEVEL SECURITY;
ALTER TABLE avm_property_value_history ENABLE ROW LEVEL SECURITY;
ALTER TABLE valuation_requests ENABLE ROW LEVEL SECURITY;
ALTER TABLE property_valuation_features ENABLE ROW LEVEL SECURITY;
ALTER TABLE valuation_reports ENABLE ROW LEVEL SECURITY;
ALTER TABLE valuation_audit_logs ENABLE ROW LEVEL SECURITY;

-- Valuation Models policies
CREATE POLICY valuation_models_tenant_isolation ON property_valuation_models
    FOR ALL USING (organization_id = current_setting('app.current_tenant')::uuid);

-- Valuations policies
CREATE POLICY valuations_tenant_isolation ON avm_property_valuations
    FOR ALL USING (organization_id = current_setting('app.current_tenant')::uuid);

-- Comparables policies
CREATE POLICY comparables_tenant_isolation ON valuation_comparables
    FOR ALL USING (organization_id = current_setting('app.current_tenant')::uuid);

-- Adjustments policies (through comparable)
CREATE POLICY adjustments_tenant_isolation ON comparable_adjustments
    FOR ALL USING (
        comparable_id IN (
            SELECT id FROM valuation_comparables
            WHERE organization_id = current_setting('app.current_tenant')::uuid
        )
    );

-- Market Data policies
CREATE POLICY market_data_tenant_isolation ON valuation_market_data
    FOR ALL USING (organization_id = current_setting('app.current_tenant')::uuid);

-- Value History policies
CREATE POLICY value_history_tenant_isolation ON avm_property_value_history
    FOR ALL USING (organization_id = current_setting('app.current_tenant')::uuid);

-- Valuation Requests policies
CREATE POLICY valuation_requests_tenant_isolation ON valuation_requests
    FOR ALL USING (organization_id = current_setting('app.current_tenant')::uuid);

-- Valuation Features policies
CREATE POLICY valuation_features_tenant_isolation ON property_valuation_features
    FOR ALL USING (organization_id = current_setting('app.current_tenant')::uuid);

-- Valuation Reports policies
CREATE POLICY valuation_reports_tenant_isolation ON valuation_reports
    FOR ALL USING (organization_id = current_setting('app.current_tenant')::uuid);

-- Audit Logs policies
CREATE POLICY valuation_audit_tenant_isolation ON valuation_audit_logs
    FOR ALL USING (organization_id = current_setting('app.current_tenant')::uuid);

-- ===========================================================================
-- Updated at triggers
-- ===========================================================================
CREATE TRIGGER update_valuation_models_updated_at
    BEFORE UPDATE ON property_valuation_models
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_valuations_updated_at
    BEFORE UPDATE ON avm_property_valuations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_comparables_updated_at
    BEFORE UPDATE ON valuation_comparables
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_market_data_updated_at
    BEFORE UPDATE ON valuation_market_data
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_valuation_requests_updated_at
    BEFORE UPDATE ON valuation_requests
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_valuation_features_updated_at
    BEFORE UPDATE ON property_valuation_features
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_valuation_reports_updated_at
    BEFORE UPDATE ON valuation_reports
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ===========================================================================
-- Comments
-- ===========================================================================
COMMENT ON TABLE property_valuation_models IS 'AVM model configurations and parameters';
COMMENT ON TABLE avm_property_valuations IS 'Property valuations with estimated values and confidence levels';
COMMENT ON TABLE valuation_comparables IS 'Comparable sales used in property valuations';
COMMENT ON TABLE comparable_adjustments IS 'Adjustments applied to comparable sales';
COMMENT ON TABLE valuation_market_data IS 'Regional and neighborhood market statistics';
COMMENT ON TABLE avm_property_value_history IS 'Historical property value tracking';
COMMENT ON TABLE valuation_requests IS 'User-initiated valuation requests';
COMMENT ON TABLE property_valuation_features IS 'Property features for hedonic model inputs';
COMMENT ON TABLE valuation_reports IS 'Generated valuation reports';
COMMENT ON TABLE valuation_audit_logs IS 'Audit trail for valuation activities';
