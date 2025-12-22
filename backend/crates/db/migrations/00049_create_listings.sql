-- Epic 15: Property Listings & Multi-Portal Sync
-- Migration: Create listings and related tables

-- ============================================
-- Listings (Story 15.1)
-- ============================================

CREATE TABLE listings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    unit_id UUID REFERENCES units(id) ON DELETE SET NULL,
    created_by UUID NOT NULL REFERENCES users(id),

    -- Listing status
    status VARCHAR(20) NOT NULL DEFAULT 'draft',  -- draft, active, paused, sold, rented, archived
    transaction_type VARCHAR(10) NOT NULL,  -- sale, rent

    -- Property details
    title VARCHAR(255) NOT NULL,
    description TEXT,
    property_type VARCHAR(20) NOT NULL DEFAULT 'apartment',  -- apartment, house, commercial, land, parking, storage, other
    size_sqm DECIMAL(10, 2),
    rooms INTEGER,
    bathrooms INTEGER,
    floor INTEGER,
    total_floors INTEGER,

    -- Address
    street VARCHAR(255) NOT NULL,
    city VARCHAR(100) NOT NULL,
    postal_code VARCHAR(20) NOT NULL,
    country VARCHAR(3) NOT NULL DEFAULT 'SK',

    -- Location coordinates
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),

    -- Pricing
    price DECIMAL(15, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',  -- EUR, CZK
    is_negotiable BOOLEAN NOT NULL DEFAULT false,

    -- Features (JSON array)
    features JSONB NOT NULL DEFAULT '[]',

    -- Timestamps
    published_at TIMESTAMPTZ,
    sold_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for listings
CREATE INDEX idx_listings_organization ON listings(organization_id);
CREATE INDEX idx_listings_unit ON listings(unit_id);
CREATE INDEX idx_listings_status ON listings(status);
CREATE INDEX idx_listings_transaction_type ON listings(transaction_type);
CREATE INDEX idx_listings_property_type ON listings(property_type);
CREATE INDEX idx_listings_city ON listings(city);
CREATE INDEX idx_listings_country ON listings(country);
CREATE INDEX idx_listings_price ON listings(price);
CREATE INDEX idx_listings_created_at ON listings(created_at DESC);
CREATE INDEX idx_listings_published_at ON listings(published_at DESC);

-- ============================================
-- Listing Photos (Story 15.2)
-- ============================================

CREATE TABLE listing_photos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    listing_id UUID NOT NULL REFERENCES listings(id) ON DELETE CASCADE,
    url VARCHAR(1000) NOT NULL,
    thumbnail_url VARCHAR(1000),
    medium_url VARCHAR(1000),
    display_order INTEGER NOT NULL DEFAULT 0,
    alt_text VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for listing_photos
CREATE INDEX idx_listing_photos_listing ON listing_photos(listing_id);
CREATE INDEX idx_listing_photos_order ON listing_photos(listing_id, display_order);

-- ============================================
-- Listing Syndications (Story 15.3 - Multi-Portal)
-- ============================================

CREATE TABLE listing_syndications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    listing_id UUID NOT NULL REFERENCES listings(id) ON DELETE CASCADE,
    portal VARCHAR(50) NOT NULL,  -- reality_portal, sreality, bezrealitky, nehnutelnosti
    external_id VARCHAR(255),     -- ID on the external portal
    status VARCHAR(20) NOT NULL DEFAULT 'pending',  -- pending, synced, failed, removed
    last_error TEXT,
    synced_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(listing_id, portal)
);

-- Indexes for listing_syndications
CREATE INDEX idx_listing_syndications_listing ON listing_syndications(listing_id);
CREATE INDEX idx_listing_syndications_portal ON listing_syndications(portal);
CREATE INDEX idx_listing_syndications_status ON listing_syndications(status);

-- ============================================
-- Updated_at trigger for listings
-- ============================================

CREATE OR REPLACE FUNCTION update_listings_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER trigger_listings_updated_at
    BEFORE UPDATE ON listings
    FOR EACH ROW
    EXECUTE FUNCTION update_listings_updated_at();

CREATE TRIGGER trigger_listing_syndications_updated_at
    BEFORE UPDATE ON listing_syndications
    FOR EACH ROW
    EXECUTE FUNCTION update_listings_updated_at();
