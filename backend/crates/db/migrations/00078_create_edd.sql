-- Epic 100, Story 100.4: Enhanced Due Diligence (EDD) Repository
-- AML/DSA Compliance for property management system
--
-- This migration creates tables for:
-- - AML risk assessments
-- - Enhanced Due Diligence (EDD) records
-- - EDD document management
-- - Country risk database
-- - DSA transparency reporting
-- - Content moderation cases

-- ============================================================================
-- ENUM TYPES
-- ============================================================================

-- AML Risk Level
CREATE TYPE aml_risk_level AS ENUM (
    'low',
    'medium',
    'high',
    'critical'
);

-- AML Assessment Status
CREATE TYPE aml_assessment_status AS ENUM (
    'pending',
    'in_progress',
    'completed',
    'requires_review',
    'approved',
    'rejected'
);

-- Country Risk Rating
CREATE TYPE country_risk_rating AS ENUM (
    'low',
    'medium',
    'high',
    'sanctioned'
);

-- EDD Status
CREATE TYPE edd_status AS ENUM (
    'not_required',
    'required',
    'in_progress',
    'pending_documents',
    'under_review',
    'completed',
    'expired'
);

-- Document Verification Status
CREATE TYPE document_verification_status AS ENUM (
    'pending',
    'verified',
    'rejected',
    'expired'
);

-- DSA Report Status
CREATE TYPE dsa_report_status AS ENUM (
    'draft',
    'generated',
    'published',
    'archived'
);

-- Moderation Status
CREATE TYPE moderation_status AS ENUM (
    'pending',
    'under_review',
    'approved',
    'removed',
    'restricted',
    'warned',
    'appealed',
    'appeal_approved',
    'appeal_rejected'
);

-- Moderated Content Type
CREATE TYPE moderated_content_type AS ENUM (
    'listing',
    'listing_photo',
    'user_profile',
    'review',
    'comment',
    'message',
    'announcement',
    'document',
    'community_post'
);

-- Violation Type
CREATE TYPE violation_type AS ENUM (
    'spam',
    'harassment',
    'hate_speech',
    'violence',
    'illegal_content',
    'misinformation',
    'fraud',
    'privacy',
    'intellectual_property',
    'inappropriate_content',
    'other'
);

-- Moderation Action Type
CREATE TYPE moderation_action_type AS ENUM (
    'remove',
    'restrict',
    'warn',
    'approve',
    'ignore',
    'escalate'
);

-- Report Source
CREATE TYPE report_source AS ENUM (
    'user',
    'automated',
    'staff',
    'external'
);

-- ============================================================================
-- COUNTRY RISK DATABASE
-- ============================================================================

CREATE TABLE country_risks (
    country_code VARCHAR(3) PRIMARY KEY,
    country_name VARCHAR(100) NOT NULL,
    risk_rating country_risk_rating NOT NULL DEFAULT 'low',
    is_sanctioned BOOLEAN NOT NULL DEFAULT FALSE,
    fatf_status VARCHAR(100),
    notes TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed with common EU countries and high-risk jurisdictions
INSERT INTO country_risks (country_code, country_name, risk_rating, is_sanctioned, fatf_status) VALUES
    ('SK', 'Slovakia', 'low', FALSE, NULL),
    ('CZ', 'Czech Republic', 'low', FALSE, NULL),
    ('AT', 'Austria', 'low', FALSE, NULL),
    ('DE', 'Germany', 'low', FALSE, NULL),
    ('PL', 'Poland', 'low', FALSE, NULL),
    ('HU', 'Hungary', 'low', FALSE, NULL),
    ('RU', 'Russia', 'high', TRUE, 'FATF Blacklist'),
    ('BY', 'Belarus', 'high', TRUE, 'FATF Blacklist'),
    ('IR', 'Iran', 'high', TRUE, 'FATF Blacklist'),
    ('KP', 'North Korea', 'high', TRUE, 'FATF Blacklist'),
    ('AE', 'United Arab Emirates', 'medium', FALSE, 'Monitoring'),
    ('PA', 'Panama', 'medium', FALSE, 'Grey List'),
    ('BS', 'Bahamas', 'medium', FALSE, 'Grey List');

-- ============================================================================
-- AML RISK ASSESSMENTS
-- ============================================================================

CREATE TABLE aml_risk_assessments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    transaction_id UUID,
    party_id UUID NOT NULL,
    party_type VARCHAR(50) NOT NULL,
    transaction_amount_cents BIGINT,
    currency VARCHAR(3),
    risk_score INTEGER NOT NULL DEFAULT 0 CHECK (risk_score >= 0 AND risk_score <= 100),
    risk_level aml_risk_level NOT NULL DEFAULT 'low',
    status aml_assessment_status NOT NULL DEFAULT 'pending',
    risk_factors JSONB,
    country_code VARCHAR(3) REFERENCES country_risks(country_code),
    country_risk country_risk_rating,
    id_verified BOOLEAN NOT NULL DEFAULT FALSE,
    source_of_funds_documented BOOLEAN NOT NULL DEFAULT FALSE,
    pep_check_completed BOOLEAN NOT NULL DEFAULT FALSE,
    is_pep BOOLEAN,
    sanctions_check_completed BOOLEAN NOT NULL DEFAULT FALSE,
    sanctions_match BOOLEAN,
    assessor_notes TEXT,
    assessed_by UUID REFERENCES users(id),
    assessed_at TIMESTAMPTZ,
    flagged_for_review BOOLEAN NOT NULL DEFAULT FALSE,
    review_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_aml_assessments_org ON aml_risk_assessments(organization_id);
CREATE INDEX idx_aml_assessments_party ON aml_risk_assessments(party_id);
CREATE INDEX idx_aml_assessments_status ON aml_risk_assessments(status);
CREATE INDEX idx_aml_assessments_flagged ON aml_risk_assessments(flagged_for_review) WHERE flagged_for_review = TRUE;
CREATE INDEX idx_aml_assessments_risk_level ON aml_risk_assessments(risk_level);
CREATE INDEX idx_aml_assessments_created ON aml_risk_assessments(created_at DESC);

-- ============================================================================
-- ENHANCED DUE DILIGENCE (EDD) RECORDS
-- ============================================================================

CREATE TABLE edd_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    aml_assessment_id UUID NOT NULL REFERENCES aml_risk_assessments(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    party_id UUID NOT NULL,
    status edd_status NOT NULL DEFAULT 'required',
    source_of_wealth TEXT,
    source_of_funds TEXT,
    beneficial_ownership JSONB,
    relationship_purpose TEXT,
    expected_transaction_patterns TEXT,
    documents_requested JSONB,
    compliance_notes JSONB DEFAULT '[]'::jsonb,
    initiated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    initiated_by UUID NOT NULL REFERENCES users(id),
    completed_at TIMESTAMPTZ,
    completed_by UUID REFERENCES users(id),
    next_review_date DATE,
    record_hash VARCHAR(64),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_edd_org ON edd_records(organization_id);
CREATE INDEX idx_edd_party ON edd_records(party_id);
CREATE INDEX idx_edd_aml ON edd_records(aml_assessment_id);
CREATE INDEX idx_edd_status ON edd_records(status);
CREATE INDEX idx_edd_review_date ON edd_records(next_review_date) WHERE next_review_date IS NOT NULL;
CREATE INDEX idx_edd_pending ON edd_records(status) WHERE status IN ('required', 'in_progress', 'pending_documents', 'under_review');

-- ============================================================================
-- EDD DOCUMENTS
-- ============================================================================

CREATE TABLE edd_documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    edd_id UUID NOT NULL REFERENCES edd_records(id) ON DELETE CASCADE,
    document_type VARCHAR(100) NOT NULL,
    file_path TEXT NOT NULL,
    original_filename VARCHAR(255) NOT NULL,
    file_size_bytes BIGINT NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    verification_status document_verification_status NOT NULL DEFAULT 'pending',
    verified_by UUID REFERENCES users(id),
    verified_at TIMESTAMPTZ,
    rejection_reason TEXT,
    expiry_date DATE,
    uploaded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    uploaded_by UUID NOT NULL REFERENCES users(id)
);

CREATE INDEX idx_edd_docs_edd ON edd_documents(edd_id);
CREATE INDEX idx_edd_docs_status ON edd_documents(verification_status);
CREATE INDEX idx_edd_docs_expiry ON edd_documents(expiry_date) WHERE expiry_date IS NOT NULL;

-- ============================================================================
-- DSA TRANSPARENCY REPORTS
-- ============================================================================

CREATE TABLE dsa_transparency_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    status dsa_report_status NOT NULL DEFAULT 'draft',
    total_moderation_actions BIGINT NOT NULL DEFAULT 0,
    content_removed_count BIGINT NOT NULL DEFAULT 0,
    content_restricted_count BIGINT NOT NULL DEFAULT 0,
    warnings_issued_count BIGINT NOT NULL DEFAULT 0,
    user_reports_received BIGINT NOT NULL DEFAULT 0,
    user_reports_resolved BIGINT NOT NULL DEFAULT 0,
    avg_resolution_time_hours FLOAT,
    automated_decisions_count BIGINT NOT NULL DEFAULT 0,
    automated_decisions_overturned BIGINT NOT NULL DEFAULT 0,
    appeals_received BIGINT NOT NULL DEFAULT 0,
    appeals_upheld BIGINT NOT NULL DEFAULT 0,
    appeals_rejected BIGINT NOT NULL DEFAULT 0,
    content_type_breakdown JSONB,
    violation_type_breakdown JSONB,
    report_file_path TEXT,
    generated_at TIMESTAMPTZ,
    generated_by UUID REFERENCES users(id),
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT valid_period CHECK (period_end > period_start)
);

CREATE INDEX idx_dsa_reports_status ON dsa_transparency_reports(status);
CREATE INDEX idx_dsa_reports_period ON dsa_transparency_reports(period_start, period_end);

-- ============================================================================
-- CONTENT MODERATION CASES
-- ============================================================================

CREATE TABLE moderation_cases (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content_type moderated_content_type NOT NULL,
    content_id UUID NOT NULL,
    content_preview TEXT,
    content_owner_id UUID NOT NULL REFERENCES users(id),
    organization_id UUID REFERENCES organizations(id),
    report_source report_source NOT NULL DEFAULT 'user',
    reported_by UUID REFERENCES users(id),
    automated_confidence INTEGER CHECK (automated_confidence >= 0 AND automated_confidence <= 100),
    violation_type violation_type,
    report_reason TEXT,
    status moderation_status NOT NULL DEFAULT 'pending',
    priority INTEGER NOT NULL DEFAULT 3 CHECK (priority >= 1 AND priority <= 5),
    assigned_to UUID REFERENCES users(id),
    assigned_at TIMESTAMPTZ,
    decision moderation_action_type,
    decision_rationale TEXT,
    action_template_id UUID,
    decided_by UUID REFERENCES users(id),
    decided_at TIMESTAMPTZ,
    appeal_filed BOOLEAN NOT NULL DEFAULT FALSE,
    appeal_reason TEXT,
    appeal_filed_at TIMESTAMPTZ,
    appeal_decision VARCHAR(50),
    appeal_decided_by UUID REFERENCES users(id),
    appeal_decided_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_mod_cases_status ON moderation_cases(status);
CREATE INDEX idx_mod_cases_content ON moderation_cases(content_type, content_id);
CREATE INDEX idx_mod_cases_owner ON moderation_cases(content_owner_id);
CREATE INDEX idx_mod_cases_org ON moderation_cases(organization_id);
CREATE INDEX idx_mod_cases_priority ON moderation_cases(priority, created_at);
CREATE INDEX idx_mod_cases_assigned ON moderation_cases(assigned_to) WHERE assigned_to IS NOT NULL;
CREATE INDEX idx_mod_cases_pending ON moderation_cases(status, priority, created_at) WHERE status IN ('pending', 'under_review');
CREATE INDEX idx_mod_cases_appeal ON moderation_cases(appeal_filed) WHERE appeal_filed = TRUE;

-- ============================================================================
-- MODERATION ACTION TEMPLATES
-- ============================================================================

CREATE TABLE moderation_action_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    violation_type violation_type NOT NULL,
    action_type moderation_action_type NOT NULL,
    rationale_template TEXT NOT NULL,
    notify_owner BOOLEAN NOT NULL DEFAULT TRUE,
    notification_template TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_mod_templates_violation ON moderation_action_templates(violation_type);
CREATE INDEX idx_mod_templates_active ON moderation_action_templates(is_active) WHERE is_active = TRUE;

-- Seed with common templates
INSERT INTO moderation_action_templates (name, violation_type, action_type, rationale_template, notify_owner) VALUES
    ('Remove Spam', 'spam', 'remove', 'Content removed as it violates our spam policy. Please review our terms of service.', TRUE),
    ('Warn for Inappropriate Language', 'inappropriate_content', 'warn', 'Warning issued for use of inappropriate language. Future violations may result in content removal.', TRUE),
    ('Remove Fraud Listing', 'fraud', 'remove', 'Listing removed due to suspected fraudulent activity. If you believe this is an error, please contact support.', TRUE),
    ('Remove Harassment', 'harassment', 'remove', 'Content removed for harassment. Such behavior is not tolerated on our platform.', TRUE),
    ('Restrict Privacy Violation', 'privacy', 'restrict', 'Content access restricted due to privacy concerns. Please ensure you have consent to share personal information.', TRUE);

-- ============================================================================
-- SUSPICIOUS ACTIVITY REPORTS (SARs)
-- ============================================================================

CREATE TABLE suspicious_activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    party_id UUID NOT NULL,
    activity_type VARCHAR(100) NOT NULL,
    description TEXT NOT NULL,
    detection_method VARCHAR(50) NOT NULL,
    risk_indicators JSONB,
    transaction_patterns JSONB,
    amount_cents BIGINT,
    currency VARCHAR(3),
    reported_to_authorities BOOLEAN NOT NULL DEFAULT FALSE,
    reported_at TIMESTAMPTZ,
    sar_reference VARCHAR(100),
    investigation_status VARCHAR(50) NOT NULL DEFAULT 'open',
    investigation_notes TEXT,
    investigated_by UUID REFERENCES users(id),
    detected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sar_org ON suspicious_activities(organization_id);
CREATE INDEX idx_sar_party ON suspicious_activities(party_id);
CREATE INDEX idx_sar_status ON suspicious_activities(investigation_status);
CREATE INDEX idx_sar_detected ON suspicious_activities(detected_at DESC);
CREATE INDEX idx_sar_reported ON suspicious_activities(reported_to_authorities) WHERE reported_to_authorities = TRUE;

-- ============================================================================
-- TRIGGERS FOR updated_at
-- ============================================================================

CREATE TRIGGER update_aml_risk_assessments_updated_at
    BEFORE UPDATE ON aml_risk_assessments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_edd_records_updated_at
    BEFORE UPDATE ON edd_records
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_dsa_transparency_reports_updated_at
    BEFORE UPDATE ON dsa_transparency_reports
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_moderation_cases_updated_at
    BEFORE UPDATE ON moderation_cases
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_suspicious_activities_updated_at
    BEFORE UPDATE ON suspicious_activities
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- COMMENTS FOR DOCUMENTATION
-- ============================================================================

COMMENT ON TABLE country_risks IS 'AML country risk database for compliance checks';
COMMENT ON TABLE aml_risk_assessments IS 'Anti-Money Laundering risk assessment records';
COMMENT ON TABLE edd_records IS 'Enhanced Due Diligence records for high-risk parties';
COMMENT ON TABLE edd_documents IS 'Supporting documents for EDD verification';
COMMENT ON TABLE dsa_transparency_reports IS 'Digital Services Act transparency reports';
COMMENT ON TABLE moderation_cases IS 'Content moderation case tracking';
COMMENT ON TABLE moderation_action_templates IS 'Pre-defined templates for moderation actions';
COMMENT ON TABLE suspicious_activities IS 'Suspicious Activity Reports (SARs) for AML compliance';
