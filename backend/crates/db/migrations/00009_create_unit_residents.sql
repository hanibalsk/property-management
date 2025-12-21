-- Migration: Create unit_residents table
-- Epic 3, Story 3.3: Resident Association with Units

-- Resident type enum
CREATE TYPE resident_type AS ENUM (
    'owner',           -- Property owner
    'tenant',          -- Renting tenant
    'family_member',   -- Family member of owner/tenant
    'subtenant'        -- Subletting from tenant
);

-- Unit residents table - tracks who lives in each unit
CREATE TABLE unit_residents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- References
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Resident info
    resident_type resident_type NOT NULL,
    is_primary BOOLEAN NOT NULL DEFAULT FALSE,  -- Primary contact for the unit

    -- Validity period
    start_date DATE NOT NULL DEFAULT CURRENT_DATE,
    end_date DATE,  -- NULL means current resident

    -- Contact preferences
    receives_notifications BOOLEAN NOT NULL DEFAULT TRUE,
    receives_mail BOOLEAN NOT NULL DEFAULT TRUE,

    -- Metadata
    notes TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id),

    -- Constraints
    CONSTRAINT valid_date_range CHECK (end_date IS NULL OR end_date >= start_date),
    CONSTRAINT unique_active_resident UNIQUE (unit_id, user_id, start_date)
);

-- Indexes
CREATE INDEX idx_unit_residents_unit ON unit_residents(unit_id);
CREATE INDEX idx_unit_residents_user ON unit_residents(user_id);
CREATE INDEX idx_unit_residents_active ON unit_residents(unit_id)
    WHERE end_date IS NULL;
CREATE INDEX idx_unit_residents_type ON unit_residents(resident_type);

-- Update trigger
CREATE TRIGGER update_unit_residents_updated_at
    BEFORE UPDATE ON unit_residents
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Row Level Security
ALTER TABLE unit_residents ENABLE ROW LEVEL SECURITY;

-- RLS Policies
-- Users can see residents in units of their organization's buildings
CREATE POLICY unit_residents_select_org ON unit_residents
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM units u
            JOIN buildings b ON u.building_id = b.id
            WHERE u.id = unit_residents.unit_id
            AND b.organization_id = current_setting('app.current_org_id', true)::UUID
        )
        OR user_id = current_setting('app.current_user_id', true)::UUID
    );

-- Managers can insert residents
CREATE POLICY unit_residents_insert_manager ON unit_residents
    FOR INSERT
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM units u
            JOIN buildings b ON u.building_id = b.id
            JOIN organization_members om ON om.organization_id = b.organization_id
            WHERE u.id = unit_residents.unit_id
            AND om.user_id = current_setting('app.current_user_id', true)::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
    );

-- Managers can update residents
CREATE POLICY unit_residents_update_manager ON unit_residents
    FOR UPDATE
    USING (
        EXISTS (
            SELECT 1 FROM units u
            JOIN buildings b ON u.building_id = b.id
            JOIN organization_members om ON om.organization_id = b.organization_id
            WHERE u.id = unit_residents.unit_id
            AND om.user_id = current_setting('app.current_user_id', true)::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
    );

-- Managers can delete residents
CREATE POLICY unit_residents_delete_manager ON unit_residents
    FOR DELETE
    USING (
        EXISTS (
            SELECT 1 FROM units u
            JOIN buildings b ON u.building_id = b.id
            JOIN organization_members om ON om.organization_id = b.organization_id
            WHERE u.id = unit_residents.unit_id
            AND om.user_id = current_setting('app.current_user_id', true)::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
    );

-- Super admin bypass
CREATE POLICY unit_residents_super_admin ON unit_residents
    FOR ALL
    USING (current_setting('app.is_super_admin', true)::BOOLEAN = TRUE);

-- Helper function to get active residents for a unit
CREATE OR REPLACE FUNCTION get_active_residents(p_unit_id UUID)
RETURNS TABLE (
    id UUID,
    user_id UUID,
    resident_type resident_type,
    is_primary BOOLEAN,
    start_date DATE
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        ur.id,
        ur.user_id,
        ur.resident_type,
        ur.is_primary,
        ur.start_date
    FROM unit_residents ur
    WHERE ur.unit_id = p_unit_id
      AND ur.end_date IS NULL
    ORDER BY ur.is_primary DESC, ur.start_date;
END;
$$ LANGUAGE plpgsql;

-- Helper function to count residents for person-month calculation
CREATE OR REPLACE FUNCTION count_residents_for_month(
    p_unit_id UUID,
    p_year INTEGER,
    p_month INTEGER
)
RETURNS INTEGER AS $$
DECLARE
    month_start DATE;
    month_end DATE;
BEGIN
    month_start := make_date(p_year, p_month, 1);
    month_end := (month_start + INTERVAL '1 month' - INTERVAL '1 day')::DATE;

    RETURN (
        SELECT COUNT(DISTINCT user_id)::INTEGER
        FROM unit_residents
        WHERE unit_id = p_unit_id
          AND start_date <= month_end
          AND (end_date IS NULL OR end_date >= month_start)
    );
END;
$$ LANGUAGE plpgsql;
