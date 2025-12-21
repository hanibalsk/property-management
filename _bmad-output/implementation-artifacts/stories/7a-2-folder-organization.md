# Story 7A.2: Folder Organization

Status: ready-for-dev

## Story

As a **property manager**,
I want to **organize documents in folders**,
So that **files are easy to find**.

## Acceptance Criteria

1. **AC-1: Folder Creation**
   - Given a manager creates a folder
   - When they specify name and parent folder
   - Then the folder is created
   - And appears in the folder tree

2. **AC-2: Document Move**
   - Given a manager moves a document to a folder
   - When the move is confirmed
   - Then the document's folder reference is updated
   - And it appears in the new location

3. **AC-3: Folder Deletion**
   - Given a folder is deleted
   - When it contains documents
   - Then the user is warned
   - And must choose: move contents or delete all

## Tasks / Subtasks

- [ ] Task 1: Database Schema & Migrations (AC: 1, 2, 3)
  - [ ] 1.1 Create `document_folders` table migration: id (UUID), organization_id, parent_id (nullable, self-reference), name, description, created_by, created_at, updated_at, deleted_at
  - [ ] 1.2 Add foreign key from documents.folder_id to document_folders.id
  - [ ] 1.3 Add RLS policies for tenant isolation
  - [ ] 1.4 Add indexes: idx_folders_org_parent, idx_folders_name
  - [ ] 1.5 Add check constraint for max folder depth (5 levels)

- [ ] Task 2: Backend Domain Models & Repository (AC: 1, 2, 3)
  - [ ] 2.1 Create Rust domain models: DocumentFolder, CreateFolder, UpdateFolder
  - [ ] 2.2 Implement FolderRepository with CRUD operations
  - [ ] 2.3 Add query methods: find_by_id, find_by_parent, get_folder_tree, count_documents_in_folder
  - [ ] 2.4 Add folder depth validation (max 5 levels)

- [ ] Task 3: Backend API Handlers (AC: 1, 2, 3)
  - [ ] 3.1 Create POST `/api/v1/documents/folders` handler for folder creation
  - [ ] 3.2 Create GET `/api/v1/documents/folders` handler for folder tree
  - [ ] 3.3 Create GET `/api/v1/documents/folders/{id}` handler for folder details
  - [ ] 3.4 Create PUT `/api/v1/documents/folders/{id}` handler for updates
  - [ ] 3.5 Create DELETE `/api/v1/documents/folders/{id}` handler with cascade option
  - [ ] 3.6 Create POST `/api/v1/documents/{id}/move` handler for moving documents

- [ ] Task 4: TypeSpec API Specification (AC: 1, 2, 3)
  - [ ] 4.1 Define DocumentFolder model in TypeSpec
  - [ ] 4.2 Define CreateFolderRequest and UpdateFolderRequest DTOs
  - [ ] 4.3 Define MoveDocumentRequest DTO
  - [ ] 4.4 Document all endpoints with OpenAPI annotations

- [ ] Task 5: Frontend Components - ppt-web (AC: 1, 2, 3)
  - [ ] 5.1 Create FolderTree component for hierarchical navigation
  - [ ] 5.2 Create CreateFolderDialog component
  - [ ] 5.3 Create MoveDocumentDialog component with folder picker
  - [ ] 5.4 Create DeleteFolderDialog with options (move contents vs delete all)
  - [ ] 5.5 Add folder breadcrumb navigation

- [ ] Task 6: Frontend State & API Integration (AC: 1, 2, 3)
  - [ ] 6.1 Create useFolders hook with TanStack Query
  - [ ] 6.2 Create useCreateFolder mutation hook
  - [ ] 6.3 Create useDeleteFolder mutation hook
  - [ ] 6.4 Create useMoveDocument mutation hook

- [ ] Task 7: Integration Testing (AC: 1, 2, 3)
  - [ ] 7.1 Write backend integration tests for folder CRUD operations
  - [ ] 7.2 Write backend tests for folder depth validation
  - [ ] 7.3 Write backend tests for document move operations
  - [ ] 7.4 Write backend tests for cascade delete

## Dev Notes

### Architecture Requirements
- Follow multi-tenancy pattern: all queries MUST include TenantContext
- Folders are organization-scoped, not building-scoped
- Maximum folder depth: 5 levels
- Soft delete with optional cascade

### Technical Specifications
- Database: PostgreSQL with RLS policies
- Self-referential foreign key for parent_id
- Backend: Rust + Axum handlers
- Frontend: React components with tree visualization

### Folder Depth Calculation
- Use recursive CTE to validate depth before insert
- Return error if new folder would exceed 5 levels

### Project Structure Notes

**Backend files to create/modify:**
- `backend/crates/db/migrations/00021_create_document_folders.sql`
- `backend/crates/db/src/repositories/document_folder.rs`
- `backend/crates/db/src/models/document_folder.rs`
- `backend/servers/api-server/src/routes/documents.rs` (add folder routes)

**Frontend files to create:**
- `frontend/apps/ppt-web/src/features/documents/components/FolderTree.tsx`
- `frontend/apps/ppt-web/src/features/documents/components/CreateFolderDialog.tsx`
- `frontend/apps/ppt-web/src/features/documents/components/MoveDocumentDialog.tsx`

### References

- [Source: _bmad-output/epics.md#Epic-7A-Story-7A.2]
- [Source: Story 7A.1 for document table reference]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

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
