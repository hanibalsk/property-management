-- Epic 109: User Type Feature Experience (Story 109.4)
-- Migration: Create feature analytics tables
-- NOTE: feature_descriptors, feature_packages, feature_package_items, user_feature_preferences
--       are already created by migrations 82 and 83. This migration only adds analytics tables.

-- Create enum for feature event type (if not exists)
DO $$ BEGIN
    CREATE TYPE feature_event_type AS ENUM (
        'access',
        'blocked',
        'upgrade_prompt',
        'upgrade_clicked',
        'toggled_on',
        'toggled_off'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- Feature usage analytics events (Story 109.4)
CREATE TABLE IF NOT EXISTS feature_usage_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    feature_flag_id UUID NOT NULL REFERENCES feature_flags(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    organization_id UUID REFERENCES organizations(id) ON DELETE SET NULL,
    event_type feature_event_type NOT NULL,
    user_type VARCHAR(50),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for feature_usage_events only (other tables already have indexes from migrations 82/83)
CREATE INDEX IF NOT EXISTS idx_feature_usage_feature ON feature_usage_events(feature_flag_id);
CREATE INDEX IF NOT EXISTS idx_feature_usage_created ON feature_usage_events(created_at);
CREATE INDEX IF NOT EXISTS idx_feature_usage_event_type ON feature_usage_events(event_type);
CREATE INDEX IF NOT EXISTS idx_feature_usage_user ON feature_usage_events(user_id) WHERE user_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_feature_usage_org ON feature_usage_events(organization_id) WHERE organization_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_feature_usage_user_type ON feature_usage_events(user_type) WHERE user_type IS NOT NULL;

-- Add comment for the new table
COMMENT ON TABLE feature_usage_events IS 'Analytics tracking for feature usage (Epic 109 Story 109.4)';
