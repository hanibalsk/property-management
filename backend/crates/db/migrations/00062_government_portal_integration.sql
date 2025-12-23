-- Migration: 00062_government_portal_integration
-- Epic 30: Government Portal Integration
-- UC-22.3: Government Portal Integration

-- ============================================================================
-- GOVERNMENT PORTAL TYPES
-- ============================================================================

-- Type of government portal
CREATE TYPE government_portal_type AS ENUM (
    'tax_authority',           -- Tax submission (e.g., VAT, income tax)
    'statistical_office',       -- Statistical reporting
    'building_authority',       -- Building permits and inspections
    'housing_registry',         -- Housing/residence registry
    'police_registry',          -- Guest/foreigner registration
    'energy_authority',         -- Energy efficiency reporting
    'environmental_agency',     -- Environmental compliance
    'labor_office',             -- Employment/contractor reporting
    'social_insurance',         -- Social security contributions
    'business_registry',        -- Company registry updates
    'data_protection',          -- GDPR/data protection authority
    'other'
);

-- Status of a submission
CREATE TYPE submission_status AS ENUM (
    'draft',
    'pending_validation',
    'validated',
    'submitted',
    'acknowledged',
    'processing',
    'accepted',
    'rejected',
    'requires_correction',
    'cancelled'
);

-- ============================================================================
-- GOVERNMENT PORTAL CONNECTIONS
-- ============================================================================

-- Configured government portal connections
CREATE TABLE government_portal_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Portal info
    portal_type government_portal_type NOT NULL,
    portal_name TEXT NOT NULL,
    portal_code TEXT,                        -- Official portal identifier
    country_code TEXT NOT NULL DEFAULT 'SK', -- ISO 3166-1 alpha-2

    -- Connection details (encrypted credentials stored separately)
    api_endpoint TEXT,
    portal_username TEXT,

    -- OAuth/API credentials
    oauth_client_id TEXT,
    api_key_id UUID,                         -- Reference to encrypted API key
    certificate_id UUID,                      -- Reference to certificate for signing

    -- Settings
    is_active BOOLEAN NOT NULL DEFAULT true,
    auto_submit BOOLEAN NOT NULL DEFAULT false,
    test_mode BOOLEAN NOT NULL DEFAULT true,

    -- Metadata
    last_connection_test TIMESTAMPTZ,
    last_successful_submission TIMESTAMPTZ,

    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT uq_gov_portal_org_type UNIQUE (organization_id, portal_type, country_code)
);

CREATE INDEX idx_gov_portal_org ON government_portal_connections(organization_id);
CREATE INDEX idx_gov_portal_type ON government_portal_connections(portal_type);
CREATE INDEX idx_gov_portal_active ON government_portal_connections(organization_id, is_active) WHERE is_active = true;

-- Trigger for updated_at
CREATE TRIGGER government_portal_connections_updated_at
    BEFORE UPDATE ON government_portal_connections
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- REGULATORY REPORT TEMPLATES
-- ============================================================================

-- Templates for different types of regulatory reports
CREATE TABLE regulatory_report_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Template identification
    template_code TEXT NOT NULL UNIQUE,      -- e.g., 'SK_VAT_MONTHLY', 'CZ_STAT_QUARTERLY'
    template_name TEXT NOT NULL,
    description TEXT,

    -- Association
    portal_type government_portal_type NOT NULL,
    country_code TEXT NOT NULL DEFAULT 'SK',

    -- Template specification
    schema_version TEXT NOT NULL,            -- Version of the report schema
    field_mappings JSONB NOT NULL DEFAULT '{}',  -- Map system fields to report fields
    validation_rules JSONB NOT NULL DEFAULT '[]', -- Validation rules for the report
    xml_template TEXT,                       -- XML template if applicable

    -- Scheduling
    frequency TEXT CHECK (frequency IN ('monthly', 'quarterly', 'yearly', 'on_demand', 'event_driven')),
    due_day_of_month INTEGER CHECK (due_day_of_month BETWEEN 1 AND 31),
    due_month_of_quarter INTEGER CHECK (due_month_of_quarter BETWEEN 1 AND 3),

    -- Metadata
    is_active BOOLEAN NOT NULL DEFAULT true,
    effective_from DATE NOT NULL DEFAULT CURRENT_DATE,
    effective_to DATE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_reg_report_template_portal ON regulatory_report_templates(portal_type, country_code);
CREATE INDEX idx_reg_report_template_active ON regulatory_report_templates(is_active) WHERE is_active = true;

-- Trigger for updated_at
CREATE TRIGGER regulatory_report_templates_updated_at
    BEFORE UPDATE ON regulatory_report_templates
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- REGULATORY SUBMISSIONS
-- ============================================================================

-- Actual regulatory report submissions
CREATE TABLE regulatory_submissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    portal_connection_id UUID REFERENCES government_portal_connections(id) ON DELETE SET NULL,
    template_id UUID REFERENCES regulatory_report_templates(id) ON DELETE SET NULL,

    -- Report identification
    submission_reference TEXT NOT NULL,      -- System-generated reference
    external_reference TEXT,                 -- Portal-assigned reference

    -- Report details
    report_type TEXT NOT NULL,
    report_period_start DATE NOT NULL,
    report_period_end DATE NOT NULL,

    -- Content
    report_data JSONB NOT NULL,              -- The actual report data
    report_xml TEXT,                         -- Generated XML if applicable
    report_pdf_url TEXT,                     -- URL to PDF version

    -- Status tracking
    status submission_status NOT NULL DEFAULT 'draft',
    validation_result JSONB,                 -- Validation errors/warnings
    submission_response JSONB,               -- Response from portal

    -- Timestamps
    validated_at TIMESTAMPTZ,
    submitted_at TIMESTAMPTZ,
    acknowledged_at TIMESTAMPTZ,
    processed_at TIMESTAMPTZ,

    -- User tracking
    prepared_by UUID REFERENCES users(id) ON DELETE SET NULL,
    submitted_by UUID REFERENCES users(id) ON DELETE SET NULL,

    -- Retry handling
    submission_attempts INTEGER NOT NULL DEFAULT 0,
    last_error TEXT,
    next_retry_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_reg_submission_org ON regulatory_submissions(organization_id);
CREATE INDEX idx_reg_submission_status ON regulatory_submissions(status);
CREATE INDEX idx_reg_submission_period ON regulatory_submissions(report_period_start, report_period_end);
CREATE INDEX idx_reg_submission_portal ON regulatory_submissions(portal_connection_id);
CREATE INDEX idx_reg_submission_pending ON regulatory_submissions(status, next_retry_at)
    WHERE status IN ('pending_validation', 'submitted', 'processing');

-- Trigger for updated_at
CREATE TRIGGER regulatory_submissions_updated_at
    BEFORE UPDATE ON regulatory_submissions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- SUBMISSION AUDIT LOG
-- ============================================================================

-- Detailed audit trail for submissions
CREATE TABLE regulatory_submission_audit (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    submission_id UUID NOT NULL REFERENCES regulatory_submissions(id) ON DELETE CASCADE,

    -- Action details
    action TEXT NOT NULL,                    -- created, validated, submitted, response_received, etc.
    previous_status submission_status,
    new_status submission_status,

    -- Context
    actor_id UUID REFERENCES users(id) ON DELETE SET NULL,
    actor_type TEXT NOT NULL DEFAULT 'user', -- user, system, portal

    -- Details
    details JSONB,
    error_message TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_reg_submission_audit_submission ON regulatory_submission_audit(submission_id);
CREATE INDEX idx_reg_submission_audit_actor ON regulatory_submission_audit(actor_id);
CREATE INDEX idx_reg_submission_audit_action ON regulatory_submission_audit(action);

-- ============================================================================
-- SUBMISSION ATTACHMENTS
-- ============================================================================

-- Documents attached to submissions
CREATE TABLE regulatory_submission_attachments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    submission_id UUID NOT NULL REFERENCES regulatory_submissions(id) ON DELETE CASCADE,

    -- Attachment details
    file_name TEXT NOT NULL,
    file_type TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    file_url TEXT NOT NULL,
    checksum TEXT,

    -- Purpose
    attachment_type TEXT NOT NULL,           -- supporting_document, signature, receipt, etc.
    description TEXT,

    -- From portal
    portal_document_id TEXT,                 -- ID assigned by portal

    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_reg_attachment_submission ON regulatory_submission_attachments(submission_id);

-- ============================================================================
-- SCHEDULED SUBMISSIONS
-- ============================================================================

-- Scheduled/recurring submissions
CREATE TABLE regulatory_submission_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    portal_connection_id UUID NOT NULL REFERENCES government_portal_connections(id) ON DELETE CASCADE,
    template_id UUID NOT NULL REFERENCES regulatory_report_templates(id) ON DELETE CASCADE,

    -- Schedule details
    is_active BOOLEAN NOT NULL DEFAULT true,
    next_due_date DATE,
    last_generated_at TIMESTAMPTZ,
    last_submission_id UUID REFERENCES regulatory_submissions(id),

    -- Settings
    auto_generate BOOLEAN NOT NULL DEFAULT true,
    auto_submit BOOLEAN NOT NULL DEFAULT false,
    notify_before_days INTEGER NOT NULL DEFAULT 7,

    -- Notifications
    notify_users JSONB DEFAULT '[]',         -- List of user IDs to notify

    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_reg_schedule_org ON regulatory_submission_schedules(organization_id);
CREATE INDEX idx_reg_schedule_due ON regulatory_submission_schedules(next_due_date) WHERE is_active = true;

-- Trigger for updated_at
CREATE TRIGGER regulatory_submission_schedules_updated_at
    BEFORE UPDATE ON regulatory_submission_schedules
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- ROW LEVEL SECURITY
-- ============================================================================

ALTER TABLE government_portal_connections ENABLE ROW LEVEL SECURITY;
ALTER TABLE regulatory_report_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE regulatory_submissions ENABLE ROW LEVEL SECURITY;
ALTER TABLE regulatory_submission_audit ENABLE ROW LEVEL SECURITY;
ALTER TABLE regulatory_submission_attachments ENABLE ROW LEVEL SECURITY;
ALTER TABLE regulatory_submission_schedules ENABLE ROW LEVEL SECURITY;

-- Portal connections - org access
CREATE POLICY gov_portal_connections_org_access ON government_portal_connections
    FOR ALL
    USING (organization_id = NULLIF(current_setting('app.current_tenant_id', TRUE), '')::UUID OR is_super_admin());

-- Templates - public read, admin write
CREATE POLICY reg_report_templates_read ON regulatory_report_templates
    FOR SELECT
    USING (TRUE);

CREATE POLICY reg_report_templates_admin ON regulatory_report_templates
    FOR ALL
    USING (is_super_admin());

-- Submissions - org access
CREATE POLICY reg_submissions_org_access ON regulatory_submissions
    FOR ALL
    USING (organization_id = NULLIF(current_setting('app.current_tenant_id', TRUE), '')::UUID OR is_super_admin());

-- Audit - via submission org
CREATE POLICY reg_submission_audit_access ON regulatory_submission_audit
    FOR SELECT
    USING (EXISTS (
        SELECT 1 FROM regulatory_submissions s
        WHERE s.id = submission_id
        AND (s.organization_id = NULLIF(current_setting('app.current_tenant_id', TRUE), '')::UUID OR is_super_admin())
    ));

-- Attachments - via submission org
CREATE POLICY reg_submission_attachments_access ON regulatory_submission_attachments
    FOR ALL
    USING (EXISTS (
        SELECT 1 FROM regulatory_submissions s
        WHERE s.id = submission_id
        AND (s.organization_id = NULLIF(current_setting('app.current_tenant_id', TRUE), '')::UUID OR is_super_admin())
    ));

-- Schedules - org access
CREATE POLICY reg_submission_schedules_org_access ON regulatory_submission_schedules
    FOR ALL
    USING (organization_id = NULLIF(current_setting('app.current_tenant_id', TRUE), '')::UUID OR is_super_admin());

-- ============================================================================
-- HELPER FUNCTIONS
-- ============================================================================

-- Function to calculate next due date based on template frequency
CREATE OR REPLACE FUNCTION calculate_next_due_date(
    p_template_id UUID,
    p_current_date DATE DEFAULT CURRENT_DATE
) RETURNS DATE AS $$
DECLARE
    v_template regulatory_report_templates%ROWTYPE;
    v_next_date DATE;
BEGIN
    SELECT * INTO v_template FROM regulatory_report_templates WHERE id = p_template_id;

    IF v_template IS NULL THEN
        RETURN NULL;
    END IF;

    CASE v_template.frequency
        WHEN 'monthly' THEN
            -- Next month, due day
            v_next_date := DATE_TRUNC('month', p_current_date) + INTERVAL '1 month' +
                           (COALESCE(v_template.due_day_of_month, 15) - 1) * INTERVAL '1 day';
        WHEN 'quarterly' THEN
            -- Next quarter, due month, due day
            v_next_date := DATE_TRUNC('quarter', p_current_date) + INTERVAL '3 months' +
                           (COALESCE(v_template.due_month_of_quarter, 1) - 1) * INTERVAL '1 month' +
                           (COALESCE(v_template.due_day_of_month, 15) - 1) * INTERVAL '1 day';
        WHEN 'yearly' THEN
            -- Next year, January, due day
            v_next_date := DATE_TRUNC('year', p_current_date) + INTERVAL '1 year' +
                           (COALESCE(v_template.due_day_of_month, 31) - 1) * INTERVAL '1 day';
        ELSE
            v_next_date := NULL;
    END CASE;

    RETURN v_next_date;
END;
$$ LANGUAGE plpgsql;

-- Function to generate submission reference
CREATE OR REPLACE FUNCTION generate_submission_reference(
    p_organization_id UUID,
    p_report_type TEXT
) RETURNS TEXT AS $$
DECLARE
    v_year TEXT;
    v_seq INTEGER;
    v_ref TEXT;
BEGIN
    v_year := TO_CHAR(NOW(), 'YYYY');

    -- Get next sequence for this org/year
    SELECT COUNT(*) + 1 INTO v_seq
    FROM regulatory_submissions
    WHERE organization_id = p_organization_id
    AND EXTRACT(YEAR FROM created_at) = EXTRACT(YEAR FROM NOW());

    v_ref := p_report_type || '-' || v_year || '-' || LPAD(v_seq::TEXT, 5, '0');

    RETURN v_ref;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- SEED DATA: Common Report Templates for Slovakia
-- ============================================================================

INSERT INTO regulatory_report_templates (template_code, template_name, description, portal_type, country_code, schema_version, frequency, due_day_of_month) VALUES
('SK_VAT_MONTHLY', 'Monthly VAT Return', 'Monthly value-added tax return', 'tax_authority', 'SK', '2024.1', 'monthly', 25),
('SK_TAX_YEARLY', 'Annual Income Tax Return', 'Yearly income tax declaration', 'tax_authority', 'SK', '2024.1', 'yearly', 31),
('SK_STAT_QUARTERLY', 'Statistical Quarterly Report', 'Quarterly statistical reporting', 'statistical_office', 'SK', '2024.1', 'quarterly', 15),
('SK_GUEST_REGISTER', 'Guest Registration', 'Foreign guest registration', 'police_registry', 'SK', '2024.1', 'event_driven', NULL),
('SK_ENERGY_YEARLY', 'Annual Energy Report', 'Building energy efficiency report', 'energy_authority', 'SK', '2024.1', 'yearly', 31);

-- Czech Republic
INSERT INTO regulatory_report_templates (template_code, template_name, description, portal_type, country_code, schema_version, frequency, due_day_of_month) VALUES
('CZ_VAT_MONTHLY', 'Monthly VAT Return', 'Monthly value-added tax return', 'tax_authority', 'CZ', '2024.1', 'monthly', 25),
('CZ_STAT_QUARTERLY', 'Statistical Quarterly Report', 'Quarterly statistical reporting', 'statistical_office', 'CZ', '2024.1', 'quarterly', 20),
('CZ_GUEST_REGISTER', 'Guest Registration', 'Foreign guest registration', 'police_registry', 'CZ', '2024.1', 'event_driven', NULL);

-- ============================================================================
-- COMMENTS
-- ============================================================================

COMMENT ON TABLE government_portal_connections IS 'Configured connections to government portals (Epic 30)';
COMMENT ON TABLE regulatory_report_templates IS 'Templates for regulatory reports (Epic 30)';
COMMENT ON TABLE regulatory_submissions IS 'Submitted regulatory reports (Epic 30)';
COMMENT ON TABLE regulatory_submission_audit IS 'Audit trail for regulatory submissions';
COMMENT ON TABLE regulatory_submission_attachments IS 'Attachments to regulatory submissions';
COMMENT ON TABLE regulatory_submission_schedules IS 'Scheduled recurring submissions';
