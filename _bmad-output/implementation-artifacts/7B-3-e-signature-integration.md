# Story 7B.3: E-Signature Integration

## Story

As a **property manager**,
I want to **send documents for electronic signature**,
So that **I can get legally binding signatures without paper**.

## Status

in-progress

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

- [ ] **Task 1: Database Schema for E-Signatures**
  - [ ] Create migration 00037 for signature_requests table
  - [ ] Add signature_request_status enum (pending, in_progress, completed, declined, expired, cancelled)
  - [ ] Add signers JSONB column for signer details and status
  - [ ] Add provider_request_id for external integration tracking
  - [ ] Add audit trail fields (created_at, completed_at, expires_at)

- [ ] **Task 2: Update Models**
  - [ ] Create SignatureRequest model
  - [ ] Create Signer struct for individual signer tracking
  - [ ] Create SignatureRequestStatus enum
  - [ ] Create request/response types for API

- [ ] **Task 3: Repository Methods**
  - [ ] Implement create_signature_request()
  - [ ] Implement find_signature_request_by_id()
  - [ ] Implement find_signature_requests_by_document()
  - [ ] Implement update_signer_status()
  - [ ] Implement complete_signature_request()
  - [ ] Implement cancel_signature_request()
  - [ ] Implement list_pending_signature_requests()

- [ ] **Task 4: API Endpoints**
  - [ ] POST /api/v1/documents/:id/signature-requests - Create signature request
  - [ ] GET /api/v1/documents/:id/signature-requests - List signature requests for document
  - [ ] GET /api/v1/signature-requests/:id - Get signature request details
  - [ ] POST /api/v1/signature-requests/:id/remind - Send reminder to pending signers
  - [ ] POST /api/v1/signature-requests/:id/cancel - Cancel signature request
  - [ ] POST /api/v1/signature-requests/:id/webhook - Handle provider webhook (signature events)

## Dev Notes

### Technical Specifications
- Integration abstracted to support DocuSign, HelloSign, or other providers
- Signers stored as JSONB array with: email, name, status, signed_at, order
- Support for sequential signing (order field)
- Webhook endpoint for async signature events
- Signed document stored as new version with signature metadata

## Dev Agent Record

### Implementation Plan
*To be filled during implementation*

## File List

*To be filled during implementation*

## Change Log

| Date | Change | Author |
|------|--------|--------|
| 2025-12-22 | Story created | AI Agent |
