-- Migration: 00038_granular_notification_preferences
-- Epic 8B: Granular Notification Preferences
--
-- Stories covered:
-- - 8B.1: Per-Event Type Preferences
-- - 8B.2: Per-Channel Delivery Preferences
-- - 8B.3: Notification Schedule (Do Not Disturb)
-- - 8B.4: Role-Based Default Preferences

-- ============================================================================
-- EVENT TYPE NOTIFICATION PREFERENCES (Stories 8B.1 & 8B.2)
-- ============================================================================

-- Event type categories for notification preferences
CREATE TYPE notification_event_category AS ENUM (
    'fault',
    'vote',
    'announcement',
    'document',
    'message',
    'critical',
    'finance',
    'facility'
);

-- Stores per-event-type and per-channel preferences
CREATE TABLE event_notification_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Event identification
    event_type TEXT NOT NULL,              -- e.g., 'fault.created', 'vote.started'
    event_category notification_event_category NOT NULL,

    -- Per-channel toggles (Story 8B.2)
    push_enabled BOOLEAN NOT NULL DEFAULT true,
    email_enabled BOOLEAN NOT NULL DEFAULT true,
    in_app_enabled BOOLEAN NOT NULL DEFAULT true, -- Always shows in UI but can affect badge counts

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT uq_event_notification_pref_user_event UNIQUE (user_id, event_type)
);

CREATE INDEX idx_event_notification_preferences_user_id ON event_notification_preferences(user_id);
CREATE INDEX idx_event_notification_preferences_category ON event_notification_preferences(event_category);

-- Trigger for updated_at
CREATE TRIGGER event_notification_preferences_updated_at
    BEFORE UPDATE ON event_notification_preferences
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- RLS
ALTER TABLE event_notification_preferences ENABLE ROW LEVEL SECURITY;

CREATE POLICY event_notification_preferences_user_access ON event_notification_preferences
    FOR ALL
    USING (user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID OR is_super_admin())
    WITH CHECK (user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID OR is_super_admin());

-- ============================================================================
-- QUIET HOURS / DO NOT DISTURB (Story 8B.3)
-- ============================================================================

CREATE TABLE notification_schedule (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE UNIQUE,

    -- Quiet hours settings
    quiet_hours_enabled BOOLEAN NOT NULL DEFAULT false,
    quiet_hours_start TIME,              -- e.g., '22:00:00'
    quiet_hours_end TIME,                -- e.g., '07:00:00'
    timezone TEXT NOT NULL DEFAULT 'UTC',

    -- Weekend settings (optional different schedule)
    weekend_quiet_hours_enabled BOOLEAN NOT NULL DEFAULT false,
    weekend_quiet_hours_start TIME,
    weekend_quiet_hours_end TIME,

    -- Digest preferences (daily/weekly summary instead of individual)
    digest_enabled BOOLEAN NOT NULL DEFAULT false,
    digest_frequency TEXT CHECK (digest_frequency IN ('daily', 'weekly')),
    digest_time TIME,                    -- When to send digest
    digest_day_of_week INTEGER CHECK (digest_day_of_week BETWEEN 0 AND 6), -- 0=Sunday for weekly

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_notification_schedule_user_id ON notification_schedule(user_id);

-- Trigger for updated_at
CREATE TRIGGER notification_schedule_updated_at
    BEFORE UPDATE ON notification_schedule
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- RLS
ALTER TABLE notification_schedule ENABLE ROW LEVEL SECURITY;

CREATE POLICY notification_schedule_user_access ON notification_schedule
    FOR ALL
    USING (user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID OR is_super_admin())
    WITH CHECK (user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID OR is_super_admin());

-- ============================================================================
-- ROLE-BASED DEFAULT PREFERENCES (Story 8B.4)
-- ============================================================================

CREATE TABLE role_notification_defaults (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    role TEXT NOT NULL,                  -- e.g., 'owner', 'manager', 'tenant'

    -- Default event preferences as JSONB
    -- Format: { "fault.created": { push: true, email: true, in_app: true }, ... }
    event_preferences JSONB NOT NULL DEFAULT '{}'::jsonb,

    -- Default quiet hours settings
    default_quiet_hours_enabled BOOLEAN NOT NULL DEFAULT false,
    default_quiet_hours_start TIME,
    default_quiet_hours_end TIME,

    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT uq_role_notification_defaults_org_role UNIQUE (organization_id, role)
);

CREATE INDEX idx_role_notification_defaults_org_id ON role_notification_defaults(organization_id);

-- Trigger for updated_at
CREATE TRIGGER role_notification_defaults_updated_at
    BEFORE UPDATE ON role_notification_defaults
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- RLS
ALTER TABLE role_notification_defaults ENABLE ROW LEVEL SECURITY;

-- Admins can manage role defaults
CREATE POLICY role_notification_defaults_admin_access ON role_notification_defaults
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = role_notification_defaults.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role = 'admin'
        )
        OR is_super_admin()
    );

-- All members can read role defaults
CREATE POLICY role_notification_defaults_read_access ON role_notification_defaults
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = role_notification_defaults.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

-- ============================================================================
-- HELD NOTIFICATIONS (for quiet hours - Story 8B.3)
-- ============================================================================

CREATE TABLE held_notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Notification details
    event_type TEXT NOT NULL,
    title TEXT NOT NULL,
    body TEXT,
    data JSONB,                          -- Additional notification data
    channels TEXT[] NOT NULL,            -- Channels to deliver to when released

    -- Held status
    held_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    release_at TIMESTAMPTZ NOT NULL,     -- When to deliver
    released_at TIMESTAMPTZ,             -- When actually delivered (null if pending)

    -- Priority (high priority bypasses quiet hours)
    is_priority BOOLEAN NOT NULL DEFAULT false
);

CREATE INDEX idx_held_notifications_user_id ON held_notifications(user_id);
CREATE INDEX idx_held_notifications_release ON held_notifications(release_at) WHERE released_at IS NULL;
CREATE INDEX idx_held_notifications_pending ON held_notifications(user_id, released_at) WHERE released_at IS NULL;

-- RLS
ALTER TABLE held_notifications ENABLE ROW LEVEL SECURITY;

CREATE POLICY held_notifications_user_access ON held_notifications
    FOR ALL
    USING (user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID OR is_super_admin());

-- ============================================================================
-- DEFAULT EVENT TYPES
-- ============================================================================

-- Create a reference table for available event types
CREATE TABLE notification_event_types (
    event_type TEXT PRIMARY KEY,
    category notification_event_category NOT NULL,
    display_name TEXT NOT NULL,
    description TEXT,
    is_priority BOOLEAN NOT NULL DEFAULT false,  -- Bypasses quiet hours
    default_push BOOLEAN NOT NULL DEFAULT true,
    default_email BOOLEAN NOT NULL DEFAULT true,
    default_in_app BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Seed default event types
INSERT INTO notification_event_types (event_type, category, display_name, description, is_priority, default_push, default_email, default_in_app) VALUES
    -- Faults
    ('fault.created', 'fault', 'New Fault Report', 'A new fault has been reported in your building', false, true, true, true),
    ('fault.updated', 'fault', 'Fault Updated', 'A fault you reported or are tracking has been updated', false, true, false, true),
    ('fault.resolved', 'fault', 'Fault Resolved', 'A fault has been marked as resolved', false, true, true, true),
    ('fault.assigned', 'fault', 'Fault Assigned', 'A fault has been assigned to you', false, true, true, true),

    -- Votes
    ('vote.created', 'vote', 'New Vote', 'A new vote has been created in your building', false, true, true, true),
    ('vote.reminder', 'vote', 'Vote Reminder', 'Reminder to cast your vote before deadline', false, true, true, true),
    ('vote.closing_soon', 'vote', 'Vote Closing Soon', 'A vote is closing within 24 hours', false, true, true, true),
    ('vote.closed', 'vote', 'Vote Closed', 'A vote has closed and results are available', false, true, true, true),

    -- Announcements
    ('announcement.posted', 'announcement', 'New Announcement', 'A new announcement has been posted', false, true, true, true),
    ('announcement.urgent', 'announcement', 'Urgent Announcement', 'An urgent announcement requires your attention', true, true, true, true),
    ('announcement.comment', 'announcement', 'Announcement Comment', 'Someone commented on an announcement', false, false, false, true),

    -- Documents
    ('document.shared', 'document', 'Document Shared', 'A document has been shared with you', false, true, true, true),
    ('document.signature_request', 'document', 'Signature Required', 'You have been asked to sign a document', true, true, true, true),
    ('document.signed', 'document', 'Document Signed', 'A document has been fully signed', false, true, true, true),

    -- Messages
    ('message.received', 'message', 'New Message', 'You have received a new direct message', false, true, true, true),
    ('message.thread_reply', 'message', 'Thread Reply', 'Someone replied to a message thread', false, false, false, true),

    -- Critical (always priority)
    ('critical.emergency', 'critical', 'Emergency Alert', 'Emergency alert for your building', true, true, true, true),
    ('critical.maintenance', 'critical', 'Scheduled Maintenance', 'Scheduled maintenance notification', true, true, true, true),
    ('critical.security', 'critical', 'Security Alert', 'Security-related alert', true, true, true, true),

    -- Finance
    ('finance.payment_due', 'finance', 'Payment Due', 'A payment is due soon', false, true, true, true),
    ('finance.payment_received', 'finance', 'Payment Received', 'Your payment has been received', false, false, true, true),
    ('finance.balance_low', 'finance', 'Low Balance Warning', 'Your account balance is running low', false, true, true, true),

    -- Facilities
    ('facility.booking_confirmed', 'facility', 'Booking Confirmed', 'Your facility booking has been confirmed', false, true, true, true),
    ('facility.booking_cancelled', 'facility', 'Booking Cancelled', 'A facility booking has been cancelled', false, true, true, true),
    ('facility.booking_reminder', 'facility', 'Booking Reminder', 'Reminder of upcoming facility booking', false, true, false, true);

-- ============================================================================
-- COMMENTS
-- ============================================================================

COMMENT ON TABLE event_notification_preferences IS 'Per-event-type and per-channel notification preferences (Epic 8B, Stories 8B.1 & 8B.2)';
COMMENT ON TABLE notification_schedule IS 'Quiet hours and digest settings (Epic 8B, Story 8B.3)';
COMMENT ON TABLE role_notification_defaults IS 'Organization role-based default preferences (Epic 8B, Story 8B.4)';
COMMENT ON TABLE held_notifications IS 'Notifications held during quiet hours for later delivery';
COMMENT ON TABLE notification_event_types IS 'Reference table of available notification event types and their defaults';
