---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - _bmad-output/prd.md
  - _bmad-output/architecture.md
  - _bmad-output/epics-008.md
  - _bmad-output/implementation-artifacts/gap-analysis-remediation.md
workflowType: 'epics-and-stories'
lastStep: 4
status: 'ready'
project_name: 'Property Management System (PPT) & Reality Portal'
user_name: 'Martin Janci'
date: '2025-12-31'
continues_from: 'epics-008.md'
phase_range: '26'
epic_range: '88-90'
---

# Property Management System (PPT) & Reality Portal - Epic Breakdown (Part 9)

## Overview

This document continues from `epics-008.md` and addresses **Phase 26: Infrastructure Completion** - implementing stubbed features, wiring frontend APIs, and completing monitoring infrastructure.

**Continuation from:** `epics-008.md` (Epics 86-87, Phase 25)

**Source:** Comprehensive gap analysis (2025-12-31)

---

## Epic List

### Phase 26: Infrastructure Completion

#### Epic 88: Quick Wins & Small Fixes
**Goal:** Complete small stubbed features that have minimal effort but improve system completeness.

**Target Apps:** api-server, ppt-web
**Estimate:** 5 stories, ~3 days
**Dependencies:** Epic 86 (completed)
**Priority:** P1 - HIGH

---

##### Story 88.1: Implement Uptime Tracking

As a **DevOps engineer**,
I want to **see accurate system uptime in health checks**,
So that **I can monitor system availability**.

**Acceptance Criteria:**

**Given** the health endpoint is called
**When** the system returns health status
**Then**:
  - Actual uptime in seconds is returned
  - Start time is tracked from application boot
  - Response includes `uptime_seconds` field
**And** value increases monotonically

**Technical Notes:**
- Use `std::time::Instant` or `chrono` for boot time tracking
- Store boot time in AppState
- Calculate uptime on each health check request

**Files to Modify:**
- `backend/servers/api-server/src/state.rs` (add boot_time)
- `backend/servers/api-server/src/routes/infrastructure.rs` (calculate uptime)

---

##### Story 88.2: Store Signed Documents

As a **user**,
I want to **access my signed documents later**,
So that **I have a permanent record**.

**Acceptance Criteria:**

**Given** a document is signed via e-signature
**When** the signed version is downloaded
**Then**:
  - Signed document is stored in document repository
  - Original document is linked to signed version
  - Signed document has `signed_` prefix in filename
**And** document appears in user's document list

**Technical Notes:**
- Reference: signatures.rs:519
- Use existing document repository for storage
- Create new document record with signed metadata

**Files to Modify:**
- `backend/servers/api-server/src/routes/signatures.rs`
- `backend/crates/db/src/repositories/documents.rs` (if needed)

---

##### Story 88.3: Implement Facilities Pagination

As a **facility manager**,
I want to **paginate through bookings**,
So that **I can manage large booking lists**.

**Acceptance Criteria:**

**Given** many facility bookings exist
**When** I navigate the bookings list
**Then**:
  - Bookings are paginated (default 20 per page)
  - Page navigation works correctly
  - Total count is displayed
**And** filters work with pagination

**Technical Notes:**
- Wire up existing pagination props in:
  - PendingBookingsPage.tsx
  - MyBookingsPage.tsx
  - FacilitiesPage.tsx
- Use API pagination parameters

**Files to Modify:**
- `frontend/apps/ppt-web/src/features/facilities/pages/PendingBookingsPage.tsx`
- `frontend/apps/ppt-web/src/features/facilities/pages/MyBookingsPage.tsx`
- `frontend/apps/ppt-web/src/features/facilities/pages/FacilitiesPage.tsx`

---

##### Story 88.4: Complete Country Name Mapping

As a **rental manager**,
I want to **see full country names in listings**,
So that **property locations are clear**.

**Acceptance Criteria:**

**Given** a rental property has country code
**When** the property is displayed
**Then**:
  - Full country name is shown (e.g., "Slovakia" not "SK")
  - All supported country codes are mapped
  - Unmapped codes fall back to code display
**And** mapping is consistent across API and UI

**Technical Notes:**
- Reference: rental.rs:1045
- Add country code to name mapping
- Consider using ISO 3166-1 library

**Files to Modify:**
- `backend/crates/db/src/repositories/rental.rs`

---

##### Story 88.5: Implement Report Export Fallback

As a **manager**,
I want to **download small reports synchronously**,
So that **I don't wait for async processing**.

**Acceptance Criteria:**

**Given** a report with < 1000 rows is requested
**When** export is triggered
**Then**:
  - Report is generated synchronously
  - Download starts immediately
  - No background job is created
**And** large reports still use async processing

**Technical Notes:**
- Reference: reports.rs:726
- Check row count before deciding sync/async
- Reuse existing report generation logic

**Files to Modify:**
- `backend/servers/api-server/src/routes/reports.rs`

---

#### Epic 89: Feature Flags & Health Monitoring
**Goal:** Complete the infrastructure monitoring features with actual database storage.

**Target Apps:** api-server
**Estimate:** 5 stories, ~1 week
**Dependencies:** Epic 88
**Priority:** P2 - MEDIUM

---

##### Story 89.1: Implement Feature Flag Storage

As a **platform admin**,
I want to **manage feature flags with persistence**,
So that **flag states survive restarts**.

**Acceptance Criteria:**

**Given** feature flags are configured
**When** the server restarts
**Then**:
  - All flag states are preserved
  - Flag overrides are persisted
  - Audit log captures all changes
**And** flags can be toggled via API

**Technical Notes:**
- Feature flag tables exist in DB
- Wire up repository to routes
- Implement list/get/update/delete operations

**Files to Modify:**
- `backend/servers/api-server/src/routes/infrastructure.rs` (lines 357, 386)
- `backend/crates/db/src/repositories/` (feature_flags.rs if exists)

---

##### Story 89.2: Implement Feature Flag Overrides

As a **developer**,
I want to **override feature flags for testing**,
So that **I can test features in production safely**.

**Acceptance Criteria:**

**Given** a feature flag exists
**When** I create an override for specific user/org
**Then**:
  - Override applies only to that context
  - Base flag value unchanged for others
  - Override can be removed
**And** overrides are listed via API

**Technical Notes:**
- Reference: infrastructure.rs:509 (list_flag_overrides returns empty)
- Implement override CRUD operations

**Files to Modify:**
- `backend/servers/api-server/src/routes/infrastructure.rs`
- `backend/crates/db/src/repositories/feature_flags.rs`

---

##### Story 89.3: Implement Health Check Storage

As a **DevOps engineer**,
I want to **view health check history**,
So that **I can identify recurring issues**.

**Acceptance Criteria:**

**Given** health checks run periodically
**When** I query health check history
**Then**:
  - All check results are stored
  - Results include timestamp, status, latency
  - History can be filtered by service/time
**And** old results are cleaned up automatically

**Technical Notes:**
- Reference: infrastructure.rs:987, 1030
- Implement health check result storage
- Add cleanup job for old results

**Files to Modify:**
- `backend/servers/api-server/src/routes/infrastructure.rs`
- `backend/crates/db/src/repositories/health_checks.rs`

---

##### Story 89.4: Implement Alert System

As a **operator**,
I want to **receive and manage alerts**,
So that **I can respond to system issues**.

**Acceptance Criteria:**

**Given** system conditions trigger alerts
**When** an alert is generated
**Then**:
  - Alert is stored in database
  - Alert appears in list endpoint
  - Alert can be acknowledged
  - Alert rules can be configured
**And** acknowledged alerts are marked

**Technical Notes:**
- Reference: infrastructure.rs:1045-1150
- Alert tables exist in DB schema
- Implement alert CRUD and acknowledgment

**Files to Modify:**
- `backend/servers/api-server/src/routes/infrastructure.rs`
- `backend/crates/db/src/repositories/alerts.rs`

---

##### Story 89.5: Implement Flag Audit Logging

As a **compliance officer**,
I want to **audit feature flag changes**,
So that **changes are traceable**.

**Acceptance Criteria:**

**Given** feature flags are modified
**When** changes occur
**Then**:
  - Change is logged with timestamp, user, old/new value
  - Audit log is queryable
  - Logs are immutable
**And** audit endpoint returns history

**Technical Notes:**
- Reference: infrastructure.rs:577 (returns empty)
- Create audit log entries on flag changes
- Implement read-only audit endpoint

**Files to Modify:**
- `backend/servers/api-server/src/routes/infrastructure.rs`
- `backend/crates/db/src/repositories/feature_flags.rs`

---

#### Epic 90: Frontend API Integration
**Goal:** Wire up frontend handlers to existing backend APIs.

**Target Apps:** ppt-web
**Estimate:** 6 stories, ~1 week
**Dependencies:** Epic 88
**Priority:** P2 - MEDIUM

---

##### Story 90.1: Package Management Handlers

As a **doorman/concierge**,
I want to **mark packages as received/picked up**,
So that **package tracking is accurate**.

**Acceptance Criteria:**

**Given** a package is registered
**When** I mark it as received/picked up
**Then**:
  - API is called with correct package ID
  - Status updates in UI immediately
  - Success/error feedback is shown
**And** package list refreshes

**Technical Notes:**
- Reference: PackagesPage.tsx TODO comments
- Use existing package API endpoints
- Add mutation hooks

**Files to Modify:**
- `frontend/apps/ppt-web/src/features/packages/pages/PackagesPage.tsx`
- `frontend/apps/ppt-web/src/features/packages/hooks/` (if needed)

---

##### Story 90.2: Visitor Check-in/out Handlers

As a **security guard**,
I want to **check visitors in and out**,
So that **visitor logs are maintained**.

**Acceptance Criteria:**

**Given** a visitor registration exists
**When** I check them in/out
**Then**:
  - API records check-in/out time
  - Status updates in visitor list
  - Badge printing option available
**And** visitor can be cancelled if no-show

**Technical Notes:**
- Reference: VisitorsPage.tsx TODO comments
- Wire up check-in/out/cancel API calls

**Files to Modify:**
- `frontend/apps/ppt-web/src/features/packages/pages/VisitorsPage.tsx`

---

##### Story 90.3: Migration Import Handlers

As a **admin**,
I want to **import data from templates**,
So that **migration is streamlined**.

**Acceptance Criteria:**

**Given** an import template is available
**When** I download and upload it
**Then**:
  - Template download works
  - Import validation runs
  - Import progress is shown
  - Retry option for failed imports
**And** import history is maintained

**Technical Notes:**
- Reference: ImportPage.tsx TODO comments
- Wire up template download, import, retry APIs

**Files to Modify:**
- `frontend/apps/ppt-web/src/features/migration/pages/ImportPage.tsx`

---

##### Story 90.4: Content Moderation Handlers

As a **compliance officer**,
I want to **moderate content effectively**,
So that **platform remains compliant**.

**Acceptance Criteria:**

**Given** content is flagged for moderation
**When** I take action
**Then**:
  - Case can be assigned to me
  - Actions (approve/reject/escalate) work
  - Notes can be added
**And** moderation history is tracked

**Technical Notes:**
- Reference: ContentModerationPage.tsx TODO comments
- Wire up case assignment, action modals

**Files to Modify:**
- `frontend/apps/ppt-web/src/features/compliance/pages/ContentModerationPage.tsx`

---

##### Story 90.5: AML Dashboard Handlers

As a **compliance officer**,
I want to **manage AML checks**,
So that **regulatory requirements are met**.

**Acceptance Criteria:**

**Given** AML checks are required
**When** I manage them via dashboard
**Then**:
  - EDD initiation triggers correctly
  - Review modal captures decisions
  - Status updates reflect in UI
**And** audit trail is maintained

**Technical Notes:**
- Reference: AmlDashboardPage.tsx TODO comments
- Wire up EDD initiation, review modal

**Files to Modify:**
- `frontend/apps/ppt-web/src/features/compliance/pages/AmlDashboardPage.tsx`

---

##### Story 90.6: DSA Reports Handlers

As a **compliance officer**,
I want to **publish and export DSA reports**,
So that **transparency requirements are met**.

**Acceptance Criteria:**

**Given** a DSA report is ready
**When** I publish or export it
**Then**:
  - Publish updates status
  - PDF download generates report
  - Published reports are accessible
**And** report history is tracked

**Technical Notes:**
- Reference: DsaReportsPage.tsx TODO comments
- Wire up publish and PDF download

**Files to Modify:**
- `frontend/apps/ppt-web/src/features/compliance/pages/DsaReportsPage.tsx`

---

## Summary

| Phase | Epics | Stories | Priority |
|-------|-------|---------|----------|
| 26: Infrastructure Completion | 88-90 | 16 | P1-P2 |

**Total:** 3 Epics, 16 Stories

### Implementation Order

1. **Epic 88** - Quick Wins (P1) ~3 days
   - Stories 88.1-88.5: Small impactful fixes

2. **Epic 89** - Feature Flags & Health Monitoring (P2) ~1 week
   - Stories 89.1-89.5: Complete infrastructure monitoring

3. **Epic 90** - Frontend API Integration (P2) ~1 week
   - Stories 90.1-90.6: Wire up frontend to backend APIs

### Parallel Implementation

Epics 88, 89, and 90 can be implemented in parallel as they target different areas:
- Epic 88: Mixed backend/frontend quick fixes
- Epic 89: Pure backend infrastructure
- Epic 90: Pure frontend API wiring
