-- UC-12: Utility Outages
-- Creates outages and outage_notifications tables with RLS

-- Outage status
CREATE TYPE outage_status AS ENUM (
    'planned',      -- Scheduled maintenance
    'ongoing',      -- Currently happening
    'resolved',     -- Outage ended
    'cancelled'     -- Planned outage was cancelled
);

-- Commodity type (which utility is affected)
CREATE TYPE outage_commodity AS ENUM (
    'water',
    'electricity',
    'gas',
    'heating',
    'internet',
    'other'
);

-- Outage severity
CREATE TYPE outage_severity AS ENUM (
    'low',          -- Minor inconvenience
    'medium',       -- Noticeable impact
    'high',         -- Significant disruption
    'critical'      -- Emergency situation
);

-- ============================================================================
-- Outages table
-- ============================================================================

CREATE TABLE IF NOT EXISTS outages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Context
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Author (who reported/created)
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Basic info
    title VARCHAR(255) NOT NULL,
    description TEXT,  -- Detailed description or cause

    -- Classification
    commodity outage_commodity NOT NULL,
    severity outage_severity NOT NULL DEFAULT 'medium',
    status outage_status NOT NULL DEFAULT 'planned',

    -- Scope: Which buildings are affected
    building_ids JSONB NOT NULL DEFAULT '[]',
    -- Format: ["building_uuid_1", "building_uuid_2"]
    -- Empty array means all buildings in the organization

    -- Timing
    scheduled_start TIMESTAMPTZ NOT NULL,  -- When outage is expected/started
    scheduled_end TIMESTAMPTZ,             -- When outage is expected to end
    actual_start TIMESTAMPTZ,              -- When outage actually started
    actual_end TIMESTAMPTZ,                -- When outage actually ended

    -- External reference (from supplier)
    external_reference VARCHAR(100),
    supplier_name VARCHAR(255),

    -- Resolution
    resolution_notes TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for outages
CREATE INDEX IF NOT EXISTS idx_outages_organization_id ON outages(organization_id);
CREATE INDEX IF NOT EXISTS idx_outages_created_by ON outages(created_by);
CREATE INDEX IF NOT EXISTS idx_outages_status ON outages(status);
CREATE INDEX IF NOT EXISTS idx_outages_commodity ON outages(commodity);
CREATE INDEX IF NOT EXISTS idx_outages_severity ON outages(severity);
CREATE INDEX IF NOT EXISTS idx_outages_scheduled_start ON outages(scheduled_start);
CREATE INDEX IF NOT EXISTS idx_outages_scheduled_end ON outages(scheduled_end);
CREATE INDEX IF NOT EXISTS idx_outages_active ON outages(organization_id, status) WHERE status IN ('planned', 'ongoing');

-- Trigger for updated_at
CREATE TRIGGER update_outages_updated_at
    BEFORE UPDATE ON outages
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Enable Row Level Security
ALTER TABLE outages ENABLE ROW LEVEL SECURITY;

-- RLS Policy
CREATE POLICY outages_tenant_isolation ON outages
    FOR ALL
    USING (
        is_super_admin()
        OR organization_id = get_current_org_id()
    )
    WITH CHECK (
        is_super_admin()
        OR organization_id = get_current_org_id()
    );

-- ============================================================================
-- Outage Notifications table (tracks who was notified)
-- ============================================================================

CREATE TABLE IF NOT EXISTS outage_notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    outage_id UUID NOT NULL REFERENCES outages(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Notification status
    notified_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    read_at TIMESTAMPTZ,
    acknowledged_at TIMESTAMPTZ,

    -- Notification method
    notification_method VARCHAR(50) NOT NULL DEFAULT 'push',  -- push, email, sms

    -- One notification record per user per outage per method
    CONSTRAINT unique_outage_notification UNIQUE (outage_id, user_id, notification_method)
);

-- Indexes for outage_notifications
CREATE INDEX IF NOT EXISTS idx_outage_notifications_outage_id ON outage_notifications(outage_id);
CREATE INDEX IF NOT EXISTS idx_outage_notifications_user_id ON outage_notifications(user_id);
CREATE INDEX IF NOT EXISTS idx_outage_notifications_read ON outage_notifications(user_id) WHERE read_at IS NULL;

-- Enable Row Level Security
ALTER TABLE outage_notifications ENABLE ROW LEVEL SECURITY;

-- RLS Policy: Inherit from outages
CREATE POLICY outage_notifications_tenant_isolation ON outage_notifications
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM outages o
            WHERE o.id = outage_notifications.outage_id
            AND o.organization_id = get_current_org_id()
        )
    )
    WITH CHECK (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM outages o
            WHERE o.id = outage_notifications.outage_id
            AND o.organization_id = get_current_org_id()
        )
    );

-- ============================================================================
-- Comments
-- ============================================================================

COMMENT ON TABLE outages IS 'Utility outages and service interruptions (UC-12)';
COMMENT ON TABLE outage_notifications IS 'Tracks notification delivery for outages';

COMMENT ON COLUMN outages.building_ids IS 'JSON array of affected building UUIDs, empty means all';
COMMENT ON COLUMN outages.scheduled_start IS 'When the outage is planned/expected to start';
COMMENT ON COLUMN outages.actual_start IS 'When the outage actually started (for unplanned outages)';
COMMENT ON COLUMN outages.external_reference IS 'Reference number from utility supplier';
