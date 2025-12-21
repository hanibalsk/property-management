-- Migration: 00018_add_user_privacy_settings
-- Epic 6 / Story 6.6: Neighbor Information (Privacy-Aware)
--
-- Adds privacy settings to users table for controlling visibility to neighbors.

-- ============================================================================
-- ADD PRIVACY COLUMNS TO USERS TABLE
-- ============================================================================

-- Profile visibility: controls what neighbors see
-- - 'visible': Full name and unit shown (default)
-- - 'hidden': Shows as "Resident of Unit X"
-- - 'contacts_only': Name shown but no contact info unless connected
ALTER TABLE users
    ADD COLUMN IF NOT EXISTS profile_visibility VARCHAR(20) NOT NULL DEFAULT 'visible'
        CHECK (profile_visibility IN ('visible', 'hidden', 'contacts_only'));

-- Whether to show contact info (email/phone) to neighbors
ALTER TABLE users
    ADD COLUMN IF NOT EXISTS show_contact_info BOOLEAN NOT NULL DEFAULT false;

-- ============================================================================
-- INDEXES
-- ============================================================================

-- Index for neighbor queries (joining with unit_residents)
-- This helps when finding neighbors in the same building
CREATE INDEX IF NOT EXISTS idx_users_privacy_visibility
    ON users(profile_visibility);

-- Comment
COMMENT ON COLUMN users.profile_visibility IS 'Privacy setting: visible (full name), hidden (anonymous), or contacts_only (name but no contact)';
COMMENT ON COLUMN users.show_contact_info IS 'Whether to show email/phone to neighbors with visible profiles';
