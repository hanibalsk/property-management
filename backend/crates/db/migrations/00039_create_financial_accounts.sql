-- Migration: 00039_create_financial_accounts
-- Epic 11: Financial Management & Payments
--
-- Stories covered:
-- - 11.1: Financial Account Structure
-- - 11.2: Fee Schedule Management

-- ============================================================================
-- FINANCIAL ACCOUNT TYPES
-- ============================================================================

CREATE TYPE financial_account_type AS ENUM (
    'operating',      -- Operating/maintenance account
    'reserve',        -- Reserve fund
    'utilities',      -- Utility payments
    'unit_ledger',    -- Individual unit ledger
    'custom'          -- Custom account type
);

CREATE TYPE transaction_type AS ENUM (
    'debit',
    'credit'
);

CREATE TYPE transaction_category AS ENUM (
    'maintenance_fee',
    'utility_charge',
    'special_assessment',
    'penalty',
    'payment_received',
    'refund',
    'transfer',
    'adjustment',
    'opening_balance',
    'other'
);

-- ============================================================================
-- FINANCIAL ACCOUNTS (Story 11.1)
-- ============================================================================

CREATE TABLE financial_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,
    unit_id UUID REFERENCES units(id) ON DELETE CASCADE,

    -- Account info
    name VARCHAR(255) NOT NULL,
    account_type financial_account_type NOT NULL,
    description TEXT,
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',

    -- Balance tracking
    balance DECIMAL(15, 2) NOT NULL DEFAULT 0.00,
    opening_balance DECIMAL(15, 2) NOT NULL DEFAULT 0.00,

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    -- Unit ledger is tied to a specific unit
    CONSTRAINT check_unit_ledger CHECK (
        (account_type = 'unit_ledger' AND unit_id IS NOT NULL) OR
        (account_type != 'unit_ledger')
    )
);

CREATE INDEX idx_financial_accounts_org ON financial_accounts(organization_id);
CREATE INDEX idx_financial_accounts_building ON financial_accounts(building_id);
CREATE INDEX idx_financial_accounts_unit ON financial_accounts(unit_id);
CREATE INDEX idx_financial_accounts_type ON financial_accounts(account_type);

-- Trigger for updated_at
CREATE TRIGGER financial_accounts_updated_at
    BEFORE UPDATE ON financial_accounts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- ACCOUNT TRANSACTIONS (Story 11.1)
-- ============================================================================

CREATE TABLE account_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES financial_accounts(id) ON DELETE CASCADE,

    -- Transaction details
    amount DECIMAL(15, 2) NOT NULL,
    transaction_type transaction_type NOT NULL,
    category transaction_category NOT NULL,
    description TEXT,

    -- Reference for double-entry bookkeeping
    reference_id UUID,  -- Links paired transactions
    counterpart_account_id UUID REFERENCES financial_accounts(id),

    -- External references
    invoice_id UUID,  -- Will reference invoices table
    payment_id UUID,  -- Will reference payments table

    -- Balance after this transaction
    balance_after DECIMAL(15, 2) NOT NULL,

    -- Metadata
    transaction_date DATE NOT NULL DEFAULT CURRENT_DATE,
    recorded_by UUID REFERENCES users(id),
    notes TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_account_transactions_account ON account_transactions(account_id);
CREATE INDEX idx_account_transactions_date ON account_transactions(transaction_date);
CREATE INDEX idx_account_transactions_category ON account_transactions(category);
CREATE INDEX idx_account_transactions_reference ON account_transactions(reference_id);
CREATE INDEX idx_account_transactions_invoice ON account_transactions(invoice_id);
CREATE INDEX idx_account_transactions_payment ON account_transactions(payment_id);

-- ============================================================================
-- FEE SCHEDULES (Story 11.2)
-- ============================================================================

CREATE TYPE fee_frequency AS ENUM (
    'monthly',
    'quarterly',
    'semi_annual',
    'annual',
    'one_time'
);

CREATE TABLE fee_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,

    -- Fee info
    name VARCHAR(255) NOT NULL,
    description TEXT,
    amount DECIMAL(15, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',
    frequency fee_frequency NOT NULL DEFAULT 'monthly',

    -- Unit filter (JSONB for flexible filtering)
    -- Example: {"unit_types": ["apartment"], "floors": [1, 2], "unit_ids": ["uuid1", "uuid2"]}
    unit_filter JSONB NOT NULL DEFAULT '{}'::jsonb,

    -- Billing settings
    billing_day INTEGER CHECK (billing_day >= 1 AND billing_day <= 28),

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,

    -- Effective dates
    effective_from DATE NOT NULL DEFAULT CURRENT_DATE,
    effective_to DATE,

    -- Metadata
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_fee_schedules_org ON fee_schedules(organization_id);
CREATE INDEX idx_fee_schedules_building ON fee_schedules(building_id);
CREATE INDEX idx_fee_schedules_active ON fee_schedules(is_active);

-- Trigger for updated_at
CREATE TRIGGER fee_schedules_updated_at
    BEFORE UPDATE ON fee_schedules
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- UNIT FEES (Story 11.2)
-- ============================================================================

CREATE TABLE unit_fees (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    fee_schedule_id UUID NOT NULL REFERENCES fee_schedules(id) ON DELETE CASCADE,

    -- Override amount (if different from schedule)
    override_amount DECIMAL(15, 2),

    -- Effective period
    effective_from DATE NOT NULL DEFAULT CURRENT_DATE,
    effective_to DATE,

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE(unit_id, fee_schedule_id, effective_from)
);

CREATE INDEX idx_unit_fees_unit ON unit_fees(unit_id);
CREATE INDEX idx_unit_fees_schedule ON unit_fees(fee_schedule_id);
CREATE INDEX idx_unit_fees_active ON unit_fees(is_active);

-- Trigger for updated_at
CREATE TRIGGER unit_fees_updated_at
    BEFORE UPDATE ON unit_fees
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- RLS POLICIES
-- ============================================================================

ALTER TABLE financial_accounts ENABLE ROW LEVEL SECURITY;
ALTER TABLE account_transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE fee_schedules ENABLE ROW LEVEL SECURITY;
ALTER TABLE unit_fees ENABLE ROW LEVEL SECURITY;

-- Financial accounts: org members can view, admins/managers can manage
CREATE POLICY financial_accounts_select ON financial_accounts
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = financial_accounts.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

CREATE POLICY financial_accounts_manage ON financial_accounts
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = financial_accounts.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = financial_accounts.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- Transactions: org members can view, admins/managers can create
CREATE POLICY account_transactions_select ON account_transactions
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM financial_accounts fa
            JOIN organization_members om ON om.organization_id = fa.organization_id
            WHERE fa.id = account_transactions.account_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

CREATE POLICY account_transactions_manage ON account_transactions
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM financial_accounts fa
            JOIN organization_members om ON om.organization_id = fa.organization_id
            WHERE fa.id = account_transactions.account_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM financial_accounts fa
            JOIN organization_members om ON om.organization_id = fa.organization_id
            WHERE fa.id = account_transactions.account_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- Fee schedules: org members can view, admins/managers can manage
CREATE POLICY fee_schedules_select ON fee_schedules
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = fee_schedules.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

CREATE POLICY fee_schedules_manage ON fee_schedules
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = fee_schedules.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = fee_schedules.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- Unit fees: inherit from fee schedule org
CREATE POLICY unit_fees_select ON unit_fees
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM fee_schedules fs
            JOIN organization_members om ON om.organization_id = fs.organization_id
            WHERE fs.id = unit_fees.fee_schedule_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

CREATE POLICY unit_fees_manage ON unit_fees
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM fee_schedules fs
            JOIN organization_members om ON om.organization_id = fs.organization_id
            WHERE fs.id = unit_fees.fee_schedule_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM fee_schedules fs
            JOIN organization_members om ON om.organization_id = fs.organization_id
            WHERE fs.id = unit_fees.fee_schedule_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- ============================================================================
-- HELPER FUNCTION: Auto-create unit ledger on unit creation
-- ============================================================================

CREATE OR REPLACE FUNCTION create_unit_ledger_account()
RETURNS TRIGGER AS $$
BEGIN
    -- Only create if the unit's building has an organization
    INSERT INTO financial_accounts (
        organization_id,
        building_id,
        unit_id,
        name,
        account_type,
        description,
        currency
    )
    SELECT
        b.organization_id,
        NEW.building_id,
        NEW.id,
        'Unit ' || NEW.unit_number || ' Ledger',
        'unit_ledger',
        'Automatically created ledger for unit ' || NEW.unit_number,
        'EUR'
    FROM buildings b
    WHERE b.id = NEW.building_id
    AND b.organization_id IS NOT NULL;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER create_unit_ledger_on_insert
    AFTER INSERT ON units
    FOR EACH ROW
    EXECUTE FUNCTION create_unit_ledger_account();
