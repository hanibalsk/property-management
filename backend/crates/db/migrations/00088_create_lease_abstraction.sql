-- Epic 133: AI Lease Abstraction & Document Intelligence
-- Provides automated extraction of key terms from lease documents

-- =============================================================================
-- LEASE DOCUMENT PROCESSING (Story 133.1)
-- =============================================================================

-- Documents uploaded for AI processing
CREATE TABLE IF NOT EXISTS lease_documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    uploaded_by UUID NOT NULL REFERENCES users(id),

    -- File information
    file_name VARCHAR(255) NOT NULL,
    file_size_bytes INTEGER NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    storage_path VARCHAR(500) NOT NULL,

    -- Processing status
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    -- pending, processing, completed, failed, review_required

    -- Optional unit association (if known before extraction)
    unit_id UUID REFERENCES units(id) ON DELETE SET NULL,

    -- Processing metadata
    processing_started_at TIMESTAMPTZ,
    processing_completed_at TIMESTAMPTZ,
    error_message TEXT,
    page_count INTEGER,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT valid_status CHECK (status IN ('pending', 'processing', 'completed', 'failed', 'review_required')),
    CONSTRAINT valid_file_size CHECK (file_size_bytes > 0 AND file_size_bytes <= 26214400), -- 25MB limit
    CONSTRAINT valid_mime_type CHECK (mime_type IN ('application/pdf', 'image/jpeg', 'image/png', 'image/tiff'))
);

-- =============================================================================
-- EXTRACTION RESULTS (Story 133.2)
-- =============================================================================

-- Individual extracted fields from lease documents
CREATE TABLE IF NOT EXISTS lease_extractions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID NOT NULL REFERENCES lease_documents(id) ON DELETE CASCADE,

    -- Extraction version (allows re-extraction)
    version INTEGER NOT NULL DEFAULT 1,

    -- Extracted fields (nullable, as not all may be found)
    tenant_name VARCHAR(255),
    tenant_name_confidence DECIMAL(5,2),
    tenant_name_location JSONB, -- { "page": 1, "bbox": [x1, y1, x2, y2] }

    landlord_name VARCHAR(255),
    landlord_name_confidence DECIMAL(5,2),
    landlord_name_location JSONB,

    property_address TEXT,
    property_address_confidence DECIMAL(5,2),
    property_address_location JSONB,

    lease_start_date DATE,
    lease_start_date_confidence DECIMAL(5,2),
    lease_start_date_location JSONB,

    lease_end_date DATE,
    lease_end_date_confidence DECIMAL(5,2),
    lease_end_date_location JSONB,

    monthly_rent DECIMAL(12,2),
    monthly_rent_confidence DECIMAL(5,2),
    monthly_rent_location JSONB,
    rent_currency VARCHAR(3) DEFAULT 'EUR',

    security_deposit DECIMAL(12,2),
    security_deposit_confidence DECIMAL(5,2),
    security_deposit_location JSONB,

    payment_due_day INTEGER, -- Day of month rent is due (1-31)
    payment_due_day_confidence DECIMAL(5,2),
    payment_due_day_location JSONB,

    -- Special clauses (array of extracted clauses)
    special_clauses JSONB DEFAULT '[]',

    -- Overall extraction quality
    overall_confidence DECIMAL(5,2),
    fields_extracted INTEGER NOT NULL DEFAULT 0,
    fields_flagged INTEGER NOT NULL DEFAULT 0, -- Fields with <80% confidence

    -- AI model info
    model_used VARCHAR(100),
    extraction_duration_ms INTEGER,

    -- Review status
    review_status VARCHAR(50) NOT NULL DEFAULT 'pending',
    -- pending, approved, rejected
    reviewed_by UUID REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT valid_confidence CHECK (overall_confidence IS NULL OR (overall_confidence >= 0 AND overall_confidence <= 100)),
    CONSTRAINT valid_review_status CHECK (review_status IN ('pending', 'approved', 'rejected'))
);

-- =============================================================================
-- EXTRACTION CORRECTIONS (Story 133.3)
-- =============================================================================

-- Audit trail for corrections made to extracted data
CREATE TABLE IF NOT EXISTS extraction_corrections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    extraction_id UUID NOT NULL REFERENCES lease_extractions(id) ON DELETE CASCADE,
    corrected_by UUID NOT NULL REFERENCES users(id),

    field_name VARCHAR(100) NOT NULL,
    original_value TEXT,
    corrected_value TEXT NOT NULL,

    correction_reason VARCHAR(500),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- LEASE IMPORTS (Story 133.4)
-- =============================================================================

-- Track imports from extractions to the lease system
CREATE TABLE IF NOT EXISTS lease_imports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    extraction_id UUID NOT NULL REFERENCES lease_extractions(id) ON DELETE CASCADE,

    -- The resulting lease record
    lease_id UUID REFERENCES leases(id) ON DELETE SET NULL,

    -- Import status
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    -- pending, imported, failed, cancelled

    imported_by UUID REFERENCES users(id),
    imported_at TIMESTAMPTZ,

    -- Validation results before import
    validation_errors JSONB DEFAULT '[]',
    validation_warnings JSONB DEFAULT '[]',

    -- What was imported
    fields_imported JSONB, -- List of field names that were imported

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT valid_import_status CHECK (status IN ('pending', 'imported', 'failed', 'cancelled'))
);

-- =============================================================================
-- INDEXES
-- =============================================================================

CREATE INDEX IF NOT EXISTS idx_lease_documents_org ON lease_documents(organization_id);
CREATE INDEX IF NOT EXISTS idx_lease_documents_status ON lease_documents(status);
CREATE INDEX IF NOT EXISTS idx_lease_documents_created ON lease_documents(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_lease_documents_unit ON lease_documents(unit_id) WHERE unit_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_lease_extractions_document ON lease_extractions(document_id);
CREATE INDEX IF NOT EXISTS idx_lease_extractions_review ON lease_extractions(review_status);
CREATE INDEX IF NOT EXISTS idx_lease_extractions_flagged ON lease_extractions(fields_flagged DESC) WHERE fields_flagged > 0;

CREATE INDEX IF NOT EXISTS idx_extraction_corrections_extraction ON extraction_corrections(extraction_id);
CREATE INDEX IF NOT EXISTS idx_lease_imports_extraction ON lease_imports(extraction_id);
CREATE INDEX IF NOT EXISTS idx_lease_imports_lease ON lease_imports(lease_id) WHERE lease_id IS NOT NULL;

-- =============================================================================
-- RLS POLICIES
-- =============================================================================

ALTER TABLE lease_documents ENABLE ROW LEVEL SECURITY;
ALTER TABLE lease_extractions ENABLE ROW LEVEL SECURITY;
ALTER TABLE extraction_corrections ENABLE ROW LEVEL SECURITY;
ALTER TABLE lease_imports ENABLE ROW LEVEL SECURITY;

-- Lease documents: org-based access
CREATE POLICY lease_documents_org_policy ON lease_documents
    USING (organization_id = get_current_org_id());

-- Extractions: access via document's org
CREATE POLICY lease_extractions_org_policy ON lease_extractions
    USING (document_id IN (
        SELECT id FROM lease_documents WHERE organization_id = get_current_org_id()
    ));

-- Corrections: access via extraction's document's org
CREATE POLICY extraction_corrections_org_policy ON extraction_corrections
    USING (extraction_id IN (
        SELECT e.id FROM lease_extractions e
        JOIN lease_documents d ON d.id = e.document_id
        WHERE d.organization_id = get_current_org_id()
    ));

-- Imports: access via extraction's document's org
CREATE POLICY lease_imports_org_policy ON lease_imports
    USING (extraction_id IN (
        SELECT e.id FROM lease_extractions e
        JOIN lease_documents d ON d.id = e.document_id
        WHERE d.organization_id = get_current_org_id()
    ));

-- =============================================================================
-- TRIGGERS
-- =============================================================================

-- Auto-update updated_at on lease_documents
CREATE TRIGGER lease_documents_updated_at
    BEFORE UPDATE ON lease_documents
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
