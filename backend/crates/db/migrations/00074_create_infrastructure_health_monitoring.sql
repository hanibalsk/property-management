-- Epic 89: Feature Flags & Health Monitoring (Story 89.3-89.4)
-- Migration: Create infrastructure health monitoring tables

-- Create enum for health check type
DO $$ BEGIN
    CREATE TYPE health_check_type AS ENUM (
        'http',
        'tcp',
        'database',
        'redis',
        's3',
        'custom'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- Create enum for health status
DO $$ BEGIN
    CREATE TYPE health_status AS ENUM (
        'healthy',
        'degraded',
        'unhealthy',
        'unknown'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- Create enum for alert severity
DO $$ BEGIN
    CREATE TYPE alert_severity AS ENUM (
        'info',
        'warning',
        'critical'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- Create enum for alert status
DO $$ BEGIN
    CREATE TYPE alert_status AS ENUM (
        'active',
        'acknowledged',
        'resolved',
        'silenced'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- Health check configurations table
CREATE TABLE IF NOT EXISTS health_check_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    check_type health_check_type NOT NULL,
    endpoint VARCHAR(500) NOT NULL,
    interval_seconds INTEGER NOT NULL DEFAULT 60,
    timeout_ms INTEGER NOT NULL DEFAULT 5000,
    failure_threshold INTEGER NOT NULL DEFAULT 3,
    success_threshold INTEGER NOT NULL DEFAULT 1,
    enabled BOOLEAN NOT NULL DEFAULT true,
    config JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Health check results table
CREATE TABLE IF NOT EXISTS health_check_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    config_id UUID NOT NULL REFERENCES health_check_configs(id) ON DELETE CASCADE,
    status health_status NOT NULL,
    latency_ms BIGINT,
    error_message TEXT,
    response_details JSONB,
    checked_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Health alert rules table
CREATE TABLE IF NOT EXISTS health_alert_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    condition VARCHAR(500) NOT NULL,
    severity alert_severity NOT NULL DEFAULT 'warning',
    notification_channels JSONB NOT NULL DEFAULT '[]'::JSONB,
    enabled BOOLEAN NOT NULL DEFAULT true,
    cooldown_seconds INTEGER NOT NULL DEFAULT 300,
    last_triggered_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Health alerts table
CREATE TABLE IF NOT EXISTS health_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rule_id UUID NOT NULL REFERENCES health_alert_rules(id) ON DELETE CASCADE,
    status alert_status NOT NULL DEFAULT 'active',
    severity alert_severity NOT NULL,
    message TEXT NOT NULL,
    context JSONB,
    triggered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    acknowledged_at TIMESTAMPTZ,
    acknowledged_by UUID REFERENCES users(id) ON DELETE SET NULL,
    acknowledged_note TEXT,
    resolved_at TIMESTAMPTZ,
    resolved_note TEXT
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_health_check_configs_enabled ON health_check_configs(enabled);
CREATE INDEX IF NOT EXISTS idx_health_check_configs_check_type ON health_check_configs(check_type);
CREATE INDEX IF NOT EXISTS idx_health_check_results_config_id ON health_check_results(config_id);
CREATE INDEX IF NOT EXISTS idx_health_check_results_checked_at ON health_check_results(checked_at DESC);
CREATE INDEX IF NOT EXISTS idx_health_check_results_status ON health_check_results(status);
CREATE INDEX IF NOT EXISTS idx_health_alert_rules_enabled ON health_alert_rules(enabled);
CREATE INDEX IF NOT EXISTS idx_health_alerts_status ON health_alerts(status);
CREATE INDEX IF NOT EXISTS idx_health_alerts_triggered_at ON health_alerts(triggered_at DESC);
CREATE INDEX IF NOT EXISTS idx_health_alerts_severity ON health_alerts(severity);

-- Auto-update triggers
CREATE OR REPLACE FUNCTION update_health_check_config_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_health_check_configs_updated_at ON health_check_configs;
CREATE TRIGGER trigger_health_check_configs_updated_at
    BEFORE UPDATE ON health_check_configs
    FOR EACH ROW
    EXECUTE FUNCTION update_health_check_config_updated_at();

DROP TRIGGER IF EXISTS trigger_health_alert_rules_updated_at ON health_alert_rules;
CREATE TRIGGER trigger_health_alert_rules_updated_at
    BEFORE UPDATE ON health_alert_rules
    FOR EACH ROW
    EXECUTE FUNCTION update_health_check_config_updated_at();

-- Seed default health check configurations
INSERT INTO health_check_configs (name, check_type, endpoint, interval_seconds, timeout_ms) VALUES
    ('Database', 'database', 'postgresql://localhost/ppt', 30, 5000),
    ('Redis', 'redis', 'redis://localhost:6379', 30, 3000),
    ('S3 Storage', 's3', 's3://bucket/health', 60, 10000)
ON CONFLICT DO NOTHING;

-- Seed default alert rules
INSERT INTO health_alert_rules (name, description, condition, severity, notification_channels) VALUES
    ('High CPU Usage', 'Alert when CPU usage exceeds 80%', 'cpu_usage > 80', 'warning', '["email", "slack"]'::JSONB),
    ('Critical CPU Usage', 'Alert when CPU usage exceeds 95%', 'cpu_usage > 95', 'critical', '["email", "slack", "pagerduty"]'::JSONB),
    ('High Memory Usage', 'Alert when memory usage exceeds 85%', 'memory_usage > 85', 'warning', '["email", "slack"]'::JSONB),
    ('Database Unhealthy', 'Alert when database health check fails', 'database_status = unhealthy', 'critical', '["email", "slack", "pagerduty"]'::JSONB),
    ('High Error Rate', 'Alert when error rate exceeds 5%', 'error_rate > 5', 'warning', '["email", "slack"]'::JSONB)
ON CONFLICT DO NOTHING;

-- Add comments
COMMENT ON TABLE health_check_configs IS 'Configuration for health check endpoints and their thresholds';
COMMENT ON TABLE health_check_results IS 'Historical results of health checks';
COMMENT ON TABLE health_alert_rules IS 'Rules that define when alerts should be triggered';
COMMENT ON TABLE health_alerts IS 'Instances of alerts triggered by rules';

-- Function to cleanup old health check results
CREATE OR REPLACE FUNCTION cleanup_old_health_check_results(retention_days INTEGER DEFAULT 30)
RETURNS BIGINT AS $$
DECLARE
    deleted_count BIGINT;
BEGIN
    DELETE FROM health_check_results
    WHERE checked_at < NOW() - (retention_days || ' days')::INTERVAL;

    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION cleanup_old_health_check_results IS 'Removes health check results older than the specified retention period';
