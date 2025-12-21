-- Migration: Create facilities and facility_bookings tables
-- Epic 3, Story 3.7: Common Areas & Facilities

-- Facility type enum
CREATE TYPE facility_type AS ENUM (
    'gym',              -- Fitness room
    'laundry',          -- Laundry room
    'meeting_room',     -- Meeting/conference room
    'party_room',       -- Party/event room
    'sauna',            -- Sauna/wellness
    'pool',             -- Swimming pool
    'playground',       -- Children's playground
    'parking',          -- Parking space
    'storage',          -- Storage room
    'garden',           -- Garden/terrace
    'bbq',              -- BBQ area
    'bike_storage',     -- Bicycle storage
    'other'             -- Other facility
);

-- Booking status enum
CREATE TYPE booking_status AS ENUM (
    'pending',          -- Awaiting approval
    'approved',         -- Approved, confirmed
    'rejected',         -- Rejected by manager
    'cancelled',        -- Cancelled by user
    'completed',        -- Booking completed
    'no_show'           -- User didn't show up
);

-- Facilities table
CREATE TABLE facilities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Reference
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,

    -- Basic info
    name VARCHAR(255) NOT NULL,
    facility_type facility_type NOT NULL,
    description TEXT,
    location VARCHAR(255),  -- e.g., "Basement level 1", "Rooftop"

    -- Capacity
    capacity INTEGER CHECK (capacity IS NULL OR capacity > 0),

    -- Booking settings
    is_bookable BOOLEAN NOT NULL DEFAULT FALSE,
    requires_approval BOOLEAN NOT NULL DEFAULT FALSE,
    max_booking_hours INTEGER DEFAULT 4,
    max_advance_days INTEGER DEFAULT 30,  -- How far in advance can book
    min_advance_hours INTEGER DEFAULT 1,  -- Minimum hours before booking

    -- Availability hours (NULL means 24/7 or follow building hours)
    available_from TIME,
    available_to TIME,
    available_days INTEGER[] DEFAULT '{1,2,3,4,5,6,0}',  -- 0=Sunday, 1=Monday, etc.

    -- Rules and fees
    rules TEXT,
    hourly_fee DECIMAL(10,2),
    deposit_amount DECIMAL(10,2),

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT TRUE,

    -- Metadata
    photos JSONB DEFAULT '[]',
    amenities JSONB DEFAULT '[]',  -- ["wifi", "projector", "kitchen"]

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Facility bookings table
CREATE TABLE facility_bookings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- References
    facility_id UUID NOT NULL REFERENCES facilities(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    unit_id UUID REFERENCES units(id) ON DELETE SET NULL,  -- Optional: which unit is booking

    -- Booking time
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,

    -- Status
    status booking_status NOT NULL DEFAULT 'pending',

    -- Details
    purpose TEXT,
    attendees INTEGER,
    notes TEXT,

    -- Approval tracking
    approved_by UUID REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    rejected_by UUID REFERENCES users(id),
    rejected_at TIMESTAMPTZ,
    rejection_reason TEXT,

    -- Cancellation
    cancelled_at TIMESTAMPTZ,
    cancellation_reason TEXT,

    -- Fees
    total_fee DECIMAL(10,2),
    deposit_paid BOOLEAN DEFAULT FALSE,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraints
    CONSTRAINT valid_booking_time CHECK (end_time > start_time),
    CONSTRAINT reasonable_duration CHECK (
        end_time - start_time <= INTERVAL '24 hours'
    )
);

-- Indexes for facilities
CREATE INDEX idx_facilities_building ON facilities(building_id);
CREATE INDEX idx_facilities_type ON facilities(facility_type);
CREATE INDEX idx_facilities_active ON facilities(building_id) WHERE is_active;
CREATE INDEX idx_facilities_bookable ON facilities(building_id) WHERE is_bookable;

-- Indexes for bookings
CREATE INDEX idx_facility_bookings_facility ON facility_bookings(facility_id);
CREATE INDEX idx_facility_bookings_user ON facility_bookings(user_id);
CREATE INDEX idx_facility_bookings_time ON facility_bookings(facility_id, start_time, end_time);
CREATE INDEX idx_facility_bookings_status ON facility_bookings(status);
CREATE INDEX idx_facility_bookings_pending ON facility_bookings(facility_id)
    WHERE status = 'pending';

-- Update triggers
CREATE TRIGGER update_facilities_updated_at
    BEFORE UPDATE ON facilities
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_facility_bookings_updated_at
    BEFORE UPDATE ON facility_bookings
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Row Level Security
ALTER TABLE facilities ENABLE ROW LEVEL SECURITY;
ALTER TABLE facility_bookings ENABLE ROW LEVEL SECURITY;

-- RLS Policies for facilities
CREATE POLICY facilities_select_org ON facilities
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM buildings b
            WHERE b.id = facilities.building_id
            AND b.organization_id = current_setting('app.current_org_id', true)::UUID
        )
    );

CREATE POLICY facilities_insert_manager ON facilities
    FOR INSERT
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM buildings b
            JOIN organization_members om ON om.organization_id = b.organization_id
            WHERE b.id = facilities.building_id
            AND om.user_id = current_setting('app.current_user_id', true)::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
    );

CREATE POLICY facilities_update_manager ON facilities
    FOR UPDATE
    USING (
        EXISTS (
            SELECT 1 FROM buildings b
            JOIN organization_members om ON om.organization_id = b.organization_id
            WHERE b.id = facilities.building_id
            AND om.user_id = current_setting('app.current_user_id', true)::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
    );

CREATE POLICY facilities_delete_manager ON facilities
    FOR DELETE
    USING (
        EXISTS (
            SELECT 1 FROM buildings b
            JOIN organization_members om ON om.organization_id = b.organization_id
            WHERE b.id = facilities.building_id
            AND om.user_id = current_setting('app.current_user_id', true)::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
    );

CREATE POLICY facilities_super_admin ON facilities
    FOR ALL
    USING (current_setting('app.is_super_admin', true)::BOOLEAN = TRUE);

-- RLS Policies for bookings
CREATE POLICY facility_bookings_select_org ON facility_bookings
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM facilities f
            JOIN buildings b ON b.id = f.building_id
            WHERE f.id = facility_bookings.facility_id
            AND b.organization_id = current_setting('app.current_org_id', true)::UUID
        )
        OR user_id = current_setting('app.current_user_id', true)::UUID
    );

CREATE POLICY facility_bookings_insert_resident ON facility_bookings
    FOR INSERT
    WITH CHECK (
        user_id = current_setting('app.current_user_id', true)::UUID
        AND EXISTS (
            SELECT 1 FROM facilities f
            JOIN buildings b ON b.id = f.building_id
            WHERE f.id = facility_bookings.facility_id
            AND b.organization_id = current_setting('app.current_org_id', true)::UUID
        )
    );

CREATE POLICY facility_bookings_update_own ON facility_bookings
    FOR UPDATE
    USING (
        user_id = current_setting('app.current_user_id', true)::UUID
        OR EXISTS (
            SELECT 1 FROM facilities f
            JOIN buildings b ON b.id = f.building_id
            JOIN organization_members om ON om.organization_id = b.organization_id
            WHERE f.id = facility_bookings.facility_id
            AND om.user_id = current_setting('app.current_user_id', true)::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
    );

CREATE POLICY facility_bookings_delete_own ON facility_bookings
    FOR DELETE
    USING (
        user_id = current_setting('app.current_user_id', true)::UUID
        AND status IN ('pending', 'approved')
    );

CREATE POLICY facility_bookings_super_admin ON facility_bookings
    FOR ALL
    USING (current_setting('app.is_super_admin', true)::BOOLEAN = TRUE);

-- Function to check availability
CREATE OR REPLACE FUNCTION check_facility_availability(
    p_facility_id UUID,
    p_start_time TIMESTAMPTZ,
    p_end_time TIMESTAMPTZ,
    p_exclude_booking_id UUID DEFAULT NULL
)
RETURNS BOOLEAN AS $$
BEGIN
    RETURN NOT EXISTS (
        SELECT 1 FROM facility_bookings
        WHERE facility_id = p_facility_id
          AND status IN ('pending', 'approved')
          AND (p_exclude_booking_id IS NULL OR id != p_exclude_booking_id)
          AND start_time < p_end_time
          AND end_time > p_start_time
    );
END;
$$ LANGUAGE plpgsql;

-- Function to get available time slots for a facility on a given day
CREATE OR REPLACE FUNCTION get_available_slots(
    p_facility_id UUID,
    p_date DATE
)
RETURNS TABLE (
    slot_start TIMESTAMPTZ,
    slot_end TIMESTAMPTZ
) AS $$
DECLARE
    v_facility RECORD;
    v_day_start TIMESTAMPTZ;
    v_day_end TIMESTAMPTZ;
BEGIN
    -- Get facility info
    SELECT * INTO v_facility FROM facilities WHERE id = p_facility_id;

    IF NOT FOUND OR NOT v_facility.is_bookable THEN
        RETURN;
    END IF;

    -- Calculate day boundaries
    IF v_facility.available_from IS NOT NULL AND v_facility.available_to IS NOT NULL THEN
        v_day_start := p_date + v_facility.available_from;
        v_day_end := p_date + v_facility.available_to;
    ELSE
        v_day_start := p_date::TIMESTAMPTZ;
        v_day_end := (p_date + INTERVAL '1 day')::TIMESTAMPTZ;
    END IF;

    -- Return hourly slots that are available
    RETURN QUERY
    WITH hours AS (
        SELECT generate_series(v_day_start, v_day_end - INTERVAL '1 hour', INTERVAL '1 hour') AS slot_start
    )
    SELECT
        h.slot_start,
        h.slot_start + INTERVAL '1 hour' AS slot_end
    FROM hours h
    WHERE check_facility_availability(p_facility_id, h.slot_start, h.slot_start + INTERVAL '1 hour');
END;
$$ LANGUAGE plpgsql;
