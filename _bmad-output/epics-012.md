---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - _bmad-output/prd.md
  - _bmad-output/architecture.md
  - _bmad-output/epics-011.md
workflowType: 'epics-and-stories'
lastStep: 4
status: 'ready'
project_name: 'Property Management System (PPT) & Reality Portal'
user_name: 'Martin Janci'
date: '2026-01-02'
continues_from: 'epics-011.md'
phase_range: '31-33'
epic_range: '102-106'
---

# Property Management System (PPT) & Reality Portal - Epic Breakdown (Part 12)

## Overview

This document continues from `epics-011.md` and addresses **Phase 31: Infrastructure Completion**, **Phase 32: Production Hardening**, and **Phase 33: Portal Syndication & Notifications** - completing remaining infrastructure TODOs, production-ready storage/caching, and portal integration.

**Continuation from:** `epics-011.md` (Epics 98-100, Phases 29-30)

**Source:** Comprehensive codebase analysis (2026-01-02)

**Key Findings from Codebase Analysis:**
- Trace storage and alert rules not implemented (infrastructure.rs)
- S3 storage upload missing for signed documents (signatures.rs:604)
- Redis client integration incomplete (health.rs, announcements.rs)
- pgvector migration needed for RAG performance (llm_document.rs:434)
- Portal syndication async jobs not triggered (listings.rs:565)
- Scheduled notification triggers pending (scheduler.rs:91)

---

## Epic List

### Phase 31: Infrastructure Completion

#### Epic 102: Distributed Tracing & Observability
**Goal:** Complete trace storage, job statistics, and alert rule management for production observability.

**Target Apps:** api-server
**Estimate:** 4 stories, ~1 week
**Dependencies:** Epic 89 (Feature Flags)
**Priority:** P1 - HIGH

**PRD Reference:** NFR-REL - System reliability, Monitoring & Observability requirements

---

##### Story 102.1: Trace Storage Implementation

As a **DevOps engineer**,
I want to **store and query distributed traces**,
So that **I can debug production issues effectively**.

**Acceptance Criteria:**

**Given** a request traverses the system
**When** the trace is completed
**Then**:
  - Trace metadata is persisted to database
  - All spans within the trace are stored
  - Traces are queryable by service, operation, duration
  - Error traces are flagged and filterable
  - Traces older than retention period are cleaned up
**And** trace UI shows real trace data

**Technical Notes:**
- Reference: `infrastructure.rs:293` - TODO for trace storage
- Create `traces` and `spans` tables if not exist
- Integrate with OpenTelemetry collector or direct storage
- Implement background cleanup for retention policy (default 7 days)

**Files to Modify:**
- `backend/crates/db/src/repositories/infrastructure.rs` (add trace storage methods)
- `backend/servers/api-server/src/routes/infrastructure.rs` (wire to real storage)
- `backend/crates/db/src/migrations/` (add trace tables if needed)

---

##### Story 102.2: Job Type Statistics Query

As a **system administrator**,
I want to **view statistics grouped by job type**,
So that **I can identify problematic job categories**.

**Acceptance Criteria:**

**Given** background jobs have been executed
**When** job type stats are requested
**Then**:
  - Total count per job type is accurate
  - Success rate is calculated correctly
  - Average duration is computed from executions
  - Pending and failed counts are current
  - Stats are filterable by time range
**And** dashboard shows real statistics

**Technical Notes:**
- Reference: `infrastructure.rs:1180` - TODO for job type stats
- Aggregate from `background_jobs` and `background_job_executions` tables
- Consider materialized view for performance on large datasets

**Files to Modify:**
- `backend/crates/db/src/repositories/background_job.rs` (add get_type_stats method)
- `backend/servers/api-server/src/routes/infrastructure.rs` (call real method)

---

##### Story 102.3: Alert Rules Management

As a **DevOps engineer**,
I want to **configure alert rules in the database**,
So that **alerts trigger based on customizable thresholds**.

**Acceptance Criteria:**

**Given** alert rules are configured
**When** thresholds are exceeded
**Then**:
  - Rules are stored and retrieved from database
  - Rules support multiple condition types (threshold, rate, pattern)
  - Rules can be enabled/disabled
  - Rule evaluation triggers alerts
  - Notification channels are configurable per rule
**And** alert rules API returns real data

**Technical Notes:**
- Reference: `infrastructure.rs:1554` - TODO for alert rules listing
- Create `health_alert_rules` table with conditions JSON
- Implement rule evaluation in background scheduler
- Support email, webhook, and in-app notification channels

**Files to Modify:**
- `backend/crates/db/src/repositories/infrastructure.rs` (add alert rules CRUD)
- `backend/servers/api-server/src/routes/infrastructure.rs` (implement list_alert_rules)
- `backend/servers/api-server/src/services/scheduler.rs` (add rule evaluation task)

---

##### Story 102.4: Prometheus Metrics Export

As a **DevOps engineer**,
I want to **export real Prometheus metrics**,
So that **Grafana dashboards show live data**.

**Acceptance Criteria:**

**Given** the metrics endpoint is called
**When** metrics are requested
**Then**:
  - HTTP request counters are accurate
  - Request duration histograms are populated
  - Database connection pool metrics are exposed
  - Background job queue depths are tracked
  - Memory and CPU usage are reported
**And** Prometheus can scrape /metrics endpoint

**Technical Notes:**
- Add `metrics` crate for Prometheus registry
- Instrument Axum middleware for HTTP metrics
- Add connection pool metrics from SQLx
- Expose metrics in text format at /api/v1/infrastructure/health/metrics

**Files to Modify:**
- `backend/servers/api-server/Cargo.toml` (add metrics crate)
- `backend/servers/api-server/src/routes/infrastructure.rs` (real metrics export)
- `backend/servers/api-server/src/middleware/` (add metrics middleware)

---

### Phase 32: Production Hardening

#### Epic 103: Storage & Caching Integration
**Goal:** Complete S3 storage integration and Redis caching for production readiness.

**Target Apps:** api-server, reality-server
**Estimate:** 5 stories, ~1.5 weeks
**Dependencies:** None
**Priority:** P1 - HIGH

**PRD Reference:** ADR-006 (Redis), ADR-007 (S3), NFR-PERF (Performance)

---

##### Story 103.1: S3 Document Upload Implementation

As a **user uploading documents**,
I want to **store documents in S3-compatible storage**,
So that **documents are reliably persisted and accessible**.

**Acceptance Criteria:**

**Given** a document upload request
**When** the document is uploaded
**Then**:
  - File is uploaded to S3 bucket with correct key
  - Metadata is stored in database
  - Pre-signed URLs are generated for access
  - Upload progress is trackable for large files
  - Failed uploads are retryable
**And** signed documents are stored correctly

**Technical Notes:**
- Reference: `signatures.rs:604` - TODO for S3 storage upload
- Use aws-sdk-s3 crate for S3 operations
- Implement multipart upload for files > 5MB
- Generate pre-signed URLs with configurable expiry

**Files to Modify:**
- `backend/crates/integrations/src/storage.rs` (implement real S3 upload)
- `backend/servers/api-server/src/routes/signatures.rs` (use storage service)
- `backend/servers/api-server/src/routes/documents.rs` (use storage service)

---

##### Story 103.2: Redis Client Integration

As a **developer**,
I want to **use Redis for caching and sessions**,
So that **system performance improves and sessions scale horizontally**.

**Acceptance Criteria:**

**Given** Redis is configured
**When** the application starts
**Then**:
  - Redis connection pool is established
  - Health check includes Redis status
  - Session data can be stored/retrieved
  - Cache operations work correctly
  - Graceful fallback when Redis unavailable
**And** multiple server instances share session state

**Technical Notes:**
- Reference: `health.rs:129` - TODO for Redis health check
- Reference: `announcements.rs:902` - Redis pub/sub integration
- Use `redis` crate with connection pooling
- Add Redis URL to configuration

**Files to Modify:**
- `backend/servers/api-server/src/state.rs` (add Redis client)
- `backend/servers/api-server/src/routes/health.rs` (add Redis health check)
- `backend/crates/api-core/src/config.rs` (add Redis configuration)

---

##### Story 103.3: Redis Session Storage

As a **user with active sessions**,
I want to **sessions stored in Redis**,
So that **I remain logged in across server restarts**.

**Acceptance Criteria:**

**Given** a user logs in
**When** a session is created
**Then**:
  - Session data is stored in Redis with TTL
  - Session is retrievable from any server instance
  - Session expiry is enforced automatically
  - Session can be invalidated on logout
  - Session count per user is trackable
**And** sessions persist across deployments

**Technical Notes:**
- Store refresh tokens and session metadata in Redis
- Use Redis SETEX for automatic TTL
- Implement session listing for "active sessions" UI
- Add session revocation for security

**Files to Modify:**
- `backend/servers/api-server/src/services/session.rs` (create Redis session service)
- `backend/servers/api-server/src/routes/auth.rs` (use session service)

---

##### Story 103.4: Redis Pub/Sub for Real-time

As a **user viewing the dashboard**,
I want to **receive real-time updates**,
So that **I see changes immediately without refreshing**.

**Acceptance Criteria:**

**Given** an event occurs (announcement, fault update)
**When** the event is published
**Then**:
  - Event is published to Redis channel
  - All server instances receive the event
  - WebSocket clients are notified
  - Event delivery is reliable
  - Dead letter queue for failed deliveries
**And** updates appear in real-time across clients

**Technical Notes:**
- Reference: `announcements.rs:902` - Redis pub/sub mention
- Use Redis PUBLISH/SUBSCRIBE for cross-instance messaging
- Bridge Redis events to WebSocket connections
- Implement channel per organization for isolation

**Files to Modify:**
- `backend/servers/api-server/src/services/pubsub.rs` (create pub/sub service)
- `backend/servers/api-server/src/routes/websocket.rs` (integrate with pub/sub)
- `backend/servers/api-server/src/routes/announcements.rs` (publish events)

---

##### Story 103.5: pgvector Migration for RAG

As a **system using AI document search**,
I want to **efficient vector similarity search**,
So that **RAG queries are fast and accurate**.

**Acceptance Criteria:**

**Given** documents with embeddings exist
**When** a similarity search is performed
**Then**:
  - pgvector extension is installed
  - Embeddings are stored as vector type
  - Cosine similarity uses database index
  - Search latency < 100ms for 100K documents
  - Fallback to application-level similarity if unavailable
**And** RAG performance improves significantly

**Technical Notes:**
- Reference: `llm_document.rs:434` - TODO for pgvector migration
- Add migration to enable pgvector extension
- Alter embedding column to vector(1536) type
- Create HNSW or IVFFlat index for fast search
- Update queries to use <=> operator

**Files to Modify:**
- `backend/crates/db/src/migrations/` (add pgvector migration)
- `backend/crates/db/src/repositories/llm_document.rs` (use native vector ops)

---

#### Epic 104: Cross-Server Health & SSO
**Goal:** Complete health checks between servers and SSO verification.

**Target Apps:** reality-server, api-server
**Estimate:** 2 stories, ~0.5 weeks
**Dependencies:** Epic 103 (Redis)
**Priority:** P2 - MEDIUM

**PRD Reference:** ADR-002 (Two servers), SSO architecture

---

##### Story 104.1: PM API Health Check in Reality Server

As a **DevOps engineer monitoring Reality Portal**,
I want to **verify PM API connectivity**,
So that **SSO and shared data access is validated**.

**Acceptance Criteria:**

**Given** Reality Portal health endpoint is called
**When** the check includes SSO dependency
**Then**:
  - PM API reachability is tested
  - SSO token validation endpoint is verified
  - Latency is measured and reported
  - Degraded status reflects PM API issues
  - Health check is cached to prevent DoS
**And** Reality Portal health reflects PM API status

**Technical Notes:**
- Reference: `reality-server/routes/health.rs:128` - TODO for PM API health
- HTTP client call to api-server /health endpoint
- Include in dependencies array with latency
- Cache result for 30 seconds to prevent cascading failures

**Files to Modify:**
- `backend/servers/reality-server/src/routes/health.rs` (add PM API check)
- `backend/servers/reality-server/src/state.rs` (add HTTP client for api-server)

---

##### Story 104.2: SSO Token Validation Caching

As a **Reality Portal user**,
I want to **fast SSO token validation**,
So that **authenticated requests are quick**.

**Acceptance Criteria:**

**Given** a user is authenticated via SSO
**When** subsequent requests are made
**Then**:
  - Token validation result is cached in Redis
  - Cache TTL matches token expiry
  - Cache invalidation on token refresh
  - Fallback to direct validation if cache miss
  - Cache hit rate is tracked
**And** validation latency drops significantly

**Technical Notes:**
- Cache JWT validation results by token hash
- Use Redis with short TTL (5 minutes)
- Include user claims in cache for faster access
- Track cache hit/miss metrics

**Files to Modify:**
- `backend/servers/reality-server/src/middleware/auth.rs` (add caching)
- `backend/servers/reality-server/src/state.rs` (add Redis client)

---

### Phase 33: Portal Syndication & Notifications

#### Epic 105: Listing Portal Syndication
**Goal:** Complete async syndication to external real estate portals.

**Target Apps:** api-server
**Estimate:** 4 stories, ~1 week
**Dependencies:** Epic 98 (Integrations), Epic 103 (Redis/Jobs)
**Priority:** P1 - HIGH

**PRD Reference:** UC-32 - External portal integration, FR35

---

##### Story 105.1: Syndication Job Queue

As a **property manager publishing listings**,
I want to **listings syndicated asynchronously**,
So that **publishing is fast and reliable**.

**Acceptance Criteria:**

**Given** a listing is published
**When** syndication is requested
**Then**:
  - Background job is created for each portal
  - Jobs are processed asynchronously
  - Progress is trackable per portal
  - Failures are retried with backoff
  - Final status is updated on listing
**And** user can see syndication progress

**Technical Notes:**
- Reference: `listings.rs:565` - TODO for async syndication jobs
- Create job per portal with listing_id in payload
- Use existing background job infrastructure
- Implement syndication worker service

**Files to Modify:**
- `backend/servers/api-server/src/routes/listings.rs` (create syndication jobs)
- `backend/servers/api-server/src/services/syndication.rs` (create worker)
- `backend/servers/api-server/src/services/scheduler.rs` (add job processor)

---

##### Story 105.2: Status Change Propagation

As a **property manager updating listing status**,
I want to **status changes propagated to portals**,
So that **all portals show consistent status**.

**Acceptance Criteria:**

**Given** a listing status is changed
**When** the listing is syndicated
**Then**:
  - Status change triggers update on all portals
  - Portal-specific status mapping is applied
  - Failed updates are retried
  - Partial failures don't block others
  - Change history is logged
**And** portals reflect new status within minutes

**Technical Notes:**
- Reference: `listings.rs:474` - TODO for status propagation
- Map internal status to portal-specific status codes
- Trigger update jobs for each active syndication
- Log status transition for audit

**Files to Modify:**
- `backend/servers/api-server/src/routes/listings.rs` (trigger propagation)
- `backend/servers/api-server/src/services/syndication.rs` (handle status update)

---

##### Story 105.3: Syndication Status Dashboard

As a **property manager**,
I want to **view syndication status for all listings**,
So that **I know which portals have my listings**.

**Acceptance Criteria:**

**Given** listings are syndicated
**When** the dashboard is viewed
**Then**:
  - Each listing shows syndication status per portal
  - Last sync time is displayed
  - Errors are shown with details
  - Retry is available for failed syndications
  - Bulk actions are supported
**And** manager has full visibility into syndication

**Technical Notes:**
- Aggregate syndication status from listing_syndications table
- Include external_id and external_url per portal
- Show sync history with timestamps

**Files to Modify:**
- `backend/servers/api-server/src/routes/listings.rs` (add syndication status endpoint)
- `backend/crates/db/src/repositories/listing.rs` (add syndication queries)

---

##### Story 105.4: Portal Webhook Receivers

As a **system receiving portal updates**,
I want to **process webhooks from portals**,
So that **external changes sync back to our system**.

**Acceptance Criteria:**

**Given** an external portal sends a webhook
**When** the webhook is received
**Then**:
  - Webhook signature is validated
  - Listing is matched by external_id
  - Changes are applied (views, inquiries)
  - Unknown events are logged not rejected
  - Duplicate events are deduplicated
**And** portal metrics sync automatically

**Technical Notes:**
- Create webhook endpoints per portal type
- Validate signatures per portal specification
- Update listing analytics (views, favorites)
- Create inquiries from portal messages

**Files to Modify:**
- `backend/servers/api-server/src/routes/webhooks/portals.rs` (create)
- `backend/crates/db/src/repositories/listing.rs` (add analytics update)

---

#### Epic 106: Scheduled Notifications System
**Goal:** Implement comprehensive scheduled notification triggers.

**Target Apps:** api-server
**Estimate:** 4 stories, ~1 week
**Dependencies:** Epic 103 (Redis)
**Priority:** P2 - MEDIUM

**PRD Reference:** FR8 - Communication & Notifications

---

##### Story 106.1: Announcement Notification Triggers

As a **resident**,
I want to **receive notifications for announcements**,
So that **I'm informed of important building news**.

**Acceptance Criteria:**

**Given** an announcement is published (immediate or scheduled)
**When** the publication occurs
**Then**:
  - Target users are determined by announcement scope
  - Push notifications are sent to mobile users
  - Email is sent based on preferences
  - In-app notification is created
  - Delivery status is tracked
**And** residents receive timely announcements

**Technical Notes:**
- Reference: `scheduler.rs:91-105` - TODO for announcement notifications
- Determine targets from target_type and target_ids
- Respect user notification preferences
- Use notification service abstraction

**Files to Modify:**
- `backend/servers/api-server/src/services/scheduler.rs` (implement notification trigger)
- `backend/servers/api-server/src/services/notification.rs` (create notification service)

---

##### Story 106.2: Vote Expiry Handler

As a **property manager**,
I want to **votes automatically closed when expired**,
So that **voting results are finalized on schedule**.

**Acceptance Criteria:**

**Given** a vote has an end_date
**When** the end_date passes
**Then**:
  - Vote status changes to closed
  - Results are calculated and stored
  - Participants are notified of results
  - Result summary is generated
  - Audit trail is complete
**And** no manual intervention is needed

**Technical Notes:**
- Reference: `scheduler.rs:75` - TODO for vote expiry
- Query votes where end_date < now() AND status = 'active'
- Calculate quorum and results
- Trigger result notification to all eligible voters

**Files to Modify:**
- `backend/servers/api-server/src/services/scheduler.rs` (add vote expiry task)
- `backend/crates/db/src/repositories/voting.rs` (add close and result methods)

---

##### Story 106.3: Reminder Notifications

As a **resident**,
I want to **receive reminders for pending actions**,
So that **I don't miss important deadlines**.

**Acceptance Criteria:**

**Given** reminders are configured
**When** reminder time is reached
**Then**:
  - Meter reading reminders are sent before deadline
  - Payment reminders are sent for overdue amounts
  - Vote reminders are sent before closure
  - Reminder frequency respects preferences
  - Reminders are not duplicated
**And** residents complete actions on time

**Technical Notes:**
- Reference: `scheduler.rs:76` - TODO for reminders
- Configure reminder timing per notification type
- Track sent reminders to prevent duplicates
- Support snooze functionality

**Files to Modify:**
- `backend/servers/api-server/src/services/scheduler.rs` (add reminder tasks)
- `backend/servers/api-server/src/services/notification.rs` (reminder templates)

---

##### Story 106.4: Session Cleanup Task

As a **system administrator**,
I want to **expired sessions cleaned automatically**,
So that **database and Redis remain performant**.

**Acceptance Criteria:**

**Given** sessions have expiry times
**When** the cleanup task runs
**Then**:
  - Expired database sessions are deleted
  - Expired Redis sessions are cleaned
  - Cleanup runs efficiently (batch deletes)
  - Cleanup frequency is configurable
  - Metrics track cleaned session count
**And** session storage remains bounded

**Technical Notes:**
- Reference: `scheduler.rs:77` - TODO for session cleanup
- Delete sessions older than refresh_token expiry + buffer
- Run during low-traffic periods
- Log cleanup statistics

**Files to Modify:**
- `backend/servers/api-server/src/services/scheduler.rs` (add cleanup task)
- `backend/crates/db/src/repositories/session.rs` (add cleanup method)

---

## Summary

| Phase | Epics | Stories | Priority |
|-------|-------|---------|----------|
| 31: Infrastructure Completion | 102 | 4 | P1 |
| 32: Production Hardening | 103-104 | 7 | P1/P2 |
| 33: Portal Syndication & Notifications | 105-106 | 8 | P1/P2 |

**Total:** 5 Epics, 19 Stories

### Implementation Order

1. **Epic 102** - Distributed Tracing & Observability (P1) ~1 week
   - Stories 102.1-102.4: Complete infrastructure monitoring

2. **Epic 103** - Storage & Caching Integration (P1) ~1.5 weeks
   - Stories 103.1-103.5: S3, Redis, pgvector

3. **Epic 104** - Cross-Server Health & SSO (P2) ~0.5 weeks
   - Stories 104.1-104.2: Server-to-server health

4. **Epic 105** - Listing Portal Syndication (P1) ~1 week
   - Stories 105.1-105.4: Async syndication

5. **Epic 106** - Scheduled Notifications (P2) ~1 week
   - Stories 106.1-106.4: Notification triggers

### Parallel Implementation

- Epic 102 and Epic 103 can be worked in parallel (different domains)
- Epic 104 depends on Epic 103 (Redis client)
- Epic 105 depends on Epic 103 (background jobs)
- Epic 106 depends on Epic 103 (notifications) and Epic 105 (scheduler patterns)

### Dependencies

```
Epic 102 (Observability) → independent
Epic 103 (Storage/Caching) → independent
Epic 104 (Health/SSO) → requires Epic 103 (Redis)
Epic 105 (Syndication) → requires Epic 103 (Jobs), Epic 98 (Integrations)
Epic 106 (Notifications) → requires Epic 103 (Redis), benefits from Epic 105
```

### Feature Flags

| Flag | Stories | Default |
|------|---------|---------|
| `infrastructure.tracing_enabled` | 102.1-102.2 | false |
| `storage.s3_enabled` | 103.1 | false |
| `cache.redis_enabled` | 103.2-103.4 | false |
| `syndication.async_enabled` | 105.1-105.4 | false |
