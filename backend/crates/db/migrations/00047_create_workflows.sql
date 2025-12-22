-- Epic 13: AI Assistant & Automation
-- Story 13.6 & 13.7: Workflow Automation

-- Workflow definitions
CREATE TABLE IF NOT EXISTS workflows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    trigger_type TEXT NOT NULL CHECK (trigger_type IN (
        'fault_created', 'fault_status_changed', 'fault_resolved',
        'payment_due', 'payment_overdue', 'payment_received',
        'document_uploaded', 'document_signed',
        'vote_created', 'vote_ended',
        'announcement_created',
        'meter_reading_due', 'meter_reading_anomaly',
        'schedule', 'manual'
    )),
    trigger_config JSONB DEFAULT '{}',
    conditions JSONB DEFAULT '[]',
    enabled BOOLEAN DEFAULT TRUE,
    last_triggered_at TIMESTAMPTZ,
    trigger_count INTEGER DEFAULT 0,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Workflow actions (ordered sequence)
CREATE TABLE IF NOT EXISTS workflow_actions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_id UUID NOT NULL REFERENCES workflows(id) ON DELETE CASCADE,
    action_order INTEGER NOT NULL,
    action_type TEXT NOT NULL CHECK (action_type IN (
        'send_notification', 'send_email', 'send_sms',
        'create_task', 'update_status', 'assign_to_user',
        'create_announcement', 'create_fault',
        'call_webhook', 'delay',
        'condition_branch'
    )),
    action_config JSONB NOT NULL DEFAULT '{}',
    on_failure TEXT NOT NULL DEFAULT 'stop' CHECK (on_failure IN ('stop', 'continue', 'retry')),
    retry_count INTEGER DEFAULT 3,
    retry_delay_seconds INTEGER DEFAULT 60,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (workflow_id, action_order)
);

-- Workflow execution history
CREATE TABLE IF NOT EXISTS workflow_executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_id UUID NOT NULL REFERENCES workflows(id) ON DELETE CASCADE,
    trigger_event JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'running', 'completed', 'failed', 'cancelled')),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    error_message TEXT,
    context JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Workflow execution steps (detailed trace)
CREATE TABLE IF NOT EXISTS workflow_execution_steps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    execution_id UUID NOT NULL REFERENCES workflow_executions(id) ON DELETE CASCADE,
    action_id UUID NOT NULL REFERENCES workflow_actions(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'running', 'completed', 'failed', 'skipped')),
    input JSONB DEFAULT '{}',
    output JSONB DEFAULT '{}',
    error_message TEXT,
    retry_attempt INTEGER DEFAULT 0,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    duration_ms INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Scheduled workflow runs
CREATE TABLE IF NOT EXISTS workflow_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_id UUID NOT NULL REFERENCES workflows(id) ON DELETE CASCADE,
    cron_expression TEXT NOT NULL,
    timezone TEXT NOT NULL DEFAULT 'UTC',
    next_run_at TIMESTAMPTZ NOT NULL,
    last_run_at TIMESTAMPTZ,
    enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_workflows_org ON workflows(organization_id);
CREATE INDEX idx_workflows_trigger ON workflows(trigger_type);
CREATE INDEX idx_workflows_enabled ON workflows(enabled);
CREATE INDEX idx_workflow_actions_workflow ON workflow_actions(workflow_id);
CREATE INDEX idx_workflow_executions_workflow ON workflow_executions(workflow_id);
CREATE INDEX idx_workflow_executions_status ON workflow_executions(status);
CREATE INDEX idx_workflow_executions_created ON workflow_executions(created_at DESC);
CREATE INDEX idx_workflow_execution_steps_execution ON workflow_execution_steps(execution_id);
CREATE INDEX idx_workflow_schedules_next ON workflow_schedules(next_run_at) WHERE enabled = TRUE;

-- RLS policies
ALTER TABLE workflows ENABLE ROW LEVEL SECURITY;
ALTER TABLE workflow_actions ENABLE ROW LEVEL SECURITY;
ALTER TABLE workflow_executions ENABLE ROW LEVEL SECURITY;
ALTER TABLE workflow_execution_steps ENABLE ROW LEVEL SECURITY;
ALTER TABLE workflow_schedules ENABLE ROW LEVEL SECURITY;

CREATE POLICY workflows_tenant_isolation ON workflows
    FOR ALL
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY workflow_actions_tenant_isolation ON workflow_actions
    FOR ALL
    USING (workflow_id IN (
        SELECT id FROM workflows
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

CREATE POLICY workflow_executions_tenant_isolation ON workflow_executions
    FOR ALL
    USING (workflow_id IN (
        SELECT id FROM workflows
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

CREATE POLICY workflow_execution_steps_tenant_isolation ON workflow_execution_steps
    FOR ALL
    USING (execution_id IN (
        SELECT e.id FROM workflow_executions e
        JOIN workflows w ON w.id = e.workflow_id
        WHERE w.organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

CREATE POLICY workflow_schedules_tenant_isolation ON workflow_schedules
    FOR ALL
    USING (workflow_id IN (
        SELECT id FROM workflows
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

-- Triggers
CREATE TRIGGER update_workflows_updated_at
    BEFORE UPDATE ON workflows
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_workflow_schedules_updated_at
    BEFORE UPDATE ON workflow_schedules
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
