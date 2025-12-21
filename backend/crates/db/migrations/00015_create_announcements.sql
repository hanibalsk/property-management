-- Epic 6: Announcements & Communication
-- Creates announcements, announcement_attachments tables with RLS

-- Announcement status
CREATE TYPE announcement_status AS ENUM (
    'draft',
    'scheduled',
    'published',
    'archived'
);

-- Target type (who receives the announcement)
CREATE TYPE announcement_target_type AS ENUM (
    'all',          -- All users in organization
    'building',     -- Specific building(s)
    'units',        -- Specific unit(s)
    'roles'         -- Specific role(s)
);

-- ============================================================================
-- Announcements table
-- ============================================================================

CREATE TABLE IF NOT EXISTS announcements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Context
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Author
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Content
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,  -- Markdown content

    -- Targeting
    target_type announcement_target_type NOT NULL DEFAULT 'all',
    target_ids JSONB NOT NULL DEFAULT '[]',
    -- Format for 'building': ["building_uuid_1", "building_uuid_2"]
    -- Format for 'units': ["unit_uuid_1", "unit_uuid_2"]
    -- Format for 'roles': ["manager", "owner", "resident"]

    -- Status and timing
    status announcement_status NOT NULL DEFAULT 'draft',
    scheduled_at TIMESTAMPTZ,  -- When to auto-publish
    published_at TIMESTAMPTZ,  -- When actually published

    -- Features
    pinned BOOLEAN NOT NULL DEFAULT FALSE,
    pinned_at TIMESTAMPTZ,
    pinned_by UUID REFERENCES users(id) ON DELETE SET NULL,
    comments_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    acknowledgment_required BOOLEAN NOT NULL DEFAULT FALSE,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for announcements
CREATE INDEX IF NOT EXISTS idx_announcements_organization_id ON announcements(organization_id);
CREATE INDEX IF NOT EXISTS idx_announcements_author_id ON announcements(author_id);
CREATE INDEX IF NOT EXISTS idx_announcements_status ON announcements(status);
CREATE INDEX IF NOT EXISTS idx_announcements_scheduled_at ON announcements(scheduled_at) WHERE status = 'scheduled';
CREATE INDEX IF NOT EXISTS idx_announcements_published_at ON announcements(published_at);
CREATE INDEX IF NOT EXISTS idx_announcements_pinned ON announcements(organization_id, pinned) WHERE pinned = TRUE;
CREATE INDEX IF NOT EXISTS idx_announcements_target_type ON announcements(target_type);

-- Trigger for updated_at
CREATE TRIGGER update_announcements_updated_at
    BEFORE UPDATE ON announcements
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Enable Row Level Security
ALTER TABLE announcements ENABLE ROW LEVEL SECURITY;

-- RLS Policy
CREATE POLICY announcements_tenant_isolation ON announcements
    FOR ALL
    USING (
        is_super_admin()
        OR organization_id = get_current_org_id()
    )
    WITH CHECK (
        is_super_admin()
        OR organization_id = get_current_org_id()
    );

-- ============================================================================
-- Announcement Attachments table (for embedded images/files)
-- ============================================================================

CREATE TABLE IF NOT EXISTS announcement_attachments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    announcement_id UUID NOT NULL REFERENCES announcements(id) ON DELETE CASCADE,

    -- File info
    file_key VARCHAR(512) NOT NULL,  -- S3 path
    file_name VARCHAR(255) NOT NULL,
    file_type VARCHAR(100) NOT NULL,  -- MIME type
    file_size BIGINT NOT NULL,        -- Size in bytes

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for announcement_attachments
CREATE INDEX IF NOT EXISTS idx_announcement_attachments_announcement_id ON announcement_attachments(announcement_id);

-- Enable Row Level Security
ALTER TABLE announcement_attachments ENABLE ROW LEVEL SECURITY;

-- RLS Policy: Inherit from announcements
CREATE POLICY announcement_attachments_tenant_isolation ON announcement_attachments
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM announcements a
            WHERE a.id = announcement_attachments.announcement_id
            AND a.organization_id = get_current_org_id()
        )
    )
    WITH CHECK (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM announcements a
            WHERE a.id = announcement_attachments.announcement_id
            AND a.organization_id = get_current_org_id()
        )
    );

-- ============================================================================
-- Announcement Reads table (for tracking read/acknowledged status)
-- Foundation for Story 6.2, created now for completeness
-- ============================================================================

CREATE TABLE IF NOT EXISTS announcement_reads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    announcement_id UUID NOT NULL REFERENCES announcements(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Read status
    read_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Acknowledgment (optional, for announcements requiring acknowledgment)
    acknowledged_at TIMESTAMPTZ,

    -- One read record per user per announcement
    CONSTRAINT unique_announcement_read UNIQUE (announcement_id, user_id)
);

-- Indexes for announcement_reads
CREATE INDEX IF NOT EXISTS idx_announcement_reads_announcement_id ON announcement_reads(announcement_id);
CREATE INDEX IF NOT EXISTS idx_announcement_reads_user_id ON announcement_reads(user_id);

-- Enable Row Level Security
ALTER TABLE announcement_reads ENABLE ROW LEVEL SECURITY;

-- RLS Policy: Users can see their own reads, managers can see all for their org
CREATE POLICY announcement_reads_tenant_isolation ON announcement_reads
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM announcements a
            WHERE a.id = announcement_reads.announcement_id
            AND a.organization_id = get_current_org_id()
        )
    )
    WITH CHECK (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM announcements a
            WHERE a.id = announcement_reads.announcement_id
            AND a.organization_id = get_current_org_id()
        )
    );

-- ============================================================================
-- Comments
-- ============================================================================

COMMENT ON TABLE announcements IS 'Organization announcements (Epic 6, UC-06)';
COMMENT ON TABLE announcement_attachments IS 'Attachments/images for announcements';
COMMENT ON TABLE announcement_reads IS 'Tracks which users have read/acknowledged announcements';

COMMENT ON COLUMN announcements.content IS 'Markdown formatted content';
COMMENT ON COLUMN announcements.target_ids IS 'JSON array of UUIDs matching target_type';
COMMENT ON COLUMN announcement_attachments.file_key IS 'S3 object key for file storage';
