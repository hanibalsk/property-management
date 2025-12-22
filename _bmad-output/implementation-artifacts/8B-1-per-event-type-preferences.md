# Story 8B.1: Per-Event Type Preferences

## Story

As a **user**,
I want to **control notifications for each event type individually**,
So that **I only receive notifications I care about**.

## Status

in-progress

## Acceptance Criteria

1. **Given** a user opens notification preferences
   **When** they view the event type list
   **Then** they see all event categories (faults, votes, announcements, documents, etc.)
   **And** can toggle each independently

2. **Given** a user disables "New Fault" notifications
   **When** a new fault is created in their building
   **Then** they don't receive push/email/in-app for that event
   **And** other users with notifications enabled still receive them

3. **Given** a new event type is added to the system
   **When** the user views preferences
   **Then** the new type appears with sensible defaults (enabled)
   **And** user can adjust as needed

## Tasks/Subtasks

- [ ] **Task 1: Database Schema Updates**
  - [ ] Create migration 00038 to extend notification_preferences
  - [ ] Add event_types JSONB column with per-type settings
  - [ ] Create default schema with all event types

- [ ] **Task 2: Update Models**
  - [ ] Create EventTypePreference struct
  - [ ] Create default event types configuration
  - [ ] Add methods to get/set individual event preferences

- [ ] **Task 3: Repository Methods**
  - [ ] Implement get_event_type_preferences()
  - [ ] Implement update_event_type_preference()
  - [ ] Implement reset_event_type_preferences()

- [ ] **Task 4: API Endpoints**
  - [ ] GET /api/v1/users/me/notification-preferences/events - List event preferences
  - [ ] PUT /api/v1/users/me/notification-preferences/events/:event_type - Update single event
  - [ ] POST /api/v1/users/me/notification-preferences/events/reset - Reset to defaults

## Dev Notes

### Event Types
- fault.created, fault.updated, fault.resolved
- vote.created, vote.reminder, vote.closed
- announcement.posted, announcement.urgent
- document.shared, document.signature_request
- message.received, message.thread_reply
- critical.emergency, critical.maintenance

### Technical Specifications
- Default schema: all event types enabled
- Migration auto-populates for existing users
- New event types added dynamically with default=true

## Dev Agent Record

### Implementation Plan
*To be filled during implementation*

## File List

*To be filled during implementation*

## Change Log

| Date | Change | Author |
|------|--------|--------|
| 2025-12-22 | Story created | AI Agent |
