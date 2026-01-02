-- Epic 77: Dispute Resolution
-- Creates tables for dispute filing, mediation, resolution, and enforcement.

-- =============================================================================
-- DISPUTES (Story 77.1)
-- =============================================================================

CREATE TABLE IF NOT EXISTS disputes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE SET NULL,
    unit_id UUID REFERENCES units(id) ON DELETE SET NULL,
    reference_number VARCHAR(50) NOT NULL UNIQUE,
    category VARCHAR(50) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    desired_resolution TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'filed',
    priority VARCHAR(20) NOT NULL DEFAULT 'medium',
    filed_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    assigned_to UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_disputes_organization ON disputes(organization_id);
CREATE INDEX IF NOT EXISTS idx_disputes_building ON disputes(building_id) WHERE building_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_disputes_status ON disputes(status);
CREATE INDEX IF NOT EXISTS idx_disputes_filed_by ON disputes(filed_by);
CREATE INDEX IF NOT EXISTS idx_disputes_created_at ON disputes(created_at DESC);

CREATE TABLE IF NOT EXISTS dispute_parties (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dispute_id UUID NOT NULL REFERENCES disputes(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    role VARCHAR(50) NOT NULL,
    notified_at TIMESTAMPTZ,
    responded_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(dispute_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_dispute_parties_dispute ON dispute_parties(dispute_id);
CREATE INDEX IF NOT EXISTS idx_dispute_parties_user ON dispute_parties(user_id);

CREATE TABLE IF NOT EXISTS dispute_evidence (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dispute_id UUID NOT NULL REFERENCES disputes(id) ON DELETE CASCADE,
    uploaded_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    filename VARCHAR(255) NOT NULL,
    original_filename VARCHAR(255) NOT NULL,
    content_type VARCHAR(100) NOT NULL,
    size_bytes BIGINT NOT NULL,
    storage_url TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_dispute_evidence_dispute ON dispute_evidence(dispute_id);

CREATE TABLE IF NOT EXISTS dispute_activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dispute_id UUID NOT NULL REFERENCES disputes(id) ON DELETE CASCADE,
    actor_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    activity_type VARCHAR(50) NOT NULL,
    description TEXT NOT NULL,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_dispute_activities_dispute ON dispute_activities(dispute_id);
CREATE INDEX IF NOT EXISTS idx_dispute_activities_created ON dispute_activities(created_at DESC);

-- =============================================================================
-- MEDIATION (Story 77.2)
-- =============================================================================

CREATE TABLE IF NOT EXISTS mediation_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dispute_id UUID NOT NULL REFERENCES disputes(id) ON DELETE CASCADE,
    mediator_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    session_type VARCHAR(50) NOT NULL,
    scheduled_at TIMESTAMPTZ NOT NULL,
    duration_minutes INTEGER,
    location TEXT,
    meeting_url TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'scheduled',
    notes TEXT,
    outcome TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_mediation_sessions_dispute ON mediation_sessions(dispute_id);
CREATE INDEX IF NOT EXISTS idx_mediation_sessions_mediator ON mediation_sessions(mediator_id);
CREATE INDEX IF NOT EXISTS idx_mediation_sessions_scheduled ON mediation_sessions(scheduled_at);

CREATE TABLE IF NOT EXISTS session_attendances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id UUID NOT NULL REFERENCES mediation_sessions(id) ON DELETE CASCADE,
    party_id UUID NOT NULL REFERENCES dispute_parties(id) ON DELETE CASCADE,
    confirmed BOOLEAN NOT NULL DEFAULT false,
    attended BOOLEAN,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(session_id, party_id)
);

CREATE INDEX IF NOT EXISTS idx_session_attendances_session ON session_attendances(session_id);

CREATE TABLE IF NOT EXISTS party_submissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dispute_id UUID NOT NULL REFERENCES disputes(id) ON DELETE CASCADE,
    party_id UUID NOT NULL REFERENCES dispute_parties(id) ON DELETE CASCADE,
    submission_type VARCHAR(50) NOT NULL,
    content TEXT NOT NULL,
    is_visible_to_all BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_party_submissions_dispute ON party_submissions(dispute_id);
CREATE INDEX IF NOT EXISTS idx_party_submissions_party ON party_submissions(party_id);

-- =============================================================================
-- RESOLUTION (Story 77.3)
-- =============================================================================

CREATE TABLE IF NOT EXISTS dispute_resolutions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dispute_id UUID NOT NULL REFERENCES disputes(id) ON DELETE CASCADE,
    proposed_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    resolution_text TEXT NOT NULL,
    terms JSONB NOT NULL DEFAULT '[]',
    status VARCHAR(50) NOT NULL DEFAULT 'proposed',
    proposed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    accepted_at TIMESTAMPTZ,
    implemented_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_dispute_resolutions_dispute ON dispute_resolutions(dispute_id);
CREATE INDEX IF NOT EXISTS idx_dispute_resolutions_status ON dispute_resolutions(status);

CREATE TABLE IF NOT EXISTS resolution_votes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    resolution_id UUID NOT NULL REFERENCES dispute_resolutions(id) ON DELETE CASCADE,
    party_id UUID NOT NULL REFERENCES dispute_parties(id) ON DELETE CASCADE,
    accepted BOOLEAN NOT NULL,
    comments TEXT,
    voted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(resolution_id, party_id)
);

CREATE INDEX IF NOT EXISTS idx_resolution_votes_resolution ON resolution_votes(resolution_id);

-- =============================================================================
-- ENFORCEMENT (Story 77.4)
-- =============================================================================

CREATE TABLE IF NOT EXISTS action_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dispute_id UUID NOT NULL REFERENCES disputes(id) ON DELETE CASCADE,
    resolution_id UUID REFERENCES dispute_resolutions(id) ON DELETE SET NULL,
    resolution_term_id VARCHAR(100),
    assigned_to UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    due_date TIMESTAMPTZ NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    completed_at TIMESTAMPTZ,
    completion_notes TEXT,
    reminder_sent_at TIMESTAMPTZ,
    escalated_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_action_items_dispute ON action_items(dispute_id);
CREATE INDEX IF NOT EXISTS idx_action_items_assigned ON action_items(assigned_to);
CREATE INDEX IF NOT EXISTS idx_action_items_due_date ON action_items(due_date);
CREATE INDEX IF NOT EXISTS idx_action_items_status ON action_items(status);

CREATE TABLE IF NOT EXISTS escalations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dispute_id UUID NOT NULL REFERENCES disputes(id) ON DELETE CASCADE,
    action_item_id UUID REFERENCES action_items(id) ON DELETE SET NULL,
    escalated_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    escalated_to UUID REFERENCES users(id) ON DELETE SET NULL,
    reason TEXT NOT NULL,
    severity VARCHAR(50) NOT NULL,
    resolved BOOLEAN NOT NULL DEFAULT false,
    resolved_at TIMESTAMPTZ,
    resolution_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_escalations_dispute ON escalations(dispute_id);
CREATE INDEX IF NOT EXISTS idx_escalations_resolved ON escalations(resolved);

-- =============================================================================
-- TRIGGERS
-- =============================================================================

CREATE OR REPLACE FUNCTION update_disputes_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_disputes_updated_at ON disputes;
CREATE TRIGGER trigger_disputes_updated_at
    BEFORE UPDATE ON disputes
    FOR EACH ROW
    EXECUTE FUNCTION update_disputes_updated_at();

DROP TRIGGER IF EXISTS trigger_mediation_sessions_updated_at ON mediation_sessions;
CREATE TRIGGER trigger_mediation_sessions_updated_at
    BEFORE UPDATE ON mediation_sessions
    FOR EACH ROW
    EXECUTE FUNCTION update_disputes_updated_at();

DROP TRIGGER IF EXISTS trigger_session_attendances_updated_at ON session_attendances;
CREATE TRIGGER trigger_session_attendances_updated_at
    BEFORE UPDATE ON session_attendances
    FOR EACH ROW
    EXECUTE FUNCTION update_disputes_updated_at();

DROP TRIGGER IF EXISTS trigger_dispute_resolutions_updated_at ON dispute_resolutions;
CREATE TRIGGER trigger_dispute_resolutions_updated_at
    BEFORE UPDATE ON dispute_resolutions
    FOR EACH ROW
    EXECUTE FUNCTION update_disputes_updated_at();

DROP TRIGGER IF EXISTS trigger_action_items_updated_at ON action_items;
CREATE TRIGGER trigger_action_items_updated_at
    BEFORE UPDATE ON action_items
    FOR EACH ROW
    EXECUTE FUNCTION update_disputes_updated_at();

DROP TRIGGER IF EXISTS trigger_escalations_updated_at ON escalations;
CREATE TRIGGER trigger_escalations_updated_at
    BEFORE UPDATE ON escalations
    FOR EACH ROW
    EXECUTE FUNCTION update_disputes_updated_at();

-- =============================================================================
-- COMMENTS
-- =============================================================================

COMMENT ON TABLE disputes IS 'Dispute cases filed between parties';
COMMENT ON TABLE dispute_parties IS 'Parties involved in disputes (complainant, respondent, witness, mediator)';
COMMENT ON TABLE dispute_evidence IS 'Evidence and attachments for disputes';
COMMENT ON TABLE dispute_activities IS 'Activity timeline for dispute tracking';
COMMENT ON TABLE mediation_sessions IS 'Scheduled mediation sessions for disputes';
COMMENT ON TABLE session_attendances IS 'Attendance tracking for mediation sessions';
COMMENT ON TABLE party_submissions IS 'Written submissions from parties during mediation';
COMMENT ON TABLE dispute_resolutions IS 'Proposed and accepted resolutions for disputes';
COMMENT ON TABLE resolution_votes IS 'Party votes on proposed resolutions';
COMMENT ON TABLE action_items IS 'Action items assigned as part of resolution enforcement';
COMMENT ON TABLE escalations IS 'Escalation records for non-compliance';
