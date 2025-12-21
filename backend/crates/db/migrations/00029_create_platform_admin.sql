-- Epic 10B: Platform Administration
-- Migration: Create platform admin infrastructure

-- Add suspension tracking columns to organizations if not exist
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name = 'organizations' AND column_name = 'suspended_at') THEN
        ALTER TABLE organizations ADD COLUMN suspended_at TIMESTAMPTZ;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name = 'organizations' AND column_name = 'suspended_by') THEN
        ALTER TABLE organizations ADD COLUMN suspended_by UUID REFERENCES users(id);
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name = 'organizations' AND column_name = 'suspension_reason') THEN
        ALTER TABLE organizations ADD COLUMN suspension_reason TEXT;
    END IF;
END $$;

-- Create organization metrics view for admin dashboard
CREATE OR REPLACE VIEW organization_metrics AS
SELECT
    o.id as organization_id,
    o.name,
    o.slug,
    o.status,
    o.created_at,
    o.updated_at,
    o.suspended_at,
    o.suspended_by,
    o.suspension_reason,
    COALESCE(member_counts.member_count, 0) as member_count,
    COALESCE(member_counts.active_member_count, 0) as active_member_count,
    COALESCE(building_counts.building_count, 0) as building_count,
    COALESCE(unit_counts.unit_count, 0) as unit_count
FROM organizations o
LEFT JOIN (
    SELECT
        organization_id,
        COUNT(*) as member_count,
        COUNT(*) FILTER (WHERE status = 'active') as active_member_count
    FROM organization_members
    GROUP BY organization_id
) member_counts ON o.id = member_counts.organization_id
LEFT JOIN (
    SELECT
        organization_id,
        COUNT(*) as building_count
    FROM buildings
    WHERE deleted_at IS NULL
    GROUP BY organization_id
) building_counts ON o.id = building_counts.organization_id
LEFT JOIN (
    SELECT
        b.organization_id,
        COUNT(u.id) as unit_count
    FROM buildings b
    LEFT JOIN units u ON u.building_id = b.id AND u.deleted_at IS NULL
    WHERE b.deleted_at IS NULL
    GROUP BY b.organization_id
) unit_counts ON o.id = unit_counts.organization_id
WHERE o.status != 'deleted';

-- Create index for efficient admin queries
CREATE INDEX IF NOT EXISTS idx_organizations_status ON organizations(status);
CREATE INDEX IF NOT EXISTS idx_organizations_created_at ON organizations(created_at DESC);

-- Add comment for documentation
COMMENT ON VIEW organization_metrics IS 'Aggregated organization metrics for platform admin dashboard (Epic 10B)';
