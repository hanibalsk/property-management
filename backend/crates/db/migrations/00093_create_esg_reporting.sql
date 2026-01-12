-- Epic 136: ESG Reporting Dashboard
-- Migration: 00088_create_esg_reporting.sql

-- =============================================================================
-- ENUMs
-- =============================================================================

-- ESG metric category
CREATE TYPE esg_metric_category AS ENUM (
    'environmental',
    'social',
    'governance'
);

-- Environmental metric type
CREATE TYPE environmental_metric_type AS ENUM (
    'energy_consumption',
    'water_usage',
    'waste_management',
    'carbon_emissions',
    'renewable_energy',
    'air_quality',
    'biodiversity'
);

-- Social metric type
CREATE TYPE social_metric_type AS ENUM (
    'tenant_satisfaction',
    'employee_safety',
    'community_engagement',
    'accessibility',
    'health_wellbeing',
    'diversity_inclusion'
);

-- Governance metric type
CREATE TYPE governance_metric_type AS ENUM (
    'data_privacy',
    'compliance_training',
    'ethics_policies',
    'stakeholder_engagement',
    'risk_management',
    'supply_chain'
);

-- Emission scope type (renamed to avoid conflict with energy module)
CREATE TYPE esg_emission_scope AS ENUM (
    'scope_1_direct',      -- Direct emissions from owned sources
    'scope_2_indirect',    -- Indirect emissions from purchased energy
    'scope_3_value_chain'  -- Other indirect emissions
);

-- Energy source type
CREATE TYPE energy_source_type AS ENUM (
    'electricity_grid',
    'natural_gas',
    'heating_oil',
    'district_heating',
    'solar_pv',
    'wind',
    'geothermal',
    'biomass'
);

-- Data entry method
CREATE TYPE esg_data_entry_method AS ENUM (
    'manual',
    'csv_import',
    'api_integration',
    'smart_meter',
    'calculated'
);

-- Compliance framework
CREATE TYPE esg_compliance_framework AS ENUM (
    'eu_taxonomy',
    'sfdr',
    'csrd',
    'gresb',
    'leed',
    'breeam',
    'iso_14001',
    'ghg_protocol'
);

-- Benchmark category
CREATE TYPE esg_benchmark_category AS ENUM (
    'industry_average',
    'regional_average',
    'best_in_class',
    'regulatory_minimum',
    'internal_target'
);

-- Report status
CREATE TYPE esg_report_status AS ENUM (
    'draft',
    'pending_review',
    'approved',
    'published',
    'archived'
);

-- =============================================================================
-- TABLES
-- =============================================================================

-- ESG Configuration per Organization
CREATE TABLE esg_configurations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Reporting settings
    reporting_currency VARCHAR(3) NOT NULL DEFAULT 'EUR',
    default_unit_system VARCHAR(20) NOT NULL DEFAULT 'metric', -- metric, imperial
    fiscal_year_start_month INT NOT NULL DEFAULT 1,

    -- Framework compliance (stored as JSONB for Rust compatibility)
    enabled_frameworks JSONB DEFAULT '[]',

    -- Carbon calculation settings
    grid_emission_factor DECIMAL(10, 4) DEFAULT 0.4, -- kg CO2 per kWh
    natural_gas_emission_factor DECIMAL(10, 4) DEFAULT 2.0, -- kg CO2 per m³

    -- Targets
    carbon_reduction_target_pct DECIMAL(5, 2), -- % reduction target
    target_year INT,
    baseline_year INT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_org_esg_config UNIQUE (organization_id)
);

-- ESG Metrics Data
CREATE TABLE esg_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,

    -- Time period
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,

    -- Metric identification
    category esg_metric_category NOT NULL,
    metric_type VARCHAR(50) NOT NULL, -- Flexible for different sub-types
    metric_name VARCHAR(100) NOT NULL,

    -- Value
    value DECIMAL(18, 4) NOT NULL,
    unit VARCHAR(50) NOT NULL,
    normalized_value DECIMAL(18, 4), -- Per m² or per unit

    -- Data quality
    data_source esg_data_entry_method NOT NULL,
    confidence_level INT CHECK (confidence_level >= 1 AND confidence_level <= 5),
    verification_status VARCHAR(50) DEFAULT 'unverified',
    verified_by UUID REFERENCES users(id),
    verified_at TIMESTAMPTZ,

    -- Notes
    notes TEXT,
    supporting_documents JSONB DEFAULT '[]',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(id)
);

-- Carbon Footprint Records
CREATE TABLE carbon_footprints (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,

    -- Time period
    year INT NOT NULL,
    month INT, -- NULL for annual records

    -- Emission source
    source_type esg_emission_scope NOT NULL,
    energy_source energy_source_type,

    -- Consumption data
    consumption_value DECIMAL(18, 4) NOT NULL,
    consumption_unit VARCHAR(20) NOT NULL,

    -- Calculated emissions
    emission_factor DECIMAL(10, 6) NOT NULL, -- kg CO2e per unit
    co2_equivalent_kg DECIMAL(18, 4) NOT NULL,

    -- Intensity metrics
    area_sqm DECIMAL(12, 2),
    co2_per_sqm DECIMAL(12, 4),
    num_units INT,
    co2_per_unit DECIMAL(12, 4),

    -- Methodology
    calculation_methodology VARCHAR(50) DEFAULT 'ghg_protocol',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ESG Benchmarks
CREATE TABLE esg_benchmarks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Benchmark definition
    name VARCHAR(100) NOT NULL,
    category esg_benchmark_category NOT NULL,
    metric_type VARCHAR(50) NOT NULL,

    -- Values
    benchmark_value DECIMAL(18, 4) NOT NULL,
    unit VARCHAR(50) NOT NULL,

    -- Context
    region VARCHAR(50), -- EU, DACH, SK, CZ
    property_type VARCHAR(50), -- residential, commercial, mixed
    source VARCHAR(200),
    effective_date DATE NOT NULL,
    expiry_date DATE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ESG Targets
CREATE TABLE esg_targets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,

    -- Target definition
    name VARCHAR(100) NOT NULL,
    category esg_metric_category NOT NULL,
    metric_type VARCHAR(50) NOT NULL,

    -- Target values
    target_value DECIMAL(18, 4) NOT NULL,
    unit VARCHAR(50) NOT NULL,
    target_date DATE NOT NULL,

    -- Baseline
    baseline_value DECIMAL(18, 4),
    baseline_date DATE,

    -- Status
    current_value DECIMAL(18, 4),
    progress_pct DECIMAL(5, 2),
    status VARCHAR(50) DEFAULT 'on_track', -- on_track, at_risk, behind, achieved

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ESG Compliance Reports
CREATE TABLE esg_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Report details
    report_type VARCHAR(50) NOT NULL, -- annual, quarterly, custom
    title VARCHAR(200) NOT NULL,
    description TEXT,

    -- Period
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,

    -- Compliance frameworks (stored as JSONB for Rust compatibility)
    frameworks JSONB DEFAULT '[]',

    -- Status
    status esg_report_status NOT NULL DEFAULT 'draft',
    submitted_at TIMESTAMPTZ,
    approved_by UUID REFERENCES users(id),
    approved_at TIMESTAMPTZ,

    -- Content
    report_data JSONB, -- Structured report content
    summary_scores JSONB, -- E/S/G scores

    -- Files
    pdf_url VARCHAR(500),
    xml_url VARCHAR(500),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(id)
);

-- EU Taxonomy Alignment Assessment
CREATE TABLE eu_taxonomy_assessments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,

    -- Assessment period
    year INT NOT NULL,

    -- Climate mitigation
    climate_mitigation_eligible BOOLEAN DEFAULT false,
    climate_mitigation_aligned BOOLEAN DEFAULT false,
    climate_mitigation_revenue_pct DECIMAL(5, 2),

    -- Climate adaptation
    climate_adaptation_eligible BOOLEAN DEFAULT false,
    climate_adaptation_aligned BOOLEAN DEFAULT false,
    climate_adaptation_revenue_pct DECIMAL(5, 2),

    -- Technical screening criteria
    energy_performance_class VARCHAR(5), -- A, B, C, etc.
    primary_energy_demand DECIMAL(12, 4), -- kWh/m²/year
    meets_nzeb_standard BOOLEAN DEFAULT false,

    -- DNSH criteria (Do No Significant Harm)
    dnsh_water BOOLEAN DEFAULT true,
    dnsh_circular_economy BOOLEAN DEFAULT true,
    dnsh_pollution BOOLEAN DEFAULT true,
    dnsh_biodiversity BOOLEAN DEFAULT true,

    -- Social safeguards
    oecd_guidelines_compliance BOOLEAN DEFAULT true,
    un_guiding_principles BOOLEAN DEFAULT true,

    -- Overall alignment
    overall_alignment_pct DECIMAL(5, 2),

    notes TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ESG Dashboard Metrics (Cached aggregations)
CREATE TABLE esg_dashboard_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,

    -- Time period
    year INT NOT NULL,
    month INT, -- NULL for annual

    -- Summary scores (0-100)
    environmental_score DECIMAL(5, 2),
    social_score DECIMAL(5, 2),
    governance_score DECIMAL(5, 2),
    overall_esg_score DECIMAL(5, 2),

    -- Key environmental metrics
    total_co2_kg DECIMAL(18, 4),
    co2_per_sqm DECIMAL(12, 4),
    energy_intensity DECIMAL(12, 4), -- kWh/m²
    water_intensity DECIMAL(12, 4), -- m³/m²
    waste_diversion_rate DECIMAL(5, 2), -- % diverted from landfill
    renewable_energy_pct DECIMAL(5, 2),

    -- Comparisons
    yoy_co2_change_pct DECIMAL(8, 2),
    benchmark_comparison JSONB, -- Comparison to various benchmarks

    -- Compliance status
    compliance_alerts JSONB, -- Array of compliance issues

    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_dashboard_metric UNIQUE (organization_id, building_id, year, month)
);

-- ESG Data Import Jobs
CREATE TABLE esg_import_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Job details
    file_name VARCHAR(255) NOT NULL,
    file_url VARCHAR(500),
    data_type VARCHAR(50) NOT NULL, -- energy, water, waste, emissions

    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, processing, completed, failed
    rows_total INT,
    rows_processed INT DEFAULT 0,
    rows_failed INT DEFAULT 0,
    error_log JSONB DEFAULT '[]',

    -- Processing
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(id)
);

-- =============================================================================
-- INDEXES
-- =============================================================================

-- ESG Configurations
CREATE INDEX idx_esg_configs_org ON esg_configurations(organization_id);

-- ESG Metrics
CREATE INDEX idx_esg_metrics_org ON esg_metrics(organization_id);
CREATE INDEX idx_esg_metrics_building ON esg_metrics(building_id);
CREATE INDEX idx_esg_metrics_period ON esg_metrics(organization_id, period_start, period_end);
CREATE INDEX idx_esg_metrics_category ON esg_metrics(organization_id, category);
CREATE INDEX idx_esg_metrics_type ON esg_metrics(organization_id, metric_type);

-- Carbon Footprints
CREATE INDEX idx_carbon_footprints_org ON carbon_footprints(organization_id);
CREATE INDEX idx_carbon_footprints_building ON carbon_footprints(building_id);
CREATE INDEX idx_carbon_footprints_year ON carbon_footprints(organization_id, year);
CREATE INDEX idx_carbon_footprints_source ON carbon_footprints(source_type);

-- ESG Benchmarks
CREATE INDEX idx_esg_benchmarks_org ON esg_benchmarks(organization_id);
CREATE INDEX idx_esg_benchmarks_metric ON esg_benchmarks(metric_type);
CREATE INDEX idx_esg_benchmarks_category ON esg_benchmarks(category);

-- ESG Targets
CREATE INDEX idx_esg_targets_org ON esg_targets(organization_id);
CREATE INDEX idx_esg_targets_building ON esg_targets(building_id);
CREATE INDEX idx_esg_targets_date ON esg_targets(target_date);

-- ESG Reports
CREATE INDEX idx_esg_reports_org ON esg_reports(organization_id);
CREATE INDEX idx_esg_reports_status ON esg_reports(status);
CREATE INDEX idx_esg_reports_period ON esg_reports(period_start, period_end);

-- EU Taxonomy
CREATE INDEX idx_eu_taxonomy_org ON eu_taxonomy_assessments(organization_id);
CREATE INDEX idx_eu_taxonomy_building ON eu_taxonomy_assessments(building_id);
CREATE INDEX idx_eu_taxonomy_year ON eu_taxonomy_assessments(year);

-- Dashboard Metrics
CREATE INDEX idx_esg_dashboard_org ON esg_dashboard_metrics(organization_id);
CREATE INDEX idx_esg_dashboard_building ON esg_dashboard_metrics(building_id);
CREATE INDEX idx_esg_dashboard_period ON esg_dashboard_metrics(year, month);

-- Import Jobs
CREATE INDEX idx_esg_import_org ON esg_import_jobs(organization_id);
CREATE INDEX idx_esg_import_status ON esg_import_jobs(status);

-- =============================================================================
-- ROW LEVEL SECURITY
-- =============================================================================

ALTER TABLE esg_configurations ENABLE ROW LEVEL SECURITY;
ALTER TABLE esg_metrics ENABLE ROW LEVEL SECURITY;
ALTER TABLE carbon_footprints ENABLE ROW LEVEL SECURITY;
ALTER TABLE esg_benchmarks ENABLE ROW LEVEL SECURITY;
ALTER TABLE esg_targets ENABLE ROW LEVEL SECURITY;
ALTER TABLE esg_reports ENABLE ROW LEVEL SECURITY;
ALTER TABLE eu_taxonomy_assessments ENABLE ROW LEVEL SECURITY;
ALTER TABLE esg_dashboard_metrics ENABLE ROW LEVEL SECURITY;
ALTER TABLE esg_import_jobs ENABLE ROW LEVEL SECURITY;

-- ESG Configurations policies
CREATE POLICY esg_configs_select ON esg_configurations
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY esg_configs_insert ON esg_configurations
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY esg_configs_update ON esg_configurations
    FOR UPDATE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- ESG Metrics policies
CREATE POLICY esg_metrics_select ON esg_metrics
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY esg_metrics_insert ON esg_metrics
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY esg_metrics_update ON esg_metrics
    FOR UPDATE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY esg_metrics_delete ON esg_metrics
    FOR DELETE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- Carbon Footprints policies
CREATE POLICY carbon_footprints_select ON carbon_footprints
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY carbon_footprints_insert ON carbon_footprints
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY carbon_footprints_update ON carbon_footprints
    FOR UPDATE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- ESG Benchmarks policies
CREATE POLICY esg_benchmarks_select ON esg_benchmarks
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY esg_benchmarks_insert ON esg_benchmarks
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY esg_benchmarks_update ON esg_benchmarks
    FOR UPDATE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- ESG Targets policies
CREATE POLICY esg_targets_select ON esg_targets
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY esg_targets_insert ON esg_targets
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY esg_targets_update ON esg_targets
    FOR UPDATE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY esg_targets_delete ON esg_targets
    FOR DELETE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- ESG Reports policies
CREATE POLICY esg_reports_select ON esg_reports
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY esg_reports_insert ON esg_reports
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY esg_reports_update ON esg_reports
    FOR UPDATE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- EU Taxonomy policies
CREATE POLICY eu_taxonomy_select ON eu_taxonomy_assessments
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY eu_taxonomy_insert ON eu_taxonomy_assessments
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY eu_taxonomy_update ON eu_taxonomy_assessments
    FOR UPDATE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- Dashboard Metrics policies
CREATE POLICY esg_dashboard_select ON esg_dashboard_metrics
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY esg_dashboard_insert ON esg_dashboard_metrics
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY esg_dashboard_update ON esg_dashboard_metrics
    FOR UPDATE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- Import Jobs policies
CREATE POLICY esg_import_select ON esg_import_jobs
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY esg_import_insert ON esg_import_jobs
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY esg_import_update ON esg_import_jobs
    FOR UPDATE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- =============================================================================
-- TRIGGERS
-- =============================================================================

CREATE TRIGGER update_esg_configs_timestamp
    BEFORE UPDATE ON esg_configurations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_esg_metrics_timestamp
    BEFORE UPDATE ON esg_metrics
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_carbon_footprints_timestamp
    BEFORE UPDATE ON carbon_footprints
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_esg_targets_timestamp
    BEFORE UPDATE ON esg_targets
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_esg_reports_timestamp
    BEFORE UPDATE ON esg_reports
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_eu_taxonomy_timestamp
    BEFORE UPDATE ON eu_taxonomy_assessments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- =============================================================================
-- SEED DATA: Default Benchmarks
-- =============================================================================

-- Note: Benchmarks will be populated per organization when they enable ESG reporting
-- Global industry benchmarks would be inserted by admin

-- =============================================================================
-- COMMENTS
-- =============================================================================

COMMENT ON TABLE esg_configurations IS 'Organization-level ESG reporting configuration';
COMMENT ON TABLE esg_metrics IS 'ESG metric data points for buildings';
COMMENT ON TABLE carbon_footprints IS 'Carbon emission calculations following GHG Protocol';
COMMENT ON TABLE esg_benchmarks IS 'Industry and regulatory benchmark values';
COMMENT ON TABLE esg_targets IS 'ESG reduction and improvement targets';
COMMENT ON TABLE esg_reports IS 'Generated ESG compliance reports';
COMMENT ON TABLE eu_taxonomy_assessments IS 'EU Taxonomy alignment assessments';
COMMENT ON TABLE esg_dashboard_metrics IS 'Cached ESG dashboard aggregations';
COMMENT ON TABLE esg_import_jobs IS 'Bulk ESG data import tracking';
