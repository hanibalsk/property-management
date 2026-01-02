-- Epic 107: Feature Descriptors & Catalog
-- Migration: Create feature descriptor infrastructure
--
-- This migration adds:
-- 1. feature_categories - hierarchical feature categorization
-- 2. feature_descriptors - rich metadata for feature flags (name, description, icon, etc.)
-- 3. feature_user_type_access - user type-based feature access matrix
-- 4. user_feature_preferences - user preferences for optional features

-- ==================== Story 107.2: Feature Category Management ====================

CREATE TABLE IF NOT EXISTS feature_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -- Unique key for the category (e.g., 'property_management', 'ai_features')
    key VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    -- Icon identifier (e.g., 'home', 'chart', 'brain')
    icon VARCHAR(50),
    -- Hex color for visual grouping (e.g., '#3B82F6')
    color VARCHAR(7),

    -- Hierarchical support
    parent_id UUID REFERENCES feature_categories(id) ON DELETE SET NULL,
    display_order INT DEFAULT 0,

    -- Localization ({"sk": {"name": "...", "description": "..."}, ...})
    translations JSONB DEFAULT '{}',
    -- Additional metadata
    metadata JSONB DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_feature_categories_parent ON feature_categories(parent_id);
CREATE INDEX IF NOT EXISTS idx_feature_categories_order ON feature_categories(display_order);

-- ==================== Story 107.1: Feature Descriptor Model ====================

CREATE TABLE IF NOT EXISTS feature_descriptors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -- One-to-one relationship with feature_flags
    feature_flag_id UUID NOT NULL UNIQUE REFERENCES feature_flags(id) ON DELETE CASCADE,

    -- Display information
    display_name VARCHAR(100) NOT NULL,
    short_description VARCHAR(255),
    full_description TEXT,
    -- Icon identifier (e.g., 'home', 'chart', 'lock')
    icon VARCHAR(50),
    -- Preview image URL for feature screenshots
    preview_image_url VARCHAR(500),

    -- Categorization
    category_id UUID REFERENCES feature_categories(id) ON DELETE SET NULL,
    -- Subcategory key within the category
    subcategory VARCHAR(50),
    -- Tags for filtering (["analytics", "premium", "new"])
    tags JSONB DEFAULT '[]',

    -- Localization ({"sk": {"name": "...", "description": "..."}, ...})
    translations JSONB DEFAULT '{}',

    -- Marketing content
    benefits JSONB DEFAULT '[]',  -- ["Benefit 1", "Benefit 2"]
    use_cases JSONB DEFAULT '[]', -- [{"title": "...", "description": "..."}, ...]

    -- Technical metadata
    api_scopes JSONB DEFAULT '[]',     -- Required OAuth scopes
    depends_on JSONB DEFAULT '[]',     -- Feature flag keys this depends on
    conflicts_with JSONB DEFAULT '[]', -- Feature flag keys this conflicts with

    -- UI display options
    display_order INT DEFAULT 0,
    is_highlighted BOOLEAN DEFAULT false,
    -- Badge text (e.g., 'NEW', 'BETA', 'POPULAR')
    badge_text VARCHAR(50),
    -- Show feature as locked/teaser when disabled
    show_teaser_when_disabled BOOLEAN DEFAULT false,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_feature_descriptors_category ON feature_descriptors(category_id);
CREATE INDEX IF NOT EXISTS idx_feature_descriptors_order ON feature_descriptors(display_order);
CREATE INDEX IF NOT EXISTS idx_feature_descriptors_highlighted ON feature_descriptors(is_highlighted) WHERE is_highlighted = true;

-- ==================== Story 107.3: User Type Feature Matrix ====================

-- Access state enum
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'feature_access_state') THEN
        CREATE TYPE feature_access_state AS ENUM ('included', 'optional', 'excluded');
    END IF;
END $$;

CREATE TABLE IF NOT EXISTS feature_user_type_access (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    feature_flag_id UUID NOT NULL REFERENCES feature_flags(id) ON DELETE CASCADE,
    -- User type (e.g., 'owner', 'tenant', 'manager', 'org_admin', 'building_admin')
    user_type VARCHAR(50) NOT NULL,

    -- Access state for this user type
    access_state feature_access_state NOT NULL DEFAULT 'excluded',

    -- Override settings
    -- Can user change their preference for 'optional' features
    can_override BOOLEAN DEFAULT false,
    -- Default enabled state for 'optional' features
    default_enabled BOOLEAN DEFAULT true,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Each feature can only have one access rule per user type
    UNIQUE(feature_flag_id, user_type)
);

CREATE INDEX IF NOT EXISTS idx_feature_user_type_access_type ON feature_user_type_access(user_type);
CREATE INDEX IF NOT EXISTS idx_feature_user_type_access_state ON feature_user_type_access(access_state);

-- ==================== Story 107.5: User Feature Preferences ====================

CREATE TABLE IF NOT EXISTS user_feature_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    feature_flag_id UUID NOT NULL REFERENCES feature_flags(id) ON DELETE CASCADE,

    -- User's preference for optional features
    is_enabled BOOLEAN NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Each user can only have one preference per feature
    UNIQUE(user_id, feature_flag_id)
);

CREATE INDEX IF NOT EXISTS idx_user_feature_preferences_user ON user_feature_preferences(user_id);

-- ==================== Seed Initial Categories ====================

INSERT INTO feature_categories (key, name, description, icon, color, display_order) VALUES
    ('core', 'Core Features', 'Essential property management functionality', 'building', '#6366F1', 1),
    ('communication', 'Communication', 'Messaging, announcements, and notifications', 'message-circle', '#8B5CF6', 2),
    ('financial', 'Financial Management', 'Invoices, payments, and budgets', 'wallet', '#22C55E', 3),
    ('maintenance', 'Maintenance & Repairs', 'Faults, work orders, and preventive maintenance', 'wrench', '#F97316', 4),
    ('documents', 'Document Management', 'Storage, sharing, and document workflows', 'file-text', '#3B82F6', 5),
    ('ai_features', 'AI & Automation', 'AI-powered features and automation', 'sparkles', '#EC4899', 6),
    ('analytics', 'Analytics & Reports', 'Insights, dashboards, and reporting', 'chart-bar', '#14B8A6', 7),
    ('integrations', 'Integrations', 'Third-party integrations and APIs', 'plug', '#64748B', 8),
    ('advanced', 'Advanced Features', 'Premium and enterprise functionality', 'crown', '#EAB308', 9)
ON CONFLICT (key) DO NOTHING;

-- ==================== Add Descriptors for Existing Feature Flags ====================

-- Get the category IDs for reference
DO $$
DECLARE
    cat_ai UUID;
    cat_core UUID;
    cat_analytics UUID;
BEGIN
    SELECT id INTO cat_ai FROM feature_categories WHERE key = 'ai_features';
    SELECT id INTO cat_core FROM feature_categories WHERE key = 'core';
    SELECT id INTO cat_analytics FROM feature_categories WHERE key = 'analytics';

    -- Add descriptors for existing feature flags
    INSERT INTO feature_descriptors (
        feature_flag_id,
        display_name,
        short_description,
        full_description,
        icon,
        category_id,
        tags,
        benefits,
        display_order
    )
    SELECT
        f.id,
        f.name,
        f.description,
        f.description,
        CASE f.key
            WHEN 'new_dashboard' THEN 'layout-dashboard'
            WHEN 'dark_mode' THEN 'moon'
            WHEN 'advanced_analytics' THEN 'chart-line'
            WHEN 'beta_features' THEN 'flask'
            WHEN 'ai_suggestions' THEN 'sparkles'
            ELSE 'flag'
        END,
        CASE f.key
            WHEN 'advanced_analytics' THEN cat_analytics
            WHEN 'ai_suggestions' THEN cat_ai
            ELSE cat_core
        END,
        CASE f.key
            WHEN 'new_dashboard' THEN '["ui", "dashboard"]'::jsonb
            WHEN 'dark_mode' THEN '["ui", "theme"]'::jsonb
            WHEN 'advanced_analytics' THEN '["analytics", "reports"]'::jsonb
            WHEN 'beta_features' THEN '["beta", "testing"]'::jsonb
            WHEN 'ai_suggestions' THEN '["ai", "automation"]'::jsonb
            ELSE '[]'::jsonb
        END,
        CASE f.key
            WHEN 'new_dashboard' THEN '["Improved navigation", "Modern design", "Better performance"]'::jsonb
            WHEN 'dark_mode' THEN '["Reduced eye strain", "Better for night use"]'::jsonb
            WHEN 'advanced_analytics' THEN '["Deep insights", "Custom reports", "Trend analysis"]'::jsonb
            WHEN 'ai_suggestions' THEN '["Faster triage", "Automated routing", "Improved accuracy"]'::jsonb
            ELSE '[]'::jsonb
        END,
        CASE f.key
            WHEN 'new_dashboard' THEN 1
            WHEN 'dark_mode' THEN 2
            WHEN 'advanced_analytics' THEN 3
            WHEN 'ai_suggestions' THEN 4
            WHEN 'beta_features' THEN 100
            ELSE 50
        END
    FROM feature_flags f
    WHERE NOT EXISTS (
        SELECT 1 FROM feature_descriptors fd WHERE fd.feature_flag_id = f.id
    );
END $$;

-- ==================== Comments ====================

COMMENT ON TABLE feature_categories IS 'Epic 107: Hierarchical feature categorization for UI grouping';
COMMENT ON TABLE feature_descriptors IS 'Epic 107: Rich metadata for feature flags (display info, marketing, technical)';
COMMENT ON TABLE feature_user_type_access IS 'Epic 107: User type-based feature access matrix';
COMMENT ON TABLE user_feature_preferences IS 'Epic 107: User preferences for optional features';
