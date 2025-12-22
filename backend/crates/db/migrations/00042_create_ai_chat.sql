-- Epic 13: AI Assistant & Automation
-- Story 13.1: AI Chatbot Interface

-- Chat sessions for AI assistant
CREATE TABLE IF NOT EXISTS ai_chat_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT,
    context JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_message_at TIMESTAMPTZ
);

-- Chat messages
CREATE TABLE IF NOT EXISTS ai_chat_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id UUID NOT NULL REFERENCES ai_chat_sessions(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
    content TEXT NOT NULL,
    confidence FLOAT,
    sources JSONB DEFAULT '[]',
    escalated BOOLEAN DEFAULT FALSE,
    escalation_reason TEXT,
    tokens_used INTEGER,
    latency_ms INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Training data for improving responses
CREATE TABLE IF NOT EXISTS ai_training_feedback (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    message_id UUID NOT NULL REFERENCES ai_chat_messages(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    rating INTEGER CHECK (rating >= 1 AND rating <= 5),
    helpful BOOLEAN,
    feedback_text TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_ai_chat_sessions_org ON ai_chat_sessions(organization_id);
CREATE INDEX idx_ai_chat_sessions_user ON ai_chat_sessions(user_id);
CREATE INDEX idx_ai_chat_sessions_updated ON ai_chat_sessions(updated_at DESC);
CREATE INDEX idx_ai_chat_messages_session ON ai_chat_messages(session_id);
CREATE INDEX idx_ai_chat_messages_created ON ai_chat_messages(created_at DESC);
CREATE INDEX idx_ai_training_feedback_message ON ai_training_feedback(message_id);

-- RLS policies
ALTER TABLE ai_chat_sessions ENABLE ROW LEVEL SECURITY;
ALTER TABLE ai_chat_messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE ai_training_feedback ENABLE ROW LEVEL SECURITY;

CREATE POLICY ai_chat_sessions_tenant_isolation ON ai_chat_sessions
    FOR ALL
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY ai_chat_messages_tenant_isolation ON ai_chat_messages
    FOR ALL
    USING (session_id IN (
        SELECT id FROM ai_chat_sessions
        WHERE organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

CREATE POLICY ai_training_feedback_tenant_isolation ON ai_training_feedback
    FOR ALL
    USING (message_id IN (
        SELECT m.id FROM ai_chat_messages m
        JOIN ai_chat_sessions s ON s.id = m.session_id
        WHERE s.organization_id = current_setting('app.current_organization_id', true)::uuid
    ));

-- Trigger for updated_at
CREATE TRIGGER update_ai_chat_sessions_updated_at
    BEFORE UPDATE ON ai_chat_sessions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
