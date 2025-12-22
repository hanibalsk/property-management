-- Epic 17: Agency & Realtor Management
-- Migration: Create agencies and related tables

-- ============================================
-- Agencies (Story 17.1)
-- ============================================

CREATE TABLE agencies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    address VARCHAR(500),
    city VARCHAR(100),
    postal_code VARCHAR(20),
    country VARCHAR(3) NOT NULL DEFAULT 'SK',
    phone VARCHAR(50),
    email VARCHAR(255) NOT NULL,
    website VARCHAR(500),
    logo_url VARCHAR(1000),
    primary_color VARCHAR(7),  -- Hex color
    description TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',  -- pending, verified, suspended
    verified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for agencies
CREATE INDEX idx_agencies_slug ON agencies(slug);
CREATE INDEX idx_agencies_status ON agencies(status);
CREATE INDEX idx_agencies_city ON agencies(city);
CREATE INDEX idx_agencies_country ON agencies(country);

-- ============================================
-- Agency Members / Realtors (Story 17.2)
-- ============================================

CREATE TABLE agency_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agency_id UUID NOT NULL REFERENCES agencies(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(20) NOT NULL DEFAULT 'agent',  -- agent, senior, admin
    is_active BOOLEAN NOT NULL DEFAULT true,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    left_at TIMESTAMPTZ,

    UNIQUE(agency_id, user_id)
);

-- Indexes for agency_members
CREATE INDEX idx_agency_members_agency ON agency_members(agency_id);
CREATE INDEX idx_agency_members_user ON agency_members(user_id);
CREATE INDEX idx_agency_members_active ON agency_members(agency_id, is_active);

-- ============================================
-- Agency Invitations
-- ============================================

CREATE TABLE agency_invitations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agency_id UUID NOT NULL REFERENCES agencies(id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL,
    role VARCHAR(20) NOT NULL DEFAULT 'agent',
    invited_by UUID NOT NULL REFERENCES users(id),
    token VARCHAR(100) NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    accepted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for agency_invitations
CREATE INDEX idx_agency_invitations_agency ON agency_invitations(agency_id);
CREATE INDEX idx_agency_invitations_token ON agency_invitations(token);
CREATE INDEX idx_agency_invitations_email ON agency_invitations(email);

-- ============================================
-- Agency Listings (Story 17.3)
-- ============================================

CREATE TABLE agency_listings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    listing_id UUID NOT NULL REFERENCES listings(id) ON DELETE CASCADE,
    agency_id UUID NOT NULL REFERENCES agencies(id) ON DELETE CASCADE,
    realtor_id UUID NOT NULL REFERENCES users(id),
    visibility VARCHAR(20) NOT NULL DEFAULT 'agency',  -- personal, agency, public
    inquiry_assignment VARCHAR(20) NOT NULL DEFAULT 'pool',  -- pool, round_robin, creator
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(listing_id)
);

-- Indexes for agency_listings
CREATE INDEX idx_agency_listings_agency ON agency_listings(agency_id);
CREATE INDEX idx_agency_listings_realtor ON agency_listings(realtor_id);
CREATE INDEX idx_agency_listings_visibility ON agency_listings(visibility);

-- ============================================
-- Listing Edit History
-- ============================================

CREATE TABLE listing_edit_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    listing_id UUID NOT NULL REFERENCES listings(id) ON DELETE CASCADE,
    editor_id UUID NOT NULL REFERENCES users(id),
    editor_name VARCHAR(255) NOT NULL,
    field_changed VARCHAR(100) NOT NULL,
    old_value TEXT,
    new_value TEXT,
    edited_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for listing_edit_history
CREATE INDEX idx_listing_edit_history_listing ON listing_edit_history(listing_id);
CREATE INDEX idx_listing_edit_history_editor ON listing_edit_history(editor_id);
CREATE INDEX idx_listing_edit_history_date ON listing_edit_history(edited_at DESC);

-- ============================================
-- Import Jobs (Story 17.4)
-- ============================================

CREATE TABLE listing_import_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agency_id UUID NOT NULL REFERENCES agencies(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id),
    source VARCHAR(50) NOT NULL,  -- csv, xml_sreality, api
    status VARCHAR(20) NOT NULL DEFAULT 'pending',  -- pending, processing, completed, failed
    total_records INTEGER NOT NULL DEFAULT 0,
    processed_records INTEGER NOT NULL DEFAULT 0,
    success_count INTEGER NOT NULL DEFAULT 0,
    failure_count INTEGER NOT NULL DEFAULT 0,
    error_log TEXT,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for listing_import_jobs
CREATE INDEX idx_listing_import_jobs_agency ON listing_import_jobs(agency_id);
CREATE INDEX idx_listing_import_jobs_user ON listing_import_jobs(user_id);
CREATE INDEX idx_listing_import_jobs_status ON listing_import_jobs(status);

-- ============================================
-- Import Field Mappings
-- ============================================

CREATE TABLE import_field_mappings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    import_job_id UUID NOT NULL REFERENCES listing_import_jobs(id) ON DELETE CASCADE,
    source_field VARCHAR(100) NOT NULL,
    target_field VARCHAR(100) NOT NULL
);

-- Index for import_field_mappings
CREATE INDEX idx_import_field_mappings_job ON import_field_mappings(import_job_id);

-- ============================================
-- Updated_at trigger for agencies
-- ============================================

CREATE OR REPLACE FUNCTION update_agencies_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER trigger_agencies_updated_at
    BEFORE UPDATE ON agencies
    FOR EACH ROW
    EXECUTE FUNCTION update_agencies_updated_at();
