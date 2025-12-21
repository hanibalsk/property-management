-- Epic 5: Building Voting & Decisions
-- Creates votes, vote_questions, vote_responses, vote_comments, vote_audit_log tables with RLS

-- Vote status
CREATE TYPE vote_status AS ENUM (
    'draft',
    'scheduled',
    'active',
    'closed',
    'cancelled'
);

-- Question types
CREATE TYPE vote_question_type AS ENUM (
    'yes_no',
    'single_choice',
    'multiple_choice',
    'ranked'
);

-- Quorum types
CREATE TYPE quorum_type AS ENUM (
    'simple_majority',
    'two_thirds',
    'weighted'
);

-- Audit action types
CREATE TYPE vote_audit_action AS ENUM (
    'vote_created',
    'vote_published',
    'vote_cancelled',
    'question_added',
    'question_removed',
    'ballot_cast',
    'ballot_updated',
    'comment_added',
    'comment_hidden',
    'vote_closed',
    'results_calculated'
);

-- ============================================================================
-- Votes table
-- ============================================================================

CREATE TABLE IF NOT EXISTS votes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Context
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,

    -- Vote details
    title VARCHAR(255) NOT NULL,
    description TEXT,

    -- Timing
    start_at TIMESTAMPTZ,
    end_at TIMESTAMPTZ NOT NULL,

    -- Configuration
    status vote_status NOT NULL DEFAULT 'draft',
    quorum_type quorum_type NOT NULL DEFAULT 'simple_majority',
    quorum_percentage INTEGER DEFAULT 50 CHECK (quorum_percentage >= 0 AND quorum_percentage <= 100),
    allow_delegation BOOLEAN NOT NULL DEFAULT TRUE,
    anonymous_voting BOOLEAN NOT NULL DEFAULT FALSE,

    -- Results (calculated after voting ends)
    participation_count INTEGER DEFAULT 0,
    eligible_count INTEGER DEFAULT 0,
    quorum_met BOOLEAN,
    results JSONB DEFAULT '{}',
    results_calculated_at TIMESTAMPTZ,

    -- Creator
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    published_by UUID REFERENCES users(id) ON DELETE SET NULL,
    published_at TIMESTAMPTZ,
    cancelled_by UUID REFERENCES users(id) ON DELETE SET NULL,
    cancelled_at TIMESTAMPTZ,
    cancellation_reason TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for votes
CREATE INDEX IF NOT EXISTS idx_votes_organization_id ON votes(organization_id);
CREATE INDEX IF NOT EXISTS idx_votes_building_id ON votes(building_id);
CREATE INDEX IF NOT EXISTS idx_votes_status ON votes(status);
CREATE INDEX IF NOT EXISTS idx_votes_end_at ON votes(end_at);
CREATE INDEX IF NOT EXISTS idx_votes_created_by ON votes(created_by);

-- Trigger for updated_at
CREATE TRIGGER update_votes_updated_at
    BEFORE UPDATE ON votes
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Enable Row Level Security
ALTER TABLE votes ENABLE ROW LEVEL SECURITY;

-- RLS Policy
CREATE POLICY votes_tenant_isolation ON votes
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
-- Vote Questions table
-- ============================================================================

CREATE TABLE IF NOT EXISTS vote_questions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vote_id UUID NOT NULL REFERENCES votes(id) ON DELETE CASCADE,

    -- Question details
    question_text TEXT NOT NULL,
    description TEXT,
    question_type vote_question_type NOT NULL DEFAULT 'yes_no',

    -- Options (for choice-based questions)
    options JSONB NOT NULL DEFAULT '[]',
    -- Format: [{"id": "uuid", "text": "Option A", "order": 1}, ...]

    -- Display order
    display_order INTEGER NOT NULL DEFAULT 0,

    -- Required
    is_required BOOLEAN NOT NULL DEFAULT TRUE,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for vote_questions
CREATE INDEX IF NOT EXISTS idx_vote_questions_vote_id ON vote_questions(vote_id);
CREATE INDEX IF NOT EXISTS idx_vote_questions_display_order ON vote_questions(vote_id, display_order);

-- Trigger for updated_at
CREATE TRIGGER update_vote_questions_updated_at
    BEFORE UPDATE ON vote_questions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Enable Row Level Security
ALTER TABLE vote_questions ENABLE ROW LEVEL SECURITY;

-- RLS Policy: Inherit from votes
CREATE POLICY vote_questions_tenant_isolation ON vote_questions
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM votes v
            WHERE v.id = vote_questions.vote_id
            AND v.organization_id = get_current_org_id()
        )
    )
    WITH CHECK (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM votes v
            WHERE v.id = vote_questions.vote_id
            AND v.organization_id = get_current_org_id()
        )
    );

-- ============================================================================
-- Vote Responses (Ballots) table
-- ============================================================================

CREATE TABLE IF NOT EXISTS vote_responses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vote_id UUID NOT NULL REFERENCES votes(id) ON DELETE CASCADE,

    -- Who voted
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,

    -- Delegation (if voting on behalf of owner)
    delegation_id UUID REFERENCES delegations(id) ON DELETE SET NULL,
    is_delegated BOOLEAN NOT NULL DEFAULT FALSE,

    -- Answers
    answers JSONB NOT NULL DEFAULT '{}',
    -- Format: {"question_id": "answer_value", ...}
    -- For yes_no: true/false
    -- For single_choice: "option_id"
    -- For multiple_choice: ["option_id", ...]
    -- For ranked: ["option_id_1st", "option_id_2nd", ...]

    -- Weight (for weighted voting)
    vote_weight DECIMAL(10, 4) NOT NULL DEFAULT 1.0,

    -- Integrity
    response_hash VARCHAR(64) NOT NULL, -- SHA-256 of answers + user_id + unit_id + timestamp
    submitted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- One vote per unit per vote
    CONSTRAINT unique_unit_vote UNIQUE (vote_id, unit_id)
);

-- Indexes for vote_responses
CREATE INDEX IF NOT EXISTS idx_vote_responses_vote_id ON vote_responses(vote_id);
CREATE INDEX IF NOT EXISTS idx_vote_responses_user_id ON vote_responses(user_id);
CREATE INDEX IF NOT EXISTS idx_vote_responses_unit_id ON vote_responses(unit_id);
CREATE INDEX IF NOT EXISTS idx_vote_responses_submitted_at ON vote_responses(submitted_at);

-- Enable Row Level Security
ALTER TABLE vote_responses ENABLE ROW LEVEL SECURITY;

-- RLS Policy: Users can see their own responses, managers can see all for their org
CREATE POLICY vote_responses_tenant_isolation ON vote_responses
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM votes v
            WHERE v.id = vote_responses.vote_id
            AND v.organization_id = get_current_org_id()
        )
    )
    WITH CHECK (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM votes v
            WHERE v.id = vote_responses.vote_id
            AND v.organization_id = get_current_org_id()
        )
    );

-- ============================================================================
-- Vote Comments (Discussion) table
-- ============================================================================

CREATE TABLE IF NOT EXISTS vote_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vote_id UUID NOT NULL REFERENCES votes(id) ON DELETE CASCADE,

    -- Author
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Threading
    parent_id UUID REFERENCES vote_comments(id) ON DELETE CASCADE,

    -- Content
    content TEXT NOT NULL,

    -- Moderation
    hidden BOOLEAN NOT NULL DEFAULT FALSE,
    hidden_by UUID REFERENCES users(id) ON DELETE SET NULL,
    hidden_at TIMESTAMPTZ,
    hidden_reason VARCHAR(255),

    -- AI consent for future sentiment analysis
    ai_consent BOOLEAN NOT NULL DEFAULT FALSE,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for vote_comments
CREATE INDEX IF NOT EXISTS idx_vote_comments_vote_id ON vote_comments(vote_id);
CREATE INDEX IF NOT EXISTS idx_vote_comments_user_id ON vote_comments(user_id);
CREATE INDEX IF NOT EXISTS idx_vote_comments_parent_id ON vote_comments(parent_id);
CREATE INDEX IF NOT EXISTS idx_vote_comments_created_at ON vote_comments(created_at);

-- Trigger for updated_at
CREATE TRIGGER update_vote_comments_updated_at
    BEFORE UPDATE ON vote_comments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Enable Row Level Security
ALTER TABLE vote_comments ENABLE ROW LEVEL SECURITY;

-- RLS Policy
CREATE POLICY vote_comments_tenant_isolation ON vote_comments
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM votes v
            WHERE v.id = vote_comments.vote_id
            AND v.organization_id = get_current_org_id()
        )
    )
    WITH CHECK (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM votes v
            WHERE v.id = vote_comments.vote_id
            AND v.organization_id = get_current_org_id()
        )
    );

-- ============================================================================
-- Vote Audit Log (Immutable) table
-- ============================================================================

CREATE TABLE IF NOT EXISTS vote_audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vote_id UUID NOT NULL REFERENCES votes(id) ON DELETE CASCADE,

    -- Actor
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,

    -- Action
    action vote_audit_action NOT NULL,

    -- Data for verification
    data_hash VARCHAR(64) NOT NULL, -- SHA-256 of action data
    data_snapshot JSONB, -- Copy of relevant data at time of action

    -- Metadata
    ip_address INET,
    user_agent TEXT,

    -- Timestamp (immutable)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for audit log
CREATE INDEX IF NOT EXISTS idx_vote_audit_log_vote_id ON vote_audit_log(vote_id);
CREATE INDEX IF NOT EXISTS idx_vote_audit_log_user_id ON vote_audit_log(user_id);
CREATE INDEX IF NOT EXISTS idx_vote_audit_log_action ON vote_audit_log(action);
CREATE INDEX IF NOT EXISTS idx_vote_audit_log_created_at ON vote_audit_log(created_at);

-- Prevent updates and deletes on audit log
CREATE OR REPLACE FUNCTION prevent_audit_log_modification()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'Audit log entries cannot be modified or deleted';
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER vote_audit_log_immutable_update
    BEFORE UPDATE ON vote_audit_log
    FOR EACH ROW
    EXECUTE FUNCTION prevent_audit_log_modification();

CREATE TRIGGER vote_audit_log_immutable_delete
    BEFORE DELETE ON vote_audit_log
    FOR EACH ROW
    EXECUTE FUNCTION prevent_audit_log_modification();

-- Enable Row Level Security
ALTER TABLE vote_audit_log ENABLE ROW LEVEL SECURITY;

-- RLS Policy: Readable by org members, only system can insert
CREATE POLICY vote_audit_log_tenant_isolation ON vote_audit_log
    FOR SELECT
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM votes v
            WHERE v.id = vote_audit_log.vote_id
            AND v.organization_id = get_current_org_id()
        )
    );

-- Separate insert policy (more permissive for system operations)
CREATE POLICY vote_audit_log_insert ON vote_audit_log
    FOR INSERT
    WITH CHECK (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM votes v
            WHERE v.id = vote_audit_log.vote_id
            AND v.organization_id = get_current_org_id()
        )
    );

-- ============================================================================
-- Comments
-- ============================================================================

COMMENT ON TABLE votes IS 'Building votes and polls (Epic 5, UC-04)';
COMMENT ON TABLE vote_questions IS 'Questions within a vote';
COMMENT ON TABLE vote_responses IS 'Submitted ballots (one per unit per vote)';
COMMENT ON TABLE vote_comments IS 'Discussion threads on votes';
COMMENT ON TABLE vote_audit_log IS 'Immutable audit trail for vote integrity';

COMMENT ON COLUMN vote_responses.response_hash IS 'SHA-256 hash for ballot integrity verification';
COMMENT ON COLUMN vote_audit_log.data_hash IS 'SHA-256 hash of action data for tamper detection';
