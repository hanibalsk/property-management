# Story 7B.3: E-Signature Integration

## Story

As a **property manager**,
I want to **send documents for electronic signature**,
So that **I can get legally binding signatures without paper**.

## Status

done

## Acceptance Criteria

1. **Given** a manager initiates e-signature on a document
   **When** they specify signers (email addresses)
   **Then** an e-signature request is created
   **And** signers receive email invitations to sign

2. **Given** a signer completes their signature
   **When** all signers have signed
   **Then** the document is marked as fully signed
   **And** a signed copy is stored in documents
   **And** all parties receive confirmation

3. **Given** a signature request is pending
   **When** the manager checks status
   **Then** they see which signers have/haven't signed
   **And** can send reminders to pending signers

## Tasks/Subtasks

- [x] **Task 1: Database Schema for E-Signatures**
  - [x] Create migration 00037 for signature_requests table
  - [x] Add signature_request_status enum (pending, in_progress, completed, declined, expired, cancelled)
  - [x] Add signers JSONB column for signer details and status
  - [x] Add provider_request_id for external integration tracking
  - [x] Add audit trail fields (created_at, completed_at, expires_at)

- [x] **Task 2: Update Models**
  - [x] Create SignatureRequest model
  - [x] Create Signer struct for individual signer tracking
  - [x] Create SignatureRequestStatus enum
  - [x] Create request/response types for API

- [x] **Task 3: Repository Methods**
  - [x] Implement create_signature_request()
  - [x] Implement find_signature_request_by_id()
  - [x] Implement find_signature_requests_by_document()
  - [x] Implement update_signer_status()
  - [x] Implement complete_signature_request()
  - [x] Implement cancel_signature_request()
  - [x] Implement list_pending_signature_requests()

- [x] **Task 4: API Endpoints**
  - [x] POST /api/v1/documents/:id/signature-requests - Create signature request
  - [x] GET /api/v1/documents/:id/signature-requests - List signature requests for document
  - [x] GET /api/v1/signature-requests/:id - Get signature request details
  - [x] POST /api/v1/signature-requests/:id/remind - Send reminder to pending signers
  - [x] POST /api/v1/signature-requests/:id/cancel - Cancel signature request
  - [x] POST /api/v1/signature-requests/:id/webhook - Handle provider webhook (signature events)

## Dev Notes

### Technical Specifications
- Integration abstracted to support DocuSign, HelloSign, or other providers
- Signers stored as JSONB array with: email, name, status, signed_at, order
- Support for sequential signing (order field)
- Webhook endpoint for async signature events
- Signed document stored as new version with signature metadata

## Dev Agent Record

### Implementation Plan
All tasks have been implemented as part of the Epic 7B document versioning and e-signature integration work.

## File List

### Database Migration
- `backend/crates/db/migrations/00037_create_signature_requests.sql` - E-signature schema

### Models
- `backend/crates/db/src/models/signature_request.rs` - SignatureRequest, Signer, SignerStatus, SignatureRequestStatus models
- `backend/crates/db/src/models/mod.rs` - Model exports

### Repository
- `backend/crates/db/src/repositories/signature_request.rs` - SignatureRequestRepository implementation
- `backend/crates/db/src/repositories/mod.rs` - Repository exports

### API Routes
- `backend/servers/api-server/src/routes/signatures.rs` - E-signature API endpoints
- `backend/servers/api-server/src/routes/mod.rs` - Route module exports
- `backend/servers/api-server/src/main.rs` - Route registration

### State
- `backend/servers/api-server/src/state.rs` - SignatureRequestRepository in AppState

## Change Log

| Date | Change | Author |
|------|--------|--------|
| 2025-12-22 | Story created | AI Agent |
| 2025-12-26 | Story verified complete - all tasks implemented | AI Agent |
