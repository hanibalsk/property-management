-- Epic 143: Board Meeting Management
-- Migration: 00092_create_board_meetings.sql
-- Creates tables for HOA/Condo board meeting management including scheduling,
-- agendas, minutes, motions, voting, and attendance tracking.

-- ============================================================================
-- ENUM TYPES
-- ============================================================================

-- Meeting types
CREATE TYPE meeting_type AS ENUM (
    'regular',           -- Regularly scheduled board meeting
    'special',           -- Special session for urgent matters
    'annual',            -- Annual general meeting (AGM)
    'emergency',         -- Emergency meeting
    'committee',         -- Committee meeting
    'executive_session'  -- Closed session for sensitive matters
);

-- Meeting status
CREATE TYPE meeting_status AS ENUM (
    'draft',             -- Meeting being planned
    'scheduled',         -- Meeting confirmed and announced
    'in_progress',       -- Meeting currently happening
    'completed',         -- Meeting finished
    'cancelled',         -- Meeting cancelled
    'postponed'          -- Meeting postponed to new date
);

-- Agenda item status
CREATE TYPE agenda_item_status AS ENUM (
    'pending',           -- Not yet discussed
    'in_discussion',     -- Currently being discussed
    'tabled',            -- Deferred to future meeting
    'completed',         -- Discussion completed
    'withdrawn'          -- Item withdrawn from agenda
);

-- Motion status
CREATE TYPE motion_status AS ENUM (
    'proposed',          -- Motion introduced
    'seconded',          -- Motion has a second
    'discussion',        -- Under discussion
    'voting',            -- Currently voting
    'passed',            -- Motion passed
    'failed',            -- Motion failed
    'tabled',            -- Motion tabled
    'withdrawn',         -- Motion withdrawn
    'amended'            -- Motion amended and resubmitted
);

-- Board member role
CREATE TYPE board_role AS ENUM (
    'president',
    'vice_president',
    'secretary',
    'treasurer',
    'director',
    'committee_chair',
    'member_at_large'
);

-- Attendance status
CREATE TYPE attendance_status AS ENUM (
    'present',
    'absent',
    'excused',
    'late',
    'remote'
);

-- Vote choice for motions
CREATE TYPE vote_choice AS ENUM (
    'in_favor',
    'opposed',
    'abstain',
    'recused'
);

-- ============================================================================
-- BOARD MEMBERS TABLE
-- ============================================================================

CREATE TABLE board_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Board position
    role board_role NOT NULL,
    title VARCHAR(100),

    -- Term information
    term_start DATE NOT NULL,
    term_end DATE,
    is_active BOOLEAN DEFAULT true,

    -- Contact preferences
    email_notifications BOOLEAN DEFAULT true,
    sms_notifications BOOLEAN DEFAULT false,

    -- Metadata
    appointed_by UUID REFERENCES users(id),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(organization_id, user_id, role, term_start)
);

-- ============================================================================
-- BOARD MEETINGS TABLE
-- ============================================================================

CREATE TABLE board_meetings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE SET NULL,

    -- Meeting identification
    meeting_number VARCHAR(50),
    title VARCHAR(200) NOT NULL,
    meeting_type meeting_type NOT NULL DEFAULT 'regular',
    status meeting_status NOT NULL DEFAULT 'draft',

    -- Schedule
    scheduled_start TIMESTAMPTZ NOT NULL,
    scheduled_end TIMESTAMPTZ,
    actual_start TIMESTAMPTZ,
    actual_end TIMESTAMPTZ,
    timezone VARCHAR(50) DEFAULT 'UTC',

    -- Location
    location_type VARCHAR(50) DEFAULT 'in_person', -- in_person, virtual, hybrid
    physical_location TEXT,
    virtual_meeting_url TEXT,
    virtual_meeting_id VARCHAR(100),
    dial_in_number VARCHAR(50),

    -- Quorum
    quorum_required INTEGER DEFAULT 3,
    quorum_present INTEGER,
    quorum_met BOOLEAN DEFAULT false,

    -- Recording
    is_recorded BOOLEAN DEFAULT false,
    recording_url TEXT,

    -- Documents
    agenda_document_id UUID,
    minutes_document_id UUID,

    -- Notifications
    notice_sent_at TIMESTAMPTZ,
    reminder_sent_at TIMESTAMPTZ,

    -- Created by
    created_by UUID NOT NULL REFERENCES users(id),
    secretary_id UUID REFERENCES users(id),

    -- Metadata
    description TEXT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- MEETING AGENDA ITEMS TABLE
-- ============================================================================

CREATE TABLE meeting_agenda_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    meeting_id UUID NOT NULL REFERENCES board_meetings(id) ON DELETE CASCADE,

    -- Item details
    item_number VARCHAR(20) NOT NULL,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    item_type VARCHAR(50) DEFAULT 'discussion', -- discussion, action, information, consent

    -- Order and timing
    display_order INTEGER NOT NULL DEFAULT 0,
    estimated_duration_minutes INTEGER,
    actual_duration_minutes INTEGER,

    -- Status
    status agenda_item_status NOT NULL DEFAULT 'pending',

    -- Presenter/owner
    presenter_id UUID REFERENCES users(id),
    presenter_name VARCHAR(200),

    -- Supporting documents
    document_ids UUID[],

    -- Discussion notes
    discussion_notes TEXT,
    outcome TEXT,

    -- Follow-up
    follow_up_required BOOLEAN DEFAULT false,
    follow_up_assignee UUID REFERENCES users(id),
    follow_up_due_date DATE,

    -- Metadata
    added_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- MEETING MOTIONS TABLE
-- ============================================================================

CREATE TABLE meeting_motions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    meeting_id UUID NOT NULL REFERENCES board_meetings(id) ON DELETE CASCADE,
    agenda_item_id UUID REFERENCES meeting_agenda_items(id) ON DELETE SET NULL,

    -- Motion identification
    motion_number VARCHAR(50),
    title VARCHAR(200) NOT NULL,
    motion_text TEXT NOT NULL,

    -- Status and process
    status motion_status NOT NULL DEFAULT 'proposed',

    -- Participants
    proposed_by UUID NOT NULL REFERENCES users(id),
    seconded_by UUID REFERENCES users(id),

    -- Voting results
    votes_in_favor INTEGER DEFAULT 0,
    votes_opposed INTEGER DEFAULT 0,
    votes_abstain INTEGER DEFAULT 0,
    votes_recused INTEGER DEFAULT 0,
    voting_started_at TIMESTAMPTZ,
    voting_ended_at TIMESTAMPTZ,

    -- Amendments
    original_motion_id UUID REFERENCES meeting_motions(id),
    amendment_text TEXT,

    -- Resolution
    resolution_text TEXT,
    effective_date DATE,

    -- Metadata
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- MOTION VOTES TABLE
-- ============================================================================

CREATE TABLE motion_votes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    motion_id UUID NOT NULL REFERENCES meeting_motions(id) ON DELETE CASCADE,
    board_member_id UUID NOT NULL REFERENCES board_members(id) ON DELETE CASCADE,

    -- Vote
    vote vote_choice NOT NULL,
    recusal_reason TEXT,

    -- Metadata
    voted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(motion_id, board_member_id)
);

-- ============================================================================
-- MEETING ATTENDANCE TABLE
-- ============================================================================

CREATE TABLE meeting_attendance (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    meeting_id UUID NOT NULL REFERENCES board_meetings(id) ON DELETE CASCADE,
    board_member_id UUID REFERENCES board_members(id) ON DELETE SET NULL,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Attendance
    status attendance_status NOT NULL DEFAULT 'present',
    attendance_type VARCHAR(50) DEFAULT 'board_member', -- board_member, guest, staff, resident

    -- Timing
    arrived_at TIMESTAMPTZ,
    departed_at TIMESTAMPTZ,

    -- Guest information (if not a board member)
    guest_name VARCHAR(200),
    guest_affiliation VARCHAR(200),

    -- Notes
    notes TEXT,

    -- Metadata
    marked_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(meeting_id, user_id)
);

-- ============================================================================
-- MEETING MINUTES TABLE
-- ============================================================================

CREATE TABLE meeting_minutes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    meeting_id UUID NOT NULL REFERENCES board_meetings(id) ON DELETE CASCADE,

    -- Minutes content
    version INTEGER NOT NULL DEFAULT 1,
    status VARCHAR(50) DEFAULT 'draft', -- draft, pending_approval, approved, published

    -- Content sections
    call_to_order TEXT,
    roll_call TEXT,
    approval_of_minutes TEXT,
    reports TEXT,
    old_business TEXT,
    new_business TEXT,
    announcements TEXT,
    adjournment TEXT,

    -- Full content (alternative to sections)
    full_content TEXT,

    -- Approval
    approved_at TIMESTAMPTZ,
    approved_by UUID REFERENCES users(id),
    approval_motion_id UUID REFERENCES meeting_motions(id),

    -- Document
    document_id UUID,

    -- Metadata
    prepared_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- ACTION ITEMS TABLE
-- ============================================================================

CREATE TABLE meeting_action_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    meeting_id UUID NOT NULL REFERENCES board_meetings(id) ON DELETE CASCADE,
    agenda_item_id UUID REFERENCES meeting_agenda_items(id) ON DELETE SET NULL,
    motion_id UUID REFERENCES meeting_motions(id) ON DELETE SET NULL,

    -- Action item details
    title VARCHAR(200) NOT NULL,
    description TEXT,

    -- Assignment
    assigned_to UUID REFERENCES users(id),
    assigned_to_name VARCHAR(200),

    -- Timing
    due_date DATE,
    completed_at TIMESTAMPTZ,

    -- Status
    status VARCHAR(50) DEFAULT 'pending', -- pending, in_progress, completed, cancelled, overdue
    priority VARCHAR(20) DEFAULT 'medium', -- low, medium, high, urgent

    -- Follow-up
    notes TEXT,
    completion_notes TEXT,

    -- Metadata
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- MEETING DOCUMENTS TABLE
-- ============================================================================

CREATE TABLE meeting_documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    meeting_id UUID NOT NULL REFERENCES board_meetings(id) ON DELETE CASCADE,

    -- Document info
    document_type VARCHAR(50) NOT NULL, -- agenda, minutes, report, attachment, presentation
    title VARCHAR(200) NOT NULL,
    description TEXT,

    -- File reference
    file_id UUID,
    file_url TEXT,
    file_name VARCHAR(255),
    file_size INTEGER,
    mime_type VARCHAR(100),

    -- Visibility
    is_public BOOLEAN DEFAULT false,
    board_only BOOLEAN DEFAULT true,

    -- Metadata
    uploaded_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- MEETING STATISTICS TABLE (Materialized/cached data)
-- ============================================================================

CREATE TABLE meeting_statistics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Time period
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,

    -- Meeting counts
    total_meetings INTEGER DEFAULT 0,
    regular_meetings INTEGER DEFAULT 0,
    special_meetings INTEGER DEFAULT 0,
    emergency_meetings INTEGER DEFAULT 0,
    cancelled_meetings INTEGER DEFAULT 0,

    -- Attendance metrics
    avg_attendance_rate NUMERIC(5,2),
    avg_quorum_margin NUMERIC(5,2),

    -- Motion metrics
    total_motions INTEGER DEFAULT 0,
    motions_passed INTEGER DEFAULT 0,
    motions_failed INTEGER DEFAULT 0,
    motions_tabled INTEGER DEFAULT 0,

    -- Duration metrics
    avg_meeting_duration_minutes INTEGER,
    total_meeting_hours NUMERIC(10,2),

    -- Action items
    total_action_items INTEGER DEFAULT 0,
    completed_action_items INTEGER DEFAULT 0,
    overdue_action_items INTEGER DEFAULT 0,

    -- Calculated at
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(organization_id, period_start, period_end)
);

-- ============================================================================
-- INDEXES
-- ============================================================================

-- Board members
CREATE INDEX IF NOT EXISTS idx_board_members_org ON board_members(organization_id);
CREATE INDEX IF NOT EXISTS idx_board_members_user ON board_members(user_id);
CREATE INDEX IF NOT EXISTS idx_board_members_active ON board_members(organization_id, is_active);
CREATE INDEX IF NOT EXISTS idx_board_members_role ON board_members(organization_id, role);

-- Board meetings
CREATE INDEX IF NOT EXISTS idx_board_meetings_org ON board_meetings(organization_id);
CREATE INDEX IF NOT EXISTS idx_board_meetings_building ON board_meetings(building_id);
CREATE INDEX IF NOT EXISTS idx_board_meetings_status ON board_meetings(organization_id, status);
CREATE INDEX IF NOT EXISTS idx_board_meetings_scheduled ON board_meetings(scheduled_start);
CREATE INDEX IF NOT EXISTS idx_board_meetings_type ON board_meetings(organization_id, meeting_type);

-- Agenda items
CREATE INDEX IF NOT EXISTS idx_agenda_items_meeting ON meeting_agenda_items(meeting_id);
CREATE INDEX IF NOT EXISTS idx_agenda_items_order ON meeting_agenda_items(meeting_id, display_order);
CREATE INDEX IF NOT EXISTS idx_agenda_items_status ON meeting_agenda_items(status);

-- Motions
CREATE INDEX IF NOT EXISTS idx_motions_meeting ON meeting_motions(meeting_id);
CREATE INDEX IF NOT EXISTS idx_motions_agenda ON meeting_motions(agenda_item_id);
CREATE INDEX IF NOT EXISTS idx_motions_status ON meeting_motions(status);
CREATE INDEX IF NOT EXISTS idx_motions_proposer ON meeting_motions(proposed_by);

-- Motion votes
CREATE INDEX IF NOT EXISTS idx_motion_votes_motion ON motion_votes(motion_id);
CREATE INDEX IF NOT EXISTS idx_motion_votes_member ON motion_votes(board_member_id);

-- Attendance
CREATE INDEX IF NOT EXISTS idx_attendance_meeting ON meeting_attendance(meeting_id);
CREATE INDEX IF NOT EXISTS idx_attendance_user ON meeting_attendance(user_id);
CREATE INDEX IF NOT EXISTS idx_attendance_member ON meeting_attendance(board_member_id);

-- Minutes
CREATE INDEX IF NOT EXISTS idx_minutes_meeting ON meeting_minutes(meeting_id);
CREATE INDEX IF NOT EXISTS idx_minutes_status ON meeting_minutes(status);

-- Action items (renamed to avoid conflict with disputes migration index)
CREATE INDEX IF NOT EXISTS idx_meeting_action_items_meeting ON meeting_action_items(meeting_id);
CREATE INDEX IF NOT EXISTS idx_meeting_action_items_assigned ON meeting_action_items(assigned_to);
CREATE INDEX IF NOT EXISTS idx_meeting_action_items_due ON meeting_action_items(due_date);
CREATE INDEX IF NOT EXISTS idx_meeting_action_items_status ON meeting_action_items(status);

-- Documents
CREATE INDEX IF NOT EXISTS idx_meeting_docs_meeting ON meeting_documents(meeting_id);
CREATE INDEX IF NOT EXISTS idx_meeting_docs_type ON meeting_documents(document_type);

-- Statistics
CREATE INDEX IF NOT EXISTS idx_meeting_stats_org ON meeting_statistics(organization_id);
CREATE INDEX IF NOT EXISTS idx_meeting_stats_period ON meeting_statistics(period_start, period_end);

-- ============================================================================
-- ROW LEVEL SECURITY
-- ============================================================================

ALTER TABLE board_members ENABLE ROW LEVEL SECURITY;
ALTER TABLE board_meetings ENABLE ROW LEVEL SECURITY;
ALTER TABLE meeting_agenda_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE meeting_motions ENABLE ROW LEVEL SECURITY;
ALTER TABLE motion_votes ENABLE ROW LEVEL SECURITY;
ALTER TABLE meeting_attendance ENABLE ROW LEVEL SECURITY;
ALTER TABLE meeting_minutes ENABLE ROW LEVEL SECURITY;
ALTER TABLE meeting_action_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE meeting_documents ENABLE ROW LEVEL SECURITY;
ALTER TABLE meeting_statistics ENABLE ROW LEVEL SECURITY;

-- RLS Policies for board_members
CREATE POLICY board_members_tenant_isolation ON board_members
    FOR ALL
    USING (organization_id = current_setting('app.current_organization_id', true)::UUID);

-- RLS Policies for board_meetings
CREATE POLICY board_meetings_tenant_isolation ON board_meetings
    FOR ALL
    USING (organization_id = current_setting('app.current_organization_id', true)::UUID);

-- RLS Policies for meeting_agenda_items (through meeting)
CREATE POLICY agenda_items_tenant_isolation ON meeting_agenda_items
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM board_meetings bm
            WHERE bm.id = meeting_agenda_items.meeting_id
            AND bm.organization_id = current_setting('app.current_organization_id', true)::UUID
        )
    );

-- RLS Policies for meeting_motions (through meeting)
CREATE POLICY motions_tenant_isolation ON meeting_motions
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM board_meetings bm
            WHERE bm.id = meeting_motions.meeting_id
            AND bm.organization_id = current_setting('app.current_organization_id', true)::UUID
        )
    );

-- RLS Policies for motion_votes (through motion -> meeting)
CREATE POLICY motion_votes_tenant_isolation ON motion_votes
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM meeting_motions mm
            JOIN board_meetings bm ON bm.id = mm.meeting_id
            WHERE mm.id = motion_votes.motion_id
            AND bm.organization_id = current_setting('app.current_organization_id', true)::UUID
        )
    );

-- RLS Policies for meeting_attendance (through meeting)
CREATE POLICY attendance_tenant_isolation ON meeting_attendance
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM board_meetings bm
            WHERE bm.id = meeting_attendance.meeting_id
            AND bm.organization_id = current_setting('app.current_organization_id', true)::UUID
        )
    );

-- RLS Policies for meeting_minutes (through meeting)
CREATE POLICY minutes_tenant_isolation ON meeting_minutes
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM board_meetings bm
            WHERE bm.id = meeting_minutes.meeting_id
            AND bm.organization_id = current_setting('app.current_organization_id', true)::UUID
        )
    );

-- RLS Policies for meeting_action_items (through meeting)
CREATE POLICY action_items_tenant_isolation ON meeting_action_items
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM board_meetings bm
            WHERE bm.id = meeting_action_items.meeting_id
            AND bm.organization_id = current_setting('app.current_organization_id', true)::UUID
        )
    );

-- RLS Policies for meeting_documents (through meeting)
CREATE POLICY meeting_docs_tenant_isolation ON meeting_documents
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM board_meetings bm
            WHERE bm.id = meeting_documents.meeting_id
            AND bm.organization_id = current_setting('app.current_organization_id', true)::UUID
        )
    );

-- RLS Policies for meeting_statistics
CREATE POLICY meeting_stats_tenant_isolation ON meeting_statistics
    FOR ALL
    USING (organization_id = current_setting('app.current_organization_id', true)::UUID);

-- ============================================================================
-- TRIGGERS
-- ============================================================================

-- Update timestamps
CREATE TRIGGER update_board_members_timestamp
    BEFORE UPDATE ON board_members
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_board_meetings_timestamp
    BEFORE UPDATE ON board_meetings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_agenda_items_timestamp
    BEFORE UPDATE ON meeting_agenda_items
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_motions_timestamp
    BEFORE UPDATE ON meeting_motions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_attendance_timestamp
    BEFORE UPDATE ON meeting_attendance
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_minutes_timestamp
    BEFORE UPDATE ON meeting_minutes
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_action_items_timestamp
    BEFORE UPDATE ON meeting_action_items
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- HELPER FUNCTIONS
-- ============================================================================

-- Function to calculate motion results
CREATE OR REPLACE FUNCTION calculate_motion_result(motion_uuid UUID)
RETURNS motion_status AS $$
DECLARE
    in_favor INTEGER;
    opposed INTEGER;
    abstain INTEGER;
    total_voting INTEGER;
BEGIN
    SELECT
        COALESCE(SUM(CASE WHEN vote = 'in_favor' THEN 1 ELSE 0 END), 0),
        COALESCE(SUM(CASE WHEN vote = 'opposed' THEN 1 ELSE 0 END), 0),
        COALESCE(SUM(CASE WHEN vote = 'abstain' THEN 1 ELSE 0 END), 0)
    INTO in_favor, opposed, abstain
    FROM motion_votes
    WHERE motion_id = motion_uuid;

    total_voting := in_favor + opposed;

    IF total_voting = 0 THEN
        RETURN 'proposed'::motion_status;
    ELSIF in_favor > opposed THEN
        RETURN 'passed'::motion_status;
    ELSE
        RETURN 'failed'::motion_status;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Function to check quorum
CREATE OR REPLACE FUNCTION check_meeting_quorum(meeting_uuid UUID)
RETURNS BOOLEAN AS $$
DECLARE
    required INTEGER;
    present INTEGER;
BEGIN
    SELECT quorum_required INTO required
    FROM board_meetings
    WHERE id = meeting_uuid;

    SELECT COUNT(*) INTO present
    FROM meeting_attendance
    WHERE meeting_id = meeting_uuid
    AND status IN ('present', 'remote', 'late');

    RETURN present >= COALESCE(required, 1);
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- COMMENTS
-- ============================================================================

COMMENT ON TABLE board_members IS 'Board member records for HOA/Condo associations';
COMMENT ON TABLE board_meetings IS 'Board meetings with scheduling and configuration';
COMMENT ON TABLE meeting_agenda_items IS 'Agenda items for board meetings';
COMMENT ON TABLE meeting_motions IS 'Formal motions raised during meetings';
COMMENT ON TABLE motion_votes IS 'Individual votes on motions by board members';
COMMENT ON TABLE meeting_attendance IS 'Attendance records for meetings';
COMMENT ON TABLE meeting_minutes IS 'Meeting minutes with approval workflow';
COMMENT ON TABLE meeting_action_items IS 'Action items arising from meetings';
COMMENT ON TABLE meeting_documents IS 'Documents attached to meetings';
COMMENT ON TABLE meeting_statistics IS 'Aggregated meeting statistics by period';
