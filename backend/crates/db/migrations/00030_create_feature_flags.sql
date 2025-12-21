-- Epic 10B, Story 10B.2: Feature Flag Management
-- Migration: Create feature flag infrastructure

-- Create feature_flags table
CREATE TABLE IF NOT EXISTS feature_flags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key VARCHAR(100) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    is_enabled BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create feature_flag_overrides table for targeted enablement
CREATE TABLE IF NOT EXISTS feature_flag_overrides (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    flag_id UUID NOT NULL REFERENCES feature_flags(id) ON DELETE CASCADE,
    scope_type VARCHAR(50) NOT NULL CHECK (scope_type IN ('organization', 'user', 'role')),
    scope_id UUID NOT NULL,
    is_enabled BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(flag_id, scope_type, scope_id)
);

-- Create indexes for efficient lookups
CREATE INDEX IF NOT EXISTS idx_feature_flags_key ON feature_flags(key);
CREATE INDEX IF NOT EXISTS idx_feature_flag_overrides_flag_id ON feature_flag_overrides(flag_id);
CREATE INDEX IF NOT EXISTS idx_feature_flag_overrides_scope ON feature_flag_overrides(scope_type, scope_id);

-- Seed initial feature flags
INSERT INTO feature_flags (key, name, description, is_enabled) VALUES
    ('new_dashboard', 'New Dashboard', 'Enable the redesigned dashboard experience', false),
    ('dark_mode', 'Dark Mode', 'Enable dark mode theme option', false),
    ('advanced_analytics', 'Advanced Analytics', 'Enable advanced analytics features', false),
    ('beta_features', 'Beta Features', 'Enable beta features for testing', false),
    ('ai_suggestions', 'AI Suggestions', 'Enable AI-powered suggestions for fault triage', false)
ON CONFLICT (key) DO NOTHING;

-- Add comment for documentation
COMMENT ON TABLE feature_flags IS 'Feature flags for controlled feature rollout (Epic 10B, Story 10B.2)';
COMMENT ON TABLE feature_flag_overrides IS 'Targeted overrides for feature flags by org, user, or role';
