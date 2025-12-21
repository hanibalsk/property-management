-- Migration: 00022_create_critical_notifications
-- Epic 8A / Story 8A.2: Critical Notification Override
--
-- System administrators can send critical notifications that bypass user preferences.
-- Users must acknowledge critical notifications before dismissing them.

-- ============================================================================
-- CRITICAL NOTIFICATIONS TABLE
-- ============================================================================

CREATE TABLE critical_notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- CRITICAL NOTIFICATION ACKNOWLEDGMENTS TABLE
-- ============================================================================

CREATE TABLE critical_notification_acknowledgments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    notification_id UUID NOT NULL REFERENCES critical_notifications(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    acknowledged_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Each user can only acknowledge once
    CONSTRAINT uq_critical_ack_notification_user UNIQUE (notification_id, user_id)
);

-- ============================================================================
-- INDEXES
-- ============================================================================

-- Fast lookup by organization
CREATE INDEX idx_critical_notifications_org ON critical_notifications(organization_id);

-- Fast lookup by creator
CREATE INDEX idx_critical_notifications_creator ON critical_notifications(created_by);

-- Fast lookup of acknowledgments by notification
CREATE INDEX idx_critical_acks_notification ON critical_notification_acknowledgments(notification_id);

-- Fast lookup of acknowledgments by user
CREATE INDEX idx_critical_acks_user ON critical_notification_acknowledgments(user_id);

-- ============================================================================
-- ROW LEVEL SECURITY
-- ============================================================================

ALTER TABLE critical_notifications ENABLE ROW LEVEL SECURITY;
ALTER TABLE critical_notification_acknowledgments ENABLE ROW LEVEL SECURITY;

-- All organization members can read critical notifications
-- Only admins can create them (enforced at API level)
CREATE POLICY critical_notifications_org_read ON critical_notifications
    FOR SELECT
    USING (
        organization_id = COALESCE(
            current_setting('request.org_id', true)::UUID,
            '00000000-0000-0000-0000-000000000000'::UUID
        )
    );

-- Only the creator or admin can insert (enforced at API level via role check)
CREATE POLICY critical_notifications_insert ON critical_notifications
    FOR INSERT
    WITH CHECK (
        created_by = COALESCE(
            current_setting('request.user_id', true)::UUID,
            '00000000-0000-0000-0000-000000000000'::UUID
        )
    );

-- Users can only read their own acknowledgments or org-wide stats
CREATE POLICY critical_acks_read ON critical_notification_acknowledgments
    FOR SELECT
    USING (
        user_id = COALESCE(
            current_setting('request.user_id', true)::UUID,
            '00000000-0000-0000-0000-000000000000'::UUID
        )
        OR EXISTS (
            SELECT 1 FROM critical_notifications cn
            WHERE cn.id = notification_id
            AND cn.organization_id = COALESCE(
                current_setting('request.org_id', true)::UUID,
                '00000000-0000-0000-0000-000000000000'::UUID
            )
        )
    );

-- Users can only acknowledge for themselves
CREATE POLICY critical_acks_insert ON critical_notification_acknowledgments
    FOR INSERT
    WITH CHECK (
        user_id = COALESCE(
            current_setting('request.user_id', true)::UUID,
            '00000000-0000-0000-0000-000000000000'::UUID
        )
    );

-- ============================================================================
-- COMMENTS
-- ============================================================================

COMMENT ON TABLE critical_notifications IS 'Critical notifications that bypass user preferences (Epic 8A, Story 8A.2)';
COMMENT ON COLUMN critical_notifications.title IS 'Short title for the critical notification';
COMMENT ON COLUMN critical_notifications.message IS 'Full message content';
COMMENT ON TABLE critical_notification_acknowledgments IS 'User acknowledgments of critical notifications';
