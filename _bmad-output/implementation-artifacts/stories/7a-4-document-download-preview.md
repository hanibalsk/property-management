# Story 7A.4: Document Download & Preview

Status: ready-for-dev

## Story

As a **user**,
I want to **preview and download documents**,
So that **I can view content without leaving the app**.

## Acceptance Criteria

1. **AC-1: PDF Preview**
   - Given a user opens a PDF document
   - When they click to view
   - Then an inline preview is displayed
   - And download button is available

2. **AC-2: Image Preview**
   - Given a user opens an image
   - When they click to view
   - Then the image is displayed in a lightbox
   - And can be downloaded

3. **AC-3: Unsupported Preview**
   - Given a document type doesn't support preview
   - When user clicks to view
   - Then download is triggered directly

## Tasks / Subtasks

- [ ] Task 1: Backend API Handlers (AC: 1, 2, 3)
  - [ ] 1.1 Create GET `/api/v1/documents/{id}/download` handler returning presigned URL
  - [ ] 1.2 Create GET `/api/v1/documents/{id}/preview` handler for preview URL (1h expiration)
  - [ ] 1.3 Add download tracking (optional): log download events
  - [ ] 1.4 Implement permission check before generating URLs

- [ ] Task 2: S3 Integration (AC: 1, 2, 3)
  - [ ] 2.1 Add presigned URL generation to storage integration crate
  - [ ] 2.2 Configure separate expiration for preview (1h) and download (15min)
  - [ ] 2.3 Handle content-disposition for download vs inline view

- [ ] Task 3: TypeSpec API Specification (AC: 1, 2, 3)
  - [ ] 3.1 Define DownloadUrlResponse with url and expires_at
  - [ ] 3.2 Define PreviewUrlResponse
  - [ ] 3.3 Document endpoints with OpenAPI annotations

- [ ] Task 4: Frontend Components - ppt-web (AC: 1, 2, 3)
  - [ ] 4.1 Create DocumentPreviewModal component
  - [ ] 4.2 Integrate PDF.js for inline PDF preview
  - [ ] 4.3 Create ImageLightbox component for images
  - [ ] 4.4 Add DownloadButton component with loading state
  - [ ] 4.5 Handle unsupported types with direct download

- [ ] Task 5: Frontend State & API Integration (AC: 1, 2, 3)
  - [ ] 5.1 Create useDocumentDownload hook
  - [ ] 5.2 Create useDocumentPreview hook
  - [ ] 5.3 Add preview modal state management

- [ ] Task 6: Integration Testing (AC: 1, 2, 3)
  - [ ] 6.1 Write backend tests for presigned URL generation
  - [ ] 6.2 Write backend tests for permission check on download
  - [ ] 6.3 Test URL expiration behavior

## Dev Notes

### Architecture Requirements
- Presigned URLs for secure, temporary access
- Preview URLs expire in 1 hour
- Download URLs expire in 15 minutes
- Content-disposition: inline for preview, attachment for download

### Technical Specifications
- S3 presigned URL generation via AWS SDK
- PDF.js for browser-based PDF rendering
- Image preview in modal/lightbox

### Supported Preview Types
- PDF: Inline viewer with PDF.js
- Images (PNG, JPG, GIF): Lightbox modal
- Others (DOC, XLS, TXT): Direct download only

### Security Considerations
- Permission check BEFORE generating presigned URL
- URLs are time-limited
- Log access for audit trail

### Project Structure Notes

**Backend files to create/modify:**
- `backend/crates/integrations/storage/src/s3.rs` (add presigned URL)
- `backend/servers/api-server/src/routes/documents.rs` (add download/preview handlers)

**Frontend files to create:**
- `frontend/apps/ppt-web/src/features/documents/components/DocumentPreviewModal.tsx`
- `frontend/apps/ppt-web/src/features/documents/components/ImageLightbox.tsx`
- `frontend/apps/ppt-web/src/features/documents/components/PdfViewer.tsx`
- `frontend/apps/ppt-web/src/features/documents/hooks/useDocumentPreview.ts`

### References

- [Source: _bmad-output/epics.md#Epic-7A-Story-7A.4]
- [Source: Story 7A.1 for document structure]

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
