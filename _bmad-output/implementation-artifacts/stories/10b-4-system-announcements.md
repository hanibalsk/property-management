# Story 10B.4: System Announcements

Status: ready-for-dev

## Story

As a **platform administrator**,
I want to **broadcast system announcements**,
So that **all users are informed of platform changes**.

## Acceptance Criteria

1. **AC-1: System Announcement Creation**
   - Given an admin creates a system announcement
   - When they specify message and severity (info, warning, critical)
   - Then the announcement is queued for display

2. **AC-2: Announcement Display**
   - Given an announcement is active
   - When users open the app
   - Then they see the banner at top of screen
   - And can dismiss (for info) or must acknowledge (for critical)

3. **AC-3: Maintenance Scheduling**
   - Given an admin schedules maintenance
   - When the maintenance window approaches
   - Then countdown is displayed
   - And reminder notifications sent

## Tasks / Subtasks

- [ ] Task 1: Database Schema & Migrations (AC: 1, 2, 3)
  - [ ] 1.1 Create `system_announcements` table: id (UUID), title, message, severity (info/warning/critical), start_at, end_at, is_dismissible, requires_acknowledgment, created_by, created_at, updated_at
  - [ ] 1.2 Create `system_announcement_acknowledgments` table: id, announcement_id (FK), user_id (FK), acknowledged_at
  - [ ] 1.3 Create `scheduled_maintenance` table: id, title, description, start_at, end_at, is_read_only_mode, announcement_id (FK), created_by, created_at
  - [ ] 1.4 Add indexes for efficient active announcement queries

- [ ] Task 2: System Announcement Models (AC: 1, 2, 3)
  - [ ] 2.1 Create SystemAnnouncement model: id, title, message, severity, start_at, end_at, is_dismissible, requires_acknowledgment
  - [ ] 2.2 Create AnnouncementSeverity enum: Info, Warning, Critical
  - [ ] 2.3 Create SystemAnnouncementAcknowledgment model: id, announcement_id, user_id, acknowledged_at
  - [ ] 2.4 Create ScheduledMaintenance model: id, title, description, start_at, end_at, is_read_only_mode
  - [ ] 2.5 Create DTOs: CreateAnnouncementRequest, ActiveAnnouncementsResponse, CreateMaintenanceRequest

- [ ] Task 3: System Announcement Repository (AC: 1, 2, 3)
  - [ ] 3.1 Create SystemAnnouncementRepository
  - [ ] 3.2 Implement create_announcement() and update_announcement()
  - [ ] 3.3 Implement get_active_announcements() returning currently visible announcements
  - [ ] 3.4 Implement record_acknowledgment() for user acknowledgments
  - [ ] 3.5 Implement get_user_unacknowledged() for critical announcements user hasn't seen
  - [ ] 3.6 Implement schedule_maintenance() and get_upcoming_maintenance()

- [ ] Task 4: System Announcement Service (AC: 1, 2, 3)
  - [ ] 4.1 Create SystemAnnouncementService for announcement orchestration
  - [ ] 4.2 Implement create_announcement() with validation and audit logging
  - [ ] 4.3 Implement get_active_for_user() filtering based on user acknowledgments
  - [ ] 4.4 Implement acknowledge_announcement() with audit logging
  - [ ] 4.5 Implement schedule_maintenance() creating announcement and scheduling events

- [ ] Task 5: System Announcement API Endpoints (AC: 1, 2, 3)
  - [ ] 5.1 POST /api/v1/platform-admin/announcements - create system announcement
  - [ ] 5.2 GET /api/v1/platform-admin/announcements - list all announcements (admin view)
  - [ ] 5.3 PUT /api/v1/platform-admin/announcements/:id - update announcement
  - [ ] 5.4 DELETE /api/v1/platform-admin/announcements/:id - cancel/delete announcement
  - [ ] 5.5 GET /api/v1/announcements/active - public endpoint for current user's active announcements
  - [ ] 5.6 POST /api/v1/announcements/:id/acknowledge - user acknowledges announcement
  - [ ] 5.7 POST /api/v1/platform-admin/maintenance - schedule maintenance window
  - [ ] 5.8 GET /api/v1/maintenance/upcoming - public endpoint for upcoming maintenance

- [ ] Task 6: Unit & Integration Tests (AC: 1, 2, 3)
  - [ ] 6.1 Test announcement creation with various severities
  - [ ] 6.2 Test active announcement filtering by time window
  - [ ] 6.3 Test acknowledgment flow for critical announcements
  - [ ] 6.4 Test maintenance scheduling and announcement linkage
  - [ ] 6.5 Test authorization - only SuperAdmin can create announcements

## Dev Notes

### Architecture Requirements
- System announcements are platform-wide (no tenant scoping)
- Active announcements determined by: now() BETWEEN start_at AND end_at
- Critical announcements require acknowledgment before using app
- Maintenance mode can enable read-only access across platform

### Technical Specifications
- Backend: Rust + Axum following existing patterns
- Banner styling based on severity: info=blue, warning=yellow, critical=red
- Maintenance countdown calculated client-side from start_at timestamp
- Acknowledgments stored per-user to track who has seen critical alerts

### Security Considerations
- Only SuperAdmin can create, modify, or delete system announcements
- Public endpoints return only active announcements (no admin metadata)
- Maintenance mode enforcement in API middleware

### Database Patterns
- Follow existing model patterns in crates/db/src/models/
- No RLS needed - platform-wide announcements
- Consider soft-delete for announcements (is_deleted flag)

### References
- [Source: _bmad-output/epics.md#Epic-10B-Story-10B.4]
- Similar to building-level announcements but platform scope

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
