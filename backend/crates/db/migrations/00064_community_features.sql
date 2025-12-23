-- Epic 37: Community & Social Features
-- Epic 38: Advanced Workflow Automation

-- =============================================================================
-- COMMUNITY GROUPS (Story 37.1)
-- =============================================================================

-- Community group types
CREATE TYPE community_group_type AS ENUM (
    'interest',       -- Interest-based (e.g., "Pet Owners", "Gardening")
    'floor',          -- Floor-specific groups
    'building',       -- Building-wide groups
    'neighborhood',   -- Cross-building neighborhood
    'official'        -- Official management groups
);

-- Community group visibility
CREATE TYPE community_group_visibility AS ENUM (
    'public',         -- Anyone in building can see and join
    'private',        -- Invite-only, visible in directory
    'secret'          -- Invite-only, not in directory
);

-- Community groups
CREATE TABLE community_groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL,
    description TEXT,
    group_type community_group_type NOT NULL DEFAULT 'interest',
    visibility community_group_visibility NOT NULL DEFAULT 'public',
    cover_image_url TEXT,
    icon VARCHAR(50),
    color VARCHAR(7),  -- Hex color
    rules TEXT,
    max_members INT,
    auto_join_new_residents BOOLEAN NOT NULL DEFAULT FALSE,
    requires_approval BOOLEAN NOT NULL DEFAULT FALSE,
    is_official BOOLEAN NOT NULL DEFAULT FALSE,
    member_count INT NOT NULL DEFAULT 0,
    post_count INT NOT NULL DEFAULT 0,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    UNIQUE(building_id, slug)
);

-- Community group members
CREATE TYPE community_member_role AS ENUM ('member', 'moderator', 'admin', 'owner');

CREATE TABLE community_group_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id UUID NOT NULL REFERENCES community_groups(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role community_member_role NOT NULL DEFAULT 'member',
    notification_settings JSONB NOT NULL DEFAULT '{"posts": true, "events": true, "mentions": true}'::jsonb,
    joined_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    invited_by UUID REFERENCES users(id),

    UNIQUE(group_id, user_id)
);

-- Group join requests (for approval-required groups)
CREATE TABLE community_join_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id UUID NOT NULL REFERENCES community_groups(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    message TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',  -- pending, approved, rejected
    reviewed_by UUID REFERENCES users(id),
    reviewed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    UNIQUE(group_id, user_id)
);

CREATE INDEX idx_community_groups_building ON community_groups(building_id);
CREATE INDEX idx_community_group_members_user ON community_group_members(user_id);
CREATE INDEX idx_community_group_members_group ON community_group_members(group_id);

-- =============================================================================
-- COMMUNITY FEED / POSTS (Story 37.2)
-- =============================================================================

-- Post types
CREATE TYPE community_post_type AS ENUM (
    'text',           -- Simple text post
    'photo',          -- Photo post
    'poll',           -- Poll/survey
    'event',          -- Event announcement
    'marketplace',    -- Item for sale/trade
    'question',       -- Q&A style
    'announcement'    -- Official announcement
);

-- Community posts
CREATE TABLE community_posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id UUID NOT NULL REFERENCES community_groups(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id),
    post_type community_post_type NOT NULL DEFAULT 'text',
    title VARCHAR(200),
    content TEXT NOT NULL,
    media_urls TEXT[],
    poll_options JSONB,  -- For poll posts: [{"id": "1", "text": "Option 1", "votes": 0}]
    poll_ends_at TIMESTAMP WITH TIME ZONE,
    poll_multiple_choice BOOLEAN DEFAULT FALSE,
    is_pinned BOOLEAN NOT NULL DEFAULT FALSE,
    is_locked BOOLEAN NOT NULL DEFAULT FALSE,  -- No more comments
    is_anonymous BOOLEAN NOT NULL DEFAULT FALSE,
    view_count INT NOT NULL DEFAULT 0,
    like_count INT NOT NULL DEFAULT 0,
    comment_count INT NOT NULL DEFAULT 0,
    share_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    edited_at TIMESTAMP WITH TIME ZONE
);

-- Post reactions
CREATE TABLE community_post_reactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id UUID NOT NULL REFERENCES community_posts(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reaction_type VARCHAR(20) NOT NULL DEFAULT 'like',  -- like, love, helpful, funny
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    UNIQUE(post_id, user_id, reaction_type)
);

-- Post comments
CREATE TABLE community_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id UUID NOT NULL REFERENCES community_posts(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES community_comments(id) ON DELETE CASCADE,  -- For nested replies
    author_id UUID NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    is_anonymous BOOLEAN NOT NULL DEFAULT FALSE,
    like_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    edited_at TIMESTAMP WITH TIME ZONE
);

-- Comment reactions
CREATE TABLE community_comment_reactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    comment_id UUID NOT NULL REFERENCES community_comments(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reaction_type VARCHAR(20) NOT NULL DEFAULT 'like',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    UNIQUE(comment_id, user_id)
);

-- Poll votes
CREATE TABLE community_poll_votes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id UUID NOT NULL REFERENCES community_posts(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    option_id VARCHAR(36) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    UNIQUE(post_id, user_id, option_id)
);

CREATE INDEX idx_community_posts_group ON community_posts(group_id);
CREATE INDEX idx_community_posts_author ON community_posts(author_id);
CREATE INDEX idx_community_posts_created ON community_posts(created_at DESC);
CREATE INDEX idx_community_comments_post ON community_comments(post_id);

-- =============================================================================
-- COMMUNITY EVENTS (Story 37.3)
-- =============================================================================

-- Event types
CREATE TYPE community_event_type AS ENUM (
    'social',         -- Social gathering
    'meeting',        -- Group meeting
    'workshop',       -- Workshop/class
    'volunteer',      -- Volunteer activity
    'sports',         -- Sports activity
    'cultural',       -- Cultural event
    'other'
);

-- Community events
CREATE TABLE community_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id UUID REFERENCES community_groups(id) ON DELETE SET NULL,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    organizer_id UUID NOT NULL REFERENCES users(id),
    title VARCHAR(200) NOT NULL,
    description TEXT,
    event_type community_event_type NOT NULL DEFAULT 'social',
    location VARCHAR(200),
    location_details TEXT,  -- Room number, floor, etc.
    is_virtual BOOLEAN NOT NULL DEFAULT FALSE,
    virtual_link TEXT,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE NOT NULL,
    all_day BOOLEAN NOT NULL DEFAULT FALSE,
    recurring_rule TEXT,  -- iCal RRULE format
    cover_image_url TEXT,
    max_attendees INT,
    requires_rsvp BOOLEAN NOT NULL DEFAULT TRUE,
    rsvp_deadline TIMESTAMP WITH TIME ZONE,
    cost_per_person DECIMAL(10, 2),
    cost_currency VARCHAR(3) DEFAULT 'EUR',
    is_public BOOLEAN NOT NULL DEFAULT TRUE,
    status VARCHAR(20) NOT NULL DEFAULT 'scheduled',  -- scheduled, cancelled, completed
    attendee_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Event RSVPs
CREATE TYPE event_rsvp_status AS ENUM ('going', 'maybe', 'not_going', 'waitlist');

CREATE TABLE community_event_rsvps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL REFERENCES community_events(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status event_rsvp_status NOT NULL DEFAULT 'going',
    guests INT NOT NULL DEFAULT 0,
    note TEXT,
    reminder_sent BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    UNIQUE(event_id, user_id)
);

-- Event comments
CREATE TABLE community_event_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL REFERENCES community_events(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_community_events_building ON community_events(building_id);
CREATE INDEX idx_community_events_group ON community_events(group_id);
CREATE INDEX idx_community_events_start ON community_events(start_time);
CREATE INDEX idx_community_event_rsvps_event ON community_event_rsvps(event_id);
CREATE INDEX idx_community_event_rsvps_user ON community_event_rsvps(user_id);

-- =============================================================================
-- ITEM MARKETPLACE (Story 37.4)
-- =============================================================================

-- Item categories
CREATE TYPE marketplace_category AS ENUM (
    'furniture',
    'electronics',
    'clothing',
    'sports',
    'books',
    'toys',
    'home_garden',
    'services',
    'free',
    'wanted',
    'other'
);

-- Item condition
CREATE TYPE item_condition AS ENUM (
    'new',
    'like_new',
    'good',
    'fair',
    'for_parts'
);

-- Marketplace items
CREATE TABLE marketplace_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    seller_id UUID NOT NULL REFERENCES users(id),
    title VARCHAR(200) NOT NULL,
    description TEXT,
    category marketplace_category NOT NULL,
    condition item_condition NOT NULL DEFAULT 'good',
    price DECIMAL(10, 2),
    currency VARCHAR(3) DEFAULT 'EUR',
    is_free BOOLEAN NOT NULL DEFAULT FALSE,
    is_negotiable BOOLEAN NOT NULL DEFAULT TRUE,
    is_trade_accepted BOOLEAN NOT NULL DEFAULT FALSE,
    photo_urls TEXT[],
    location VARCHAR(200),  -- "Floor 3", "Garage", etc.
    pickup_details TEXT,
    view_count INT NOT NULL DEFAULT 0,
    inquiry_count INT NOT NULL DEFAULT 0,
    status VARCHAR(20) NOT NULL DEFAULT 'active',  -- active, sold, reserved, expired, removed
    sold_to UUID REFERENCES users(id),
    sold_at TIMESTAMP WITH TIME ZONE,
    expires_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Item inquiries
CREATE TABLE marketplace_inquiries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    item_id UUID NOT NULL REFERENCES marketplace_items(id) ON DELETE CASCADE,
    buyer_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    message TEXT NOT NULL,
    offer_price DECIMAL(10, 2),
    status VARCHAR(20) NOT NULL DEFAULT 'pending',  -- pending, replied, accepted, declined
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Favorite items
CREATE TABLE marketplace_favorites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    item_id UUID NOT NULL REFERENCES marketplace_items(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    UNIQUE(item_id, user_id)
);

CREATE INDEX idx_marketplace_items_building ON marketplace_items(building_id);
CREATE INDEX idx_marketplace_items_seller ON marketplace_items(seller_id);
CREATE INDEX idx_marketplace_items_category ON marketplace_items(category);
CREATE INDEX idx_marketplace_items_status ON marketplace_items(status);
CREATE INDEX idx_marketplace_inquiries_item ON marketplace_inquiries(item_id);

-- =============================================================================
-- WORKFLOW AUTOMATION (Epic 38)
-- =============================================================================

-- Workflow trigger types
CREATE TYPE workflow_automation_trigger AS ENUM (
    'schedule',           -- Cron-based schedule
    'event',              -- Event-triggered (fault created, payment received, etc.)
    'condition',          -- Condition-based (balance > X, occupancy < Y)
    'manual'              -- Manually triggered
);

-- Automation action types
CREATE TYPE automation_action_type AS ENUM (
    'send_notification',
    'send_email',
    'create_task',
    'assign_fault',
    'update_status',
    'generate_report',
    'call_webhook',
    'run_script'
);

-- Workflow automation rules
CREATE TABLE workflow_automation_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    trigger_type workflow_automation_trigger NOT NULL,
    trigger_config JSONB NOT NULL DEFAULT '{}'::jsonb,
    -- For schedule: {"cron": "0 9 * * 1", "timezone": "Europe/Bratislava"}
    -- For event: {"event_type": "fault.created", "conditions": {"priority": "high"}}
    -- For condition: {"check": "unit.balance", "operator": ">", "value": 500}
    conditions JSONB,  -- Additional conditions to check
    actions JSONB NOT NULL DEFAULT '[]'::jsonb,
    -- [{"type": "send_notification", "config": {"template": "...", "recipients": ["manager"]}}]
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    last_run_at TIMESTAMP WITH TIME ZONE,
    next_run_at TIMESTAMP WITH TIME ZONE,
    run_count INT NOT NULL DEFAULT 0,
    error_count INT NOT NULL DEFAULT 0,
    last_error TEXT,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Automation execution log
CREATE TABLE workflow_automation_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rule_id UUID NOT NULL REFERENCES workflow_automation_rules(id) ON DELETE CASCADE,
    trigger_data JSONB,  -- What triggered this execution
    actions_executed JSONB,  -- Results of each action
    status VARCHAR(20) NOT NULL DEFAULT 'pending',  -- pending, running, success, partial_failure, failed
    started_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE,
    error_message TEXT,
    duration_ms INT
);

-- Automation templates (predefined workflows)
CREATE TABLE workflow_automation_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    description TEXT,
    category VARCHAR(50) NOT NULL,  -- 'maintenance', 'financial', 'communication', 'compliance'
    trigger_type workflow_automation_trigger NOT NULL,
    trigger_config_template JSONB NOT NULL,
    actions_template JSONB NOT NULL,
    is_system BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_automation_rules_org ON workflow_automation_rules(organization_id);
CREATE INDEX idx_automation_rules_active ON workflow_automation_rules(is_active) WHERE is_active = TRUE;
CREATE INDEX idx_automation_rules_next_run ON workflow_automation_rules(next_run_at) WHERE is_active = TRUE;
CREATE INDEX idx_automation_logs_rule ON workflow_automation_logs(rule_id);
CREATE INDEX idx_automation_logs_started ON workflow_automation_logs(started_at DESC);

-- =============================================================================
-- INSERT DEFAULT AUTOMATION TEMPLATES
-- =============================================================================

INSERT INTO workflow_automation_templates (name, description, category, trigger_type, trigger_config_template, actions_template, is_system) VALUES
('Weekly Maintenance Report', 'Send weekly summary of open maintenance requests to managers', 'maintenance', 'schedule',
 '{"cron": "0 9 * * 1", "timezone": "Europe/Bratislava"}',
 '[{"type": "generate_report", "config": {"report": "maintenance_summary", "period": "week"}}, {"type": "send_email", "config": {"recipients": ["managers"], "template": "maintenance_weekly"}}]',
 TRUE),

('Overdue Payment Reminder', 'Send reminder when payment is overdue by 7 days', 'financial', 'condition',
 '{"check": "invoice.days_overdue", "operator": ">=", "value": 7}',
 '[{"type": "send_notification", "config": {"template": "payment_reminder", "recipients": ["unit_owner"]}}, {"type": "send_email", "config": {"template": "payment_reminder_email"}}]',
 TRUE),

('High Priority Fault Escalation', 'Auto-assign high priority faults and notify manager', 'maintenance', 'event',
 '{"event_type": "fault.created", "conditions": {"priority": "high"}}',
 '[{"type": "assign_fault", "config": {"assign_to": "on_call"}}, {"type": "send_notification", "config": {"template": "high_priority_fault", "recipients": ["manager", "maintenance_lead"]}}]',
 TRUE),

('Monthly Statement Generation', 'Generate and send monthly statements on the 1st', 'financial', 'schedule',
 '{"cron": "0 8 1 * *", "timezone": "Europe/Bratislava"}',
 '[{"type": "generate_report", "config": {"report": "monthly_statement", "per_unit": true}}, {"type": "send_email", "config": {"template": "monthly_statement", "recipients": ["unit_owner"]}}]',
 TRUE);

-- =============================================================================
-- TRIGGER FUNCTIONS
-- =============================================================================

-- Update member count on join/leave
CREATE OR REPLACE FUNCTION update_group_member_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE community_groups SET member_count = member_count + 1, updated_at = NOW()
        WHERE id = NEW.group_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE community_groups SET member_count = member_count - 1, updated_at = NOW()
        WHERE id = OLD.group_id;
    END IF;
    RETURN NULL;
END;
RETURNS TRIGGER AS $$

CREATE TRIGGER trg_group_member_count
AFTER INSERT OR DELETE ON community_group_members
FOR EACH ROW EXECUTE FUNCTION update_group_member_count();

-- Update post count on create/delete
CREATE OR REPLACE FUNCTION update_group_post_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE community_groups SET post_count = post_count + 1, updated_at = NOW()
        WHERE id = NEW.group_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE community_groups SET post_count = post_count - 1, updated_at = NOW()
        WHERE id = OLD.group_id;
    END IF;
    RETURN NULL;
END;
RETURNS TRIGGER AS $$

CREATE TRIGGER trg_group_post_count
AFTER INSERT OR DELETE ON community_posts
FOR EACH ROW EXECUTE FUNCTION update_group_post_count();

-- Update event attendee count
CREATE OR REPLACE FUNCTION update_event_attendee_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
        UPDATE community_events SET attendee_count = (
            SELECT COUNT(*) FROM community_event_rsvps
            WHERE event_id = NEW.event_id AND status IN ('going', 'maybe')
        ), updated_at = NOW()
        WHERE id = NEW.event_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE community_events SET attendee_count = (
            SELECT COUNT(*) FROM community_event_rsvps
            WHERE event_id = OLD.event_id AND status IN ('going', 'maybe')
        ), updated_at = NOW()
        WHERE id = OLD.event_id;
    END IF;
    RETURN NULL;
END;
RETURNS TRIGGER AS $$

CREATE TRIGGER trg_event_attendee_count
AFTER INSERT OR UPDATE OR DELETE ON community_event_rsvps
FOR EACH ROW EXECUTE FUNCTION update_event_attendee_count();

-- Update comment count on post
CREATE OR REPLACE FUNCTION update_post_comment_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE community_posts SET comment_count = comment_count + 1, updated_at = NOW()
        WHERE id = NEW.post_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE community_posts SET comment_count = comment_count - 1, updated_at = NOW()
        WHERE id = OLD.post_id;
    END IF;
    RETURN NULL;
END;
RETURNS TRIGGER AS $$

CREATE TRIGGER trg_post_comment_count
AFTER INSERT OR DELETE ON community_comments
FOR EACH ROW EXECUTE FUNCTION update_post_comment_count();
