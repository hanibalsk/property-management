---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - _bmad-output/prd.md
  - _bmad-output/architecture.md
  - _bmad-output/epics-007.md
  - _bmad-output/implementation-artifacts/gap-analysis-remediation.md
workflowType: 'epics-and-stories'
lastStep: 4
status: 'ready'
project_name: 'Property Management System (PPT) & Reality Portal'
user_name: 'Martin Janci'
date: '2025-12-31'
continues_from: 'epics-007.md'
phase_range: '25'
epic_range: '86-87'
---

# Property Management System (PPT) & Reality Portal - Epic Breakdown (Part 8)

## Overview

This document continues from `epics-007.md` and addresses **final polish and cleanup** after the comprehensive security hardening and test infrastructure work.

**Continuation from:** `epics-007.md` (Epics 79-85, Phases 22-24)

**Source:** Post-implementation gap analysis (2025-12-31)

---

## Epic List

### Phase 25: Final Polish & Cleanup

#### Epic 86: Code Quality & Test Stability
**Goal:** Remove dead code, fix test environment issues, and clean up debugging artifacts.

**Target Apps:** api-server, ppt-web
**Estimate:** 5 stories, ~1 week
**Dependencies:** Epic 79-85 (completed)
**Priority:** P2 - MEDIUM

---

##### Story 86.1: Fix TOTP Test Environment Configuration

As a **developer**,
I want to **run TOTP tests without manual environment setup**,
So that **CI tests pass reliably**.

**Acceptance Criteria:**

**Given** the test suite runs
**When** TOTP tests execute
**Then**:
  - RUST_ENV=development is set in test fixtures
  - All 6 TOTP tests pass
  - No panic on missing environment variables
**And** tests work in CI without additional setup

**Technical Notes:**
- Add `#[ctor::ctor]` or test setup to set RUST_ENV=development
- Alternative: Use `temp_env` crate for test env management
- Ensure CI workflow doesn't need changes

**Files to Modify:**
- `backend/servers/api-server/src/services/totp.rs` (tests module)

---

##### Story 86.2: Remove Dead Handler Stubs

As a **developer**,
I want to **remove empty handler modules**,
So that **the codebase is cleaner**.

**Acceptance Criteria:**

**Given** empty handler modules exist
**When** I check for dead code
**Then**:
  - Empty `handlers/rentals/mod.rs` is removed
  - Empty `handlers/listings/mod.rs` is removed
  - Empty `handlers/organizations/mod.rs` is removed
  - Empty `handlers/integrations/mod.rs` is removed
**And** no compilation errors occur

**Technical Notes:**
- These modules only contain TODO comments
- Routes are already implemented in `routes/` directory
- Update `handlers/mod.rs` to remove unused module declarations

**Files to Modify:**
- `backend/servers/api-server/src/handlers/mod.rs`
- Delete: `backend/servers/api-server/src/handlers/rentals/mod.rs`
- Delete: `backend/servers/api-server/src/handlers/listings/mod.rs`
- Delete: `backend/servers/api-server/src/handlers/organizations/mod.rs`
- Delete: `backend/servers/api-server/src/handlers/integrations/mod.rs`

---

##### Story 86.3: Remove Frontend Console.log Statements

As a **developer**,
I want to **remove debugging console.log statements**,
So that **production bundles are clean**.

**Acceptance Criteria:**

**Given** 54 console.log statements exist across 11 files
**When** I clean up the codebase
**Then**:
  - All console.log statements are replaced with proper handlers or removed
  - Registry page handlers call API (16 handlers)
  - Developer portal handlers call API (13 handlers)
  - All other files cleaned up
**And** ESLint no-console rule passes

**Technical Notes:**
- Files with console.log:
  - `registry/pages/RegistryPage.tsx` (16)
  - `developer/pages/DeveloperPortalPage.tsx` (13)
  - `competitive/pages/CompetitiveAnalysisPage.tsx` (5)
  - `compliance/pages/ContentModerationPage.tsx` (5)
  - `packages/pages/VisitorsPage.tsx` (4)
  - `packages/pages/PackagesPage.tsx` (3)
  - And 5 more files with 1-2 each

**Files to Modify:**
- `frontend/apps/ppt-web/src/features/registry/pages/RegistryPage.tsx`
- `frontend/apps/ppt-web/src/features/developer/pages/DeveloperPortalPage.tsx`
- And 9 other files listed above

---

##### Story 86.4: Wire Empty Frontend Handlers

As a **user**,
I want to **buttons that do something when clicked**,
So that **the UI is functional**.

**Acceptance Criteria:**

**Given** 20 empty `() => {}` handlers exist
**When** I click buttons
**Then**:
  - Handlers either call API or
  - Buttons are hidden/disabled for unimplemented features
**And** no silent failures occur

**Technical Notes:**
- Files with empty handlers:
  - `forms/pages/ViewFormPage.tsx` (4)
  - `migration/pages/ImportPage.tsx` (7)
  - `facilities/pages/*.tsx` (3 files, 1 each)
  - `migration/pages/ExportPage.tsx` (1)
- Either implement or add disabled state with tooltip

**Files to Modify:**
- `frontend/apps/ppt-web/src/features/forms/pages/ViewFormPage.tsx`
- `frontend/apps/ppt-web/src/features/migration/pages/ImportPage.tsx`
- `frontend/apps/ppt-web/src/features/migration/pages/ExportPage.tsx`
- `frontend/apps/ppt-web/src/features/facilities/pages/PendingBookingsPage.tsx`
- `frontend/apps/ppt-web/src/features/facilities/pages/FacilitiesPage.tsx`
- `frontend/apps/ppt-web/src/features/facilities/pages/MyBookingsPage.tsx`

---

##### Story 86.5: Update Gap Analysis Tracker

As a **developer**,
I want to **update the gap analysis document**,
So that **it reflects current state**.

**Acceptance Criteria:**

**Given** the gap analysis tracker
**When** I review completed work
**Then**:
  - All completed items are marked as Fixed
  - Pending items are updated with accurate status
  - Agent assignments are cleared for completed work
**And** document serves as accurate reference

**Technical Notes:**
- Update `_bmad-output/implementation-artifacts/gap-analysis-remediation.md`
- Mark routes as implemented (not handlers)
- Update frontend status based on actual cleanup
- Remove outdated agent references

**Files to Modify:**
- `_bmad-output/implementation-artifacts/gap-analysis-remediation.md`

---

#### Epic 87: Phase 2 Preparation
**Goal:** Document and scaffold Phase 2 features (OAuth integrations, AI assistant, infrastructure tracing).

**Target Apps:** api-server
**Estimate:** 3 stories, ~1 week
**Dependencies:** Epic 86 (Code Quality)
**Priority:** P3 - LOW

---

##### Story 87.1: Document OAuth Integration Requirements

As a **architect**,
I want to **document Airbnb/Booking OAuth requirements**,
So that **Phase 2 implementation is clear**.

**Acceptance Criteria:**

**Given** TODO comments exist for OAuth flows
**When** I document requirements
**Then**:
  - Airbnb OAuth2 flow is documented
  - Booking.com OAuth2 flow is documented
  - Token storage requirements are specified
  - Refresh token handling is designed
**And** document is added to docs/

**Technical Notes:**
- Reference lines: rentals.rs:345, rentals.rs:366
- Create `docs/phase2/oauth-integrations.md`
- Include sequence diagrams

**Files to Create:**
- `docs/phase2/oauth-integrations.md`

---

##### Story 87.2: Document AI Assistant Architecture

As a **architect**,
I want to **document AI assistant response flow**,
So that **Phase 2 implementation is planned**.

**Acceptance Criteria:**

**Given** AI assistant TODO exists
**When** I document architecture
**Then**:
  - LLM provider selection is documented
  - Conversation context handling is designed
  - Rate limiting strategy is specified
  - Cost estimation is provided
**And** document guides implementation

**Technical Notes:**
- Reference line: ai.rs:271
- Create `docs/phase2/ai-assistant.md`
- Consider OpenAI/Anthropic/local models

**Files to Create:**
- `docs/phase2/ai-assistant.md`

---

##### Story 87.3: Document Infrastructure Tracing Strategy

As a **DevOps engineer**,
I want to **document tracing infrastructure requirements**,
So that **observability is properly planned**.

**Acceptance Criteria:**

**Given** TODO comments exist for tracing
**When** I document requirements
**Then**:
  - Trace storage backend is selected (Jaeger/Tempo/etc)
  - Span ingestion pipeline is designed
  - Feature flag storage is specified
  - Integration with existing infrastructure is planned
**And** document guides implementation

**Technical Notes:**
- Reference lines: infrastructure.rs:279-386
- Create `docs/phase2/infrastructure-tracing.md`
- Consider OpenTelemetry collector

**Files to Create:**
- `docs/phase2/infrastructure-tracing.md`

---

## Summary

| Phase | Epics | Stories | Priority |
|-------|-------|---------|----------|
| 25: Final Polish | 86-87 | 8 | P2-P3 |

**Total:** 2 Epics, 8 Stories

### Implementation Order

1. **Epic 86** - Code Quality & Test Stability (P2)
   - Story 86.1: Fix TOTP tests (blocks CI)
   - Story 86.2: Remove dead code
   - Story 86.3: Remove console.logs
   - Story 86.4: Wire empty handlers
   - Story 86.5: Update tracker

2. **Epic 87** - Phase 2 Preparation (P3)
   - Story 87.1-87.3: Documentation only, no code changes
