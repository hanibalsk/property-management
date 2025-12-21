# Story 6.4: Pinned Announcements

Status: ready-for-dev

## Story

As a **property manager**,
I want to **pin important announcements**,
so that **they remain visible at the top**.

## Acceptance Criteria

1. **AC-1: Pin Announcement**
   - Given a manager pins an announcement
   - When residents view the announcement list
   - Then pinned items appear at the top
   - And are visually distinguished

2. **AC-2: Multiple Pinned Limit**
   - Given multiple announcements are pinned
   - When displayed
   - Then they're sorted by pin date
   - And limit of 3 pinned per building enforced

3. **AC-3: Unpin Announcement**
   - Given a manager unpins an announcement
   - When the action is taken
   - Then it returns to chronological order
   - And changes immediately visible

## Tasks / Subtasks

- [ ] Task 1: Database Schema & Migrations (AC: 1, 2, 3)
  - [ ] 1.1 Add columns to announcements table: pinned_at (TIMESTAMP NULL), pinned_by (UUID NULL FK to users)
  - [ ] 1.2 Add index: idx_announcements_pinned for efficient pinned queries
  - [ ] 1.3 Create database function/trigger to enforce 3 pinned limit per org/building

- [ ] Task 2: Backend Domain Models & Repository (AC: 1, 2, 3)
  - [ ] 2.1 Update Announcement model with pinned_at, pinned_by fields
  - [ ] 2.2 Implement pin_announcement(announcement_id, user_id) method
  - [ ] 2.3 Implement unpin_announcement(announcement_id) method
  - [ ] 2.4 Implement get_pinned_count(org_id) for limit enforcement
  - [ ] 2.5 Update list query to return pinned first, then by published_at

- [ ] Task 3: Backend API Handlers (AC: 1, 2, 3)
  - [ ] 3.1 Create POST `/api/v1/announcements/{id}/pin` handler (manager only)
  - [ ] 3.2 Create POST `/api/v1/announcements/{id}/unpin` handler (manager only)
  - [ ] 3.3 Add validation for 3 pinned limit with clear error message
  - [ ] 3.4 Update list endpoint to sort pinned first

- [ ] Task 4: TypeSpec API Specification (AC: 1, 2, 3)
  - [ ] 4.1 Update Announcement model with pinned fields
  - [ ] 4.2 Add PinAnnouncementResponse model
  - [ ] 4.3 Document pin/unpin endpoints with OpenAPI annotations

- [ ] Task 5: Frontend Components - ppt-web (AC: 1, 2, 3)
  - [ ] 5.1 Update AnnouncementCard with pinned visual indicator (already has basic icon)
  - [ ] 5.2 Update AnnouncementList to show pinned section at top
  - [ ] 5.3 Add pin/unpin toggle button for managers
  - [ ] 5.4 Show error toast when 3 pinned limit reached
  - [ ] 5.5 Add "Pinned" badge/section header

- [ ] Task 6: Frontend State & API Integration (AC: 1, 2, 3)
  - [ ] 6.1 Create usePinAnnouncement mutation hook
  - [ ] 6.2 Create useUnpinAnnouncement mutation hook
  - [ ] 6.3 Update announcement list queries to handle pinned sorting

- [ ] Task 7: Integration Testing (AC: 1, 2, 3)
  - [ ] 7.1 Write backend tests for pin/unpin operations
  - [ ] 7.2 Write backend tests for 3 pinned limit enforcement
  - [ ] 7.3 Write backend tests for pinned sorting in list
  - [ ] 7.4 Write backend tests for manager-only authorization

## Dev Notes

### Architecture Requirements
- Follow multi-tenancy pattern: all queries MUST include TenantContext
- Only managers can pin/unpin announcements
- Pinned limit is per organization (3 max)
- Auto-unpin after 30 days is a future enhancement (not in this story)

### Technical Specifications
- Database: PostgreSQL, add columns to existing announcements table
- Backend: Rust + Axum handlers extending announcements module
- Sorting: ORDER BY pinned_at DESC NULLS LAST, published_at DESC

### Visual Design
- Pinned announcements show pin icon (already implemented in AnnouncementCard)
- Separate "Pinned" section at top of list
- Different background color or border for pinned items

### Dependencies
- Story 6.1: Announcement infrastructure (announcements table must exist)

### References
- [Source: _bmad-output/epics.md#Epic-6-Story-6.4]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

N/A

### Completion Notes List

(To be filled during development)

### File List

(To be filled during development)
