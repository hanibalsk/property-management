-- Epic 102: Distributed Tracing & Observability (Story 102.1)
-- Migration: Create distributed tracing tables for trace and span storage

-- Create enum for span kind
DO $$ BEGIN
    CREATE TYPE span_kind AS ENUM (
        'server',
        'client',
        'producer',
        'consumer',
        'internal'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- Create enum for span status
DO $$ BEGIN
    CREATE TYPE span_status AS ENUM (
        'unset',
        'ok',
        'error'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- Distributed traces table
CREATE TABLE IF NOT EXISTS distributed_traces (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trace_id VARCHAR(32) NOT NULL,
    root_span_id VARCHAR(16) NOT NULL,
    service_name VARCHAR(100) NOT NULL,
    operation_name VARCHAR(255) NOT NULL,
    http_method VARCHAR(10),
    http_path VARCHAR(500),
    http_status_code INTEGER,
    duration_ms BIGINT NOT NULL,
    has_error BOOLEAN NOT NULL DEFAULT false,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    org_id UUID REFERENCES organizations(id) ON DELETE SET NULL,
    attributes JSONB,
    started_at TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for trace queries
CREATE INDEX IF NOT EXISTS idx_distributed_traces_trace_id ON distributed_traces(trace_id);
CREATE INDEX IF NOT EXISTS idx_distributed_traces_service_name ON distributed_traces(service_name);
CREATE INDEX IF NOT EXISTS idx_distributed_traces_operation_name ON distributed_traces(operation_name);
CREATE INDEX IF NOT EXISTS idx_distributed_traces_started_at ON distributed_traces(started_at DESC);
CREATE INDEX IF NOT EXISTS idx_distributed_traces_duration_ms ON distributed_traces(duration_ms);
CREATE INDEX IF NOT EXISTS idx_distributed_traces_has_error ON distributed_traces(has_error) WHERE has_error = true;
CREATE INDEX IF NOT EXISTS idx_distributed_traces_user_id ON distributed_traces(user_id) WHERE user_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_distributed_traces_org_id ON distributed_traces(org_id) WHERE org_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_distributed_traces_http_status ON distributed_traces(http_status_code);

-- Distributed spans table
CREATE TABLE IF NOT EXISTS distributed_spans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trace_id UUID NOT NULL REFERENCES distributed_traces(id) ON DELETE CASCADE,
    span_id VARCHAR(16) NOT NULL,
    parent_span_id VARCHAR(16),
    service_name VARCHAR(100) NOT NULL,
    operation_name VARCHAR(255) NOT NULL,
    span_kind span_kind NOT NULL DEFAULT 'internal',
    duration_ms BIGINT NOT NULL,
    status span_status NOT NULL DEFAULT 'unset',
    error_message TEXT,
    attributes JSONB,
    events JSONB,
    started_at TIMESTAMPTZ NOT NULL,
    ended_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for span queries
CREATE INDEX IF NOT EXISTS idx_distributed_spans_trace_id ON distributed_spans(trace_id);
CREATE INDEX IF NOT EXISTS idx_distributed_spans_span_id ON distributed_spans(span_id);
CREATE INDEX IF NOT EXISTS idx_distributed_spans_parent_span_id ON distributed_spans(parent_span_id);
CREATE INDEX IF NOT EXISTS idx_distributed_spans_service_name ON distributed_spans(service_name);
CREATE INDEX IF NOT EXISTS idx_distributed_spans_started_at ON distributed_spans(started_at DESC);
CREATE INDEX IF NOT EXISTS idx_distributed_spans_status ON distributed_spans(status) WHERE status = 'error';

-- Add comments for documentation
COMMENT ON TABLE distributed_traces IS 'Stores distributed traces representing request flows across services (Epic 102, Story 102.1)';
COMMENT ON COLUMN distributed_traces.trace_id IS 'OpenTelemetry trace ID (128-bit hex string, stored as 32 chars)';
COMMENT ON COLUMN distributed_traces.root_span_id IS 'ID of the root span that initiated this trace';
COMMENT ON COLUMN distributed_traces.service_name IS 'Name of the service that initiated the trace';
COMMENT ON COLUMN distributed_traces.operation_name IS 'Operation name (e.g., GET /api/v1/buildings)';
COMMENT ON COLUMN distributed_traces.attributes IS 'Additional trace attributes as JSON';

COMMENT ON TABLE distributed_spans IS 'Stores individual spans within distributed traces';
COMMENT ON COLUMN distributed_spans.span_id IS 'OpenTelemetry span ID (64-bit hex string, stored as 16 chars)';
COMMENT ON COLUMN distributed_spans.parent_span_id IS 'Parent span ID (null for root span)';
COMMENT ON COLUMN distributed_spans.span_kind IS 'Type of span: server, client, producer, consumer, internal';
COMMENT ON COLUMN distributed_spans.events IS 'Span events as JSON array';

-- Function to cleanup old traces (retention policy)
CREATE OR REPLACE FUNCTION cleanup_old_traces(retention_days INTEGER DEFAULT 30)
RETURNS BIGINT AS $$
DECLARE
    deleted_count BIGINT;
BEGIN
    DELETE FROM distributed_traces
    WHERE started_at < NOW() - (retention_days || ' days')::INTERVAL;

    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION cleanup_old_traces IS 'Removes traces older than the specified retention period';

-- Function to get trace statistics for dashboard
CREATE OR REPLACE FUNCTION get_trace_statistics(time_window_hours INTEGER DEFAULT 24)
RETURNS TABLE (
    total_traces BIGINT,
    error_traces BIGINT,
    error_rate_percent DOUBLE PRECISION,
    avg_duration_ms DOUBLE PRECISION,
    p95_duration_ms DOUBLE PRECISION,
    p99_duration_ms DOUBLE PRECISION,
    requests_per_minute DOUBLE PRECISION
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        COUNT(*)::BIGINT AS total_traces,
        COUNT(*) FILTER (WHERE has_error)::BIGINT AS error_traces,
        CASE
            WHEN COUNT(*) > 0 THEN (COUNT(*) FILTER (WHERE has_error)::DOUBLE PRECISION / COUNT(*)::DOUBLE PRECISION) * 100
            ELSE 0
        END AS error_rate_percent,
        AVG(duration_ms) AS avg_duration_ms,
        PERCENTILE_CONT(0.95) WITHIN GROUP (ORDER BY duration_ms) AS p95_duration_ms,
        PERCENTILE_CONT(0.99) WITHIN GROUP (ORDER BY duration_ms) AS p99_duration_ms,
        CASE
            WHEN time_window_hours > 0 THEN COUNT(*)::DOUBLE PRECISION / (time_window_hours * 60)
            ELSE 0
        END AS requests_per_minute
    FROM distributed_traces
    WHERE started_at > NOW() - (time_window_hours || ' hours')::INTERVAL;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION get_trace_statistics IS 'Returns aggregated trace statistics for the specified time window';

-- Add RLS policies for traces (org-scoped access)
ALTER TABLE distributed_traces ENABLE ROW LEVEL SECURITY;

-- Policy: Users can see traces in their organization or without org context
CREATE POLICY distributed_traces_org_select ON distributed_traces
    FOR SELECT
    USING (
        org_id IS NULL
        OR org_id = current_setting('app.current_org_id', true)::UUID
        OR is_current_user_super_admin()
    );

-- Policy: System can insert traces
CREATE POLICY distributed_traces_system_insert ON distributed_traces
    FOR INSERT
    WITH CHECK (
        is_current_user_super_admin()
        OR COALESCE(NULLIF(current_setting('app.is_system', true), ''), 'false') = 'true'
    );

-- Spans inherit access through trace_id foreign key
ALTER TABLE distributed_spans ENABLE ROW LEVEL SECURITY;

CREATE POLICY distributed_spans_select ON distributed_spans
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM distributed_traces t
            WHERE t.id = distributed_spans.trace_id
        )
    );

CREATE POLICY distributed_spans_system_insert ON distributed_spans
    FOR INSERT
    WITH CHECK (
        is_current_user_super_admin()
        OR COALESCE(NULLIF(current_setting('app.is_system', true), ''), 'false') = 'true'
    );
