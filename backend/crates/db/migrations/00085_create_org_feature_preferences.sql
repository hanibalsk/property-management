-- Epic 110, Story 110.3: Organization Feature Preferences
-- Migration: Create organization feature preferences table
-- Allows org admins to toggle optional features for their organization

-- Create organization_feature_preferences table
CREATE TABLE IF NOT EXISTS organization_feature_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    feature_flag_id UUID NOT NULL REFERENCES feature_flags(id) ON DELETE CASCADE,
    is_enabled BOOLEAN NOT NULL,
    updated_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(organization_id, feature_flag_id)
);

-- Create indexes for efficient lookups
CREATE INDEX IF NOT EXISTS idx_org_feature_prefs_org_id ON organization_feature_preferences(organization_id);
CREATE INDEX IF NOT EXISTS idx_org_feature_prefs_flag_id ON organization_feature_preferences(feature_flag_id);

-- Add trigger for updated_at
CREATE OR REPLACE FUNCTION update_org_feature_preferences_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_org_feature_preferences_updated_at
    BEFORE UPDATE ON organization_feature_preferences
    FOR EACH ROW
    EXECUTE FUNCTION update_org_feature_preferences_updated_at();

-- Add RLS policies
ALTER TABLE organization_feature_preferences ENABLE ROW LEVEL SECURITY;

-- Policy: Users can view preferences for organizations they belong to
CREATE POLICY org_feature_prefs_select_policy ON organization_feature_preferences
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM organization_members
            WHERE organization_members.organization_id = organization_feature_preferences.organization_id
            AND organization_members.user_id = current_setting('app.user_id', true)::uuid
        )
    );

-- Policy: Only org admins can modify preferences
CREATE POLICY org_feature_prefs_modify_policy ON organization_feature_preferences
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            JOIN roles r ON om.role_id = r.id
            WHERE om.organization_id = organization_feature_preferences.organization_id
            AND om.user_id = current_setting('app.user_id', true)::uuid
            AND r.permissions::jsonb ? 'organization:manage_features'
        )
    );

-- Add comment for documentation
COMMENT ON TABLE organization_feature_preferences IS 'Organization-level feature flag preferences allowing org admins to toggle optional features (Epic 110, Story 110.3)';
COMMENT ON COLUMN organization_feature_preferences.is_enabled IS 'Whether this feature is enabled for this organization (overrides global setting if feature allows org-level control)';
COMMENT ON COLUMN organization_feature_preferences.updated_by IS 'User who last modified this preference';
