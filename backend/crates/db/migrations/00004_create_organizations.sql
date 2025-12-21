-- Epic 2A, Story 2A.1: Organization Creation
-- Creates organizations table with Row-Level Security foundation

-- Enable RLS extension if not already enabled
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Organizations table
CREATE TABLE IF NOT EXISTS organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Basic info
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    contact_email VARCHAR(255) NOT NULL,

    -- Branding (Story 2A.4)
    logo_url VARCHAR(500),
    primary_color VARCHAR(7), -- Hex color e.g. #FF5733

    -- Settings (JSONB for extensibility)
    settings JSONB NOT NULL DEFAULT '{}',

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'suspended', 'deleted')),

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Organization status enum for Rust mapping
COMMENT ON COLUMN organizations.status IS 'active, suspended, deleted';

-- Indexes
CREATE INDEX IF NOT EXISTS idx_organizations_slug ON organizations(slug);
CREATE INDEX IF NOT EXISTS idx_organizations_status ON organizations(status);
CREATE INDEX IF NOT EXISTS idx_organizations_contact_email ON organizations(contact_email);

-- Trigger for updated_at
CREATE TRIGGER update_organizations_updated_at
    BEFORE UPDATE ON organizations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- RLS Policy: Organizations table itself is not tenant-scoped
-- (admins can see all, members can see their own org via membership)
-- We'll add RLS in Story 2A.2

-- Function to set tenant context for RLS
-- This will be called by middleware before each request
CREATE OR REPLACE FUNCTION set_tenant_context(org_id UUID)
RETURNS void AS $$
BEGIN
    PERFORM set_config('app.current_org_id', org_id::TEXT, TRUE);
END;
$$ LANGUAGE plpgsql;

-- Function to get current tenant context
CREATE OR REPLACE FUNCTION get_current_org_id()
RETURNS UUID AS $$
BEGIN
    RETURN NULLIF(current_setting('app.current_org_id', TRUE), '')::UUID;
EXCEPTION
    WHEN OTHERS THEN
        RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Function to check if current user is super admin (bypasses RLS)
CREATE OR REPLACE FUNCTION is_super_admin()
RETURNS BOOLEAN AS $$
BEGIN
    RETURN COALESCE(current_setting('app.is_super_admin', TRUE)::BOOLEAN, FALSE);
EXCEPTION
    WHEN OTHERS THEN
        RETURN FALSE;
END;
$$ LANGUAGE plpgsql;
