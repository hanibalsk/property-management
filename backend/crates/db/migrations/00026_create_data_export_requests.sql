-- Migration: 00026_create_data_export_requests
-- Epic 9 / Story 9.3: GDPR Data Export
--
-- Creates infrastructure for GDPR Article 15 data portability requests.
-- Users can request export of all their personal data.

-- ============================================================================
-- DATA EXPORT REQUEST STATUS ENUM
-- ============================================================================

DO $$ BEGIN
    CREATE TYPE data_export_status AS ENUM (
        'pending',      -- Request submitted, awaiting processing
        'processing',   -- Export is being generated
        'ready',        -- Export file is ready for download
        'downloaded',   -- User has downloaded the export
        'expired',      -- Download link has expired (7 days)
        'failed'        -- Export generation failed
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- ============================================================================
-- DATA EXPORT REQUESTS TABLE
-- ============================================================================

CREATE TABLE IF NOT EXISTS data_export_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- User requesting the export
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Request status
    status data_export_status NOT NULL DEFAULT 'pending',

    -- Export format (json or csv)
    format VARCHAR(10) NOT NULL DEFAULT 'json' CHECK (format IN ('json', 'csv')),

    -- What data to include (null = all)
    include_categories JSONB DEFAULT NULL,

    -- File info (populated when ready)
    file_path VARCHAR(500),
    file_size_bytes BIGINT,
    file_hash VARCHAR(64),  -- SHA-256 hash for integrity

    -- Download tracking
    download_token UUID UNIQUE,  -- One-time download token
    downloaded_at TIMESTAMPTZ,
    download_count INTEGER NOT NULL DEFAULT 0,

    -- Expiration (GDPR: reasonable access window, default 7 days)
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '7 days'),

    -- Processing info
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    error_message TEXT,

    -- Audit
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- INDEXES
-- ============================================================================

-- Find pending exports to process
CREATE INDEX IF NOT EXISTS idx_data_export_status
    ON data_export_requests(status);

-- Find user's export requests
CREATE INDEX IF NOT EXISTS idx_data_export_user_id
    ON data_export_requests(user_id);

-- Find by download token
CREATE INDEX IF NOT EXISTS idx_data_export_download_token
    ON data_export_requests(download_token)
    WHERE download_token IS NOT NULL;

-- Find expired requests for cleanup
CREATE INDEX IF NOT EXISTS idx_data_export_expires_at
    ON data_export_requests(expires_at)
    WHERE status IN ('ready', 'downloaded');

-- ============================================================================
-- TRIGGERS
-- ============================================================================

-- Updated_at trigger
CREATE TRIGGER update_data_export_requests_updated_at
    BEFORE UPDATE ON data_export_requests
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- ROW LEVEL SECURITY
-- ============================================================================

ALTER TABLE data_export_requests ENABLE ROW LEVEL SECURITY;

-- Users can only see their own export requests
CREATE POLICY data_export_select_own ON data_export_requests
    FOR SELECT
    USING (user_id = current_setting('app.current_user_id', true)::uuid);

-- Users can only create export requests for themselves
CREATE POLICY data_export_insert_own ON data_export_requests
    FOR INSERT
    WITH CHECK (user_id = current_setting('app.current_user_id', true)::uuid);

-- Super admin can see all (for compliance audits)
CREATE POLICY data_export_admin_all ON data_export_requests
    FOR ALL
    USING (
        current_setting('app.current_user_role', true) = 'SUPER_ADMIN'
    );

-- ============================================================================
-- COMMENTS
-- ============================================================================

COMMENT ON TABLE data_export_requests IS 'GDPR Article 15 data portability export requests';
COMMENT ON COLUMN data_export_requests.include_categories IS 'Optional array of data categories to include: profile, activity, documents, etc.';
COMMENT ON COLUMN data_export_requests.download_token IS 'One-time secure token for downloading the export file';
COMMENT ON COLUMN data_export_requests.file_hash IS 'SHA-256 hash of export file for integrity verification';
