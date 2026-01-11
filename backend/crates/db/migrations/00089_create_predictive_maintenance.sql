-- Epic 134: Predictive Maintenance & Equipment Intelligence
-- Story 134.1: Equipment Registry
-- Story 134.2: Maintenance History Tracking
-- Story 134.3: Failure Prediction Engine
-- Story 134.4: Predictive Maintenance Dashboard

-- ============================================================================
-- EQUIPMENT REGISTRY
-- ============================================================================

CREATE TABLE IF NOT EXISTS equipment_registry (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,

    -- Basic Info
    name VARCHAR(255) NOT NULL,
    equipment_type VARCHAR(100) NOT NULL, -- hvac, elevator, plumbing, electrical, fire_safety, security, other
    category VARCHAR(100), -- heating, cooling, ventilation, lift, pump, generator, etc.
    manufacturer VARCHAR(255),
    model VARCHAR(255),
    serial_number VARCHAR(255),

    -- Location
    location_description TEXT, -- e.g., "Basement level 1, Room B-12"
    floor_number INTEGER,
    unit_id UUID REFERENCES units(id) ON DELETE SET NULL, -- If equipment is unit-specific

    -- Installation & Lifecycle
    installation_date DATE,
    warranty_expiry_date DATE,
    expected_lifespan_years INTEGER,
    replacement_cost DECIMAL(15, 2),

    -- Health & Status
    health_score INTEGER CHECK (health_score >= 0 AND health_score <= 100), -- 0-100, updated by prediction engine
    status VARCHAR(50) NOT NULL DEFAULT 'operational', -- operational, needs_maintenance, under_repair, decommissioned
    last_prediction_at TIMESTAMPTZ,
    next_predicted_failure DATE,
    failure_probability DECIMAL(5, 4), -- 0.0000 to 1.0000

    -- Metadata
    notes TEXT,
    specifications JSONB DEFAULT '{}', -- Technical specs, capacity, etc.

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    updated_by UUID REFERENCES users(id) ON DELETE SET NULL
);

-- Indexes for equipment_registry
CREATE INDEX IF NOT EXISTS idx_equipment_registry_org ON equipment_registry(organization_id);
CREATE INDEX IF NOT EXISTS idx_equipment_registry_building ON equipment_registry(building_id);
CREATE INDEX IF NOT EXISTS idx_equipment_registry_type ON equipment_registry(equipment_type);
CREATE INDEX IF NOT EXISTS idx_equipment_registry_status ON equipment_registry(status);
CREATE INDEX IF NOT EXISTS idx_equipment_registry_health ON equipment_registry(health_score);
CREATE INDEX IF NOT EXISTS idx_equipment_registry_next_failure ON equipment_registry(next_predicted_failure) WHERE next_predicted_failure IS NOT NULL;

-- ============================================================================
-- EQUIPMENT DOCUMENTS (manuals, warranties, etc.)
-- ============================================================================

CREATE TABLE IF NOT EXISTS equipment_documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    equipment_id UUID NOT NULL REFERENCES equipment_registry(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    document_type VARCHAR(50) NOT NULL, -- manual, warranty, certificate, diagram, photo
    title VARCHAR(255) NOT NULL,
    file_path VARCHAR(512),
    file_size INTEGER,
    mime_type VARCHAR(100),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    uploaded_by UUID REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_equipment_documents_equipment ON equipment_documents(equipment_id);
CREATE INDEX IF NOT EXISTS idx_equipment_documents_org ON equipment_documents(organization_id);

-- ============================================================================
-- MAINTENANCE LOGS (Story 134.2)
-- ============================================================================

CREATE TABLE IF NOT EXISTS maintenance_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    equipment_id UUID NOT NULL REFERENCES equipment_registry(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Maintenance Details
    maintenance_type VARCHAR(50) NOT NULL, -- preventive, corrective, emergency, inspection
    description TEXT NOT NULL,

    -- Timing
    scheduled_date DATE,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    duration_minutes INTEGER,

    -- Cost & Vendor
    cost DECIMAL(15, 2),
    currency VARCHAR(3) DEFAULT 'EUR',
    vendor_id UUID, -- References vendor table if exists
    vendor_name VARCHAR(255),
    technician_name VARCHAR(255),

    -- Parts & Work
    parts_replaced JSONB DEFAULT '[]', -- [{name, part_number, cost, quantity}]
    work_performed TEXT,

    -- Related Records
    fault_id UUID, -- Link to fault report if applicable
    work_order_id UUID, -- Link to work order if applicable

    -- Outcome
    outcome VARCHAR(50), -- completed, partial, failed, deferred
    notes TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_maintenance_logs_equipment ON maintenance_logs(equipment_id);
CREATE INDEX IF NOT EXISTS idx_maintenance_logs_org ON maintenance_logs(organization_id);
CREATE INDEX IF NOT EXISTS idx_maintenance_logs_type ON maintenance_logs(maintenance_type);
CREATE INDEX IF NOT EXISTS idx_maintenance_logs_completed ON maintenance_logs(completed_at);
CREATE INDEX IF NOT EXISTS idx_maintenance_logs_scheduled ON maintenance_logs(scheduled_date) WHERE scheduled_date IS NOT NULL;

-- ============================================================================
-- MAINTENANCE LOG PHOTOS
-- ============================================================================

CREATE TABLE IF NOT EXISTS maintenance_log_photos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    maintenance_log_id UUID NOT NULL REFERENCES maintenance_logs(id) ON DELETE CASCADE,

    file_path VARCHAR(512) NOT NULL,
    file_size INTEGER,
    mime_type VARCHAR(100),
    caption VARCHAR(255),
    photo_type VARCHAR(50), -- before, after, during, part, issue

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    uploaded_by UUID REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_maintenance_log_photos_log ON maintenance_log_photos(maintenance_log_id);

-- ============================================================================
-- PREDICTION HISTORY (Story 134.3)
-- ============================================================================

CREATE TABLE IF NOT EXISTS equipment_predictions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    equipment_id UUID NOT NULL REFERENCES equipment_registry(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Prediction Results
    health_score INTEGER NOT NULL CHECK (health_score >= 0 AND health_score <= 100),
    failure_probability DECIMAL(5, 4) NOT NULL,
    predicted_failure_date DATE,
    confidence_level DECIMAL(5, 4), -- Confidence in prediction

    -- Contributing Factors
    factors JSONB NOT NULL DEFAULT '[]', -- [{factor, weight, value}]
    -- Example: [{"factor": "age", "weight": 0.3, "value": "5 years"},
    --           {"factor": "usage_hours", "weight": 0.25, "value": "12000 hrs"}]

    -- Model Info
    model_version VARCHAR(50),
    model_type VARCHAR(50), -- time_based, usage_based, ml_model

    -- Recommendations
    recommended_action VARCHAR(100), -- schedule_maintenance, replace, monitor, none
    recommended_date DATE,
    urgency VARCHAR(20), -- critical, high, medium, low

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_equipment_predictions_equipment ON equipment_predictions(equipment_id);
CREATE INDEX IF NOT EXISTS idx_equipment_predictions_org ON equipment_predictions(organization_id);
CREATE INDEX IF NOT EXISTS idx_equipment_predictions_created ON equipment_predictions(created_at);
CREATE INDEX IF NOT EXISTS idx_equipment_predictions_health ON equipment_predictions(health_score);

-- ============================================================================
-- MAINTENANCE ALERTS
-- ============================================================================

CREATE TABLE IF NOT EXISTS maintenance_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    equipment_id UUID NOT NULL REFERENCES equipment_registry(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    prediction_id UUID REFERENCES equipment_predictions(id) ON DELETE SET NULL,

    alert_type VARCHAR(50) NOT NULL, -- health_threshold, predicted_failure, warranty_expiry, overdue_maintenance
    severity VARCHAR(20) NOT NULL, -- critical, high, medium, low

    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'active', -- active, acknowledged, resolved, dismissed
    acknowledged_at TIMESTAMPTZ,
    acknowledged_by UUID REFERENCES users(id) ON DELETE SET NULL,
    resolved_at TIMESTAMPTZ,
    resolved_by UUID REFERENCES users(id) ON DELETE SET NULL,

    -- Action Tracking
    maintenance_log_id UUID REFERENCES maintenance_logs(id) ON DELETE SET NULL, -- If maintenance was performed

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_maintenance_alerts_equipment ON maintenance_alerts(equipment_id);
CREATE INDEX IF NOT EXISTS idx_maintenance_alerts_org ON maintenance_alerts(organization_id);
CREATE INDEX IF NOT EXISTS idx_maintenance_alerts_status ON maintenance_alerts(status);
CREATE INDEX IF NOT EXISTS idx_maintenance_alerts_severity ON maintenance_alerts(severity);
CREATE INDEX IF NOT EXISTS idx_maintenance_alerts_type ON maintenance_alerts(alert_type);

-- ============================================================================
-- HEALTH THRESHOLDS CONFIGURATION
-- ============================================================================

CREATE TABLE IF NOT EXISTS equipment_health_thresholds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    equipment_type VARCHAR(100) NOT NULL, -- Apply to specific equipment type

    -- Threshold Levels
    critical_threshold INTEGER NOT NULL DEFAULT 20, -- Below this = critical alert
    warning_threshold INTEGER NOT NULL DEFAULT 50, -- Below this = warning alert

    -- Alert Settings
    alert_on_critical BOOLEAN NOT NULL DEFAULT TRUE,
    alert_on_warning BOOLEAN NOT NULL DEFAULT TRUE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(organization_id, equipment_type)
);

CREATE INDEX IF NOT EXISTS idx_health_thresholds_org ON equipment_health_thresholds(organization_id);

-- ============================================================================
-- ROW LEVEL SECURITY
-- ============================================================================

-- Equipment Registry
ALTER TABLE equipment_registry ENABLE ROW LEVEL SECURITY;

CREATE POLICY equipment_registry_tenant_isolation ON equipment_registry
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid)
    WITH CHECK (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY equipment_registry_select_policy ON equipment_registry
    FOR SELECT USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY equipment_registry_insert_policy ON equipment_registry
    FOR INSERT WITH CHECK (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY equipment_registry_update_policy ON equipment_registry
    FOR UPDATE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY equipment_registry_delete_policy ON equipment_registry
    FOR DELETE USING (
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

-- Equipment Documents
ALTER TABLE equipment_documents ENABLE ROW LEVEL SECURITY;

CREATE POLICY equipment_documents_tenant_isolation ON equipment_documents
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid)
    WITH CHECK (organization_id = current_setting('app.current_organization_id', true)::uuid);

-- Maintenance Logs
ALTER TABLE maintenance_logs ENABLE ROW LEVEL SECURITY;

CREATE POLICY maintenance_logs_tenant_isolation ON maintenance_logs
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid)
    WITH CHECK (organization_id = current_setting('app.current_organization_id', true)::uuid);

-- Maintenance Log Photos (inherit from parent)
ALTER TABLE maintenance_log_photos ENABLE ROW LEVEL SECURITY;

CREATE POLICY maintenance_log_photos_tenant_isolation ON maintenance_log_photos
    USING (
        maintenance_log_id IN (
            SELECT id FROM maintenance_logs
            WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
        )
    );

-- Equipment Predictions
ALTER TABLE equipment_predictions ENABLE ROW LEVEL SECURITY;

CREATE POLICY equipment_predictions_tenant_isolation ON equipment_predictions
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid)
    WITH CHECK (organization_id = current_setting('app.current_organization_id', true)::uuid);

-- Maintenance Alerts
ALTER TABLE maintenance_alerts ENABLE ROW LEVEL SECURITY;

CREATE POLICY maintenance_alerts_tenant_isolation ON maintenance_alerts
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid)
    WITH CHECK (organization_id = current_setting('app.current_organization_id', true)::uuid);

-- Health Thresholds
ALTER TABLE equipment_health_thresholds ENABLE ROW LEVEL SECURITY;

CREATE POLICY health_thresholds_tenant_isolation ON equipment_health_thresholds
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid)
    WITH CHECK (organization_id = current_setting('app.current_organization_id', true)::uuid);

-- ============================================================================
-- COMMENTS
-- ============================================================================

COMMENT ON TABLE equipment_registry IS 'Equipment inventory with lifecycle tracking (Epic 134, Story 134.1)';
COMMENT ON TABLE equipment_documents IS 'Manuals and documentation attached to equipment';
COMMENT ON TABLE maintenance_logs IS 'Historical maintenance records (Story 134.2)';
COMMENT ON TABLE maintenance_log_photos IS 'Photos documenting maintenance work';
COMMENT ON TABLE equipment_predictions IS 'AI prediction history for equipment health (Story 134.3)';
COMMENT ON TABLE maintenance_alerts IS 'Alerts generated from predictions and thresholds';
COMMENT ON TABLE equipment_health_thresholds IS 'Configurable alert thresholds per equipment type';
