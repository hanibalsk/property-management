-- Epic 2B, Story 2B.1: Buildings Table
-- Creates buildings table with Row-Level Security for tenant isolation

-- Buildings table
CREATE TABLE IF NOT EXISTS buildings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Organization (tenant) context
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Address
    street VARCHAR(255) NOT NULL,
    city VARCHAR(100) NOT NULL,
    postal_code VARCHAR(20) NOT NULL,
    country VARCHAR(100) NOT NULL DEFAULT 'Slovakia',

    -- Basic info
    name VARCHAR(255), -- Optional building name
    description TEXT,

    -- Building details
    year_built INTEGER CHECK (year_built >= 1800 AND year_built <= 2100),
    total_floors INTEGER NOT NULL DEFAULT 1 CHECK (total_floors >= 1),
    total_entrances INTEGER NOT NULL DEFAULT 1 CHECK (total_entrances >= 1),

    -- Amenities (JSONB array for flexibility)
    amenities JSONB NOT NULL DEFAULT '[]',
    -- Example: ["elevator", "parking", "garden", "playground", "gym"]

    -- Contacts (JSONB for building-specific contacts)
    contacts JSONB NOT NULL DEFAULT '[]',
    -- Example: [{"name": "Caretaker", "phone": "+421...", "email": "..."}]

    -- Settings (extensible configuration)
    settings JSONB NOT NULL DEFAULT '{}',

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'archived')),

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_buildings_organization_id ON buildings(organization_id);
CREATE INDEX IF NOT EXISTS idx_buildings_status ON buildings(status);
CREATE INDEX IF NOT EXISTS idx_buildings_city ON buildings(city);
CREATE INDEX IF NOT EXISTS idx_buildings_postal_code ON buildings(postal_code);

-- Full-text search for address
CREATE INDEX IF NOT EXISTS idx_buildings_address_search ON buildings
    USING GIN (to_tsvector('simple', street || ' ' || city));

-- Trigger for updated_at
CREATE TRIGGER update_buildings_updated_at
    BEFORE UPDATE ON buildings
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Enable Row Level Security
ALTER TABLE buildings ENABLE ROW LEVEL SECURITY;

-- RLS Policy: Buildings are visible to members of the same organization
CREATE POLICY buildings_tenant_isolation ON buildings
    FOR ALL
    USING (
        -- Super admins can see all
        is_super_admin()
        OR
        -- Users can see buildings in their organization
        organization_id = get_current_org_id()
    )
    WITH CHECK (
        -- Super admins can modify all
        is_super_admin()
        OR
        -- Users can only create/update buildings in their organization
        organization_id = get_current_org_id()
    );

-- Computed column for unit count (will be populated when units table exists)
COMMENT ON TABLE buildings IS 'Buildings managed by organizations (Epic 2B, UC-15)';
COMMENT ON COLUMN buildings.amenities IS 'JSONB array of amenity strings';
COMMENT ON COLUMN buildings.contacts IS 'JSONB array of contact objects {name, phone, email, role}';
