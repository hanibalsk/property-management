-- Epic 19: Lease Management & Tenant Screening
-- Migration: 00052_create_lease_management.sql

-- =============================================================================
-- ENUMs
-- =============================================================================

-- Application status
CREATE TYPE tenant_application_status AS ENUM (
    'draft',
    'submitted',
    'under_review',
    'screening_pending',
    'screening_complete',
    'approved',
    'rejected',
    'withdrawn'
);

-- Screening status
CREATE TYPE screening_status AS ENUM (
    'pending_consent',
    'consent_received',
    'in_progress',
    'completed',
    'failed',
    'expired'
);

-- Screening type
CREATE TYPE screening_type AS ENUM (
    'background_check',
    'credit_check',
    'reference_check',
    'employment_verification',
    'income_verification',
    'rental_history'
);

-- Lease status
CREATE TYPE lease_status AS ENUM (
    'draft',
    'pending_signature',
    'active',
    'renewed',
    'terminated',
    'expired',
    'cancelled'
);

-- Termination reason
CREATE TYPE termination_reason AS ENUM (
    'end_of_term',
    'mutual_agreement',
    'tenant_breach',
    'landlord_breach',
    'property_sale',
    'renovation',
    'other'
);

-- =============================================================================
-- TABLES
-- =============================================================================

-- Tenant Applications
CREATE TABLE tenant_applications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,

    -- Applicant info
    applicant_name VARCHAR(255) NOT NULL,
    applicant_email VARCHAR(255) NOT NULL,
    applicant_phone VARCHAR(50),
    date_of_birth DATE,
    national_id VARCHAR(50),

    -- Current address
    current_address TEXT,
    current_landlord_name VARCHAR(255),
    current_landlord_phone VARCHAR(50),
    current_rent_amount DECIMAL(12, 2),
    current_tenancy_start DATE,

    -- Employment
    employer_name VARCHAR(255),
    employer_phone VARCHAR(50),
    job_title VARCHAR(255),
    employment_start DATE,
    monthly_income DECIMAL(12, 2),

    -- Desired lease
    desired_move_in DATE,
    desired_lease_term_months INT DEFAULT 12,
    proposed_rent DECIMAL(12, 2),

    -- Co-applicants
    co_applicants JSONB DEFAULT '[]'::jsonb,

    -- Documents
    documents JSONB DEFAULT '[]'::jsonb, -- [{name, url, type, uploaded_at}]

    -- Status tracking
    status tenant_application_status NOT NULL DEFAULT 'draft',
    submitted_at TIMESTAMPTZ,
    reviewed_by UUID REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    decision_notes TEXT,

    -- Metadata
    source VARCHAR(50), -- 'portal', 'direct', 'agent'
    referral_code VARCHAR(50),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Tenant Screening Records
CREATE TABLE tenant_screenings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID NOT NULL REFERENCES tenant_applications(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Screening type
    screening_type screening_type NOT NULL,
    provider VARCHAR(100), -- 'internal', 'equifax', 'experian', etc.

    -- Consent
    consent_requested_at TIMESTAMPTZ,
    consent_received_at TIMESTAMPTZ,
    consent_document_url VARCHAR(500),

    -- Status
    status screening_status NOT NULL DEFAULT 'pending_consent',
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,

    -- Results (encrypted/summarized)
    result_summary TEXT,
    risk_score INT CHECK (risk_score >= 0 AND risk_score <= 100),
    passed BOOLEAN,
    flags JSONB DEFAULT '[]'::jsonb, -- Red flags identified

    -- Raw response (encrypted)
    raw_response_encrypted BYTEA,

    -- Expiry
    expires_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Lease Templates
CREATE TABLE lease_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    name VARCHAR(255) NOT NULL,
    description TEXT,

    -- Template content
    content_html TEXT NOT NULL,
    content_variables JSONB DEFAULT '[]'::jsonb, -- Available merge fields

    -- Settings
    default_term_months INT DEFAULT 12,
    default_security_deposit_months DECIMAL(3, 1) DEFAULT 1.0,
    default_notice_period_days INT DEFAULT 30,

    -- Clauses
    clauses JSONB DEFAULT '[]'::jsonb, -- [{id, title, content, optional}]

    -- Metadata
    is_default BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    version INT DEFAULT 1,

    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Leases
CREATE TABLE leases (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    application_id UUID REFERENCES tenant_applications(id),
    template_id UUID REFERENCES lease_templates(id),

    -- Lease parties
    landlord_user_id UUID REFERENCES users(id),
    landlord_name VARCHAR(255) NOT NULL,
    landlord_address TEXT,

    -- Primary tenant
    tenant_user_id UUID REFERENCES users(id),
    tenant_name VARCHAR(255) NOT NULL,
    tenant_email VARCHAR(255) NOT NULL,
    tenant_phone VARCHAR(50),

    -- Additional occupants
    occupants JSONB DEFAULT '[]'::jsonb, -- [{name, relationship, dob}]

    -- Lease terms
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    term_months INT NOT NULL,
    is_fixed_term BOOLEAN DEFAULT TRUE,

    -- Financial terms
    monthly_rent DECIMAL(12, 2) NOT NULL,
    security_deposit DECIMAL(12, 2) NOT NULL,
    deposit_held_by VARCHAR(255),
    rent_due_day INT DEFAULT 1 CHECK (rent_due_day >= 1 AND rent_due_day <= 28),
    late_fee_amount DECIMAL(12, 2),
    late_fee_grace_days INT DEFAULT 5,

    -- Utilities & inclusions
    utilities_included JSONB DEFAULT '[]'::jsonb, -- ['water', 'electricity', 'gas', 'internet']
    parking_spaces INT DEFAULT 0,
    storage_units INT DEFAULT 0,

    -- Rules
    pets_allowed BOOLEAN DEFAULT FALSE,
    pet_deposit DECIMAL(12, 2),
    max_occupants INT,
    smoking_allowed BOOLEAN DEFAULT FALSE,

    -- Document
    document_url VARCHAR(500),
    document_version INT DEFAULT 1,

    -- Status
    status lease_status NOT NULL DEFAULT 'draft',

    -- Signatures
    landlord_signed_at TIMESTAMPTZ,
    tenant_signed_at TIMESTAMPTZ,
    signature_request_id UUID REFERENCES signature_requests(id),

    -- Termination
    terminated_at TIMESTAMPTZ,
    termination_reason termination_reason,
    termination_notes TEXT,
    termination_initiated_by UUID REFERENCES users(id),

    -- Renewal tracking
    previous_lease_id UUID REFERENCES leases(id),
    renewed_to_lease_id UUID REFERENCES leases(id),
    renewal_offered_at TIMESTAMPTZ,
    renewal_offer_expires_at TIMESTAMPTZ,

    -- Metadata
    notes TEXT,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Lease Amendments
CREATE TABLE lease_amendments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lease_id UUID NOT NULL REFERENCES leases(id) ON DELETE CASCADE,

    -- Amendment details
    amendment_number INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,

    -- Changes
    changes JSONB NOT NULL, -- {field: {old, new}}
    effective_date DATE NOT NULL,

    -- Document
    document_url VARCHAR(500),

    -- Signatures
    landlord_signed_at TIMESTAMPTZ,
    tenant_signed_at TIMESTAMPTZ,

    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Lease Payment Schedule
CREATE TABLE lease_payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lease_id UUID NOT NULL REFERENCES leases(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Payment details
    due_date DATE NOT NULL,
    amount DECIMAL(12, 2) NOT NULL,
    payment_type VARCHAR(50) NOT NULL, -- 'rent', 'deposit', 'late_fee', 'utility', 'other'
    description VARCHAR(255),

    -- Payment status
    paid_at TIMESTAMPTZ,
    paid_amount DECIMAL(12, 2),
    payment_method VARCHAR(50), -- 'bank_transfer', 'card', 'cash', 'check'
    payment_reference VARCHAR(100),

    -- Late fees
    is_late BOOLEAN DEFAULT FALSE,
    late_fee_applied DECIMAL(12, 2),

    -- Invoice link
    invoice_id UUID,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Lease Reminders
CREATE TABLE lease_reminders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lease_id UUID NOT NULL REFERENCES leases(id) ON DELETE CASCADE,

    -- Reminder type
    reminder_type VARCHAR(50) NOT NULL, -- 'renewal', 'payment', 'inspection', 'expiration'

    -- Timing
    trigger_date DATE NOT NULL,
    days_before_event INT,

    -- Content
    subject VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,

    -- Recipients
    recipients JSONB NOT NULL, -- [{user_id, email, type: 'landlord'|'tenant'}]

    -- Status
    sent_at TIMESTAMPTZ,
    acknowledged_at TIMESTAMPTZ,
    acknowledged_by UUID REFERENCES users(id),

    -- Recurrence
    is_recurring BOOLEAN DEFAULT FALSE,
    recurrence_pattern VARCHAR(50), -- 'monthly', 'yearly'

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- INDEXES
-- =============================================================================

-- Applications
CREATE INDEX idx_tenant_applications_org ON tenant_applications(organization_id);
CREATE INDEX idx_tenant_applications_unit ON tenant_applications(unit_id);
CREATE INDEX idx_tenant_applications_status ON tenant_applications(status);
CREATE INDEX idx_tenant_applications_applicant_email ON tenant_applications(applicant_email);
CREATE INDEX idx_tenant_applications_submitted ON tenant_applications(submitted_at);

-- Screenings
CREATE INDEX idx_tenant_screenings_application ON tenant_screenings(application_id);
CREATE INDEX idx_tenant_screenings_org ON tenant_screenings(organization_id);
CREATE INDEX idx_tenant_screenings_status ON tenant_screenings(status);
CREATE INDEX idx_tenant_screenings_type ON tenant_screenings(screening_type);

-- Templates
CREATE INDEX idx_lease_templates_org ON lease_templates(organization_id);
CREATE INDEX idx_lease_templates_active ON lease_templates(is_active) WHERE is_active = TRUE;

-- Leases
CREATE INDEX idx_leases_org ON leases(organization_id);
CREATE INDEX idx_leases_unit ON leases(unit_id);
CREATE INDEX idx_leases_status ON leases(status);
CREATE INDEX idx_leases_tenant ON leases(tenant_user_id);
CREATE INDEX idx_leases_landlord ON leases(landlord_user_id);
CREATE INDEX idx_leases_dates ON leases(start_date, end_date);
CREATE INDEX idx_leases_expiring ON leases(end_date) WHERE status = 'active';

-- Amendments
CREATE INDEX idx_lease_amendments_lease ON lease_amendments(lease_id);

-- Payments
CREATE INDEX idx_lease_payments_lease ON lease_payments(lease_id);
CREATE INDEX idx_lease_payments_org ON lease_payments(organization_id);
CREATE INDEX idx_lease_payments_due ON lease_payments(due_date);
CREATE INDEX idx_lease_payments_unpaid ON lease_payments(due_date) WHERE paid_at IS NULL;

-- Reminders
CREATE INDEX idx_lease_reminders_lease ON lease_reminders(lease_id);
CREATE INDEX idx_lease_reminders_trigger ON lease_reminders(trigger_date) WHERE sent_at IS NULL;

-- =============================================================================
-- TRIGGERS
-- =============================================================================

-- Updated at triggers
CREATE TRIGGER update_tenant_applications_updated_at
    BEFORE UPDATE ON tenant_applications
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_tenant_screenings_updated_at
    BEFORE UPDATE ON tenant_screenings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_lease_templates_updated_at
    BEFORE UPDATE ON lease_templates
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_leases_updated_at
    BEFORE UPDATE ON leases
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_lease_payments_updated_at
    BEFORE UPDATE ON lease_payments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- =============================================================================
-- RLS POLICIES
-- =============================================================================

ALTER TABLE tenant_applications ENABLE ROW LEVEL SECURITY;
ALTER TABLE tenant_screenings ENABLE ROW LEVEL SECURITY;
ALTER TABLE lease_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE leases ENABLE ROW LEVEL SECURITY;
ALTER TABLE lease_amendments ENABLE ROW LEVEL SECURITY;
ALTER TABLE lease_payments ENABLE ROW LEVEL SECURITY;
ALTER TABLE lease_reminders ENABLE ROW LEVEL SECURITY;

-- RLS policies (simplified - in production would be more granular)
CREATE POLICY tenant_applications_org_isolation ON tenant_applications
    USING (organization_id = current_setting('app.current_org_id', true)::uuid);

CREATE POLICY tenant_screenings_org_isolation ON tenant_screenings
    USING (organization_id = current_setting('app.current_org_id', true)::uuid);

CREATE POLICY lease_templates_org_isolation ON lease_templates
    USING (organization_id = current_setting('app.current_org_id', true)::uuid);

CREATE POLICY leases_org_isolation ON leases
    USING (organization_id = current_setting('app.current_org_id', true)::uuid);

CREATE POLICY lease_amendments_org_isolation ON lease_amendments
    USING (lease_id IN (SELECT id FROM leases WHERE organization_id = current_setting('app.current_org_id', true)::uuid));

CREATE POLICY lease_payments_org_isolation ON lease_payments
    USING (organization_id = current_setting('app.current_org_id', true)::uuid);

CREATE POLICY lease_reminders_org_isolation ON lease_reminders
    USING (lease_id IN (SELECT id FROM leases WHERE organization_id = current_setting('app.current_org_id', true)::uuid));
