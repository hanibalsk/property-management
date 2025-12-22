-- Epic 10B, Story 10B.6: User Onboarding Tour
-- Migration: Create user onboarding infrastructure

-- Create user_onboarding_progress table
CREATE TABLE IF NOT EXISTS user_onboarding_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tour_id VARCHAR(50) NOT NULL DEFAULT 'main',
    completed_steps JSONB NOT NULL DEFAULT '[]',
    current_step VARCHAR(50),
    is_completed BOOLEAN NOT NULL DEFAULT false,
    is_skipped BOOLEAN NOT NULL DEFAULT false,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, tour_id)
);

-- Create onboarding_tours table (tour definitions)
CREATE TABLE IF NOT EXISTS onboarding_tours (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tour_id VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    steps JSONB NOT NULL DEFAULT '[]',
    target_roles VARCHAR(255)[], -- Which roles see this tour
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_user_onboarding_user ON user_onboarding_progress(user_id);
CREATE INDEX IF NOT EXISTS idx_user_onboarding_tour ON user_onboarding_progress(tour_id);
CREATE INDEX IF NOT EXISTS idx_user_onboarding_incomplete ON user_onboarding_progress(user_id)
    WHERE is_completed = false AND is_skipped = false;

-- Insert default main tour
INSERT INTO onboarding_tours (tour_id, name, description, steps, target_roles, is_active)
VALUES (
    'main',
    'Welcome Tour',
    'Introduction to the Property Management platform',
    '[
        {"id": "welcome", "title": "Welcome!", "content": "Welcome to Property Management. This quick tour will help you get started.", "target": null},
        {"id": "dashboard", "title": "Dashboard", "content": "This is your dashboard. Here you can see an overview of your properties and tasks.", "target": "[data-tour=dashboard]"},
        {"id": "buildings", "title": "Buildings", "content": "Manage your buildings and units here.", "target": "[data-tour=buildings]"},
        {"id": "announcements", "title": "Announcements", "content": "Post and view announcements for your community.", "target": "[data-tour=announcements]"},
        {"id": "faults", "title": "Faults", "content": "Report and track maintenance issues.", "target": "[data-tour=faults]"},
        {"id": "profile", "title": "Profile", "content": "Update your profile and settings.", "target": "[data-tour=profile]"},
        {"id": "complete", "title": "All Set!", "content": "You are ready to go. Explore the platform and reach out if you have questions.", "target": null}
    ]'::jsonb,
    ARRAY['Owner', 'Manager', 'Resident'],
    true
)
ON CONFLICT (tour_id) DO NOTHING;

-- Add comments for documentation
COMMENT ON TABLE user_onboarding_progress IS 'Tracks user progress through onboarding tours (Epic 10B, Story 10B.6)';
COMMENT ON TABLE onboarding_tours IS 'Defines available onboarding tours and their steps';
COMMENT ON COLUMN user_onboarding_progress.completed_steps IS 'Array of step IDs that have been completed';
COMMENT ON COLUMN onboarding_tours.steps IS 'JSON array of tour steps with id, title, content, and target selector';
