-- Migration: 00040_create_invoices_payments
-- Epic 11: Financial Management & Payments
--
-- Stories covered:
-- - 11.3: Invoice Generation
-- - 11.4: Payment Recording
-- - 11.5: Online Payment Integration
-- - 11.6: Payment Reminders & Overdue Handling

-- ============================================================================
-- INVOICE TYPES AND STATUS
-- ============================================================================

CREATE TYPE invoice_status AS ENUM (
    'draft',
    'sent',
    'paid',
    'partial',
    'overdue',
    'cancelled',
    'void'
);

CREATE TYPE payment_method AS ENUM (
    'bank_transfer',
    'card',
    'cash',
    'check',
    'online',
    'direct_debit',
    'other'
);

CREATE TYPE payment_status AS ENUM (
    'pending',
    'completed',
    'failed',
    'refunded',
    'cancelled'
);

-- ============================================================================
-- INVOICES (Story 11.3)
-- ============================================================================

CREATE TABLE invoices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,

    -- Invoice info
    invoice_number VARCHAR(50) NOT NULL,
    billing_period_start DATE,
    billing_period_end DATE,

    -- Status and dates
    status invoice_status NOT NULL DEFAULT 'draft',
    issue_date DATE NOT NULL DEFAULT CURRENT_DATE,
    due_date DATE NOT NULL,
    paid_date DATE,

    -- Amounts
    subtotal DECIMAL(15, 2) NOT NULL DEFAULT 0.00,
    tax_amount DECIMAL(15, 2) NOT NULL DEFAULT 0.00,
    total DECIMAL(15, 2) NOT NULL DEFAULT 0.00,
    amount_paid DECIMAL(15, 2) NOT NULL DEFAULT 0.00,
    balance_due DECIMAL(15, 2) NOT NULL DEFAULT 0.00,

    -- Currency
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',

    -- Notes
    notes TEXT,
    internal_notes TEXT,

    -- PDF storage
    pdf_file_path TEXT,
    pdf_generated_at TIMESTAMPTZ,

    -- Metadata
    created_by UUID REFERENCES users(id),
    sent_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE(organization_id, invoice_number)
);

CREATE INDEX idx_invoices_org ON invoices(organization_id);
CREATE INDEX idx_invoices_unit ON invoices(unit_id);
CREATE INDEX idx_invoices_status ON invoices(status);
CREATE INDEX idx_invoices_due_date ON invoices(due_date);
CREATE INDEX idx_invoices_number ON invoices(invoice_number);

-- Trigger for updated_at
CREATE TRIGGER invoices_updated_at
    BEFORE UPDATE ON invoices
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- INVOICE ITEMS (Story 11.3)
-- ============================================================================

CREATE TABLE invoice_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    invoice_id UUID NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,

    -- Item details
    description TEXT NOT NULL,
    quantity DECIMAL(10, 2) NOT NULL DEFAULT 1.00,
    unit_price DECIMAL(15, 2) NOT NULL,
    amount DECIMAL(15, 2) NOT NULL,

    -- Tax
    tax_rate DECIMAL(5, 2) DEFAULT 0.00,
    tax_amount DECIMAL(15, 2) DEFAULT 0.00,

    -- References
    fee_schedule_id UUID REFERENCES fee_schedules(id),
    meter_reading_id UUID,  -- Will reference meter_readings

    -- Sort order
    sort_order INTEGER NOT NULL DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_invoice_items_invoice ON invoice_items(invoice_id);
CREATE INDEX idx_invoice_items_fee_schedule ON invoice_items(fee_schedule_id);

-- ============================================================================
-- PAYMENTS (Story 11.4)
-- ============================================================================

CREATE TABLE payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE,

    -- Payment details
    amount DECIMAL(15, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',
    payment_method payment_method NOT NULL,
    status payment_status NOT NULL DEFAULT 'completed',

    -- Reference
    reference VARCHAR(255),
    external_reference VARCHAR(255),  -- Gateway transaction ID

    -- Dates
    payment_date DATE NOT NULL DEFAULT CURRENT_DATE,

    -- Notes
    notes TEXT,

    -- Metadata
    recorded_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_payments_org ON payments(organization_id);
CREATE INDEX idx_payments_unit ON payments(unit_id);
CREATE INDEX idx_payments_status ON payments(status);
CREATE INDEX idx_payments_date ON payments(payment_date);
CREATE INDEX idx_payments_reference ON payments(reference);

-- Trigger for updated_at
CREATE TRIGGER payments_updated_at
    BEFORE UPDATE ON payments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- PAYMENT ALLOCATIONS (Story 11.4)
-- ============================================================================

CREATE TABLE payment_allocations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    payment_id UUID NOT NULL REFERENCES payments(id) ON DELETE CASCADE,
    invoice_id UUID NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,

    -- Amount allocated to this invoice
    amount DECIMAL(15, 2) NOT NULL,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE(payment_id, invoice_id)
);

CREATE INDEX idx_payment_allocations_payment ON payment_allocations(payment_id);
CREATE INDEX idx_payment_allocations_invoice ON payment_allocations(invoice_id);

-- ============================================================================
-- CREDIT BALANCES (Story 11.4 - Overpayments)
-- ============================================================================

CREATE TABLE unit_credit_balances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    unit_id UUID NOT NULL REFERENCES units(id) ON DELETE CASCADE UNIQUE,

    -- Current credit balance
    balance DECIMAL(15, 2) NOT NULL DEFAULT 0.00,
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_unit_credit_balances_unit ON unit_credit_balances(unit_id);

-- Trigger for updated_at
CREATE TRIGGER unit_credit_balances_updated_at
    BEFORE UPDATE ON unit_credit_balances
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- ONLINE PAYMENTS (Story 11.5)
-- ============================================================================

CREATE TABLE online_payment_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    invoice_id UUID NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,

    -- Gateway info
    provider VARCHAR(50) NOT NULL,  -- 'stripe', 'paypal', etc.
    session_id VARCHAR(255) NOT NULL,
    checkout_url TEXT,

    -- Amount
    amount DECIMAL(15, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'EUR',

    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'pending',  -- pending, completed, failed, expired
    expires_at TIMESTAMPTZ,

    -- Result
    payment_id UUID REFERENCES payments(id),
    error_message TEXT,

    -- Metadata
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_online_payment_sessions_org ON online_payment_sessions(organization_id);
CREATE INDEX idx_online_payment_sessions_invoice ON online_payment_sessions(invoice_id);
CREATE INDEX idx_online_payment_sessions_session ON online_payment_sessions(session_id);
CREATE INDEX idx_online_payment_sessions_status ON online_payment_sessions(status);

-- Trigger for updated_at
CREATE TRIGGER online_payment_sessions_updated_at
    BEFORE UPDATE ON online_payment_sessions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- PAYMENT REMINDERS (Story 11.6)
-- ============================================================================

CREATE TABLE reminder_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,

    -- Schedule info
    name VARCHAR(255) NOT NULL,
    days_before_due INTEGER,  -- Negative for before, positive for after
    days_after_due INTEGER,

    -- Template
    email_template_id UUID,  -- References email templates
    notification_template TEXT,

    -- Settings
    is_active BOOLEAN NOT NULL DEFAULT true,
    include_sms BOOLEAN NOT NULL DEFAULT false,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_reminder_schedules_org ON reminder_schedules(organization_id);
CREATE INDEX idx_reminder_schedules_active ON reminder_schedules(is_active);

-- Trigger for updated_at
CREATE TRIGGER reminder_schedules_updated_at
    BEFORE UPDATE ON reminder_schedules
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- REMINDER LOGS (Story 11.6)
-- ============================================================================

CREATE TABLE reminder_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reminder_schedule_id UUID REFERENCES reminder_schedules(id) ON DELETE SET NULL,
    invoice_id UUID NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,

    -- Reminder info
    reminder_type VARCHAR(50) NOT NULL,  -- 'before_due', 'after_due', 'escalation'
    channel VARCHAR(50) NOT NULL,  -- 'email', 'sms', 'push', 'in_app'

    -- Status
    sent_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    delivered_at TIMESTAMPTZ,
    opened_at TIMESTAMPTZ,
    failed_at TIMESTAMPTZ,
    error_message TEXT,

    -- Metadata
    recipient_email VARCHAR(255),
    recipient_phone VARCHAR(50)
);

CREATE INDEX idx_reminder_logs_schedule ON reminder_logs(reminder_schedule_id);
CREATE INDEX idx_reminder_logs_invoice ON reminder_logs(invoice_id);
CREATE INDEX idx_reminder_logs_sent_at ON reminder_logs(sent_at);

-- ============================================================================
-- LATE FEES CONFIGURATION (Story 11.6)
-- ============================================================================

CREATE TABLE late_fee_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE UNIQUE,

    -- Fee settings
    enabled BOOLEAN NOT NULL DEFAULT false,
    grace_period_days INTEGER NOT NULL DEFAULT 0,

    -- Fee calculation
    fee_type VARCHAR(20) NOT NULL DEFAULT 'fixed',  -- 'fixed', 'percentage', 'daily_percentage'
    fee_amount DECIMAL(15, 2),  -- Fixed amount or percentage value
    max_fee_amount DECIMAL(15, 2),  -- Cap for percentage-based fees

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_late_fee_configs_org ON late_fee_configs(organization_id);

-- Trigger for updated_at
CREATE TRIGGER late_fee_configs_updated_at
    BEFORE UPDATE ON late_fee_configs
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- RLS POLICIES
-- ============================================================================

ALTER TABLE invoices ENABLE ROW LEVEL SECURITY;
ALTER TABLE invoice_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE payments ENABLE ROW LEVEL SECURITY;
ALTER TABLE payment_allocations ENABLE ROW LEVEL SECURITY;
ALTER TABLE unit_credit_balances ENABLE ROW LEVEL SECURITY;
ALTER TABLE online_payment_sessions ENABLE ROW LEVEL SECURITY;
ALTER TABLE reminder_schedules ENABLE ROW LEVEL SECURITY;
ALTER TABLE reminder_logs ENABLE ROW LEVEL SECURITY;
ALTER TABLE late_fee_configs ENABLE ROW LEVEL SECURITY;

-- Invoices: org members can view, admins/managers can manage
CREATE POLICY invoices_select ON invoices
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = invoices.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

CREATE POLICY invoices_manage ON invoices
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = invoices.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = invoices.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- Invoice items inherit from invoice
CREATE POLICY invoice_items_select ON invoice_items
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM invoices i
            JOIN organization_members om ON om.organization_id = i.organization_id
            WHERE i.id = invoice_items.invoice_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

CREATE POLICY invoice_items_manage ON invoice_items
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM invoices i
            JOIN organization_members om ON om.organization_id = i.organization_id
            WHERE i.id = invoice_items.invoice_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM invoices i
            JOIN organization_members om ON om.organization_id = i.organization_id
            WHERE i.id = invoice_items.invoice_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- Payments: org members can view, admins/managers can manage
CREATE POLICY payments_select ON payments
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = payments.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

CREATE POLICY payments_manage ON payments
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = payments.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = payments.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- Payment allocations inherit from payment
CREATE POLICY payment_allocations_select ON payment_allocations
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM payments p
            JOIN organization_members om ON om.organization_id = p.organization_id
            WHERE p.id = payment_allocations.payment_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

CREATE POLICY payment_allocations_manage ON payment_allocations
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM payments p
            JOIN organization_members om ON om.organization_id = p.organization_id
            WHERE p.id = payment_allocations.payment_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM payments p
            JOIN organization_members om ON om.organization_id = p.organization_id
            WHERE p.id = payment_allocations.payment_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- Credit balances inherit from unit
CREATE POLICY unit_credit_balances_select ON unit_credit_balances
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM units u
            JOIN buildings b ON b.id = u.building_id
            JOIN organization_members om ON om.organization_id = b.organization_id
            WHERE u.id = unit_credit_balances.unit_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

CREATE POLICY unit_credit_balances_manage ON unit_credit_balances
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM units u
            JOIN buildings b ON b.id = u.building_id
            JOIN organization_members om ON om.organization_id = b.organization_id
            WHERE u.id = unit_credit_balances.unit_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM units u
            JOIN buildings b ON b.id = u.building_id
            JOIN organization_members om ON om.organization_id = b.organization_id
            WHERE u.id = unit_credit_balances.unit_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- Online payment sessions
CREATE POLICY online_payment_sessions_select ON online_payment_sessions
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = online_payment_sessions.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

CREATE POLICY online_payment_sessions_manage ON online_payment_sessions
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = online_payment_sessions.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = online_payment_sessions.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
        )
        OR is_super_admin()
    );

-- Reminder schedules: admins/managers only
CREATE POLICY reminder_schedules_select ON reminder_schedules
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = reminder_schedules.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

CREATE POLICY reminder_schedules_manage ON reminder_schedules
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = reminder_schedules.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = reminder_schedules.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- Reminder logs inherit from invoice
CREATE POLICY reminder_logs_select ON reminder_logs
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM invoices i
            JOIN organization_members om ON om.organization_id = i.organization_id
            WHERE i.id = reminder_logs.invoice_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

CREATE POLICY reminder_logs_manage ON reminder_logs
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM invoices i
            JOIN organization_members om ON om.organization_id = i.organization_id
            WHERE i.id = reminder_logs.invoice_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM invoices i
            JOIN organization_members om ON om.organization_id = i.organization_id
            WHERE i.id = reminder_logs.invoice_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type IN ('org_admin', 'manager')
        )
        OR is_super_admin()
    );

-- Late fee configs: admins only
CREATE POLICY late_fee_configs_select ON late_fee_configs
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = late_fee_configs.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type = 'org_admin'
        )
        OR is_super_admin()
    );

CREATE POLICY late_fee_configs_manage ON late_fee_configs
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = late_fee_configs.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type = 'org_admin'
        )
        OR is_super_admin()
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM organization_members om
            WHERE om.organization_id = late_fee_configs.organization_id
            AND om.user_id = NULLIF(current_setting('app.current_user_id', TRUE), '')::UUID
            AND om.role_type = 'org_admin'
        )
        OR is_super_admin()
    );

-- ============================================================================
-- HELPER FUNCTION: Generate invoice number
-- ============================================================================

CREATE OR REPLACE FUNCTION generate_invoice_number(org_id UUID)
RETURNS VARCHAR(50) AS $$
DECLARE
    year_prefix VARCHAR(4);
    next_num INTEGER;
    result VARCHAR(50);
BEGIN
    year_prefix := TO_CHAR(CURRENT_DATE, 'YYYY');

    SELECT COALESCE(MAX(
        CASE
            WHEN invoice_number ~ ('^INV-' || year_prefix || '-\d+$')
            THEN CAST(SUBSTRING(invoice_number FROM '\d+$') AS INTEGER)
            ELSE 0
        END
    ), 0) + 1
    INTO next_num
    FROM invoices
    WHERE organization_id = org_id;

    result := 'INV-' || year_prefix || '-' || LPAD(next_num::TEXT, 6, '0');
    RETURN result;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- HELPER FUNCTION: Update invoice status based on payments
-- ============================================================================

CREATE OR REPLACE FUNCTION update_invoice_payment_status()
RETURNS TRIGGER AS $$
DECLARE
    total_allocated DECIMAL(15, 2);
    invoice_total DECIMAL(15, 2);
    new_status invoice_status;
BEGIN
    -- Calculate total allocated to this invoice
    SELECT COALESCE(SUM(amount), 0)
    INTO total_allocated
    FROM payment_allocations
    WHERE invoice_id = COALESCE(NEW.invoice_id, OLD.invoice_id);

    -- Get invoice total
    SELECT total
    INTO invoice_total
    FROM invoices
    WHERE id = COALESCE(NEW.invoice_id, OLD.invoice_id);

    -- Determine new status
    IF total_allocated >= invoice_total THEN
        new_status := 'paid';
    ELSIF total_allocated > 0 THEN
        new_status := 'partial';
    ELSE
        -- Check if overdue
        IF (SELECT due_date < CURRENT_DATE FROM invoices WHERE id = COALESCE(NEW.invoice_id, OLD.invoice_id)) THEN
            new_status := 'overdue';
        ELSE
            new_status := 'sent';
        END IF;
    END IF;

    -- Update invoice
    UPDATE invoices
    SET
        amount_paid = total_allocated,
        balance_due = total - total_allocated,
        status = new_status,
        paid_date = CASE WHEN new_status = 'paid' THEN CURRENT_DATE ELSE NULL END
    WHERE id = COALESCE(NEW.invoice_id, OLD.invoice_id);

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_invoice_on_allocation
    AFTER INSERT OR UPDATE OR DELETE ON payment_allocations
    FOR EACH ROW
    EXECUTE FUNCTION update_invoice_payment_status();
