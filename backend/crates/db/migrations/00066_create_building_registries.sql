-- Epic 57: Building Registries (Pets & Vehicles)
-- FR180: Residents can register pets with details and documents
-- FR181: Residents can register vehicles for parking management
-- FR182: Managers can view and manage registry entries
-- FR183: System enforces registry rules per building

-- =============================================================================
-- REGISTRY TYPES AND ENUMS
-- =============================================================================

-- Pet types
CREATE TYPE pet_type AS ENUM (
    'dog',
    'cat',
    'bird',
    'fish',
    'rabbit',
    'hamster',
    'reptile',
    'other'
);

-- Pet size categories
CREATE TYPE pet_size AS ENUM (
    'small',      -- < 10 kg
    'medium',     -- 10-25 kg
    'large',      -- 25-45 kg
    'extra_large' -- > 45 kg
);

-- Vehicle types
CREATE TYPE vehicle_type AS ENUM (
    'car',
    'motorcycle',
    'bicycle',
    'electric_scooter',
    'truck',
    'van',
    'other'
);

-- Registry entry status
CREATE TYPE registry_status AS ENUM (
    'pending',    -- Awaiting approval
    'approved',   -- Active and approved
    'rejected',   -- Rejected by manager
    'expired',    -- Registration expired
    'inactive'    -- Deactivated by owner
);

-- =============================================================================
-- PET REGISTRY (Story 57.1)
-- =============================================================================

CREATE TABLE pet_registrations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    owner_id UUID NOT NULL REFERENCES users(id),

    -- Pet details
    name VARCHAR(100) NOT NULL,
    pet_type pet_type NOT NULL,
    breed VARCHAR(100),
    pet_size pet_size,
    weight_kg DECIMAL(5, 2),
    color VARCHAR(50),
    date_of_birth DATE,
    microchip_number VARCHAR(50),

    -- Status and approval
    status registry_status NOT NULL DEFAULT 'pending',
    registration_number VARCHAR(50) UNIQUE,
    registered_at TIMESTAMP WITH TIME ZONE,
    expires_at TIMESTAMP WITH TIME ZONE,

    -- Documents
    photo_url TEXT,
    vaccination_document_url TEXT,
    vaccination_expiry DATE,
    license_document_url TEXT,
    insurance_document_url TEXT,

    -- Notes
    special_needs TEXT,
    notes TEXT,

    -- Manager actions
    reviewed_by UUID REFERENCES users(id),
    reviewed_at TIMESTAMP WITH TIME ZONE,
    rejection_reason TEXT,

    -- Audit
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- VEHICLE REGISTRY (Story 57.2)
-- =============================================================================

CREATE TABLE vehicle_registrations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    owner_id UUID NOT NULL REFERENCES users(id),

    -- Vehicle details
    vehicle_type vehicle_type NOT NULL,
    make VARCHAR(100) NOT NULL,
    model VARCHAR(100) NOT NULL,
    year INT,
    color VARCHAR(50),
    license_plate VARCHAR(20) NOT NULL,
    vin VARCHAR(17),

    -- Status and approval
    status registry_status NOT NULL DEFAULT 'pending',
    registration_number VARCHAR(50) UNIQUE,
    registered_at TIMESTAMP WITH TIME ZONE,
    expires_at TIMESTAMP WITH TIME ZONE,

    -- Parking
    parking_spot_id UUID,
    parking_permit_number VARCHAR(50),

    -- Documents
    photo_url TEXT,
    registration_document_url TEXT,
    insurance_document_url TEXT,
    insurance_expiry DATE,

    -- Notes
    notes TEXT,

    -- Manager actions
    reviewed_by UUID REFERENCES users(id),
    reviewed_at TIMESTAMP WITH TIME ZONE,
    rejection_reason TEXT,

    -- Audit
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- PARKING SPOTS
-- =============================================================================

CREATE TABLE parking_spots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,

    -- Spot details
    spot_number VARCHAR(20) NOT NULL,
    floor VARCHAR(20),
    section VARCHAR(50),
    spot_type VARCHAR(50) NOT NULL DEFAULT 'standard', -- standard, handicapped, electric, motorcycle, bicycle

    -- Assignment
    assigned_unit_id UUID REFERENCES units(id),
    assigned_vehicle_id UUID REFERENCES vehicle_registrations(id),
    is_available BOOLEAN NOT NULL DEFAULT TRUE,

    -- Features
    has_electric_charging BOOLEAN NOT NULL DEFAULT FALSE,
    is_covered BOOLEAN NOT NULL DEFAULT FALSE,
    width_meters DECIMAL(4, 2),
    length_meters DECIMAL(4, 2),

    -- Fees
    monthly_fee DECIMAL(10, 2),
    fee_currency VARCHAR(3) DEFAULT 'EUR',

    -- Notes
    notes TEXT,

    -- Audit
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    UNIQUE(building_id, spot_number)
);

-- =============================================================================
-- REGISTRY RULES (Story 57.4)
-- =============================================================================

CREATE TABLE building_registry_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,

    -- Pet rules
    pets_allowed BOOLEAN NOT NULL DEFAULT TRUE,
    max_pets_per_unit INT DEFAULT 2,
    allowed_pet_types pet_type[],
    max_pet_weight_kg DECIMAL(5, 2),
    requires_pet_approval BOOLEAN NOT NULL DEFAULT TRUE,
    requires_pet_vaccination BOOLEAN NOT NULL DEFAULT TRUE,
    requires_pet_insurance BOOLEAN NOT NULL DEFAULT FALSE,
    pet_deposit_amount DECIMAL(10, 2),
    pet_monthly_fee DECIMAL(10, 2),
    restricted_breeds TEXT[], -- List of restricted dog breeds

    -- Vehicle rules
    vehicles_allowed BOOLEAN NOT NULL DEFAULT TRUE,
    max_vehicles_per_unit INT DEFAULT 2,
    allowed_vehicle_types vehicle_type[],
    requires_vehicle_approval BOOLEAN NOT NULL DEFAULT FALSE,
    requires_vehicle_insurance BOOLEAN NOT NULL DEFAULT TRUE,
    parking_fee_included BOOLEAN NOT NULL DEFAULT FALSE,
    guest_parking_allowed BOOLEAN NOT NULL DEFAULT TRUE,
    guest_parking_max_hours INT DEFAULT 24,

    -- General settings
    registration_validity_months INT DEFAULT 12, -- How long registration is valid
    renewal_reminder_days INT DEFAULT 30, -- Days before expiry to send reminder

    -- Notes
    additional_rules TEXT,

    -- Audit
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    UNIQUE(building_id)
);

-- =============================================================================
-- REGISTRY HISTORY (for audit trail)
-- =============================================================================

CREATE TABLE registry_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    registry_type VARCHAR(20) NOT NULL, -- 'pet' or 'vehicle'
    registry_id UUID NOT NULL,
    action VARCHAR(50) NOT NULL, -- created, updated, approved, rejected, expired, renewed
    old_status registry_status,
    new_status registry_status,
    changes JSONB,
    performed_by UUID REFERENCES users(id),
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- INDEXES
-- =============================================================================

-- Pet registrations indexes
CREATE INDEX idx_pet_registrations_building ON pet_registrations(building_id);
CREATE INDEX idx_pet_registrations_unit ON pet_registrations(unit_id);
CREATE INDEX idx_pet_registrations_owner ON pet_registrations(owner_id);
CREATE INDEX idx_pet_registrations_status ON pet_registrations(status);
CREATE INDEX idx_pet_registrations_building_status ON pet_registrations(building_id, status);

-- Vehicle registrations indexes
CREATE INDEX idx_vehicle_registrations_building ON vehicle_registrations(building_id);
CREATE INDEX idx_vehicle_registrations_unit ON vehicle_registrations(unit_id);
CREATE INDEX idx_vehicle_registrations_owner ON vehicle_registrations(owner_id);
CREATE INDEX idx_vehicle_registrations_status ON vehicle_registrations(status);
CREATE INDEX idx_vehicle_registrations_license ON vehicle_registrations(license_plate);
CREATE INDEX idx_vehicle_registrations_building_status ON vehicle_registrations(building_id, status);

-- Parking spots indexes
CREATE INDEX idx_parking_spots_building ON parking_spots(building_id);
CREATE INDEX idx_parking_spots_available ON parking_spots(building_id, is_available) WHERE is_available = TRUE;

-- Registry rules index
CREATE INDEX idx_registry_rules_building ON building_registry_rules(building_id);

-- Registry history indexes
CREATE INDEX idx_registry_history_type_id ON registry_history(registry_type, registry_id);
CREATE INDEX idx_registry_history_created ON registry_history(created_at DESC);

-- =============================================================================
-- ROW-LEVEL SECURITY
-- =============================================================================

ALTER TABLE pet_registrations ENABLE ROW LEVEL SECURITY;
ALTER TABLE vehicle_registrations ENABLE ROW LEVEL SECURITY;
ALTER TABLE parking_spots ENABLE ROW LEVEL SECURITY;
ALTER TABLE building_registry_rules ENABLE ROW LEVEL SECURITY;
ALTER TABLE registry_history ENABLE ROW LEVEL SECURITY;

-- Pet registrations policies
CREATE POLICY pet_registrations_tenant_isolation ON pet_registrations
    USING (building_id IN (
        SELECT building_id FROM user_building_access WHERE user_id = current_setting('app.current_user_id')::UUID
    ));

-- Vehicle registrations policies
CREATE POLICY vehicle_registrations_tenant_isolation ON vehicle_registrations
    USING (building_id IN (
        SELECT building_id FROM user_building_access WHERE user_id = current_setting('app.current_user_id')::UUID
    ));

-- Parking spots policies
CREATE POLICY parking_spots_tenant_isolation ON parking_spots
    USING (building_id IN (
        SELECT building_id FROM user_building_access WHERE user_id = current_setting('app.current_user_id')::UUID
    ));

-- Registry rules policies
CREATE POLICY registry_rules_tenant_isolation ON building_registry_rules
    USING (building_id IN (
        SELECT building_id FROM user_building_access WHERE user_id = current_setting('app.current_user_id')::UUID
    ));

-- =============================================================================
-- TRIGGER FUNCTIONS
-- =============================================================================

-- Generate registration number for pets
CREATE OR REPLACE FUNCTION generate_pet_registration_number()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.status = 'approved' AND NEW.registration_number IS NULL THEN
        NEW.registration_number := 'PET-' || TO_CHAR(NOW(), 'YYYY') || '-' ||
            LPAD(NEXTVAL('pet_registration_seq')::TEXT, 6, '0');
        NEW.registered_at := NOW();
        NEW.expires_at := NOW() + INTERVAL '1 year';
    END IF;
    NEW.updated_at := NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create sequence for pet registration numbers
CREATE SEQUENCE IF NOT EXISTS pet_registration_seq START 1;

CREATE TRIGGER trg_pet_registration_number
BEFORE UPDATE ON pet_registrations
FOR EACH ROW
WHEN (NEW.status = 'approved' AND OLD.status != 'approved')
EXECUTE FUNCTION generate_pet_registration_number();

-- Generate registration number for vehicles
CREATE OR REPLACE FUNCTION generate_vehicle_registration_number()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.status = 'approved' AND NEW.registration_number IS NULL THEN
        NEW.registration_number := 'VEH-' || TO_CHAR(NOW(), 'YYYY') || '-' ||
            LPAD(NEXTVAL('vehicle_registration_seq')::TEXT, 6, '0');
        NEW.registered_at := NOW();
        NEW.expires_at := NOW() + INTERVAL '1 year';
    END IF;
    NEW.updated_at := NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create sequence for vehicle registration numbers
CREATE SEQUENCE IF NOT EXISTS vehicle_registration_seq START 1;

CREATE TRIGGER trg_vehicle_registration_number
BEFORE UPDATE ON vehicle_registrations
FOR EACH ROW
WHEN (NEW.status = 'approved' AND OLD.status != 'approved')
EXECUTE FUNCTION generate_vehicle_registration_number();

-- Log registry changes
CREATE OR REPLACE FUNCTION log_registry_change()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_TABLE_NAME = 'pet_registrations' THEN
        INSERT INTO registry_history (registry_type, registry_id, action, old_status, new_status, performed_by)
        VALUES (
            'pet',
            COALESCE(NEW.id, OLD.id),
            CASE
                WHEN TG_OP = 'INSERT' THEN 'created'
                WHEN TG_OP = 'DELETE' THEN 'deleted'
                WHEN NEW.status != OLD.status THEN NEW.status::TEXT
                ELSE 'updated'
            END,
            CASE WHEN TG_OP != 'INSERT' THEN OLD.status END,
            CASE WHEN TG_OP != 'DELETE' THEN NEW.status END,
            CASE WHEN TG_OP = 'DELETE' THEN NULL ELSE NEW.reviewed_by END
        );
    ELSIF TG_TABLE_NAME = 'vehicle_registrations' THEN
        INSERT INTO registry_history (registry_type, registry_id, action, old_status, new_status, performed_by)
        VALUES (
            'vehicle',
            COALESCE(NEW.id, OLD.id),
            CASE
                WHEN TG_OP = 'INSERT' THEN 'created'
                WHEN TG_OP = 'DELETE' THEN 'deleted'
                WHEN NEW.status != OLD.status THEN NEW.status::TEXT
                ELSE 'updated'
            END,
            CASE WHEN TG_OP != 'INSERT' THEN OLD.status END,
            CASE WHEN TG_OP != 'DELETE' THEN NEW.status END,
            CASE WHEN TG_OP = 'DELETE' THEN NULL ELSE NEW.reviewed_by END
        );
    END IF;
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_log_pet_changes
AFTER INSERT OR UPDATE OR DELETE ON pet_registrations
FOR EACH ROW EXECUTE FUNCTION log_registry_change();

CREATE TRIGGER trg_log_vehicle_changes
AFTER INSERT OR UPDATE OR DELETE ON vehicle_registrations
FOR EACH ROW EXECUTE FUNCTION log_registry_change();

-- Update parking spot availability
CREATE OR REPLACE FUNCTION update_parking_spot_availability()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'UPDATE' AND NEW.parking_spot_id IS DISTINCT FROM OLD.parking_spot_id THEN
        -- Free old spot
        IF OLD.parking_spot_id IS NOT NULL THEN
            UPDATE parking_spots
            SET is_available = TRUE, assigned_vehicle_id = NULL, updated_at = NOW()
            WHERE id = OLD.parking_spot_id;
        END IF;
        -- Assign new spot
        IF NEW.parking_spot_id IS NOT NULL THEN
            UPDATE parking_spots
            SET is_available = FALSE, assigned_vehicle_id = NEW.id, updated_at = NOW()
            WHERE id = NEW.parking_spot_id;
        END IF;
    ELSIF TG_OP = 'INSERT' AND NEW.parking_spot_id IS NOT NULL THEN
        UPDATE parking_spots
        SET is_available = FALSE, assigned_vehicle_id = NEW.id, updated_at = NOW()
        WHERE id = NEW.parking_spot_id;
    ELSIF TG_OP = 'DELETE' AND OLD.parking_spot_id IS NOT NULL THEN
        UPDATE parking_spots
        SET is_available = TRUE, assigned_vehicle_id = NULL, updated_at = NOW()
        WHERE id = OLD.parking_spot_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_parking_spot_availability
AFTER INSERT OR UPDATE OR DELETE ON vehicle_registrations
FOR EACH ROW EXECUTE FUNCTION update_parking_spot_availability();
