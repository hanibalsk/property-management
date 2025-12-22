-- Migration: 00036_add_document_templates
-- Epic 7B / Story 7B.2: Document Templates & Generation
--
-- Adds template support for standardized document generation:
-- - document_templates: stores template definitions with placeholders
-- - Placeholder syntax uses Mustache-style {{placeholder_name}}

-- ============================================================================
-- ENUM: Template Type
-- ============================================================================

CREATE TYPE document_template_type AS ENUM (
    'lease',       -- Rental/lease agreements
    'notice',      -- Official notices (rent increase, termination, etc.)
    'invoice',     -- Invoice templates
    'report',      -- Periodic reports
    'minutes',     -- Meeting minutes
    'contract',    -- General contracts
    'custom'       -- User-defined templates
);

-- ============================================================================
-- TABLE: Document Templates
-- ============================================================================

CREATE TABLE document_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id),

    -- Template metadata
    name VARCHAR(255) NOT NULL,
    description TEXT,
    template_type document_template_type NOT NULL DEFAULT 'custom',

    -- Template content (markdown with {{placeholder}} syntax)
    content TEXT NOT NULL,

    -- Placeholder definitions as JSONB array
    -- Each placeholder: { name: string, type: 'text'|'date'|'number'|'currency', required: bool, default_value?: string, description?: string }
    placeholders JSONB NOT NULL DEFAULT '[]',

    -- Usage tracking
    usage_count INTEGER NOT NULL DEFAULT 0,

    -- Audit fields
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,

    -- Constraints
    CONSTRAINT uq_template_name_per_org UNIQUE (organization_id, name, deleted_at)
);

-- ============================================================================
-- INDEXES
-- ============================================================================

-- Find templates by organization
CREATE INDEX idx_document_templates_org ON document_templates(organization_id) WHERE deleted_at IS NULL;

-- Find templates by type
CREATE INDEX idx_document_templates_type ON document_templates(template_type) WHERE deleted_at IS NULL;

-- Search templates by name
CREATE INDEX idx_document_templates_name ON document_templates USING gin(to_tsvector('english', name)) WHERE deleted_at IS NULL;

-- ============================================================================
-- TABLE: Generated Documents (Links)
-- ============================================================================

-- Add template reference to documents table
ALTER TABLE documents ADD COLUMN template_id UUID REFERENCES document_templates(id);
ALTER TABLE documents ADD COLUMN generation_metadata JSONB;

-- Index for finding documents generated from a template
CREATE INDEX idx_documents_template ON documents(template_id) WHERE template_id IS NOT NULL AND deleted_at IS NULL;

-- ============================================================================
-- TRIGGER: Update usage count on document generation
-- ============================================================================

CREATE OR REPLACE FUNCTION update_template_usage_count()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.template_id IS NOT NULL AND (TG_OP = 'INSERT' OR OLD.template_id IS DISTINCT FROM NEW.template_id) THEN
        UPDATE document_templates
        SET usage_count = usage_count + 1
        WHERE id = NEW.template_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_update_template_usage
    AFTER INSERT OR UPDATE OF template_id ON documents
    FOR EACH ROW
    EXECUTE FUNCTION update_template_usage_count();

-- ============================================================================
-- RLS POLICIES
-- ============================================================================

ALTER TABLE document_templates ENABLE ROW LEVEL SECURITY;

-- Read: members of the organization can read templates
CREATE POLICY document_templates_read ON document_templates
    FOR SELECT
    USING (
        organization_id::text = current_setting('app.tenant_id', true)
        OR organization_id IN (
            SELECT om.organization_id
            FROM organization_members om
            WHERE om.user_id = current_setting('request.user_id', true)::uuid
            AND om.status = 'active'
        )
    );

-- Write: only managers can create/update templates
CREATE POLICY document_templates_write ON document_templates
    FOR ALL
    USING (
        organization_id::text = current_setting('app.tenant_id', true)
    );

-- ============================================================================
-- COMMENTS
-- ============================================================================

COMMENT ON TABLE document_templates IS 'Stores document templates with placeholder definitions (Epic 7B, Story 7B.2)';
COMMENT ON COLUMN document_templates.content IS 'Template content in markdown with {{placeholder}} syntax';
COMMENT ON COLUMN document_templates.placeholders IS 'JSONB array of placeholder definitions: [{name, type, required, default_value, description}]';
COMMENT ON COLUMN document_templates.usage_count IS 'Number of documents generated from this template';
COMMENT ON COLUMN documents.template_id IS 'Reference to template if document was generated from one';
COMMENT ON COLUMN documents.generation_metadata IS 'JSONB storing placeholder values used during generation';
