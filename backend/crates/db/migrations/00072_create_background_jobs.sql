-- Phase 1.3: Background Jobs Infrastructure (Epic 71, Story 71.3)
-- This creates tables for persistent job queue storage and execution tracking

-- Create enum for background job status
DO $$ BEGIN
    CREATE TYPE background_job_status AS ENUM (
        'pending',
        'scheduled',
        'running',
        'completed',
        'failed',
        'retrying',
        'cancelled',
        'timed_out'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- Background jobs table
CREATE TABLE IF NOT EXISTS background_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_type VARCHAR(100) NOT NULL,
    priority INTEGER NOT NULL DEFAULT 0,
    status background_job_status NOT NULL DEFAULT 'pending',
    payload JSONB NOT NULL DEFAULT '{}',
    result JSONB,
    error_message TEXT,
    error_details JSONB,
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,
    scheduled_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    duration_ms BIGINT,
    queue VARCHAR(50) NOT NULL DEFAULT 'default',
    worker_id VARCHAR(100),
    retry_delay_seconds INTEGER,
    org_id UUID REFERENCES organizations(id) ON DELETE SET NULL,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for efficient job queries
CREATE INDEX IF NOT EXISTS idx_background_jobs_status ON background_jobs(status);
CREATE INDEX IF NOT EXISTS idx_background_jobs_queue_status ON background_jobs(queue, status);
CREATE INDEX IF NOT EXISTS idx_background_jobs_scheduled_at ON background_jobs(scheduled_at) WHERE status IN ('pending', 'scheduled');
CREATE INDEX IF NOT EXISTS idx_background_jobs_priority ON background_jobs(priority DESC, scheduled_at ASC) WHERE status = 'pending';
CREATE INDEX IF NOT EXISTS idx_background_jobs_job_type ON background_jobs(job_type);
CREATE INDEX IF NOT EXISTS idx_background_jobs_org_id ON background_jobs(org_id) WHERE org_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_background_jobs_created_by ON background_jobs(created_by) WHERE created_by IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_background_jobs_created_at ON background_jobs(created_at);

-- Background job execution history
CREATE TABLE IF NOT EXISTS background_job_executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_id UUID NOT NULL REFERENCES background_jobs(id) ON DELETE CASCADE,
    attempt INTEGER NOT NULL,
    status background_job_status NOT NULL,
    worker_id VARCHAR(100) NOT NULL,
    error_message TEXT,
    error_details JSONB,
    duration_ms BIGINT,
    started_at TIMESTAMPTZ NOT NULL,
    ended_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_background_job_executions_job_id ON background_job_executions(job_id);
CREATE INDEX IF NOT EXISTS idx_background_job_executions_started_at ON background_job_executions(started_at);

-- Add comments for documentation
COMMENT ON TABLE background_jobs IS 'Persistent queue for background job processing. Jobs are picked up by workers and executed asynchronously.';
COMMENT ON COLUMN background_jobs.job_type IS 'Type of job (e.g., email_send, report_generate, data_export)';
COMMENT ON COLUMN background_jobs.priority IS 'Higher values = higher priority. Default is 0.';
COMMENT ON COLUMN background_jobs.queue IS 'Queue name for job routing. Workers can subscribe to specific queues.';
COMMENT ON COLUMN background_jobs.worker_id IS 'ID of the worker that claimed/is processing this job.';
COMMENT ON COLUMN background_jobs.retry_delay_seconds IS 'Delay before next retry attempt for exponential backoff.';

COMMENT ON TABLE background_job_executions IS 'Execution history for background jobs. Tracks each attempt for debugging and auditing.';

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_background_job_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for auto-updating updated_at
DROP TRIGGER IF EXISTS trigger_background_jobs_updated_at ON background_jobs;
CREATE TRIGGER trigger_background_jobs_updated_at
    BEFORE UPDATE ON background_jobs
    FOR EACH ROW
    EXECUTE FUNCTION update_background_job_updated_at();

-- Function to claim a job for processing (atomic operation)
CREATE OR REPLACE FUNCTION claim_background_job(
    p_queue VARCHAR(50),
    p_worker_id VARCHAR(100),
    p_job_types VARCHAR(100)[] DEFAULT NULL
)
RETURNS background_jobs AS $$
DECLARE
    claimed_job background_jobs;
BEGIN
    -- Atomically claim a job: find the highest priority pending job and mark it as running
    UPDATE background_jobs
    SET status = 'running',
        worker_id = p_worker_id,
        started_at = NOW(),
        attempts = attempts + 1
    WHERE id = (
        SELECT id FROM background_jobs
        WHERE queue = p_queue
          AND status IN ('pending', 'scheduled')
          AND scheduled_at <= NOW()
          AND (p_job_types IS NULL OR job_type = ANY(p_job_types))
        ORDER BY priority DESC, scheduled_at ASC
        LIMIT 1
        FOR UPDATE SKIP LOCKED
    )
    RETURNING * INTO claimed_job;

    RETURN claimed_job;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION claim_background_job IS 'Atomically claim a job for processing. Uses SKIP LOCKED to prevent contention between workers.';

-- Function to complete a job
CREATE OR REPLACE FUNCTION complete_background_job(
    p_job_id UUID,
    p_result JSONB DEFAULT NULL
)
RETURNS background_jobs AS $$
DECLARE
    completed_job background_jobs;
BEGIN
    UPDATE background_jobs
    SET status = 'completed',
        result = p_result,
        completed_at = NOW(),
        duration_ms = EXTRACT(EPOCH FROM (NOW() - started_at)) * 1000
    WHERE id = p_job_id AND status = 'running'
    RETURNING * INTO completed_job;

    -- Record execution
    IF completed_job IS NOT NULL THEN
        INSERT INTO background_job_executions (job_id, attempt, status, worker_id, duration_ms, started_at, ended_at)
        VALUES (completed_job.id, completed_job.attempts, 'completed', completed_job.worker_id, completed_job.duration_ms, completed_job.started_at, NOW());
    END IF;

    RETURN completed_job;
END;
$$ LANGUAGE plpgsql;

-- Function to fail a job
CREATE OR REPLACE FUNCTION fail_background_job(
    p_job_id UUID,
    p_error_message TEXT,
    p_error_details JSONB DEFAULT NULL
)
RETURNS background_jobs AS $$
DECLARE
    failed_job background_jobs;
    new_status background_job_status;
    retry_delay INTEGER;
BEGIN
    -- Get current job state
    SELECT * INTO failed_job FROM background_jobs WHERE id = p_job_id AND status = 'running';

    IF failed_job IS NULL THEN
        RETURN NULL;
    END IF;

    -- Determine if we should retry or fail permanently
    IF failed_job.attempts < failed_job.max_attempts THEN
        new_status := 'retrying';
        -- Exponential backoff: 2^attempt * 10 seconds (10s, 20s, 40s, ...)
        retry_delay := POWER(2, failed_job.attempts) * 10;
    ELSE
        new_status := 'failed';
        retry_delay := NULL;
    END IF;

    -- Update job
    UPDATE background_jobs
    SET status = new_status,
        error_message = p_error_message,
        error_details = p_error_details,
        completed_at = CASE WHEN new_status = 'failed' THEN NOW() ELSE NULL END,
        scheduled_at = CASE WHEN new_status = 'retrying' THEN NOW() + (retry_delay || ' seconds')::INTERVAL ELSE scheduled_at END,
        retry_delay_seconds = retry_delay,
        duration_ms = EXTRACT(EPOCH FROM (NOW() - started_at)) * 1000
    WHERE id = p_job_id
    RETURNING * INTO failed_job;

    -- Record execution
    INSERT INTO background_job_executions (job_id, attempt, status, worker_id, error_message, error_details, duration_ms, started_at, ended_at)
    VALUES (failed_job.id, failed_job.attempts, new_status, failed_job.worker_id, p_error_message, p_error_details, failed_job.duration_ms, failed_job.started_at, NOW());

    RETURN failed_job;
END;
$$ LANGUAGE plpgsql;

-- Function to get queue statistics
CREATE OR REPLACE FUNCTION get_background_job_queue_stats(p_queue VARCHAR(50))
RETURNS TABLE (
    queue VARCHAR(50),
    pending_count BIGINT,
    running_count BIGINT,
    failed_count_24h BIGINT,
    completed_count_24h BIGINT,
    avg_duration_ms DOUBLE PRECISION,
    p95_duration_ms DOUBLE PRECISION,
    retrying_count BIGINT,
    oldest_pending_age_seconds BIGINT
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        p_queue AS queue,
        COUNT(*) FILTER (WHERE bj.status = 'pending') AS pending_count,
        COUNT(*) FILTER (WHERE bj.status = 'running') AS running_count,
        COUNT(*) FILTER (WHERE bj.status = 'failed' AND bj.completed_at > NOW() - INTERVAL '24 hours') AS failed_count_24h,
        COUNT(*) FILTER (WHERE bj.status = 'completed' AND bj.completed_at > NOW() - INTERVAL '24 hours') AS completed_count_24h,
        AVG(bj.duration_ms) FILTER (WHERE bj.status = 'completed' AND bj.completed_at > NOW() - INTERVAL '24 hours') AS avg_duration_ms,
        PERCENTILE_CONT(0.95) WITHIN GROUP (ORDER BY bj.duration_ms) FILTER (WHERE bj.status = 'completed' AND bj.completed_at > NOW() - INTERVAL '24 hours') AS p95_duration_ms,
        COUNT(*) FILTER (WHERE bj.status = 'retrying') AS retrying_count,
        EXTRACT(EPOCH FROM (NOW() - MIN(bj.scheduled_at) FILTER (WHERE bj.status = 'pending')))::BIGINT AS oldest_pending_age_seconds
    FROM background_jobs bj
    WHERE bj.queue = p_queue;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION get_background_job_queue_stats IS 'Get statistics for a specific job queue including pending, running, failed counts and performance metrics.';

-- Create RLS policies for background jobs (org-scoped access)
ALTER TABLE background_jobs ENABLE ROW LEVEL SECURITY;

-- Policy: Users can see jobs in their organization
CREATE POLICY background_jobs_org_select ON background_jobs
    FOR SELECT
    USING (
        org_id IS NULL
        OR org_id = current_setting('app.current_org_id', true)::UUID
        OR is_current_user_super_admin()
    );

-- Policy: Users can insert jobs for their organization
CREATE POLICY background_jobs_org_insert ON background_jobs
    FOR INSERT
    WITH CHECK (
        org_id IS NULL
        OR org_id = current_setting('app.current_org_id', true)::UUID
        OR is_current_user_super_admin()
    );

-- Policy: Only system can update jobs (workers use service account)
CREATE POLICY background_jobs_system_update ON background_jobs
    FOR UPDATE
    USING (
        is_current_user_super_admin()
        OR COALESCE(NULLIF(current_setting('app.is_system', true), ''), 'false') = 'true'
    );

-- Grant permissions for service role to manage jobs
-- (Assuming a service role exists for background workers)
