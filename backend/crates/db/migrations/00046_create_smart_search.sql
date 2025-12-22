-- Epic 13: AI Assistant & Automation
-- Story 13.5: Smart Search with NLP

-- Vector embeddings for semantic search
CREATE TABLE IF NOT EXISTS search_embeddings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    source_type TEXT NOT NULL CHECK (source_type IN ('document', 'announcement', 'fault', 'message', 'comment')),
    source_id UUID NOT NULL,
    content_hash TEXT NOT NULL,
    embedding FLOAT[] NOT NULL,
    metadata JSONB DEFAULT '{}',
    indexed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (source_type, source_id)
);

-- Search history for analytics and personalization
CREATE TABLE IF NOT EXISTS search_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    query TEXT NOT NULL,
    query_type TEXT NOT NULL DEFAULT 'semantic' CHECK (query_type IN ('semantic', 'keyword', 'hybrid')),
    result_count INTEGER NOT NULL DEFAULT 0,
    clicked_results UUID[] DEFAULT '{}',
    search_duration_ms INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Saved searches for quick access
CREATE TABLE IF NOT EXISTS saved_searches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    query TEXT NOT NULL,
    filters JSONB DEFAULT '{}',
    alert_enabled BOOLEAN DEFAULT FALSE,
    alert_frequency TEXT CHECK (alert_frequency IN ('daily', 'weekly', 'instant')),
    last_alerted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_search_embeddings_org ON search_embeddings(organization_id);
CREATE INDEX idx_search_embeddings_source ON search_embeddings(source_type, source_id);
CREATE INDEX idx_search_embeddings_indexed ON search_embeddings(indexed_at DESC);
CREATE INDEX idx_search_history_org ON search_history(organization_id);
CREATE INDEX idx_search_history_user ON search_history(user_id);
CREATE INDEX idx_search_history_created ON search_history(created_at DESC);
CREATE INDEX idx_saved_searches_user ON saved_searches(user_id);

-- RLS policies
ALTER TABLE search_embeddings ENABLE ROW LEVEL SECURITY;
ALTER TABLE search_history ENABLE ROW LEVEL SECURITY;
ALTER TABLE saved_searches ENABLE ROW LEVEL SECURITY;

CREATE POLICY search_embeddings_tenant_isolation ON search_embeddings
    FOR ALL
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY search_history_tenant_isolation ON search_history
    FOR ALL
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY saved_searches_user_isolation ON saved_searches
    FOR ALL
    USING (user_id = current_setting('app.current_user_id', true)::uuid);

-- Trigger
CREATE TRIGGER update_saved_searches_updated_at
    BEFORE UPDATE ON saved_searches
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
