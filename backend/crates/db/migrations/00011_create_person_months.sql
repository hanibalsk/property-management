-- Migration: Create person_months table
-- Epic 3, Story 3.5: Person-Month Tracking

-- Person-month source enum
CREATE TYPE person_month_source AS ENUM (
    'manual',        -- Manually entered by manager
    'calculated',    -- Auto-calculated from residents
    'imported'       -- Imported from external system
);

-- Person months table - tracks occupancy for fee allocation
CREATE TABLE person_months (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Reference
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,

    -- Time period
    year INTEGER NOT NULL,
    month INTEGER NOT NULL CHECK (month >= 1 AND month <= 12),

    -- Count
    count INTEGER NOT NULL CHECK (count >= 0),

    -- Source tracking
    source person_month_source NOT NULL DEFAULT 'manual',

    -- Notes
    notes TEXT,

    -- Audit
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),

    -- Unique constraint - one entry per unit per month
    CONSTRAINT unique_unit_month UNIQUE (unit_id, year, month)
);

-- Indexes
CREATE INDEX idx_person_months_unit ON person_months(unit_id);
CREATE INDEX idx_person_months_period ON person_months(year, month);
CREATE INDEX idx_person_months_unit_year ON person_months(unit_id, year);

-- Update trigger
CREATE TRIGGER update_person_months_updated_at
    BEFORE UPDATE ON person_months
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Row Level Security
ALTER TABLE person_months ENABLE ROW LEVEL SECURITY;

-- RLS Policies
-- Users can see person-months for units in their organization
CREATE POLICY person_months_select_org ON person_months
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM units u
            JOIN buildings b ON u.building_id = b.id
            WHERE u.id = person_months.unit_id
            AND b.organization_id = current_setting('app.current_org_id', true)::UUID
        )
    );

-- Managers can insert person-months
CREATE POLICY person_months_insert_manager ON person_months
    FOR INSERT
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM units u
            JOIN buildings b ON u.building_id = b.id
            JOIN organization_members om ON om.organization_id = b.organization_id
            WHERE u.id = person_months.unit_id
            AND om.user_id = current_setting('app.current_user_id', true)::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
    );

-- Managers can update person-months
CREATE POLICY person_months_update_manager ON person_months
    FOR UPDATE
    USING (
        EXISTS (
            SELECT 1 FROM units u
            JOIN buildings b ON u.building_id = b.id
            JOIN organization_members om ON om.organization_id = b.organization_id
            WHERE u.id = person_months.unit_id
            AND om.user_id = current_setting('app.current_user_id', true)::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
    );

-- Managers can delete person-months
CREATE POLICY person_months_delete_manager ON person_months
    FOR DELETE
    USING (
        EXISTS (
            SELECT 1 FROM units u
            JOIN buildings b ON u.building_id = b.id
            JOIN organization_members om ON om.organization_id = b.organization_id
            WHERE u.id = person_months.unit_id
            AND om.user_id = current_setting('app.current_user_id', true)::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
    );

-- Super admin bypass
CREATE POLICY person_months_super_admin ON person_months
    FOR ALL
    USING (current_setting('app.is_super_admin', true)::BOOLEAN = TRUE);

-- Building-level aggregation view
CREATE OR REPLACE VIEW building_person_months AS
SELECT
    b.id AS building_id,
    b.organization_id,
    pm.year,
    pm.month,
    SUM(pm.count) AS total_count,
    COUNT(pm.id) AS unit_count
FROM buildings b
JOIN units u ON u.building_id = b.id
LEFT JOIN person_months pm ON pm.unit_id = u.id
GROUP BY b.id, b.organization_id, pm.year, pm.month;

-- Function to get or calculate person-months for a unit
CREATE OR REPLACE FUNCTION get_person_month(
    p_unit_id UUID,
    p_year INTEGER,
    p_month INTEGER
)
RETURNS TABLE (
    count INTEGER,
    source person_month_source,
    is_calculated BOOLEAN
) AS $$
DECLARE
    v_existing RECORD;
    v_calculated INTEGER;
BEGIN
    -- Check for existing entry
    SELECT pm.count, pm.source
    INTO v_existing
    FROM person_months pm
    WHERE pm.unit_id = p_unit_id
      AND pm.year = p_year
      AND pm.month = p_month;

    IF FOUND THEN
        RETURN QUERY SELECT v_existing.count, v_existing.source, FALSE;
    ELSE
        -- Calculate from residents
        v_calculated := count_residents_for_month(p_unit_id, p_year, p_month);
        RETURN QUERY SELECT v_calculated, 'calculated'::person_month_source, TRUE;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Function to bulk upsert person-months for a building
CREATE OR REPLACE FUNCTION upsert_person_months(
    p_unit_id UUID,
    p_year INTEGER,
    p_month INTEGER,
    p_count INTEGER,
    p_source person_month_source DEFAULT 'manual',
    p_user_id UUID DEFAULT NULL
)
RETURNS UUID AS $$
DECLARE
    v_id UUID;
BEGIN
    INSERT INTO person_months (unit_id, year, month, count, source, created_by, updated_by)
    VALUES (p_unit_id, p_year, p_month, p_count, p_source, p_user_id, p_user_id)
    ON CONFLICT (unit_id, year, month)
    DO UPDATE SET
        count = EXCLUDED.count,
        source = EXCLUDED.source,
        updated_by = EXCLUDED.updated_by,
        updated_at = NOW()
    RETURNING id INTO v_id;

    RETURN v_id;
END;
$$ LANGUAGE plpgsql;

-- Function to get yearly summary for a unit
CREATE OR REPLACE FUNCTION get_yearly_person_months(
    p_unit_id UUID,
    p_year INTEGER
)
RETURNS TABLE (
    month INTEGER,
    count INTEGER,
    source person_month_source
) AS $$
BEGIN
    RETURN QUERY
    SELECT m.month, COALESCE(pm.count, 0), COALESCE(pm.source, 'calculated'::person_month_source)
    FROM generate_series(1, 12) AS m(month)
    LEFT JOIN person_months pm ON pm.unit_id = p_unit_id
        AND pm.year = p_year
        AND pm.month = m.month
    ORDER BY m.month;
END;
$$ LANGUAGE plpgsql;
