-- Migration: 00027_add_scheduled_deletion_to_users
-- Epic 9 / Story 9.4: GDPR Data Deletion
--
-- Adds scheduled deletion timestamp to users for GDPR Article 17 compliance.
-- Users have a 30-day grace period to cancel deletion requests.

-- ============================================================================
-- ADD SCHEDULED DELETION COLUMN
-- ============================================================================

ALTER TABLE users
    ADD COLUMN IF NOT EXISTS scheduled_deletion_at TIMESTAMPTZ DEFAULT NULL;

-- ============================================================================
-- INDEX FOR CLEANUP JOB
-- ============================================================================

-- Index for finding accounts ready for deletion
CREATE INDEX IF NOT EXISTS idx_users_scheduled_deletion
    ON users(scheduled_deletion_at)
    WHERE scheduled_deletion_at IS NOT NULL;

-- ============================================================================
-- COMMENTS
-- ============================================================================

COMMENT ON COLUMN users.scheduled_deletion_at IS 'When account is scheduled for GDPR deletion (30-day grace period). NULL = no pending deletion.';
