-- Epic 13: AI Assistant & Automation
-- Story 13.2: Sentiment Analysis for Messages

-- Add sentiment columns to existing tables
ALTER TABLE announcements ADD COLUMN IF NOT EXISTS sentiment_score FLOAT;
ALTER TABLE announcements ADD COLUMN IF NOT EXISTS sentiment_analyzed_at TIMESTAMPTZ;

ALTER TABLE announcement_comments ADD COLUMN IF NOT EXISTS sentiment_score FLOAT;
ALTER TABLE announcement_comments ADD COLUMN IF NOT EXISTS sentiment_analyzed_at TIMESTAMPTZ;

ALTER TABLE messages ADD COLUMN IF NOT EXISTS sentiment_score FLOAT;
ALTER TABLE messages ADD COLUMN IF NOT EXISTS sentiment_analyzed_at TIMESTAMPTZ;

ALTER TABLE faults ADD COLUMN IF NOT EXISTS description_sentiment FLOAT;
ALTER TABLE faults ADD COLUMN IF NOT EXISTS sentiment_analyzed_at TIMESTAMPTZ;

ALTER TABLE fault_timeline ADD COLUMN IF NOT EXISTS sentiment_score FLOAT;

-- Sentiment trends table for aggregated metrics
CREATE TABLE IF NOT EXISTS sentiment_trends (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    avg_sentiment FLOAT NOT NULL,
    message_count INTEGER NOT NULL DEFAULT 0,
    negative_count INTEGER NOT NULL DEFAULT 0,
    neutral_count INTEGER NOT NULL DEFAULT 0,
    positive_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (organization_id, building_id, date)
);

-- Sentiment alerts when threshold is breached
CREATE TABLE IF NOT EXISTS sentiment_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    building_id UUID REFERENCES buildings(id) ON DELETE CASCADE,
    alert_type TEXT NOT NULL CHECK (alert_type IN ('spike_negative', 'sustained_decline', 'anomaly')),
    threshold_breached FLOAT NOT NULL,
    current_sentiment FLOAT NOT NULL,
    previous_sentiment FLOAT,
    sample_message_ids UUID[] DEFAULT '{}',
    acknowledged BOOLEAN DEFAULT FALSE,
    acknowledged_by UUID REFERENCES users(id),
    acknowledged_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Organization-level sentiment thresholds configuration
CREATE TABLE IF NOT EXISTS sentiment_thresholds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    negative_threshold FLOAT NOT NULL DEFAULT -0.3,
    alert_on_spike BOOLEAN DEFAULT TRUE,
    spike_threshold_change FLOAT DEFAULT 0.2,
    min_messages_for_alert INTEGER DEFAULT 5,
    enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (organization_id)
);

-- Indexes
CREATE INDEX idx_sentiment_trends_org ON sentiment_trends(organization_id);
CREATE INDEX idx_sentiment_trends_building ON sentiment_trends(building_id);
CREATE INDEX idx_sentiment_trends_date ON sentiment_trends(date DESC);
CREATE INDEX idx_sentiment_alerts_org ON sentiment_alerts(organization_id);
CREATE INDEX idx_sentiment_alerts_building ON sentiment_alerts(building_id);
CREATE INDEX idx_sentiment_alerts_created ON sentiment_alerts(created_at DESC);

-- RLS policies
ALTER TABLE sentiment_trends ENABLE ROW LEVEL SECURITY;
ALTER TABLE sentiment_alerts ENABLE ROW LEVEL SECURITY;
ALTER TABLE sentiment_thresholds ENABLE ROW LEVEL SECURITY;

CREATE POLICY sentiment_trends_tenant_isolation ON sentiment_trends
    FOR ALL
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY sentiment_alerts_tenant_isolation ON sentiment_alerts
    FOR ALL
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

CREATE POLICY sentiment_thresholds_tenant_isolation ON sentiment_thresholds
    FOR ALL
    USING (organization_id = current_setting('app.current_organization_id', true)::uuid);

-- Trigger for updated_at
CREATE TRIGGER update_sentiment_trends_updated_at
    BEFORE UPDATE ON sentiment_trends
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_sentiment_thresholds_updated_at
    BEFORE UPDATE ON sentiment_thresholds
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
