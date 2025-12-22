# Story 7B.1: Document Versioning & History

## Story

As a **property manager**,
I want to **track document versions and view history**,
So that **I can see changes over time and restore previous versions**.

## Status

in-progress

## Acceptance Criteria

1. **Given** a user uploads a new version of an existing document
   **When** the upload completes
   **Then** the new version is saved with version number incremented
   **And** previous versions remain accessible in version history
   **And** the document list shows the latest version by default

2. **Given** a user views document version history
   **When** they select a previous version
   **Then** they can view or download that specific version
   **And** see who uploaded it and when

3. **Given** a user restores a previous version
   **When** they confirm the restoration
   **Then** the old version becomes the new current version
   **And** a new version entry is created (not destructive)

## Tasks/Subtasks

- [ ] **Task 1: Database Schema for Document Versions**
  - [ ] Create migration 00035 to extend documents table with version_number column
  - [ ] Add parent_document_id column for version chain linking
  - [ ] Add is_current_version column to identify latest version
  - [ ] Add indexes for version queries
  - [ ] Create document_versions view for history queries

- [ ] **Task 2: Update Document Models**
  - [ ] Add version fields to Document model (version_number, parent_document_id, is_current_version)
  - [ ] Create DocumentVersion struct for version history responses
  - [ ] Create DocumentVersionHistory struct for full version list
  - [ ] Add versioning-related request/response types

- [ ] **Task 3: Repository Methods for Versioning**
  - [ ] Implement create_version() - upload new version of existing doc
  - [ ] Implement get_version_history() - list all versions of a document
  - [ ] Implement get_version() - get specific version by ID
  - [ ] Implement restore_version() - create new current version from old one

- [ ] **Task 4: API Endpoints for Versioning**
  - [ ] POST /api/v1/documents/:id/versions - Upload new version
  - [ ] GET /api/v1/documents/:id/versions - Get version history
  - [ ] GET /api/v1/documents/:id/versions/:version_id - Get specific version
  - [ ] POST /api/v1/documents/:id/versions/:version_id/restore - Restore version

- [ ] **Task 5: Tests for Document Versioning**
  - [ ] Unit tests for version model logic
  - [ ] Repository tests for version CRUD operations
  - [ ] Integration tests for version API endpoints

## Dev Notes

### Technical Specifications
- Extend `documents` table: add version_number (default 1), parent_document_id (nullable for version chains)
- Create `document_versions` view for history queries
- S3: each version is a separate object (no overwrite)
- Version chain: parent_document_id points to the original document (first version)

### Architecture Requirements
- Follow existing Axum patterns from document routes
- Use repository pattern matching existing DocumentRepository
- Apply RLS policies for tenant isolation
- Use existing error handling patterns

### Previous Learnings
- Use proper JSONB handling for metadata as seen in existing document code
- Follow migration naming convention: 00035_*.sql
- Ensure RLS policies are consistent with existing document policies

## Dev Agent Record

### Implementation Plan
*To be filled during implementation*

### Debug Log
*To be filled during implementation*

### Completion Notes
*To be filled upon completion*

## File List

*To be filled during implementation*

## Change Log

| Date | Change | Author |
|------|--------|--------|
| 2025-12-22 | Story created | AI Agent |
