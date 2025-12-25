-- Epic 57: Building Registries (Pets & Vehicles)
-- Migration for pet and vehicle registration system

-- Pet type enum
CREATE TYPE pet_type AS ENUM ('dog', 'cat', 'bird', 'fish', 'reptile', 'rodent', 'other');

-- Pet size enum
CREATE TYPE pet_size AS ENUM ('small', 'medium', 'large', 'extra_large');

-- Vehicle type enum
CREATE TYPE vehicle_type AS ENUM ('car', 'motorcycle', 'bicycle', 'scooter', 'other');

-- Registry status enum
CREATE TYPE registry_status AS ENUM ('pending', 'approved', 'rejected', 'expired');

-- Pet registrations table
CREATE TABLE pet_registrations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    pet_name VARCHAR(100) NOT NULL,
    pet_type pet_type NOT NULL,
    breed VARCHAR(100),
    pet_size pet_size NOT NULL,
    weight_kg DECIMAL(5, 2),
    age_years INTEGER,
    color VARCHAR(50),
    microchip_id VARCHAR(50),
    vaccination_date DATE,
    vaccination_document_id UUID REFERENCES documents(id),
    special_needs TEXT,
    status registry_status NOT NULL DEFAULT 'pending',
    reviewed_by UUID REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    rejection_reason TEXT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Vehicle registrations table
CREATE TABLE vehicle_registrations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    vehicle_type vehicle_type NOT NULL,
    make VARCHAR(50) NOT NULL,
    model VARCHAR(50) NOT NULL,
    year INTEGER,
    color VARCHAR(30),
    license_plate VARCHAR(20) NOT NULL,
    registration_document_id UUID REFERENCES documents(id),
    insurance_document_id UUID REFERENCES documents(id),
    parking_spot_id UUID,
    status registry_status NOT NULL DEFAULT 'pending',
    reviewed_by UUID REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    rejection_reason TEXT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Parking spots table
CREATE TABLE parking_spots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    spot_number VARCHAR(20) NOT NULL,
    spot_type VARCHAR(30) NOT NULL DEFAULT 'standard',
    floor_level VARCHAR(10),
    is_covered BOOLEAN NOT NULL DEFAULT FALSE,
    is_reserved BOOLEAN NOT NULL DEFAULT FALSE,
    assigned_to_unit_id UUID REFERENCES units(id),
    monthly_fee DECIMAL(10, 2),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, building_id, spot_number)
);

-- Building registry rules table
CREATE TABLE building_registry_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    pets_allowed BOOLEAN NOT NULL DEFAULT TRUE,
    pets_require_approval BOOLEAN NOT NULL DEFAULT TRUE,
    max_pets_per_unit INTEGER,
    allowed_pet_types pet_type[],
    banned_pet_breeds TEXT[],
    max_pet_weight DECIMAL(5, 2),
    vehicles_require_approval BOOLEAN NOT NULL DEFAULT FALSE,
    max_vehicles_per_unit INTEGER,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, building_id)
);

-- Indexes for pet_registrations
CREATE INDEX idx_pet_registrations_tenant ON pet_registrations(tenant_id);
CREATE INDEX idx_pet_registrations_unit ON pet_registrations(unit_id);
CREATE INDEX idx_pet_registrations_owner ON pet_registrations(owner_id);
CREATE INDEX idx_pet_registrations_status ON pet_registrations(status);

-- Indexes for vehicle_registrations
CREATE INDEX idx_vehicle_registrations_tenant ON vehicle_registrations(tenant_id);
CREATE INDEX idx_vehicle_registrations_unit ON vehicle_registrations(unit_id);
CREATE INDEX idx_vehicle_registrations_owner ON vehicle_registrations(owner_id);
CREATE INDEX idx_vehicle_registrations_status ON vehicle_registrations(status);
CREATE INDEX idx_vehicle_registrations_license_plate ON vehicle_registrations(license_plate);

-- Indexes for parking_spots
CREATE INDEX idx_parking_spots_tenant ON parking_spots(tenant_id);
CREATE INDEX idx_parking_spots_building ON parking_spots(building_id);
CREATE INDEX idx_parking_spots_assigned_unit ON parking_spots(assigned_to_unit_id);

-- Indexes for building_registry_rules
CREATE INDEX idx_building_registry_rules_tenant ON building_registry_rules(tenant_id);
CREATE INDEX idx_building_registry_rules_building ON building_registry_rules(building_id);

-- Add foreign key for parking spot in vehicle registrations
ALTER TABLE vehicle_registrations
    ADD CONSTRAINT fk_vehicle_parking_spot
    FOREIGN KEY (parking_spot_id) REFERENCES parking_spots(id);

-- Row Level Security
ALTER TABLE pet_registrations ENABLE ROW LEVEL SECURITY;
ALTER TABLE vehicle_registrations ENABLE ROW LEVEL SECURITY;
ALTER TABLE parking_spots ENABLE ROW LEVEL SECURITY;
ALTER TABLE building_registry_rules ENABLE ROW LEVEL SECURITY;

-- RLS Policies for pet_registrations
CREATE POLICY pet_registrations_tenant_isolation ON pet_registrations
    USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

CREATE POLICY pet_registrations_insert ON pet_registrations
    FOR INSERT WITH CHECK (tenant_id = current_setting('app.current_tenant_id')::UUID);

-- RLS Policies for vehicle_registrations
CREATE POLICY vehicle_registrations_tenant_isolation ON vehicle_registrations
    USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

CREATE POLICY vehicle_registrations_insert ON vehicle_registrations
    FOR INSERT WITH CHECK (tenant_id = current_setting('app.current_tenant_id')::UUID);

-- RLS Policies for parking_spots
CREATE POLICY parking_spots_tenant_isolation ON parking_spots
    USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

CREATE POLICY parking_spots_insert ON parking_spots
    FOR INSERT WITH CHECK (tenant_id = current_setting('app.current_tenant_id')::UUID);

-- RLS Policies for building_registry_rules
CREATE POLICY building_registry_rules_tenant_isolation ON building_registry_rules
    USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

CREATE POLICY building_registry_rules_insert ON building_registry_rules
    FOR INSERT WITH CHECK (tenant_id = current_setting('app.current_tenant_id')::UUID);

-- Trigger function (create if not exists)
CREATE OR REPLACE FUNCTION trigger_set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


-- Updated at triggers
CREATE TRIGGER set_pet_registrations_updated_at
    BEFORE UPDATE ON pet_registrations
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TRIGGER set_vehicle_registrations_updated_at
    BEFORE UPDATE ON vehicle_registrations
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TRIGGER set_parking_spots_updated_at
    BEFORE UPDATE ON parking_spots
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_updated_at();

CREATE TRIGGER set_building_registry_rules_updated_at
    BEFORE UPDATE ON building_registry_rules
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_updated_at();
