-- Epic 108: Feature Packages & Bundles
-- Story 108.1: Feature Package Model
--
-- Creates the schema for feature packages, package items, and organization packages.
-- Feature packages bundle multiple features for subscription plans or standalone purchases.

-- Package type enum
DO $$ BEGIN
    CREATE TYPE package_type AS ENUM ('base', 'addon', 'trial');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Feature packages table
CREATE TABLE IF NOT EXISTS feature_packages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    description TEXT,
    short_description VARCHAR(255),
    icon VARCHAR(50),
    package_type package_type NOT NULL DEFAULT 'base',
    parent_package_id UUID REFERENCES feature_packages(id),
    linked_plan_id UUID REFERENCES subscription_plans(id),
    standalone_monthly_price DECIMAL(10,2),
    standalone_annual_price DECIMAL(10,2),
    currency VARCHAR(3) DEFAULT 'EUR',
    max_users INT,
    max_buildings INT,
    max_units INT,
    display_order INT DEFAULT 0,
    is_highlighted BOOLEAN DEFAULT false,
    highlight_text VARCHAR(50),
    color VARCHAR(7),
    is_active BOOLEAN DEFAULT true,
    is_public BOOLEAN DEFAULT true,
    version INT DEFAULT 1,
    valid_from TIMESTAMPTZ,
    valid_until TIMESTAMPTZ,
    translations JSONB DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Feature package items (features included in a package)
CREATE TABLE IF NOT EXISTS feature_package_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    package_id UUID NOT NULL REFERENCES feature_packages(id) ON DELETE CASCADE,
    feature_flag_id UUID NOT NULL REFERENCES feature_flags(id) ON DELETE CASCADE,
    custom_description TEXT,
    usage_limit INT,
    usage_unit VARCHAR(50),
    display_order INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(package_id, feature_flag_id)
);

-- Organization packages (packages assigned to organizations)
CREATE TABLE IF NOT EXISTS organization_packages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    package_id UUID NOT NULL REFERENCES feature_packages(id),
    source VARCHAR(50) NOT NULL,
    subscription_id UUID REFERENCES organization_subscriptions(id),
    is_active BOOLEAN DEFAULT true,
    activated_at TIMESTAMPTZ DEFAULT NOW(),
    deactivated_at TIMESTAMPTZ,
    valid_from TIMESTAMPTZ DEFAULT NOW(),
    valid_until TIMESTAMPTZ,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(organization_id, package_id)
);

-- Indexes for efficient querying
CREATE INDEX IF NOT EXISTS idx_feature_packages_type ON feature_packages(package_type);
CREATE INDEX IF NOT EXISTS idx_feature_packages_plan ON feature_packages(linked_plan_id);
CREATE INDEX IF NOT EXISTS idx_feature_packages_active ON feature_packages(is_active) WHERE is_active = true;
CREATE INDEX IF NOT EXISTS idx_feature_packages_public ON feature_packages(is_public) WHERE is_public = true;
CREATE INDEX IF NOT EXISTS idx_feature_packages_key ON feature_packages(key);

CREATE INDEX IF NOT EXISTS idx_package_items_package ON feature_package_items(package_id);
CREATE INDEX IF NOT EXISTS idx_package_items_feature ON feature_package_items(feature_flag_id);

CREATE INDEX IF NOT EXISTS idx_org_packages_org ON organization_packages(organization_id);
CREATE INDEX IF NOT EXISTS idx_org_packages_package ON organization_packages(package_id);
CREATE INDEX IF NOT EXISTS idx_org_packages_active ON organization_packages(is_active) WHERE is_active = true;
CREATE INDEX IF NOT EXISTS idx_org_packages_subscription ON organization_packages(subscription_id);

-- Add comments for documentation
COMMENT ON TABLE feature_packages IS 'Feature packages that bundle multiple features for subscription plans or standalone purchases';
COMMENT ON TABLE feature_package_items IS 'Features included in a package with optional custom limits';
COMMENT ON TABLE organization_packages IS 'Packages assigned to organizations (via subscription or direct purchase)';

COMMENT ON COLUMN feature_packages.key IS 'Unique identifier key for the package (e.g., starter, professional, enterprise)';
COMMENT ON COLUMN feature_packages.package_type IS 'Type of package: base (standalone plan), addon (requires base), trial (time-limited)';
COMMENT ON COLUMN feature_packages.linked_plan_id IS 'If set, this package is automatically assigned with this subscription plan';
COMMENT ON COLUMN feature_packages.is_highlighted IS 'Whether to visually highlight this package in the UI (e.g., "Most Popular")';
COMMENT ON COLUMN feature_packages.translations IS 'JSON object with translations for name, description, etc.';

COMMENT ON COLUMN feature_package_items.usage_limit IS 'Optional limit for this feature within the package (null = unlimited)';
COMMENT ON COLUMN feature_package_items.usage_unit IS 'Unit for the usage limit (e.g., "documents", "API calls")';

COMMENT ON COLUMN organization_packages.source IS 'How the package was acquired: subscription, purchase, promotion, trial';
