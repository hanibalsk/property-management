-- Migration: 00023_fix_notification_preferences_trigger
-- Epic 8A: Fix RLS issues with notification_preferences and critical_notifications
--
-- Issues fixed:
-- 1. The trigger that creates default notification preferences for new users
--    was blocked by RLS because request.user_id is not set during user creation.
-- 2. RLS policies were missing super admin bypass for test/admin operations.

-- ============================================================================
-- FIX TRIGGER FUNCTION
-- ============================================================================

-- Drop and recreate the function with SECURITY DEFINER
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
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Set the search_path to prevent privilege escalation attacks
-- This is a security best practice for SECURITY DEFINER functions
ALTER FUNCTION create_default_notification_preferences() SET search_path = public;

-- ============================================================================
-- ADD SUPER ADMIN BYPASS POLICIES
-- ============================================================================

-- Drop existing policies to recreate them with super admin bypass
DROP POLICY IF EXISTS notification_preferences_user_access ON notification_preferences;
DROP POLICY IF EXISTS critical_notifications_org_read ON critical_notifications;
DROP POLICY IF EXISTS critical_notifications_insert ON critical_notifications;
DROP POLICY IF EXISTS critical_acks_read ON critical_notification_acknowledgments;
DROP POLICY IF EXISTS critical_acks_insert ON critical_notification_acknowledgments;

-- notification_preferences: Users can only access their own preferences (or super admin)
CREATE POLICY notification_preferences_user_access ON notification_preferences
    FOR ALL
    USING (
        user_id = COALESCE(current_setting('request.user_id', true)::UUID, '00000000-0000-0000-0000-000000000000'::UUID)
        OR is_super_admin()
    )
    WITH CHECK (
        user_id = COALESCE(current_setting('request.user_id', true)::UUID, '00000000-0000-0000-0000-000000000000'::UUID)
        OR is_super_admin()
    );

-- critical_notifications: Org members can read (or super admin)
CREATE POLICY critical_notifications_org_read ON critical_notifications
    FOR SELECT
    USING (
        organization_id = COALESCE(
            current_setting('request.org_id', true)::UUID,
            '00000000-0000-0000-0000-000000000000'::UUID
        )
        OR is_super_admin()
    );

-- critical_notifications: Creator or super admin can insert
CREATE POLICY critical_notifications_insert ON critical_notifications
    FOR INSERT
    WITH CHECK (
        created_by = COALESCE(
            current_setting('request.user_id', true)::UUID,
            '00000000-0000-0000-0000-000000000000'::UUID
        )
        OR is_super_admin()
    );

-- critical_notification_acknowledgments: Users can read their own or org stats (or super admin)
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
        OR is_super_admin()
    );

-- critical_notification_acknowledgments: Users can insert for themselves (or super admin)
CREATE POLICY critical_acks_insert ON critical_notification_acknowledgments
    FOR INSERT
    WITH CHECK (
        user_id = COALESCE(
            current_setting('request.user_id', true)::UUID,
            '00000000-0000-0000-0000-000000000000'::UUID
        )
        OR is_super_admin()
    );

-- ============================================================================
-- COMMENTS
-- ============================================================================

COMMENT ON FUNCTION create_default_notification_preferences() IS
    'Creates default notification preferences for new users. Uses SECURITY DEFINER to bypass RLS.';
