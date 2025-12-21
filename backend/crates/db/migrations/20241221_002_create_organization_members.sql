-- Epic 2A, Story 2A.5 & 2A.6: Organization Members and Roles
-- Creates organization membership junction table and roles table with RBAC

-- Roles table (organization-specific roles)
CREATE TABLE IF NOT EXISTS roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Role info
    name VARCHAR(100) NOT NULL,
    description TEXT,

    -- Permissions (JSONB array of permission strings)
    -- Format: ["resource:action", ...] e.g. ["faults:create", "votes:manage"]
    permissions JSONB NOT NULL DEFAULT '[]',

    -- System roles cannot be deleted/modified
    is_system BOOLEAN NOT NULL DEFAULT FALSE,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Unique role name per organization
    UNIQUE(organization_id, name)
);

-- Organization members junction table
CREATE TABLE IF NOT EXISTS organization_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Role assignment (can be custom role or reference system role)
    role_id UUID REFERENCES roles(id) ON DELETE SET NULL,

    -- Legacy role field for backward compatibility with TenantRole enum
    role_type VARCHAR(50) NOT NULL DEFAULT 'member',

    -- Membership status
    status VARCHAR(20) NOT NULL DEFAULT 'active' CHECK (status IN ('pending', 'active', 'suspended', 'removed')),

    -- Invitation tracking
    invited_by UUID REFERENCES users(id),
    invited_at TIMESTAMPTZ,
    joined_at TIMESTAMPTZ,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- A user can only be a member of an organization once
    UNIQUE(organization_id, user_id)
);

-- Indexes for roles
CREATE INDEX IF NOT EXISTS idx_roles_organization_id ON roles(organization_id);
CREATE INDEX IF NOT EXISTS idx_roles_name ON roles(name);

-- Indexes for organization_members
CREATE INDEX IF NOT EXISTS idx_org_members_organization_id ON organization_members(organization_id);
CREATE INDEX IF NOT EXISTS idx_org_members_user_id ON organization_members(user_id);
CREATE INDEX IF NOT EXISTS idx_org_members_role_id ON organization_members(role_id);
CREATE INDEX IF NOT EXISTS idx_org_members_status ON organization_members(status);

-- Triggers for updated_at
CREATE TRIGGER update_roles_updated_at
    BEFORE UPDATE ON roles
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_org_members_updated_at
    BEFORE UPDATE ON organization_members
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Insert default system roles for each new organization
CREATE OR REPLACE FUNCTION create_default_roles()
RETURNS TRIGGER AS $$
BEGIN
    -- Super Admin role (full access)
    INSERT INTO roles (organization_id, name, description, permissions, is_system)
    VALUES (NEW.id, 'Super Admin', 'Full platform access', '["*"]', TRUE);

    -- Organization Admin role
    INSERT INTO roles (organization_id, name, description, permissions, is_system)
    VALUES (NEW.id, 'Organization Admin', 'Full organization access',
        '["organization:*", "buildings:*", "units:*", "users:*", "faults:*", "votes:*", "documents:*", "announcements:*"]', TRUE);

    -- Manager role
    INSERT INTO roles (organization_id, name, description, permissions, is_system)
    VALUES (NEW.id, 'Manager', 'Building management access',
        '["buildings:read", "buildings:update", "units:*", "faults:*", "votes:manage", "documents:manage", "announcements:*"]', TRUE);

    -- Technical Manager role
    INSERT INTO roles (organization_id, name, description, permissions, is_system)
    VALUES (NEW.id, 'Technical Manager', 'Technical operations access',
        '["buildings:read", "units:read", "faults:*", "documents:read"]', TRUE);

    -- Owner role
    INSERT INTO roles (organization_id, name, description, permissions, is_system)
    VALUES (NEW.id, 'Owner', 'Property owner access',
        '["units:read", "faults:create", "faults:read", "votes:vote", "documents:read", "announcements:read"]', TRUE);

    -- Tenant role
    INSERT INTO roles (organization_id, name, description, permissions, is_system)
    VALUES (NEW.id, 'Tenant', 'Tenant access',
        '["units:read", "faults:create", "faults:read", "documents:read", "announcements:read"]', TRUE);

    -- Resident role
    INSERT INTO roles (organization_id, name, description, permissions, is_system)
    VALUES (NEW.id, 'Resident', 'Basic resident access',
        '["announcements:read", "faults:create", "documents:read"]', TRUE);

    -- Guest role
    INSERT INTO roles (organization_id, name, description, permissions, is_system)
    VALUES (NEW.id, 'Guest', 'Limited temporary access',
        '["announcements:read"]', TRUE);

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER create_organization_default_roles
    AFTER INSERT ON organizations
    FOR EACH ROW
    EXECUTE FUNCTION create_default_roles();

-- Helper function to check if user has permission in org
CREATE OR REPLACE FUNCTION user_has_permission(
    p_user_id UUID,
    p_org_id UUID,
    p_permission VARCHAR
)
RETURNS BOOLEAN AS $$
DECLARE
    v_permissions JSONB;
    v_permission TEXT;
BEGIN
    -- Get permissions for user in this org
    SELECT r.permissions INTO v_permissions
    FROM organization_members om
    JOIN roles r ON r.id = om.role_id
    WHERE om.user_id = p_user_id
      AND om.organization_id = p_org_id
      AND om.status = 'active';

    IF v_permissions IS NULL THEN
        RETURN FALSE;
    END IF;

    -- Check for wildcard permission
    IF v_permissions ? '*' THEN
        RETURN TRUE;
    END IF;

    -- Check for exact permission match
    IF v_permissions ? p_permission THEN
        RETURN TRUE;
    END IF;

    -- Check for resource wildcard (e.g., "faults:*" matches "faults:create")
    FOR v_permission IN SELECT jsonb_array_elements_text(v_permissions)
    LOOP
        IF v_permission LIKE '%:*' THEN
            IF split_part(p_permission, ':', 1) = split_part(v_permission, ':', 1) THEN
                RETURN TRUE;
            END IF;
        END IF;
    END LOOP;

    RETURN FALSE;
END;
$$ LANGUAGE plpgsql;
