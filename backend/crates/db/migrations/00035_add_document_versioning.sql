-- Migration: 00035_add_document_versioning
-- Epic 7B / Story 7B.1: Document Versioning & History
--
-- Adds versioning support to documents:
-- - version_number: starts at 1, increments on new version upload
-- - parent_document_id: links to the original document (version chain root)
-- - is_current_version: identifies the latest/active version

-- ============================================================================
-- ADD VERSION COLUMNS TO DOCUMENTS TABLE
-- ============================================================================

-- Version number (1-based, increments with each new version)
ALTER TABLE documents ADD COLUMN version_number INTEGER NOT NULL DEFAULT 1;

-- Parent document ID: points to the original document (root of version chain)
-- NULL means this is the original document (first version)
ALTER TABLE documents ADD COLUMN parent_document_id UUID REFERENCES documents(id) ON DELETE SET NULL;

-- Flag to identify the current (latest) version
ALTER TABLE documents ADD COLUMN is_current_version BOOLEAN NOT NULL DEFAULT true;

-- ============================================================================
-- INDEXES
-- ============================================================================

-- Index for finding version history of a document
CREATE INDEX idx_documents_parent_document ON documents(parent_document_id) WHERE deleted_at IS NULL;

-- Index for finding current version quickly
CREATE INDEX idx_documents_current_version ON documents(id) WHERE is_current_version = true AND deleted_at IS NULL;

-- Composite index for version chain queries
CREATE INDEX idx_documents_version_chain ON documents(COALESCE(parent_document_id, id), version_number DESC) WHERE deleted_at IS NULL;

-- ============================================================================
-- VIEW: Document Version History
-- ============================================================================

-- This view provides a convenient way to query document version history
CREATE OR REPLACE VIEW document_versions AS
SELECT
    d.id,
    d.organization_id,
    COALESCE(d.parent_document_id, d.id) AS root_document_id,
    d.title,
    d.description,
    d.category,
    d.file_key,
    d.file_name,
    d.mime_type,
    d.size_bytes,
    d.version_number,
    d.is_current_version,
    d.created_by,
    d.created_at,
    u.email AS created_by_email,
    u.name AS created_by_name
FROM documents d
JOIN users u ON u.id = d.created_by
WHERE d.deleted_at IS NULL
ORDER BY d.version_number DESC;

COMMENT ON VIEW document_versions IS 'Document version history with uploader information (Epic 7B, Story 7B.1)';

-- ============================================================================
-- FUNCTION: Get next version number for a document
-- ============================================================================

CREATE OR REPLACE FUNCTION get_next_document_version(doc_id UUID)
RETURNS INTEGER AS $$
DECLARE
    max_version INTEGER;
    root_id UUID;
BEGIN
    -- Find the root document ID (either parent_document_id or the doc itself)
    SELECT COALESCE(parent_document_id, id) INTO root_id
    FROM documents
    WHERE id = doc_id;

    -- Get the max version number in the chain
    SELECT COALESCE(MAX(version_number), 0) INTO max_version
    FROM documents
    WHERE (parent_document_id = root_id OR id = root_id)
      AND deleted_at IS NULL;

    RETURN max_version + 1;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION get_next_document_version IS 'Returns the next version number for a document version chain';

-- ============================================================================
-- TRIGGER: Ensure only one current version per document chain
-- ============================================================================

CREATE OR REPLACE FUNCTION ensure_single_current_version()
RETURNS TRIGGER AS $$
BEGIN
    -- If this is marked as current version, unmark all others in the chain
    IF NEW.is_current_version = true THEN
        UPDATE documents
        SET is_current_version = false
        WHERE (
            id = NEW.parent_document_id
            OR parent_document_id = NEW.parent_document_id
            OR parent_document_id = NEW.id
            OR id = (SELECT parent_document_id FROM documents WHERE id = NEW.id)
        )
        AND id != NEW.id
        AND deleted_at IS NULL;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_ensure_single_current_version
    AFTER INSERT OR UPDATE OF is_current_version ON documents
    FOR EACH ROW
    WHEN (NEW.is_current_version = true)
    EXECUTE FUNCTION ensure_single_current_version();

-- ============================================================================
-- COMMENTS
-- ============================================================================

COMMENT ON COLUMN documents.version_number IS 'Version number (1-based), increments with each new version upload';
COMMENT ON COLUMN documents.parent_document_id IS 'Points to the original document in a version chain (NULL = original)';
COMMENT ON COLUMN documents.is_current_version IS 'True if this is the current/active version of the document';
