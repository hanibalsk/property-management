-- Migration: Create delegations table
-- Epic 3, Story 3.4: Ownership Delegation

-- Delegation scope enum
CREATE TYPE delegation_scope AS ENUM (
    'all',           -- Full delegation (voting, documents, faults, communications)
    'voting',        -- Only voting rights
    'documents',     -- Only document access
    'faults',        -- Only fault reporting/viewing
    'financial'      -- Financial matters (view invoices, payments)
);

-- Delegation status enum
CREATE TYPE delegation_status AS ENUM (
    'pending',       -- Invitation sent, awaiting acceptance
    'active',        -- Delegation is active
    'revoked',       -- Revoked by owner
    'expired',       -- Past end_date
    'declined'       -- Declined by delegate
);

-- Delegations table - tracks ownership delegations
CREATE TABLE delegations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Owner (who is delegating)
    owner_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Delegate (who receives the delegation)
    delegate_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Context - which unit(s) this delegation applies to
    unit_id UUID REFERENCES units(id) ON DELETE CASCADE,  -- NULL means all units owned by owner

    -- Delegation details
    scopes delegation_scope[] NOT NULL DEFAULT '{all}',
    status delegation_status NOT NULL DEFAULT 'pending',

    -- Validity period
    start_date DATE NOT NULL DEFAULT CURRENT_DATE,
    end_date DATE,  -- NULL means indefinite

    -- Invitation tracking
    invitation_token VARCHAR(255),
    invitation_sent_at TIMESTAMPTZ,
    accepted_at TIMESTAMPTZ,
    declined_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    revoked_reason TEXT,

    -- Audit
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraints
    CONSTRAINT no_self_delegation CHECK (owner_user_id != delegate_user_id),
    CONSTRAINT valid_date_range CHECK (end_date IS NULL OR end_date >= start_date)
);

-- Indexes
CREATE INDEX idx_delegations_owner ON delegations(owner_user_id);
CREATE INDEX idx_delegations_delegate ON delegations(delegate_user_id);
CREATE INDEX idx_delegations_unit ON delegations(unit_id) WHERE unit_id IS NOT NULL;
CREATE INDEX idx_delegations_status ON delegations(status);
CREATE INDEX idx_delegations_active ON delegations(delegate_user_id)
    WHERE status = 'active';
CREATE INDEX idx_delegations_token ON delegations(invitation_token)
    WHERE invitation_token IS NOT NULL;

-- Update trigger
CREATE TRIGGER update_delegations_updated_at
    BEFORE UPDATE ON delegations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Row Level Security
ALTER TABLE delegations ENABLE ROW LEVEL SECURITY;

-- RLS Policies
-- Owners can see their own delegations
CREATE POLICY delegations_select_owner ON delegations
    FOR SELECT
    USING (owner_user_id = current_setting('app.current_user_id', true)::UUID);

-- Delegates can see delegations to them
CREATE POLICY delegations_select_delegate ON delegations
    FOR SELECT
    USING (delegate_user_id = current_setting('app.current_user_id', true)::UUID);

-- Owners can create delegations
CREATE POLICY delegations_insert_owner ON delegations
    FOR INSERT
    WITH CHECK (owner_user_id = current_setting('app.current_user_id', true)::UUID);

-- Owners can update their delegations (e.g., revoke)
CREATE POLICY delegations_update_owner ON delegations
    FOR UPDATE
    USING (owner_user_id = current_setting('app.current_user_id', true)::UUID);

-- Delegates can update (accept/decline)
CREATE POLICY delegations_update_delegate ON delegations
    FOR UPDATE
    USING (
        delegate_user_id = current_setting('app.current_user_id', true)::UUID
        AND status = 'pending'
    );

-- Owners can delete their delegations
CREATE POLICY delegations_delete_owner ON delegations
    FOR DELETE
    USING (owner_user_id = current_setting('app.current_user_id', true)::UUID);

-- Super admin bypass
CREATE POLICY delegations_super_admin ON delegations
    FOR ALL
    USING (current_setting('app.is_super_admin', true)::BOOLEAN = TRUE);

-- Delegation audit log
CREATE TABLE delegation_audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    delegation_id UUID NOT NULL REFERENCES delegations(id) ON DELETE CASCADE,
    action VARCHAR(50) NOT NULL,  -- 'created', 'accepted', 'declined', 'revoked', 'expired'
    actor_user_id UUID REFERENCES users(id),
    details JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_delegation_audit_delegation ON delegation_audit_log(delegation_id);
CREATE INDEX idx_delegation_audit_created ON delegation_audit_log(created_at);

-- RLS for audit log (same as delegations)
ALTER TABLE delegation_audit_log ENABLE ROW LEVEL SECURITY;

CREATE POLICY delegation_audit_select ON delegation_audit_log
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM delegations d
            WHERE d.id = delegation_audit_log.delegation_id
            AND (
                d.owner_user_id = current_setting('app.current_user_id', true)::UUID
                OR d.delegate_user_id = current_setting('app.current_user_id', true)::UUID
            )
        )
    );

CREATE POLICY delegation_audit_super_admin ON delegation_audit_log
    FOR ALL
    USING (current_setting('app.is_super_admin', true)::BOOLEAN = TRUE);

-- Helper function to check if user has delegation for a scope
CREATE OR REPLACE FUNCTION has_delegation(
    p_delegate_user_id UUID,
    p_unit_id UUID,
    p_scope delegation_scope
)
RETURNS BOOLEAN AS $$
BEGIN
    RETURN EXISTS (
        SELECT 1 FROM delegations
        WHERE delegate_user_id = p_delegate_user_id
          AND status = 'active'
          AND (unit_id = p_unit_id OR unit_id IS NULL)
          AND (p_scope = ANY(scopes) OR 'all' = ANY(scopes))
          AND start_date <= CURRENT_DATE
          AND (end_date IS NULL OR end_date >= CURRENT_DATE)
    );
END;
$$ LANGUAGE plpgsql;

-- Function to create delegation with audit log
CREATE OR REPLACE FUNCTION create_delegation(
    p_owner_user_id UUID,
    p_delegate_user_id UUID,
    p_unit_id UUID,
    p_scopes delegation_scope[],
    p_start_date DATE DEFAULT CURRENT_DATE,
    p_end_date DATE DEFAULT NULL
)
RETURNS UUID AS $$
DECLARE
    v_delegation_id UUID;
    v_token VARCHAR(255);
BEGIN
    -- Generate invitation token
    v_token := encode(gen_random_bytes(32), 'hex');

    -- Create delegation
    INSERT INTO delegations (
        owner_user_id, delegate_user_id, unit_id, scopes,
        status, start_date, end_date, invitation_token, invitation_sent_at
    ) VALUES (
        p_owner_user_id, p_delegate_user_id, p_unit_id, p_scopes,
        'pending', p_start_date, p_end_date, v_token, NOW()
    )
    RETURNING id INTO v_delegation_id;

    -- Log creation
    INSERT INTO delegation_audit_log (delegation_id, action, actor_user_id, details)
    VALUES (v_delegation_id, 'created', p_owner_user_id, jsonb_build_object(
        'scopes', p_scopes,
        'unit_id', p_unit_id
    ));

    RETURN v_delegation_id;
END;
$$ LANGUAGE plpgsql;
