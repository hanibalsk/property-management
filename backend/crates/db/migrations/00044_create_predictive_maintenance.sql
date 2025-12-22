-- Epic 13: AI Assistant & Automation
-- Story 13.3: Predictive Maintenance

-- Equipment tracking for predictive maintenance
CREATE TABLE IF NOT EXISTS equipment (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    facility_id UUID REFERENCES facilities(id) ON DELETE SET NULL,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    manufacturer TEXT,
    model TEXT,
    serial_number TEXT,
    installation_date DATE,
    warranty_expires DATE,
    expected_lifespan_years INTEGER,
    maintenance_interval_days INTEGER,
    last_maintenance_date DATE,
    next_maintenance_due DATE,
    status TEXT NOT NULL DEFAULT 'operational' CHECK (status IN ('operational', 'needs_maintenance', 'under_repair', 'decommissioned')),
    notes TEXT,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Maintenance history for equipment
CREATE TABLE IF NOT EXISTS equipment_maintenance (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    equipment_id UUID NOT NULL REFERENCES equipment(id) ON DELETE CASCADE,
    maintenance_type TEXT NOT NULL CHECK (maintenance_type IN ('preventive', 'corrective', 'emergency', 'inspection')),
    description TEXT NOT NULL,
    performed_by UUID REFERENCES users(id),
    external_vendor TEXT,
    cost DECIMAL(12, 2),
    parts_replaced TEXT[],
    fault_id UUID REFERENCES faults(id) ON DELETE SET NULL,
    scheduled_date DATE,
    completed_date DATE,
    status TEXT NOT NULL DEFAULT 'scheduled' CHECK (status IN ('scheduled', 'in_progress', 'completed', 'cancelled')),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Predictive maintenance predictions
CREATE TABLE IF NOT EXISTS maintenance_predictions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    equipment_id UUID NOT NULL REFERENCES equipment(id) ON DELETE CASCADE,
    risk_score FLOAT NOT NULL CHECK (risk_score >= 0 AND risk_score <= 100),
    predicted_failure_date DATE,
    confidence FLOAT NOT NULL CHECK (confidence >= 0 AND confidence <= 1),
    recommendation TEXT NOT NULL,
    factors JSONB DEFAULT '{}',
    acknowledged BOOLEAN DEFAULT FALSE,
    acknowledged_by UUID REFERENCES users(id),
    action_taken TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_equipment_org ON equipment(organization_id);
CREATE INDEX idx_equipment_building ON equipment(building_id);
CREATE INDEX idx_equipment_facility ON equipment(facility_id);
CREATE INDEX idx_equipment_status ON equipment(status);
CREATE INDEX idx_equipment_next_maintenance ON equipment(next_maintenance_due);
CREATE INDEX idx_equipment_maintenance_equipment ON equipment_maintenance(equipment_id);
CREATE INDEX idx_equipment_maintenance_status ON equipment_maintenance(status);
CREATE INDEX idx_maintenance_predictions_equipment ON maintenance_predictions(equipment_id);
CREATE INDEX idx_maintenance_predictions_risk ON maintenance_predictions(risk_score DESC);

-- RLS policies
ALTER TABLE equipment ENABLE ROW LEVEL SECURITY;
ALTER TABLE equipment_maintenance ENABLE ROW LEVEL SECURITY;
ALTER TABLE maintenance_predictions ENABLE ROW LEVEL SECURITY;

CREATE POLICY equipment_tenant_isolation ON equipment
    FOR ALL
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY equipment_maintenance_tenant_isolation ON equipment_maintenance
    FOR ALL
    USING (equipment_id IN (
        SELECT id FROM equipment
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

CREATE POLICY maintenance_predictions_tenant_isolation ON maintenance_predictions
    FOR ALL
    USING (equipment_id IN (
        SELECT id FROM equipment
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

-- Triggers
CREATE TRIGGER update_equipment_updated_at
    BEFORE UPDATE ON equipment
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_equipment_maintenance_updated_at
    BEFORE UPDATE ON equipment_maintenance
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_maintenance_predictions_updated_at
    BEFORE UPDATE ON maintenance_predictions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
