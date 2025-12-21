-- Epic 6: Announcements & Communication - Story 6.3
-- Creates announcement_comments table for threaded discussions

-- ============================================================================
-- Announcement Comments table
-- ============================================================================

CREATE TABLE IF NOT EXISTS announcement_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    announcement_id UUID NOT NULL REFERENCES announcements(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Threading: NULL = top-level comment, set = reply to another comment
    parent_id UUID REFERENCES announcement_comments(id) ON DELETE CASCADE,

    -- Content
    content TEXT NOT NULL,

    -- AI training consent (per GDPR/privacy requirements)
    ai_training_consent BOOLEAN NOT NULL DEFAULT FALSE,

    -- Soft delete for moderation (preserves thread integrity)
    deleted_at TIMESTAMPTZ,
    deleted_by UUID REFERENCES users(id) ON DELETE SET NULL,
    deletion_reason VARCHAR(255),

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for announcement_comments
CREATE INDEX IF NOT EXISTS idx_announcement_comments_announcement_id ON announcement_comments(announcement_id);
CREATE INDEX IF NOT EXISTS idx_announcement_comments_user_id ON announcement_comments(user_id);
CREATE INDEX IF NOT EXISTS idx_announcement_comments_parent_id ON announcement_comments(parent_id);
CREATE INDEX IF NOT EXISTS idx_announcement_comments_created_at ON announcement_comments(created_at);
CREATE INDEX IF NOT EXISTS idx_announcement_comments_not_deleted ON announcement_comments(announcement_id, created_at)
    WHERE deleted_at IS NULL;

-- Trigger for updated_at
CREATE TRIGGER update_announcement_comments_updated_at
    BEFORE UPDATE ON announcement_comments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Enable Row Level Security
ALTER TABLE announcement_comments ENABLE ROW LEVEL SECURITY;

-- RLS Policy: Inherit tenant isolation from announcements
CREATE POLICY announcement_comments_tenant_isolation ON announcement_comments
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM announcements a
            WHERE a.id = announcement_comments.announcement_id
            AND a.organization_id = get_current_org_id()
        )
    )
    WITH CHECK (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM announcements a
            WHERE a.id = announcement_comments.announcement_id
            AND a.organization_id = get_current_org_id()
        )
    );

-- ============================================================================
-- Comments
-- ============================================================================

COMMENT ON TABLE announcement_comments IS 'Threaded comments on announcements (Epic 6, Story 6.3)';
COMMENT ON COLUMN announcement_comments.parent_id IS 'NULL = top-level comment, UUID = reply to parent comment';
COMMENT ON COLUMN announcement_comments.ai_training_consent IS 'User consent for AI training use (default false)';
COMMENT ON COLUMN announcement_comments.deleted_at IS 'Soft delete timestamp for moderation';
COMMENT ON COLUMN announcement_comments.deletion_reason IS 'Moderation reason when deleted by manager';
