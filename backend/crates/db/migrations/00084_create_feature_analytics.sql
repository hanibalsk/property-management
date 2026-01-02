-- Epic 109: User Type Feature Experience (Story 109.4)
-- Migration: Create feature analytics and user type access tables

-- Create enum for feature access state
DO $$ BEGIN
    CREATE TYPE feature_access_state AS ENUM (
        'included',
        'optional',
        'excluded'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- Create enum for feature event type
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

-- Feature descriptors for UI display metadata (Epic 107 foundation)
CREATE TABLE IF NOT EXISTS feature_descriptors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    feature_flag_id UUID NOT NULL REFERENCES feature_flags(id) ON DELETE CASCADE,
    display_name VARCHAR(255) NOT NULL,
    short_description TEXT,
    long_description TEXT,
    icon VARCHAR(100),
    badge_text VARCHAR(50),
    help_url TEXT,
    category VARCHAR(100),
    sort_order INTEGER DEFAULT 0,
    is_premium BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_feature_descriptor UNIQUE (feature_flag_id)
);

-- User type access matrix (defines which features are available to which user types)
CREATE TABLE IF NOT EXISTS user_type_feature_access (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    feature_flag_id UUID NOT NULL REFERENCES feature_flags(id) ON DELETE CASCADE,
    user_type VARCHAR(50) NOT NULL,
    access_state feature_access_state NOT NULL DEFAULT 'excluded',
    default_enabled BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_user_type_feature UNIQUE (feature_flag_id, user_type)
);

-- Feature packages for bundling features (Epic 108 foundation)
CREATE TABLE IF NOT EXISTS feature_packages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    is_active BOOLEAN DEFAULT true,
    price_monthly_cents INTEGER,
    price_yearly_cents INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Features included in packages
CREATE TABLE IF NOT EXISTS feature_package_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    package_id UUID NOT NULL REFERENCES feature_packages(id) ON DELETE CASCADE,
    feature_flag_id UUID NOT NULL REFERENCES feature_flags(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_package_feature UNIQUE (package_id, feature_flag_id)
);

-- Organization package subscriptions
CREATE TABLE IF NOT EXISTS organization_feature_packages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    package_id UUID NOT NULL REFERENCES feature_packages(id) ON DELETE CASCADE,
    is_active BOOLEAN DEFAULT true,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_org_package UNIQUE (organization_id, package_id)
);

-- User feature preferences (for optional features)
CREATE TABLE IF NOT EXISTS user_feature_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    feature_flag_id UUID NOT NULL REFERENCES feature_flags(id) ON DELETE CASCADE,
    is_enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_user_feature_pref UNIQUE (user_id, feature_flag_id)
);

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

-- Create indexes for efficient querying
CREATE INDEX IF NOT EXISTS idx_feature_descriptors_flag ON feature_descriptors(feature_flag_id);
CREATE INDEX IF NOT EXISTS idx_feature_descriptors_category ON feature_descriptors(category);

CREATE INDEX IF NOT EXISTS idx_user_type_feature_access_flag ON user_type_feature_access(feature_flag_id);
CREATE INDEX IF NOT EXISTS idx_user_type_feature_access_user_type ON user_type_feature_access(user_type);
CREATE INDEX IF NOT EXISTS idx_user_type_feature_access_state ON user_type_feature_access(access_state);

CREATE INDEX IF NOT EXISTS idx_feature_package_items_package ON feature_package_items(package_id);
CREATE INDEX IF NOT EXISTS idx_feature_package_items_flag ON feature_package_items(feature_flag_id);

CREATE INDEX IF NOT EXISTS idx_org_feature_packages_org ON organization_feature_packages(organization_id);
CREATE INDEX IF NOT EXISTS idx_org_feature_packages_package ON organization_feature_packages(package_id);
CREATE INDEX IF NOT EXISTS idx_org_feature_packages_active ON organization_feature_packages(is_active) WHERE is_active = true;

CREATE INDEX IF NOT EXISTS idx_user_feature_prefs_user ON user_feature_preferences(user_id);
CREATE INDEX IF NOT EXISTS idx_user_feature_prefs_flag ON user_feature_preferences(feature_flag_id);

CREATE INDEX IF NOT EXISTS idx_feature_usage_feature ON feature_usage_events(feature_flag_id);
CREATE INDEX IF NOT EXISTS idx_feature_usage_created ON feature_usage_events(created_at);
CREATE INDEX IF NOT EXISTS idx_feature_usage_event_type ON feature_usage_events(event_type);
CREATE INDEX IF NOT EXISTS idx_feature_usage_user ON feature_usage_events(user_id) WHERE user_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_feature_usage_org ON feature_usage_events(organization_id) WHERE organization_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_feature_usage_user_type ON feature_usage_events(user_type) WHERE user_type IS NOT NULL;

-- Auto-update updated_at triggers
CREATE OR REPLACE FUNCTION update_feature_descriptors_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_feature_descriptors_updated_at ON feature_descriptors;
CREATE TRIGGER trigger_feature_descriptors_updated_at
    BEFORE UPDATE ON feature_descriptors
    FOR EACH ROW
    EXECUTE FUNCTION update_feature_descriptors_updated_at();

CREATE OR REPLACE FUNCTION update_user_type_feature_access_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_user_type_feature_access_updated_at ON user_type_feature_access;
CREATE TRIGGER trigger_user_type_feature_access_updated_at
    BEFORE UPDATE ON user_type_feature_access
    FOR EACH ROW
    EXECUTE FUNCTION update_user_type_feature_access_updated_at();

CREATE OR REPLACE FUNCTION update_feature_packages_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_feature_packages_updated_at ON feature_packages;
CREATE TRIGGER trigger_feature_packages_updated_at
    BEFORE UPDATE ON feature_packages
    FOR EACH ROW
    EXECUTE FUNCTION update_feature_packages_updated_at();

CREATE OR REPLACE FUNCTION update_user_feature_preferences_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_user_feature_preferences_updated_at ON user_feature_preferences;
CREATE TRIGGER trigger_user_feature_preferences_updated_at
    BEFORE UPDATE ON user_feature_preferences
    FOR EACH ROW
    EXECUTE FUNCTION update_user_feature_preferences_updated_at();

-- Add comments
COMMENT ON TABLE feature_descriptors IS 'UI display metadata for feature flags (Epic 107/109)';
COMMENT ON TABLE user_type_feature_access IS 'Access control matrix defining feature availability by user type (Epic 109)';
COMMENT ON TABLE feature_packages IS 'Feature bundles for subscription/upgrade purposes (Epic 108/109)';
COMMENT ON TABLE feature_package_items IS 'Features included in each package (Epic 108/109)';
COMMENT ON TABLE organization_feature_packages IS 'Organization subscriptions to feature packages (Epic 109)';
COMMENT ON TABLE user_feature_preferences IS 'User preferences for optional features (Epic 109)';
COMMENT ON TABLE feature_usage_events IS 'Analytics tracking for feature usage (Epic 109 Story 109.4)';
