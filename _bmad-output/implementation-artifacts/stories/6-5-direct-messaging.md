# Story 6.5: Direct Messaging

Status: ready-for-dev

## Story

As a **resident**,
I want to **send messages to other users**,
so that **I can communicate privately about building matters**.

## Acceptance Criteria

1. **AC-1: Start Conversation**
   - Given a resident starts a conversation
   - When they select a recipient within their org
   - Then a direct message thread is created
   - And recipient is notified

2. **AC-2: Message Exchange**
   - Given messages are exchanged
   - When either party views the thread
   - Then messages appear in chronological order
   - And read receipts show when messages were seen

3. **AC-3: Block User**
   - Given a user wants privacy
   - When they block another user
   - Then blocked user cannot send messages
   - And blocker doesn't see messages from blocked user

## Tasks / Subtasks

- [ ] Task 1: Database Schema & Migrations (AC: 1, 2, 3)
  - [ ] 1.1 Create `message_threads` table: id (UUID), organization_id (FK), participant_ids (JSONB array of user IDs), last_message_at (TIMESTAMP), created_at
  - [ ] 1.2 Create `messages` table: id (UUID), thread_id (FK), sender_id (FK), content (TEXT), read_at (TIMESTAMP NULL), created_at, deleted_at (soft delete)
  - [ ] 1.3 Create `user_blocks` table: id (UUID), blocker_id (FK), blocked_id (FK), created_at
  - [ ] 1.4 Add RLS policies for all tables
  - [ ] 1.5 Add indexes for efficient queries

- [ ] Task 2: Backend Domain Models & Repository (AC: 1, 2, 3)
  - [ ] 2.1 Create MessageThread model
  - [ ] 2.2 Create Message model
  - [ ] 2.3 Create UserBlock model
  - [ ] 2.4 Implement MessageThreadRepository with CRUD operations
  - [ ] 2.5 Implement MessageRepository with CRUD operations
  - [ ] 2.6 Implement UserBlockRepository
  - [ ] 2.7 Implement get_or_create_thread(user_id, recipient_id) method
  - [ ] 2.8 Implement is_blocked(user_id, other_user_id) check

- [ ] Task 3: Backend API Handlers (AC: 1, 2, 3)
  - [ ] 3.1 Create GET `/api/v1/messages/threads` handler - list conversations
  - [ ] 3.2 Create POST `/api/v1/messages/threads` handler - start conversation
  - [ ] 3.3 Create GET `/api/v1/messages/threads/{id}` handler - get thread with messages
  - [ ] 3.4 Create POST `/api/v1/messages/threads/{id}/messages` handler - send message
  - [ ] 3.5 Create POST `/api/v1/messages/threads/{id}/read` handler - mark as read
  - [ ] 3.6 Create POST `/api/v1/users/{id}/block` handler
  - [ ] 3.7 Create DELETE `/api/v1/users/{id}/block` handler - unblock
  - [ ] 3.8 Create GET `/api/v1/users/blocked` handler - list blocked users

- [ ] Task 4: TypeSpec API Specification (AC: 1, 2, 3)
  - [ ] 4.1 Add MessageThread model to TypeSpec
  - [ ] 4.2 Add Message model to TypeSpec
  - [ ] 4.3 Add CreateThreadRequest, SendMessageRequest models
  - [ ] 4.4 Add ThreadListResponse, ThreadDetailResponse
  - [ ] 4.5 Document all endpoints with OpenAPI annotations

- [ ] Task 5: Frontend Components - ppt-web (AC: 1, 2, 3)
  - [ ] 5.1 Create ThreadList component showing conversations
  - [ ] 5.2 Create ThreadItem component with last message preview
  - [ ] 5.3 Create MessageThread component for viewing conversation
  - [ ] 5.4 Create MessageBubble component for individual messages
  - [ ] 5.5 Create MessageInput component for sending
  - [ ] 5.6 Create NewThreadModal for starting conversations
  - [ ] 5.7 Create BlockedUsersPage for managing blocks

- [ ] Task 6: Frontend State & API Integration (AC: 1, 2, 3)
  - [ ] 6.1 Create useThreads hook with TanStack Query
  - [ ] 6.2 Create useThread hook for single thread with messages
  - [ ] 6.3 Create useSendMessage mutation hook
  - [ ] 6.4 Create useBlockUser, useUnblockUser mutation hooks
  - [ ] 6.5 Create useBlockedUsers query hook
  - [ ] 6.6 Implement optimistic updates for sending messages

- [ ] Task 7: Frontend Pages (AC: 1, 2, 3)
  - [ ] 7.1 Create MessagesPage with thread list and selected thread view
  - [ ] 7.2 Add messages link to navigation
  - [ ] 7.3 Create responsive layout (list + detail on desktop, separate on mobile)

- [ ] Task 8: Integration Testing (AC: 1, 2, 3)
  - [ ] 8.1 Write backend tests for thread creation
  - [ ] 8.2 Write backend tests for message sending
  - [ ] 8.3 Write backend tests for read receipts
  - [ ] 8.4 Write backend tests for blocking functionality
  - [ ] 8.5 Write backend tests for RLS isolation

## Dev Notes

### Architecture Requirements
- Follow multi-tenancy pattern: all queries MUST include TenantContext
- Users can only message others in the same organization
- Messages are soft-deleted
- Blocked users cannot send messages or see blocker's profile

### Technical Specifications
- Database: PostgreSQL with RLS policies
- Backend: Rust + Axum handlers in new messages module
- Thread participants stored as JSONB array for flexibility

### Real-time Considerations
- Initial implementation uses polling for new messages
- WebSocket support deferred to future enhancement
- Read receipts updated on thread view

### Privacy
- Respect user visibility settings from profile
- Block list is private (blocked user doesn't know they're blocked)

### Dependencies
- Epic 1: User authentication (user_id references)
- Epic 2B: Notification infrastructure (deferred)

### References
- [Source: _bmad-output/epics.md#Epic-6-Story-6.5]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

N/A

### Completion Notes List

(To be filled during development)

### File List

(To be filled during development)
