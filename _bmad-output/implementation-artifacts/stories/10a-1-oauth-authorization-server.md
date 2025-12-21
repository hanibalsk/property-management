# Story 10A.1: OAuth 2.0 Authorization Server

Status: completed

## Story

As a **system architect**,
I want to **implement OAuth 2.0 authorization server**,
So that **future applications can authenticate via SSO**.

## Acceptance Criteria

1. **AC-1: Authorization Code Grant Flow**
   - Given an OAuth client is registered
   - When it initiates authorization flow
   - Then user is prompted to authorize
   - And authorization code is returned

2. **AC-2: Token Exchange**
   - Given a client exchanges authorization code
   - When the code is valid
   - Then access and refresh tokens are issued
   - And client can access protected resources

3. **AC-3: Protected Resource Access**
   - Given an access token is used
   - When it's valid and not expired
   - Then the protected resource is accessible
   - And token claims are available

## Tasks / Subtasks

- [x] Task 1: Database Schema & Migrations (AC: 1, 2, 3)
  - [x] 1.1 Create `oauth_clients` table: id (UUID), client_id (VARCHAR unique), client_secret_hash, name, redirect_uris (JSONB), scopes (JSONB), is_active, created_at, updated_at
  - [x] 1.2 Create `oauth_authorizations` table: id (UUID), user_id (FK), client_id (VARCHAR FK), code_hash, scopes (JSONB), redirect_uri, code_challenge, code_challenge_method, expires_at, used_at, created_at
  - [x] 1.3 Create `oauth_access_tokens` table: id (UUID), user_id (FK), client_id (VARCHAR FK), token_hash, scopes (JSONB), expires_at, revoked_at, created_at
  - [x] 1.4 Add RLS policies for multi-tenant isolation
  - [x] 1.5 Create indexes for efficient lookups (client_id, token_hash, code_hash)

- [x] Task 2: OAuth Domain Models (AC: 1, 2, 3)
  - [x] 2.1 Create OAuthClient model: client_id, client_secret_hash, name, redirect_uris, scopes, is_active
  - [x] 2.2 Create OAuthAuthorization model: user_id, client_id, code_hash, scopes, redirect_uri, pkce fields, expires_at
  - [x] 2.3 Create OAuthAccessToken model: user_id, client_id, token_hash, scopes, expires_at, revoked_at
  - [x] 2.4 Create DTOs: AuthorizeRequest, TokenRequest, TokenResponse, AuthorizationGrant

- [x] Task 3: OAuth Repository Implementation (AC: 1, 2, 3)
  - [x] 3.1 Create OAuthRepository with client CRUD operations
  - [x] 3.2 Implement authorization code create/consume methods
  - [x] 3.3 Implement access token create/revoke/validate methods
  - [x] 3.4 Add cleanup methods for expired tokens and codes

- [x] Task 4: OAuth Service Implementation (AC: 1, 2, 3)
  - [x] 4.1 Create OAuthService for OAuth flow orchestration
  - [x] 4.2 Implement validate_authorize_request() - validate client, redirect_uri, scopes
  - [x] 4.3 Implement create_authorization_code() with PKCE support
  - [x] 4.4 Implement exchange_code_for_tokens() with code verification
  - [x] 4.5 Implement validate_access_token() for resource access
  - [x] 4.6 Implement refresh_token_grant() for token refresh

- [x] Task 5: OAuth API Endpoints (AC: 1, 2, 3)
  - [x] 5.1 GET /api/v1/oauth/authorize - authorization endpoint (render consent page)
  - [x] 5.2 POST /api/v1/oauth/authorize - process user consent, return auth code
  - [x] 5.3 POST /api/v1/oauth/token - token endpoint (exchange code for tokens)
  - [x] 5.4 POST /api/v1/oauth/revoke - revoke access/refresh tokens
  - [x] 5.5 Add OAuth routes to main router

- [x] Task 6: Unit & Integration Tests (AC: 1, 2, 3)
  - [x] 6.1 Test authorization code generation and validation
  - [x] 6.2 Test token exchange flow
  - [x] 6.3 Test PKCE challenge verification
  - [x] 6.4 Test token revocation
  - [x] 6.5 Test invalid/expired code handling
  - [x] 6.6 Test invalid client credentials handling

## Dev Notes

### Architecture Requirements
- OAuth 2.0 Authorization Code flow with PKCE (RFC 7636)
- Secure code and token hashing using Argon2id
- Authorization codes: 32 bytes, base64url encoded, 10-minute expiry
- Access tokens: 32 bytes, base64url encoded, 15-minute expiry
- PKCE: S256 method required for public clients

### Technical Specifications
- Backend: Rust + Axum following existing patterns
- Tables follow snake_case convention
- API responses use standard JSON format
- Token format: opaque tokens (not JWT for OAuth - internal JWT for API auth remains)
- Scopes: profile, email, org:read, full

### Security Considerations
- Never log authorization codes or tokens
- Client secrets hashed like passwords (Argon2id)
- Authorization codes single-use (consumed on exchange)
- PKCE required for all authorization flows
- Rate limiting on token endpoint

### Database Patterns
- Follow existing model patterns in crates/db/src/models/
- Follow repository patterns in crates/db/src/repositories/
- Use sqlx with FromRow derive

### References
- [Source: _bmad-output/epics.md#Epic-10A-Story-10A.1]
- [RFC 6749: OAuth 2.0](https://tools.ietf.org/html/rfc6749)
- [RFC 7636: PKCE](https://tools.ietf.org/html/rfc7636)

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

N/A

### Completion Notes List

- Implemented complete OAuth 2.0 Authorization Server with PKCE support
- Created 5 database tables with proper indexes and cleanup function
- Repository supports all CRUD operations with token rotation
- Service implements full authorization code and token flows
- API endpoints follow RFC 6749 and RFC 7662 specifications
- All clippy warnings fixed, tests passing

### File List

- `backend/crates/db/migrations/00028_create_oauth_provider.sql` - Database schema
- `backend/crates/db/src/models/oauth.rs` - Domain models and DTOs
- `backend/crates/db/src/repositories/oauth.rs` - Repository implementation
- `backend/servers/api-server/src/services/oauth.rs` - OAuth service
- `backend/servers/api-server/src/routes/oauth.rs` - API endpoints

## Change Log

| Date | Change |
|------|--------|
| 2025-12-21 | Story created |
