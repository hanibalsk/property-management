-- Epic 2B, Story 2B.2: Units Table
-- Creates units table with Row-Level Security for tenant isolation

-- Unit types enum
-- apartment: Residential apartment
-- commercial: Commercial/office space
-- parking: Parking space
-- storage: Storage unit/cellar
-- other: Other type

-- Units table
CREATE TABLE IF NOT EXISTS units (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Building reference
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,

    -- Entrance (optional, for multi-entrance buildings)
    entrance VARCHAR(50), -- e.g., "A", "1", "Main"

    -- Unit identification
    designation VARCHAR(50) NOT NULL, -- e.g., "3B", "101", "P-15"
    floor INTEGER NOT NULL DEFAULT 0, -- 0 = ground floor, negative for basement

    -- Unit details
    unit_type VARCHAR(20) NOT NULL DEFAULT 'apartment'
        CHECK (unit_type IN ('apartment', 'commercial', 'parking', 'storage', 'other')),
    size_sqm NUMERIC(10, 2), -- Size in square meters
    rooms INTEGER CHECK (rooms >= 0), -- Number of rooms (nullable for non-residential)

    -- Ownership
    ownership_share NUMERIC(5, 2) NOT NULL DEFAULT 100.00
        CHECK (ownership_share >= 0 AND ownership_share <= 100),
    -- Ownership share as percentage of building (for voting rights, costs)

    -- Current occupancy status
    occupancy_status VARCHAR(20) NOT NULL DEFAULT 'unknown'
        CHECK (occupancy_status IN ('owner_occupied', 'rented', 'vacant', 'unknown')),

    -- Additional info
    description TEXT,
    notes TEXT, -- Internal notes for managers

    -- Settings (extensible configuration)
    settings JSONB NOT NULL DEFAULT '{}',

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'active'
        CHECK (status IN ('active', 'archived')),

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Unique constraint: no duplicate designations within the same building
    CONSTRAINT unique_unit_per_building UNIQUE (building_id, designation)
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_units_building_id ON units(building_id);
CREATE INDEX IF NOT EXISTS idx_units_floor ON units(floor);
CREATE INDEX IF NOT EXISTS idx_units_unit_type ON units(unit_type);
CREATE INDEX IF NOT EXISTS idx_units_status ON units(status);
CREATE INDEX IF NOT EXISTS idx_units_occupancy_status ON units(occupancy_status);

-- Trigger for updated_at
CREATE TRIGGER update_units_updated_at
    BEFORE UPDATE ON units
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Enable Row Level Security
ALTER TABLE units ENABLE ROW LEVEL SECURITY;

-- RLS Policy: Units are visible to members of the building's organization
CREATE POLICY units_tenant_isolation ON units
    FOR ALL
    USING (
        -- Super admins can see all
        is_super_admin()
        OR
        -- Users can see units in buildings within their organization
        EXISTS (
            SELECT 1 FROM buildings b
            WHERE b.id = units.building_id
            AND b.organization_id = get_current_org_id()
        )
    )
    WITH CHECK (
        -- Super admins can modify all
        is_super_admin()
        OR
        -- Users can only create/update units in their organization's buildings
        EXISTS (
            SELECT 1 FROM buildings b
            WHERE b.id = units.building_id
            AND b.organization_id = get_current_org_id()
        )
    );

-- Unit-Owner assignments table (UC-15.6)
CREATE TABLE IF NOT EXISTS unit_owners (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- References
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Ownership details
    ownership_percentage NUMERIC(5, 2) NOT NULL DEFAULT 100.00
        CHECK (ownership_percentage > 0 AND ownership_percentage <= 100),
    is_primary BOOLEAN NOT NULL DEFAULT TRUE, -- Primary contact for this unit

    -- Validity period (for tracking historical ownership)
    valid_from DATE NOT NULL DEFAULT CURRENT_DATE,
    valid_until DATE, -- NULL means currently active

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'active'
        CHECK (status IN ('active', 'inactive')),

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Ensure no overlapping active ownerships exceed 100%
    -- (enforced at application level for complex scenarios)
    CONSTRAINT unique_owner_per_unit UNIQUE (unit_id, user_id, valid_from)
);

-- Indexes for unit_owners
CREATE INDEX IF NOT EXISTS idx_unit_owners_unit_id ON unit_owners(unit_id);
CREATE INDEX IF NOT EXISTS idx_unit_owners_user_id ON unit_owners(user_id);
CREATE INDEX IF NOT EXISTS idx_unit_owners_status ON unit_owners(status);
CREATE INDEX IF NOT EXISTS idx_unit_owners_valid ON unit_owners(valid_from, valid_until);

-- Trigger for unit_owners updated_at
CREATE TRIGGER update_unit_owners_updated_at
    BEFORE UPDATE ON unit_owners
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Enable RLS on unit_owners
ALTER TABLE unit_owners ENABLE ROW LEVEL SECURITY;

-- RLS Policy: Unit owners visible to organization members
CREATE POLICY unit_owners_tenant_isolation ON unit_owners
    FOR ALL
    USING (
        is_super_admin()
        OR
        EXISTS (
            SELECT 1 FROM units u
            JOIN buildings b ON b.id = u.building_id
            WHERE u.id = unit_owners.unit_id
            AND b.organization_id = get_current_org_id()
        )
    )
    WITH CHECK (
        is_super_admin()
        OR
        EXISTS (
            SELECT 1 FROM units u
            JOIN buildings b ON b.id = u.building_id
            WHERE u.id = unit_owners.unit_id
            AND b.organization_id = get_current_org_id()
        )
    );

COMMENT ON TABLE units IS 'Units (apartments, etc.) within buildings (Epic 2B, UC-15)';
COMMENT ON TABLE unit_owners IS 'Ownership assignments for units (UC-15.6)';
COMMENT ON COLUMN units.ownership_share IS 'Percentage share of building ownership (voting rights)';
COMMENT ON COLUMN unit_owners.ownership_percentage IS 'Percentage of unit owned by this user';
