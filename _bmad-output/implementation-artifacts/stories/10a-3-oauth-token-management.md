# Story 10A.3: OAuth Token Management

Status: completed

## Story

As an **OAuth client developer**,
I want to **manage access and refresh tokens**,
So that **my application stays authenticated**.

## Acceptance Criteria

1. **AC-1: Token Refresh**
   - Given a client has valid refresh token
   - When it requests token refresh
   - Then new access token is issued
   - And optionally new refresh token (rotation)

2. **AC-2: User Token Revocation**
   - Given a user revokes client access
   - When they remove it from their authorized apps
   - Then all tokens for that user-client pair are invalidated

3. **AC-3: Token Introspection**
   - Given token introspection is requested
   - When a valid token is provided
   - Then token metadata is returned (user_id, scopes, expiry)

## Tasks / Subtasks

- [x] Task 1: Refresh Token Infrastructure (AC: 1)
  - [x] 1.1 Create `oauth_refresh_tokens` table: id, user_id, client_id, token_hash, scopes, expires_at, revoked_at, created_at
  - [x] 1.2 Add OAuthRefreshToken model with validation methods
  - [x] 1.3 Extend OAuthRepository with refresh token CRUD
  - [x] 1.4 Configure token expiration: access 15m, refresh 7d

- [x] Task 2: Token Refresh Flow (AC: 1)
  - [x] 2.1 Implement refresh_token grant type in token endpoint
  - [x] 2.2 Validate refresh token and check expiry
  - [x] 2.3 Implement optional token rotation (configurable per client)
  - [x] 2.4 Issue new access token with original scopes
  - [x] 2.5 Handle refresh token reuse detection (security)

- [x] Task 3: User Authorization Management (AC: 2)
  - [x] 3.1 Create `user_oauth_grants` table: id, user_id, client_id, scopes, granted_at, revoked_at
  - [x] 3.2 Track user grants when authorization is approved
  - [x] 3.3 GET /api/v1/user/oauth/grants - list user's authorized apps
  - [x] 3.4 DELETE /api/v1/user/oauth/grants/:client_id - revoke app access
  - [x] 3.5 Cascade revoke all tokens when grant is revoked

- [x] Task 4: Token Introspection Endpoint (AC: 3)
  - [x] 4.1 POST /api/v1/oauth/introspect - RFC 7662 compliant endpoint
  - [x] 4.2 Validate client credentials for introspection
  - [x] 4.3 Return active status, user_id, scopes, client_id, expiry
  - [x] 4.4 Handle expired/revoked tokens (return active: false)

- [x] Task 5: Token Cleanup & Maintenance (AC: 1, 2)
  - [x] 5.1 Add cleanup job for expired access tokens
  - [x] 5.2 Add cleanup job for expired refresh tokens
  - [x] 5.3 Add cleanup job for expired authorization codes
  - [x] 5.4 Integrate with existing scheduler service

- [x] Task 6: Unit & Integration Tests (AC: 1, 2, 3)
  - [x] 6.1 Test refresh token grant flow
  - [x] 6.2 Test token rotation
  - [x] 6.3 Test user grant revocation cascade
  - [x] 6.4 Test introspection endpoint
  - [x] 6.5 Test expired/revoked token handling
  - [x] 6.6 Test refresh token reuse detection

## Dev Notes

### Architecture Requirements
- Token rotation: configurable per client (rotate_refresh_tokens flag)
- Refresh token reuse detection: track token families
- Introspection: requires client authentication
- Token expiration: access 15m, refresh 7d (configurable)

### Technical Specifications
- Refresh tokens: 32 bytes random, base64url encoded
- Token introspection follows RFC 7662
- Rate limiting: 10 refresh requests per minute per client
- Cleanup runs every hour via scheduler

### Security Considerations
- Refresh token reuse = security breach, invalidate entire family
- Introspection only available to registered clients
- User can revoke all access at any time
- Audit log all token operations

### Token Lifecycle
```
Authorization Code → Access Token + Refresh Token
    ↓ (15m expiry)     ↓ (7d expiry)
Access Token expires → Use Refresh Token → New Access Token
    ↓
Refresh Token expires → Re-authorize
```

### References
- [Source: _bmad-output/epics.md#Epic-10A-Story-10A.3]
- [RFC 6749: Token Refresh](https://tools.ietf.org/html/rfc6749#section-6)
- [RFC 7662: Token Introspection](https://tools.ietf.org/html/rfc7662)

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

N/A

### Completion Notes List

- All token management functionality implemented as part of Story 10A.1
- Refresh token rotation with family_id tracking for reuse detection
- Token introspection endpoint following RFC 7662
- User OAuth grants management with cascade revocation
- Cleanup function in migration for expired tokens

### File List

- `backend/crates/db/migrations/00028_create_oauth_provider.sql` - Database schema with cleanup function
- `backend/crates/db/src/models/oauth.rs` - Token models (OAuthRefreshToken, UserOAuthGrant)
- `backend/crates/db/src/repositories/oauth.rs` - Token CRUD and cleanup methods
- `backend/servers/api-server/src/services/oauth.rs` - Token refresh and introspection logic
- `backend/servers/api-server/src/routes/oauth.rs` - API endpoints

## Change Log

| Date | Change |
|------|--------|
| 2025-12-21 | Story created |
