-- Migration: 00061_notification_grouping
-- Epic 29: Advanced Notification Preferences
-- Story 29.4: Smart Notification Grouping

-- ============================================================================
-- NOTIFICATION GROUPS (Story 29.4)
-- ============================================================================

-- Notification group for similar notifications
CREATE TABLE notification_groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Grouping key: entity_type + entity_id (e.g., fault_123, vote_456)
    group_key TEXT NOT NULL,
    entity_type TEXT NOT NULL,           -- fault, vote, announcement, etc.
    entity_id UUID NOT NULL,

    -- Group metadata
    title TEXT NOT NULL,                 -- e.g., "Fault: Broken elevator"
    notification_count INTEGER NOT NULL DEFAULT 1,
    latest_notification_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    -- Status
    is_read BOOLEAN NOT NULL DEFAULT false,
    read_at TIMESTAMPTZ,
    is_expanded BOOLEAN NOT NULL DEFAULT false,  -- User expanded to see all

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT uq_notification_groups_user_key UNIQUE (user_id, group_key)
);

CREATE INDEX idx_notification_groups_user_id ON notification_groups(user_id);
CREATE INDEX idx_notification_groups_entity ON notification_groups(entity_type, entity_id);
CREATE INDEX idx_notification_groups_unread ON notification_groups(user_id, is_read)
    WHERE is_read = false;
CREATE INDEX idx_notification_groups_latest ON notification_groups(user_id, latest_notification_at DESC);

-- Individual notifications within a group
CREATE TABLE grouped_notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id UUID NOT NULL REFERENCES notification_groups(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Notification details
    event_type TEXT NOT NULL,
    title TEXT NOT NULL,
    body TEXT,
    data JSONB,

    -- Actor (who triggered this notification)
    actor_id UUID REFERENCES users(id) ON DELETE SET NULL,
    actor_name TEXT,

    -- Read status (individual)
    is_read BOOLEAN NOT NULL DEFAULT false,
    read_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_grouped_notifications_group_id ON grouped_notifications(group_id);
CREATE INDEX idx_grouped_notifications_user_id ON grouped_notifications(user_id);
CREATE INDEX idx_grouped_notifications_created ON grouped_notifications(group_id, created_at DESC);

-- Trigger for updated_at on notification_groups
CREATE TRIGGER notification_groups_updated_at
    BEFORE UPDATE ON notification_groups
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- RLS
ALTER TABLE notification_groups ENABLE ROW LEVEL SECURITY;
ALTER TABLE grouped_notifications ENABLE ROW LEVEL SECURITY;

CREATE POLICY notification_groups_user_access ON notification_groups
    FOR ALL
    USING (user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID OR is_super_admin())
    WITH CHECK (user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID OR is_super_admin());

CREATE POLICY grouped_notifications_user_access ON grouped_notifications
    FOR ALL
    USING (user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID OR is_super_admin())
    WITH CHECK (user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID OR is_super_admin());

-- ============================================================================
-- NOTIFICATION DIGEST GENERATION (Story 29.3 enhancement)
-- ============================================================================

-- Digest generation history
CREATE TABLE notification_digests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Digest period
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    digest_type TEXT NOT NULL CHECK (digest_type IN ('hourly', 'daily', 'weekly')),

    -- Digest content
    notification_count INTEGER NOT NULL,
    category_counts JSONB NOT NULL DEFAULT '{}',  -- { "fault": 5, "vote": 2, ... }
    summary_html TEXT,
    summary_text TEXT,

    -- Delivery status
    email_sent_at TIMESTAMPTZ,
    push_sent_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_notification_digests_user ON notification_digests(user_id);
CREATE INDEX idx_notification_digests_period ON notification_digests(user_id, period_end DESC);

-- Digest notifications (included in each digest)
CREATE TABLE digest_notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    digest_id UUID NOT NULL REFERENCES notification_digests(id) ON DELETE CASCADE,

    -- Original notification reference
    event_type TEXT NOT NULL,
    event_category TEXT NOT NULL,
    title TEXT NOT NULL,
    body TEXT,
    entity_type TEXT,
    entity_id UUID,

    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_digest_notifications_digest ON digest_notifications(digest_id);

-- RLS
ALTER TABLE notification_digests ENABLE ROW LEVEL SECURITY;
ALTER TABLE digest_notifications ENABLE ROW LEVEL SECURITY;

CREATE POLICY notification_digests_user_access ON notification_digests
    FOR ALL
    USING (user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID OR is_super_admin());

CREATE POLICY digest_notifications_user_access ON digest_notifications
    FOR ALL
    USING (EXISTS (
        SELECT 1 FROM notification_digests d
        WHERE d.id = digest_id
        AND d.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
    ) OR is_super_admin());

-- ============================================================================
-- HELPER FUNCTIONS
-- ============================================================================

-- Function to add notification to group (or create new group)
CREATE OR REPLACE FUNCTION add_notification_to_group(
    p_user_id UUID,
    p_entity_type TEXT,
    p_entity_id UUID,
    p_group_title TEXT,
    p_event_type TEXT,
    p_title TEXT,
    p_body TEXT DEFAULT NULL,
    p_data JSONB DEFAULT NULL,
    p_actor_id UUID DEFAULT NULL,
    p_actor_name TEXT DEFAULT NULL
) RETURNS UUID AS $$
DECLARE
    v_group_key TEXT;
    v_group_id UUID;
    v_notification_id UUID;
BEGIN
    -- Generate group key
    v_group_key := p_entity_type || '_' || p_entity_id::text;

    -- Upsert group
    INSERT INTO notification_groups (user_id, group_key, entity_type, entity_id, title, notification_count, latest_notification_at)
    VALUES (p_user_id, v_group_key, p_entity_type, p_entity_id, p_group_title, 1, NOW())
    ON CONFLICT (user_id, group_key) DO UPDATE SET
        notification_count = notification_groups.notification_count + 1,
        latest_notification_at = NOW(),
        is_read = false,
        updated_at = NOW()
    RETURNING id INTO v_group_id;

    -- Insert notification
    INSERT INTO grouped_notifications (group_id, user_id, event_type, title, body, data, actor_id, actor_name)
    VALUES (v_group_id, p_user_id, p_event_type, p_title, p_body, p_data, p_actor_id, p_actor_name)
    RETURNING id INTO v_notification_id;

    RETURN v_notification_id;
END;
$$ LANGUAGE plpgsql;

-- Function to get grouped notifications for a user
CREATE OR REPLACE FUNCTION get_grouped_notifications(
    p_user_id UUID,
    p_limit INTEGER DEFAULT 50,
    p_offset INTEGER DEFAULT 0,
    p_include_read BOOLEAN DEFAULT false
) RETURNS TABLE (
    group_id UUID,
    group_key TEXT,
    entity_type TEXT,
    entity_id UUID,
    title TEXT,
    notification_count INTEGER,
    latest_notification_at TIMESTAMPTZ,
    is_read BOOLEAN,
    first_notifications JSONB
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        ng.id,
        ng.group_key,
        ng.entity_type,
        ng.entity_id,
        ng.title,
        ng.notification_count,
        ng.latest_notification_at,
        ng.is_read,
        (
            SELECT jsonb_agg(row_to_json(gn.*) ORDER BY gn.created_at DESC)
            FROM (
                SELECT gn.id, gn.event_type, gn.title, gn.body, gn.actor_name, gn.created_at
                FROM grouped_notifications gn
                WHERE gn.group_id = ng.id
                ORDER BY gn.created_at DESC
                LIMIT 5  -- Show last 5 in summary
            ) gn
        ) as first_notifications
    FROM notification_groups ng
    WHERE ng.user_id = p_user_id
      AND (p_include_read OR NOT ng.is_read)
    ORDER BY ng.latest_notification_at DESC
    LIMIT p_limit
    OFFSET p_offset;
END;
$$ LANGUAGE plpgsql;

-- Function to mark group as read
CREATE OR REPLACE FUNCTION mark_notification_group_read(
    p_user_id UUID,
    p_group_id UUID
) RETURNS BOOLEAN AS $$
BEGIN
    UPDATE notification_groups
    SET is_read = true, read_at = NOW()
    WHERE id = p_group_id AND user_id = p_user_id;

    UPDATE grouped_notifications
    SET is_read = true, read_at = NOW()
    WHERE group_id = p_group_id AND user_id = p_user_id;

    RETURN FOUND;
END;
$$ LANGUAGE plpgsql;

-- Function to mark all groups as read
CREATE OR REPLACE FUNCTION mark_all_notification_groups_read(
    p_user_id UUID
) RETURNS INTEGER AS $$
DECLARE
    v_count INTEGER;
BEGIN
    UPDATE notification_groups
    SET is_read = true, read_at = NOW()
    WHERE user_id = p_user_id AND is_read = false;

    GET DIAGNOSTICS v_count = ROW_COUNT;

    UPDATE grouped_notifications
    SET is_read = true, read_at = NOW()
    WHERE user_id = p_user_id AND is_read = false;

    RETURN v_count;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- COMMENTS
-- ============================================================================

COMMENT ON TABLE notification_groups IS 'Groups similar notifications by entity (Epic 29, Story 29.4)';
COMMENT ON TABLE grouped_notifications IS 'Individual notifications within a group';
COMMENT ON TABLE notification_digests IS 'Generated notification digests (Epic 29, Story 29.3)';
COMMENT ON TABLE digest_notifications IS 'Notifications included in each digest';
COMMENT ON FUNCTION add_notification_to_group IS 'Adds a notification to a group, creating group if needed';
COMMENT ON FUNCTION get_grouped_notifications IS 'Gets grouped notifications for a user with pagination';
