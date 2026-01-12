-- Epic 137: Smart Building Certification
-- Tracks building certifications like LEED, BREEAM, WELL, Fitwel, Energy Star

-- Certification Programs Enum
CREATE TYPE certification_program AS ENUM (
    'leed',           -- Leadership in Energy and Environmental Design
    'breeam',         -- Building Research Establishment Environmental Assessment Method
    'well',           -- WELL Building Standard
    'fitwel',         -- Fitwel Certification
    'energy_star',    -- EPA Energy Star
    'green_globes',   -- Green Globes Certification
    'living_building_challenge',
    'passive_house',
    'nabers',         -- National Australian Built Environment Rating System
    'dgnb',           -- Deutsche Gesellschaft für Nachhaltiges Bauen
    'hqe',            -- Haute Qualité Environnementale
    'casbee',         -- Comprehensive Assessment System for Built Environment Efficiency
    'edge',           -- Excellence in Design for Greater Efficiencies
    'boma_best',      -- Building Owners and Managers Association BEST
    'other'
);

-- Certification Level Enum
CREATE TYPE certification_level AS ENUM (
    'certified',
    'silver',
    'gold',
    'platinum',
    'one_star',
    'two_star',
    'three_star',
    'four_star',
    'five_star',
    'pending',
    'expired',
    'other'
);

-- Certification Status Enum
CREATE TYPE certification_status AS ENUM (
    'planning',
    'in_progress',
    'under_review',
    'achieved',
    'renewed',
    'expired',
    'revoked'
);

-- Credit Category Type
CREATE TYPE credit_category_type AS ENUM (
    'energy_efficiency',
    'water_efficiency',
    'materials_resources',
    'indoor_environmental_quality',
    'sustainable_sites',
    'location_transportation',
    'innovation',
    'regional_priority',
    'wellness',
    'air_quality',
    'light',
    'thermal_comfort',
    'sound',
    'materials',
    'water',
    'nourishment',
    'movement',
    'community',
    'mind',
    'other'
);

-- Building Certifications Table
CREATE TABLE building_certifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,

    -- Certification Details
    program certification_program NOT NULL,
    version VARCHAR(50),                           -- e.g., "LEED v4.1", "WELL v2"
    level certification_level NOT NULL,
    status certification_status NOT NULL DEFAULT 'planning',

    -- Scores
    total_points_possible INTEGER,
    total_points_achieved INTEGER,
    percentage_achieved DECIMAL(5,2),

    -- Dates
    application_date DATE,
    certification_date DATE,
    expiration_date DATE,
    renewal_date DATE,

    -- Reference Information
    certificate_number VARCHAR(100),
    project_id VARCHAR(100),                       -- Program-specific project ID
    assessor_name VARCHAR(200),
    assessor_organization VARCHAR(200),

    -- Documentation
    certificate_url TEXT,
    scorecard_url TEXT,
    notes TEXT,

    -- Cost Tracking
    application_fee DECIMAL(10,2),
    certification_fee DECIMAL(10,2),
    annual_fee DECIMAL(10,2),
    total_investment DECIMAL(12,2),

    -- Metadata
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Certification Credits Table (Individual credit tracking)
CREATE TABLE certification_credits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    certification_id UUID NOT NULL REFERENCES building_certifications(id) ON DELETE CASCADE,

    -- Credit Details
    category credit_category_type NOT NULL,
    credit_code VARCHAR(50),                       -- e.g., "EA1", "WE2", "EQ3"
    credit_name VARCHAR(300) NOT NULL,
    description TEXT,

    -- Points
    points_possible INTEGER NOT NULL DEFAULT 1,
    points_achieved INTEGER NOT NULL DEFAULT 0,
    is_prerequisite BOOLEAN DEFAULT FALSE,

    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'not_started', -- not_started, in_progress, pending_review, achieved, denied
    documentation_status VARCHAR(50) DEFAULT 'missing', -- missing, partial, complete, submitted

    -- Documentation
    evidence_urls JSONB DEFAULT '[]',
    notes TEXT,

    -- Responsible Party
    responsible_user_id UUID REFERENCES users(id),
    due_date DATE,

    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Certification Documents Table
CREATE TABLE certification_documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    certification_id UUID NOT NULL REFERENCES building_certifications(id) ON DELETE CASCADE,
    credit_id UUID REFERENCES certification_credits(id) ON DELETE SET NULL,

    -- Document Details
    document_type VARCHAR(100) NOT NULL,           -- e.g., "energy_model", "commissioning_report", "floor_plan"
    title VARCHAR(300) NOT NULL,
    description TEXT,
    file_url TEXT NOT NULL,
    file_size_bytes BIGINT,
    file_type VARCHAR(50),

    -- Version Control
    version INTEGER DEFAULT 1,
    is_current BOOLEAN DEFAULT TRUE,
    supersedes_id UUID REFERENCES certification_documents(id),

    -- Submission Status
    submitted_date TIMESTAMPTZ,
    review_status VARCHAR(50) DEFAULT 'draft',     -- draft, submitted, under_review, approved, rejected
    reviewer_comments TEXT,

    -- Metadata
    uploaded_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Certification Milestones Table
CREATE TABLE certification_milestones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    certification_id UUID NOT NULL REFERENCES building_certifications(id) ON DELETE CASCADE,

    -- Milestone Details
    milestone_name VARCHAR(200) NOT NULL,
    description TEXT,
    phase VARCHAR(100),                            -- e.g., "design", "construction", "operations"

    -- Timeline
    target_date DATE,
    actual_date DATE,
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, in_progress, completed, skipped

    -- Dependencies
    depends_on_milestone_id UUID REFERENCES certification_milestones(id),

    -- Assignments
    assigned_to UUID REFERENCES users(id),
    notes TEXT,

    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Certification Benchmarks Table (Comparison with similar buildings)
CREATE TABLE certification_benchmarks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    certification_id UUID NOT NULL REFERENCES building_certifications(id) ON DELETE CASCADE,

    -- Benchmark Details
    metric_name VARCHAR(200) NOT NULL,             -- e.g., "Energy Use Intensity", "Water Use per sqft"
    metric_unit VARCHAR(50),                       -- e.g., "kBtu/sqft", "gal/sqft"
    building_value DECIMAL(12,4),
    benchmark_25th_percentile DECIMAL(12,4),
    benchmark_50th_percentile DECIMAL(12,4),
    benchmark_75th_percentile DECIMAL(12,4),
    benchmark_source VARCHAR(200),                 -- e.g., "CBECS 2018", "Energy Star Portfolio Manager"

    -- Percentile Ranking
    percentile_rank INTEGER,                       -- 0-100

    -- Period
    measurement_period_start DATE,
    measurement_period_end DATE,

    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Certification Audit Logs
CREATE TABLE certification_audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    certification_id UUID NOT NULL REFERENCES building_certifications(id) ON DELETE CASCADE,

    -- Audit Details
    action VARCHAR(100) NOT NULL,                  -- e.g., "status_change", "credit_updated", "document_uploaded"
    entity_type VARCHAR(100),                      -- e.g., "certification", "credit", "document"
    entity_id UUID,
    previous_value JSONB,
    new_value JSONB,
    notes TEXT,

    -- Actor
    performed_by UUID REFERENCES users(id),
    performed_at TIMESTAMPTZ DEFAULT NOW()
);

-- Certification Cost Tracking
CREATE TABLE certification_costs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    certification_id UUID NOT NULL REFERENCES building_certifications(id) ON DELETE CASCADE,

    -- Cost Details
    cost_type VARCHAR(100) NOT NULL,               -- e.g., "registration_fee", "consulting", "equipment", "testing"
    description VARCHAR(300),
    amount DECIMAL(12,2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'USD',

    -- Date
    incurred_date DATE,
    paid_date DATE,
    invoice_number VARCHAR(100),

    -- Vendor
    vendor_name VARCHAR(200),
    vendor_id UUID REFERENCES vendors(id),

    -- Metadata
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Certification Reminder Configuration
CREATE TABLE certification_reminders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    certification_id UUID NOT NULL REFERENCES building_certifications(id) ON DELETE CASCADE,

    -- Reminder Details
    reminder_type VARCHAR(100) NOT NULL,           -- e.g., "expiration", "renewal", "milestone"
    days_before INTEGER NOT NULL,
    message TEXT,

    -- Recipients
    notify_users JSONB DEFAULT '[]',               -- Array of user IDs
    notify_roles JSONB DEFAULT '[]',               -- Array of role names

    -- Status
    is_active BOOLEAN DEFAULT TRUE,
    last_sent_at TIMESTAMPTZ,

    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_building_certifications_org ON building_certifications(organization_id);
CREATE INDEX idx_building_certifications_building ON building_certifications(building_id);
CREATE INDEX idx_building_certifications_program ON building_certifications(program);
CREATE INDEX idx_building_certifications_status ON building_certifications(status);
CREATE INDEX idx_building_certifications_expiration ON building_certifications(expiration_date);

CREATE INDEX idx_certification_credits_org ON certification_credits(organization_id);
CREATE INDEX idx_certification_credits_cert ON certification_credits(certification_id);
CREATE INDEX idx_certification_credits_category ON certification_credits(category);
CREATE INDEX idx_certification_credits_status ON certification_credits(status);

CREATE INDEX idx_certification_documents_org ON certification_documents(organization_id);
CREATE INDEX idx_certification_documents_cert ON certification_documents(certification_id);
CREATE INDEX idx_certification_documents_credit ON certification_documents(credit_id);

CREATE INDEX idx_certification_milestones_org ON certification_milestones(organization_id);
CREATE INDEX idx_certification_milestones_cert ON certification_milestones(certification_id);
CREATE INDEX idx_certification_milestones_status ON certification_milestones(status);

CREATE INDEX idx_certification_benchmarks_org ON certification_benchmarks(organization_id);
CREATE INDEX idx_certification_benchmarks_cert ON certification_benchmarks(certification_id);

CREATE INDEX idx_certification_audit_logs_org ON certification_audit_logs(organization_id);
CREATE INDEX idx_certification_audit_logs_cert ON certification_audit_logs(certification_id);
CREATE INDEX idx_certification_audit_logs_action ON certification_audit_logs(action);

CREATE INDEX idx_certification_costs_org ON certification_costs(organization_id);
CREATE INDEX idx_certification_costs_cert ON certification_costs(certification_id);

CREATE INDEX idx_certification_reminders_org ON certification_reminders(organization_id);
CREATE INDEX idx_certification_reminders_cert ON certification_reminders(certification_id);

-- RLS Policies
ALTER TABLE building_certifications ENABLE ROW LEVEL SECURITY;
ALTER TABLE certification_credits ENABLE ROW LEVEL SECURITY;
ALTER TABLE certification_documents ENABLE ROW LEVEL SECURITY;
ALTER TABLE certification_milestones ENABLE ROW LEVEL SECURITY;
ALTER TABLE certification_benchmarks ENABLE ROW LEVEL SECURITY;
ALTER TABLE certification_audit_logs ENABLE ROW LEVEL SECURITY;
ALTER TABLE certification_costs ENABLE ROW LEVEL SECURITY;
ALTER TABLE certification_reminders ENABLE ROW LEVEL SECURITY;

CREATE POLICY building_certifications_org_policy ON building_certifications
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', TRUE)::UUID);

CREATE POLICY certification_credits_org_policy ON certification_credits
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', TRUE)::UUID);

CREATE POLICY certification_documents_org_policy ON certification_documents
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', TRUE)::UUID);

CREATE POLICY certification_milestones_org_policy ON certification_milestones
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', TRUE)::UUID);

CREATE POLICY certification_benchmarks_org_policy ON certification_benchmarks
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', TRUE)::UUID);

CREATE POLICY certification_audit_logs_org_policy ON certification_audit_logs
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', TRUE)::UUID);

CREATE POLICY certification_costs_org_policy ON certification_costs
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', TRUE)::UUID);

CREATE POLICY certification_reminders_org_policy ON certification_reminders
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', TRUE)::UUID);

-- Trigger for updated_at
CREATE OR REPLACE FUNCTION update_building_certifications_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER building_certifications_updated_at
    BEFORE UPDATE ON building_certifications
    FOR EACH ROW EXECUTE FUNCTION update_building_certifications_updated_at();

CREATE TRIGGER certification_credits_updated_at
    BEFORE UPDATE ON certification_credits
    FOR EACH ROW EXECUTE FUNCTION update_building_certifications_updated_at();

CREATE TRIGGER certification_documents_updated_at
    BEFORE UPDATE ON certification_documents
    FOR EACH ROW EXECUTE FUNCTION update_building_certifications_updated_at();

CREATE TRIGGER certification_milestones_updated_at
    BEFORE UPDATE ON certification_milestones
    FOR EACH ROW EXECUTE FUNCTION update_building_certifications_updated_at();

CREATE TRIGGER certification_benchmarks_updated_at
    BEFORE UPDATE ON certification_benchmarks
    FOR EACH ROW EXECUTE FUNCTION update_building_certifications_updated_at();

CREATE TRIGGER certification_reminders_updated_at
    BEFORE UPDATE ON certification_reminders
    FOR EACH ROW EXECUTE FUNCTION update_building_certifications_updated_at();
