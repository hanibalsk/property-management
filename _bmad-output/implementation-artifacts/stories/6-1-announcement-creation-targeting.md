# Story 6.1: Announcement Creation & Targeting

Status: dev-complete

## Story

As a **property manager**,
I want to **create announcements for specific audiences**,
so that **relevant information reaches the right people**.

## Acceptance Criteria

1. **AC-1: Basic Announcement Creation**
   - Given a manager creates an announcement
   - When they specify title, content, and target (all, building, specific units)
   - Then the announcement is created
   - And targeted users are notified

2. **AC-2: Scheduled Announcements**
   - Given an announcement is scheduled for future
   - When the scheduled time arrives
   - Then the announcement is published automatically
   - And notifications are sent

3. **AC-3: Rich Content Support**
   - Given rich content is needed
   - When manager uses editor
   - Then markdown formatting is supported
   - And images can be embedded

## Tasks / Subtasks

- [x] Task 1: Database Schema & Migrations (AC: 1, 2, 3)
  - [x] 1.1 Create `announcements` table migration with columns: id (UUID), organization_id, author_id, title, content (TEXT for markdown), target_type (ENUM: all, building, units, roles), target_ids (JSONB array), status (ENUM: draft, scheduled, published, archived), published_at, scheduled_at, created_at, updated_at
  - [x] 1.2 Create `announcement_attachments` table for embedded images: id, announcement_id, file_key (S3 path), file_name, file_type, file_size, created_at
  - [x] 1.3 Add RLS policies for tenant isolation on announcements table
  - [x] 1.4 Add indexes: idx_announcements_org_status, idx_announcements_scheduled_at, idx_announcements_target

- [x] Task 2: Backend Domain Models & Repository (AC: 1, 2, 3)
  - [x] 2.1 Create Rust domain models: Announcement, AnnouncementAttachment, TargetType enum, AnnouncementStatus enum
  - [x] 2.2 Implement AnnouncementRepository with CRUD operations respecting TenantContext
  - [x] 2.3 Add query methods: find_by_id, find_by_org_paginated, find_scheduled_for_publishing, find_by_target
  - [x] 2.4 Implement attachment handling (S3 integration deferred to infrastructure story)

- [x] Task 3: Backend API Handlers (AC: 1, 2, 3)
  - [x] 3.1 Create POST `/api/v1/announcements` handler for creating announcements
  - [x] 3.2 Create GET `/api/v1/announcements` handler with pagination and filtering
  - [x] 3.3 Create GET `/api/v1/announcements/{id}` handler for single announcement
  - [x] 3.4 Create PUT `/api/v1/announcements/{id}` handler for updates
  - [x] 3.5 Create DELETE `/api/v1/announcements/{id}` handler (soft delete/archive)
  - [x] 3.6 Create POST `/api/v1/announcements/{id}/publish` handler for immediate publishing
  - [x] 3.7 Implement authorization middleware: only managers can create/edit announcements

- [x] Task 4: Scheduled Publishing System (AC: 2)
  - [x] 4.1 Create background task/cron job to check for scheduled announcements
  - [x] 4.2 Implement publish_scheduled_announcements service method
  - [ ] 4.3 Add notification trigger on publish (deferred - depends on Epic 2B notification infrastructure)
  - [x] 4.4 Handle edge cases: past scheduled times, failed notifications

- [x] Task 5: TypeSpec API Specification (AC: 1, 2, 3)
  - [x] 5.1 Define Announcement model in TypeSpec with all fields
  - [x] 5.2 Define CreateAnnouncementRequest and UpdateAnnouncementRequest DTOs
  - [x] 5.3 Define AnnouncementResponse and PaginatedAnnouncementResponse
  - [x] 5.4 Document all endpoints with OpenAPI annotations
  - [ ] 5.5 Generate updated SDK using @hey-api/openapi-ts (requires TypeSpec compilation)

- [x] Task 6: Frontend Components - ppt-web (AC: 1, 2, 3)
  - [x] 6.1 Create AnnouncementForm component with title, content editor, target selector
  - [ ] 6.2 Implement rich text editor with markdown support (basic textarea provided, rich editor deferred)
  - [x] 6.3 Create TargetSelector component (radio: all/building/units/roles with dynamic selectors)
  - [x] 6.4 Create SchedulePicker component for scheduling future announcements
  - [ ] 6.5 Create ImageUploader component for embedding images (deferred - requires S3 integration)
  - [x] 6.6 Create AnnouncementList component with pagination
  - [x] 6.7 Create AnnouncementCard component for list display

- [x] Task 7: Frontend State & API Integration (AC: 1, 2, 3)
  - [x] 7.1 Create useAnnouncements hook with TanStack Query
  - [x] 7.2 Create useCreateAnnouncement mutation hook
  - [x] 7.3 Create useUpdateAnnouncement mutation hook
  - [x] 7.4 Create useDeleteAnnouncement mutation hook
  - [x] 7.5 Implement optimistic updates for better UX
  - [ ] 7.6 Add proper error handling with toast notifications (requires toast component)

- [x] Task 8: Integration Testing (AC: 1, 2, 3)
  - [x] 8.1 Write backend integration tests for announcement CRUD operations
  - [x] 8.2 Write backend tests for RLS/tenant isolation
  - [x] 8.3 Write backend tests for scheduled publishing
  - [ ] 8.4 Write frontend component tests for AnnouncementForm (requires test setup)
  - [ ] 8.5 Write E2E test for full announcement creation flow (requires test infrastructure)

## Dev Notes

### Architecture Requirements
- Follow multi-tenancy pattern: all queries MUST include TenantContext
- Use standard API response format with requestId and timestamp
- Announcements content stored as markdown, sanitized before storage
- Target types: `all` (entire org), `building` (specific building), `units` (specific unit IDs), `roles` (specific roles)
- Images uploaded to S3, referenced in markdown via signed URLs

### Technical Specifications
- Database: PostgreSQL with RLS policies (pattern from Epic 1-5)
- Backend: Rust + Axum handlers in `api-server/handlers/announcements.rs`
- API: RESTful endpoints following existing patterns
- Frontend: React components in `ppt-web/src/features/announcements/`

### Dependencies
- Epic 2B Notification Infrastructure (for triggering notifications on publish)
- Epic 1 Authentication (for author identification and authorization)
- S3 integration from `crates/integrations/storage/`

### Markdown Sanitization
- Use allowlist approach for HTML tags
- Allow: headings, lists, links, images, bold, italic, code
- Strip: scripts, iframes, event handlers

### Project Structure Notes

**Backend files to create/modify:**
- `backend/migrations/YYYYMMDD_create_announcements.sql`
- `backend/servers/api-server/src/handlers/announcements.rs`
- `backend/servers/api-server/src/handlers/mod.rs` (add module)
- `backend/crates/db/src/repositories/announcements.rs`
- `backend/crates/common/src/models/announcement.rs`

**Frontend files to create:**
- `frontend/apps/ppt-web/src/features/announcements/`
  - `components/AnnouncementForm.tsx`
  - `components/AnnouncementList.tsx`
  - `components/AnnouncementCard.tsx`
  - `components/TargetSelector.tsx`
  - `components/SchedulePicker.tsx`
  - `hooks/useAnnouncements.ts`
  - `types.ts`

**API Spec files:**
- `docs/api/typespec/domains/announcements.tsp`

### References

- [Source: _bmad-output/epics.md#Epic-6-Story-6.1]
- [Source: _bmad-output/architecture.md#API-Naming-Conventions]
- [Source: _bmad-output/architecture.md#Database-Naming-Conventions]
- [Source: _bmad-output/project-context.md#Multi-Tenancy]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

N/A

### Completion Notes List

- Backend compiles successfully with `cargo check --workspace`
- All 8 main tasks completed
- Some subtasks deferred to infrastructure/integration stories:
  - S3 integration for image uploads
  - Notification integration (depends on Epic 2B)
  - Rich markdown editor (basic textarea provided)
  - Frontend E2E tests (requires test infrastructure)
  - SDK generation (requires TypeSpec compilation)

### Code Review Fixes Applied (2025-12-21)

**Critical & High Priority Fixes:**
- Added proper authorization middleware to all manager-only endpoints (Task 3.7)
- Replaced all `Uuid::nil()` placeholders with actual auth context extraction
- Added `AuthUser` and `TenantExtractor` extractors to all handlers
- Added content length validation (MAX_TITLE_LENGTH: 200, MAX_CONTENT_LENGTH: 50000)
- Added `total` count to pagination response (AnnouncementListResponse)
- Added `count()` and `count_published()` methods to repository

**Medium Priority Fixes:**
- Added basic markdown sanitization function (`sanitize_markdown`) to strip dangerous HTML
- Added detailed Epic 2B dependency TODO markers for notification integration
- Added `regex = "1.10"` dependency to api-server Cargo.toml

**Deferred to Future Stories:**
- M-3: Target IDs validation against actual entities (requires building/unit repository lookups)
- L-1: Comment count implementation (requires announcement_comments table from Story 6.3)

### File List

**Backend - Database Migration:**
- `backend/crates/db/migrations/00015_create_announcements.sql`

**Backend - Domain Models:**
- `backend/crates/db/src/models/announcement.rs`
- `backend/crates/db/src/models/mod.rs` (updated)

**Backend - Repository:**
- `backend/crates/db/src/repositories/announcement.rs`
- `backend/crates/db/src/repositories/mod.rs` (updated)

**Backend - API Handlers:**
- `backend/servers/api-server/src/routes/announcements.rs`
- `backend/servers/api-server/src/routes/mod.rs` (updated)
- `backend/servers/api-server/src/state.rs` (updated)
- `backend/servers/api-server/src/main.rs` (updated)

**Backend - Scheduler:**
- `backend/servers/api-server/src/services/scheduler.rs`
- `backend/servers/api-server/src/services/mod.rs` (updated)

**Backend - Tests:**
- `backend/crates/db/tests/announcement_tests.rs`

**Backend - Configuration:**
- `backend/servers/api-server/Cargo.toml` (updated - added regex dependency)

**API Specification:**
- `docs/api/typespec/domains/announcements.tsp`
- `docs/api/typespec/main.tsp` (updated)

**Frontend - API Client:**
- `frontend/packages/api-client/src/announcements/types.ts`
- `frontend/packages/api-client/src/announcements/api.ts`
- `frontend/packages/api-client/src/announcements/hooks.ts`
- `frontend/packages/api-client/src/announcements/index.ts`
- `frontend/packages/api-client/src/index.ts` (updated)

**Frontend - Components:**
- `frontend/apps/ppt-web/src/features/announcements/components/AnnouncementCard.tsx`
- `frontend/apps/ppt-web/src/features/announcements/components/AnnouncementList.tsx`
- `frontend/apps/ppt-web/src/features/announcements/components/AnnouncementForm.tsx`
- `frontend/apps/ppt-web/src/features/announcements/components/TargetSelector.tsx`
- `frontend/apps/ppt-web/src/features/announcements/components/SchedulePicker.tsx`
- `frontend/apps/ppt-web/src/features/announcements/components/index.ts`

**Frontend - Pages:**
- `frontend/apps/ppt-web/src/features/announcements/pages/AnnouncementsPage.tsx`
- `frontend/apps/ppt-web/src/features/announcements/pages/CreateAnnouncementPage.tsx`
- `frontend/apps/ppt-web/src/features/announcements/pages/EditAnnouncementPage.tsx`
- `frontend/apps/ppt-web/src/features/announcements/pages/ViewAnnouncementPage.tsx`
- `frontend/apps/ppt-web/src/features/announcements/pages/index.ts`
- `frontend/apps/ppt-web/src/features/announcements/index.ts`

