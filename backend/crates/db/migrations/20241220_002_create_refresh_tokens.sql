-- Epic 1, Story 1.2: Email/Password Login
-- Creates refresh_tokens table for JWT session management

-- Refresh tokens table
CREATE TABLE IF NOT EXISTS refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,

    -- Session metadata (Story 1.5)
    user_agent TEXT,
    ip_address VARCHAR(45),  -- IPv4 (15) or IPv6 (45) as string
    device_info TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Login attempts table for rate limiting
CREATE TABLE IF NOT EXISTS login_attempts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL,
    ip_address VARCHAR(45) NOT NULL,  -- IPv4 (15) or IPv6 (45) as string
    success BOOLEAN NOT NULL,
    attempt_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_user_id ON refresh_tokens(user_id);
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_expires ON refresh_tokens(expires_at) WHERE revoked_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_login_attempts_email_time ON login_attempts(email, attempt_at);
CREATE INDEX IF NOT EXISTS idx_login_attempts_ip_time ON login_attempts(ip_address, attempt_at);

-- Cleanup old login attempts (older than 1 hour)
CREATE OR REPLACE FUNCTION cleanup_old_login_attempts()
RETURNS void AS $$
BEGIN
    DELETE FROM login_attempts WHERE attempt_at < NOW() - INTERVAL '1 hour';
END;
$$ LANGUAGE plpgsql;
