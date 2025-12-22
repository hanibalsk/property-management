-- Migration: 00041_create_meters_utilities
-- Epic 12: Meter Readings & Utilities
--
-- Stories covered:
-- - 12.1: Meter Registration
-- - 12.2: Self-Reading Submission
-- - 12.3: Reading Validation & Approval
-- - 12.4: Utility Cost Distribution
-- - 12.5: Consumption History & Analytics
-- - 12.6: Automatic Meter Reading Integration

-- ============================================================================
-- METER TYPES AND STATUS
-- ============================================================================

CREATE TYPE meter_type AS ENUM (
    'electricity',
    'gas',
    'water',
    'heat',
    'cold_water',
    'hot_water',
    'solar',
    'other'
);

CREATE TYPE reading_source AS ENUM (
    'manual',
    'photo',
    'automatic',
    'estimated'
);

CREATE TYPE reading_status AS ENUM (
    'pending',
    'approved',
    'rejected',
    'estimated'
);

CREATE TYPE distribution_method AS ENUM (
    'consumption',    -- Based on actual consumption
    'area',           -- Based on unit area (m²)
    'equal',          -- Equal split among units
    'occupants',      -- Based on number of occupants
    'hybrid'          -- Combination of methods
);

-- ============================================================================
-- METERS (Story 12.1)
-- ============================================================================

CREATE TABLE meters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    unit_id UUID REFERENCES units(id) ON DELETE CASCADE,  -- NULL for common area meters

    -- Meter info
    meter_number VARCHAR(100) NOT NULL,
    meter_type meter_type NOT NULL,
    location TEXT,
    description TEXT,

    -- Readings
    initial_reading DECIMAL(15, 3) NOT NULL DEFAULT 0,
    current_reading DECIMAL(15, 3),
    last_reading_date DATE,

    -- Unit of measurement
    unit_of_measure VARCHAR(20) NOT NULL DEFAULT 'kWh',  -- kWh, m³, GJ, etc.

    -- Smart meter config
    is_smart_meter BOOLEAN NOT NULL DEFAULT false,
    smart_meter_provider VARCHAR(100),
    smart_meter_api_key_encrypted TEXT,  -- Encrypted API key

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_shared BOOLEAN NOT NULL DEFAULT false,  -- Common area meter
    installed_at DATE,
    decommissioned_at DATE,

    -- Replaced meter reference
    replaced_meter_id UUID REFERENCES meters(id),
    replacement_reading DECIMAL(15, 3),  -- Final reading when replaced

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE(building_id, meter_number)
);

CREATE INDEX idx_meters_org ON meters(organization_id);
CREATE INDEX idx_meters_building ON meters(building_id);
CREATE INDEX idx_meters_unit ON meters(unit_id);
CREATE INDEX idx_meters_type ON meters(meter_type);
CREATE INDEX idx_meters_active ON meters(is_active);
CREATE INDEX idx_meters_shared ON meters(is_shared);

-- Trigger for updated_at
CREATE TRIGGER meters_updated_at
    BEFORE UPDATE ON meters
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- METER READINGS (Story 12.2)
-- ============================================================================

CREATE TABLE meter_readings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    meter_id UUID NOT NULL REFERENCES meters(id) ON DELETE CASCADE,

    -- Reading data
    reading DECIMAL(15, 3) NOT NULL,
    reading_date DATE NOT NULL DEFAULT CURRENT_DATE,
    reading_time TIME,

    -- Source and evidence
    source reading_source NOT NULL DEFAULT 'manual',
    photo_url TEXT,
    ocr_reading DECIMAL(15, 3),  -- OCR-extracted value

    -- Validation
    status reading_status NOT NULL DEFAULT 'pending',
    validated_by UUID REFERENCES users(id),
    validated_at TIMESTAMPTZ,
    validation_notes TEXT,

    -- Calculated consumption (current - previous)
    consumption DECIMAL(15, 3),
    previous_reading_id UUID REFERENCES meter_readings(id),

    -- Submission window
    submission_window_id UUID,  -- References submission_windows

    -- Anomaly detection
    is_anomaly BOOLEAN NOT NULL DEFAULT false,
    anomaly_reason TEXT,

    -- Metadata
    submitted_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_meter_readings_meter ON meter_readings(meter_id);
CREATE INDEX idx_meter_readings_date ON meter_readings(reading_date);
CREATE INDEX idx_meter_readings_status ON meter_readings(status);
CREATE INDEX idx_meter_readings_submission_window ON meter_readings(submission_window_id);
CREATE INDEX idx_meter_readings_submitted_by ON meter_readings(submitted_by);

-- Trigger for updated_at
CREATE TRIGGER meter_readings_updated_at
    BEFORE UPDATE ON meter_readings
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- READING SUBMISSION WINDOWS (Story 12.2)
-- ============================================================================

CREATE TABLE reading_submission_windows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,

    -- Window info
    name VARCHAR(255) NOT NULL,
    description TEXT,

    -- Period
    billing_period_start DATE NOT NULL,
    billing_period_end DATE NOT NULL,

    -- Submission window
    submission_start DATE NOT NULL,
    submission_end DATE NOT NULL,

    -- Status
    is_open BOOLEAN NOT NULL DEFAULT true,
    is_finalized BOOLEAN NOT NULL DEFAULT false,
    finalized_at TIMESTAMPTZ,
    finalized_by UUID REFERENCES users(id),

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_submission_windows_org ON reading_submission_windows(organization_id);
CREATE INDEX idx_submission_windows_building ON reading_submission_windows(building_id);
CREATE INDEX idx_submission_windows_open ON reading_submission_windows(is_open);

-- Trigger for updated_at
CREATE TRIGGER reading_submission_windows_updated_at
    BEFORE UPDATE ON reading_submission_windows
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- READING VALIDATION RULES (Story 12.3)
-- ============================================================================

CREATE TABLE reading_validation_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Rule config
    meter_type meter_type NOT NULL,
    rule_name VARCHAR(255) NOT NULL,

    -- Thresholds
    min_consumption_threshold DECIMAL(15, 3),  -- Flag if below
    max_consumption_threshold DECIMAL(15, 3),  -- Flag if above
    max_increase_percentage DECIMAL(5, 2),  -- Max % increase from average
    max_decrease_percentage DECIMAL(5, 2),  -- Max % decrease from average

    -- Comparison settings
    comparison_months INTEGER NOT NULL DEFAULT 12,  -- Months for historical average

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE(organization_id, meter_type, rule_name)
);

CREATE INDEX idx_validation_rules_org ON reading_validation_rules(organization_id);
CREATE INDEX idx_validation_rules_type ON reading_validation_rules(meter_type);

-- Trigger for updated_at
CREATE TRIGGER reading_validation_rules_updated_at
    BEFORE UPDATE ON reading_validation_rules
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- UTILITY BILLS (Story 12.4)
-- ============================================================================

CREATE TABLE utility_bills (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,

    -- Bill info
    bill_number VARCHAR(100),
    meter_type meter_type NOT NULL,
    provider_name VARCHAR(255),

    -- Period
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,

    -- Amounts
    total_amount DECIMAL(15, 2) NOT NULL,
    total_consumption DECIMAL(15, 3),
    unit_price DECIMAL(10, 4),  -- Price per unit
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',

    -- Distribution
    distribution_method distribution_method NOT NULL DEFAULT 'consumption',
    is_distributed BOOLEAN NOT NULL DEFAULT false,
    distributed_at TIMESTAMPTZ,
    distributed_by UUID REFERENCES users(id),

    -- File storage
    bill_file_path TEXT,

    -- Timestamps
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_utility_bills_org ON utility_bills(organization_id);
CREATE INDEX idx_utility_bills_building ON utility_bills(building_id);
CREATE INDEX idx_utility_bills_type ON utility_bills(meter_type);
CREATE INDEX idx_utility_bills_period ON utility_bills(period_start, period_end);

-- Trigger for updated_at
CREATE TRIGGER utility_bills_updated_at
    BEFORE UPDATE ON utility_bills
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- UTILITY BILL DISTRIBUTION (Story 12.4)
-- ============================================================================

CREATE TABLE utility_bill_distributions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    utility_bill_id UUID NOT NULL REFERENCES utility_bills(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,

    -- Distribution details
    consumption DECIMAL(15, 3),
    consumption_percentage DECIMAL(5, 2),
    amount DECIMAL(15, 2) NOT NULL,

    -- Distribution factors
    area_factor DECIMAL(10, 4),  -- Unit area / total area
    occupant_factor DECIMAL(10, 4),  -- Unit occupants / total occupants

    -- Invoice link
    invoice_item_id UUID REFERENCES invoice_items(id),

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE(utility_bill_id, unit_id)
);

CREATE INDEX idx_bill_distributions_bill ON utility_bill_distributions(utility_bill_id);
CREATE INDEX idx_bill_distributions_unit ON utility_bill_distributions(unit_id);

-- ============================================================================
-- CONSUMPTION ANALYTICS (Story 12.5)
-- ============================================================================

-- Pre-calculated monthly consumption aggregates for performance
CREATE TABLE consumption_aggregates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    meter_id UUID NOT NULL REFERENCES meters(id) ON DELETE CASCADE,

    -- Period
    year INTEGER NOT NULL,
    month INTEGER NOT NULL CHECK (month >= 1 AND month <= 12),

    -- Aggregated values
    total_consumption DECIMAL(15, 3) NOT NULL DEFAULT 0,
    reading_count INTEGER NOT NULL DEFAULT 0,
    avg_daily_consumption DECIMAL(15, 3),
    min_reading DECIMAL(15, 3),
    max_reading DECIMAL(15, 3),

    -- Comparison data
    building_avg_consumption DECIMAL(15, 3),
    percentile_rank DECIMAL(5, 2),  -- Where unit stands in building (0-100)

    -- Timestamps
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE(meter_id, year, month)
);

CREATE INDEX idx_consumption_aggregates_meter ON consumption_aggregates(meter_id);
CREATE INDEX idx_consumption_aggregates_period ON consumption_aggregates(year, month);

-- ============================================================================
-- SMART METER PROVIDERS (Story 12.6)
-- ============================================================================

CREATE TABLE smart_meter_providers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Provider info
    name VARCHAR(255) NOT NULL,
    provider_type VARCHAR(100) NOT NULL,  -- 'api', 'mqtt', 'modbus', etc.

    -- Authentication
    api_endpoint TEXT,
    api_key_encrypted TEXT,
    auth_type VARCHAR(50),  -- 'api_key', 'oauth', 'basic'

    -- Settings
    polling_interval_minutes INTEGER DEFAULT 60,
    is_active BOOLEAN NOT NULL DEFAULT true,

    -- Status
    last_sync_at TIMESTAMPTZ,
    last_error TEXT,
    error_count INTEGER NOT NULL DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_smart_meter_providers_org ON smart_meter_providers(organization_id);
CREATE INDEX idx_smart_meter_providers_active ON smart_meter_providers(is_active);

-- Trigger for updated_at
CREATE TRIGGER smart_meter_providers_updated_at
    BEFORE UPDATE ON smart_meter_providers
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- SMART METER READING LOGS (Story 12.6)
-- ============================================================================

CREATE TABLE smart_meter_reading_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    meter_id UUID NOT NULL REFERENCES meters(id) ON DELETE CASCADE,
    provider_id UUID NOT NULL REFERENCES smart_meter_providers(id) ON DELETE CASCADE,

    -- Reading data
    reading DECIMAL(15, 3) NOT NULL,
    reading_timestamp TIMESTAMPTZ NOT NULL,
    raw_data JSONB,

    -- Processing status
    processed BOOLEAN NOT NULL DEFAULT false,
    meter_reading_id UUID REFERENCES meter_readings(id),
    error_message TEXT,

    -- Timestamps
    received_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_smart_reading_logs_meter ON smart_meter_reading_logs(meter_id);
CREATE INDEX idx_smart_reading_logs_provider ON smart_meter_reading_logs(provider_id);
CREATE INDEX idx_smart_reading_logs_timestamp ON smart_meter_reading_logs(reading_timestamp);
CREATE INDEX idx_smart_reading_logs_processed ON smart_meter_reading_logs(processed);

-- ============================================================================
-- MISSING READING ALERTS (Story 12.6)
-- ============================================================================

CREATE TABLE missing_reading_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    meter_id UUID NOT NULL REFERENCES meters(id) ON DELETE CASCADE,

    -- Alert info
    expected_date DATE NOT NULL,
    alert_type VARCHAR(50) NOT NULL,  -- 'smart_meter_failure', 'submission_missed'
    message TEXT,

    -- Resolution
    is_resolved BOOLEAN NOT NULL DEFAULT false,
    resolved_at TIMESTAMPTZ,
    resolved_by UUID REFERENCES users(id),
    resolution_notes TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_missing_reading_alerts_meter ON missing_reading_alerts(meter_id);
CREATE INDEX idx_missing_reading_alerts_resolved ON missing_reading_alerts(is_resolved);
CREATE INDEX idx_missing_reading_alerts_date ON missing_reading_alerts(expected_date);

-- ============================================================================
-- RLS POLICIES
-- ============================================================================

ALTER TABLE meters ENABLE ROW LEVEL SECURITY;
ALTER TABLE meter_readings ENABLE ROW LEVEL SECURITY;
ALTER TABLE reading_submission_windows ENABLE ROW LEVEL SECURITY;
ALTER TABLE reading_validation_rules ENABLE ROW LEVEL SECURITY;
ALTER TABLE utility_bills ENABLE ROW LEVEL SECURITY;
ALTER TABLE utility_bill_distributions ENABLE ROW LEVEL SECURITY;
ALTER TABLE consumption_aggregates ENABLE ROW LEVEL SECURITY;
ALTER TABLE smart_meter_providers ENABLE ROW LEVEL SECURITY;
ALTER TABLE smart_meter_reading_logs ENABLE ROW LEVEL SECURITY;
ALTER TABLE missing_reading_alerts ENABLE ROW LEVEL SECURITY;

-- Meters: org members can view, admins/managers can manage
CREATE POLICY meters_select ON meters
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = meters.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

CREATE POLICY meters_manage ON meters
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = meters.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = meters.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- Meter readings: org members can view and submit, managers approve
CREATE POLICY meter_readings_select ON meter_readings
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM meters m
            JOIN organization_members om ON om.organization_id = m.organization_id
            WHERE m.id = meter_readings.meter_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

CREATE POLICY meter_readings_insert ON meter_readings
    FOR INSERT
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM meters m
            JOIN organization_members om ON om.organization_id = m.organization_id
            WHERE m.id = meter_readings.meter_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

CREATE POLICY meter_readings_manage ON meter_readings
    FOR UPDATE
    USING (
        EXISTS (
            SELECT 1 FROM meters m
            JOIN organization_members om ON om.organization_id = m.organization_id
            WHERE m.id = meter_readings.meter_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- Submission windows: org members can view, admins/managers manage
CREATE POLICY submission_windows_select ON reading_submission_windows
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = reading_submission_windows.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

CREATE POLICY submission_windows_manage ON reading_submission_windows
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = reading_submission_windows.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = reading_submission_windows.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- Validation rules: admins/managers only
CREATE POLICY validation_rules_select ON reading_validation_rules
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = reading_validation_rules.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

CREATE POLICY validation_rules_manage ON reading_validation_rules
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = reading_validation_rules.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = reading_validation_rules.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- Utility bills: admins/managers manage
CREATE POLICY utility_bills_select ON utility_bills
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = utility_bills.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

CREATE POLICY utility_bills_manage ON utility_bills
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = utility_bills.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = utility_bills.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- Bill distributions inherit from utility bill
CREATE POLICY bill_distributions_select ON utility_bill_distributions
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM utility_bills ub
            JOIN organization_members om ON om.organization_id = ub.organization_id
            WHERE ub.id = utility_bill_distributions.utility_bill_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

CREATE POLICY bill_distributions_manage ON utility_bill_distributions
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM utility_bills ub
            JOIN organization_members om ON om.organization_id = ub.organization_id
            WHERE ub.id = utility_bill_distributions.utility_bill_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM utility_bills ub
            JOIN organization_members om ON om.organization_id = ub.organization_id
            WHERE ub.id = utility_bill_distributions.utility_bill_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- Consumption aggregates: members can view own unit's data
CREATE POLICY consumption_aggregates_select ON consumption_aggregates
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM meters m
            JOIN organization_members om ON om.organization_id = m.organization_id
            WHERE m.id = consumption_aggregates.meter_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

CREATE POLICY consumption_aggregates_manage ON consumption_aggregates
    FOR ALL
    USING (is_super_admin())
    WITH CHECK (is_super_admin());

-- Smart meter providers: admins only
CREATE POLICY smart_meter_providers_select ON smart_meter_providers
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = smart_meter_providers.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type = 'org_admin'
        )
        OR is_super_admin()
    );

CREATE POLICY smart_meter_providers_manage ON smart_meter_providers
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = smart_meter_providers.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type = 'org_admin'
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = smart_meter_providers.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type = 'org_admin'
        )
        OR is_super_admin()
    );

-- Smart meter reading logs: system managed
CREATE POLICY smart_reading_logs_select ON smart_meter_reading_logs
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM meters m
            JOIN organization_members om ON om.organization_id = m.organization_id
            WHERE m.id = smart_meter_reading_logs.meter_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

CREATE POLICY smart_reading_logs_manage ON smart_meter_reading_logs
    FOR ALL
    USING (is_super_admin())
    WITH CHECK (is_super_admin());

-- Missing reading alerts
CREATE POLICY missing_reading_alerts_select ON missing_reading_alerts
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM meters m
            JOIN organization_members om ON om.organization_id = m.organization_id
            WHERE m.id = missing_reading_alerts.meter_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

CREATE POLICY missing_reading_alerts_manage ON missing_reading_alerts
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM meters m
            JOIN organization_members om ON om.organization_id = m.organization_id
            WHERE m.id = missing_reading_alerts.meter_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM meters m
            JOIN organization_members om ON om.organization_id = m.organization_id
            WHERE m.id = missing_reading_alerts.meter_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- ============================================================================
-- HELPER FUNCTION: Calculate consumption from readings
-- ============================================================================

CREATE OR REPLACE FUNCTION calculate_reading_consumption()
RETURNS TRIGGER AS $$
DECLARE
    prev_reading RECORD;
BEGIN
    -- Find previous reading for this meter
    SELECT id, reading INTO prev_reading
    FROM meter_readings
    WHERE meter_id = NEW.meter_id
    AND reading_date < NEW.reading_date
    ORDER BY reading_date DESC, created_at DESC
    LIMIT 1;

    IF prev_reading IS NOT NULL THEN
        NEW.previous_reading_id := prev_reading.id;
        NEW.consumption := NEW.reading - prev_reading.reading;

        -- Flag if consumption is negative (reading went backwards)
        IF NEW.consumption < 0 THEN
            NEW.is_anomaly := true;
            NEW.anomaly_reason := 'Reading is less than previous reading';
        END IF;
    END IF;

    -- Update meter's current reading
    UPDATE meters
    SET
        current_reading = NEW.reading,
        last_reading_date = NEW.reading_date
    WHERE id = NEW.meter_id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER calculate_consumption_on_insert
    BEFORE INSERT ON meter_readings
    FOR EACH ROW
    EXECUTE FUNCTION calculate_reading_consumption();

-- ============================================================================
-- HELPER FUNCTION: Detect anomalies in readings
-- ============================================================================

CREATE OR REPLACE FUNCTION detect_reading_anomalies()
RETURNS TRIGGER AS $$
DECLARE
    avg_consumption DECIMAL(15, 3);
    max_threshold DECIMAL(15, 3);
    min_threshold DECIMAL(15, 3);
    rule RECORD;
    meter_rec RECORD;
BEGIN
    -- Get meter type
    SELECT meter_type INTO meter_rec FROM meters WHERE id = NEW.meter_id;

    -- Get validation rules
    SELECT * INTO rule
    FROM reading_validation_rules
    WHERE organization_id = (SELECT organization_id FROM meters WHERE id = NEW.meter_id)
    AND meter_type = meter_rec.meter_type
    AND is_active = true
    LIMIT 1;

    IF rule IS NOT NULL AND NEW.consumption IS NOT NULL THEN
        -- Calculate average consumption over comparison months
        SELECT AVG(consumption) INTO avg_consumption
        FROM meter_readings
        WHERE meter_id = NEW.meter_id
        AND consumption IS NOT NULL
        AND reading_date > CURRENT_DATE - (rule.comparison_months || ' months')::INTERVAL;

        IF avg_consumption IS NOT NULL AND avg_consumption > 0 THEN
            -- Check max increase
            IF rule.max_increase_percentage IS NOT NULL THEN
                max_threshold := avg_consumption * (1 + rule.max_increase_percentage / 100);
                IF NEW.consumption > max_threshold THEN
                    NEW.is_anomaly := true;
                    NEW.anomaly_reason := COALESCE(NEW.anomaly_reason || '; ', '') ||
                        'Consumption ' || ROUND(NEW.consumption, 2)::TEXT ||
                        ' exceeds max threshold ' || ROUND(max_threshold, 2)::TEXT;
                END IF;
            END IF;

            -- Check max decrease
            IF rule.max_decrease_percentage IS NOT NULL THEN
                min_threshold := avg_consumption * (1 - rule.max_decrease_percentage / 100);
                IF NEW.consumption < min_threshold AND NEW.consumption >= 0 THEN
                    NEW.is_anomaly := true;
                    NEW.anomaly_reason := COALESCE(NEW.anomaly_reason || '; ', '') ||
                        'Consumption ' || ROUND(NEW.consumption, 2)::TEXT ||
                        ' below min threshold ' || ROUND(min_threshold, 2)::TEXT;
                END IF;
            END IF;
        END IF;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER detect_anomalies_on_insert
    BEFORE INSERT ON meter_readings
    FOR EACH ROW
    EXECUTE FUNCTION detect_reading_anomalies();
