-- Epic 59: News & Media Management
-- Creates news_articles, article_reactions, article_comments tables with RLS

-- Article status
CREATE TYPE article_status AS ENUM ('draft', 'published', 'archived');

-- Reaction types for articles and comments
CREATE TYPE reaction_type AS ENUM ('like', 'love', 'surprised', 'sad', 'angry');

-- News Articles table
CREATE TABLE IF NOT EXISTS news_articles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    excerpt TEXT,
    cover_image_url TEXT,
    building_ids JSONB NOT NULL DEFAULT '[]',
    status article_status NOT NULL DEFAULT 'draft',
    published_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ,
    pinned BOOLEAN NOT NULL DEFAULT FALSE,
    pinned_at TIMESTAMPTZ,
    pinned_by UUID REFERENCES users(id) ON DELETE SET NULL,
    comments_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    reactions_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    view_count INT NOT NULL DEFAULT 0,
    reaction_count INT NOT NULL DEFAULT 0,
    comment_count INT NOT NULL DEFAULT 0,
    share_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_news_articles_organization_id ON news_articles(organization_id);
CREATE INDEX idx_news_articles_status ON news_articles(status);
CREATE INDEX idx_news_articles_published_at ON news_articles(published_at) WHERE status = 'published';

CREATE TRIGGER update_news_articles_updated_at
    BEFORE UPDATE ON news_articles
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

ALTER TABLE news_articles ENABLE ROW LEVEL SECURITY;

CREATE POLICY news_articles_tenant_isolation ON news_articles
    FOR ALL
    USING (is_super_admin() OR organization_id = get_current_org_id())
    WITH CHECK (is_super_admin() OR organization_id = get_current_org_id());

-- Article Media table
CREATE TABLE IF NOT EXISTS article_media (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    article_id UUID NOT NULL REFERENCES news_articles(id) ON DELETE CASCADE,
    media_type VARCHAR(50) NOT NULL,
    file_key VARCHAR(512),
    file_name VARCHAR(255),
    file_size BIGINT,
    mime_type VARCHAR(100),
    embed_url TEXT,
    embed_html TEXT,
    width INT,
    height INT,
    alt_text VARCHAR(255),
    caption TEXT,
    display_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_article_media_article_id ON article_media(article_id);

ALTER TABLE article_media ENABLE ROW LEVEL SECURITY;

CREATE POLICY article_media_tenant_isolation ON article_media
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM news_articles a
            WHERE a.id = article_media.article_id
            AND a.organization_id = get_current_org_id()
        )
    )
    WITH CHECK (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM news_articles a
            WHERE a.id = article_media.article_id
            AND a.organization_id = get_current_org_id()
        )
    );

-- Article Reactions table
CREATE TABLE IF NOT EXISTS article_reactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    article_id UUID NOT NULL REFERENCES news_articles(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reaction reaction_type NOT NULL DEFAULT 'like',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_article_reaction UNIQUE (article_id, user_id)
);

CREATE INDEX idx_article_reactions_article_id ON article_reactions(article_id);

ALTER TABLE article_reactions ENABLE ROW LEVEL SECURITY;

CREATE POLICY article_reactions_tenant_isolation ON article_reactions
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM news_articles a
            WHERE a.id = article_reactions.article_id
            AND a.organization_id = get_current_org_id()
        )
    )
    WITH CHECK (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM news_articles a
            WHERE a.id = article_reactions.article_id
            AND a.organization_id = get_current_org_id()
        )
    );

-- Article Comments table
CREATE TABLE IF NOT EXISTS article_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    article_id UUID NOT NULL REFERENCES news_articles(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES article_comments(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    is_moderated BOOLEAN NOT NULL DEFAULT FALSE,
    moderated_at TIMESTAMPTZ,
    moderated_by UUID REFERENCES users(id) ON DELETE SET NULL,
    moderation_reason TEXT,
    deleted_at TIMESTAMPTZ,
    deleted_by UUID REFERENCES users(id) ON DELETE SET NULL,
    like_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_article_comments_article_id ON article_comments(article_id);
CREATE INDEX idx_article_comments_parent_id ON article_comments(parent_id);

CREATE TRIGGER update_article_comments_updated_at
    BEFORE UPDATE ON article_comments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

ALTER TABLE article_comments ENABLE ROW LEVEL SECURITY;

CREATE POLICY article_comments_tenant_isolation ON article_comments
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM news_articles a
            WHERE a.id = article_comments.article_id
            AND a.organization_id = get_current_org_id()
        )
    )
    WITH CHECK (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM news_articles a
            WHERE a.id = article_comments.article_id
            AND a.organization_id = get_current_org_id()
        )
    );

-- Article Views table
CREATE TABLE IF NOT EXISTS article_views (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    article_id UUID NOT NULL REFERENCES news_articles(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    viewed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    duration_seconds INT,
    CONSTRAINT unique_article_view UNIQUE (article_id, user_id)
);

CREATE INDEX idx_article_views_article_id ON article_views(article_id);

ALTER TABLE article_views ENABLE ROW LEVEL SECURITY;

CREATE POLICY article_views_tenant_isolation ON article_views
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM news_articles a
            WHERE a.id = article_views.article_id
            AND a.organization_id = get_current_org_id()
        )
    )
    WITH CHECK (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM news_articles a
            WHERE a.id = article_views.article_id
            AND a.organization_id = get_current_org_id()
        )
    );

-- Article Comment Reactions table
CREATE TABLE IF NOT EXISTS article_comment_reactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    comment_id UUID NOT NULL REFERENCES article_comments(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reaction reaction_type NOT NULL DEFAULT 'like',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_comment_reaction UNIQUE (comment_id, user_id)
);

CREATE INDEX idx_article_comment_reactions_comment_id ON article_comment_reactions(comment_id);

ALTER TABLE article_comment_reactions ENABLE ROW LEVEL SECURITY;

CREATE POLICY article_comment_reactions_tenant_isolation ON article_comment_reactions
    FOR ALL
    USING (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM article_comments c
            JOIN news_articles a ON a.id = c.article_id
            WHERE c.id = article_comment_reactions.comment_id
            AND a.organization_id = get_current_org_id()
        )
    )
    WITH CHECK (
        is_super_admin()
        OR EXISTS (
            SELECT 1 FROM article_comments c
            JOIN news_articles a ON a.id = c.article_id
            WHERE c.id = article_comment_reactions.comment_id
            AND a.organization_id = get_current_org_id()
        )
    );

COMMENT ON TABLE news_articles IS 'News articles for residents (Epic 59)';
COMMENT ON TABLE article_media IS 'Media attachments for news articles';
COMMENT ON TABLE article_reactions IS 'Reactions to news articles (Story 59.2)';
COMMENT ON TABLE article_comments IS 'Comments on news articles (Story 59.3)';
