-- Epic 22: Insurance Management
-- Migration: 00055_create_insurance.sql

-- ============================================
-- Story 22.1: Policy Registry
-- ============================================

-- Insurance policies table
CREATE TABLE IF NOT EXISTS insurance_policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Policy identification
    policy_number VARCHAR(100) NOT NULL,
    policy_name VARCHAR(255) NOT NULL,

    -- Insurance provider
    provider_name VARCHAR(255) NOT NULL,
    provider_contact VARCHAR(255),
    provider_phone VARCHAR(50),
    provider_email VARCHAR(255),

    -- Policy type
    policy_type VARCHAR(50) NOT NULL,

    -- Coverage details
    coverage_amount DECIMAL(15, 2),
    deductible DECIMAL(10, 2),
    premium_amount DECIMAL(10, 2),
    premium_frequency VARCHAR(50) DEFAULT 'annual', -- monthly, quarterly, annual

    -- Coverage scope (what's covered)
    building_id UUID REFERENCES buildings(id) ON DELETE SET NULL,
    unit_id UUID REFERENCES units(id) ON DELETE SET NULL,
    coverage_description TEXT,

    -- Dates
    effective_date DATE NOT NULL,
    expiration_date DATE NOT NULL,
    renewal_date DATE,

    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    auto_renew BOOLEAN DEFAULT FALSE,

    -- Metadata
    terms JSONB DEFAULT '{}', -- Additional terms and conditions
    metadata JSONB DEFAULT '{}',

    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    -- Constraints
    CONSTRAINT valid_policy_type CHECK (policy_type IN (
        'property', 'liability', 'flood', 'earthquake', 'fire',
        'umbrella', 'equipment', 'directors_officers', 'cyber', 'workers_comp', 'other'
    )),
    CONSTRAINT valid_policy_status CHECK (status IN ('active', 'expired', 'cancelled', 'pending', 'suspended')),
    CONSTRAINT valid_premium_frequency CHECK (premium_frequency IN ('monthly', 'quarterly', 'semi_annual', 'annual')),
    CONSTRAINT valid_policy_dates CHECK (expiration_date >= effective_date)
);

-- Indexes
CREATE INDEX idx_insurance_policies_organization ON insurance_policies(organization_id);
CREATE INDEX idx_insurance_policies_status ON insurance_policies(status);
CREATE INDEX idx_insurance_policies_type ON insurance_policies(policy_type);
CREATE INDEX idx_insurance_policies_building ON insurance_policies(building_id) WHERE building_id IS NOT NULL;
CREATE INDEX idx_insurance_policies_unit ON insurance_policies(unit_id) WHERE unit_id IS NOT NULL;
CREATE INDEX idx_insurance_policies_expiration ON insurance_policies(expiration_date) WHERE status = 'active';
CREATE INDEX idx_insurance_policies_policy_number ON insurance_policies(organization_id, policy_number);

-- ============================================
-- Story 22.2: Policy Document Management
-- ============================================

CREATE TABLE IF NOT EXISTS insurance_policy_documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    policy_id UUID NOT NULL REFERENCES insurance_policies(id) ON DELETE CASCADE,
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,

    document_type VARCHAR(50) NOT NULL DEFAULT 'policy', -- policy, endorsement, certificate, rider, declaration, invoice

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    UNIQUE(policy_id, document_id)
);

CREATE INDEX idx_insurance_policy_documents_policy ON insurance_policy_documents(policy_id);

-- ============================================
-- Story 22.3: Claims Submission & Tracking
-- ============================================

CREATE TABLE IF NOT EXISTS insurance_claims (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    policy_id UUID NOT NULL REFERENCES insurance_policies(id) ON DELETE CASCADE,

    -- Claim identification
    claim_number VARCHAR(100),
    provider_claim_number VARCHAR(100), -- Insurance company's claim number

    -- Claim details
    incident_date DATE NOT NULL,
    incident_description TEXT NOT NULL,
    incident_location TEXT,

    -- Related entities
    building_id UUID REFERENCES buildings(id) ON DELETE SET NULL,
    unit_id UUID REFERENCES units(id) ON DELETE SET NULL,
    fault_id UUID REFERENCES faults(id) ON DELETE SET NULL, -- If claim is related to a fault

    -- Financial
    claimed_amount DECIMAL(15, 2),
    approved_amount DECIMAL(15, 2),
    paid_amount DECIMAL(15, 2) DEFAULT 0,
    deductible_applied DECIMAL(10, 2),

    -- Currency
    currency VARCHAR(3) DEFAULT 'EUR',

    -- Status tracking
    status VARCHAR(50) NOT NULL DEFAULT 'draft',

    -- Workflow
    submitted_by UUID REFERENCES users(id) ON DELETE SET NULL,
    submitted_at TIMESTAMP WITH TIME ZONE,
    reviewed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    reviewed_at TIMESTAMP WITH TIME ZONE,
    adjuster_name VARCHAR(255),
    adjuster_phone VARCHAR(50),
    adjuster_email VARCHAR(255),

    -- Resolution
    resolution_notes TEXT,
    denial_reason TEXT,

    -- Metadata
    metadata JSONB DEFAULT '{}',

    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    -- Constraints
    CONSTRAINT valid_claim_status CHECK (status IN (
        'draft', 'submitted', 'under_review', 'information_requested',
        'approved', 'partially_approved', 'denied', 'paid', 'closed', 'withdrawn'
    ))
);

-- Indexes
CREATE INDEX idx_insurance_claims_organization ON insurance_claims(organization_id);
CREATE INDEX idx_insurance_claims_policy ON insurance_claims(policy_id);
CREATE INDEX idx_insurance_claims_status ON insurance_claims(status);
CREATE INDEX idx_insurance_claims_building ON insurance_claims(building_id) WHERE building_id IS NOT NULL;
CREATE INDEX idx_insurance_claims_incident_date ON insurance_claims(incident_date);
CREATE INDEX idx_insurance_claims_claim_number ON insurance_claims(organization_id, claim_number) WHERE claim_number IS NOT NULL;

-- Claim documents junction table
CREATE TABLE IF NOT EXISTS insurance_claim_documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    claim_id UUID NOT NULL REFERENCES insurance_claims(id) ON DELETE CASCADE,
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,

    document_type VARCHAR(50) NOT NULL DEFAULT 'evidence', -- evidence, estimate, invoice, correspondence, settlement

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    UNIQUE(claim_id, document_id)
);

CREATE INDEX idx_insurance_claim_documents_claim ON insurance_claim_documents(claim_id);

-- Claim status history
CREATE TABLE IF NOT EXISTS insurance_claim_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    claim_id UUID NOT NULL REFERENCES insurance_claims(id) ON DELETE CASCADE,

    old_status VARCHAR(50),
    new_status VARCHAR(50) NOT NULL,
    changed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    notes TEXT,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_insurance_claim_history_claim ON insurance_claim_history(claim_id);

-- ============================================
-- Story 22.4: Renewal Reminders
-- ============================================

CREATE TABLE IF NOT EXISTS insurance_renewal_reminders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    policy_id UUID NOT NULL REFERENCES insurance_policies(id) ON DELETE CASCADE,

    -- Reminder configuration
    days_before_expiry INTEGER NOT NULL, -- Days before expiration to send reminder
    reminder_type VARCHAR(50) NOT NULL DEFAULT 'email', -- email, notification, both

    -- Status
    sent_at TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN DEFAULT TRUE,

    -- Metadata
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    CONSTRAINT valid_days_before CHECK (days_before_expiry > 0),
    CONSTRAINT valid_reminder_type CHECK (reminder_type IN ('email', 'notification', 'both'))
);

CREATE INDEX idx_insurance_renewal_reminders_policy ON insurance_renewal_reminders(policy_id);
CREATE INDEX idx_insurance_renewal_reminders_active ON insurance_renewal_reminders(is_active) WHERE is_active = TRUE;

-- ============================================
-- Insurance Coverage Summary View
-- ============================================

CREATE OR REPLACE VIEW insurance_coverage_summary AS
SELECT
    p.organization_id,
    p.building_id,
    COUNT(*) AS total_policies,
    COUNT(*) FILTER (WHERE p.status = 'active') AS active_policies,
    SUM(p.coverage_amount) FILTER (WHERE p.status = 'active') AS total_coverage,
    SUM(p.premium_amount) FILTER (WHERE p.status = 'active') AS total_premiums,
    MIN(p.expiration_date) FILTER (WHERE p.status = 'active') AS next_expiration,
    array_agg(DISTINCT p.policy_type) FILTER (WHERE p.status = 'active') AS coverage_types
FROM insurance_policies p
GROUP BY p.organization_id, p.building_id;

-- ============================================
-- RLS Policies
-- ============================================

-- Enable RLS
ALTER TABLE insurance_policies ENABLE ROW LEVEL SECURITY;
ALTER TABLE insurance_policy_documents ENABLE ROW LEVEL SECURITY;
ALTER TABLE insurance_claims ENABLE ROW LEVEL SECURITY;
ALTER TABLE insurance_claim_documents ENABLE ROW LEVEL SECURITY;
ALTER TABLE insurance_claim_history ENABLE ROW LEVEL SECURITY;
ALTER TABLE insurance_renewal_reminders ENABLE ROW LEVEL SECURITY;

-- Insurance policies policies
CREATE POLICY insurance_policies_org_isolation ON insurance_policies
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', true)::UUID);

-- Policy documents policies
CREATE POLICY insurance_policy_documents_org_isolation ON insurance_policy_documents
    FOR ALL USING (
        policy_id IN (
            SELECT id FROM insurance_policies WHERE organization_id = current_setting('app.current_organization_id', true)::UUID
        )
    );

-- Insurance claims policies
CREATE POLICY insurance_claims_org_isolation ON insurance_claims
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', true)::UUID);

-- Claim documents policies
CREATE POLICY insurance_claim_documents_org_isolation ON insurance_claim_documents
    FOR ALL USING (
        claim_id IN (
            SELECT id FROM insurance_claims WHERE organization_id = current_setting('app.current_organization_id', true)::UUID
        )
    );

-- Claim history policies
CREATE POLICY insurance_claim_history_org_isolation ON insurance_claim_history
    FOR ALL USING (
        claim_id IN (
            SELECT id FROM insurance_claims WHERE organization_id = current_setting('app.current_organization_id', true)::UUID
        )
    );

-- Renewal reminders policies
CREATE POLICY insurance_renewal_reminders_org_isolation ON insurance_renewal_reminders
    FOR ALL USING (
        policy_id IN (
            SELECT id FROM insurance_policies WHERE organization_id = current_setting('app.current_organization_id', true)::UUID
        )
    );

-- ============================================
-- Triggers for updated_at
-- ============================================

CREATE TRIGGER update_insurance_policies_updated_at
    BEFORE UPDATE ON insurance_policies
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_insurance_claims_updated_at
    BEFORE UPDATE ON insurance_claims
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_insurance_renewal_reminders_updated_at
    BEFORE UPDATE ON insurance_renewal_reminders
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================
-- Trigger for claim status history
-- ============================================

CREATE OR REPLACE FUNCTION log_claim_status_change()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.status IS DISTINCT FROM NEW.status THEN
        INSERT INTO insurance_claim_history (claim_id, old_status, new_status, changed_by)
        VALUES (NEW.id, OLD.status, NEW.status, NULL);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER track_claim_status_changes
    AFTER UPDATE ON insurance_claims
    FOR EACH ROW
    EXECUTE FUNCTION log_claim_status_change();

-- ============================================
-- Function to check expiring policies
-- ============================================

CREATE OR REPLACE FUNCTION get_expiring_policies(
    p_organization_id UUID,
    p_days_ahead INTEGER DEFAULT 30
)
RETURNS TABLE (
    policy_id UUID,
    policy_number VARCHAR(100),
    policy_name VARCHAR(255),
    policy_type VARCHAR(50),
    provider_name VARCHAR(255),
    expiration_date DATE,
    days_until_expiry INTEGER,
    coverage_amount DECIMAL(15, 2),
    auto_renew BOOLEAN
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        p.id,
        p.policy_number,
        p.policy_name,
        p.policy_type,
        p.provider_name,
        p.expiration_date,
        (p.expiration_date - CURRENT_DATE)::INTEGER,
        p.coverage_amount,
        p.auto_renew
    FROM insurance_policies p
    WHERE p.organization_id = p_organization_id
      AND p.status = 'active'
      AND p.expiration_date <= CURRENT_DATE + p_days_ahead
    ORDER BY p.expiration_date;
END;
$$ LANGUAGE plpgsql;
