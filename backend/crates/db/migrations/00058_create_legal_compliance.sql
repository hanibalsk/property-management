-- Epic 25: Legal Document & Compliance
-- Migration: 00058_create_legal_compliance.sql
-- Creates tables for legal documents, compliance tracking, and legal notices.

-- ===========================================
-- Legal Documents Table
-- ===========================================
CREATE TABLE IF NOT EXISTS legal_documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    building_id UUID,
    document_type VARCHAR(100) NOT NULL,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    parties JSONB DEFAULT '[]',
    effective_date DATE,
    expiry_date DATE,
    file_path VARCHAR(1000),
    file_name VARCHAR(255),
    file_size BIGINT,
    mime_type VARCHAR(100),
    is_confidential BOOLEAN DEFAULT FALSE,
    retention_period_months INTEGER,
    retention_expires_at DATE,
    tags TEXT[],
    metadata JSONB DEFAULT '{}',
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE legal_documents IS 'Repository for legal documents';
COMMENT ON COLUMN legal_documents.document_type IS 'contract, lease_template, regulation, court_order, legal_notice, minutes, policy, permit, certificate, other';
COMMENT ON COLUMN legal_documents.parties IS 'JSON array of parties involved in the document';

-- ===========================================
-- Legal Document Versions Table
-- ===========================================
CREATE TABLE IF NOT EXISTS legal_document_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID NOT NULL REFERENCES legal_documents(id) ON DELETE CASCADE,
    version_number INTEGER NOT NULL,
    file_path VARCHAR(1000) NOT NULL,
    file_name VARCHAR(255) NOT NULL,
    file_size BIGINT,
    mime_type VARCHAR(100),
    change_notes TEXT,
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(document_id, version_number)
);

COMMENT ON TABLE legal_document_versions IS 'Version history for legal documents';

-- ===========================================
-- Compliance Requirements Table
-- ===========================================
CREATE TABLE IF NOT EXISTS compliance_requirements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    building_id UUID,
    name VARCHAR(500) NOT NULL,
    description TEXT,
    category VARCHAR(100) NOT NULL,
    regulation_reference VARCHAR(255),
    frequency VARCHAR(50) NOT NULL DEFAULT 'annually',
    last_verified_at TIMESTAMPTZ,
    last_verified_by UUID,
    next_due_date DATE,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    is_mandatory BOOLEAN DEFAULT TRUE,
    responsible_party VARCHAR(255),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE compliance_requirements IS 'Regulatory compliance requirements tracking';
COMMENT ON COLUMN compliance_requirements.category IS 'fire_safety, elevator, electrical, plumbing, accessibility, environmental, health, privacy, other';
COMMENT ON COLUMN compliance_requirements.frequency IS 'once, monthly, quarterly, semi_annually, annually, biennially, as_needed';
COMMENT ON COLUMN compliance_requirements.status IS 'pending, compliant, non_compliant, in_progress, waived, expired';

-- ===========================================
-- Compliance Verification History Table
-- ===========================================
CREATE TABLE IF NOT EXISTS compliance_verifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    requirement_id UUID NOT NULL REFERENCES compliance_requirements(id) ON DELETE CASCADE,
    verified_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    verified_by UUID NOT NULL,
    status VARCHAR(50) NOT NULL,
    notes TEXT,
    evidence_document_id UUID REFERENCES legal_documents(id),
    inspector_name VARCHAR(255),
    certificate_number VARCHAR(100),
    valid_until DATE,
    issues_found TEXT,
    corrective_actions TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE compliance_verifications IS 'History of compliance verifications';

-- ===========================================
-- Legal Notices Table
-- ===========================================
CREATE TABLE IF NOT EXISTS legal_notices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    building_id UUID,
    notice_type VARCHAR(100) NOT NULL,
    subject VARCHAR(500) NOT NULL,
    content TEXT NOT NULL,
    priority VARCHAR(50) DEFAULT 'normal',
    delivery_method VARCHAR(50) NOT NULL DEFAULT 'email',
    requires_acknowledgment BOOLEAN DEFAULT FALSE,
    acknowledgment_deadline TIMESTAMPTZ,
    sent_at TIMESTAMPTZ,
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE legal_notices IS 'Legal notices sent to residents/units';
COMMENT ON COLUMN legal_notices.notice_type IS 'rent_increase, lease_violation, eviction_warning, rules_update, general_notice, assembly_meeting, other';
COMMENT ON COLUMN legal_notices.priority IS 'low, normal, high, urgent';
COMMENT ON COLUMN legal_notices.delivery_method IS 'email, mail, both, in_app';

-- ===========================================
-- Legal Notice Recipients Table
-- ===========================================
CREATE TABLE IF NOT EXISTS legal_notice_recipients (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    notice_id UUID NOT NULL REFERENCES legal_notices(id) ON DELETE CASCADE,
    recipient_type VARCHAR(50) NOT NULL,
    recipient_id UUID NOT NULL,
    recipient_name VARCHAR(255),
    recipient_email VARCHAR(255),
    delivery_status VARCHAR(50) DEFAULT 'pending',
    delivered_at TIMESTAMPTZ,
    delivery_error TEXT,
    acknowledged_at TIMESTAMPTZ,
    acknowledgment_method VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE legal_notice_recipients IS 'Recipients of legal notices';
COMMENT ON COLUMN legal_notice_recipients.recipient_type IS 'user, unit, building, all';
COMMENT ON COLUMN legal_notice_recipients.delivery_status IS 'pending, sent, delivered, failed, bounced';

-- ===========================================
-- Compliance Templates Table
-- ===========================================
CREATE TABLE IF NOT EXISTS compliance_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID,
    name VARCHAR(255) NOT NULL,
    category VARCHAR(100) NOT NULL,
    description TEXT,
    checklist_items JSONB NOT NULL DEFAULT '[]',
    frequency VARCHAR(50) NOT NULL DEFAULT 'annually',
    is_system BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE compliance_templates IS 'Templates for common compliance requirements';

-- ===========================================
-- Compliance Audit Trail Table
-- ===========================================
CREATE TABLE IF NOT EXISTS compliance_audit_trail (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    requirement_id UUID REFERENCES compliance_requirements(id),
    document_id UUID REFERENCES legal_documents(id),
    notice_id UUID REFERENCES legal_notices(id),
    action VARCHAR(100) NOT NULL,
    action_by UUID NOT NULL,
    action_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    old_values JSONB,
    new_values JSONB,
    notes TEXT
);

COMMENT ON TABLE compliance_audit_trail IS 'Audit trail for all compliance-related activities';

-- ===========================================
-- Indexes
-- ===========================================
CREATE INDEX idx_legal_documents_organization ON legal_documents(organization_id);
CREATE INDEX idx_legal_documents_building ON legal_documents(building_id) WHERE building_id IS NOT NULL;
CREATE INDEX idx_legal_documents_type ON legal_documents(document_type);
CREATE INDEX idx_legal_documents_effective ON legal_documents(effective_date);
CREATE INDEX idx_legal_documents_expiry ON legal_documents(expiry_date) WHERE expiry_date IS NOT NULL;

CREATE INDEX idx_legal_versions_document ON legal_document_versions(document_id);

CREATE INDEX idx_compliance_req_organization ON compliance_requirements(organization_id);
CREATE INDEX idx_compliance_req_building ON compliance_requirements(building_id) WHERE building_id IS NOT NULL;
CREATE INDEX idx_compliance_req_status ON compliance_requirements(status);
CREATE INDEX idx_compliance_req_due ON compliance_requirements(next_due_date);
CREATE INDEX idx_compliance_req_category ON compliance_requirements(category);

CREATE INDEX idx_compliance_verifications_requirement ON compliance_verifications(requirement_id);
CREATE INDEX idx_compliance_verifications_date ON compliance_verifications(verified_at);

CREATE INDEX idx_legal_notices_organization ON legal_notices(organization_id);
CREATE INDEX idx_legal_notices_building ON legal_notices(building_id) WHERE building_id IS NOT NULL;
CREATE INDEX idx_legal_notices_type ON legal_notices(notice_type);
CREATE INDEX idx_legal_notices_sent ON legal_notices(sent_at);

CREATE INDEX idx_notice_recipients_notice ON legal_notice_recipients(notice_id);
CREATE INDEX idx_notice_recipients_recipient ON legal_notice_recipients(recipient_id);
CREATE INDEX idx_notice_recipients_status ON legal_notice_recipients(delivery_status);

CREATE INDEX idx_compliance_templates_organization ON compliance_templates(organization_id) WHERE organization_id IS NOT NULL;
CREATE INDEX idx_compliance_templates_category ON compliance_templates(category);

CREATE INDEX idx_compliance_audit_organization ON compliance_audit_trail(organization_id);
CREATE INDEX idx_compliance_audit_action ON compliance_audit_trail(action);
CREATE INDEX idx_compliance_audit_date ON compliance_audit_trail(action_at);

-- ===========================================
-- RLS Policies
-- ===========================================
ALTER TABLE legal_documents ENABLE ROW LEVEL SECURITY;
ALTER TABLE legal_document_versions ENABLE ROW LEVEL SECURITY;
ALTER TABLE compliance_requirements ENABLE ROW LEVEL SECURITY;
ALTER TABLE compliance_verifications ENABLE ROW LEVEL SECURITY;
ALTER TABLE legal_notices ENABLE ROW LEVEL SECURITY;
ALTER TABLE legal_notice_recipients ENABLE ROW LEVEL SECURITY;
ALTER TABLE compliance_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE compliance_audit_trail ENABLE ROW LEVEL SECURITY;

-- Legal documents policies
CREATE POLICY legal_documents_tenant_isolation ON legal_documents
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY legal_versions_tenant_isolation ON legal_document_versions
    USING (document_id IN (
        SELECT id FROM legal_documents
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

CREATE POLICY compliance_req_tenant_isolation ON compliance_requirements
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY compliance_verifications_tenant_isolation ON compliance_verifications
    USING (requirement_id IN (
        SELECT id FROM compliance_requirements
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

CREATE POLICY legal_notices_tenant_isolation ON legal_notices
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY notice_recipients_tenant_isolation ON legal_notice_recipients
    USING (notice_id IN (
        SELECT id FROM legal_notices
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

CREATE POLICY compliance_templates_tenant_isolation ON compliance_templates
    USING (organization_id IS NULL OR organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY compliance_audit_tenant_isolation ON compliance_audit_trail
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

-- ===========================================
-- Triggers
-- ===========================================
CREATE TRIGGER set_legal_documents_updated_at
    BEFORE UPDATE ON legal_documents
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_compliance_requirements_updated_at
    BEFORE UPDATE ON compliance_requirements
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_legal_notices_updated_at
    BEFORE UPDATE ON legal_notices
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_compliance_templates_updated_at
    BEFORE UPDATE ON compliance_templates
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ===========================================
-- Function to update compliance status based on due date
-- ===========================================
CREATE OR REPLACE FUNCTION update_compliance_status()
RETURNS TRIGGER AS $$
BEGIN
    -- If due date passed and status is not compliant, mark as expired
    IF NEW.next_due_date < CURRENT_DATE AND NEW.status = 'pending' THEN
        NEW.status := 'expired';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_update_compliance_status
BEFORE UPDATE ON compliance_requirements
FOR EACH ROW EXECUTE FUNCTION update_compliance_status();
