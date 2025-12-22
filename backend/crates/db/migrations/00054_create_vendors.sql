-- Epic 21: Supplier & Vendor Management
-- Migration: 00054_create_vendors.sql

-- ============================================
-- Story 21.1: Vendor Registry
-- ============================================

-- Vendors table
CREATE TABLE IF NOT EXISTS vendors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Company information
    company_name VARCHAR(255) NOT NULL,
    contact_name VARCHAR(255),
    phone VARCHAR(50),
    email VARCHAR(255),
    website VARCHAR(500),
    address TEXT,

    -- Business details
    services TEXT[] NOT NULL DEFAULT '{}', -- plumbing, electrical, HVAC, cleaning, landscaping, security, other
    license_number VARCHAR(100),
    tax_id VARCHAR(50),

    -- Contract information
    contract_start DATE,
    contract_end DATE,
    hourly_rate DECIMAL(10, 2),

    -- Performance tracking
    rating DECIMAL(3, 2) CHECK (rating >= 0 AND rating <= 5),
    total_jobs INTEGER DEFAULT 0,
    completed_jobs INTEGER DEFAULT 0,

    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    is_preferred BOOLEAN DEFAULT FALSE,
    notes TEXT,

    -- Metadata
    metadata JSONB DEFAULT '{}',

    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    -- Constraints
    CONSTRAINT valid_vendor_status CHECK (status IN ('active', 'inactive', 'suspended', 'pending_approval'))
);

-- Indexes
CREATE INDEX idx_vendors_organization ON vendors(organization_id);
CREATE INDEX idx_vendors_status ON vendors(status);
CREATE INDEX idx_vendors_services ON vendors USING GIN(services);
CREATE INDEX idx_vendors_contract_end ON vendors(contract_end) WHERE contract_end IS NOT NULL;
CREATE INDEX idx_vendors_is_preferred ON vendors(organization_id, is_preferred) WHERE is_preferred = TRUE;

-- ============================================
-- Vendor Contacts (multiple contacts per vendor)
-- ============================================

CREATE TABLE IF NOT EXISTS vendor_contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vendor_id UUID NOT NULL REFERENCES vendors(id) ON DELETE CASCADE,

    name VARCHAR(255) NOT NULL,
    role VARCHAR(100),
    phone VARCHAR(50),
    email VARCHAR(255),
    is_primary BOOLEAN DEFAULT FALSE,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_vendor_contacts_vendor ON vendor_contacts(vendor_id);
CREATE INDEX idx_vendor_contacts_primary ON vendor_contacts(vendor_id, is_primary) WHERE is_primary = TRUE;

-- ============================================
-- Story 21.3: Contract Management
-- ============================================

CREATE TABLE IF NOT EXISTS vendor_contracts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vendor_id UUID NOT NULL REFERENCES vendors(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Contract details
    contract_number VARCHAR(100),
    title VARCHAR(255) NOT NULL,
    description TEXT,

    -- Dates
    start_date DATE NOT NULL,
    end_date DATE,
    renewal_date DATE,

    -- Financial terms
    contract_value DECIMAL(15, 2),
    payment_terms VARCHAR(255), -- Net 30, Net 60, etc.

    -- Contract type
    contract_type VARCHAR(50) NOT NULL DEFAULT 'service',

    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'draft',
    auto_renew BOOLEAN DEFAULT FALSE,

    -- Metadata
    terms JSONB DEFAULT '{}', -- Additional terms and conditions
    metadata JSONB DEFAULT '{}',

    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    signed_at TIMESTAMP WITH TIME ZONE,

    -- Constraints
    CONSTRAINT valid_contract_type CHECK (contract_type IN ('service', 'maintenance', 'project', 'retainer', 'one_time')),
    CONSTRAINT valid_contract_status CHECK (status IN ('draft', 'pending_approval', 'active', 'expired', 'terminated', 'renewed'))
);

CREATE INDEX idx_vendor_contracts_vendor ON vendor_contracts(vendor_id);
CREATE INDEX idx_vendor_contracts_organization ON vendor_contracts(organization_id);
CREATE INDEX idx_vendor_contracts_status ON vendor_contracts(status);
CREATE INDEX idx_vendor_contracts_end_date ON vendor_contracts(end_date) WHERE status = 'active';

-- Contract documents junction table
CREATE TABLE IF NOT EXISTS vendor_contract_documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    contract_id UUID NOT NULL REFERENCES vendor_contracts(id) ON DELETE CASCADE,
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,

    document_type VARCHAR(50) NOT NULL DEFAULT 'contract', -- contract, amendment, attachment, certificate

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    UNIQUE(contract_id, document_id)
);

CREATE INDEX idx_contract_documents_contract ON vendor_contract_documents(contract_id);

-- ============================================
-- Story 21.4: Vendor Invoice Processing
-- ============================================

CREATE TABLE IF NOT EXISTS vendor_invoices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    vendor_id UUID NOT NULL REFERENCES vendors(id) ON DELETE CASCADE,
    contract_id UUID REFERENCES vendor_contracts(id) ON DELETE SET NULL,

    -- Invoice details
    invoice_number VARCHAR(100) NOT NULL,
    invoice_date DATE NOT NULL,
    due_date DATE,

    -- Amounts
    subtotal DECIMAL(15, 2) NOT NULL,
    tax_amount DECIMAL(15, 2) DEFAULT 0,
    total_amount DECIMAL(15, 2) NOT NULL,
    paid_amount DECIMAL(15, 2) DEFAULT 0,

    -- Currency
    currency VARCHAR(3) DEFAULT 'EUR',

    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'pending',

    -- Related work orders
    work_order_ids UUID[] DEFAULT '{}',

    -- Payment info
    payment_method VARCHAR(50),
    payment_reference VARCHAR(255),
    paid_at TIMESTAMP WITH TIME ZONE,

    -- Description
    description TEXT,
    line_items JSONB DEFAULT '[]', -- Array of {description, quantity, unit_price, amount}

    -- Approval workflow
    submitted_by UUID REFERENCES users(id) ON DELETE SET NULL,
    approved_by UUID REFERENCES users(id) ON DELETE SET NULL,
    approved_at TIMESTAMP WITH TIME ZONE,
    rejected_by UUID REFERENCES users(id) ON DELETE SET NULL,
    rejection_reason TEXT,

    -- Metadata
    metadata JSONB DEFAULT '{}',

    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    -- Constraints
    CONSTRAINT valid_invoice_status CHECK (status IN ('draft', 'pending', 'approved', 'rejected', 'partially_paid', 'paid', 'cancelled', 'overdue')),
    CONSTRAINT valid_invoice_amounts CHECK (total_amount >= 0 AND paid_amount >= 0 AND paid_amount <= total_amount)
);

CREATE INDEX idx_vendor_invoices_organization ON vendor_invoices(organization_id);
CREATE INDEX idx_vendor_invoices_vendor ON vendor_invoices(vendor_id);
CREATE INDEX idx_vendor_invoices_status ON vendor_invoices(status);
CREATE INDEX idx_vendor_invoices_due_date ON vendor_invoices(due_date) WHERE status IN ('pending', 'approved');
CREATE INDEX idx_vendor_invoices_invoice_number ON vendor_invoices(organization_id, invoice_number);
CREATE INDEX idx_vendor_invoices_work_orders ON vendor_invoices USING GIN(work_order_ids);

-- ============================================
-- Vendor Ratings and Reviews
-- ============================================

CREATE TABLE IF NOT EXISTS vendor_ratings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vendor_id UUID NOT NULL REFERENCES vendors(id) ON DELETE CASCADE,
    work_order_id UUID REFERENCES work_orders(id) ON DELETE SET NULL,
    rated_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Rating details
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    quality_rating INTEGER CHECK (quality_rating >= 1 AND quality_rating <= 5),
    timeliness_rating INTEGER CHECK (timeliness_rating >= 1 AND timeliness_rating <= 5),
    communication_rating INTEGER CHECK (communication_rating >= 1 AND communication_rating <= 5),
    value_rating INTEGER CHECK (value_rating >= 1 AND value_rating <= 5),

    -- Review
    review_text TEXT,

    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_vendor_ratings_vendor ON vendor_ratings(vendor_id);
CREATE INDEX idx_vendor_ratings_work_order ON vendor_ratings(work_order_id);

-- ============================================
-- Vendor Service Areas
-- ============================================

CREATE TABLE IF NOT EXISTS vendor_service_areas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vendor_id UUID NOT NULL REFERENCES vendors(id) ON DELETE CASCADE,

    -- Location
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,
    city VARCHAR(255),
    postal_code VARCHAR(20),
    region VARCHAR(255),

    -- Coverage details
    is_primary_area BOOLEAN DEFAULT FALSE,
    travel_fee DECIMAL(10, 2),

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    -- Either building_id or location info should be set
    CONSTRAINT vendor_area_has_location CHECK (
        building_id IS NOT NULL OR city IS NOT NULL OR postal_code IS NOT NULL OR region IS NOT NULL
    )
);

CREATE INDEX idx_vendor_service_areas_vendor ON vendor_service_areas(vendor_id);
CREATE INDEX idx_vendor_service_areas_building ON vendor_service_areas(building_id);

-- ============================================
-- RLS Policies
-- ============================================

-- Enable RLS
ALTER TABLE vendors ENABLE ROW LEVEL SECURITY;
ALTER TABLE vendor_contacts ENABLE ROW LEVEL SECURITY;
ALTER TABLE vendor_contracts ENABLE ROW LEVEL SECURITY;
ALTER TABLE vendor_contract_documents ENABLE ROW LEVEL SECURITY;
ALTER TABLE vendor_invoices ENABLE ROW LEVEL SECURITY;
ALTER TABLE vendor_ratings ENABLE ROW LEVEL SECURITY;
ALTER TABLE vendor_service_areas ENABLE ROW LEVEL SECURITY;

-- Vendors policies
CREATE POLICY vendors_org_isolation ON vendors
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', true)::UUID);

-- Vendor contacts policies
CREATE POLICY vendor_contacts_org_isolation ON vendor_contacts
    FOR ALL USING (
        vendor_id IN (
            SELECT id FROM vendors WHERE organization_id = current_setting('app.current_organization_id', true)::UUID
        )
    );

-- Vendor contracts policies
CREATE POLICY vendor_contracts_org_isolation ON vendor_contracts
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', true)::UUID);

-- Contract documents policies
CREATE POLICY vendor_contract_documents_org_isolation ON vendor_contract_documents
    FOR ALL USING (
        contract_id IN (
            SELECT id FROM vendor_contracts WHERE organization_id = current_setting('app.current_organization_id', true)::UUID
        )
    );

-- Vendor invoices policies
CREATE POLICY vendor_invoices_org_isolation ON vendor_invoices
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', true)::UUID);

-- Vendor ratings policies
CREATE POLICY vendor_ratings_org_isolation ON vendor_ratings
    FOR ALL USING (
        vendor_id IN (
            SELECT id FROM vendors WHERE organization_id = current_setting('app.current_organization_id', true)::UUID
        )
    );

-- Vendor service areas policies
CREATE POLICY vendor_service_areas_org_isolation ON vendor_service_areas
    FOR ALL USING (
        vendor_id IN (
            SELECT id FROM vendors WHERE organization_id = current_setting('app.current_organization_id', true)::UUID
        )
    );

-- ============================================
-- Triggers for updated_at
-- ============================================

CREATE TRIGGER update_vendors_updated_at
    BEFORE UPDATE ON vendors
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_vendor_contacts_updated_at
    BEFORE UPDATE ON vendor_contacts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_vendor_contracts_updated_at
    BEFORE UPDATE ON vendor_contracts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_vendor_invoices_updated_at
    BEFORE UPDATE ON vendor_invoices
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_vendor_ratings_updated_at
    BEFORE UPDATE ON vendor_ratings
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================
-- Function to update vendor rating
-- ============================================

CREATE OR REPLACE FUNCTION update_vendor_rating()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE vendors
    SET rating = (
        SELECT ROUND(AVG(rating)::numeric, 2)
        FROM vendor_ratings
        WHERE vendor_id = COALESCE(NEW.vendor_id, OLD.vendor_id)
    ),
    updated_at = NOW()
    WHERE id = COALESCE(NEW.vendor_id, OLD.vendor_id);

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_vendor_rating_after_rating
    AFTER INSERT OR UPDATE OR DELETE ON vendor_ratings
    FOR EACH ROW
    EXECUTE FUNCTION update_vendor_rating();

-- ============================================
-- Function to update vendor job counts
-- ============================================

CREATE OR REPLACE FUNCTION update_vendor_job_counts()
RETURNS TRIGGER AS $$
DECLARE
    v_vendor_id UUID;
BEGIN
    -- Get vendor_id from the work order
    IF TG_OP = 'DELETE' THEN
        v_vendor_id := OLD.vendor_id;
    ELSE
        v_vendor_id := NEW.vendor_id;
    END IF;

    -- Skip if no vendor assigned
    IF v_vendor_id IS NULL THEN
        RETURN COALESCE(NEW, OLD);
    END IF;

    -- Update vendor job counts
    UPDATE vendors
    SET
        total_jobs = (
            SELECT COUNT(*) FROM work_orders WHERE vendor_id = v_vendor_id
        ),
        completed_jobs = (
            SELECT COUNT(*) FROM work_orders WHERE vendor_id = v_vendor_id AND status = 'completed'
        ),
        updated_at = NOW()
    WHERE id = v_vendor_id;

    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

-- Note: This trigger should be created after ensuring work_orders table exists with vendor_id
-- CREATE TRIGGER update_vendor_counts_on_work_order
--     AFTER INSERT OR UPDATE OR DELETE ON work_orders
--     FOR EACH ROW
--     EXECUTE FUNCTION update_vendor_job_counts();
