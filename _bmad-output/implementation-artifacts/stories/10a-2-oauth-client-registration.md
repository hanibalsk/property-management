# Story 10A.2: OAuth Client Registration

Status: completed

## Story

As a **platform administrator**,
I want to **register OAuth clients**,
So that **trusted applications can integrate**.

## Acceptance Criteria

1. **AC-1: Register OAuth Client**
   - Given an admin registers a new OAuth client
   - When they provide name, redirect URIs, and scopes
   - Then client_id and client_secret are generated
   - And the client is active

2. **AC-2: Client Authorization Display**
   - Given a client is registered
   - When it makes an authorization request
   - Then user sees client name and requested scopes
   - And can approve or deny

3. **AC-3: Client Revocation**
   - Given an admin revokes a client
   - When revocation is processed
   - Then all tokens for that client are invalidated
   - And new authorizations are blocked

## Tasks / Subtasks

- [x] Task 1: Client Registration DTOs (AC: 1)
  - [x] 1.1 Create RegisterClientRequest: name, redirect_uris, scopes, description
  - [x] 1.2 Create RegisterClientResponse: client_id, client_secret (plaintext, shown once), name, created_at
  - [x] 1.3 Create ClientListResponse: id, client_id, name, scopes, is_active, created_at
  - [x] 1.4 Create UpdateClientRequest: name, redirect_uris, scopes, is_active

- [x] Task 2: Client Repository Extensions (AC: 1, 3)
  - [x] 2.1 Implement create_client() with generated client_id/secret
  - [x] 2.2 Implement list_clients() for admin dashboard
  - [x] 2.3 Implement update_client() for configuration changes
  - [x] 2.4 Implement revoke_client() with cascade token invalidation
  - [x] 2.5 Implement regenerate_client_secret()

- [x] Task 3: Client Service Implementation (AC: 1, 2, 3)
  - [x] 3.1 Create OAuthClientService for client management
  - [x] 3.2 Implement register_client() with secret generation and hashing
  - [x] 3.3 Implement get_client_for_consent() to fetch display info
  - [x] 3.4 Implement revoke_client() with token cleanup
  - [x] 3.5 Implement regenerate_secret() showing new secret once

- [x] Task 4: Admin API Endpoints (AC: 1, 3)
  - [x] 4.1 POST /api/v1/admin/oauth/clients - register new client
  - [x] 4.2 GET /api/v1/admin/oauth/clients - list all clients
  - [x] 4.3 GET /api/v1/admin/oauth/clients/:id - get client details
  - [x] 4.4 PATCH /api/v1/admin/oauth/clients/:id - update client
  - [x] 4.5 DELETE /api/v1/admin/oauth/clients/:id - revoke client
  - [x] 4.6 POST /api/v1/admin/oauth/clients/:id/regenerate-secret - regenerate secret

- [x] Task 5: Consent Page Support (AC: 2)
  - [x] 5.1 Create consent page data endpoint returning client name, scopes, user info
  - [x] 5.2 Implement scope descriptions for human-readable display
  - [x] 5.3 Create UserAuthorizedClients model for tracking granted permissions

- [x] Task 6: Audit Logging (AC: 1, 3)
  - [x] 6.1 Log client registration events
  - [x] 6.2 Log client revocation events
  - [x] 6.3 Log secret regeneration events

- [x] Task 7: Unit & Integration Tests (AC: 1, 2, 3)
  - [x] 7.1 Test client registration with valid/invalid data
  - [x] 7.2 Test client listing and filtering
  - [x] 7.3 Test client revocation cascade
  - [x] 7.4 Test secret regeneration
  - [x] 7.5 Test admin authorization checks

## Dev Notes

### Architecture Requirements
- Client secrets: 32 bytes random, base64url encoded
- Client IDs: 16 bytes random, base64url encoded
- Secret shown only once during creation or regeneration
- Admin-only endpoints (require super_admin role)

### Technical Specifications
- Scopes available: profile, email, org:read, full
- Redirect URIs validated against whitelist
- Client secrets hashed with Argon2id
- Audit log entries for all client management operations

### Security Considerations
- Admin-only access to client management
- Secrets never logged or stored in plaintext
- Revocation immediately invalidates all tokens
- Rate limiting on client creation

### Scope Definitions
| Scope | Description |
|-------|-------------|
| profile | Access to user's basic profile (name, avatar) |
| email | Access to user's email address |
| org:read | Read-only access to organization data |
| full | Full access to user's data and actions |

### References
- [Source: _bmad-output/epics.md#Epic-10A-Story-10A.2]
- [RFC 6749: Client Registration](https://tools.ietf.org/html/rfc6749#section-2)

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

N/A

### Completion Notes List

- All client management functionality implemented as part of Story 10A.1
- Admin endpoints with full CRUD operations
- Audit logging for all client management operations
- Consent page data with scope descriptions
- Tests for client operations included

### File List

- `backend/crates/db/src/models/oauth.rs` - DTOs and models
- `backend/crates/db/src/repositories/oauth.rs` - Repository implementation
- `backend/servers/api-server/src/services/oauth.rs` - Service implementation
- `backend/servers/api-server/src/routes/oauth.rs` - Admin API endpoints

## Change Log

| Date | Change |
|------|--------|
| 2025-12-21 | Story created |
