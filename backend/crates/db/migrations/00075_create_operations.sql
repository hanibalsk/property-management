-- Epic 73: Infrastructure & Operations (Story 73.1-73.4)
-- Creates tables for blue-green deployments, database migrations,
-- disaster recovery, and cost monitoring.

-- =============================================================================
-- ENUMS
-- =============================================================================

DO $$ BEGIN
    CREATE TYPE deployment_environment AS ENUM ('blue', 'green');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE deployment_status AS ENUM (
        'pending', 'deploying', 'health_checking', 'switching',
        'active', 'rolled_back', 'failed'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE migration_status AS ENUM (
        'pending', 'running', 'completed', 'rolled_back', 'failed'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE migration_strategy AS ENUM (
        'standard', 'expand_contract', 'online_ddl', 'shadow_table'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE backup_type AS ENUM (
        'full', 'incremental', 'differential', 'point_in_time'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE backup_status AS ENUM (
        'in_progress', 'completed', 'verified', 'failed', 'expired'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE recovery_status AS ENUM (
        'initiated', 'restoring', 'validating', 'completed', 'failed'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE cost_alert_severity AS ENUM ('info', 'warning', 'critical');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE cloud_service_type AS ENUM (
        'compute', 'database', 'storage', 'network', 'cache', 'cdn', 'monitoring', 'other'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- =============================================================================
-- BLUE-GREEN DEPLOYMENTS (Story 73.1)
-- =============================================================================

CREATE TABLE IF NOT EXISTS deployments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    version VARCHAR(100) NOT NULL,
    environment deployment_environment NOT NULL,
    status deployment_status NOT NULL DEFAULT 'pending',
    previous_version VARCHAR(100),
    git_commit VARCHAR(100),
    git_branch VARCHAR(255),
    deployed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    switched_at TIMESTAMPTZ,
    rolled_back_at TIMESTAMPTZ,
    error_message TEXT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_deployments_environment ON deployments(environment);
CREATE INDEX IF NOT EXISTS idx_deployments_status ON deployments(status);
CREATE INDEX IF NOT EXISTS idx_deployments_started_at ON deployments(started_at DESC);

CREATE TABLE IF NOT EXISTS deployment_health_checks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    deployment_id UUID NOT NULL REFERENCES deployments(id) ON DELETE CASCADE,
    check_name VARCHAR(100) NOT NULL,
    is_healthy BOOLEAN NOT NULL DEFAULT false,
    response_time_ms INTEGER,
    status_code INTEGER,
    error_message TEXT,
    checked_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_deployment_health_checks_deployment ON deployment_health_checks(deployment_id);
CREATE INDEX IF NOT EXISTS idx_deployment_health_checks_checked_at ON deployment_health_checks(checked_at DESC);

-- =============================================================================
-- DATABASE MIGRATIONS (Story 73.2)
-- =============================================================================

CREATE TABLE IF NOT EXISTS database_migrations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    version VARCHAR(100) NOT NULL UNIQUE,
    strategy migration_strategy NOT NULL DEFAULT 'standard',
    status migration_status NOT NULL DEFAULT 'pending',
    is_backward_compatible BOOLEAN NOT NULL DEFAULT true,
    estimated_duration_secs INTEGER,
    actual_duration_secs INTEGER,
    affected_tables TEXT[],
    rollback_sql TEXT,
    executed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    error_message TEXT,
    progress_percentage INTEGER DEFAULT 0,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_database_migrations_status ON database_migrations(status);
CREATE INDEX IF NOT EXISTS idx_database_migrations_version ON database_migrations(version);

CREATE TABLE IF NOT EXISTS migration_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    migration_id UUID NOT NULL REFERENCES database_migrations(id) ON DELETE CASCADE,
    message TEXT NOT NULL,
    log_level VARCHAR(20) NOT NULL DEFAULT 'info',
    details JSONB,
    logged_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_migration_logs_migration ON migration_logs(migration_id);

CREATE TABLE IF NOT EXISTS schema_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    version VARCHAR(100) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    checksum VARCHAR(64) NOT NULL,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    applied_by UUID REFERENCES users(id) ON DELETE SET NULL
);

-- =============================================================================
-- DISASTER RECOVERY (Story 73.3)
-- =============================================================================

CREATE TABLE IF NOT EXISTS backups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    backup_type backup_type NOT NULL DEFAULT 'full',
    status backup_status NOT NULL DEFAULT 'in_progress',
    size_bytes BIGINT NOT NULL DEFAULT 0,
    storage_location TEXT,
    storage_region VARCHAR(50),
    is_encrypted BOOLEAN NOT NULL DEFAULT true,
    encryption_key_id VARCHAR(255),
    checksum VARCHAR(128),
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    verified_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    error_message TEXT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_backups_status ON backups(status);
CREATE INDEX IF NOT EXISTS idx_backups_type ON backups(backup_type);
CREATE INDEX IF NOT EXISTS idx_backups_started_at ON backups(started_at DESC);
CREATE INDEX IF NOT EXISTS idx_backups_expires_at ON backups(expires_at) WHERE expires_at IS NOT NULL;

CREATE TABLE IF NOT EXISTS recovery_operations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    backup_id UUID NOT NULL REFERENCES backups(id) ON DELETE CASCADE,
    status recovery_status NOT NULL DEFAULT 'initiated',
    target_point_in_time TIMESTAMPTZ,
    initiated_by UUID REFERENCES users(id) ON DELETE SET NULL,
    reason TEXT,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    data_loss_window_secs INTEGER,
    recovery_time_secs INTEGER,
    error_message TEXT,
    validation_result JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_recovery_operations_backup ON recovery_operations(backup_id);
CREATE INDEX IF NOT EXISTS idx_recovery_operations_status ON recovery_operations(status);

CREATE TABLE IF NOT EXISTS dr_drills (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    drill_type VARCHAR(100) NOT NULL,
    is_successful BOOLEAN NOT NULL,
    rto_target_secs INTEGER NOT NULL,
    rto_actual_secs INTEGER NOT NULL,
    rpo_target_secs INTEGER NOT NULL,
    rpo_actual_secs INTEGER NOT NULL,
    conducted_by UUID REFERENCES users(id) ON DELETE SET NULL,
    conducted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    findings TEXT,
    improvements TEXT,
    next_drill_due DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_dr_drills_conducted_at ON dr_drills(conducted_at DESC);

-- =============================================================================
-- COST MONITORING (Story 73.4)
-- =============================================================================

CREATE TABLE IF NOT EXISTS infrastructure_costs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_type cloud_service_type NOT NULL,
    service_name VARCHAR(255) NOT NULL,
    resource_id VARCHAR(255),
    resource_tags JSONB,
    cost_amount DECIMAL(12, 4) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',
    usage_quantity DECIMAL(18, 6) NOT NULL,
    usage_unit VARCHAR(50),
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    region VARCHAR(50),
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_infrastructure_costs_service ON infrastructure_costs(service_type);
CREATE INDEX IF NOT EXISTS idx_infrastructure_costs_period ON infrastructure_costs(period_start, period_end);
CREATE INDEX IF NOT EXISTS idx_infrastructure_costs_recorded ON infrastructure_costs(recorded_at DESC);

CREATE TABLE IF NOT EXISTS cost_budgets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    budget_amount DECIMAL(12, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',
    period_type VARCHAR(20) NOT NULL DEFAULT 'monthly',
    current_spend DECIMAL(12, 2) NOT NULL DEFAULT 0,
    forecasted_spend DECIMAL(12, 2) NOT NULL DEFAULT 0,
    alert_threshold_percent INTEGER NOT NULL DEFAULT 80,
    is_exceeded BOOLEAN NOT NULL DEFAULT false,
    service_type_filter cloud_service_type,
    tags_filter JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS cost_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    budget_id UUID REFERENCES cost_budgets(id) ON DELETE SET NULL,
    severity cost_alert_severity NOT NULL DEFAULT 'info',
    message TEXT NOT NULL,
    current_amount DECIMAL(12, 2) NOT NULL,
    threshold_amount DECIMAL(12, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',
    is_acknowledged BOOLEAN NOT NULL DEFAULT false,
    acknowledged_by UUID REFERENCES users(id) ON DELETE SET NULL,
    acknowledged_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_cost_alerts_budget ON cost_alerts(budget_id);
CREATE INDEX IF NOT EXISTS idx_cost_alerts_severity ON cost_alerts(severity);
CREATE INDEX IF NOT EXISTS idx_cost_alerts_acknowledged ON cost_alerts(is_acknowledged);

CREATE TABLE IF NOT EXISTS resource_utilization (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_type cloud_service_type NOT NULL,
    resource_id VARCHAR(255) NOT NULL,
    resource_name VARCHAR(255) NOT NULL,
    cpu_utilization_percent DECIMAL(5, 2) NOT NULL DEFAULT 0,
    memory_utilization_percent DECIMAL(5, 2) NOT NULL DEFAULT 0,
    storage_utilization_percent DECIMAL(5, 2),
    network_in_bytes BIGINT,
    network_out_bytes BIGINT,
    is_underutilized BOOLEAN NOT NULL DEFAULT false,
    is_overutilized BOOLEAN NOT NULL DEFAULT false,
    measured_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_resource_utilization_service ON resource_utilization(service_type);
CREATE INDEX IF NOT EXISTS idx_resource_utilization_underutilized ON resource_utilization(is_underutilized) WHERE is_underutilized = true;
CREATE INDEX IF NOT EXISTS idx_resource_utilization_measured ON resource_utilization(measured_at DESC);

CREATE TABLE IF NOT EXISTS cost_optimization_recommendations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    resource_id VARCHAR(255) NOT NULL,
    resource_name VARCHAR(255) NOT NULL,
    service_type cloud_service_type NOT NULL,
    recommendation_type VARCHAR(100) NOT NULL,
    description TEXT NOT NULL,
    estimated_savings DECIMAL(12, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',
    effort_level VARCHAR(20) NOT NULL DEFAULT 'medium',
    is_implemented BOOLEAN NOT NULL DEFAULT false,
    implemented_at TIMESTAMPTZ,
    implemented_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_cost_recommendations_implemented ON cost_optimization_recommendations(is_implemented);
CREATE INDEX IF NOT EXISTS idx_cost_recommendations_service ON cost_optimization_recommendations(service_type);

-- =============================================================================
-- TRIGGERS
-- =============================================================================

-- Auto-update updated_at for deployments
CREATE OR REPLACE FUNCTION update_deployments_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_deployments_updated_at ON deployments;
CREATE TRIGGER trigger_deployments_updated_at
    BEFORE UPDATE ON deployments
    FOR EACH ROW
    EXECUTE FUNCTION update_deployments_updated_at();

-- Auto-update updated_at for database_migrations
DROP TRIGGER IF EXISTS trigger_database_migrations_updated_at ON database_migrations;
CREATE TRIGGER trigger_database_migrations_updated_at
    BEFORE UPDATE ON database_migrations
    FOR EACH ROW
    EXECUTE FUNCTION update_deployments_updated_at();

-- Auto-update updated_at for cost_budgets
DROP TRIGGER IF EXISTS trigger_cost_budgets_updated_at ON cost_budgets;
CREATE TRIGGER trigger_cost_budgets_updated_at
    BEFORE UPDATE ON cost_budgets
    FOR EACH ROW
    EXECUTE FUNCTION update_deployments_updated_at();

-- =============================================================================
-- COMMENTS
-- =============================================================================

COMMENT ON TABLE deployments IS 'Blue-green deployment records for zero-downtime releases';
COMMENT ON TABLE deployment_health_checks IS 'Health check results for deployment verification';
COMMENT ON TABLE database_migrations IS 'Database migration tracking with safety features';
COMMENT ON TABLE migration_logs IS 'Detailed logs for migration progress tracking';
COMMENT ON TABLE schema_versions IS 'Schema version history for rollback capability';
COMMENT ON TABLE backups IS 'Backup records for disaster recovery';
COMMENT ON TABLE recovery_operations IS 'Recovery operation tracking';
COMMENT ON TABLE dr_drills IS 'Disaster recovery drill records for compliance';
COMMENT ON TABLE infrastructure_costs IS 'Cloud infrastructure cost tracking';
COMMENT ON TABLE cost_budgets IS 'Cost budgets for monitoring and alerting';
COMMENT ON TABLE cost_alerts IS 'Cost threshold breach alerts';
COMMENT ON TABLE resource_utilization IS 'Resource utilization metrics for optimization';
COMMENT ON TABLE cost_optimization_recommendations IS 'Cost optimization suggestions';
