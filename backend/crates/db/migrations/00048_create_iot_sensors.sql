-- Epic 14: IoT & Smart Building
-- Story 14.1: IoT Sensor Registration

-- IoT sensors table
CREATE TABLE IF NOT EXISTS sensors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    unit_id UUID REFERENCES units(id) ON DELETE SET NULL,
    name TEXT NOT NULL,
    sensor_type TEXT NOT NULL CHECK (sensor_type IN (
        'temperature', 'humidity', 'motion', 'co2', 'water_leak',
        'energy', 'smoke', 'door', 'window', 'light', 'pressure',
        'noise', 'air_quality', 'occupancy'
    )),
    location TEXT,
    location_description TEXT,
    -- Connection configuration (encrypted in app layer)
    connection_type TEXT NOT NULL DEFAULT 'api' CHECK (connection_type IN ('api', 'mqtt', 'webhook', 'polling')),
    connection_config JSONB DEFAULT '{}',
    api_key_hash TEXT,
    -- Data configuration
    unit_of_measurement TEXT,
    data_interval_seconds INTEGER DEFAULT 300,
    -- Status tracking
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'active', 'offline', 'error', 'maintenance', 'disabled')),
    last_seen_at TIMESTAMPTZ,
    last_reading_at TIMESTAMPTZ,
    last_error TEXT,
    error_count INTEGER DEFAULT 0,
    -- Metadata
    manufacturer TEXT,
    model TEXT,
    firmware_version TEXT,
    serial_number TEXT,
    installed_at TIMESTAMPTZ,
    metadata JSONB DEFAULT '{}',
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(id)
);

-- Sensor readings table (time-series data)
CREATE TABLE IF NOT EXISTS sensor_readings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sensor_id UUID NOT NULL REFERENCES sensors(id) ON DELETE CASCADE,
    value FLOAT NOT NULL,
    unit TEXT NOT NULL,
    quality TEXT DEFAULT 'good' CHECK (quality IN ('good', 'uncertain', 'bad')),
    raw_data JSONB,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Sensor thresholds configuration
CREATE TABLE IF NOT EXISTS sensor_thresholds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sensor_id UUID NOT NULL REFERENCES sensors(id) ON DELETE CASCADE,
    metric TEXT NOT NULL DEFAULT 'value',
    comparison TEXT NOT NULL CHECK (comparison IN ('gt', 'gte', 'lt', 'lte', 'eq', 'neq', 'range_outside', 'range_inside')),
    warning_value FLOAT,
    warning_high FLOAT,
    critical_value FLOAT,
    critical_high FLOAT,
    enabled BOOLEAN DEFAULT TRUE,
    alert_cooldown_minutes INTEGER DEFAULT 5,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (sensor_id, metric)
);

-- Sensor alerts
CREATE TABLE IF NOT EXISTS sensor_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sensor_id UUID NOT NULL REFERENCES sensors(id) ON DELETE CASCADE,
    threshold_id UUID NOT NULL REFERENCES sensor_thresholds(id) ON DELETE CASCADE,
    severity TEXT NOT NULL CHECK (severity IN ('warning', 'critical')),
    triggered_value FLOAT NOT NULL,
    threshold_value FLOAT NOT NULL,
    message TEXT,
    triggered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMPTZ,
    resolved_value FLOAT,
    acknowledged_by UUID REFERENCES users(id),
    acknowledged_at TIMESTAMPTZ,
    notification_sent BOOLEAN DEFAULT FALSE
);

-- Default threshold templates per sensor type
CREATE TABLE IF NOT EXISTS sensor_threshold_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE,
    sensor_type TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    comparison TEXT NOT NULL,
    warning_value FLOAT,
    warning_high FLOAT,
    critical_value FLOAT,
    critical_high FLOAT,
    is_default BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (organization_id, sensor_type, name)
);

-- Sensor-fault correlations
CREATE TABLE IF NOT EXISTS sensor_fault_correlations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sensor_id UUID NOT NULL REFERENCES sensors(id) ON DELETE CASCADE,
    fault_id UUID NOT NULL REFERENCES faults(id) ON DELETE CASCADE,
    correlation_type TEXT NOT NULL CHECK (correlation_type IN ('automatic', 'manual', 'suggested')),
    confidence FLOAT,
    sensor_data_start TIMESTAMPTZ,
    sensor_data_end TIMESTAMPTZ,
    summary TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id),
    UNIQUE (sensor_id, fault_id)
);

-- Indexes for sensor readings (optimized for time-series queries)
CREATE INDEX idx_sensor_readings_sensor_time ON sensor_readings(sensor_id, timestamp DESC);
CREATE INDEX idx_sensor_readings_timestamp ON sensor_readings(timestamp DESC);

-- Indexes for sensors
CREATE INDEX idx_sensors_org ON sensors(organization_id);
CREATE INDEX idx_sensors_building ON sensors(building_id);
CREATE INDEX idx_sensors_unit ON sensors(unit_id);
CREATE INDEX idx_sensors_type ON sensors(sensor_type);
CREATE INDEX idx_sensors_status ON sensors(status);

-- Indexes for alerts
CREATE INDEX idx_sensor_alerts_sensor ON sensor_alerts(sensor_id);
CREATE INDEX idx_sensor_alerts_triggered ON sensor_alerts(triggered_at DESC);
CREATE INDEX idx_sensor_alerts_unresolved ON sensor_alerts(sensor_id) WHERE resolved_at IS NULL;

-- Indexes for correlations
CREATE INDEX idx_sensor_correlations_sensor ON sensor_fault_correlations(sensor_id);
CREATE INDEX idx_sensor_correlations_fault ON sensor_fault_correlations(fault_id);

-- RLS policies
ALTER TABLE sensors ENABLE ROW LEVEL SECURITY;
ALTER TABLE sensor_readings ENABLE ROW LEVEL SECURITY;
ALTER TABLE sensor_thresholds ENABLE ROW LEVEL SECURITY;
ALTER TABLE sensor_alerts ENABLE ROW LEVEL SECURITY;
ALTER TABLE sensor_threshold_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE sensor_fault_correlations ENABLE ROW LEVEL SECURITY;

CREATE POLICY sensors_tenant_isolation ON sensors
    FOR ALL
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY sensor_readings_tenant_isolation ON sensor_readings
    FOR ALL
    USING (sensor_id IN (
        SELECT id FROM sensors
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

CREATE POLICY sensor_thresholds_tenant_isolation ON sensor_thresholds
    FOR ALL
    USING (sensor_id IN (
        SELECT id FROM sensors
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

CREATE POLICY sensor_alerts_tenant_isolation ON sensor_alerts
    FOR ALL
    USING (sensor_id IN (
        SELECT id FROM sensors
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

CREATE POLICY sensor_templates_tenant_isolation ON sensor_threshold_templates
    FOR ALL
    USING (
        organization_id IS NULL OR
        organization_id = current_setting('app.current_organization_id', true)::uuid
    );

CREATE POLICY sensor_correlations_tenant_isolation ON sensor_fault_correlations
    FOR ALL
    USING (sensor_id IN (
        SELECT id FROM sensors
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

-- Triggers for updated_at
CREATE TRIGGER update_sensors_updated_at
    BEFORE UPDATE ON sensors
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_sensor_thresholds_updated_at
    BEFORE UPDATE ON sensor_thresholds
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Insert default threshold templates
INSERT INTO sensor_threshold_templates (sensor_type, name, description, comparison, warning_value, critical_value, is_default)
VALUES
    ('temperature', 'Comfort (Celsius)', 'Standard comfort range', 'range_outside', 18.0, 15.0, true),
    ('temperature', 'Comfort (Celsius)', 'Standard comfort range', 'range_outside', 25.0, 30.0, true),
    ('humidity', 'Standard', 'Standard humidity range', 'range_outside', 30.0, 20.0, true),
    ('humidity', 'Standard', 'Standard humidity range', 'range_outside', 60.0, 70.0, true),
    ('co2', 'Indoor Air Quality', 'CO2 levels in ppm', 'gt', 1000.0, 2000.0, true),
    ('water_leak', 'Water Detection', 'Any water detected', 'gt', 0.0, 0.0, true),
    ('smoke', 'Smoke Detection', 'Any smoke detected', 'gt', 0.0, 0.0, true),
    ('energy', 'High Consumption', 'Unusual energy usage', 'gt', NULL, NULL, false),
    ('noise', 'Noise Levels', 'Decibel levels', 'gt', 60.0, 80.0, true),
    ('air_quality', 'AQI Standard', 'Air quality index', 'gt', 100.0, 150.0, true)
ON CONFLICT DO NOTHING;
