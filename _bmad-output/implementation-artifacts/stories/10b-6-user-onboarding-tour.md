# Story 10B.6: User Onboarding Tour

Status: ready-for-dev

## Story

As a **new user**,
I want to **see an interactive onboarding tour**,
So that **I understand how to use the system**.

## Acceptance Criteria

1. **AC-1: Automatic Tour Start**
   - Given a user logs in for the first time
   - When the dashboard loads
   - Then an onboarding tour starts automatically
   - And highlights key features step by step

2. **AC-2: Progressive Tour Steps**
   - Given a user is in the tour
   - When they complete a step
   - Then the next step is shown
   - And progress is tracked

3. **AC-3: Tour Restart**
   - Given a user skips or completes the tour
   - When they want to revisit
   - Then they can restart from help menu
   - And progress is reset

## Tasks / Subtasks

- [ ] Task 1: Database Schema & Migrations (AC: 1, 2, 3)
  - [ ] 1.1 Add `onboarding_completed_at` column to `users` table (nullable timestamp)
  - [ ] 1.2 Create `onboarding_tours` table: id, key, name, description, target_roles (JSONB), is_active, created_at
  - [ ] 1.3 Create `onboarding_steps` table: id, tour_id (FK), step_order, title, content, target_selector, placement, action_type
  - [ ] 1.4 Create `user_onboarding_progress` table: id, user_id (FK), tour_id (FK), current_step, completed_steps (JSONB), started_at, completed_at
  - [ ] 1.5 Seed default tours for manager and resident roles

- [ ] Task 2: Onboarding Models (AC: 1, 2, 3)
  - [ ] 2.1 Create OnboardingTour model: id, key, name, description, target_roles, is_active
  - [ ] 2.2 Create OnboardingStep model: id, tour_id, step_order, title, content, target_selector, placement, action_type
  - [ ] 2.3 Create UserOnboardingProgress model: id, user_id, tour_id, current_step, completed_steps, started_at, completed_at
  - [ ] 2.4 Create StepPlacement enum: Top, Bottom, Left, Right, Center
  - [ ] 2.5 Create DTOs: TourWithStepsResponse, StepProgressRequest, OnboardingStatusResponse

- [ ] Task 3: Onboarding Repository (AC: 1, 2, 3)
  - [ ] 3.1 Create OnboardingRepository
  - [ ] 3.2 Implement get_tour_for_role() returning tour with all steps
  - [ ] 3.3 Implement get_user_progress() for current tour state
  - [ ] 3.4 Implement update_progress() for step completion
  - [ ] 3.5 Implement reset_progress() for tour restart
  - [ ] 3.6 Implement mark_onboarding_complete() updating users table

- [ ] Task 4: Onboarding Service (AC: 1, 2, 3)
  - [ ] 4.1 Create OnboardingService for tour orchestration
  - [ ] 4.2 Implement get_onboarding_status() returning if user needs onboarding
  - [ ] 4.3 Implement start_tour() initializing progress record
  - [ ] 4.4 Implement complete_step() advancing through tour
  - [ ] 4.5 Implement skip_tour() marking as completed without full traversal
  - [ ] 4.6 Implement restart_tour() resetting progress

- [ ] Task 5: Onboarding API Endpoints (AC: 1, 2, 3)
  - [ ] 5.1 GET /api/v1/onboarding/status - get current user's onboarding status
  - [ ] 5.2 GET /api/v1/onboarding/tour - get tour for current user's role
  - [ ] 5.3 POST /api/v1/onboarding/start - start or resume tour
  - [ ] 5.4 POST /api/v1/onboarding/step/:step_id/complete - mark step as completed
  - [ ] 5.5 POST /api/v1/onboarding/skip - skip remaining tour
  - [ ] 5.6 POST /api/v1/onboarding/restart - restart tour from beginning
  - [ ] 5.7 POST /api/v1/onboarding/complete - mark entire onboarding as complete

- [ ] Task 6: Admin Tour Management (AC: 1, 2)
  - [ ] 6.1 GET /api/v1/platform-admin/onboarding/tours - list all tours
  - [ ] 6.2 POST /api/v1/platform-admin/onboarding/tours - create new tour
  - [ ] 6.3 PUT /api/v1/platform-admin/onboarding/tours/:id - update tour
  - [ ] 6.4 POST /api/v1/platform-admin/onboarding/tours/:id/steps - add step to tour
  - [ ] 6.5 PUT /api/v1/platform-admin/onboarding/tours/:id/steps/:sid - update step
  - [ ] 6.6 DELETE /api/v1/platform-admin/onboarding/tours/:id/steps/:sid - remove step

- [ ] Task 7: Unit & Integration Tests (AC: 1, 2, 3)
  - [ ] 7.1 Test tour retrieval based on user role
  - [ ] 7.2 Test step progression and progress tracking
  - [ ] 7.3 Test tour skip and restart flows
  - [ ] 7.4 Test onboarding status check on login
  - [ ] 7.5 Test admin tour management CRUD

## Dev Notes

### Architecture Requirements
- Tours are role-specific (manager tour vs resident tour)
- Progress tracked per-user per-tour
- Tour steps define: element selector, tooltip content, placement
- Frontend library integration: React Joyride or similar

### Technical Specifications
- Backend: Rust + Axum following existing patterns
- Tour configuration stored in database (admin-editable)
- Default tours seeded on migration
- target_selector uses CSS selectors for element targeting

### Security Considerations
- All users can access their own onboarding status
- Only SuperAdmin can manage tour definitions
- Progress is user-scoped (no cross-user visibility)

### Database Patterns
- Follow existing model patterns in crates/db/src/models/
- Tour steps ordered by step_order column
- Soft-delete for tours (is_active flag) to preserve progress history

### References
- [Source: _bmad-output/epics.md#Epic-10B-Story-10B.6]
- Frontend integration with React Joyride or Shepherd.js

## Dev Agent Record

### Agent Model Used

TBD

### Debug Log References

N/A

### Completion Notes List

(To be filled during implementation)

### File List

(To be filled during implementation)

## Change Log

| Date | Change |
|------|--------|
| 2025-12-21 | Story created |
