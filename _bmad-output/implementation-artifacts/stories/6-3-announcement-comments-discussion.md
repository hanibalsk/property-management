# Story 6.3: Announcement Comments & Discussion

Status: ready-for-dev

## Story

As a **resident**,
I want to **comment on announcements**,
so that **I can ask questions or provide feedback**.

## Acceptance Criteria

1. **AC-1: Add Comment**
   - Given comments are enabled on an announcement
   - When a resident adds a comment
   - Then the comment is posted
   - And author and announcement creator are notified

2. **AC-2: Threaded Replies**
   - Given a comment exists
   - When another resident replies
   - Then a threaded reply is created
   - And parent comment author is notified

3. **AC-3: Comments Disabled**
   - Given comments are disabled
   - When a resident views the announcement
   - Then no comment input is shown
   - And reason is displayed ("Comments closed")

## Tasks / Subtasks

- [ ] Task 1: Database Schema & Migrations (AC: 1, 2, 3)
  - [ ] 1.1 Create `announcement_comments` table: id (UUID), announcement_id (FK), user_id (FK), parent_id (UUID NULL for replies), content (TEXT), ai_training_consent (BOOLEAN DEFAULT false), created_at, updated_at, deleted_at (soft delete)
  - [ ] 1.2 Add RLS policies for tenant isolation
  - [ ] 1.3 Add indexes: idx_comments_announcement, idx_comments_parent, idx_comments_user
  - [ ] 1.4 Add `comments_enabled` boolean column to announcements table if not exists

- [ ] Task 2: Backend Domain Models & Repository (AC: 1, 2, 3)
  - [ ] 2.1 Create AnnouncementComment model with all fields
  - [ ] 2.2 Create AnnouncementCommentRepository with CRUD operations
  - [ ] 2.3 Implement get_comments_for_announcement(announcement_id) with threading
  - [ ] 2.4 Implement create_comment(announcement_id, user_id, content, parent_id?)
  - [ ] 2.5 Implement delete_comment(comment_id, user_id) - soft delete, author only
  - [ ] 2.6 Implement get_comment_count(announcement_id)

- [ ] Task 3: Backend API Handlers (AC: 1, 2, 3)
  - [ ] 3.1 Create GET `/api/v1/announcements/{id}/comments` handler with pagination
  - [ ] 3.2 Create POST `/api/v1/announcements/{id}/comments` handler
  - [ ] 3.3 Create DELETE `/api/v1/announcements/{id}/comments/{commentId}` handler
  - [ ] 3.4 Add comment moderation endpoint for managers: DELETE with reason
  - [ ] 3.5 Update announcement response to include comment_count

- [ ] Task 4: TypeSpec API Specification (AC: 1, 2, 3)
  - [ ] 4.1 Add AnnouncementComment model to TypeSpec
  - [ ] 4.2 Add CreateCommentRequest and CommentResponse models
  - [ ] 4.3 Add ThreadedCommentsResponse for nested structure
  - [ ] 4.4 Document all endpoints with OpenAPI annotations

- [ ] Task 5: Frontend Components - ppt-web (AC: 1, 2, 3)
  - [ ] 5.1 Create CommentForm component for adding comments
  - [ ] 5.2 Create CommentItem component with reply button
  - [ ] 5.3 Create CommentThread component for nested display
  - [ ] 5.4 Create CommentList component with load more pagination
  - [ ] 5.5 Add "Comments closed" message when disabled
  - [ ] 5.6 Add AI consent checkbox to comment form

- [ ] Task 6: Frontend State & API Integration (AC: 1, 2, 3)
  - [ ] 6.1 Create useComments hook with TanStack Query
  - [ ] 6.2 Create useCreateComment mutation hook
  - [ ] 6.3 Create useDeleteComment mutation hook
  - [ ] 6.4 Implement optimistic updates for comment creation

- [ ] Task 7: Integration Testing (AC: 1, 2, 3)
  - [ ] 7.1 Write backend tests for comment CRUD
  - [ ] 7.2 Write backend tests for threaded replies
  - [ ] 7.3 Write backend tests for comment moderation
  - [ ] 7.4 Write backend tests for comments disabled scenario

## Dev Notes

### Architecture Requirements
- Follow multi-tenancy pattern: all queries MUST include TenantContext
- Comments are soft-deleted to preserve thread integrity
- Only comment author can delete their own comments
- Managers can moderate (delete with reason) any comment

### Technical Specifications
- Database: PostgreSQL with RLS policies
- Backend: Rust + Axum handlers in new comments module
- Threading: parent_id NULL = top-level, parent_id set = reply
- Max nesting depth: 2 levels (comment → reply)

### AI Consent
- ai_training_consent flag stored per comment
- Default false, user must opt-in
- Used for future AI training features

### Dependencies
- Story 6.1: Announcement infrastructure
- Epic 2B: Notification infrastructure (for comment notifications - deferred)

### References
- [Source: _bmad-output/epics.md#Epic-6-Story-6.3]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

N/A

### Completion Notes List

(To be filled during development)

### File List

(To be filled during development)
