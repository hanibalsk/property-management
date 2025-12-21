-- Epic 4: Fault Reporting & Resolution
-- Creates faults, fault_attachments, and fault_timeline tables with RLS

-- Fault categories (for AI suggestions)
CREATE TYPE fault_category AS ENUM (
    'plumbing',
    'electrical',
    'heating',
    'structural',
    'exterior',
    'elevator',
    'common_area',
    'security',
    'cleaning',
    'other'
);

-- Fault status workflow
CREATE TYPE fault_status AS ENUM (
    'new',           -- Just reported
    'triaged',       -- Manager reviewed and prioritized
    'in_progress',   -- Being worked on
    'waiting_parts', -- Waiting for parts/materials
    'scheduled',     -- Work scheduled for specific date
    'resolved',      -- Work completed, awaiting confirmation
    'closed',        -- Confirmed by reporter or auto-closed
    'reopened'       -- Reporter reopened after resolution
);

-- Fault priority
CREATE TYPE fault_priority AS ENUM (
    'low',
    'medium',
    'high',
    'urgent'
);

-- Main faults table
CREATE TABLE IF NOT EXISTS faults (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Context
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    unit_id UUID REFERENCES units(id) ON DELETE SET NULL, -- Optional, fault can be for common area

    -- Reporter
    reporter_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Fault details
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    location_description VARCHAR(500), -- e.g., "Kitchen sink", "Hallway 3rd floor"

    -- Classification
    category fault_category NOT NULL DEFAULT 'other',
    priority fault_priority NOT NULL DEFAULT 'medium',
    status fault_status NOT NULL DEFAULT 'new',

    -- AI metadata (for Phase 3 training)
    ai_category fault_category,
    ai_priority fault_priority,
    ai_confidence DECIMAL(5, 4), -- 0.0000 to 1.0000
    ai_processed_at TIMESTAMPTZ,

    -- Assignment
    assigned_to UUID REFERENCES users(id) ON DELETE SET NULL,
    assigned_at TIMESTAMPTZ,

    -- Triage
    triaged_by UUID REFERENCES users(id) ON DELETE SET NULL,
    triaged_at TIMESTAMPTZ,

    -- Resolution
    resolved_at TIMESTAMPTZ,
    resolved_by UUID REFERENCES users(id) ON DELETE SET NULL,
    resolution_notes TEXT,

    -- Confirmation
    confirmed_at TIMESTAMPTZ,
    confirmed_by UUID REFERENCES users(id) ON DELETE SET NULL,

    -- Feedback
    rating INTEGER CHECK (rating >= 1 AND rating <= 5),
    feedback TEXT,

    -- Scheduling
    scheduled_date DATE,
    estimated_completion DATE,

    -- Offline sync support
    idempotency_key VARCHAR(64) UNIQUE,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for faults
CREATE INDEX IF NOT EXISTS idx_faults_organization_id ON faults(organization_id);
CREATE INDEX IF NOT EXISTS idx_faults_building_id ON faults(building_id);
CREATE INDEX IF NOT EXISTS idx_faults_unit_id ON faults(unit_id);
CREATE INDEX IF NOT EXISTS idx_faults_reporter_id ON faults(reporter_id);
CREATE INDEX IF NOT EXISTS idx_faults_assigned_to ON faults(assigned_to);
CREATE INDEX IF NOT EXISTS idx_faults_status ON faults(status);
CREATE INDEX IF NOT EXISTS idx_faults_priority ON faults(priority);
CREATE INDEX IF NOT EXISTS idx_faults_category ON faults(category);
CREATE INDEX IF NOT EXISTS idx_faults_created_at ON faults(created_at DESC);

-- Composite indexes for common queries
CREATE INDEX IF NOT EXISTS idx_faults_building_status ON faults(building_id, status);
CREATE INDEX IF NOT EXISTS idx_faults_org_status_priority ON faults(organization_id, status, priority);

-- Full-text search
CREATE INDEX IF NOT EXISTS idx_faults_search ON faults
    USING GIN (to_tsvector('simple', title || ' ' || description));

-- Trigger for updated_at
CREATE TRIGGER update_faults_updated_at
    BEFORE UPDATE ON faults
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Enable Row Level Security
ALTER TABLE faults ENABLE ROW LEVEL SECURITY;

-- RLS Policy: Faults visible to organization members
CREATE POLICY faults_tenant_isolation ON faults
    FOR ALL
    USING (
        is_super_admin()
        OR organization_id = get_current_org_id()
    )
    WITH CHECK (
        is_super_admin()
        OR organization_id = get_current_org_id()
    );

---

-- Fault attachments (photos, documents)
CREATE TABLE IF NOT EXISTS fault_attachments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fault_id UUID NOT NULL REFERENCES faults(id) ON DELETE CASCADE,

    -- File info
    filename VARCHAR(255) NOT NULL,
    original_filename VARCHAR(255) NOT NULL,
    content_type VARCHAR(100) NOT NULL,
    size_bytes INTEGER NOT NULL CHECK (size_bytes > 0),

    -- Storage
    storage_url TEXT NOT NULL,
    thumbnail_url TEXT, -- For images

    -- Metadata
    uploaded_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    description VARCHAR(500),

    -- Image metadata (for photos)
    width INTEGER,
    height INTEGER,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for attachments
CREATE INDEX IF NOT EXISTS idx_fault_attachments_fault_id ON fault_attachments(fault_id);
CREATE INDEX IF NOT EXISTS idx_fault_attachments_uploaded_by ON fault_attachments(uploaded_by);

-- Enable Row Level Security
ALTER TABLE fault_attachments ENABLE ROW LEVEL SECURITY;

-- RLS Policy: Attachments inherit fault visibility
CREATE POLICY fault_attachments_tenant_isolation ON fault_attachments
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM faults f
            WHERE f.id = fault_attachments.fault_id
            AND f.organization_id = get_current_org_id()
        )
    )
    WITH CHECK (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM faults f
            WHERE f.id = fault_attachments.fault_id
            AND f.organization_id = get_current_org_id()
        )
    );

---

-- Fault timeline (status history, notes, activities)
CREATE TYPE timeline_action AS ENUM (
    'created',
    'triaged',
    'assigned',
    'status_changed',
    'priority_changed',
    'work_note',
    'comment',
    'attachment_added',
    'scheduled',
    'resolved',
    'confirmed',
    'reopened',
    'rated'
);

CREATE TABLE IF NOT EXISTS fault_timeline (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fault_id UUID NOT NULL REFERENCES faults(id) ON DELETE CASCADE,

    -- Actor
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Action
    action timeline_action NOT NULL,

    -- Details (varies by action)
    note TEXT,
    old_value VARCHAR(50), -- For status/priority changes
    new_value VARCHAR(50),
    metadata JSONB DEFAULT '{}', -- Extensible data

    -- Visibility
    is_internal BOOLEAN NOT NULL DEFAULT FALSE, -- Internal notes not visible to reporter

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for timeline
CREATE INDEX IF NOT EXISTS idx_fault_timeline_fault_id ON fault_timeline(fault_id);
CREATE INDEX IF NOT EXISTS idx_fault_timeline_user_id ON fault_timeline(user_id);
CREATE INDEX IF NOT EXISTS idx_fault_timeline_action ON fault_timeline(action);
CREATE INDEX IF NOT EXISTS idx_fault_timeline_created_at ON fault_timeline(created_at);

-- Composite index for chronological timeline view
CREATE INDEX IF NOT EXISTS idx_fault_timeline_fault_created ON fault_timeline(fault_id, created_at);

-- Enable Row Level Security
ALTER TABLE fault_timeline ENABLE ROW LEVEL SECURITY;

-- RLS Policy: Timeline inherits fault visibility, with internal note restriction
CREATE POLICY fault_timeline_tenant_isolation ON fault_timeline
    FOR ALL
    USING (
        is_super_admin()
        OR (
            EXISTS (
                SELECT 1 FROM faults f
                WHERE f.id = fault_timeline.fault_id
                AND f.organization_id = get_current_org_id()
            )
            -- Internal notes only visible to managers (check via metadata in application layer)
        )
    )
    WITH CHECK (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM faults f
            WHERE f.id = fault_timeline.fault_id
            AND f.organization_id = get_current_org_id()
        )
    );

---

-- Comments on tables
COMMENT ON TABLE faults IS 'Fault reports with AI metadata (Epic 4, UC-03)';
COMMENT ON TABLE fault_attachments IS 'Photos and documents attached to faults';
COMMENT ON TABLE fault_timeline IS 'Chronological history of fault activities';

COMMENT ON COLUMN faults.ai_category IS 'AI-suggested category (for training data)';
COMMENT ON COLUMN faults.ai_confidence IS 'Confidence score 0.0-1.0 from AI classification';
COMMENT ON COLUMN faults.idempotency_key IS 'Client-generated key for offline sync deduplication';
COMMENT ON COLUMN fault_timeline.is_internal IS 'Internal notes hidden from fault reporter';
