# Story 7A.3: Permission-Based Document Access

Status: ready-for-dev

## Story

As a **user**,
I want to **view documents based on my permissions**,
So that **I only see what I'm authorized to access**.

## Acceptance Criteria

1. **AC-1: Building Scope Access**
   - Given a document is uploaded with "Building X" scope
   - When a resident of Building X views documents
   - Then they can see and download the document

2. **AC-2: Role-Based Access**
   - Given a document has "Owners only" permission
   - When a tenant tries to view it
   - Then the document is not visible in their list

3. **AC-3: Specific User Sharing**
   - Given a document is shared with specific users
   - When those users access documents
   - Then the shared document appears in their list

## Tasks / Subtasks

- [ ] Task 1: Database Schema & Migrations (AC: 1, 2, 3)
  - [ ] 1.1 Add access_scope column to documents table: ENUM (organization, building, unit, role, users)
  - [ ] 1.2 Add access_target_ids column (JSONB array) for building IDs, unit IDs, or user IDs
  - [ ] 1.3 Add access_roles column (JSONB array) for role-based access
  - [ ] 1.4 Update RLS policies to filter by access scope and user context

- [ ] Task 2: Backend Domain Models & Repository (AC: 1, 2, 3)
  - [ ] 2.1 Create AccessScope enum: Organization, Building, Unit, Role, Users
  - [ ] 2.2 Update Document model with access fields
  - [ ] 2.3 Implement access check methods in DocumentRepository
  - [ ] 2.4 Add query method: find_accessible_by_user with full permission check

- [ ] Task 3: Backend API Handlers (AC: 1, 2, 3)
  - [ ] 3.1 Update POST `/api/v1/documents` to accept access_scope and targets
  - [ ] 3.2 Update GET `/api/v1/documents` to filter by user permissions
  - [ ] 3.3 Add permission check middleware for document access
  - [ ] 3.4 Create PUT `/api/v1/documents/{id}/access` handler for updating permissions

- [ ] Task 4: TypeSpec API Specification (AC: 1, 2, 3)
  - [ ] 4.1 Define AccessScope enum in TypeSpec
  - [ ] 4.2 Update Document model with access fields
  - [ ] 4.3 Define UpdateDocumentAccessRequest DTO

- [ ] Task 5: Frontend Components - ppt-web (AC: 1, 2, 3)
  - [ ] 5.1 Create AccessScopeSelector component
  - [ ] 5.2 Create BuildingSelector for building scope
  - [ ] 5.3 Create UserSelector for specific user sharing
  - [ ] 5.4 Create RoleSelector for role-based access
  - [ ] 5.5 Update DocumentUploadForm with access controls

- [ ] Task 6: Frontend State & API Integration (AC: 1, 2, 3)
  - [ ] 6.1 Update useDocuments hook to use filtered API
  - [ ] 6.2 Create useUpdateDocumentAccess mutation hook
  - [ ] 6.3 Add access scope to document creation flow

- [ ] Task 7: Integration Testing (AC: 1, 2, 3)
  - [ ] 7.1 Write backend tests for building scope access
  - [ ] 7.2 Write backend tests for role-based access
  - [ ] 7.3 Write backend tests for specific user sharing
  - [ ] 7.4 Write backend tests for cross-scope denial

## Dev Notes

### Architecture Requirements
- Follow multi-tenancy pattern: all queries MUST include TenantContext
- Access control is additive - user must match at least one criterion
- Organization scope = all users in org can access
- Building scope = users with units in that building
- Unit scope = users assigned to specific units
- Role scope = users with specific roles (Owner, Tenant, Manager)
- Users scope = specific user IDs

### Technical Specifications
- Permission check happens at repository level
- RLS policies handle tenant isolation
- Application logic handles access scope filtering

### Access Resolution Order
1. Check if user is document creator (always has access)
2. Check if access_scope is organization (all org users)
3. Check if user's building matches access_target_ids
4. Check if user's unit matches access_target_ids
5. Check if user's role matches access_roles
6. Check if user's ID is in access_target_ids

### Project Structure Notes

**Backend files to modify:**
- `backend/crates/db/migrations/00022_add_document_access.sql`
- `backend/crates/db/src/models/document.rs`
- `backend/crates/db/src/repositories/document.rs`
- `backend/servers/api-server/src/routes/documents.rs`

**Frontend files to create/modify:**
- `frontend/apps/ppt-web/src/features/documents/components/AccessScopeSelector.tsx`
- `frontend/apps/ppt-web/src/features/documents/components/DocumentUploadForm.tsx` (update)

### References

- [Source: _bmad-output/epics.md#Epic-7A-Story-7A.3]
- [Source: Story 6.1 for target_type pattern]

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
