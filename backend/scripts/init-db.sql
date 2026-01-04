-- =============================================================================
-- PPT Database Initialization Script
-- =============================================================================
-- This script runs automatically when PostgreSQL container starts for the first time.
-- It sets up the database with required extensions and initial configuration.
--
-- Note: Actual table migrations should be handled by SQLx migrations.

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create schema for better organization (optional)
-- CREATE SCHEMA IF NOT EXISTS ppt;

-- Grant permissions (if using separate app user)
-- CREATE USER ppt_app WITH PASSWORD 'app_password';
-- GRANT CONNECT ON DATABASE ppt TO ppt_app;
-- GRANT USAGE ON SCHEMA public TO ppt_app;
-- ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO ppt_app;
-- ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON SEQUENCES TO ppt_app;

-- Row-Level Security policy helper function
CREATE OR REPLACE FUNCTION current_tenant_id()
RETURNS UUID AS $$
BEGIN
    RETURN NULLIF(current_setting('app.current_tenant_id', true), '')::UUID;
EXCEPTION
    WHEN OTHERS THEN
        RETURN NULL;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Session user helper function
CREATE OR REPLACE FUNCTION current_user_id()
RETURNS UUID AS $$
BEGIN
    RETURN NULLIF(current_setting('app.current_user_id', true), '')::UUID;
EXCEPTION
    WHEN OTHERS THEN
        RETURN NULL;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Log successful initialization
DO $$
BEGIN
    RAISE NOTICE 'PPT Database initialized successfully at %', NOW();
END $$;
