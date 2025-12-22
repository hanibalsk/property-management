-- Migration: Create E-Signature Request Tables (Story 7B.3)
--
-- This migration adds support for electronic signature workflows:
-- - signature_requests: Tracks signature requests for documents
-- - signature_request_status: Enum for request lifecycle states
-- - signers stored as JSONB with individual status tracking

-- Create signature request status enum
CREATE TYPE signature_request_status AS ENUM (
    'pending',      -- Request created, waiting for signers
    'in_progress',  -- At least one signer has signed
    'completed',    -- All signers have signed
    'declined',     -- A signer declined to sign
    'expired',      -- Request expired before completion
    'cancelled'     -- Request was cancelled by requester
);

-- Create signature requests table
CREATE TABLE IF NOT EXISTS signature_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Request details
    status signature_request_status NOT NULL DEFAULT 'pending',
    subject TEXT,                           -- Email subject/title for the request
    message TEXT,                           -- Message to signers

    -- Signers as JSONB array
    -- Each signer: { email, name, order, status, signed_at, declined_at, declined_reason }
    signers JSONB NOT NULL DEFAULT '[]'::jsonb,

    -- External provider integration
    provider TEXT,                          -- e.g., 'docusign', 'hellosign', 'internal'
    provider_request_id TEXT,               -- External provider's request ID
    provider_metadata JSONB,                -- Additional provider-specific data

    -- Signed document (when completed)
    signed_document_id UUID REFERENCES documents(id) ON DELETE SET NULL,

    -- Audit fields
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at TIMESTAMPTZ,                 -- When the request expires
    completed_at TIMESTAMPTZ,               -- When all signers completed
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Create indexes for common queries
CREATE INDEX idx_signature_requests_document_id ON signature_requests(document_id);
CREATE INDEX idx_signature_requests_organization_id ON signature_requests(organization_id);
CREATE INDEX idx_signature_requests_status ON signature_requests(status);
CREATE INDEX idx_signature_requests_created_by ON signature_requests(created_by);
CREATE INDEX idx_signature_requests_expires_at ON signature_requests(expires_at) WHERE expires_at IS NOT NULL;
CREATE INDEX idx_signature_requests_provider ON signature_requests(provider, provider_request_id) WHERE provider_request_id IS NOT NULL;

-- GIN index for JSONB signer email search
CREATE INDEX idx_signature_requests_signers ON signature_requests USING GIN (signers);

-- Add updated_at trigger
CREATE TRIGGER signature_requests_updated_at
    BEFORE UPDATE ON signature_requests
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Enable RLS
ALTER TABLE signature_requests ENABLE ROW LEVEL SECURITY;

-- RLS Policies
-- Users can view signature requests they created or where they are a signer
CREATE POLICY signature_requests_select_policy ON signature_requests
    FOR SELECT
    USING (
        created_by = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        OR EXISTS (
            SELECT 1 FROM jsonb_array_elements(signers) AS signer
            WHERE signer->>'email' = (SELECT email FROM users WHERE id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID)
        )
        OR EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = signature_requests.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role IN ('admin', 'manager')
        )
        OR is_super_admin()
    );

-- Only org admins/managers or the creator can insert
CREATE POLICY signature_requests_insert_policy ON signature_requests
    FOR INSERT
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = signature_requests.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role IN ('admin', 'manager')
        )
        OR is_super_admin()
    );

-- Only creator or org admins can update
CREATE POLICY signature_requests_update_policy ON signature_requests
    FOR UPDATE
    USING (
        created_by = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        OR EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = signature_requests.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role = 'admin'
        )
        OR is_super_admin()
    );

-- Only creator or org admins can delete
CREATE POLICY signature_requests_delete_policy ON signature_requests
    FOR DELETE
    USING (
        created_by = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        OR EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = signature_requests.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role = 'admin'
        )
        OR is_super_admin()
    );

-- Add signature_status column to documents to track if signed
ALTER TABLE documents ADD COLUMN IF NOT EXISTS signature_status TEXT CHECK (
    signature_status IS NULL OR signature_status IN ('none', 'pending', 'partial', 'signed')
);
ALTER TABLE documents ADD COLUMN IF NOT EXISTS signature_request_id UUID REFERENCES signature_requests(id) ON DELETE SET NULL;

-- Create index for documents with signature status
CREATE INDEX idx_documents_signature_status ON documents(signature_status) WHERE signature_status IS NOT NULL;

-- Add comments
COMMENT ON TABLE signature_requests IS 'E-signature requests for documents (Story 7B.3)';
COMMENT ON COLUMN signature_requests.signers IS 'JSONB array of signers: [{email, name, order, status, signed_at, declined_at, declined_reason}]';
COMMENT ON COLUMN signature_requests.provider IS 'E-signature provider: docusign, hellosign, internal';
COMMENT ON COLUMN signature_requests.provider_request_id IS 'External provider request/envelope ID for webhook correlation';
COMMENT ON COLUMN documents.signature_status IS 'Signature status: none, pending, partial, signed';
