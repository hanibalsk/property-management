-- Epic 31-34: Reality Portal Professional
-- Migration: Portal users, favorites, saved searches, agencies, realtors, property import

-- ============================================
-- Portal Users (Story 31.1 - User Identity)
-- ============================================

CREATE TABLE portal_users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255),  -- NULL if SSO-only
    pm_user_id UUID REFERENCES users(id) ON DELETE SET NULL,  -- Link to PM user for SSO
    provider VARCHAR(20) NOT NULL DEFAULT 'local',  -- local, google, facebook, pm_sso
    email_verified BOOLEAN NOT NULL DEFAULT false,
    profile_image_url VARCHAR(1000),
    locale VARCHAR(10) NOT NULL DEFAULT 'sk',

    -- Notification preferences
    email_notifications BOOLEAN NOT NULL DEFAULT true,
    push_notifications BOOLEAN NOT NULL DEFAULT true,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_portal_users_email ON portal_users(email);
CREATE INDEX idx_portal_users_pm_user ON portal_users(pm_user_id);

-- ============================================
-- Portal Sessions
-- ============================================

CREATE TABLE portal_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES portal_users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL UNIQUE,
    device_info VARCHAR(500),
    ip_address INET,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_portal_sessions_user ON portal_sessions(user_id);
CREATE INDEX idx_portal_sessions_token ON portal_sessions(token_hash);
CREATE INDEX idx_portal_sessions_expires ON portal_sessions(expires_at);

-- ============================================
-- Portal Favorites (Story 31.1)
-- ============================================

CREATE TABLE portal_favorites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES portal_users(id) ON DELETE CASCADE,
    listing_id UUID NOT NULL REFERENCES listings(id) ON DELETE CASCADE,
    notes TEXT,

    -- Price tracking (Story 31.4)
    original_price DECIMAL(15, 2),  -- Price when favorited
    price_alert_enabled BOOLEAN NOT NULL DEFAULT true,
    last_price_alert_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(user_id, listing_id)
);

CREATE INDEX idx_portal_favorites_user ON portal_favorites(user_id);
CREATE INDEX idx_portal_favorites_listing ON portal_favorites(listing_id);
CREATE INDEX idx_portal_favorites_price_alert ON portal_favorites(user_id, price_alert_enabled) WHERE price_alert_enabled = true;

-- ============================================
-- Price History (Story 31.4 - Price Change Alerts)
-- ============================================

CREATE TABLE listing_price_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    listing_id UUID NOT NULL REFERENCES listings(id) ON DELETE CASCADE,
    old_price DECIMAL(15, 2) NOT NULL,
    new_price DECIMAL(15, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',
    change_percentage DECIMAL(5, 2),  -- Calculated: (new - old) / old * 100
    changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_listing_price_history_listing ON listing_price_history(listing_id);
CREATE INDEX idx_listing_price_history_changed ON listing_price_history(changed_at DESC);

-- Trigger to track price changes
CREATE OR REPLACE FUNCTION track_listing_price_change()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.price IS DISTINCT FROM NEW.price THEN
        INSERT INTO listing_price_history (listing_id, old_price, new_price, currency, change_percentage)
        VALUES (
            NEW.id,
            OLD.price,
            NEW.price,
            NEW.currency,
            CASE WHEN OLD.price > 0 THEN ((NEW.price - OLD.price) / OLD.price * 100)::DECIMAL(5,2) ELSE 0 END
        );
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_listing_price_change
    AFTER UPDATE OF price ON listings
    FOR EACH ROW
    EXECUTE FUNCTION track_listing_price_change();

-- ============================================
-- Portal Saved Searches (Story 31.2, 31.3)
-- ============================================

CREATE TABLE portal_saved_searches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES portal_users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    criteria JSONB NOT NULL,  -- SearchCriteria: {property_type, price_min, price_max, city, etc.}

    -- Alert settings (Story 31.3)
    alerts_enabled BOOLEAN NOT NULL DEFAULT true,
    alert_frequency VARCHAR(20) NOT NULL DEFAULT 'daily',  -- instant, daily, weekly

    -- Tracking
    last_matched_at TIMESTAMPTZ,  -- Last time new listings matched
    match_count INTEGER NOT NULL DEFAULT 0,  -- Total matches
    last_alert_sent_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_portal_saved_searches_user ON portal_saved_searches(user_id);
CREATE INDEX idx_portal_saved_searches_alerts ON portal_saved_searches(alerts_enabled, alert_frequency) WHERE alerts_enabled = true;

-- ============================================
-- Search Alert Queue (for background processing)
-- ============================================

CREATE TABLE search_alert_queue (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    saved_search_id UUID NOT NULL REFERENCES portal_saved_searches(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES portal_users(id) ON DELETE CASCADE,
    matching_listing_ids UUID[] NOT NULL,
    alert_type VARCHAR(20) NOT NULL DEFAULT 'new_listing',  -- new_listing, price_change
    status VARCHAR(20) NOT NULL DEFAULT 'pending',  -- pending, sent, failed
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_search_alert_queue_status ON search_alert_queue(status) WHERE status = 'pending';
CREATE INDEX idx_search_alert_queue_user ON search_alert_queue(user_id);

-- ============================================
-- Reality Agencies (Story 32.1)
-- ============================================

CREATE TABLE reality_agencies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,

    -- Contact
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(50),
    website VARCHAR(500),

    -- Address
    street VARCHAR(255),
    city VARCHAR(100),
    postal_code VARCHAR(20),
    country VARCHAR(3) NOT NULL DEFAULT 'SK',

    -- Branding (Story 32.4)
    logo_url VARCHAR(1000),
    banner_url VARCHAR(1000),
    primary_color VARCHAR(7),  -- Hex: #FF5733
    secondary_color VARCHAR(7),
    logo_watermark_position VARCHAR(20) DEFAULT 'bottom-right',  -- top-left, top-right, bottom-left, bottom-right

    -- Description
    description TEXT,
    tagline VARCHAR(255),

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'pending',  -- pending, verified, suspended
    verified_at TIMESTAMPTZ,

    -- Subscription/Plan
    plan VARCHAR(20) NOT NULL DEFAULT 'basic',  -- basic, professional, enterprise
    max_listings INTEGER NOT NULL DEFAULT 20,
    max_realtors INTEGER NOT NULL DEFAULT 3,

    -- Stats cache (refreshed periodically)
    total_listings INTEGER NOT NULL DEFAULT 0,
    active_listings INTEGER NOT NULL DEFAULT 0,
    total_views INTEGER NOT NULL DEFAULT 0,
    total_inquiries INTEGER NOT NULL DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_reality_agencies_slug ON reality_agencies(slug);
CREATE INDEX idx_reality_agencies_status ON reality_agencies(status);
CREATE INDEX idx_reality_agencies_city ON reality_agencies(city);

-- ============================================
-- Agency Members (Story 32.2)
-- ============================================

CREATE TABLE reality_agency_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agency_id UUID NOT NULL REFERENCES reality_agencies(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES portal_users(id) ON DELETE CASCADE,
    role VARCHAR(20) NOT NULL DEFAULT 'realtor',  -- owner, manager, realtor

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    left_at TIMESTAMPTZ,

    UNIQUE(agency_id, user_id)
);

CREATE INDEX idx_reality_agency_members_agency ON reality_agency_members(agency_id);
CREATE INDEX idx_reality_agency_members_user ON reality_agency_members(user_id);
CREATE INDEX idx_reality_agency_members_active ON reality_agency_members(agency_id, is_active) WHERE is_active = true;

-- ============================================
-- Agency Invitations (Story 32.2)
-- ============================================

CREATE TABLE reality_agency_invitations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agency_id UUID NOT NULL REFERENCES reality_agencies(id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL,
    role VARCHAR(20) NOT NULL DEFAULT 'realtor',
    invited_by UUID NOT NULL REFERENCES portal_users(id),
    token VARCHAR(100) NOT NULL UNIQUE,
    message TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    accepted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_reality_agency_invitations_agency ON reality_agency_invitations(agency_id);
CREATE INDEX idx_reality_agency_invitations_email ON reality_agency_invitations(email);
CREATE INDEX idx_reality_agency_invitations_token ON reality_agency_invitations(token);

-- ============================================
-- Realtor Profiles (Story 33.1)
-- ============================================

CREATE TABLE realtor_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES portal_users(id) ON DELETE CASCADE UNIQUE,

    -- Profile
    photo_url VARCHAR(1000),
    bio TEXT,
    tagline VARCHAR(255),

    -- Professional info
    specializations TEXT[],  -- ['residential', 'commercial', 'luxury']
    experience_years INTEGER,
    languages TEXT[],  -- ['sk', 'en', 'de']

    -- Credentials
    license_number VARCHAR(100),
    license_verified_at TIMESTAMPTZ,
    certifications TEXT[],

    -- Contact (can override agency contact)
    phone VARCHAR(50),
    whatsapp VARCHAR(50),
    email_public VARCHAR(255),

    -- Social
    linkedin_url VARCHAR(500),
    facebook_url VARCHAR(500),
    instagram_url VARCHAR(500),

    -- Settings
    show_phone BOOLEAN NOT NULL DEFAULT true,
    show_email BOOLEAN NOT NULL DEFAULT true,
    accept_inquiries BOOLEAN NOT NULL DEFAULT true,

    -- Stats cache
    total_listings INTEGER NOT NULL DEFAULT 0,
    active_listings INTEGER NOT NULL DEFAULT 0,
    total_views INTEGER NOT NULL DEFAULT 0,
    total_inquiries INTEGER NOT NULL DEFAULT 0,
    avg_response_time_hours INTEGER,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_realtor_profiles_user ON realtor_profiles(user_id);
CREATE INDEX idx_realtor_profiles_license ON realtor_profiles(license_number) WHERE license_number IS NOT NULL;

-- ============================================
-- Realtor Listings (Story 33.2)
-- ============================================

CREATE TABLE realtor_listings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    listing_id UUID NOT NULL REFERENCES listings(id) ON DELETE CASCADE UNIQUE,
    realtor_id UUID NOT NULL REFERENCES portal_users(id),
    agency_id UUID REFERENCES reality_agencies(id) ON DELETE SET NULL,

    -- Visibility
    visibility VARCHAR(20) NOT NULL DEFAULT 'public',  -- public, agency, private
    show_realtor_info BOOLEAN NOT NULL DEFAULT true,
    show_agency_branding BOOLEAN NOT NULL DEFAULT true,

    -- Featured/promoted
    is_featured BOOLEAN NOT NULL DEFAULT false,
    featured_until TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_realtor_listings_realtor ON realtor_listings(realtor_id);
CREATE INDEX idx_realtor_listings_agency ON realtor_listings(agency_id);
CREATE INDEX idx_realtor_listings_featured ON realtor_listings(is_featured, featured_until) WHERE is_featured = true;

-- ============================================
-- Listing Inquiries (Story 33.3)
-- ============================================

CREATE TABLE listing_inquiries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    listing_id UUID NOT NULL REFERENCES listings(id) ON DELETE CASCADE,
    realtor_id UUID NOT NULL REFERENCES portal_users(id),

    -- Inquirer
    user_id UUID REFERENCES portal_users(id) ON DELETE SET NULL,  -- NULL if guest
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(50),

    -- Inquiry
    message TEXT NOT NULL,
    inquiry_type VARCHAR(20) NOT NULL DEFAULT 'info',  -- info, viewing, offer
    preferred_contact VARCHAR(20) NOT NULL DEFAULT 'email',  -- email, phone, whatsapp
    preferred_time VARCHAR(50),  -- e.g., "weekday evenings"

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'new',  -- new, read, responded, scheduled, closed
    read_at TIMESTAMPTZ,
    responded_at TIMESTAMPTZ,

    -- Metadata
    source VARCHAR(50),  -- website, mobile, portal_search
    utm_source VARCHAR(100),
    utm_medium VARCHAR(100),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_listing_inquiries_listing ON listing_inquiries(listing_id);
CREATE INDEX idx_listing_inquiries_realtor ON listing_inquiries(realtor_id);
CREATE INDEX idx_listing_inquiries_status ON listing_inquiries(realtor_id, status);
CREATE INDEX idx_listing_inquiries_created ON listing_inquiries(created_at DESC);

-- ============================================
-- Inquiry Messages (Thread)
-- ============================================

CREATE TABLE inquiry_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    inquiry_id UUID NOT NULL REFERENCES listing_inquiries(id) ON DELETE CASCADE,
    sender_type VARCHAR(20) NOT NULL,  -- user, realtor
    sender_id UUID NOT NULL,
    message TEXT NOT NULL,
    attachments JSONB,  -- [{url, filename, size}]
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_inquiry_messages_inquiry ON inquiry_messages(inquiry_id);

-- ============================================
-- Listing Analytics (Story 33.4)
-- ============================================

CREATE TABLE listing_analytics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    listing_id UUID NOT NULL REFERENCES listings(id) ON DELETE CASCADE,
    date DATE NOT NULL,

    -- Metrics
    views INTEGER NOT NULL DEFAULT 0,
    unique_views INTEGER NOT NULL DEFAULT 0,
    favorites_added INTEGER NOT NULL DEFAULT 0,
    favorites_removed INTEGER NOT NULL DEFAULT 0,
    inquiries INTEGER NOT NULL DEFAULT 0,
    phone_clicks INTEGER NOT NULL DEFAULT 0,
    share_clicks INTEGER NOT NULL DEFAULT 0,

    -- Source breakdown
    source_website INTEGER NOT NULL DEFAULT 0,
    source_mobile INTEGER NOT NULL DEFAULT 0,
    source_search INTEGER NOT NULL DEFAULT 0,
    source_direct INTEGER NOT NULL DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(listing_id, date)
);

CREATE INDEX idx_listing_analytics_listing ON listing_analytics(listing_id);
CREATE INDEX idx_listing_analytics_date ON listing_analytics(date DESC);

-- ============================================
-- CRM Connections (Story 34.2)
-- ============================================

CREATE TABLE crm_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agency_id UUID NOT NULL REFERENCES reality_agencies(id) ON DELETE CASCADE,

    -- CRM info
    crm_type VARCHAR(50) NOT NULL,  -- sreality, bezrealitky, reality_sk, custom_api
    name VARCHAR(255) NOT NULL,

    -- Credentials (encrypted at app level)
    api_endpoint VARCHAR(500),
    api_key_encrypted TEXT,
    oauth_client_id VARCHAR(255),
    oauth_client_secret_encrypted TEXT,
    oauth_refresh_token_encrypted TEXT,

    -- Field mapping
    field_mapping JSONB NOT NULL DEFAULT '{}',  -- {crm_field: our_field}

    -- Sync settings
    sync_enabled BOOLEAN NOT NULL DEFAULT true,
    sync_frequency VARCHAR(20) NOT NULL DEFAULT 'daily',  -- hourly, daily, realtime
    sync_direction VARCHAR(20) NOT NULL DEFAULT 'import',  -- import, export, bidirectional

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'pending',  -- pending, connected, error, disconnected
    last_sync_at TIMESTAMPTZ,
    last_error TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_crm_connections_agency ON crm_connections(agency_id);
CREATE INDEX idx_crm_connections_type ON crm_connections(crm_type);
CREATE INDEX idx_crm_connections_sync ON crm_connections(sync_enabled, sync_frequency) WHERE sync_enabled = true;

-- ============================================
-- Import Jobs (Story 34.1, 34.3, 34.4)
-- ============================================

CREATE TABLE portal_import_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agency_id UUID REFERENCES reality_agencies(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES portal_users(id),

    -- Source
    source_type VARCHAR(50) NOT NULL,  -- csv, excel, xml_feed, crm_sync
    source_url VARCHAR(1000),  -- For XML/RSS feeds
    source_filename VARCHAR(255),  -- For uploads
    crm_connection_id UUID REFERENCES crm_connections(id) ON DELETE SET NULL,

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'pending',  -- pending, validating, processing, completed, failed

    -- Progress
    total_records INTEGER NOT NULL DEFAULT 0,
    processed_records INTEGER NOT NULL DEFAULT 0,
    success_count INTEGER NOT NULL DEFAULT 0,
    skip_count INTEGER NOT NULL DEFAULT 0,
    failure_count INTEGER NOT NULL DEFAULT 0,

    -- Results
    created_listings UUID[],
    updated_listings UUID[],
    error_log JSONB,  -- [{row, field, error}]

    -- Timing
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_portal_import_jobs_agency ON portal_import_jobs(agency_id);
CREATE INDEX idx_portal_import_jobs_user ON portal_import_jobs(user_id);
CREATE INDEX idx_portal_import_jobs_status ON portal_import_jobs(status);

-- ============================================
-- Feed Subscriptions (Story 34.4)
-- ============================================

CREATE TABLE feed_subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agency_id UUID NOT NULL REFERENCES reality_agencies(id) ON DELETE CASCADE,

    -- Feed info
    name VARCHAR(255) NOT NULL,
    feed_url VARCHAR(1000) NOT NULL,
    feed_format VARCHAR(20) NOT NULL,  -- rss, atom, xml_custom

    -- Authentication
    auth_type VARCHAR(20),  -- none, basic, bearer, api_key
    auth_credentials_encrypted TEXT,

    -- Mapping
    field_mapping JSONB NOT NULL DEFAULT '{}',

    -- Schedule
    refresh_frequency VARCHAR(20) NOT NULL DEFAULT 'daily',  -- hourly, twice_daily, daily
    last_fetched_at TIMESTAMPTZ,
    next_fetch_at TIMESTAMPTZ,

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    last_error TEXT,
    consecutive_failures INTEGER NOT NULL DEFAULT 0,

    -- Stats
    total_imported INTEGER NOT NULL DEFAULT 0,
    last_import_count INTEGER NOT NULL DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_feed_subscriptions_agency ON feed_subscriptions(agency_id);
CREATE INDEX idx_feed_subscriptions_active ON feed_subscriptions(is_active, next_fetch_at) WHERE is_active = true;

-- ============================================
-- External Listing IDs (deduplication)
-- ============================================

CREATE TABLE external_listing_ids (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    listing_id UUID NOT NULL REFERENCES listings(id) ON DELETE CASCADE,
    source_type VARCHAR(50) NOT NULL,  -- crm:sreality, feed:my_rss, csv_import
    external_id VARCHAR(255) NOT NULL,
    source_url VARCHAR(1000),
    last_synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(source_type, external_id)
);

CREATE INDEX idx_external_listing_ids_listing ON external_listing_ids(listing_id);
CREATE INDEX idx_external_listing_ids_source ON external_listing_ids(source_type, external_id);

-- ============================================
-- Viewing Schedules (Story 33.3)
-- ============================================

CREATE TABLE viewing_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    inquiry_id UUID NOT NULL REFERENCES listing_inquiries(id) ON DELETE CASCADE,
    listing_id UUID NOT NULL REFERENCES listings(id) ON DELETE CASCADE,
    realtor_id UUID NOT NULL REFERENCES portal_users(id),

    -- Attendee
    attendee_name VARCHAR(255) NOT NULL,
    attendee_email VARCHAR(255) NOT NULL,
    attendee_phone VARCHAR(50),

    -- Schedule
    scheduled_at TIMESTAMPTZ NOT NULL,
    duration_minutes INTEGER NOT NULL DEFAULT 30,

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'scheduled',  -- scheduled, confirmed, completed, cancelled, no_show
    confirmed_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    cancellation_reason TEXT,

    -- Notes
    internal_notes TEXT,  -- For realtor
    meeting_notes TEXT,   -- Post-viewing notes

    -- Reminders
    reminder_sent_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_viewing_schedules_inquiry ON viewing_schedules(inquiry_id);
CREATE INDEX idx_viewing_schedules_listing ON viewing_schedules(listing_id);
CREATE INDEX idx_viewing_schedules_realtor ON viewing_schedules(realtor_id);
CREATE INDEX idx_viewing_schedules_scheduled ON viewing_schedules(scheduled_at);
CREATE INDEX idx_viewing_schedules_status ON viewing_schedules(status);

-- ============================================
-- Updated_at Triggers
-- ============================================

CREATE OR REPLACE FUNCTION update_portal_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_portal_users_updated_at
    BEFORE UPDATE ON portal_users
    FOR EACH ROW
    EXECUTE FUNCTION update_portal_updated_at();

CREATE TRIGGER trigger_portal_saved_searches_updated_at
    BEFORE UPDATE ON portal_saved_searches
    FOR EACH ROW
    EXECUTE FUNCTION update_portal_updated_at();

CREATE TRIGGER trigger_reality_agencies_updated_at
    BEFORE UPDATE ON reality_agencies
    FOR EACH ROW
    EXECUTE FUNCTION update_portal_updated_at();

CREATE TRIGGER trigger_realtor_profiles_updated_at
    BEFORE UPDATE ON realtor_profiles
    FOR EACH ROW
    EXECUTE FUNCTION update_portal_updated_at();

CREATE TRIGGER trigger_crm_connections_updated_at
    BEFORE UPDATE ON crm_connections
    FOR EACH ROW
    EXECUTE FUNCTION update_portal_updated_at();

CREATE TRIGGER trigger_feed_subscriptions_updated_at
    BEFORE UPDATE ON feed_subscriptions
    FOR EACH ROW
    EXECUTE FUNCTION update_portal_updated_at();

CREATE TRIGGER trigger_viewing_schedules_updated_at
    BEFORE UPDATE ON viewing_schedules
    FOR EACH ROW
    EXECUTE FUNCTION update_portal_updated_at();

-- ============================================
-- Helper Functions
-- ============================================

-- Generate unique agency slug
CREATE OR REPLACE FUNCTION generate_agency_slug(agency_name TEXT)
RETURNS TEXT AS $$
DECLARE
    base_slug TEXT;
    final_slug TEXT;
    counter INTEGER := 0;
BEGIN
    base_slug := lower(regexp_replace(agency_name, '[^a-zA-Z0-9]+', '-', 'g'));
    base_slug := trim(both '-' from base_slug);
    final_slug := base_slug;

    WHILE EXISTS (SELECT 1 FROM reality_agencies WHERE slug = final_slug) LOOP
        counter := counter + 1;
        final_slug := base_slug || '-' || counter;
    END LOOP;

    RETURN final_slug;
END;
$$ LANGUAGE plpgsql;

-- Get agency stats
CREATE OR REPLACE FUNCTION refresh_agency_stats(p_agency_id UUID)
RETURNS VOID AS $$
BEGIN
    UPDATE reality_agencies SET
        total_listings = (
            SELECT COUNT(*) FROM realtor_listings rl
            JOIN listings l ON l.id = rl.listing_id
            WHERE rl.agency_id = p_agency_id
        ),
        active_listings = (
            SELECT COUNT(*) FROM realtor_listings rl
            JOIN listings l ON l.id = rl.listing_id
            WHERE rl.agency_id = p_agency_id AND l.status = 'active'
        ),
        total_views = COALESCE((
            SELECT SUM(la.views) FROM listing_analytics la
            JOIN realtor_listings rl ON rl.listing_id = la.listing_id
            WHERE rl.agency_id = p_agency_id
        ), 0),
        total_inquiries = (
            SELECT COUNT(*) FROM listing_inquiries li
            JOIN realtor_listings rl ON rl.listing_id = li.listing_id
            WHERE rl.agency_id = p_agency_id
        )
    WHERE id = p_agency_id;
END;
$$ LANGUAGE plpgsql;

-- Get realtor stats
CREATE OR REPLACE FUNCTION refresh_realtor_stats(p_user_id UUID)
RETURNS VOID AS $$
BEGIN
    UPDATE realtor_profiles SET
        total_listings = (
            SELECT COUNT(*) FROM realtor_listings WHERE realtor_id = p_user_id
        ),
        active_listings = (
            SELECT COUNT(*) FROM realtor_listings rl
            JOIN listings l ON l.id = rl.listing_id
            WHERE rl.realtor_id = p_user_id AND l.status = 'active'
        ),
        total_views = COALESCE((
            SELECT SUM(la.views) FROM listing_analytics la
            JOIN realtor_listings rl ON rl.listing_id = la.listing_id
            WHERE rl.realtor_id = p_user_id
        ), 0),
        total_inquiries = (
            SELECT COUNT(*) FROM listing_inquiries WHERE realtor_id = p_user_id
        )
    WHERE user_id = p_user_id;
END;
$$ LANGUAGE plpgsql;

-- Track listing view
CREATE OR REPLACE FUNCTION track_listing_view(
    p_listing_id UUID,
    p_source VARCHAR DEFAULT 'website'
)
RETURNS VOID AS $$
BEGIN
    INSERT INTO listing_analytics (listing_id, date, views, source_website, source_mobile, source_search, source_direct)
    VALUES (
        p_listing_id,
        CURRENT_DATE,
        1,
        CASE WHEN p_source = 'website' THEN 1 ELSE 0 END,
        CASE WHEN p_source = 'mobile' THEN 1 ELSE 0 END,
        CASE WHEN p_source = 'search' THEN 1 ELSE 0 END,
        CASE WHEN p_source = 'direct' THEN 1 ELSE 0 END
    )
    ON CONFLICT (listing_id, date) DO UPDATE SET
        views = listing_analytics.views + 1,
        source_website = listing_analytics.source_website + CASE WHEN p_source = 'website' THEN 1 ELSE 0 END,
        source_mobile = listing_analytics.source_mobile + CASE WHEN p_source = 'mobile' THEN 1 ELSE 0 END,
        source_search = listing_analytics.source_search + CASE WHEN p_source = 'search' THEN 1 ELSE 0 END,
        source_direct = listing_analytics.source_direct + CASE WHEN p_source = 'direct' THEN 1 ELSE 0 END;
END;
$$ LANGUAGE plpgsql;
