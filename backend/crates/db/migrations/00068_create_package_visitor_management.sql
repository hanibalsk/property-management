-- Epic 58: Package & Visitor Management
-- Migration for package tracking and visitor pre-registration system

-- ============================================================================
-- Enums
-- ============================================================================

-- Package status values
CREATE TYPE package_status AS ENUM (
    'expected',      -- Resident registered package, waiting for arrival
    'received',      -- Staff logged package arrival
    'notified',      -- Resident was notified
    'picked_up',     -- Resident collected package
    'returned',      -- Package was returned to sender
    'unclaimed'      -- Package not picked up within time limit
);

-- Package carrier/provider
CREATE TYPE package_carrier AS ENUM (
    'usps',
    'ups',
    'fedex',
    'dhl',
    'amazon',
    'other'
);

-- Visitor status
CREATE TYPE visitor_status AS ENUM (
    'pending',       -- Pre-registered, waiting for arrival
    'checked_in',    -- Visitor arrived and checked in
    'checked_out',   -- Visitor left
    'expired',       -- Access code expired without use
    'cancelled'      -- Registration cancelled
);

-- Visitor purpose categories
CREATE TYPE visitor_purpose AS ENUM (
    'guest',         -- Personal guest
    'delivery',      -- One-time delivery
    'service',       -- Service provider (plumber, electrician, etc.)
    'contractor',    -- Construction/renovation work
    'real_estate',   -- Property viewing
    'other'
);

-- ============================================================================
-- Packages Table
-- ============================================================================

CREATE TABLE packages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    resident_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Package identification
    tracking_number VARCHAR(100),
    carrier package_carrier NOT NULL DEFAULT 'other',
    carrier_name VARCHAR(100), -- For 'other' carrier
    description TEXT,

    -- Status tracking
    status package_status NOT NULL DEFAULT 'expected',
    expected_date DATE,
    received_at TIMESTAMPTZ,
    received_by UUID REFERENCES users(id),
    notified_at TIMESTAMPTZ,
    picked_up_at TIMESTAMPTZ,
    picked_up_by UUID REFERENCES users(id), -- Who actually picked up (could be different from resident)

    -- Additional info
    storage_location VARCHAR(200), -- e.g., "Lobby locker #5", "Mailroom shelf B"
    photo_url TEXT,
    notes TEXT,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for packages
CREATE INDEX idx_packages_tenant_id ON packages(tenant_id);
CREATE INDEX idx_packages_building_id ON packages(building_id);
CREATE INDEX idx_packages_unit_id ON packages(unit_id);
CREATE INDEX idx_packages_resident_id ON packages(resident_id);
CREATE INDEX idx_packages_status ON packages(status);
CREATE INDEX idx_packages_tracking_number ON packages(tracking_number);
CREATE INDEX idx_packages_received_at ON packages(received_at);
CREATE INDEX idx_packages_expected_date ON packages(expected_date);

-- ============================================================================
-- Visitors Table
-- ============================================================================

CREATE TABLE visitors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    host_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Visitor information
    visitor_name VARCHAR(200) NOT NULL,
    visitor_email VARCHAR(255),
    visitor_phone VARCHAR(50),
    company_name VARCHAR(200), -- For service providers, contractors
    purpose visitor_purpose NOT NULL DEFAULT 'guest',
    purpose_notes TEXT,

    -- Access control
    access_code VARCHAR(20) NOT NULL, -- 6-digit alphanumeric by default
    access_code_expires_at TIMESTAMPTZ NOT NULL,

    -- Visit scheduling
    expected_arrival TIMESTAMPTZ NOT NULL,
    expected_departure TIMESTAMPTZ, -- Optional for recurring/unknown duration

    -- Status tracking
    status visitor_status NOT NULL DEFAULT 'pending',
    checked_in_at TIMESTAMPTZ,
    checked_in_by UUID REFERENCES users(id), -- Staff who verified
    checked_out_at TIMESTAMPTZ,
    checked_out_by UUID REFERENCES users(id),

    -- Notifications
    notification_sent_at TIMESTAMPTZ,
    notification_method VARCHAR(20), -- 'email', 'sms', 'both'

    -- Additional info
    vehicle_license_plate VARCHAR(20),
    notes TEXT,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for visitors
CREATE INDEX idx_visitors_tenant_id ON visitors(tenant_id);
CREATE INDEX idx_visitors_building_id ON visitors(building_id);
CREATE INDEX idx_visitors_unit_id ON visitors(unit_id);
CREATE INDEX idx_visitors_host_id ON visitors(host_id);
CREATE INDEX idx_visitors_status ON visitors(status);
CREATE INDEX idx_visitors_access_code ON visitors(access_code);
CREATE INDEX idx_visitors_expected_arrival ON visitors(expected_arrival);
CREATE INDEX idx_visitors_checked_in_at ON visitors(checked_in_at);

-- ============================================================================
-- Package Notifications Log
-- ============================================================================

CREATE TABLE package_notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    package_id UUID NOT NULL REFERENCES packages(id) ON DELETE CASCADE,
    notification_type VARCHAR(50) NOT NULL, -- 'arrival', 'reminder', 'unclaimed_warning'
    sent_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    sent_via VARCHAR(20) NOT NULL, -- 'email', 'push', 'sms'
    success BOOLEAN NOT NULL DEFAULT true,
    error_message TEXT
);

CREATE INDEX idx_package_notifications_package_id ON package_notifications(package_id);

-- ============================================================================
-- Visitor Access Log
-- ============================================================================

CREATE TABLE visitor_access_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    visitor_id UUID NOT NULL REFERENCES visitors(id) ON DELETE CASCADE,
    action VARCHAR(50) NOT NULL, -- 'code_verified', 'check_in', 'check_out', 'access_denied'
    performed_by UUID REFERENCES users(id),
    performed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    location VARCHAR(200), -- e.g., "Main Entrance", "Parking Gate"
    notes TEXT
);

CREATE INDEX idx_visitor_access_logs_visitor_id ON visitor_access_logs(visitor_id);
CREATE INDEX idx_visitor_access_logs_performed_at ON visitor_access_logs(performed_at);

-- ============================================================================
-- Building Package Settings
-- ============================================================================

CREATE TABLE building_package_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,

    -- Package policies
    max_storage_days INTEGER NOT NULL DEFAULT 7,
    send_reminder_after_days INTEGER NOT NULL DEFAULT 3,
    require_photo_on_receipt BOOLEAN NOT NULL DEFAULT false,
    allow_resident_self_pickup BOOLEAN NOT NULL DEFAULT true,

    -- Notification settings
    notify_on_arrival BOOLEAN NOT NULL DEFAULT true,
    send_daily_summary BOOLEAN NOT NULL DEFAULT false,

    -- Storage info
    storage_instructions TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(tenant_id, building_id)
);

CREATE INDEX idx_building_package_settings_tenant_building ON building_package_settings(tenant_id, building_id);

-- ============================================================================
-- Building Visitor Settings
-- ============================================================================

CREATE TABLE building_visitor_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,

    -- Access code settings
    default_code_validity_hours INTEGER NOT NULL DEFAULT 24,
    code_length INTEGER NOT NULL DEFAULT 6,
    require_purpose BOOLEAN NOT NULL DEFAULT false,

    -- Pre-registration limits
    max_visitors_per_day_per_unit INTEGER,
    max_advance_registration_days INTEGER NOT NULL DEFAULT 30,

    -- Notification settings
    notify_host_on_checkin BOOLEAN NOT NULL DEFAULT true,
    send_visitor_instructions BOOLEAN NOT NULL DEFAULT true,

    -- Verification requirements
    require_id_verification BOOLEAN NOT NULL DEFAULT false,
    require_photo BOOLEAN NOT NULL DEFAULT false,

    -- Instructions
    visitor_instructions TEXT, -- Sent to visitors
    staff_instructions TEXT, -- Displayed to staff

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(tenant_id, building_id)
);

CREATE INDEX idx_building_visitor_settings_tenant_building ON building_visitor_settings(tenant_id, building_id);

-- ============================================================================
-- Row Level Security Policies
-- ============================================================================

ALTER TABLE packages ENABLE ROW LEVEL SECURITY;
ALTER TABLE visitors ENABLE ROW LEVEL SECURITY;
ALTER TABLE package_notifications ENABLE ROW LEVEL SECURITY;
ALTER TABLE visitor_access_logs ENABLE ROW LEVEL SECURITY;
ALTER TABLE building_package_settings ENABLE ROW LEVEL SECURITY;
ALTER TABLE building_visitor_settings ENABLE ROW LEVEL SECURITY;

-- Packages policies
CREATE POLICY packages_tenant_isolation ON packages
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Visitors policies
CREATE POLICY visitors_tenant_isolation ON visitors
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Package notifications policies
CREATE POLICY package_notifications_tenant_isolation ON package_notifications
    USING (package_id IN (
        SELECT id FROM packages
        WHERE tenant_id = current_setting('app.current_tenant_id', true)::uuid
    ));

-- Visitor access logs policies
CREATE POLICY visitor_access_logs_tenant_isolation ON visitor_access_logs
    USING (visitor_id IN (
        SELECT id FROM visitors
        WHERE tenant_id = current_setting('app.current_tenant_id', true)::uuid
    ));

-- Building package settings policies
CREATE POLICY building_package_settings_tenant_isolation ON building_package_settings
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Building visitor settings policies
CREATE POLICY building_visitor_settings_tenant_isolation ON building_visitor_settings
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- ============================================================================
-- Triggers for updated_at
-- ============================================================================

CREATE TRIGGER update_packages_updated_at
    BEFORE UPDATE ON packages
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_visitors_updated_at
    BEFORE UPDATE ON visitors
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_building_package_settings_updated_at
    BEFORE UPDATE ON building_package_settings
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_building_visitor_settings_updated_at
    BEFORE UPDATE ON building_visitor_settings
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- Helper function for access code generation
-- ============================================================================

CREATE OR REPLACE FUNCTION generate_visitor_access_code(length INTEGER DEFAULT 6)
RETURNS VARCHAR AS $$
DECLARE
    chars VARCHAR := 'ABCDEFGHJKLMNPQRSTUVWXYZ23456789'; -- Excluding confusing chars: 0, O, I, 1
    result VARCHAR := '';
    i INTEGER;
BEGIN
    FOR i IN 1..length LOOP
        result := result || substr(chars, floor(random() * length(chars) + 1)::integer, 1);
    END LOOP;
    RETURN result;
END;
$$ LANGUAGE plpgsql;
