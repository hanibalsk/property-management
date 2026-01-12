-- Epic 135: Enhanced Tenant Screening with AI Risk Scoring
-- Migration: 00088_create_enhanced_tenant_screening.sql

-- =============================================================================
-- ENUMs
-- =============================================================================

-- AI risk score category
CREATE TYPE ai_risk_category AS ENUM (
    'excellent',      -- 80-100: Very low risk
    'good',           -- 60-79: Low risk
    'fair',           -- 40-59: Moderate risk
    'poor',           -- 20-39: High risk
    'very_poor'       -- 0-19: Very high risk
);

-- Risk factor category (for Fair Housing compliance - no protected classes)
CREATE TYPE risk_factor_category AS ENUM (
    'credit_history',
    'rental_history',
    'income_stability',
    'employment_stability',
    'eviction_history',
    'criminal_background',
    'identity_verification',
    'reference_quality'
);

-- Risk factor impact
CREATE TYPE risk_factor_impact AS ENUM (
    'positive',
    'neutral',
    'negative',
    'critical'
);

-- Screening provider type
CREATE TYPE screening_provider_type AS ENUM (
    'credit_bureau',
    'background_check',
    'eviction_database',
    'identity_verification',
    'employment_verification',
    'rental_history'
);

-- Provider integration status
CREATE TYPE provider_integration_status AS ENUM (
    'active',
    'inactive',
    'error',
    'rate_limited',
    'maintenance'
);

-- =============================================================================
-- TABLES
-- =============================================================================

-- AI Risk Scoring Models Configuration
CREATE TABLE ai_risk_scoring_models (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Model configuration
    name VARCHAR(100) NOT NULL,
    description TEXT,
    version VARCHAR(20) NOT NULL DEFAULT '1.0',
    is_active BOOLEAN NOT NULL DEFAULT false,

    -- Factor weights (must sum to 1.0)
    credit_history_weight DECIMAL(3, 2) NOT NULL DEFAULT 0.25,
    rental_history_weight DECIMAL(3, 2) NOT NULL DEFAULT 0.20,
    income_stability_weight DECIMAL(3, 2) NOT NULL DEFAULT 0.20,
    employment_stability_weight DECIMAL(3, 2) NOT NULL DEFAULT 0.15,
    eviction_history_weight DECIMAL(3, 2) NOT NULL DEFAULT 0.10,
    criminal_background_weight DECIMAL(3, 2) NOT NULL DEFAULT 0.05,
    identity_verification_weight DECIMAL(3, 2) NOT NULL DEFAULT 0.03,
    reference_quality_weight DECIMAL(3, 2) NOT NULL DEFAULT 0.02,

    -- Thresholds for categories
    excellent_threshold INT NOT NULL DEFAULT 80,
    good_threshold INT NOT NULL DEFAULT 60,
    fair_threshold INT NOT NULL DEFAULT 40,
    poor_threshold INT NOT NULL DEFAULT 20,

    -- Auto-approval/rejection settings
    auto_approve_threshold INT, -- NULL means no auto-approval
    auto_reject_threshold INT,  -- NULL means no auto-rejection

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(id),

    CONSTRAINT valid_weights CHECK (
        credit_history_weight + rental_history_weight + income_stability_weight +
        employment_stability_weight + eviction_history_weight + criminal_background_weight +
        identity_verification_weight + reference_quality_weight = 1.0
    ),
    CONSTRAINT valid_thresholds CHECK (
        excellent_threshold > good_threshold AND
        good_threshold > fair_threshold AND
        fair_threshold > poor_threshold
    )
);

-- Partial unique index to ensure only one active model per organization
CREATE UNIQUE INDEX unique_active_model_per_org ON ai_risk_scoring_models(organization_id)
    WHERE is_active = true;

-- Screening Provider Configurations
CREATE TABLE screening_provider_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Provider details
    provider_name VARCHAR(100) NOT NULL,
    provider_type screening_provider_type NOT NULL,
    api_endpoint VARCHAR(500),

    -- Credentials (encrypted)
    api_key_encrypted BYTEA,
    api_secret_encrypted BYTEA,

    -- Status
    status provider_integration_status NOT NULL DEFAULT 'inactive',
    last_health_check TIMESTAMPTZ,
    error_message TEXT,

    -- Rate limiting
    rate_limit_per_hour INT DEFAULT 100,
    requests_this_hour INT DEFAULT 0,
    hour_reset_at TIMESTAMPTZ DEFAULT NOW(),

    -- Priority (for fallback ordering)
    priority INT NOT NULL DEFAULT 1,

    -- Costs
    cost_per_check DECIMAL(8, 2),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_provider_per_org UNIQUE (organization_id, provider_name)
);

-- Enhanced Screening Results with AI scoring
CREATE TABLE screening_ai_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    screening_id UUID NOT NULL REFERENCES tenant_screenings(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    model_id UUID NOT NULL REFERENCES ai_risk_scoring_models(id),

    -- AI Risk Score
    ai_risk_score INT NOT NULL CHECK (ai_risk_score >= 0 AND ai_risk_score <= 100),
    risk_category ai_risk_category NOT NULL,

    -- Component scores (0-100 each)
    credit_score_component INT CHECK (credit_score_component >= 0 AND credit_score_component <= 100),
    rental_history_component INT CHECK (rental_history_component >= 0 AND rental_history_component <= 100),
    income_stability_component INT CHECK (income_stability_component >= 0 AND income_stability_component <= 100),
    employment_stability_component INT CHECK (employment_stability_component >= 0 AND employment_stability_component <= 100),
    eviction_history_component INT CHECK (eviction_history_component >= 0 AND eviction_history_component <= 100),
    criminal_background_component INT CHECK (criminal_background_component >= 0 AND criminal_background_component <= 100),
    identity_verification_component INT CHECK (identity_verification_component >= 0 AND identity_verification_component <= 100),
    reference_quality_component INT CHECK (reference_quality_component >= 0 AND reference_quality_component <= 100),

    -- Recommendation
    recommendation VARCHAR(50) NOT NULL, -- 'approve', 'conditional_approve', 'review_required', 'reject'
    recommendation_reason TEXT,

    -- Comparison to typical tenants
    percentile_rank INT CHECK (percentile_rank >= 0 AND percentile_rank <= 100),
    typical_tenant_score INT,

    -- Confidence
    confidence_score DECIMAL(5, 2) CHECK (confidence_score >= 0 AND confidence_score <= 100),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Risk Factors (individual factors that contributed to the score)
CREATE TABLE screening_risk_factors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ai_result_id UUID NOT NULL REFERENCES screening_ai_results(id) ON DELETE CASCADE,

    -- Factor details
    category risk_factor_category NOT NULL,
    factor_name VARCHAR(100) NOT NULL,
    factor_description TEXT NOT NULL,

    -- Impact
    impact risk_factor_impact NOT NULL,
    score_impact INT NOT NULL, -- Points added/subtracted from base score

    -- Source data
    source_provider VARCHAR(100),
    source_data JSONB, -- Raw data that generated this factor (sanitized)

    -- Fair Housing compliance flag
    is_compliant BOOLEAN NOT NULL DEFAULT true,
    compliance_notes TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Credit Check Results (detailed storage)
CREATE TABLE screening_credit_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    screening_id UUID NOT NULL REFERENCES tenant_screenings(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Provider info
    provider_config_id UUID REFERENCES screening_provider_configs(id),
    provider_name VARCHAR(100) NOT NULL,

    -- Credit scores
    credit_score INT CHECK (credit_score >= 300 AND credit_score <= 850),
    score_model VARCHAR(50), -- 'FICO', 'VantageScore', etc.
    score_date DATE,

    -- Credit summary
    total_accounts INT,
    open_accounts INT,
    delinquent_accounts INT,
    collections_count INT,
    total_debt DECIMAL(12, 2),
    available_credit DECIMAL(12, 2),
    utilization_ratio DECIMAL(5, 2),

    -- Payment history
    on_time_payments_pct DECIMAL(5, 2),
    late_30_days_count INT,
    late_60_days_count INT,
    late_90_plus_count INT,

    -- Bankruptcies and public records
    bankruptcies_count INT DEFAULT 0,
    most_recent_bankruptcy_date DATE,
    public_records_count INT DEFAULT 0,

    -- Raw response (encrypted for compliance)
    raw_response_encrypted BYTEA,

    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ -- Data retention limit
);

-- Background Check Results
CREATE TABLE screening_background_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    screening_id UUID NOT NULL REFERENCES tenant_screenings(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Provider info
    provider_config_id UUID REFERENCES screening_provider_configs(id),
    provider_name VARCHAR(100) NOT NULL,

    -- Criminal background
    has_criminal_records BOOLEAN,
    felony_count INT DEFAULT 0,
    misdemeanor_count INT DEFAULT 0,
    most_recent_offense_date DATE,
    offense_categories JSONB, -- Array of offense types

    -- Sex offender registry
    sex_offender_check_passed BOOLEAN,

    -- Terrorist watchlist
    watchlist_check_passed BOOLEAN,

    -- Identity verification
    identity_verified BOOLEAN,
    identity_match_score DECIMAL(5, 2),
    ssn_verified BOOLEAN,
    address_history_verified BOOLEAN,

    -- Raw response (encrypted)
    raw_response_encrypted BYTEA,

    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ
);

-- Eviction History Results
CREATE TABLE screening_eviction_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    screening_id UUID NOT NULL REFERENCES tenant_screenings(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Provider info
    provider_config_id UUID REFERENCES screening_provider_configs(id),
    provider_name VARCHAR(100) NOT NULL,

    -- Eviction records
    has_eviction_records BOOLEAN,
    eviction_count INT DEFAULT 0,
    most_recent_eviction_date DATE,

    -- Eviction details (array of records)
    eviction_records JSONB, -- [{date, state, reason, judgment_amount, landlord}]

    -- Unlawful detainer filings
    unlawful_detainer_count INT DEFAULT 0,

    -- Raw response (encrypted)
    raw_response_encrypted BYTEA,

    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ
);

-- Screening Request Queue
CREATE TABLE screening_request_queue (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    screening_id UUID NOT NULL REFERENCES tenant_screenings(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Check type
    check_type screening_provider_type NOT NULL,
    provider_config_id UUID REFERENCES screening_provider_configs(id),

    -- Queue status
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, processing, completed, failed, retry
    priority INT NOT NULL DEFAULT 5, -- 1 = highest, 10 = lowest

    -- Retry handling
    attempt_count INT NOT NULL DEFAULT 0,
    max_attempts INT NOT NULL DEFAULT 3,
    next_retry_at TIMESTAMPTZ,
    last_error TEXT,

    -- Processing
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Screening Reports (generated PDF reports)
CREATE TABLE screening_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    screening_id UUID NOT NULL REFERENCES tenant_screenings(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Report details
    report_type VARCHAR(50) NOT NULL, -- 'full', 'summary', 'adverse_action'
    generated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    generated_by UUID NOT NULL REFERENCES users(id),

    -- Storage
    file_url VARCHAR(500),
    file_size_bytes BIGINT,

    -- Content hash for integrity
    content_hash VARCHAR(64),

    -- Expiry (GDPR compliance)
    expires_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ
);

-- =============================================================================
-- INDEXES
-- =============================================================================

-- AI Risk Scoring Models
CREATE INDEX idx_ai_risk_models_org ON ai_risk_scoring_models(organization_id);
CREATE INDEX idx_ai_risk_models_active ON ai_risk_scoring_models(organization_id, is_active) WHERE is_active = true;

-- Provider Configs
CREATE INDEX idx_provider_configs_org ON screening_provider_configs(organization_id);
CREATE INDEX idx_provider_configs_type ON screening_provider_configs(organization_id, provider_type);
CREATE INDEX idx_provider_configs_status ON screening_provider_configs(status) WHERE status = 'active';

-- AI Results
CREATE INDEX idx_ai_results_screening ON screening_ai_results(screening_id);
CREATE INDEX idx_ai_results_org ON screening_ai_results(organization_id);
CREATE INDEX idx_ai_results_category ON screening_ai_results(organization_id, risk_category);
CREATE INDEX idx_ai_results_score ON screening_ai_results(organization_id, ai_risk_score);

-- Risk Factors
CREATE INDEX idx_risk_factors_result ON screening_risk_factors(ai_result_id);
CREATE INDEX idx_risk_factors_category ON screening_risk_factors(category);
CREATE INDEX idx_risk_factors_impact ON screening_risk_factors(impact) WHERE impact IN ('negative', 'critical');

-- Credit Results
CREATE INDEX idx_credit_results_screening ON screening_credit_results(screening_id);
CREATE INDEX idx_credit_results_org ON screening_credit_results(organization_id);
CREATE INDEX idx_credit_results_expiry ON screening_credit_results(expires_at) WHERE expires_at IS NOT NULL;

-- Background Results
CREATE INDEX idx_background_results_screening ON screening_background_results(screening_id);
CREATE INDEX idx_background_results_org ON screening_background_results(organization_id);

-- Eviction Results
CREATE INDEX idx_eviction_results_screening ON screening_eviction_results(screening_id);
CREATE INDEX idx_eviction_results_org ON screening_eviction_results(organization_id);

-- Request Queue
CREATE INDEX idx_request_queue_status ON screening_request_queue(status, priority) WHERE status = 'pending';
CREATE INDEX idx_request_queue_retry ON screening_request_queue(next_retry_at) WHERE status = 'retry';
CREATE INDEX idx_request_queue_screening ON screening_request_queue(screening_id);

-- Reports
CREATE INDEX idx_reports_screening ON screening_reports(screening_id);
CREATE INDEX idx_reports_org ON screening_reports(organization_id);
CREATE INDEX idx_reports_expiry ON screening_reports(expires_at) WHERE expires_at IS NOT NULL AND deleted_at IS NULL;

-- =============================================================================
-- ROW LEVEL SECURITY
-- =============================================================================

ALTER TABLE ai_risk_scoring_models ENABLE ROW LEVEL SECURITY;
ALTER TABLE screening_provider_configs ENABLE ROW LEVEL SECURITY;
ALTER TABLE screening_ai_results ENABLE ROW LEVEL SECURITY;
ALTER TABLE screening_risk_factors ENABLE ROW LEVEL SECURITY;
ALTER TABLE screening_credit_results ENABLE ROW LEVEL SECURITY;
ALTER TABLE screening_background_results ENABLE ROW LEVEL SECURITY;
ALTER TABLE screening_eviction_results ENABLE ROW LEVEL SECURITY;
ALTER TABLE screening_request_queue ENABLE ROW LEVEL SECURITY;
ALTER TABLE screening_reports ENABLE ROW LEVEL SECURITY;

-- AI Risk Scoring Models policies
CREATE POLICY ai_risk_models_select ON ai_risk_scoring_models
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY ai_risk_models_insert ON ai_risk_scoring_models
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY ai_risk_models_update ON ai_risk_scoring_models
    FOR UPDATE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY ai_risk_models_delete ON ai_risk_scoring_models
    FOR DELETE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- Provider Configs policies
CREATE POLICY provider_configs_select ON screening_provider_configs
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY provider_configs_insert ON screening_provider_configs
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY provider_configs_update ON screening_provider_configs
    FOR UPDATE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY provider_configs_delete ON screening_provider_configs
    FOR DELETE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- AI Results policies
CREATE POLICY ai_results_select ON screening_ai_results
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY ai_results_insert ON screening_ai_results
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- Risk Factors policies (via join to ai_results)
CREATE POLICY risk_factors_select ON screening_risk_factors
    FOR SELECT USING (
        EXISTS (
            SELECT 1 FROM screening_ai_results
            WHERE screening_ai_results.id = screening_risk_factors.ai_result_id
            AND screening_ai_results.organization_id = current_setting('app.current_organization_id', true)::uuid
        )
    );

CREATE POLICY risk_factors_insert ON screening_risk_factors
    FOR INSERT WITH CHECK (
        EXISTS (
            SELECT 1 FROM screening_ai_results
            WHERE screening_ai_results.id = screening_risk_factors.ai_result_id
            AND screening_ai_results.organization_id = current_setting('app.current_organization_id', true)::uuid
        )
    );

-- Credit Results policies
CREATE POLICY credit_results_select ON screening_credit_results
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY credit_results_insert ON screening_credit_results
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- Background Results policies
CREATE POLICY background_results_select ON screening_background_results
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY background_results_insert ON screening_background_results
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- Eviction Results policies
CREATE POLICY eviction_results_select ON screening_eviction_results
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY eviction_results_insert ON screening_eviction_results
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- Request Queue policies
CREATE POLICY request_queue_select ON screening_request_queue
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY request_queue_insert ON screening_request_queue
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY request_queue_update ON screening_request_queue
    FOR UPDATE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- Reports policies
CREATE POLICY reports_select ON screening_reports
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY reports_insert ON screening_reports
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY reports_update ON screening_reports
    FOR UPDATE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- =============================================================================
-- TRIGGERS
-- =============================================================================

-- Auto-update timestamps
CREATE TRIGGER update_ai_risk_models_timestamp
    BEFORE UPDATE ON ai_risk_scoring_models
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_provider_configs_timestamp
    BEFORE UPDATE ON screening_provider_configs
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_request_queue_timestamp
    BEFORE UPDATE ON screening_request_queue
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- =============================================================================
-- SEED DATA: Default AI Risk Scoring Model
-- =============================================================================

-- Note: This will be created per organization when they enable AI screening
-- No global seed data needed

-- =============================================================================
-- COMMENTS
-- =============================================================================

COMMENT ON TABLE ai_risk_scoring_models IS 'AI risk scoring model configurations per organization';
COMMENT ON TABLE screening_provider_configs IS 'Third-party screening provider integration configs';
COMMENT ON TABLE screening_ai_results IS 'AI-generated risk scores and recommendations';
COMMENT ON TABLE screening_risk_factors IS 'Individual risk factors contributing to AI score';
COMMENT ON TABLE screening_credit_results IS 'Credit bureau check results';
COMMENT ON TABLE screening_background_results IS 'Criminal and background check results';
COMMENT ON TABLE screening_eviction_results IS 'Eviction history check results';
COMMENT ON TABLE screening_request_queue IS 'Queue for processing screening requests';
COMMENT ON TABLE screening_reports IS 'Generated PDF screening reports';
