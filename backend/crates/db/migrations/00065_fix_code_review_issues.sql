-- Migration 00065: Fix Code Review Issues from PRs 34, 35, 36
-- Addresses column mismatches, XSS vulnerabilities, and index optimization

-- ============================================
-- Fix PR #35: Column name mismatches in feed_subscriptions
-- Model expects: feed_type, sync_interval, last_sync_at, next_sync_at
-- Migration had: feed_format, refresh_frequency, last_fetched_at, next_fetch_at
-- ============================================

-- Rename columns to match model expectations
ALTER TABLE feed_subscriptions
    RENAME COLUMN feed_format TO feed_type;

ALTER TABLE feed_subscriptions
    RENAME COLUMN refresh_frequency TO sync_interval;

ALTER TABLE feed_subscriptions
    RENAME COLUMN last_fetched_at TO last_sync_at;

ALTER TABLE feed_subscriptions
    RENAME COLUMN next_fetch_at TO next_sync_at;

-- Drop the last_import_count column that doesn't exist in the model
ALTER TABLE feed_subscriptions
    DROP COLUMN IF EXISTS last_import_count;

-- ============================================
-- Fix PR #34: XSS vulnerability in document_search_headline
-- The function returns raw HTML with <mark> tags that could contain XSS
-- if the document text contains malicious HTML.
-- Fix: HTML-escape the document text before adding highlight markers
-- ============================================

-- Drop and recreate the function with HTML escaping
DROP FUNCTION IF EXISTS document_search_headline(TEXT, tsquery);

CREATE OR REPLACE FUNCTION document_search_headline(
    doc_text TEXT,
    search_query tsquery
) RETURNS TEXT AS $function$
DECLARE
    escaped_text TEXT;
BEGIN
    -- Escape HTML special characters in the source text
    escaped_text := COALESCE(doc_text, '');
    escaped_text := REPLACE(escaped_text, '&', '&amp;');
    escaped_text := REPLACE(escaped_text, '<', '&lt;');
    escaped_text := REPLACE(escaped_text, '>', '&gt;');
    escaped_text := REPLACE(escaped_text, '"', '&quot;');
    escaped_text := REPLACE(escaped_text, '''', '&#39;');

    -- Now apply ts_headline with safe marker tags
    RETURN ts_headline('english', escaped_text, search_query,
        'StartSel=<mark>, StopSel=</mark>, MaxWords=50, MinWords=20');
END;
$function$ LANGUAGE plpgsql IMMUTABLE;

COMMENT ON FUNCTION document_search_headline(TEXT, tsquery) IS
'Returns search headline with HTML-escaped content and highlighted matches. Safe for direct HTML rendering.';

-- ============================================
-- Fix PR #36: Add index for get_due_rules optimization
-- ============================================

CREATE INDEX IF NOT EXISTS idx_automation_rules_scheduled_due
    ON workflow_automation_rules(next_run_at)
    WHERE is_active = true AND trigger_type = 'schedule';

COMMENT ON INDEX idx_automation_rules_scheduled_due IS
'Optimized index for get_due_rules query that finds active scheduled rules due for execution';
