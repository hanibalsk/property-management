# Story 7A.1: Document Upload with Metadata

Status: ready-for-dev

## Story

As a **user**,
I want to **upload documents with metadata**,
So that **important files are stored and categorized**.

## Acceptance Criteria

1. **AC-1: Document Upload**
   - Given a user uploads a document
   - When they select a file and add title, category
   - Then the document is uploaded to storage
   - And metadata is saved
   - And the document appears in the list

2. **AC-2: Size Limit Validation**
   - Given a document exceeds size limit (50MB)
   - When upload is attempted
   - Then user sees error with size limit info
   - And upload is rejected

3. **AC-3: File Type Validation**
   - Given an unsupported file type is uploaded
   - When the file is submitted
   - Then user sees list of supported formats
   - And upload is rejected

## Tasks / Subtasks

- [ ] Task 1: Database Schema & Migrations (AC: 1, 2, 3)
  - [ ] 1.1 Create `documents` table migration with columns: id (UUID), organization_id, folder_id (nullable), title, description, category, file_key (S3 path), file_name, mime_type, size_bytes, created_by, created_at, updated_at, deleted_at (soft delete)
  - [ ] 1.2 Create `document_categories` enum type: contracts, invoices, reports, manuals, certificates, other
  - [ ] 1.3 Add RLS policies for tenant isolation on documents table
  - [ ] 1.4 Add indexes: idx_documents_org_folder, idx_documents_category, idx_documents_created_by

- [ ] Task 2: Backend Domain Models & Repository (AC: 1, 2, 3)
  - [ ] 2.1 Create Rust domain models: Document, DocumentCategory enum, CreateDocument, UpdateDocument
  - [ ] 2.2 Implement DocumentRepository with CRUD operations respecting TenantContext
  - [ ] 2.3 Add query methods: find_by_id, find_by_org_paginated, find_by_folder, find_by_category
  - [ ] 2.4 Add validation constants: MAX_FILE_SIZE (50MB), ALLOWED_MIME_TYPES

- [ ] Task 3: Backend API Handlers (AC: 1, 2, 3)
  - [ ] 3.1 Create POST `/api/v1/documents` handler for creating document records (metadata only)
  - [ ] 3.2 Create GET `/api/v1/documents` handler with pagination and filtering
  - [ ] 3.3 Create GET `/api/v1/documents/{id}` handler for single document
  - [ ] 3.4 Create PUT `/api/v1/documents/{id}` handler for metadata updates
  - [ ] 3.5 Create DELETE `/api/v1/documents/{id}` handler (soft delete)
  - [ ] 3.6 Add file size and type validation before accepting uploads

- [ ] Task 4: TypeSpec API Specification (AC: 1, 2, 3)
  - [ ] 4.1 Define Document model in TypeSpec with all fields
  - [ ] 4.2 Define CreateDocumentRequest and UpdateDocumentRequest DTOs
  - [ ] 4.3 Define DocumentResponse and PaginatedDocumentResponse
  - [ ] 4.4 Document all endpoints with OpenAPI annotations

- [ ] Task 5: Frontend Components - ppt-web (AC: 1, 2, 3)
  - [ ] 5.1 Create DocumentUploadForm component with title, description, category selector
  - [ ] 5.2 Create FileDropzone component for drag-and-drop upload
  - [ ] 5.3 Create DocumentList component with pagination
  - [ ] 5.4 Create DocumentCard component for list display
  - [ ] 5.5 Add file size and type validation on client side

- [ ] Task 6: Frontend State & API Integration (AC: 1, 2, 3)
  - [ ] 6.1 Create useDocuments hook with TanStack Query
  - [ ] 6.2 Create useCreateDocument mutation hook
  - [ ] 6.3 Create useUpdateDocument mutation hook
  - [ ] 6.4 Create useDeleteDocument mutation hook

- [ ] Task 7: Integration Testing (AC: 1, 2, 3)
  - [ ] 7.1 Write backend integration tests for document CRUD operations
  - [ ] 7.2 Write backend tests for RLS/tenant isolation
  - [ ] 7.3 Write backend tests for file size and type validation

## Dev Notes

### Architecture Requirements
- Follow multi-tenancy pattern: all queries MUST include TenantContext
- Use standard API response format with requestId and timestamp
- Document metadata stored in PostgreSQL, file content in S3 (integration deferred)
- Categories: contracts, invoices, reports, manuals, certificates, other

### Technical Specifications
- Database: PostgreSQL with RLS policies (pattern from Epic 1-6)
- Backend: Rust + Axum handlers in `api-server/handlers/documents.rs`
- API: RESTful endpoints following existing patterns
- Frontend: React components in `ppt-web/src/features/documents/`

### File Size Limits
- Maximum file size: 50MB
- Supported formats: PDF, DOC, DOCX, XLS, XLSX, PNG, JPG, JPEG, GIF, TXT

### Project Structure Notes

**Backend files to create/modify:**
- `backend/crates/db/migrations/00020_create_documents.sql`
- `backend/servers/api-server/src/routes/documents.rs`
- `backend/servers/api-server/src/routes/mod.rs` (add module)
- `backend/crates/db/src/repositories/document.rs`
- `backend/crates/db/src/models/document.rs`

**Frontend files to create:**
- `frontend/apps/ppt-web/src/features/documents/`
  - `components/DocumentUploadForm.tsx`
  - `components/DocumentList.tsx`
  - `components/DocumentCard.tsx`
  - `components/FileDropzone.tsx`
  - `hooks/useDocuments.ts`
  - `types.ts`

**API Spec files:**
- `docs/api/typespec/domains/documents.tsp`

### References

- [Source: _bmad-output/epics.md#Epic-7A-Story-7A.1]
- [Source: _bmad-output/architecture.md#API-Naming-Conventions]
- [Source: _bmad-output/project-context.md#Multi-Tenancy]

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
