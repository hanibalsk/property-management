-- Migration: 00024_create_user_2fa.sql
-- Epic 9: Privacy, Security & GDPR
-- Story 9.1: TOTP Two-Factor Authentication Setup

-- Create user_2fa table for storing TOTP secrets and backup codes
CREATE TABLE IF NOT EXISTS user_2fa (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    -- TOTP secret (encrypted/hashed in application layer)
    secret VARCHAR(255) NOT NULL,
    -- Whether 2FA is currently enabled
    enabled BOOLEAN NOT NULL DEFAULT false,
    -- When 2FA was enabled
    enabled_at TIMESTAMPTZ,
    -- Hashed backup codes (10 codes, each hashed like passwords)
    backup_codes JSONB NOT NULL DEFAULT '[]'::jsonb,
    -- Remaining unused backup codes count
    backup_codes_remaining INTEGER NOT NULL DEFAULT 10,
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for fast user lookup
CREATE INDEX IF NOT EXISTS idx_user_2fa_user_id ON user_2fa(user_id);

-- Automatic updated_at trigger
CREATE TRIGGER update_user_2fa_updated_at
    BEFORE UPDATE ON user_2fa
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- RLS policy: users can only access their own 2FA settings
ALTER TABLE user_2fa ENABLE ROW LEVEL SECURITY;

-- Policy: users can view their own 2FA settings
CREATE POLICY user_2fa_select_own ON user_2fa
    FOR SELECT
    USING (user_id = current_setting('app.current_user_id', true)::uuid);

-- Policy: users can insert their own 2FA settings
CREATE POLICY user_2fa_insert_own ON user_2fa
    FOR INSERT
    WITH CHECK (user_id = current_setting('app.current_user_id', true)::uuid);

-- Policy: users can update their own 2FA settings
CREATE POLICY user_2fa_update_own ON user_2fa
    FOR UPDATE
    USING (user_id = current_setting('app.current_user_id', true)::uuid)
    WITH CHECK (user_id = current_setting('app.current_user_id', true)::uuid);

-- Policy: users can delete their own 2FA settings
CREATE POLICY user_2fa_delete_own ON user_2fa
    FOR DELETE
    USING (user_id = current_setting('app.current_user_id', true)::uuid);

-- Super admin bypass
CREATE POLICY user_2fa_super_admin ON user_2fa
    FOR ALL
    USING (is_super_admin());

COMMENT ON TABLE user_2fa IS 'Stores TOTP 2FA secrets and backup codes for users';
COMMENT ON COLUMN user_2fa.secret IS 'Base32-encoded TOTP secret (encrypted)';
COMMENT ON COLUMN user_2fa.backup_codes IS 'Array of hashed backup codes';
COMMENT ON COLUMN user_2fa.backup_codes_remaining IS 'Count of remaining unused backup codes';
