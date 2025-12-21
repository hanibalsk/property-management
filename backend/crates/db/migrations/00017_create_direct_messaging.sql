-- Migration: 00017_create_direct_messaging
-- Epic 6 / Story 6.5: Direct Messaging
--
-- Creates tables for direct messaging between users within an organization:
-- - message_threads: Conversation threads between participants
-- - messages: Individual messages within threads
-- - user_blocks: User blocking for privacy

-- ============================================================================
-- MESSAGE THREADS TABLE
-- ============================================================================

CREATE TABLE IF NOT EXISTS message_threads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Participant user IDs (exactly 2 for direct messages)
    participant_ids UUID[] NOT NULL,

    -- Last message timestamp for sorting
    last_message_at TIMESTAMPTZ,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Ensure exactly 2 participants
    CONSTRAINT message_threads_two_participants CHECK (array_length(participant_ids, 1) = 2)
);

-- Indexes for message_threads
CREATE INDEX IF NOT EXISTS idx_message_threads_organization ON message_threads(organization_id);
CREATE INDEX IF NOT EXISTS idx_message_threads_participants ON message_threads USING GIN(participant_ids);
CREATE INDEX IF NOT EXISTS idx_message_threads_last_message ON message_threads(last_message_at DESC NULLS LAST);

COMMENT ON TABLE message_threads IS 'Direct message conversation threads between users';
COMMENT ON COLUMN message_threads.participant_ids IS 'Array of exactly 2 user UUIDs participating in the thread';

-- ============================================================================
-- MESSAGES TABLE
-- ============================================================================

CREATE TABLE IF NOT EXISTS messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    thread_id UUID NOT NULL REFERENCES message_threads(id) ON DELETE CASCADE,
    sender_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Message content
    content TEXT NOT NULL,

    -- Read receipt
    read_at TIMESTAMPTZ,

    -- Soft delete
    deleted_at TIMESTAMPTZ,
    deleted_by UUID REFERENCES users(id) ON DELETE SET NULL,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Content length limit (10KB)
    CONSTRAINT messages_content_length CHECK (length(content) <= 10000)
);

-- Indexes for messages
CREATE INDEX IF NOT EXISTS idx_messages_thread ON messages(thread_id);
CREATE INDEX IF NOT EXISTS idx_messages_sender ON messages(sender_id);
CREATE INDEX IF NOT EXISTS idx_messages_thread_created ON messages(thread_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_messages_unread ON messages(thread_id, read_at) WHERE deleted_at IS NULL;

COMMENT ON TABLE messages IS 'Individual messages within a conversation thread';
COMMENT ON COLUMN messages.read_at IS 'Timestamp when recipient read the message (NULL if unread)';

-- ============================================================================
-- USER BLOCKS TABLE
-- ============================================================================

CREATE TABLE IF NOT EXISTS user_blocks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- The user doing the blocking
    blocker_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- The user being blocked
    blocked_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Prevent self-blocking and duplicate blocks
    CONSTRAINT user_blocks_no_self_block CHECK (blocker_id != blocked_id),
    CONSTRAINT user_blocks_unique UNIQUE (blocker_id, blocked_id)
);

-- Indexes for user_blocks
CREATE INDEX IF NOT EXISTS idx_user_blocks_blocker ON user_blocks(blocker_id);
CREATE INDEX IF NOT EXISTS idx_user_blocks_blocked ON user_blocks(blocked_id);

COMMENT ON TABLE user_blocks IS 'User blocking for privacy (blocked user cannot message blocker)';

-- ============================================================================
-- TRIGGERS
-- ============================================================================

-- Update message_threads.last_message_at when a new message is inserted
CREATE OR REPLACE FUNCTION update_thread_last_message()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE message_threads
    SET last_message_at = NEW.created_at,
        updated_at = NOW()
    WHERE id = NEW.thread_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_thread_last_message
    AFTER INSERT ON messages
    FOR EACH ROW
    EXECUTE FUNCTION update_thread_last_message();

-- Update updated_at on message_threads changes
CREATE TRIGGER trigger_message_threads_updated_at
    BEFORE UPDATE ON message_threads
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

-- Update updated_at on messages changes
CREATE TRIGGER trigger_messages_updated_at
    BEFORE UPDATE ON messages
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

-- ============================================================================
-- ROW LEVEL SECURITY
-- ============================================================================

-- Enable RLS
ALTER TABLE message_threads ENABLE ROW LEVEL SECURITY;
ALTER TABLE messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE user_blocks ENABLE ROW LEVEL SECURITY;

-- Message threads: Users can only see threads they participate in
CREATE POLICY message_threads_tenant_isolation ON message_threads
    FOR ALL
    USING (
        current_setting('app.current_user_id', true)::UUID = ANY(participant_ids)
    );

-- Messages: Users can only see messages in threads they participate in
CREATE POLICY messages_tenant_isolation ON messages
    FOR ALL
    USING (
        thread_id IN (
            SELECT id FROM message_threads
            WHERE current_setting('app.current_user_id', true)::UUID = ANY(participant_ids)
        )
    );

-- User blocks: Users can only see/manage their own blocks
CREATE POLICY user_blocks_owner_isolation ON user_blocks
    FOR ALL
    USING (
        blocker_id = current_setting('app.current_user_id', true)::UUID
    );
