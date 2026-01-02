-- Epic 102: Distributed Tracing & Observability (Story 102.3)
-- Migration: Create health alert rules table for alert management

-- Create health alert rules table
CREATE TABLE IF NOT EXISTS health_alert_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    condition TEXT NOT NULL,
    severity alert_severity NOT NULL DEFAULT 'warning',
    notification_channels JSONB NOT NULL DEFAULT '[]',
    enabled BOOLEAN NOT NULL DEFAULT true,
    cooldown_seconds INTEGER NOT NULL DEFAULT 300,
    last_triggered_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT health_alert_rules_name_unique UNIQUE(name),
    CONSTRAINT health_alert_rules_cooldown_positive CHECK (cooldown_seconds > 0)
);

-- Create index for enabled rules
CREATE INDEX IF NOT EXISTS idx_health_alert_rules_enabled ON health_alert_rules(enabled) WHERE enabled = true;
CREATE INDEX IF NOT EXISTS idx_health_alert_rules_severity ON health_alert_rules(severity);
CREATE INDEX IF NOT EXISTS idx_health_alert_rules_last_triggered ON health_alert_rules(last_triggered_at);

-- Add comments for documentation
COMMENT ON TABLE health_alert_rules IS 'Stores alert rule definitions for health monitoring (Epic 102, Story 102.3)';
COMMENT ON COLUMN health_alert_rules.condition IS 'Alert condition expression (e.g., "cpu_usage > 80", "error_rate > 5")';
COMMENT ON COLUMN health_alert_rules.notification_channels IS 'JSON array of notification channels (e.g., ["email", "slack"])';
COMMENT ON COLUMN health_alert_rules.cooldown_seconds IS 'Minimum time between alert triggers to prevent spam';
COMMENT ON COLUMN health_alert_rules.last_triggered_at IS 'Last time this rule triggered an alert';

-- Add RLS policy
ALTER TABLE health_alert_rules ENABLE ROW LEVEL SECURITY;

-- Super admins can manage alert rules
CREATE POLICY health_alert_rules_admin_all ON health_alert_rules
    FOR ALL
    USING (is_current_user_super_admin())
    WITH CHECK (is_current_user_super_admin());

-- All authenticated users can read alert rules
CREATE POLICY health_alert_rules_read ON health_alert_rules
    FOR SELECT
    USING (true);

-- Insert some default alert rules
INSERT INTO health_alert_rules (name, description, condition, severity, notification_channels, enabled)
VALUES
    (
        'High CPU Usage',
        'Alert when CPU usage exceeds 80%',
        'cpu_usage > 80',
        'warning',
        '["email"]',
        true
    ),
    (
        'Critical CPU Usage',
        'Alert when CPU usage exceeds 95%',
        'cpu_usage > 95',
        'critical',
        '["email", "pagerduty"]',
        true
    ),
    (
        'High Error Rate',
        'Alert when error rate exceeds 5%',
        'error_rate > 5',
        'warning',
        '["email"]',
        true
    ),
    (
        'Critical Error Rate',
        'Alert when error rate exceeds 10%',
        'error_rate > 10',
        'critical',
        '["email", "pagerduty"]',
        true
    ),
    (
        'Database Connection Pool Low',
        'Alert when available database connections are low',
        'db_connections_available < 5',
        'warning',
        '["email"]',
        true
    ),
    (
        'Slow Response Time',
        'Alert when average response time exceeds 2 seconds',
        'avg_response_time_ms > 2000',
        'warning',
        '["email"]',
        true
    ),
    (
        'Failed Background Jobs',
        'Alert when failed jobs count exceeds threshold',
        'failed_jobs_24h > 100',
        'warning',
        '["email"]',
        true
    )
ON CONFLICT (name) DO NOTHING;
