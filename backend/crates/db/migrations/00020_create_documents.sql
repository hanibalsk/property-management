-- Epic 7A: Basic Document Management
-- Story 7A.1: Document Upload with Metadata
-- Migration: Create documents and document_folders tables

-- ============================================================================
-- Document Categories Enum
-- ============================================================================

CREATE TYPE document_category AS ENUM (
    'contracts',
    'invoices',
    'reports',
    'manuals',
    'certificates',
    'other'
);

-- ============================================================================
-- Access Scope Enum (Story 7A.3)
-- ============================================================================

CREATE TYPE document_access_scope AS ENUM (
    'organization',
    'building',
    'unit',
    'role',
    'users'
);

-- ============================================================================
-- Document Folders Table (Story 7A.2)
-- ============================================================================

CREATE TABLE document_folders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES document_folders(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,

    -- Ensure unique folder names within same parent
    CONSTRAINT uq_folder_name_in_parent UNIQUE NULLS NOT DISTINCT (organization_id, parent_id, name, deleted_at)
);

-- Index for folder queries
CREATE INDEX idx_document_folders_org_parent ON document_folders(organization_id, parent_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_document_folders_name ON document_folders(organization_id, name) WHERE deleted_at IS NULL;

-- ============================================================================
-- Documents Table
-- ============================================================================

CREATE TABLE documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    folder_id UUID REFERENCES document_folders(id) ON DELETE SET NULL,

    -- Document metadata
    title VARCHAR(500) NOT NULL,
    description TEXT,
    category document_category NOT NULL DEFAULT 'other',

    -- File information
    file_key VARCHAR(1024) NOT NULL, -- S3 path
    file_name VARCHAR(500) NOT NULL,
    mime_type VARCHAR(255) NOT NULL,
    size_bytes BIGINT NOT NULL,

    -- Access control (Story 7A.3)
    access_scope document_access_scope NOT NULL DEFAULT 'organization',
    access_target_ids JSONB DEFAULT '[]'::jsonb, -- Array of building/unit/user UUIDs
    access_roles JSONB DEFAULT '[]'::jsonb, -- Array of role strings

    -- Audit
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

-- Indexes for document queries
CREATE INDEX idx_documents_org_folder ON documents(organization_id, folder_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_documents_category ON documents(organization_id, category) WHERE deleted_at IS NULL;
CREATE INDEX idx_documents_created_by ON documents(organization_id, created_by) WHERE deleted_at IS NULL;
CREATE INDEX idx_documents_access_scope ON documents(organization_id, access_scope) WHERE deleted_at IS NULL;

-- GIN index for JSONB access target queries
CREATE INDEX idx_documents_access_targets ON documents USING gin(access_target_ids) WHERE deleted_at IS NULL;
CREATE INDEX idx_documents_access_roles ON documents USING gin(access_roles) WHERE deleted_at IS NULL;

-- ============================================================================
-- Document Shares Table (Story 7A.5)
-- ============================================================================

CREATE TYPE document_share_type AS ENUM (
    'user',
    'role',
    'building',
    'link'
);

CREATE TABLE document_shares (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    share_type document_share_type NOT NULL,
    target_id UUID, -- user_id or building_id (null for link shares)
    target_role VARCHAR(50), -- Role name for role shares
    shared_by UUID NOT NULL REFERENCES users(id),

    -- Link share specific
    share_token VARCHAR(64) UNIQUE,
    password_hash VARCHAR(255), -- Optional password protection

    -- Expiration
    expires_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for share queries
CREATE INDEX idx_document_shares_document ON document_shares(document_id) WHERE revoked_at IS NULL;
CREATE INDEX idx_document_shares_token ON document_shares(share_token) WHERE revoked_at IS NULL AND share_token IS NOT NULL;
CREATE INDEX idx_document_shares_target ON document_shares(target_id) WHERE revoked_at IS NULL AND target_id IS NOT NULL;

-- ============================================================================
-- Document Share Access Log (Story 7A.5)
-- ============================================================================

CREATE TABLE document_share_access_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    share_id UUID NOT NULL REFERENCES document_shares(id) ON DELETE CASCADE,
    accessed_by UUID REFERENCES users(id), -- Null for anonymous link access
    accessed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ip_address INET
);

CREATE INDEX idx_share_access_log_share ON document_share_access_log(share_id);
CREATE INDEX idx_share_access_log_time ON document_share_access_log(accessed_at);

-- ============================================================================
-- Row Level Security Policies
-- ============================================================================

ALTER TABLE document_folders ENABLE ROW LEVEL SECURITY;
ALTER TABLE documents ENABLE ROW LEVEL SECURITY;
ALTER TABLE document_shares ENABLE ROW LEVEL SECURITY;
ALTER TABLE document_share_access_log ENABLE ROW LEVEL SECURITY;

-- Folder policies: users can see folders in their organization
CREATE POLICY folder_tenant_isolation ON document_folders
    FOR ALL
    USING (organization_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (organization_id = current_setting('app.tenant_id', true)::uuid);

-- Document policies: users can see documents they have access to
-- Note: This is a basic tenant isolation policy; application-level checks handle access_scope
CREATE POLICY document_tenant_isolation ON documents
    FOR ALL
    USING (organization_id = current_setting('app.tenant_id', true)::uuid)
    WITH CHECK (organization_id = current_setting('app.tenant_id', true)::uuid);

-- Share policies: users can see shares for documents they can access
CREATE POLICY share_tenant_isolation ON document_shares
    FOR ALL
    USING (
        document_id IN (
            SELECT id FROM documents
            WHERE organization_id = current_setting('app.tenant_id', true)::uuid
        )
    );

-- Access log policies: inherit from share
CREATE POLICY access_log_tenant_isolation ON document_share_access_log
    FOR ALL
    USING (
        share_id IN (
            SELECT ds.id FROM document_shares ds
            JOIN documents d ON d.id = ds.document_id
            WHERE d.organization_id = current_setting('app.tenant_id', true)::uuid
        )
    );

-- ============================================================================
-- Trigger for updated_at
-- ============================================================================

CREATE TRIGGER update_documents_updated_at
    BEFORE UPDATE ON documents
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_document_folders_updated_at
    BEFORE UPDATE ON document_folders
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- Function to check folder depth (max 5 levels)
-- ============================================================================

CREATE OR REPLACE FUNCTION check_folder_depth()
RETURNS TRIGGER AS $$
DECLARE
    depth INTEGER := 0;
    current_parent UUID := NEW.parent_id;
BEGIN
    WHILE current_parent IS NOT NULL AND depth < 6 LOOP
        SELECT parent_id INTO current_parent
        FROM document_folders
        WHERE id = current_parent;
        depth := depth + 1;
    END LOOP;

    IF depth >= 5 THEN
        RAISE EXCEPTION 'Maximum folder depth of 5 exceeded';
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_folder_depth_trigger
    BEFORE INSERT OR UPDATE ON document_folders
    FOR EACH ROW
    WHEN (NEW.parent_id IS NOT NULL)
    EXECUTE FUNCTION check_folder_depth();

-- ============================================================================
-- Comments
-- ============================================================================

COMMENT ON TABLE document_folders IS 'Hierarchical folder structure for document organization (Epic 7A)';
COMMENT ON TABLE documents IS 'Document metadata and access control (Epic 7A)';
COMMENT ON TABLE document_shares IS 'Document sharing configuration (Epic 7A Story 7A.5)';
COMMENT ON TABLE document_share_access_log IS 'Audit log for document share access (Epic 7A Story 7A.5)';
COMMENT ON COLUMN documents.access_scope IS 'Determines who can access: organization (all), building, unit, role, or specific users';
COMMENT ON COLUMN documents.access_target_ids IS 'JSONB array of UUIDs - building/unit/user IDs based on access_scope';
COMMENT ON COLUMN documents.access_roles IS 'JSONB array of role strings for role-based access';
