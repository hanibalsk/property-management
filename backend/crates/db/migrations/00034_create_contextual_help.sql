-- Epic 10B, Story 10B.7: Contextual Help & Documentation
-- Migration: Create help articles and FAQ infrastructure

-- Create help_articles table
CREATE TABLE IF NOT EXISTS help_articles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug VARCHAR(100) NOT NULL UNIQUE,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    summary TEXT,
    category VARCHAR(50) NOT NULL,
    tags VARCHAR(50)[] DEFAULT '{}',
    context_keys VARCHAR(100)[] DEFAULT '{}', -- e.g., ['page:dashboard', 'feature:faults']
    is_published BOOLEAN NOT NULL DEFAULT false,
    view_count INTEGER NOT NULL DEFAULT 0,
    helpful_count INTEGER NOT NULL DEFAULT 0,
    not_helpful_count INTEGER NOT NULL DEFAULT 0,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create help_categories table
CREATE TABLE IF NOT EXISTS help_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    icon VARCHAR(50),
    display_order INTEGER NOT NULL DEFAULT 0,
    parent_id UUID REFERENCES help_categories(id),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create faq table
CREATE TABLE IF NOT EXISTS faq (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    question TEXT NOT NULL,
    answer TEXT NOT NULL,
    category VARCHAR(50) NOT NULL,
    display_order INTEGER NOT NULL DEFAULT 0,
    is_published BOOLEAN NOT NULL DEFAULT true,
    view_count INTEGER NOT NULL DEFAULT 0,
    helpful_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create tooltips table
CREATE TABLE IF NOT EXISTS tooltips (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key VARCHAR(100) NOT NULL UNIQUE, -- e.g., 'button:submit-fault', 'field:email'
    title VARCHAR(255),
    content TEXT NOT NULL,
    position VARCHAR(20) DEFAULT 'top', -- top, bottom, left, right
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create user_article_feedback table
CREATE TABLE IF NOT EXISTS user_article_feedback (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    article_id UUID NOT NULL REFERENCES help_articles(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    is_helpful BOOLEAN NOT NULL,
    feedback_text TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(article_id, user_id)
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_help_articles_category ON help_articles(category);
CREATE INDEX IF NOT EXISTS idx_help_articles_published ON help_articles(is_published) WHERE is_published = true;
CREATE INDEX IF NOT EXISTS idx_help_articles_context ON help_articles USING GIN(context_keys);
CREATE INDEX IF NOT EXISTS idx_help_articles_tags ON help_articles USING GIN(tags);
CREATE INDEX IF NOT EXISTS idx_faq_category ON faq(category);
CREATE INDEX IF NOT EXISTS idx_faq_published ON faq(is_published) WHERE is_published = true;
CREATE INDEX IF NOT EXISTS idx_tooltips_key ON tooltips(key) WHERE is_active = true;

-- Insert default help categories
INSERT INTO help_categories (slug, name, description, display_order) VALUES
    ('getting-started', 'Getting Started', 'Learn the basics of using the platform', 1),
    ('buildings', 'Buildings & Units', 'Managing buildings, floors, and units', 2),
    ('faults', 'Fault Reporting', 'How to report and track maintenance issues', 3),
    ('voting', 'Voting & Polls', 'Participating in community decisions', 4),
    ('announcements', 'Announcements', 'Creating and viewing announcements', 5),
    ('documents', 'Documents', 'Managing and sharing documents', 6),
    ('account', 'Account & Settings', 'Managing your account and preferences', 7),
    ('troubleshooting', 'Troubleshooting', 'Common issues and solutions', 8)
ON CONFLICT (slug) DO NOTHING;

-- Insert sample FAQ
INSERT INTO faq (question, answer, category, display_order) VALUES
    ('How do I reset my password?', 'Go to the login page and click "Forgot Password". Enter your email address and follow the instructions sent to your email.', 'account', 1),
    ('How do I report a fault?', 'Navigate to Faults > New Fault. Fill in the description, select the location, and optionally attach photos. Submit to create the fault report.', 'faults', 1),
    ('Can I vote on behalf of someone else?', 'Yes, if you have been granted delegation rights. The owner must set up delegation in their account settings.', 'voting', 1),
    ('How do I share a document?', 'Open the document, click Share, and select the recipients or make it public. You can also create a shareable link.', 'documents', 1)
ON CONFLICT DO NOTHING;

-- Insert sample tooltips
INSERT INTO tooltips (key, title, content, position) VALUES
    ('field:email', 'Email Address', 'Enter your email address. This will be used for login and notifications.', 'bottom'),
    ('button:submit-fault', 'Submit Fault', 'Click to submit your fault report. You will receive a confirmation email.', 'top'),
    ('field:vote-proxy', 'Proxy Vote', 'If enabled, you can vote on behalf of other owners who have delegated their vote to you.', 'right')
ON CONFLICT (key) DO NOTHING;

-- Add comments for documentation
COMMENT ON TABLE help_articles IS 'Help documentation articles (Epic 10B, Story 10B.7)';
COMMENT ON TABLE help_categories IS 'Categories for organizing help articles';
COMMENT ON TABLE faq IS 'Frequently asked questions';
COMMENT ON TABLE tooltips IS 'Contextual tooltips for UI elements';
COMMENT ON TABLE user_article_feedback IS 'User feedback on help articles';
COMMENT ON COLUMN help_articles.context_keys IS 'Keys for context-based help (e.g., page:dashboard, feature:faults)';
