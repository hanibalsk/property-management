-- Epic 18: Short-Term Rental Integration
-- Migration: Create short-term rental platform connections, bookings, and guest registration

-- ============================================
-- Rental Platform Type (Story 18.1)
-- ============================================

CREATE TYPE rental_platform AS ENUM ('airbnb', 'booking', 'vrbo', 'direct');

-- ============================================
-- Booking Status
-- ============================================

CREATE TYPE rental_booking_status AS ENUM (
    'pending',
    'confirmed',
    'checked_in',
    'checked_out',
    'cancelled',
    'no_show'
);

-- ============================================
-- Guest Registration Status
-- ============================================

CREATE TYPE guest_registration_status AS ENUM (
    'pending',
    'registered',
    'reported',
    'expired'
);

-- ============================================
-- Platform Connections (Story 18.1)
-- ============================================

CREATE TABLE rental_platform_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    platform rental_platform NOT NULL,

    -- OAuth credentials (encrypted at application level)
    access_token TEXT,
    refresh_token TEXT,
    token_expires_at TIMESTAMPTZ,

    -- External identifiers
    external_property_id VARCHAR(255),
    external_listing_url VARCHAR(1000),

    -- Connection status
    is_active BOOLEAN NOT NULL DEFAULT true,
    last_sync_at TIMESTAMPTZ,
    sync_error TEXT,

    -- Calendar sync settings
    sync_calendar BOOLEAN NOT NULL DEFAULT true,
    sync_interval_minutes INTEGER NOT NULL DEFAULT 15,
    block_other_platforms BOOLEAN NOT NULL DEFAULT true,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- One connection per platform per unit
    UNIQUE(unit_id, platform)
);

-- Indexes for rental_platform_connections
CREATE INDEX idx_rental_platform_connections_org ON rental_platform_connections(organization_id);
CREATE INDEX idx_rental_platform_connections_unit ON rental_platform_connections(unit_id);
CREATE INDEX idx_rental_platform_connections_platform ON rental_platform_connections(platform);
CREATE INDEX idx_rental_platform_connections_active ON rental_platform_connections(is_active) WHERE is_active = true;
CREATE INDEX idx_rental_platform_connections_sync ON rental_platform_connections(last_sync_at) WHERE is_active = true;

-- ============================================
-- Rental Bookings (Story 18.2)
-- ============================================

CREATE TABLE rental_bookings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    connection_id UUID REFERENCES rental_platform_connections(id) ON DELETE SET NULL,

    -- Platform info
    platform rental_platform NOT NULL,
    external_booking_id VARCHAR(255),
    external_booking_url VARCHAR(1000),

    -- Guest info (basic)
    guest_name VARCHAR(255) NOT NULL,
    guest_email VARCHAR(255),
    guest_phone VARCHAR(50),
    guest_count INTEGER NOT NULL DEFAULT 1,

    -- Booking dates
    check_in DATE NOT NULL,
    check_out DATE NOT NULL,
    check_in_time TIME DEFAULT '14:00',
    check_out_time TIME DEFAULT '10:00',

    -- Financial
    total_amount DECIMAL(12, 2),
    currency VARCHAR(3) DEFAULT 'EUR',
    platform_fee DECIMAL(12, 2),
    cleaning_fee DECIMAL(12, 2),

    -- Status
    status rental_booking_status NOT NULL DEFAULT 'pending',
    cancelled_at TIMESTAMPTZ,
    cancellation_reason TEXT,

    -- Notes
    guest_notes TEXT,
    internal_notes TEXT,

    -- Sync metadata
    synced_at TIMESTAMPTZ,
    raw_data JSONB,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Check dates are valid
    CONSTRAINT check_booking_dates CHECK (check_out > check_in),
    -- External booking ID unique per platform
    CONSTRAINT unique_external_booking UNIQUE NULLS NOT DISTINCT (platform, external_booking_id)
);

-- Indexes for rental_bookings
CREATE INDEX idx_rental_bookings_org ON rental_bookings(organization_id);
CREATE INDEX idx_rental_bookings_unit ON rental_bookings(unit_id);
CREATE INDEX idx_rental_bookings_connection ON rental_bookings(connection_id);
CREATE INDEX idx_rental_bookings_platform ON rental_bookings(platform);
CREATE INDEX idx_rental_bookings_status ON rental_bookings(status);
CREATE INDEX idx_rental_bookings_dates ON rental_bookings(check_in, check_out);
CREATE INDEX idx_rental_bookings_checkin ON rental_bookings(check_in);
CREATE INDEX idx_rental_bookings_external ON rental_bookings(platform, external_booking_id);

-- ============================================
-- Guest Registration (Story 18.3)
-- ============================================

CREATE TABLE rental_guests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    booking_id UUID NOT NULL REFERENCES rental_bookings(id) ON DELETE CASCADE,

    -- Personal info (for legal compliance)
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    date_of_birth DATE,
    nationality VARCHAR(3),  -- ISO 3166-1 alpha-3

    -- Identification
    id_type VARCHAR(50),  -- passport, national_id, driving_license
    id_number VARCHAR(100),
    id_issuing_country VARCHAR(3),
    id_expiry_date DATE,
    id_document_url VARCHAR(1000),  -- Stored in S3

    -- Contact
    email VARCHAR(255),
    phone VARCHAR(50),

    -- Address
    address_street VARCHAR(255),
    address_city VARCHAR(100),
    address_postal_code VARCHAR(20),
    address_country VARCHAR(3),

    -- Registration status
    status guest_registration_status NOT NULL DEFAULT 'pending',
    registered_at TIMESTAMPTZ,
    reported_at TIMESTAMPTZ,
    report_reference VARCHAR(100),

    -- Is this the primary guest (booking holder)?
    is_primary BOOLEAN NOT NULL DEFAULT false,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for rental_guests
CREATE INDEX idx_rental_guests_org ON rental_guests(organization_id);
CREATE INDEX idx_rental_guests_booking ON rental_guests(booking_id);
CREATE INDEX idx_rental_guests_status ON rental_guests(status);
CREATE INDEX idx_rental_guests_nationality ON rental_guests(nationality);
CREATE INDEX idx_rental_guests_name ON rental_guests(last_name, first_name);
CREATE INDEX idx_rental_guests_primary ON rental_guests(booking_id, is_primary) WHERE is_primary = true;

-- ============================================
-- Authority Reports (Story 18.4)
-- ============================================

CREATE TABLE rental_guest_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,

    -- Report period
    report_type VARCHAR(50) NOT NULL,  -- monthly, quarterly, annual
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,

    -- Authority info
    authority_code VARCHAR(50) NOT NULL,  -- SK_UHUL, CZ_CIZPOL, etc.
    authority_name VARCHAR(255) NOT NULL,

    -- Report content
    total_guests INTEGER NOT NULL DEFAULT 0,
    guests_by_nationality JSONB,  -- { "SK": 10, "CZ": 5, ... }

    -- Generated files
    report_file_url VARCHAR(1000),
    report_format VARCHAR(10) NOT NULL,  -- pdf, csv, xml

    -- Submission status
    status VARCHAR(20) NOT NULL DEFAULT 'draft',  -- draft, generated, submitted, confirmed
    submitted_at TIMESTAMPTZ,
    submission_reference VARCHAR(100),
    submission_response TEXT,

    -- Metadata
    generated_by UUID REFERENCES users(id),
    submitted_by UUID REFERENCES users(id),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- One report per period per building per authority
    UNIQUE(building_id, authority_code, period_start, period_end)
);

-- Indexes for rental_guest_reports
CREATE INDEX idx_rental_guest_reports_org ON rental_guest_reports(organization_id);
CREATE INDEX idx_rental_guest_reports_building ON rental_guest_reports(building_id);
CREATE INDEX idx_rental_guest_reports_period ON rental_guest_reports(period_start, period_end);
CREATE INDEX idx_rental_guest_reports_status ON rental_guest_reports(status);
CREATE INDEX idx_rental_guest_reports_authority ON rental_guest_reports(authority_code);

-- ============================================
-- Calendar Blocks (for availability management)
-- ============================================

CREATE TABLE rental_calendar_blocks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,

    -- Block dates
    block_start DATE NOT NULL,
    block_end DATE NOT NULL,

    -- Block reason
    reason VARCHAR(50) NOT NULL,  -- booking, maintenance, owner_use, blocked
    booking_id UUID REFERENCES rental_bookings(id) ON DELETE CASCADE,

    -- Source
    source_platform rental_platform,
    synced_at TIMESTAMPTZ,

    notes TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Check dates are valid
    CONSTRAINT check_block_dates CHECK (block_end >= block_start)
);

-- Indexes for rental_calendar_blocks
CREATE INDEX idx_rental_calendar_blocks_org ON rental_calendar_blocks(organization_id);
CREATE INDEX idx_rental_calendar_blocks_unit ON rental_calendar_blocks(unit_id);
CREATE INDEX idx_rental_calendar_blocks_dates ON rental_calendar_blocks(block_start, block_end);
CREATE INDEX idx_rental_calendar_blocks_booking ON rental_calendar_blocks(booking_id);
CREATE INDEX idx_rental_calendar_blocks_reason ON rental_calendar_blocks(reason);

-- ============================================
-- iCal Feeds (for external calendar integration)
-- ============================================

CREATE TABLE rental_ical_feeds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,

    -- Feed info
    feed_name VARCHAR(100) NOT NULL,
    feed_token VARCHAR(100) NOT NULL UNIQUE,  -- For public URL access
    feed_url VARCHAR(1000),  -- Generated public URL

    -- Import feeds (from external)
    import_url VARCHAR(1000),
    import_platform rental_platform,
    last_import_at TIMESTAMPTZ,
    import_error TEXT,

    is_active BOOLEAN NOT NULL DEFAULT true,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for rental_ical_feeds
CREATE INDEX idx_rental_ical_feeds_org ON rental_ical_feeds(organization_id);
CREATE INDEX idx_rental_ical_feeds_unit ON rental_ical_feeds(unit_id);
CREATE INDEX idx_rental_ical_feeds_token ON rental_ical_feeds(feed_token);
CREATE INDEX idx_rental_ical_feeds_active ON rental_ical_feeds(is_active) WHERE is_active = true;

-- ============================================
-- Updated_at Triggers
-- ============================================

CREATE OR REPLACE FUNCTION update_rental_tables_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER trigger_rental_platform_connections_updated_at
    BEFORE UPDATE ON rental_platform_connections
    FOR EACH ROW
    EXECUTE FUNCTION update_rental_tables_updated_at();

CREATE TRIGGER trigger_rental_bookings_updated_at
    BEFORE UPDATE ON rental_bookings
    FOR EACH ROW
    EXECUTE FUNCTION update_rental_tables_updated_at();

CREATE TRIGGER trigger_rental_guests_updated_at
    BEFORE UPDATE ON rental_guests
    FOR EACH ROW
    EXECUTE FUNCTION update_rental_tables_updated_at();

CREATE TRIGGER trigger_rental_guest_reports_updated_at
    BEFORE UPDATE ON rental_guest_reports
    FOR EACH ROW
    EXECUTE FUNCTION update_rental_tables_updated_at();

CREATE TRIGGER trigger_rental_ical_feeds_updated_at
    BEFORE UPDATE ON rental_ical_feeds
    FOR EACH ROW
    EXECUTE FUNCTION update_rental_tables_updated_at();
