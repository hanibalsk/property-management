# Story 10B.5: Support Data Access

Status: ready-for-dev

## Story

As a **support staff member**,
I want to **access organization data for troubleshooting**,
So that **I can help users resolve issues**.

## Acceptance Criteria

1. **AC-1: Read-Only Org Access**
   - Given support staff is granted access to an org
   - When they view org data
   - Then they see user-facing data (not internal metadata)
   - And access is read-only

2. **AC-2: Time-Limited Access**
   - Given support access is requested
   - When org admin approves (or auto-approved for support tier)
   - Then access is granted for limited time (24h)
   - And all actions are logged

3. **AC-3: Automatic Revocation**
   - Given support access expires
   - When the time limit is reached
   - Then access is automatically revoked
   - And support staff is notified

## Tasks / Subtasks

- [ ] Task 1: Database Schema & Migrations (AC: 1, 2, 3)
  - [ ] 1.1 Create `support_access_requests` table: id (UUID), support_user_id (FK), org_id (FK), reason, status (pending/approved/denied/expired), approved_by, approved_at, expires_at, created_at
  - [ ] 1.2 Create `support_access_logs` table: id, request_id (FK), action, resource_type, resource_id, details (JSONB), created_at
  - [ ] 1.3 Add support_tier column to organizations for auto-approval policy
  - [ ] 1.4 Create indexes for efficient lookup of active access grants

- [ ] Task 2: Support Access Models (AC: 1, 2, 3)
  - [ ] 2.1 Create SupportAccessRequest model: id, support_user_id, org_id, reason, status, approved_by, expires_at
  - [ ] 2.2 Create SupportAccessStatus enum: Pending, Approved, Denied, Expired, Revoked
  - [ ] 2.3 Create SupportAccessLog model: id, request_id, action, resource_type, resource_id, details
  - [ ] 2.4 Create DTOs: RequestAccessRequest, ApproveAccessRequest, SupportAccessResponse

- [ ] Task 3: Support Access Repository (AC: 1, 2, 3)
  - [ ] 3.1 Create SupportAccessRepository
  - [ ] 3.2 Implement create_access_request() for support staff
  - [ ] 3.3 Implement approve_access() setting expiration time
  - [ ] 3.4 Implement get_active_access() checking expiration
  - [ ] 3.5 Implement log_support_action() for audit trail
  - [ ] 3.6 Implement expire_stale_access() for cleanup job

- [ ] Task 4: Support Access Service (AC: 1, 2, 3)
  - [ ] 4.1 Create SupportAccessService for access orchestration
  - [ ] 4.2 Implement request_access() with org notification
  - [ ] 4.3 Implement approve_access() with auto-approval logic for support tier
  - [ ] 4.4 Implement verify_access() checking if support user has valid grant
  - [ ] 4.5 Implement revoke_access() with notification to support user

- [ ] Task 5: Support Access Middleware (AC: 1)
  - [ ] 5.1 Create SupportContextExtractor for impersonation-style access
  - [ ] 5.2 Implement read-only mode enforcement for support context
  - [ ] 5.3 Implement automatic action logging for all support requests
  - [ ] 5.4 Add support context header: X-Support-Access-Token

- [ ] Task 6: Support Access API Endpoints (AC: 1, 2, 3)
  - [ ] 6.1 POST /api/v1/support/access-requests - request access to organization
  - [ ] 6.2 GET /api/v1/support/access-requests - list own access requests (support view)
  - [ ] 6.3 GET /api/v1/organizations/:id/support-requests - list pending requests (org admin view)
  - [ ] 6.4 POST /api/v1/organizations/:id/support-requests/:rid/approve - approve request
  - [ ] 6.5 POST /api/v1/organizations/:id/support-requests/:rid/deny - deny request
  - [ ] 6.6 POST /api/v1/support/access/:id/revoke - revoke own access or admin revokes
  - [ ] 6.7 GET /api/v1/support/access/:id/logs - view support access audit trail

- [ ] Task 7: Background Jobs (AC: 3)
  - [ ] 7.1 Create expire_support_access job running every 5 minutes
  - [ ] 7.2 Implement notification to support staff when access expires
  - [ ] 7.3 Implement reminder notification 1 hour before expiration

- [ ] Task 8: Unit & Integration Tests (AC: 1, 2, 3)
  - [ ] 8.1 Test access request flow end-to-end
  - [ ] 8.2 Test auto-approval for support tier organizations
  - [ ] 8.3 Test read-only enforcement in support context
  - [ ] 8.4 Test automatic expiration and revocation
  - [ ] 8.5 Test audit logging completeness

## Dev Notes

### Architecture Requirements
- Support access is time-limited (default 24h, max 72h)
- All support actions logged to dedicated audit table
- Read-only mode enforced at API layer (reject POST/PUT/DELETE)
- Org admins can revoke support access at any time

### Technical Specifications
- Backend: Rust + Axum following existing patterns
- Support context implemented via X-Support-Access-Token header
- Support users need SupportStaff role (separate from SuperAdmin)
- Notification via existing email/notification infrastructure

### Security Considerations
- Support staff cannot access: passwords, tokens, audit logs, PII exports
- All support data views logged with request_id reference
- Rate limiting on access requests (max 5 pending per support user)
- Org admins notified of all support access grants

### Database Patterns
- Follow existing model patterns in crates/db/src/models/
- Support access logs separate from main audit_log (different retention)
- Use existing notification patterns for alerts

### References
- [Source: _bmad-output/epics.md#Epic-10B-Story-10B.5]
- Audit logging patterns: backend/crates/db/src/models/audit_log.rs

## Dev Agent Record

### Agent Model Used

TBD

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
