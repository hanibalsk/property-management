-- Migration: 00025_create_audit_logs.sql
-- Epic 9: Privacy, Security & GDPR
-- Story 9.6: Compliance Audit Logs

-- Create audit_action enum
DO $$ BEGIN
    CREATE TYPE audit_action AS ENUM (
        -- Authentication actions
        'login',
        'logout',
        'login_failed',
        'password_changed',
        'password_reset_requested',
        'password_reset_completed',
        -- 2FA actions
        'mfa_enabled',
        'mfa_disabled',
        'mfa_backup_code_used',
        'mfa_backup_codes_regenerated',
        -- Account actions
        'account_created',
        'account_updated',
        'account_suspended',
        'account_reactivated',
        'account_deleted',
        -- GDPR actions
        'data_export_requested',
        'data_export_downloaded',
        'data_deletion_requested',
        'data_deletion_cancelled',
        'data_deletion_completed',
        -- Privacy actions
        'privacy_settings_updated',
        -- Role/permission actions
        'role_assigned',
        'role_removed',
        'permissions_changed',
        -- Organization actions
        'org_member_added',
        'org_member_removed',
        'org_settings_changed',
        -- Generic CRUD
        'resource_created',
        'resource_updated',
        'resource_deleted',
        'resource_accessed'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- Create audit_logs table
CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -- Who performed the action
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    -- What action was performed
    action audit_action NOT NULL,
    -- What resource was affected (e.g., 'user', 'organization', 'document')
    resource_type VARCHAR(100),
    -- ID of the affected resource
    resource_id UUID,
    -- Organization context (if applicable)
    org_id UUID REFERENCES organizations(id) ON DELETE SET NULL,
    -- Detailed information about the action
    details JSONB,
    -- Previous state (for updates)
    old_values JSONB,
    -- New state (for updates)
    new_values JSONB,
    -- Request metadata
    ip_address VARCHAR(45),
    user_agent TEXT,
    -- Hash for integrity verification (computed on insert)
    integrity_hash VARCHAR(64),
    -- Previous log entry hash (for chain verification)
    previous_hash VARCHAR(64),
    -- Timestamp
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for common query patterns
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_action ON audit_logs(action);
CREATE INDEX IF NOT EXISTS idx_audit_logs_resource ON audit_logs(resource_type, resource_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_org_id ON audit_logs(org_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON audit_logs(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_action_date ON audit_logs(user_id, action, created_at DESC);

-- RLS policy: Only super admins can query audit logs directly
-- Regular access should go through application layer with proper authorization
ALTER TABLE audit_logs ENABLE ROW LEVEL SECURITY;

-- Super admin can access all audit logs
CREATE POLICY audit_logs_super_admin ON audit_logs
    FOR ALL
    USING (is_super_admin());

-- Users can view their own audit logs (for GDPR transparency)
CREATE POLICY audit_logs_own_actions ON audit_logs
    FOR SELECT
    USING (user_id = current_setting('app.current_user_id', true)::uuid);

-- Prevent direct modifications (audit logs are append-only)
CREATE POLICY audit_logs_insert_only ON audit_logs
    FOR INSERT
    WITH CHECK (true);

-- Note: No UPDATE or DELETE policies - audit logs should be immutable

COMMENT ON TABLE audit_logs IS 'Immutable audit trail for compliance and security tracking';
COMMENT ON COLUMN audit_logs.integrity_hash IS 'SHA-256 hash of log entry for tamper detection';
COMMENT ON COLUMN audit_logs.previous_hash IS 'Hash of previous entry for chain verification';
COMMENT ON COLUMN audit_logs.details IS 'Additional context about the action (e.g., request parameters)';
COMMENT ON COLUMN audit_logs.old_values IS 'State before change (for update actions)';
COMMENT ON COLUMN audit_logs.new_values IS 'State after change (for update actions)';
