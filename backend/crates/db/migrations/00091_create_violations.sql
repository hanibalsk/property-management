-- Epic 142: Violation Tracking & Enforcement
-- HOA/Condo rule violations, fines, and enforcement actions

-- =============================================================================
-- ENUM TYPES
-- =============================================================================

-- Violation category (type of rule violated)
CREATE TYPE violation_category AS ENUM (
    'noise',           -- Noise complaints
    'parking',         -- Parking violations
    'pet',             -- Pet-related violations
    'maintenance',     -- Property maintenance issues
    'architectural',   -- Unauthorized modifications
    'common_area',     -- Common area misuse
    'lease',           -- Lease term violations
    'payment',         -- Payment-related violations
    'safety',          -- Safety violations
    'other'            -- Other violations
);

-- Violation severity level
CREATE TYPE violation_severity AS ENUM (
    'minor',      -- Warning only, no fine
    'moderate',   -- Small fine, first offense
    'major',      -- Larger fine, repeated offense
    'critical'    -- Severe fine, immediate action required
);

-- Violation status
CREATE TYPE violation_status AS ENUM (
    'reported',     -- Initially reported
    'under_review', -- Being investigated
    'confirmed',    -- Violation confirmed
    'disputed',     -- Resident disputing
    'resolved',     -- Issue resolved
    'dismissed',    -- Not a valid violation
    'escalated'     -- Escalated to legal/board
);

-- Enforcement action type
CREATE TYPE enforcement_action_type AS ENUM (
    'warning',           -- Written warning
    'first_fine',        -- First fine assessment
    'increased_fine',    -- Increased/repeated fine
    'privilege_suspension', -- Amenity access suspended
    'legal_action',      -- Legal proceedings initiated
    'lien',              -- Lien placed on property
    'other'              -- Other enforcement
);

-- Enforcement action status
CREATE TYPE enforcement_status AS ENUM (
    'pending',      -- Action pending
    'sent',         -- Notice sent
    'acknowledged', -- Resident acknowledged
    'paid',         -- Fine paid
    'appealed',     -- Action appealed
    'completed',    -- Action completed
    'cancelled'     -- Action cancelled
);

-- Appeal status
CREATE TYPE appeal_status AS ENUM (
    'submitted',    -- Appeal submitted
    'under_review', -- Being reviewed
    'hearing_scheduled', -- Hearing date set
    'approved',     -- Appeal approved
    'denied',       -- Appeal denied
    'withdrawn'     -- Appeal withdrawn
);

-- =============================================================================
-- TABLES
-- =============================================================================

-- Rule definitions (CC&Rs, house rules)
CREATE TABLE community_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE, -- NULL = org-wide rule

    -- Rule details
    rule_code VARCHAR(50) NOT NULL,           -- e.g., "NOISE-001"
    title VARCHAR(200) NOT NULL,
    description TEXT,
    category violation_category NOT NULL,

    -- Fine structure
    first_offense_fine DECIMAL(10, 2) DEFAULT 0,
    second_offense_fine DECIMAL(10, 2) DEFAULT 0,
    third_offense_fine DECIMAL(10, 2) DEFAULT 0,
    max_fine DECIMAL(10, 2),
    fine_escalation_days INTEGER DEFAULT 30,  -- Days between offenses for escalation

    -- Rule metadata
    effective_date DATE NOT NULL DEFAULT CURRENT_DATE,
    expiry_date DATE,
    is_active BOOLEAN DEFAULT true,
    requires_board_approval BOOLEAN DEFAULT false,

    -- Document reference
    source_document_id UUID REFERENCES documents(id),
    section_reference VARCHAR(100),

    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID REFERENCES users(id),

    UNIQUE(organization_id, rule_code)
);

-- Violation reports
CREATE TABLE violations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,
    unit_id UUID REFERENCES units(id) ON DELETE SET NULL,

    -- Violation details
    violation_number VARCHAR(50) NOT NULL,     -- e.g., "VIO-2026-0001"
    rule_id UUID REFERENCES community_rules(id) ON DELETE SET NULL,
    category violation_category NOT NULL,
    severity violation_severity NOT NULL DEFAULT 'minor',
    status violation_status NOT NULL DEFAULT 'reported',

    -- Description
    title VARCHAR(200) NOT NULL,
    description TEXT NOT NULL,
    location VARCHAR(200),                     -- Where violation occurred

    -- Involved parties
    violator_id UUID REFERENCES users(id),     -- If known resident
    violator_name VARCHAR(200),                -- For non-residents or unknown
    violator_unit VARCHAR(50),                 -- Unit number if known
    reporter_id UUID REFERENCES users(id),     -- Who reported
    assigned_to UUID REFERENCES users(id),     -- Staff assigned to handle

    -- Timing
    occurred_at TIMESTAMPTZ NOT NULL,
    reported_at TIMESTAMPTZ DEFAULT NOW(),
    reviewed_at TIMESTAMPTZ,
    resolved_at TIMESTAMPTZ,

    -- Evidence
    evidence_description TEXT,
    witness_count INTEGER DEFAULT 0,

    -- Resolution
    resolution_notes TEXT,
    resolution_type VARCHAR(50),               -- 'corrected', 'fined', 'dismissed', etc.

    -- Offense tracking
    offense_number INTEGER DEFAULT 1,          -- 1st, 2nd, 3rd offense
    previous_violation_id UUID REFERENCES violations(id),

    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(organization_id, violation_number)
);

-- Violation evidence (photos, documents)
CREATE TABLE violation_evidence (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    violation_id UUID NOT NULL REFERENCES violations(id) ON DELETE CASCADE,

    -- File details
    file_name VARCHAR(255) NOT NULL,
    file_type VARCHAR(100) NOT NULL,
    file_size INTEGER,
    storage_path VARCHAR(500),

    -- Metadata
    description TEXT,
    captured_at TIMESTAMPTZ,
    uploaded_by UUID REFERENCES users(id),

    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Enforcement actions (warnings, fines)
CREATE TABLE enforcement_actions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    violation_id UUID NOT NULL REFERENCES violations(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Action details
    action_type enforcement_action_type NOT NULL,
    status enforcement_status NOT NULL DEFAULT 'pending',

    -- Fine details (if applicable)
    fine_amount DECIMAL(10, 2),
    due_date DATE,
    paid_amount DECIMAL(10, 2) DEFAULT 0,
    paid_at TIMESTAMPTZ,

    -- Notice details
    notice_sent_at TIMESTAMPTZ,
    notice_method VARCHAR(50),                 -- 'email', 'mail', 'hand_delivered'
    notice_document_id UUID REFERENCES documents(id),

    -- Description
    description TEXT,
    notes TEXT,

    -- Privilege suspension details
    suspended_privileges TEXT[],               -- List of suspended amenities
    suspension_start DATE,
    suspension_end DATE,

    -- Issued by
    issued_by UUID REFERENCES users(id),
    approved_by UUID REFERENCES users(id),

    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Violation appeals
CREATE TABLE violation_appeals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    violation_id UUID NOT NULL REFERENCES violations(id) ON DELETE CASCADE,
    enforcement_action_id UUID REFERENCES enforcement_actions(id) ON DELETE SET NULL,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Appeal details
    appeal_number VARCHAR(50) NOT NULL,        -- e.g., "APP-2026-0001"
    status appeal_status NOT NULL DEFAULT 'submitted',

    -- Content
    reason TEXT NOT NULL,                      -- Why appealing
    requested_outcome TEXT,                    -- What they want
    supporting_evidence TEXT,

    -- Appellant
    appellant_id UUID NOT NULL REFERENCES users(id),

    -- Hearing details
    hearing_date TIMESTAMPTZ,
    hearing_location VARCHAR(200),
    hearing_notes TEXT,

    -- Decision
    decision TEXT,
    decision_date TIMESTAMPTZ,
    decided_by UUID REFERENCES users(id),
    fine_adjustment DECIMAL(10, 2),            -- Positive = reduction, negative = increase

    -- Audit
    submitted_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(organization_id, appeal_number)
);

-- Violation comments/activity log
CREATE TABLE violation_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    violation_id UUID NOT NULL REFERENCES violations(id) ON DELETE CASCADE,

    -- Comment details
    comment_type VARCHAR(50) NOT NULL,         -- 'note', 'status_change', 'communication', 'internal'
    content TEXT NOT NULL,
    is_internal BOOLEAN DEFAULT false,         -- Only visible to staff

    -- Author
    author_id UUID REFERENCES users(id),

    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Violation notifications sent
CREATE TABLE violation_notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    violation_id UUID NOT NULL REFERENCES violations(id) ON DELETE CASCADE,

    -- Notification details
    notification_type VARCHAR(50) NOT NULL,    -- 'initial_notice', 'reminder', 'escalation', 'resolution'
    recipient_id UUID REFERENCES users(id),
    recipient_email VARCHAR(255),

    -- Content
    subject VARCHAR(200),
    body TEXT,

    -- Delivery
    sent_at TIMESTAMPTZ DEFAULT NOW(),
    delivered_at TIMESTAMPTZ,
    read_at TIMESTAMPTZ,
    delivery_status VARCHAR(50) DEFAULT 'sent'
);

-- Fine payment records
CREATE TABLE fine_payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    enforcement_action_id UUID NOT NULL REFERENCES enforcement_actions(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Payment details
    amount DECIMAL(10, 2) NOT NULL,
    payment_method VARCHAR(50),                -- 'credit_card', 'bank_transfer', 'check', 'cash'
    transaction_reference VARCHAR(100),

    -- Payer
    payer_id UUID REFERENCES users(id),
    payer_name VARCHAR(200),

    -- Status
    status VARCHAR(50) DEFAULT 'completed',    -- 'pending', 'completed', 'failed', 'refunded'
    processed_at TIMESTAMPTZ DEFAULT NOW(),

    -- Notes
    notes TEXT,

    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW(),
    recorded_by UUID REFERENCES users(id)
);

-- Violation statistics (materialized for reporting)
CREATE TABLE violation_statistics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,

    -- Period
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    period_type VARCHAR(20) NOT NULL,          -- 'daily', 'weekly', 'monthly', 'yearly'

    -- Counts
    total_violations INTEGER DEFAULT 0,
    new_violations INTEGER DEFAULT 0,
    resolved_violations INTEGER DEFAULT 0,
    dismissed_violations INTEGER DEFAULT 0,

    -- By category
    noise_violations INTEGER DEFAULT 0,
    parking_violations INTEGER DEFAULT 0,
    pet_violations INTEGER DEFAULT 0,
    maintenance_violations INTEGER DEFAULT 0,
    architectural_violations INTEGER DEFAULT 0,
    other_violations INTEGER DEFAULT 0,

    -- Fines
    total_fines_assessed DECIMAL(12, 2) DEFAULT 0,
    total_fines_collected DECIMAL(12, 2) DEFAULT 0,
    total_fines_waived DECIMAL(12, 2) DEFAULT 0,

    -- Appeals
    total_appeals INTEGER DEFAULT 0,
    appeals_approved INTEGER DEFAULT 0,
    appeals_denied INTEGER DEFAULT 0,

    -- Average resolution time (days)
    avg_resolution_time DECIMAL(5, 1),

    -- Audit
    calculated_at TIMESTAMPTZ DEFAULT NOW()
);

-- =============================================================================
-- INDEXES
-- =============================================================================

-- Community rules
CREATE INDEX idx_community_rules_org ON community_rules(organization_id);
CREATE INDEX idx_community_rules_building ON community_rules(building_id);
CREATE INDEX idx_community_rules_category ON community_rules(category);
CREATE INDEX idx_community_rules_active ON community_rules(organization_id, is_active) WHERE is_active = true;

-- Violations
CREATE INDEX idx_violations_org ON violations(organization_id);
CREATE INDEX idx_violations_building ON violations(building_id);
CREATE INDEX idx_violations_unit ON violations(unit_id);
CREATE INDEX idx_violations_status ON violations(organization_id, status);
CREATE INDEX idx_violations_category ON violations(organization_id, category);
CREATE INDEX idx_violations_violator ON violations(violator_id);
CREATE INDEX idx_violations_reporter ON violations(reporter_id);
CREATE INDEX idx_violations_assigned ON violations(assigned_to);
CREATE INDEX idx_violations_occurred ON violations(occurred_at DESC);
CREATE INDEX idx_violations_open ON violations(organization_id, status)
    WHERE status NOT IN ('resolved', 'dismissed');

-- Evidence
CREATE INDEX idx_violation_evidence_violation ON violation_evidence(violation_id);

-- Enforcement actions
CREATE INDEX idx_enforcement_actions_violation ON enforcement_actions(violation_id);
CREATE INDEX idx_enforcement_actions_org ON enforcement_actions(organization_id);
CREATE INDEX idx_enforcement_actions_status ON enforcement_actions(organization_id, status);
CREATE INDEX idx_enforcement_actions_due ON enforcement_actions(due_date) WHERE status = 'sent';

-- Appeals
CREATE INDEX idx_violation_appeals_violation ON violation_appeals(violation_id);
CREATE INDEX idx_violation_appeals_org ON violation_appeals(organization_id);
CREATE INDEX idx_violation_appeals_status ON violation_appeals(organization_id, status);
CREATE INDEX idx_violation_appeals_appellant ON violation_appeals(appellant_id);

-- Comments
CREATE INDEX idx_violation_comments_violation ON violation_comments(violation_id);

-- Notifications
CREATE INDEX idx_violation_notifications_violation ON violation_notifications(violation_id);

-- Payments
CREATE INDEX idx_fine_payments_action ON fine_payments(enforcement_action_id);
CREATE INDEX idx_fine_payments_org ON fine_payments(organization_id);

-- Statistics
CREATE INDEX idx_violation_statistics_org ON violation_statistics(organization_id);
CREATE INDEX idx_violation_statistics_period ON violation_statistics(organization_id, period_type, period_start DESC);

-- =============================================================================
-- ROW LEVEL SECURITY
-- =============================================================================

ALTER TABLE community_rules ENABLE ROW LEVEL SECURITY;
ALTER TABLE violations ENABLE ROW LEVEL SECURITY;
ALTER TABLE violation_evidence ENABLE ROW LEVEL SECURITY;
ALTER TABLE enforcement_actions ENABLE ROW LEVEL SECURITY;
ALTER TABLE violation_appeals ENABLE ROW LEVEL SECURITY;
ALTER TABLE violation_comments ENABLE ROW LEVEL SECURITY;
ALTER TABLE violation_notifications ENABLE ROW LEVEL SECURITY;
ALTER TABLE fine_payments ENABLE ROW LEVEL SECURITY;
ALTER TABLE violation_statistics ENABLE ROW LEVEL SECURITY;

-- Community rules policies
CREATE POLICY "community_rules_tenant_isolation" ON community_rules
    FOR ALL USING (organization_id = current_setting('app.current_tenant')::uuid);

-- Violations policies
CREATE POLICY "violations_tenant_isolation" ON violations
    FOR ALL USING (organization_id = current_setting('app.current_tenant')::uuid);

-- Evidence policies
CREATE POLICY "violation_evidence_tenant_isolation" ON violation_evidence
    FOR ALL USING (
        violation_id IN (
            SELECT id FROM violations
            WHERE organization_id = current_setting('app.current_tenant')::uuid
        )
    );

-- Enforcement actions policies
CREATE POLICY "enforcement_actions_tenant_isolation" ON enforcement_actions
    FOR ALL USING (organization_id = current_setting('app.current_tenant')::uuid);

-- Appeals policies
CREATE POLICY "violation_appeals_tenant_isolation" ON violation_appeals
    FOR ALL USING (organization_id = current_setting('app.current_tenant')::uuid);

-- Comments policies
CREATE POLICY "violation_comments_tenant_isolation" ON violation_comments
    FOR ALL USING (
        violation_id IN (
            SELECT id FROM violations
            WHERE organization_id = current_setting('app.current_tenant')::uuid
        )
    );

-- Notifications policies
CREATE POLICY "violation_notifications_tenant_isolation" ON violation_notifications
    FOR ALL USING (
        violation_id IN (
            SELECT id FROM violations
            WHERE organization_id = current_setting('app.current_tenant')::uuid
        )
    );

-- Payments policies
CREATE POLICY "fine_payments_tenant_isolation" ON fine_payments
    FOR ALL USING (organization_id = current_setting('app.current_tenant')::uuid);

-- Statistics policies
CREATE POLICY "violation_statistics_tenant_isolation" ON violation_statistics
    FOR ALL USING (organization_id = current_setting('app.current_tenant')::uuid);

-- =============================================================================
-- TRIGGERS
-- =============================================================================

-- Auto-update timestamps
CREATE TRIGGER update_community_rules_updated_at
    BEFORE UPDATE ON community_rules
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_violations_updated_at
    BEFORE UPDATE ON violations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_enforcement_actions_updated_at
    BEFORE UPDATE ON enforcement_actions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_violation_appeals_updated_at
    BEFORE UPDATE ON violation_appeals
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- =============================================================================
-- COMMENTS
-- =============================================================================

COMMENT ON TABLE community_rules IS 'CC&R and house rule definitions for violation tracking';
COMMENT ON TABLE violations IS 'Reported violations of community rules';
COMMENT ON TABLE violation_evidence IS 'Photos and documents supporting violation reports';
COMMENT ON TABLE enforcement_actions IS 'Warnings, fines, and other enforcement actions';
COMMENT ON TABLE violation_appeals IS 'Appeals submitted against violations or fines';
COMMENT ON TABLE violation_comments IS 'Activity log and comments on violations';
COMMENT ON TABLE violation_notifications IS 'Record of notifications sent regarding violations';
COMMENT ON TABLE fine_payments IS 'Payment records for violation fines';
COMMENT ON TABLE violation_statistics IS 'Pre-calculated statistics for reporting';
