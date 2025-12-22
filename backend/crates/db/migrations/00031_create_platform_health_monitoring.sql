-- Epic 10B, Story 10B.3: Platform Health Monitoring
-- Migration: Create health monitoring infrastructure

-- Create platform_metrics table for time-series metrics storage
CREATE TABLE IF NOT EXISTS platform_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    metric_type VARCHAR(50) NOT NULL,
    metric_name VARCHAR(100) NOT NULL,
    value NUMERIC NOT NULL,
    metadata JSONB,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create metric_thresholds table for configurable alert thresholds
CREATE TABLE IF NOT EXISTS metric_thresholds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    metric_name VARCHAR(100) NOT NULL UNIQUE,
    warning_threshold NUMERIC,
    critical_threshold NUMERIC,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create metric_alerts table for threshold breach tracking
CREATE TABLE IF NOT EXISTS metric_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    metric_name VARCHAR(100) NOT NULL,
    threshold_type VARCHAR(20) NOT NULL CHECK (threshold_type IN ('warning', 'critical')),
    value NUMERIC NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    acknowledged_at TIMESTAMPTZ,
    acknowledged_by UUID REFERENCES users(id)
);

-- Create indexes for efficient time-range queries
CREATE INDEX IF NOT EXISTS idx_platform_metrics_name_time ON platform_metrics(metric_name, recorded_at DESC);
CREATE INDEX IF NOT EXISTS idx_platform_metrics_type_time ON platform_metrics(metric_type, recorded_at DESC);
CREATE INDEX IF NOT EXISTS idx_platform_metrics_recorded_at ON platform_metrics(recorded_at DESC);
CREATE INDEX IF NOT EXISTS idx_metric_alerts_unacknowledged ON metric_alerts(acknowledged_at) WHERE acknowledged_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_metric_alerts_created_at ON metric_alerts(created_at DESC);

-- Seed initial metric thresholds
INSERT INTO metric_thresholds (metric_name, warning_threshold, critical_threshold, is_active) VALUES
    ('api_latency_p99', 500, 1000, true),  -- milliseconds
    ('api_error_rate', 1.0, 5.0, true),     -- percentage
    ('active_users', NULL, NULL, true),     -- informational, no thresholds
    ('db_connection_pool_usage', 70, 90, true),  -- percentage
    ('memory_usage', 70, 90, true),         -- percentage
    ('disk_usage', 80, 95, true),           -- percentage
    ('queue_depth', 1000, 5000, true)       -- count
ON CONFLICT (metric_name) DO NOTHING;

-- Add comments for documentation
COMMENT ON TABLE platform_metrics IS 'Time-series metrics for platform health monitoring (Epic 10B, Story 10B.3)';
COMMENT ON TABLE metric_thresholds IS 'Configurable thresholds for metric alerts';
COMMENT ON TABLE metric_alerts IS 'Alerts generated when metrics exceed thresholds';
COMMENT ON COLUMN platform_metrics.metadata IS 'Additional context for the metric (e.g., endpoint, error type)';
