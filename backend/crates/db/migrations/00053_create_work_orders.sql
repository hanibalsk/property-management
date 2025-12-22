-- Epic 20: Maintenance Scheduling & Work Orders
-- Extends Epic 13's equipment/maintenance infrastructure with work orders and scheduling

-- Work orders for maintenance tasks (Story 20.2)
CREATE TABLE IF NOT EXISTS work_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    equipment_id UUID REFERENCES equipment(id) ON DELETE SET NULL,
    fault_id UUID REFERENCES faults(id) ON DELETE SET NULL,

    -- Work order details
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    priority TEXT NOT NULL DEFAULT 'medium' CHECK (priority IN ('low', 'medium', 'high', 'urgent')),
    work_type TEXT NOT NULL DEFAULT 'corrective' CHECK (work_type IN ('preventive', 'corrective', 'emergency', 'inspection')),

    -- Assignment
    assigned_to UUID REFERENCES users(id) ON DELETE SET NULL,
    vendor_id UUID,  -- Will reference vendors table (Epic 21)

    -- Scheduling
    scheduled_date DATE,
    due_date DATE,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,

    -- Cost tracking
    estimated_cost DECIMAL(12, 2),
    actual_cost DECIMAL(12, 2),

    -- Status
    status TEXT NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'assigned', 'in_progress', 'on_hold', 'completed', 'cancelled')),
    resolution_notes TEXT,

    -- Source tracking (for auto-generated work orders)
    source TEXT DEFAULT 'manual' CHECK (source IN ('manual', 'fault', 'schedule', 'prediction')),
    schedule_id UUID,  -- Reference to maintenance_schedules

    -- Metadata
    attachments UUID[] DEFAULT '{}',  -- Document IDs
    tags TEXT[] DEFAULT '{}',
    metadata JSONB DEFAULT '{}',

    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Work order comments/updates (for activity timeline)
CREATE TABLE IF NOT EXISTS work_order_updates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    work_order_id UUID NOT NULL REFERENCES work_orders(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id),
    update_type TEXT NOT NULL DEFAULT 'comment' CHECK (update_type IN ('comment', 'status_change', 'assignment', 'cost_update')),
    content TEXT NOT NULL,
    old_value TEXT,
    new_value TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Maintenance schedules for preventive maintenance (Story 20.3)
CREATE TABLE IF NOT EXISTS maintenance_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,
    equipment_id UUID REFERENCES equipment(id) ON DELETE CASCADE,

    -- Schedule details
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    work_type TEXT NOT NULL DEFAULT 'preventive',

    -- Frequency pattern
    frequency TEXT NOT NULL CHECK (frequency IN ('daily', 'weekly', 'biweekly', 'monthly', 'quarterly', 'semiannual', 'annual')),
    day_of_week INTEGER CHECK (day_of_week IS NULL OR (day_of_week >= 0 AND day_of_week <= 6)),  -- 0 = Sunday
    day_of_month INTEGER CHECK (day_of_month IS NULL OR (day_of_month >= 1 AND day_of_month <= 28)),
    month_of_year INTEGER CHECK (month_of_year IS NULL OR (month_of_year >= 1 AND month_of_year <= 12)),

    -- Assignment
    default_assignee UUID REFERENCES users(id) ON DELETE SET NULL,
    default_vendor_id UUID,  -- Will reference vendors table (Epic 21)

    -- Scheduling
    start_date DATE NOT NULL,
    end_date DATE,
    next_due_date DATE NOT NULL,
    last_run_date DATE,

    -- Work order generation
    auto_create_work_order BOOLEAN DEFAULT TRUE,
    advance_days INTEGER DEFAULT 7,  -- Create work order X days before due
    estimated_duration_hours DECIMAL(4, 1),
    estimated_cost DECIMAL(12, 2),

    -- Status
    is_active BOOLEAN DEFAULT TRUE,

    -- Metadata
    checklist JSONB DEFAULT '[]',  -- Checklist items for the task
    metadata JSONB DEFAULT '{}',

    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT equipment_or_building CHECK (equipment_id IS NOT NULL OR building_id IS NOT NULL)
);

-- Schedule execution history
CREATE TABLE IF NOT EXISTS schedule_executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    schedule_id UUID NOT NULL REFERENCES maintenance_schedules(id) ON DELETE CASCADE,
    work_order_id UUID REFERENCES work_orders(id) ON DELETE SET NULL,
    due_date DATE NOT NULL,
    executed_at TIMESTAMPTZ,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'created', 'skipped', 'completed')),
    skipped_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_work_orders_org ON work_orders(organization_id);
CREATE INDEX idx_work_orders_building ON work_orders(building_id);
CREATE INDEX idx_work_orders_equipment ON work_orders(equipment_id);
CREATE INDEX idx_work_orders_fault ON work_orders(fault_id);
CREATE INDEX idx_work_orders_assigned_to ON work_orders(assigned_to);
CREATE INDEX idx_work_orders_vendor ON work_orders(vendor_id);
CREATE INDEX idx_work_orders_status ON work_orders(status);
CREATE INDEX idx_work_orders_priority ON work_orders(priority);
CREATE INDEX idx_work_orders_due_date ON work_orders(due_date);
CREATE INDEX idx_work_orders_schedule ON work_orders(schedule_id);

CREATE INDEX idx_work_order_updates_work_order ON work_order_updates(work_order_id);
CREATE INDEX idx_work_order_updates_created_at ON work_order_updates(created_at DESC);

CREATE INDEX idx_maintenance_schedules_org ON maintenance_schedules(organization_id);
CREATE INDEX idx_maintenance_schedules_building ON maintenance_schedules(building_id);
CREATE INDEX idx_maintenance_schedules_equipment ON maintenance_schedules(equipment_id);
CREATE INDEX idx_maintenance_schedules_next_due ON maintenance_schedules(next_due_date) WHERE is_active = TRUE;
CREATE INDEX idx_maintenance_schedules_active ON maintenance_schedules(is_active) WHERE is_active = TRUE;

CREATE INDEX idx_schedule_executions_schedule ON schedule_executions(schedule_id);
CREATE INDEX idx_schedule_executions_due_date ON schedule_executions(due_date);
CREATE INDEX idx_schedule_executions_status ON schedule_executions(status);

-- RLS policies
ALTER TABLE work_orders ENABLE ROW LEVEL SECURITY;
ALTER TABLE work_order_updates ENABLE ROW LEVEL SECURITY;
ALTER TABLE maintenance_schedules ENABLE ROW LEVEL SECURITY;
ALTER TABLE schedule_executions ENABLE ROW LEVEL SECURITY;

CREATE POLICY work_orders_tenant_isolation ON work_orders
    FOR ALL
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY work_order_updates_tenant_isolation ON work_order_updates
    FOR ALL
    USING (work_order_id IN (
        SELECT id FROM work_orders
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

CREATE POLICY maintenance_schedules_tenant_isolation ON maintenance_schedules
    FOR ALL
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY schedule_executions_tenant_isolation ON schedule_executions
    FOR ALL
    USING (schedule_id IN (
        SELECT id FROM maintenance_schedules
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

-- Triggers
CREATE TRIGGER update_work_orders_updated_at
    BEFORE UPDATE ON work_orders
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_maintenance_schedules_updated_at
    BEFORE UPDATE ON maintenance_schedules
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Function to calculate next due date based on frequency
CREATE OR REPLACE FUNCTION calculate_next_schedule_date(
    p_frequency TEXT,
    p_current_date DATE,
    p_day_of_week INTEGER,
    p_day_of_month INTEGER,
    p_month_of_year INTEGER
) RETURNS DATE AS $$
DECLARE
    v_next_date DATE;
BEGIN
    CASE p_frequency
        WHEN 'daily' THEN
            v_next_date := p_current_date + INTERVAL '1 day';
        WHEN 'weekly' THEN
            v_next_date := p_current_date + INTERVAL '1 week';
            IF p_day_of_week IS NOT NULL THEN
                v_next_date := v_next_date + (p_day_of_week - EXTRACT(DOW FROM v_next_date)::INTEGER) * INTERVAL '1 day';
                IF v_next_date <= p_current_date THEN
                    v_next_date := v_next_date + INTERVAL '1 week';
                END IF;
            END IF;
        WHEN 'biweekly' THEN
            v_next_date := p_current_date + INTERVAL '2 weeks';
        WHEN 'monthly' THEN
            v_next_date := p_current_date + INTERVAL '1 month';
            IF p_day_of_month IS NOT NULL THEN
                v_next_date := DATE_TRUNC('month', v_next_date) + (p_day_of_month - 1) * INTERVAL '1 day';
                IF v_next_date <= p_current_date THEN
                    v_next_date := v_next_date + INTERVAL '1 month';
                END IF;
            END IF;
        WHEN 'quarterly' THEN
            v_next_date := p_current_date + INTERVAL '3 months';
        WHEN 'semiannual' THEN
            v_next_date := p_current_date + INTERVAL '6 months';
        WHEN 'annual' THEN
            v_next_date := p_current_date + INTERVAL '1 year';
            IF p_month_of_year IS NOT NULL AND p_day_of_month IS NOT NULL THEN
                v_next_date := DATE_TRUNC('year', v_next_date) +
                               (p_month_of_year - 1) * INTERVAL '1 month' +
                               (p_day_of_month - 1) * INTERVAL '1 day';
                IF v_next_date <= p_current_date THEN
                    v_next_date := v_next_date + INTERVAL '1 year';
                END IF;
            END IF;
        ELSE
            v_next_date := p_current_date + INTERVAL '1 month';
    END CASE;

    RETURN v_next_date;
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- Comments
COMMENT ON TABLE work_orders IS 'Epic 20: Work orders for maintenance tasks';
COMMENT ON TABLE work_order_updates IS 'Epic 20: Activity log for work orders';
COMMENT ON TABLE maintenance_schedules IS 'Epic 20: Recurring preventive maintenance schedules';
COMMENT ON TABLE schedule_executions IS 'Epic 20: History of scheduled maintenance runs';
COMMENT ON FUNCTION calculate_next_schedule_date IS 'Calculates next due date based on maintenance frequency';
