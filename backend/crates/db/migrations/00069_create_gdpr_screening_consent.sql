-- Epic 63: GDPR-Compliant Tenant Screening
-- Migration: 00069_create_gdpr_screening_consent.sql

-- =============================================================================
-- ENUMs
-- =============================================================================

-- Consent status
CREATE TYPE screening_consent_status AS ENUM (
    'pending',
    'granted',
    'denied',
    'withdrawn',
    'expired'
);

-- Consent scope - granular consent options
CREATE TYPE screening_consent_scope AS ENUM (
    'credit_check',
    'background_check',
    'reference_check',
    'employment_verification',
    'income_verification',
    'rental_history',
    'criminal_record',
    'eviction_history'
);

-- =============================================================================
-- TABLES
-- =============================================================================

-- Screening Consent Requests
CREATE TABLE screening_consent_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    application_id UUID NOT NULL REFERENCES tenant_applications(id) ON DELETE CASCADE,
    screening_id UUID REFERENCES tenant_screenings(id),

    -- Request details
    requested_by UUID NOT NULL REFERENCES users(id),
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL, -- GDPR: time-limited consent

    -- Consent scopes requested
    requested_scopes screening_consent_scope[] NOT NULL,

    -- Clear explanation of data usage (GDPR Article 13)
    purpose_description TEXT NOT NULL,
    data_to_be_collected TEXT NOT NULL, -- What data will be collected
    data_recipients TEXT, -- Who will have access
    retention_period TEXT NOT NULL, -- How long data will be kept
    legal_basis TEXT NOT NULL, -- Legal basis for processing

    -- Third-party processors
    third_party_processors JSONB DEFAULT '[]'::jsonb, -- [{name, purpose, country}]

    -- Contact information (GDPR: data controller)
    data_controller_name VARCHAR(255) NOT NULL,
    data_controller_email VARCHAR(255) NOT NULL,
    data_protection_officer_email VARCHAR(255),

    -- Status
    status screening_consent_status NOT NULL DEFAULT 'pending',

    -- Response tracking
    responded_at TIMESTAMPTZ,
    response_ip_address INET,
    response_user_agent TEXT,

    -- Reminder tracking
    reminder_sent_at TIMESTAMPTZ,
    reminder_count INT DEFAULT 0,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Screening Consent Records (Audit Trail)
CREATE TABLE screening_consents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    application_id UUID NOT NULL REFERENCES tenant_applications(id) ON DELETE CASCADE,
    consent_request_id UUID NOT NULL REFERENCES screening_consent_requests(id) ON DELETE CASCADE,

    -- Applicant info (denormalized for audit)
    applicant_name VARCHAR(255) NOT NULL,
    applicant_email VARCHAR(255) NOT NULL,
    applicant_user_id UUID REFERENCES users(id),

    -- Consent details
    granted_scopes screening_consent_scope[] NOT NULL,
    denied_scopes screening_consent_scope[] DEFAULT ARRAY[]::screening_consent_scope[],

    -- Status
    status screening_consent_status NOT NULL DEFAULT 'granted',

    -- Timestamps (GDPR audit trail)
    granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    withdrawn_at TIMESTAMPTZ,
    withdrawal_reason TEXT,

    -- Technical details for proof
    ip_address INET NOT NULL,
    user_agent TEXT,
    consent_method VARCHAR(50) NOT NULL, -- 'web', 'mobile', 'email', 'sms'

    -- Consent form version (for tracking)
    consent_form_version VARCHAR(50) NOT NULL,
    consent_text_hash VARCHAR(64) NOT NULL, -- SHA-256 of consent text shown

    -- Digital signature/proof
    signature_data JSONB, -- {type: 'checkbox'|'signature', data: '...'}

    -- Expiry
    expires_at TIMESTAMPTZ NOT NULL,
    expiry_notified_at TIMESTAMPTZ,

    -- Data minimization flags
    minimal_data_only BOOLEAN DEFAULT TRUE,
    anonymize_after_decision BOOLEAN DEFAULT TRUE,

    -- Metadata
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Consent Audit Log (GDPR Article 30: Records of processing)
CREATE TABLE screening_consent_audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    consent_id UUID NOT NULL REFERENCES screening_consents(id) ON DELETE CASCADE,

    -- Event details
    event_type VARCHAR(50) NOT NULL, -- 'granted', 'withdrawn', 'accessed', 'shared', 'deleted'
    event_timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Actor
    actor_user_id UUID REFERENCES users(id),
    actor_name VARCHAR(255),
    actor_role VARCHAR(50),

    -- Event data
    event_data JSONB, -- Context-specific data

    -- Technical details
    ip_address INET,
    user_agent TEXT,

    -- Purpose of access
    access_purpose TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Data Processing Activities (GDPR Article 30 compliance)
CREATE TABLE screening_data_processing_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    application_id UUID NOT NULL REFERENCES tenant_applications(id) ON DELETE CASCADE,
    consent_id UUID NOT NULL REFERENCES screening_consents(id) ON DELETE CASCADE,

    -- Processing details
    processing_purpose VARCHAR(255) NOT NULL,
    data_categories TEXT[] NOT NULL, -- ['personal_info', 'financial', 'criminal']
    processing_legal_basis VARCHAR(100) NOT NULL, -- 'consent', 'contract', 'legal_obligation'

    -- Data subjects
    data_subject_categories TEXT[] NOT NULL, -- ['applicant', 'guarantor']

    -- Recipients
    recipient_categories TEXT[], -- ['property_manager', 'credit_agency']

    -- Transfers
    international_transfers BOOLEAN DEFAULT FALSE,
    transfer_safeguards TEXT, -- Mechanisms for international transfers

    -- Retention
    retention_period VARCHAR(100) NOT NULL,
    deletion_scheduled_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ,

    -- Security measures
    security_measures TEXT[], -- ['encryption', 'access_control', 'pseudonymization']

    -- Processing timestamps
    processing_started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processing_completed_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Right to Access Requests (GDPR Article 15)
CREATE TABLE screening_data_access_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Requester
    requester_email VARCHAR(255) NOT NULL,
    requester_name VARCHAR(255),
    requester_user_id UUID REFERENCES users(id),

    -- Request details
    request_type VARCHAR(50) NOT NULL, -- 'access', 'portability', 'rectification', 'erasure'
    request_description TEXT,

    -- Related applications/consents
    related_application_ids UUID[],
    related_consent_ids UUID[],

    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- 'pending', 'verified', 'processing', 'completed', 'rejected'

    -- Identity verification
    identity_verified_at TIMESTAMPTZ,
    verification_method VARCHAR(50), -- 'email', 'id_document', 'video_call'
    verified_by UUID REFERENCES users(id),

    -- Response
    response_delivered_at TIMESTAMPTZ,
    response_method VARCHAR(50), -- 'email', 'download', 'mail'
    response_data_url VARCHAR(500),
    rejection_reason TEXT,

    -- SLA tracking (GDPR: 1 month to respond)
    due_date DATE NOT NULL,
    extended_due_date DATE,
    extension_reason TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- INDEXES
-- =============================================================================

-- Consent Requests
CREATE INDEX idx_screening_consent_requests_org ON screening_consent_requests(organization_id);
CREATE INDEX idx_screening_consent_requests_application ON screening_consent_requests(application_id);
CREATE INDEX idx_screening_consent_requests_status ON screening_consent_requests(status);
CREATE INDEX idx_screening_consent_requests_expires ON screening_consent_requests(expires_at) WHERE status = 'pending';
CREATE INDEX idx_screening_consent_requests_requested_by ON screening_consent_requests(requested_by);

-- Consents
CREATE INDEX idx_screening_consents_org ON screening_consents(organization_id);
CREATE INDEX idx_screening_consents_application ON screening_consents(application_id);
CREATE INDEX idx_screening_consents_request ON screening_consents(consent_request_id);
CREATE INDEX idx_screening_consents_status ON screening_consents(status);
CREATE INDEX idx_screening_consents_applicant_email ON screening_consents(applicant_email);
CREATE INDEX idx_screening_consents_expires ON screening_consents(expires_at) WHERE status = 'granted';
CREATE INDEX idx_screening_consents_granted_at ON screening_consents(granted_at);

-- Audit Log
CREATE INDEX idx_screening_consent_audit_consent ON screening_consent_audit_log(consent_id);
CREATE INDEX idx_screening_consent_audit_type ON screening_consent_audit_log(event_type);
CREATE INDEX idx_screening_consent_audit_timestamp ON screening_consent_audit_log(event_timestamp);
CREATE INDEX idx_screening_consent_audit_actor ON screening_consent_audit_log(actor_user_id);

-- Processing Records
CREATE INDEX idx_screening_processing_org ON screening_data_processing_records(organization_id);
CREATE INDEX idx_screening_processing_application ON screening_data_processing_records(application_id);
CREATE INDEX idx_screening_processing_consent ON screening_data_processing_records(consent_id);
CREATE INDEX idx_screening_processing_deletion ON screening_data_processing_records(deletion_scheduled_at) WHERE deleted_at IS NULL;

-- Access Requests
CREATE INDEX idx_screening_access_requests_org ON screening_data_access_requests(organization_id);
CREATE INDEX idx_screening_access_requests_email ON screening_data_access_requests(requester_email);
CREATE INDEX idx_screening_access_requests_status ON screening_data_access_requests(status);
CREATE INDEX idx_screening_access_requests_due ON screening_data_access_requests(due_date) WHERE status IN ('pending', 'verified', 'processing');

-- =============================================================================
-- TRIGGERS
-- =============================================================================

-- Updated at triggers
CREATE TRIGGER update_screening_consent_requests_updated_at
    BEFORE UPDATE ON screening_consent_requests
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_screening_consents_updated_at
    BEFORE UPDATE ON screening_consents
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_screening_processing_updated_at
    BEFORE UPDATE ON screening_data_processing_records
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_screening_access_requests_updated_at
    BEFORE UPDATE ON screening_data_access_requests
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Audit logging trigger
CREATE OR REPLACE FUNCTION log_screening_consent_change()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO screening_consent_audit_log (
            consent_id,
            event_type,
            event_data,
            actor_name
        ) VALUES (
            NEW.id,
            'granted',
            jsonb_build_object('scopes', NEW.granted_scopes),
            NEW.applicant_name
        );
    ELSIF TG_OP = 'UPDATE' AND OLD.status != NEW.status THEN
        INSERT INTO screening_consent_audit_log (
            consent_id,
            event_type,
            event_data,
            actor_name
        ) VALUES (
            NEW.id,
            CASE
                WHEN NEW.status = 'withdrawn' THEN 'withdrawn'
                WHEN NEW.status = 'expired' THEN 'expired'
                ELSE 'status_changed'
            END,
            jsonb_build_object(
                'old_status', OLD.status,
                'new_status', NEW.status,
                'reason', NEW.withdrawal_reason
            ),
            NEW.applicant_name
        );
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER screening_consent_audit_trigger
    AFTER INSERT OR UPDATE ON screening_consents
    FOR EACH ROW EXECUTE FUNCTION log_screening_consent_change();

-- =============================================================================
-- RLS POLICIES
-- =============================================================================

ALTER TABLE screening_consent_requests ENABLE ROW LEVEL SECURITY;
ALTER TABLE screening_consents ENABLE ROW LEVEL SECURITY;
ALTER TABLE screening_consent_audit_log ENABLE ROW LEVEL SECURITY;
ALTER TABLE screening_data_processing_records ENABLE ROW LEVEL SECURITY;
ALTER TABLE screening_data_access_requests ENABLE ROW LEVEL SECURITY;

-- RLS policies
CREATE POLICY screening_consent_requests_org_isolation ON screening_consent_requests
    USING (organization_id = current_setting('app.current_org_id', true)::uuid);

CREATE POLICY screening_consents_org_isolation ON screening_consents
    USING (organization_id = current_setting('app.current_org_id', true)::uuid);

CREATE POLICY screening_audit_log_org_isolation ON screening_consent_audit_log
    USING (consent_id IN (
        SELECT id FROM screening_consents
        WHERE organization_id = current_setting('app.current_org_id', true)::uuid
    ));

CREATE POLICY screening_processing_org_isolation ON screening_data_processing_records
    USING (organization_id = current_setting('app.current_org_id', true)::uuid);

CREATE POLICY screening_access_requests_org_isolation ON screening_data_access_requests
    USING (organization_id = current_setting('app.current_org_id', true)::uuid);

-- =============================================================================
-- FUNCTIONS
-- =============================================================================

-- Function to check if consent is still valid
CREATE OR REPLACE FUNCTION is_screening_consent_valid(consent_uuid UUID)
RETURNS BOOLEAN AS $$
DECLARE
    consent_record screening_consents%ROWTYPE;
BEGIN
    SELECT * INTO consent_record FROM screening_consents WHERE id = consent_uuid;

    IF NOT FOUND THEN
        RETURN FALSE;
    END IF;

    RETURN consent_record.status = 'granted'
        AND consent_record.expires_at > NOW()
        AND consent_record.withdrawn_at IS NULL;
END;
$$ LANGUAGE plpgsql;

-- Function to auto-expire old consents
CREATE OR REPLACE FUNCTION expire_old_screening_consents()
RETURNS void AS $$
BEGIN
    UPDATE screening_consents
    SET
        status = 'expired',
        updated_at = NOW()
    WHERE
        status = 'granted'
        AND expires_at < NOW()
        AND status != 'expired';
END;
$$ LANGUAGE plpgsql;

-- =============================================================================
-- COMMENTS
-- =============================================================================

COMMENT ON TABLE screening_consent_requests IS 'GDPR-compliant consent requests for tenant screening';
COMMENT ON TABLE screening_consents IS 'Audit trail of screening consents (GDPR Article 30)';
COMMENT ON TABLE screening_consent_audit_log IS 'Complete audit log of all consent-related events';
COMMENT ON TABLE screening_data_processing_records IS 'Record of processing activities (GDPR Article 30)';
COMMENT ON TABLE screening_data_access_requests IS 'Data subject access requests (GDPR Article 15)';

COMMENT ON COLUMN screening_consents.consent_text_hash IS 'SHA-256 hash of consent text shown to user for verification';
COMMENT ON COLUMN screening_consents.minimal_data_only IS 'Flag indicating data minimization principle was followed';
COMMENT ON COLUMN screening_consents.anonymize_after_decision IS 'Auto-anonymize data after decision is made';
