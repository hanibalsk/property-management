-- Epic 23: Emergency Management
-- Stories: 23.1 through 23.6 (Emergency Protocols, Contacts, Incident Reporting, Broadcast, Drills, Statistics)

-- Emergency Protocols
-- Defines emergency procedures and response plans per organization/building
CREATE TABLE IF NOT EXISTS emergency_protocols (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    protocol_type VARCHAR(50) NOT NULL DEFAULT 'other',
    description TEXT,
    steps JSONB NOT NULL DEFAULT '[]',
    contacts JSONB NOT NULL DEFAULT '[]',
    evacuation_info TEXT,
    attachments JSONB DEFAULT '[]',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    priority INTEGER NOT NULL DEFAULT 0,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE emergency_protocols IS 'Emergency protocols and response procedures';
COMMENT ON COLUMN emergency_protocols.protocol_type IS 'Type: fire, flood, gas_leak, power_outage, security_threat, medical, natural_disaster, other';
COMMENT ON COLUMN emergency_protocols.steps IS 'Array of step objects: [{order, title, description, responsible}]';
COMMENT ON COLUMN emergency_protocols.contacts IS 'Array of contact references: [{contact_id, role, priority}]';

-- Emergency Contacts
-- Key contacts for emergency situations per building
CREATE TABLE IF NOT EXISTS emergency_contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    role VARCHAR(100) NOT NULL,
    phone VARCHAR(50),
    phone_secondary VARCHAR(50),
    email VARCHAR(255),
    address TEXT,
    notes TEXT,
    priority_order INTEGER NOT NULL DEFAULT 0,
    contact_type VARCHAR(50) NOT NULL DEFAULT 'other',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    available_hours VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE emergency_contacts IS 'Emergency contact directory';
COMMENT ON COLUMN emergency_contacts.contact_type IS 'Type: fire_department, police, ambulance, utility_company, building_manager, security, maintenance, medical, other';
COMMENT ON COLUMN emergency_contacts.available_hours IS 'Availability schedule, e.g., "24/7" or "Mon-Fri 9-17"';

-- Emergency Incidents
-- Reports of emergency situations
CREATE TABLE IF NOT EXISTS emergency_incidents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    unit_id UUID REFERENCES units(id),
    reported_by UUID NOT NULL REFERENCES users(id),
    incident_type VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL DEFAULT 'medium',
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    location_details TEXT,
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),
    status VARCHAR(50) NOT NULL DEFAULT 'reported',
    resolution TEXT,
    resolved_by UUID REFERENCES users(id),
    resolved_at TIMESTAMPTZ,
    protocol_id UUID REFERENCES emergency_protocols(id),
    fault_id UUID REFERENCES faults(id),
    metadata JSONB DEFAULT '{}',
    reported_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE emergency_incidents IS 'Emergency incident reports';
COMMENT ON COLUMN emergency_incidents.incident_type IS 'Type: fire, flood, gas_leak, power_outage, security_threat, medical, natural_disaster, structural, other';
COMMENT ON COLUMN emergency_incidents.severity IS 'Severity: low, medium, high, critical';
COMMENT ON COLUMN emergency_incidents.status IS 'Status: reported, acknowledged, responding, contained, resolved, closed';

-- Emergency Incident Attachments
-- Photos, videos, documents related to incidents
CREATE TABLE IF NOT EXISTS emergency_incident_attachments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    incident_id UUID NOT NULL REFERENCES emergency_incidents(id) ON DELETE CASCADE,
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    attachment_type VARCHAR(50) NOT NULL DEFAULT 'photo',
    description TEXT,
    uploaded_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE emergency_incident_attachments IS 'Attachments for emergency incidents (photos, videos, documents)';
COMMENT ON COLUMN emergency_incident_attachments.attachment_type IS 'Type: photo, video, document, audio';

-- Emergency Incident Updates
-- Timeline of updates during incident response
CREATE TABLE IF NOT EXISTS emergency_incident_updates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    incident_id UUID NOT NULL REFERENCES emergency_incidents(id) ON DELETE CASCADE,
    update_type VARCHAR(50) NOT NULL DEFAULT 'note',
    previous_status VARCHAR(50),
    new_status VARCHAR(50),
    message TEXT NOT NULL,
    updated_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE emergency_incident_updates IS 'Timeline of incident updates and status changes';
COMMENT ON COLUMN emergency_incident_updates.update_type IS 'Type: status_change, note, action_taken, resource_assigned';

-- Emergency Broadcasts
-- Emergency alerts sent to building residents
CREATE TABLE IF NOT EXISTS emergency_broadcasts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    incident_id UUID REFERENCES emergency_incidents(id),
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    severity VARCHAR(20) NOT NULL DEFAULT 'high',
    channels JSONB NOT NULL DEFAULT '["push", "email"]',
    sent_by UUID NOT NULL REFERENCES users(id),
    sent_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    recipient_count INTEGER DEFAULT 0,
    delivered_count INTEGER DEFAULT 0,
    acknowledged_count INTEGER DEFAULT 0,
    metadata JSONB DEFAULT '{}'
);

COMMENT ON TABLE emergency_broadcasts IS 'Emergency broadcast alerts to residents';
COMMENT ON COLUMN emergency_broadcasts.channels IS 'Delivery channels: ["push", "sms", "email"]';
COMMENT ON COLUMN emergency_broadcasts.severity IS 'Severity: low, medium, high, critical';

-- Emergency Broadcast Acknowledgments
-- Track who has acknowledged receiving the alert
CREATE TABLE IF NOT EXISTS emergency_broadcast_acknowledgments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    broadcast_id UUID NOT NULL REFERENCES emergency_broadcasts(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status VARCHAR(50) NOT NULL DEFAULT 'safe',
    message TEXT,
    acknowledged_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(broadcast_id, user_id)
);

COMMENT ON TABLE emergency_broadcast_acknowledgments IS 'User acknowledgments for emergency broadcasts';
COMMENT ON COLUMN emergency_broadcast_acknowledgments.status IS 'Status: safe, need_help, evacuated, other';

-- Emergency Drills
-- Scheduled emergency drills and exercises
CREATE TABLE IF NOT EXISTS emergency_drills (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    protocol_id UUID REFERENCES emergency_protocols(id),
    drill_type VARCHAR(50) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    scheduled_at TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ,
    status VARCHAR(50) NOT NULL DEFAULT 'scheduled',
    participants_expected INTEGER,
    participants_actual INTEGER,
    duration_minutes INTEGER,
    notes TEXT,
    issues_found JSONB DEFAULT '[]',
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE emergency_drills IS 'Emergency drill scheduling and results';
COMMENT ON COLUMN emergency_drills.drill_type IS 'Type: fire, evacuation, lockdown, first_aid, full_scale';
COMMENT ON COLUMN emergency_drills.status IS 'Status: scheduled, in_progress, completed, cancelled';

-- Indexes
CREATE INDEX IF NOT EXISTS idx_emergency_protocols_org ON emergency_protocols(organization_id);
CREATE INDEX IF NOT EXISTS idx_emergency_protocols_building ON emergency_protocols(building_id);
CREATE INDEX IF NOT EXISTS idx_emergency_protocols_type ON emergency_protocols(protocol_type);
CREATE INDEX IF NOT EXISTS idx_emergency_protocols_active ON emergency_protocols(is_active) WHERE is_active = TRUE;

CREATE INDEX IF NOT EXISTS idx_emergency_contacts_org ON emergency_contacts(organization_id);
CREATE INDEX IF NOT EXISTS idx_emergency_contacts_building ON emergency_contacts(building_id);
CREATE INDEX IF NOT EXISTS idx_emergency_contacts_type ON emergency_contacts(contact_type);
CREATE INDEX IF NOT EXISTS idx_emergency_contacts_priority ON emergency_contacts(priority_order);

CREATE INDEX IF NOT EXISTS idx_emergency_incidents_org ON emergency_incidents(organization_id);
CREATE INDEX IF NOT EXISTS idx_emergency_incidents_building ON emergency_incidents(building_id);
CREATE INDEX IF NOT EXISTS idx_emergency_incidents_status ON emergency_incidents(status);
CREATE INDEX IF NOT EXISTS idx_emergency_incidents_severity ON emergency_incidents(severity);
CREATE INDEX IF NOT EXISTS idx_emergency_incidents_type ON emergency_incidents(incident_type);
CREATE INDEX IF NOT EXISTS idx_emergency_incidents_reported ON emergency_incidents(reported_at DESC);
CREATE INDEX IF NOT EXISTS idx_emergency_incidents_active ON emergency_incidents(status) WHERE status NOT IN ('resolved', 'closed');

CREATE INDEX IF NOT EXISTS idx_emergency_incident_attachments_incident ON emergency_incident_attachments(incident_id);
CREATE INDEX IF NOT EXISTS idx_emergency_incident_updates_incident ON emergency_incident_updates(incident_id);
CREATE INDEX IF NOT EXISTS idx_emergency_incident_updates_time ON emergency_incident_updates(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_emergency_broadcasts_org ON emergency_broadcasts(organization_id);
CREATE INDEX IF NOT EXISTS idx_emergency_broadcasts_building ON emergency_broadcasts(building_id);
CREATE INDEX IF NOT EXISTS idx_emergency_broadcasts_incident ON emergency_broadcasts(incident_id);
CREATE INDEX IF NOT EXISTS idx_emergency_broadcasts_active ON emergency_broadcasts(is_active) WHERE is_active = TRUE;
CREATE INDEX IF NOT EXISTS idx_emergency_broadcasts_sent ON emergency_broadcasts(sent_at DESC);

CREATE INDEX IF NOT EXISTS idx_emergency_broadcast_acks_broadcast ON emergency_broadcast_acknowledgments(broadcast_id);
CREATE INDEX IF NOT EXISTS idx_emergency_broadcast_acks_user ON emergency_broadcast_acknowledgments(user_id);

CREATE INDEX IF NOT EXISTS idx_emergency_drills_org ON emergency_drills(organization_id);
CREATE INDEX IF NOT EXISTS idx_emergency_drills_building ON emergency_drills(building_id);
CREATE INDEX IF NOT EXISTS idx_emergency_drills_scheduled ON emergency_drills(scheduled_at);
CREATE INDEX IF NOT EXISTS idx_emergency_drills_status ON emergency_drills(status);

-- RLS Policies
ALTER TABLE emergency_protocols ENABLE ROW LEVEL SECURITY;
ALTER TABLE emergency_contacts ENABLE ROW LEVEL SECURITY;
ALTER TABLE emergency_incidents ENABLE ROW LEVEL SECURITY;
ALTER TABLE emergency_incident_attachments ENABLE ROW LEVEL SECURITY;
ALTER TABLE emergency_incident_updates ENABLE ROW LEVEL SECURITY;
ALTER TABLE emergency_broadcasts ENABLE ROW LEVEL SECURITY;
ALTER TABLE emergency_broadcast_acknowledgments ENABLE ROW LEVEL SECURITY;
ALTER TABLE emergency_drills ENABLE ROW LEVEL SECURITY;

-- RLS Policies for emergency_protocols
CREATE POLICY emergency_protocols_tenant_isolation ON emergency_protocols
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', TRUE)::UUID);

-- RLS Policies for emergency_contacts
CREATE POLICY emergency_contacts_tenant_isolation ON emergency_contacts
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', TRUE)::UUID);

-- RLS Policies for emergency_incidents
CREATE POLICY emergency_incidents_tenant_isolation ON emergency_incidents
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', TRUE)::UUID);

-- RLS Policies for emergency_incident_attachments
CREATE POLICY emergency_incident_attachments_tenant_isolation ON emergency_incident_attachments
    FOR ALL USING (
        EXISTS (
            SELECT 1 FROM emergency_incidents i
            WHERE i.id = incident_id
            AND i.organization_id = current_setting('app.current_organization_id', TRUE)::UUID
        )
    );

-- RLS Policies for emergency_incident_updates
CREATE POLICY emergency_incident_updates_tenant_isolation ON emergency_incident_updates
    FOR ALL USING (
        EXISTS (
            SELECT 1 FROM emergency_incidents i
            WHERE i.id = incident_id
            AND i.organization_id = current_setting('app.current_organization_id', TRUE)::UUID
        )
    );

-- RLS Policies for emergency_broadcasts
CREATE POLICY emergency_broadcasts_tenant_isolation ON emergency_broadcasts
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', TRUE)::UUID);

-- RLS Policies for emergency_broadcast_acknowledgments
CREATE POLICY emergency_broadcast_acks_tenant_isolation ON emergency_broadcast_acknowledgments
    FOR ALL USING (
        EXISTS (
            SELECT 1 FROM emergency_broadcasts b
            WHERE b.id = broadcast_id
            AND b.organization_id = current_setting('app.current_organization_id', TRUE)::UUID
        )
    );

-- RLS Policies for emergency_drills
CREATE POLICY emergency_drills_tenant_isolation ON emergency_drills
    FOR ALL USING (organization_id = current_setting('app.current_organization_id', TRUE)::UUID);

-- Triggers for updated_at
CREATE TRIGGER set_updated_at_emergency_protocols
    BEFORE UPDATE ON emergency_protocols
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_updated_at_emergency_contacts
    BEFORE UPDATE ON emergency_contacts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_updated_at_emergency_incidents
    BEFORE UPDATE ON emergency_incidents
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_updated_at_emergency_drills
    BEFORE UPDATE ON emergency_drills
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
