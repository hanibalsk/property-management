-- Migration: Create Forms Management Tables
-- Epic 54: Forms Management (FR166-FR170)
-- Supports form templates, field definitions, submissions, and digital signatures

-- Form Status Enum
DO $$ BEGIN
    CREATE TYPE form_status AS ENUM ('draft', 'published', 'archived');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Form Field Type Enum
DO $$ BEGIN
    CREATE TYPE form_field_type AS ENUM (
        'text',
        'textarea',
        'number',
        'email',
        'phone',
        'date',
        'datetime',
        'checkbox',
        'radio',
        'select',
        'multiselect',
        'file',
        'signature'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Form Submission Status Enum
DO $$ BEGIN
    CREATE TYPE form_submission_status AS ENUM ('pending', 'reviewed', 'approved', 'rejected');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- ============================================================================
-- Forms Table - Main form template storage
-- ============================================================================
CREATE TABLE IF NOT EXISTS forms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE SET NULL,

    -- Form metadata
    title VARCHAR(500) NOT NULL,
    description TEXT,
    category VARCHAR(100),
    status form_status NOT NULL DEFAULT 'draft',
    version INT NOT NULL DEFAULT 1,

    -- Targeting
    target_type VARCHAR(50) DEFAULT 'all', -- 'all', 'building', 'units', 'roles'
    target_ids JSONB DEFAULT '[]'::jsonb,

    -- Configuration
    require_signatures BOOLEAN DEFAULT FALSE,
    allow_multiple_submissions BOOLEAN DEFAULT FALSE,
    submission_deadline TIMESTAMPTZ,
    confirmation_message TEXT,

    -- PDF template settings
    pdf_template_settings JSONB DEFAULT '{}'::jsonb,

    -- Audit fields
    created_by UUID NOT NULL REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    published_by UUID REFERENCES users(id),
    published_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,

    CONSTRAINT chk_forms_title_not_empty CHECK (LENGTH(TRIM(title)) > 0)
);

-- ============================================================================
-- Form Fields Table - Field definitions for each form
-- ============================================================================
CREATE TABLE IF NOT EXISTS form_fields (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    form_id UUID NOT NULL REFERENCES forms(id) ON DELETE CASCADE,

    -- Field definition
    field_key VARCHAR(100) NOT NULL,
    label VARCHAR(255) NOT NULL,
    field_type form_field_type NOT NULL,

    -- Field configuration
    required BOOLEAN DEFAULT FALSE,
    help_text TEXT,
    placeholder VARCHAR(255),
    default_value TEXT,

    -- Validation
    validation_rules JSONB DEFAULT '{}'::jsonb,
    -- Example: {"min": 0, "max": 100, "pattern": "^[A-Z].*", "minLength": 5, "maxLength": 500}

    -- Options for select/radio/checkbox fields
    options JSONB DEFAULT '[]'::jsonb,
    -- Example: [{"value": "opt1", "label": "Option 1"}, {"value": "opt2", "label": "Option 2"}]

    -- Layout
    field_order INT NOT NULL DEFAULT 0,
    width VARCHAR(20) DEFAULT 'full', -- 'full', 'half', 'third'
    section VARCHAR(100), -- Optional section grouping

    -- Conditional display
    conditional_display JSONB DEFAULT NULL,
    -- Example: {"field": "has_pet", "operator": "equals", "value": true}

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT uq_form_field_key UNIQUE (form_id, field_key),
    CONSTRAINT chk_field_key_format CHECK (field_key ~ '^[a-z][a-z0-9_]*$'),
    CONSTRAINT chk_field_order_positive CHECK (field_order >= 0)
);

-- ============================================================================
-- Form Submissions Table - User responses to forms
-- ============================================================================
CREATE TABLE IF NOT EXISTS form_submissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    form_id UUID NOT NULL REFERENCES forms(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE SET NULL,
    unit_id UUID REFERENCES units(id) ON DELETE SET NULL,

    -- Submitter info
    submitted_by UUID NOT NULL REFERENCES users(id),
    submitted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Submission data (key-value pairs matching form fields)
    data JSONB NOT NULL DEFAULT '{}'::jsonb,

    -- File attachments (references to uploaded files)
    attachments JSONB DEFAULT '[]'::jsonb,
    -- Example: [{"field_key": "id_document", "file_id": "uuid", "filename": "id.pdf"}]

    -- Digital signature data
    signature_data JSONB DEFAULT NULL,
    -- Example: {"signature_image": "base64...", "signed_at": "2024-01-01T12:00:00Z", "ip_address": "..."}

    -- Status tracking
    status form_submission_status NOT NULL DEFAULT 'pending',
    reviewed_by UUID REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    review_notes TEXT,

    -- Tracking metadata
    ip_address INET,
    user_agent TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT chk_submission_data_not_empty CHECK (data != '{}'::jsonb)
);

-- ============================================================================
-- Form Downloads Table - Track form PDF downloads
-- ============================================================================
CREATE TABLE IF NOT EXISTS form_downloads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    form_id UUID NOT NULL REFERENCES forms(id) ON DELETE CASCADE,
    downloaded_by UUID NOT NULL REFERENCES users(id),
    downloaded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ip_address INET,
    user_agent TEXT
);

-- ============================================================================
-- Indexes for Performance
-- ============================================================================

-- Forms indexes
CREATE INDEX IF NOT EXISTS idx_forms_org_status ON forms(organization_id, status) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_forms_building ON forms(building_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_forms_category ON forms(organization_id, category) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_forms_created_at ON forms(created_at DESC) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_forms_published_at ON forms(published_at DESC) WHERE published_at IS NOT NULL AND deleted_at IS NULL;

-- Form fields indexes
CREATE INDEX IF NOT EXISTS idx_form_fields_form ON form_fields(form_id);
CREATE INDEX IF NOT EXISTS idx_form_fields_order ON form_fields(form_id, field_order);

-- Form submissions indexes
CREATE INDEX IF NOT EXISTS idx_form_submissions_form ON form_submissions(form_id);
CREATE INDEX IF NOT EXISTS idx_form_submissions_user ON form_submissions(submitted_by);
CREATE INDEX IF NOT EXISTS idx_form_submissions_org ON form_submissions(organization_id);
CREATE INDEX IF NOT EXISTS idx_form_submissions_status ON form_submissions(form_id, status);
CREATE INDEX IF NOT EXISTS idx_form_submissions_created ON form_submissions(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_form_submissions_building ON form_submissions(building_id) WHERE building_id IS NOT NULL;

-- Form downloads indexes
CREATE INDEX IF NOT EXISTS idx_form_downloads_form ON form_downloads(form_id);
CREATE INDEX IF NOT EXISTS idx_form_downloads_user ON form_downloads(downloaded_by);

-- ============================================================================
-- Triggers for Updated Timestamps
-- ============================================================================

-- Trigger function (reuse if exists)
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Forms updated_at trigger
DROP TRIGGER IF EXISTS update_forms_updated_at ON forms;
CREATE TRIGGER update_forms_updated_at
    BEFORE UPDATE ON forms
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Form fields updated_at trigger
DROP TRIGGER IF EXISTS update_form_fields_updated_at ON form_fields;
CREATE TRIGGER update_form_fields_updated_at
    BEFORE UPDATE ON form_fields
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Form submissions updated_at trigger
DROP TRIGGER IF EXISTS update_form_submissions_updated_at ON form_submissions;
CREATE TRIGGER update_form_submissions_updated_at
    BEFORE UPDATE ON form_submissions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- Row Level Security (RLS)
-- ============================================================================

-- Enable RLS on all tables
ALTER TABLE forms ENABLE ROW LEVEL SECURITY;
ALTER TABLE form_fields ENABLE ROW LEVEL SECURITY;
ALTER TABLE form_submissions ENABLE ROW LEVEL SECURITY;
ALTER TABLE form_downloads ENABLE ROW LEVEL SECURITY;

-- Forms RLS policies
DROP POLICY IF EXISTS forms_org_isolation ON forms;
CREATE POLICY forms_org_isolation ON forms
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

-- Form fields RLS policies (inherit from forms)
DROP POLICY IF EXISTS form_fields_org_isolation ON form_fields;
CREATE POLICY form_fields_org_isolation ON form_fields
    USING (form_id IN (
        SELECT id FROM forms
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

-- Form submissions RLS policies
DROP POLICY IF EXISTS form_submissions_org_isolation ON form_submissions;
CREATE POLICY form_submissions_org_isolation ON form_submissions
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

-- Form downloads RLS policies
DROP POLICY IF EXISTS form_downloads_org_isolation ON form_downloads;
CREATE POLICY form_downloads_org_isolation ON form_downloads
    USING (form_id IN (
        SELECT id FROM forms
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

-- ============================================================================
-- Comments for Documentation
-- ============================================================================

COMMENT ON TABLE forms IS 'Form templates that can be created by managers and filled out by residents';
COMMENT ON TABLE form_fields IS 'Field definitions for each form template';
COMMENT ON TABLE form_submissions IS 'Submitted form responses from users';
COMMENT ON TABLE form_downloads IS 'Tracks PDF downloads of forms for analytics';

COMMENT ON COLUMN forms.target_type IS 'Determines who can see/fill the form: all, building, units, roles';
COMMENT ON COLUMN forms.target_ids IS 'JSON array of IDs (building_ids, unit_ids, role names) based on target_type';
COMMENT ON COLUMN forms.require_signatures IS 'Whether the form requires a digital signature';
COMMENT ON COLUMN forms.allow_multiple_submissions IS 'Whether users can submit the form multiple times';

COMMENT ON COLUMN form_fields.field_key IS 'Unique identifier for the field within the form, used in data JSON';
COMMENT ON COLUMN form_fields.validation_rules IS 'JSON object with validation rules: min, max, pattern, minLength, maxLength';
COMMENT ON COLUMN form_fields.options IS 'JSON array of options for select/radio/checkbox fields';
COMMENT ON COLUMN form_fields.conditional_display IS 'JSON object defining when this field should be shown';

COMMENT ON COLUMN form_submissions.data IS 'JSON object with field_key -> value pairs for all submitted fields';
COMMENT ON COLUMN form_submissions.signature_data IS 'JSON object containing digital signature image and metadata';
