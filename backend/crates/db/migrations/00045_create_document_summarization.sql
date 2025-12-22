-- Epic 13: AI Assistant & Automation
-- Story 13.4: Automatic Document Summarization

-- Add AI summary columns to documents table
ALTER TABLE documents ADD COLUMN IF NOT EXISTS ai_summary TEXT;
ALTER TABLE documents ADD COLUMN IF NOT EXISTS ai_summary_generated_at TIMESTAMPTZ;
ALTER TABLE documents ADD COLUMN IF NOT EXISTS ai_key_points JSONB DEFAULT '[]';
ALTER TABLE documents ADD COLUMN IF NOT EXISTS ai_action_items JSONB DEFAULT '[]';
ALTER TABLE documents ADD COLUMN IF NOT EXISTS word_count INTEGER;
ALTER TABLE documents ADD COLUMN IF NOT EXISTS language_detected TEXT;
