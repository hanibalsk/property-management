-- Phase 1.2: Add is_super_admin field to users table
-- This enables proper super admin access control without hardcoding

-- Add is_super_admin column with default false
ALTER TABLE users ADD COLUMN IF NOT EXISTS is_super_admin BOOLEAN NOT NULL DEFAULT FALSE;

-- Create index for super admin lookups
CREATE INDEX IF NOT EXISTS idx_users_is_super_admin ON users(is_super_admin) WHERE is_super_admin = TRUE;

-- Add comment for documentation
COMMENT ON COLUMN users.is_super_admin IS 'Platform-level super administrator flag. Super admins have full access across all organizations.';

-- Update RLS policies to respect is_super_admin flag
-- This creates a helper function for consistent super admin checks
CREATE OR REPLACE FUNCTION is_current_user_super_admin()
RETURNS BOOLEAN AS $$
BEGIN
    -- Check the session setting first (for performance)
    IF current_setting('app.is_super_admin', true) = 'true' THEN
        RETURN TRUE;
    END IF;

    -- Fallback to database check
    RETURN EXISTS(
        SELECT 1 FROM users
        WHERE id = current_setting('app.current_user_id', true)::UUID
        AND is_super_admin = TRUE
        AND status = 'active'
    );
EXCEPTION
    WHEN OTHERS THEN
        RETURN FALSE;
END;
$$ LANGUAGE plpgsql STABLE;

COMMENT ON FUNCTION is_current_user_super_admin() IS 'Check if the current session user is a super admin. Uses session setting with database fallback.';
