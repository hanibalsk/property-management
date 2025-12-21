-- Epic 10B, Story 10B.4: System Announcements
-- Migration: Create system announcements infrastructure

-- Create system_announcements table
CREATE TABLE IF NOT EXISTS system_announcements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    severity VARCHAR(20) NOT NULL CHECK (severity IN ('info', 'warning', 'critical')),
    start_at TIMESTAMPTZ NOT NULL,
    end_at TIMESTAMPTZ,
    is_dismissible BOOLEAN NOT NULL DEFAULT true,
    requires_acknowledgment BOOLEAN NOT NULL DEFAULT false,
    is_deleted BOOLEAN NOT NULL DEFAULT false,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create system_announcement_acknowledgments table
CREATE TABLE IF NOT EXISTS system_announcement_acknowledgments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    announcement_id UUID NOT NULL REFERENCES system_announcements(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    acknowledged_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(announcement_id, user_id)
);

-- Create scheduled_maintenance table
CREATE TABLE IF NOT EXISTS scheduled_maintenance (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    start_at TIMESTAMPTZ NOT NULL,
    end_at TIMESTAMPTZ NOT NULL,
    is_read_only_mode BOOLEAN NOT NULL DEFAULT false,
    announcement_id UUID REFERENCES system_announcements(id) ON DELETE SET NULL,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_system_announcements_active
    ON system_announcements(start_at, end_at)
    WHERE is_deleted = false;

CREATE INDEX IF NOT EXISTS idx_system_announcements_severity
    ON system_announcements(severity)
    WHERE is_deleted = false;

CREATE INDEX IF NOT EXISTS idx_announcement_acknowledgments_user
    ON system_announcement_acknowledgments(user_id, announcement_id);

CREATE INDEX IF NOT EXISTS idx_scheduled_maintenance_upcoming
    ON scheduled_maintenance(start_at)
    WHERE start_at > NOW();

-- Add comments for documentation
COMMENT ON TABLE system_announcements IS 'Platform-wide system announcements (Epic 10B, Story 10B.4)';
COMMENT ON TABLE system_announcement_acknowledgments IS 'User acknowledgments of critical announcements';
COMMENT ON TABLE scheduled_maintenance IS 'Scheduled maintenance windows with optional read-only mode';
COMMENT ON COLUMN system_announcements.severity IS 'info, warning, or critical';
COMMENT ON COLUMN system_announcements.requires_acknowledgment IS 'If true, users must acknowledge before using app';
COMMENT ON COLUMN scheduled_maintenance.is_read_only_mode IS 'If true, platform enters read-only mode during maintenance';
