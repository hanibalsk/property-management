-- Epic 89: Feature Flags & Health Monitoring (Story 89.1-89.5)
-- Migration: Create infrastructure feature flag tables with audit logging

-- Create enum for feature flag value type (if not exists)
DO $$ BEGIN
    CREATE TYPE feature_flag_value_type AS ENUM (
        'boolean',
        'string',
        'number',
        'json'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- Create enum for feature flag override type
DO $$ BEGIN
    CREATE TYPE feature_flag_override_type AS ENUM (
        'user',
        'organization',
        'percentage'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- Create enum for feature flag audit action
DO $$ BEGIN
    CREATE TYPE feature_flag_audit_action AS ENUM (
        'created',
        'updated',
        'enabled',
        'disabled',
        'rollout_changed',
        'targeting_changed',
        'override_added',
        'override_removed',
        'archived',
        'deleted'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- Create infrastructure_feature_flags table (separate from platform_admin feature_flags)
CREATE TABLE IF NOT EXISTS infrastructure_feature_flags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key VARCHAR(100) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    enabled BOOLEAN NOT NULL DEFAULT false,
    rollout_percentage INTEGER NOT NULL DEFAULT 100 CHECK (rollout_percentage >= 0 AND rollout_percentage <= 100),
    targeting_rules JSONB,
    default_value JSONB NOT NULL DEFAULT 'false'::JSONB,
    value_type feature_flag_value_type NOT NULL DEFAULT 'boolean',
    environment VARCHAR(50) NOT NULL DEFAULT 'production',
    tags TEXT[],
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_evaluated_at TIMESTAMPTZ
);

-- Create infrastructure_feature_flag_overrides table
CREATE TABLE IF NOT EXISTS infrastructure_feature_flag_overrides (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    flag_id UUID NOT NULL REFERENCES infrastructure_feature_flags(id) ON DELETE CASCADE,
    override_type feature_flag_override_type NOT NULL,
    target_id UUID,
    value JSONB NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT true,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create infrastructure_feature_flag_audit_logs table
CREATE TABLE IF NOT EXISTS infrastructure_feature_flag_audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    flag_id UUID NOT NULL REFERENCES infrastructure_feature_flags(id) ON DELETE CASCADE,
    action feature_flag_audit_action NOT NULL,
    performed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    previous_state JSONB,
    new_state JSONB,
    context JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_infra_feature_flags_key ON infrastructure_feature_flags(key);
CREATE INDEX IF NOT EXISTS idx_infra_feature_flags_environment ON infrastructure_feature_flags(environment);
CREATE INDEX IF NOT EXISTS idx_infra_feature_flags_enabled ON infrastructure_feature_flags(enabled);
CREATE INDEX IF NOT EXISTS idx_infra_feature_flag_overrides_flag_id ON infrastructure_feature_flag_overrides(flag_id);
CREATE INDEX IF NOT EXISTS idx_infra_feature_flag_overrides_target ON infrastructure_feature_flag_overrides(override_type, target_id);
CREATE INDEX IF NOT EXISTS idx_infra_feature_flag_overrides_expires ON infrastructure_feature_flag_overrides(expires_at) WHERE expires_at IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_infra_feature_flag_audit_logs_flag_id ON infrastructure_feature_flag_audit_logs(flag_id);
CREATE INDEX IF NOT EXISTS idx_infra_feature_flag_audit_logs_created_at ON infrastructure_feature_flag_audit_logs(created_at DESC);

-- Auto-update updated_at trigger
CREATE OR REPLACE FUNCTION update_infrastructure_feature_flag_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_infrastructure_feature_flags_updated_at ON infrastructure_feature_flags;
CREATE TRIGGER trigger_infrastructure_feature_flags_updated_at
    BEFORE UPDATE ON infrastructure_feature_flags
    FOR EACH ROW
    EXECUTE FUNCTION update_infrastructure_feature_flag_updated_at();

-- Add comments
COMMENT ON TABLE infrastructure_feature_flags IS 'Feature flags for Epic 89 infrastructure monitoring with advanced targeting and audit logging';
COMMENT ON TABLE infrastructure_feature_flag_overrides IS 'User and organization-specific overrides for feature flags';
COMMENT ON TABLE infrastructure_feature_flag_audit_logs IS 'Audit trail for all feature flag changes';
