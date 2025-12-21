-- Migration: 00021_create_notification_preferences
-- Epic 8A / Story 8A.1: Channel-Level Notification Toggles
--
-- Users can toggle notifications on/off per channel (push, email, in_app).
-- Preferences are user-scoped (not organization-scoped).

-- ============================================================================
-- NOTIFICATION CHANNEL ENUM
-- ============================================================================

CREATE TYPE notification_channel AS ENUM ('push', 'email', 'in_app');

-- ============================================================================
-- NOTIFICATION PREFERENCES TABLE
-- ============================================================================

CREATE TABLE notification_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    channel notification_channel NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT true,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Each user can only have one preference per channel
    CONSTRAINT uq_notification_preferences_user_channel UNIQUE (user_id, channel)
);

-- ============================================================================
-- INDEXES
-- ============================================================================

-- Fast lookup by user_id
CREATE INDEX idx_notification_preferences_user_id ON notification_preferences(user_id);

-- ============================================================================
-- ROW LEVEL SECURITY
-- ============================================================================

ALTER TABLE notification_preferences ENABLE ROW LEVEL SECURITY;

-- Users can only access their own preferences
-- This uses the request.user_id session variable set by the application
CREATE POLICY notification_preferences_user_access ON notification_preferences
    FOR ALL
    USING (user_id = COALESCE(current_setting('request.user_id', true)::UUID, '00000000-0000-0000-0000-000000000000'::UUID))
    WITH CHECK (user_id = COALESCE(current_setting('request.user_id', true)::UUID, '00000000-0000-0000-0000-000000000000'::UUID));

-- ============================================================================
-- TRIGGER: Create default preferences for new users
-- ============================================================================

CREATE OR REPLACE FUNCTION create_default_notification_preferences()
RETURNS TRIGGER AS $$
BEGIN
    -- Insert default preferences for all channels (all enabled)
    INSERT INTO notification_preferences (user_id, channel, enabled)
    VALUES
        (NEW.id, 'push', true),
        (NEW.id, 'email', true),
        (NEW.id, 'in_app', true);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_create_default_notification_preferences
    AFTER INSERT ON users
    FOR EACH ROW
    EXECUTE FUNCTION create_default_notification_preferences();

-- ============================================================================
-- BACKFILL: Create preferences for existing users
-- ============================================================================

INSERT INTO notification_preferences (user_id, channel, enabled)
SELECT u.id, c.channel, true
FROM users u
CROSS JOIN (
    SELECT 'push'::notification_channel AS channel
    UNION ALL SELECT 'email'::notification_channel
    UNION ALL SELECT 'in_app'::notification_channel
) c
ON CONFLICT (user_id, channel) DO NOTHING;

-- ============================================================================
-- COMMENTS
-- ============================================================================

COMMENT ON TABLE notification_preferences IS 'User notification preferences per channel (Epic 8A, Story 8A.1)';
COMMENT ON COLUMN notification_preferences.channel IS 'Notification channel: push, email, or in_app';
COMMENT ON COLUMN notification_preferences.enabled IS 'Whether notifications are enabled for this channel';
