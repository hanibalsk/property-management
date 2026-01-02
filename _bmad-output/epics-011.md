---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - _bmad-output/prd.md
  - _bmad-output/architecture.md
  - _bmad-output/epics-010.md
workflowType: 'epics-and-stories'
lastStep: 4
status: 'ready'
project_name: 'Property Management System (PPT) & Reality Portal'
user_name: 'Martin Janci'
date: '2026-01-01'
continues_from: 'epics-010.md'
phase_range: '29-30'
epic_range: '98-100'
---

# Property Management System (PPT) & Reality Portal - Epic Breakdown (Part 11)

## Overview

This document continues from `epics-010.md` and addresses **Phase 29: Integration Completion** and **Phase 30: Quality Assurance & Hardening** - completing OAuth flows, removing integration stubs, and improving test coverage.

**Continuation from:** `epics-010.md` (Epics 91-95, Phases 27-28)

**Source:** Comprehensive gap analysis (2026-01-01)

**Key Findings from Gap Analysis:**
- OAuth token exchange missing for voice devices (ai.rs:2776-2790)
- Rental platform OAuth flows not implemented (rentals.rs:345-373)
- 15+ integration stubs in airbnb.rs, booking.rs, integrations.rs
- Voice webhook certificate validation incomplete
- 24 auth integration tests are placeholders
- Several repository stubs (operations, dispute, owner_analytics)

---

## Epic List

### Phase 29: Integration Completion

#### Epic 98: Complete OAuth & Integration Flows
**Goal:** Implement all OAuth token exchanges and complete integration stubs.

**Target Apps:** api-server
**Estimate:** 6 stories, ~1.5 weeks
**Dependencies:** Epics 91, 96
**Priority:** P1 - HIGH

**PRD Reference:** FR35 - Integration with external platforms (UC-22, UC-32)

---

##### Story 98.1: Voice Device OAuth Token Exchange

As a **user**,
I want to **link my voice assistant with proper OAuth authentication**,
So that **voice commands are securely authenticated with my account**.

**Acceptance Criteria:**

**Given** a voice device link request with an auth_code
**When** the linking process completes
**Then**:
  - OAuth authorization code is exchanged for tokens
  - Access token is stored encrypted
  - Refresh token is stored for renewal
  - Token expiry is tracked
  - Device is associated with user account
**And** voice commands work immediately after linking

**Technical Notes:**
- Reference: `ai.rs:2776-2790` - Phase 2 TODO for OAuth tokens
- Exchange `auth_code` for tokens using OAuth 2.0 client credentials
- Store tokens encrypted in `voice_assistants` table
- Implement token refresh logic for expired tokens

**Files to Modify:**
- `backend/servers/api-server/src/routes/ai.rs` (complete OAuth exchange in link_voice_device)
- `backend/crates/db/src/models/voice_assistant.rs` (ensure token fields exist)

---

##### Story 98.2: Rental Platform OAuth Implementation

As a **property owner**,
I want to **connect my Airbnb and Booking.com accounts**,
So that **reservations sync automatically**.

**Acceptance Criteria:**

**Given** an OAuth callback from Airbnb or Booking.com
**When** the callback is processed
**Then**:
  - State parameter is validated against stored value
  - Authorization code is exchanged for tokens
  - Tokens are encrypted and stored in connection record
  - Connection status is updated to "connected"
  - User is redirected to success page
**And** sync can begin immediately

**Technical Notes:**
- Reference: `rentals.rs:345` - TODO for Airbnb OAuth flow
- Reference: `rentals.rs:366` - TODO for Booking.com OAuth flow
- Follow OAuth 2.0 specification for each platform
- Use secure random state parameter validation

**Files to Modify:**
- `backend/servers/api-server/src/routes/rentals.rs` (implement OAuth callbacks)

---

##### Story 98.3: Airbnb Integration Implementation

As a **property owner with Airbnb listings**,
I want to **sync my Airbnb reservations and listings**,
So that **I have a unified view of all my properties**.

**Acceptance Criteria:**

**Given** a connected Airbnb account
**When** a sync is triggered
**Then**:
  - Listings are fetched from Airbnb API
  - Reservations are fetched and stored locally
  - Booking conflicts are detected
  - Sync status is tracked
  - Errors are logged and retryable
**And** data is updated in real-time via webhooks

**Technical Notes:**
- Reference: `airbnb.rs:550-648` - Multiple stub implementations
- Implement actual API calls using Airbnb API v2
- Parse JSON responses properly (not stubs)
- Handle rate limiting and retries

**Files to Modify:**
- `backend/crates/integrations/src/airbnb.rs` (replace all stubs with real implementations)
- `backend/servers/api-server/src/routes/integrations.rs` (update integration routes)

---

##### Story 98.4: Booking.com Integration Implementation

As a **property owner with Booking.com listings**,
I want to **sync my Booking.com properties and reservations**,
So that **all reservations are visible in one place**.

**Acceptance Criteria:**

**Given** a connected Booking.com account
**When** a sync is triggered
**Then**:
  - Properties are fetched from Booking.com API
  - Reservations are parsed from XML responses
  - Availability calendar is synchronized
  - Rate plans are imported
  - Errors are handled gracefully
**And** updates propagate bidirectionally

**Technical Notes:**
- Reference: `booking.rs:323-786` - Multiple stub implementations
- Booking.com uses XML API - implement using quick-xml
- Handle OTA_HotelRes format for reservations
- Implement proper authentication using credentials

**Files to Modify:**
- `backend/crates/integrations/src/booking.rs` (replace all stubs with real implementations)
- Add `quick-xml` dependency to Cargo.toml

---

##### Story 98.5: Integration Management Routes

As a **developer**,
I want to **complete all integration management endpoints**,
So that **users can manage their external platform connections**.

**Acceptance Criteria:**

**Given** integration management routes exist
**When** they are called
**Then**:
  - `sync_airbnb_listings` returns actual synced listings
  - `connect_booking` validates and stores credentials
  - `process_reservations` processes real reservation data
  - All 10 stub implementations are replaced
  - Error responses are informative
**And** all endpoints are tested

**Technical Notes:**
- Reference: `integrations.rs:3342-4003` - 10 stub implementations
- Wire up to actual integration clients (airbnb.rs, booking.rs)
- Implement background job triggering for long-running syncs
- Add proper error handling and status tracking

**Files to Modify:**
- `backend/servers/api-server/src/routes/integrations.rs` (complete all stubs)

---

##### Story 98.6: Voice Webhook Certificate Validation

As a **security engineer**,
I want to **proper certificate validation on voice webhooks**,
So that **only legitimate requests are processed**.

**Acceptance Criteria:**

**Given** a webhook request from Alexa or Google
**When** the request is validated
**Then**:
  - Alexa: Signature is verified using public key from certificate
  - Alexa: Timestamp is within 150 seconds
  - Alexa: Certificate chain is validated
  - Google: JWT ID token is verified
  - Google: Project ID matches configuration
  - Invalid requests are rejected with 401
**And** validation is logged for audit

**Technical Notes:**
- Reference: `voice_webhooks.rs:524` - TODO for certificate validation
- Fetch Alexa certificate from URL in header
- Verify certificate chain and signature
- For Google, validate using google-auth-library-rust

**Files to Modify:**
- `backend/servers/api-server/src/routes/voice_webhooks.rs` (complete validation)
- Add certificate verification dependencies

---

### Phase 30: Quality Assurance & Hardening

#### Epic 99: Test Infrastructure & Coverage
**Goal:** Complete integration tests and improve test coverage for critical paths.

**Target Apps:** api-server
**Estimate:** 4 stories, ~1 week
**Dependencies:** None
**Priority:** P2 - MEDIUM

**PRD Reference:** NFR-REL - System reliability requirements

---

##### Story 99.1: Auth Integration Tests

As a **developer**,
I want to **complete authentication integration tests**,
So that **auth flows are verified end-to-end**.

**Acceptance Criteria:**

**Given** auth integration test file exists
**When** tests are executed
**Then**:
  - All 24 placeholder tests are implemented
  - Registration flow is tested with valid/invalid data
  - Login flow is tested with correct/wrong credentials
  - Token refresh is tested for expiry
  - MFA setup and verification are tested
  - Password reset flow is tested
**And** tests run in CI pipeline

**Technical Notes:**
- Reference: `auth_tests.rs` - 24 tests marked #[ignore]
- Initialize test database using sqlx::test
- Use test fixtures for user data
- Mock email sending for password reset

**Files to Modify:**
- `backend/servers/api-server/tests/integration/auth_tests.rs` (implement all tests)
- `backend/servers/api-server/tests/common/mod.rs` (add test utilities)

---

##### Story 99.2: Document Access Control Tests

As a **developer**,
I want to **test document access control properly**,
So that **users only see documents they should**.

**Acceptance Criteria:**

**Given** documents with various access levels
**When** different users request documents
**Then**:
  - Organization admins see all org documents
  - Building managers see their building documents
  - Owners see unit-specific documents for their units
  - Tenants see tenant-accessible documents
  - Non-authorized users get 403 Forbidden
**And** tests cover all role combinations

**Technical Notes:**
- Reference: `documents.rs:645` - TODO for building/unit context
- Test each role's document visibility
- Test permission inheritance
- Test folder hierarchy access

**Files to Create:**
- `backend/servers/api-server/tests/integration/document_access_tests.rs`

---

##### Story 99.3: Integration Sync Tests

As a **developer**,
I want to **test external integration sync flows**,
So that **platform integrations work reliably**.

**Acceptance Criteria:**

**Given** mock external API responses
**When** sync operations run
**Then**:
  - Airbnb listing sync is tested
  - Booking.com reservation sync is tested
  - Error handling is tested
  - Retry logic is verified
  - Rate limiting is handled
**And** tests use mock servers (wiremock)

**Technical Notes:**
- Use wiremock for mocking external APIs
- Test success and failure scenarios
- Verify data transformation
- Test idempotency of sync operations

**Files to Create:**
- `backend/servers/api-server/tests/integration/integration_sync_tests.rs`

---

##### Story 99.4: Health Check Tests

As a **developer**,
I want to **test health check endpoints thoroughly**,
So that **monitoring works correctly**.

**Acceptance Criteria:**

**Given** health check endpoints exist
**When** they are called
**Then**:
  - Database connectivity is verified
  - Uptime tracking works correctly
  - Degraded status is reported appropriately
  - Health history is recorded
  - Redis health check works (when implemented)
**And** health endpoints are documented

**Technical Notes:**
- Reference: `health.rs:129` - TODO for Redis health check
- Test with healthy and unhealthy dependencies
- Test timeout handling
- Verify response format matches API spec

**Files to Create:**
- `backend/servers/api-server/tests/integration/health_tests.rs`

---

#### Epic 100: Repository & Data Layer Completion
**Goal:** Complete repository implementations that are currently stubs.

**Target Apps:** api-server
**Estimate:** 4 stories, ~1 week
**Dependencies:** None
**Priority:** P2 - MEDIUM

**PRD Reference:** NFR-PERF, NFR-REL - Performance and reliability

---

##### Story 100.1: Operations Repository Implementation

As a **system administrator**,
I want to **view real infrastructure metrics**,
So that **I can monitor system operations**.

**Acceptance Criteria:**

**Given** the operations dashboard is accessed
**When** metrics are requested
**Then**:
  - Real uptime is calculated from boot time
  - Background job stats are accurate
  - Trace storage is functional
  - Alert rules are persisted
  - Metrics history is queryable
**And** data is updated in real-time

**Technical Notes:**
- Reference: `operations.rs:2` - Stub implementation
- Reference: `infrastructure.rs:293` - Trace storage TODO
- Reference: `infrastructure.rs:1180` - Job stats TODO
- Implement actual database queries
- Store metrics in time-series format if needed

**Files to Modify:**
- `backend/crates/db/src/repositories/operations.rs` (replace stubs)
- `backend/servers/api-server/src/routes/infrastructure.rs` (wire to real repo)

---

##### Story 100.2: Dispute Repository Implementation

As a **property manager**,
I want to **manage resident disputes**,
So that **issues are tracked and resolved**.

**Acceptance Criteria:**

**Given** a dispute is filed
**When** dispute management is used
**Then**:
  - Disputes are stored in database
  - Status changes are tracked
  - Resolution history is maintained
  - Related parties are notified
  - Disputes are searchable and filterable
**And** dispute data is audit-logged

**Technical Notes:**
- Reference: `dispute.rs:2` - Stub implementation
- Create proper database tables if not exist
- Implement CRUD operations
- Add status workflow (open, mediation, resolved, escalated)

**Files to Modify:**
- `backend/crates/db/src/repositories/dispute.rs` (replace stubs)

---

##### Story 100.3: Owner Analytics Repository

As a **property owner**,
I want to **view investment analytics**,
So that **I can track property performance**.

**Acceptance Criteria:**

**Given** owner analytics is accessed
**When** analytics data is requested
**Then**:
  - ROI calculations are accurate
  - Expense tracking is complete
  - Income breakdown is detailed
  - Comparison periods work
  - Export to CSV/PDF works
**And** data is calculated efficiently

**Technical Notes:**
- Reference: `owner_analytics.rs:2` - Stub file
- Aggregate data from payments, expenses, units
- Calculate key metrics (NOI, cap rate, cash-on-cash)
- Cache expensive calculations

**Files to Modify:**
- `backend/crates/db/src/models/owner_analytics.rs` (full implementation)
- `backend/crates/db/src/repositories/owner_analytics.rs` (create)

---

##### Story 100.4: EDD Repository Implementation

As a **compliance officer**,
I want to **verify EDD records in the database**,
So that **AML/DSA compliance can be enforced**.

**Acceptance Criteria:**

**Given** EDD verification is required
**When** a check is performed
**Then**:
  - EDD records are stored in database
  - Verification status is queryable
  - Document references are linked
  - Audit trail is maintained
  - Expiry is tracked
**And** compliance reports can be generated

**Technical Notes:**
- Reference: `aml_dsa.rs:611` - TODO for EDD repository
- Create EDD table if not exists
- Link to documents and users
- Implement compliance checks

**Files to Modify:**
- `backend/crates/db/src/repositories/edd.rs` (create)
- `backend/servers/api-server/src/routes/aml_dsa.rs` (wire to repo)

---

## Summary

| Phase | Epics | Stories | Priority |
|-------|-------|---------|----------|
| 29: Integration Completion | 98 | 6 | P1 |
| 30: Quality Assurance | 99-100 | 8 | P2 |

**Total:** 3 Epics, 14 Stories

### Implementation Order

1. **Epic 98** - Complete OAuth & Integration Flows (P1) ~1.5 weeks
   - Stories 98.1-98.6: OAuth and integration implementations

2. **Epic 99** - Test Infrastructure & Coverage (P2) ~1 week
   - Stories 99.1-99.4: Complete integration tests

3. **Epic 100** - Repository & Data Layer (P2) ~1 week
   - Stories 100.1-100.4: Complete repository stubs

### Parallel Implementation

- Epic 99 and Epic 100 can be worked in parallel (different domains)
- Epic 98 should be completed first as it has P1 priority
- Story 98.3-98.4 (Airbnb/Booking) can be parallelized
- Story 99.1-99.4 are independent and can be parallelized

### Dependencies

```
Epic 98.1 (Voice OAuth) → requires Epic 91 (LLM client) ✓
Epic 98.2 (Rental OAuth) → requires Epic 96 (OAuth Phase 2) ✓
Epic 98.3-98.4 (Integrations) → independent
Epic 99 (Tests) → benefits from Epic 98 completion
Epic 100 (Repos) → independent
```

### Feature Flags

No new feature flags required - these complete existing features.
