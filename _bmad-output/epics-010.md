---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - _bmad-output/prd.md
  - _bmad-output/architecture.md
  - _bmad-output/epics-009.md
  - _bmad-output/implementation-artifacts/gap-analysis-remediation.md
workflowType: 'epics-and-stories'
lastStep: 4
status: 'ready'
project_name: 'Property Management System (PPT) & Reality Portal'
user_name: 'Martin Janci'
date: '2026-01-01'
continues_from: 'epics-009.md'
phase_range: '27-28'
epic_range: '91-95'
---

# Property Management System (PPT) & Reality Portal - Epic Breakdown (Part 10)

## Overview

This document continues from `epics-009.md` and addresses **Phase 27: AI/LLM Integration Completion** and **Phase 28: Production Hardening** - implementing the stubbed AI features, voice assistant OAuth, and production-grade enhancements.

**Continuation from:** `epics-009.md` (Epics 88-90, Phase 26)

**Source:** Comprehensive backend analysis (2026-01-01)

**Key Findings from Gap Analysis:**
- AI Chat Assistant has placeholder responses (needs LLM integration)
- Lease generation uses placeholder template (needs LLM)
- Listing description generation is template-based (needs LLM)
- Voice assistant OAuth token exchange pending
- Workflow execution is async-stubbed

---

## Epic List

### Phase 27: AI/LLM Integration Completion

#### Epic 91: AI Chat Assistant LLM Integration
**Goal:** Wire the AI chat assistant to an actual LLM provider (OpenAI/Anthropic) with conversation context and RAG capabilities.

**Target Apps:** api-server
**Estimate:** 5 stories, ~1 week
**Dependencies:** None
**Priority:** P1 - HIGH

**PRD Reference:** FR64 - Users can interact with AI chatbot for common questions (UC-20.4)

---

##### Story 91.1: LLM Provider Configuration

As a **platform admin**,
I want to **configure LLM provider credentials and settings**,
So that **the AI assistant can make real API calls**.

**Acceptance Criteria:**

**Given** the platform needs LLM capabilities
**When** an admin configures LLM settings
**Then**:
  - Provider selection (OpenAI, Anthropic, or local) is supported
  - API keys are stored securely (encrypted at rest)
  - Model selection is configurable (e.g., gpt-4o, claude-3-sonnet)
  - Rate limits and cost controls can be set
  - Configuration is per-organization (tenant-specific)
**And** settings are validated before saving

**Technical Notes:**
- Add LLM configuration to organization settings
- Use existing encryption infrastructure for API keys
- Create `LlmConfig` model in database
- Environment variable fallback for platform-wide defaults

**Files to Create/Modify:**
- `backend/crates/db/src/models/llm_config.rs` (new)
- `backend/crates/db/src/repositories/llm_config.rs` (new)
- `backend/servers/api-server/src/routes/infrastructure.rs` (add LLM config endpoints)

---

##### Story 91.2: LLM Client Implementation

As a **developer**,
I want to **have a unified LLM client interface**,
So that **different providers can be used interchangeably**.

**Acceptance Criteria:**

**Given** an LLM request needs to be made
**When** the chat service processes a message
**Then**:
  - OpenAI client sends properly formatted requests
  - Anthropic client sends properly formatted requests
  - Response streaming is supported
  - Token usage is tracked
  - Errors are handled gracefully with retry logic
**And** provider switching is transparent to the chat service

**Technical Notes:**
- Create `LlmClient` trait in `backend/crates/integrations/src/llm/`
- Implement `OpenAiClient` and `AnthropicClient`
- Use async streaming for real-time responses
- Track tokens for cost monitoring

**Files to Create:**
- `backend/crates/integrations/src/llm/mod.rs`
- `backend/crates/integrations/src/llm/openai.rs`
- `backend/crates/integrations/src/llm/anthropic.rs`
- `backend/crates/integrations/src/llm/client.rs` (trait definition)

---

##### Story 91.3: Chat Context & Conversation Memory

As a **user**,
I want to **have contextual conversations with the AI**,
So that **it remembers previous messages in the session**.

**Acceptance Criteria:**

**Given** an active chat session exists
**When** I send a new message
**Then**:
  - Previous messages in session are included as context
  - System prompt includes organization/building context
  - Conversation is truncated intelligently to fit token limits
  - Session context persists across page refreshes
**And** context is tenant-isolated

**Technical Notes:**
- Implement context window management (e.g., last 10 messages + summary)
- Add organization context (name, settings, role) to system prompt
- Use existing `AiChatSession` and `AiChatMessage` models
- Reference: `ai.rs:271-278` (current placeholder)

**Files to Modify:**
- `backend/servers/api-server/src/routes/ai.rs` (replace placeholder in `send_message`)
- `backend/servers/api-server/src/services/ai_chat.rs` (new service)

---

##### Story 91.4: RAG Document Retrieval

As a **user**,
I want to **ask questions about building documents**,
So that **the AI can answer based on my organization's documents**.

**Acceptance Criteria:**

**Given** documents exist in the organization
**When** I ask a question related to document content
**Then**:
  - Relevant document chunks are retrieved
  - Document content is included in LLM context
  - Source documents are cited in responses
  - Access permissions are respected
**And** retrieval latency is < 500ms

**Technical Notes:**
- Use existing document repository for content retrieval
- Implement semantic search using pgvector (if available) or keyword matching
- Reference: Epic 84 implemented pgvector foundations (Story 84.5)
- Limit retrieved chunks to fit token budget

**Files to Modify:**
- `backend/servers/api-server/src/services/ai_chat.rs`
- `backend/crates/db/src/repositories/documents.rs` (add semantic search)

---

##### Story 91.5: Chat Response Streaming

As a **user**,
I want to **see AI responses appear incrementally**,
So that **I don't wait for the complete response**.

**Acceptance Criteria:**

**Given** I send a message to the AI
**When** the AI generates a response
**Then**:
  - Response tokens stream to the frontend in real-time
  - Partial responses are visible as they generate
  - WebSocket or SSE is used for streaming
  - Complete response is saved to database when finished
**And** streaming works reliably on mobile

**Technical Notes:**
- Implement SSE endpoint for chat streaming
- Use `text/event-stream` content type
- Handle connection drops gracefully
- Save complete message after stream ends

**Files to Modify:**
- `backend/servers/api-server/src/routes/ai.rs` (add streaming endpoint)
- `frontend/apps/ppt-web/src/features/ai/hooks/useAiChat.ts` (streaming support)

---

#### Epic 92: Intelligent Document Generation
**Goal:** Implement LLM-powered document generation for leases, listings, and other templates.

**Target Apps:** api-server
**Estimate:** 4 stories, ~1 week
**Dependencies:** Epic 91 (LLM client)
**Priority:** P2 - MEDIUM

**PRD Reference:** FR67 - System can summarize long documents automatically (UC-20.7)

---

##### Story 92.1: Lease Document Generation

As a **property manager**,
I want to **generate lease documents using AI**,
So that **I save time on document preparation**.

**Acceptance Criteria:**

**Given** a lease generation request with property and tenant details
**When** the generation is triggered
**Then**:
  - AI generates lease content based on Slovak/Czech law templates
  - Property details are populated from unit data
  - Tenant information is included accurately
  - Generation completes within 30 seconds
  - Generated content is editable before finalization
**And** result is stored in the document repository

**Technical Notes:**
- Reference: `ai.rs` lease generation routes (currently stubbed)
- Use structured prompts with legal template patterns
- Include country-specific legal requirements
- Allow template customization per organization

**Files to Modify:**
- `backend/servers/api-server/src/routes/ai.rs` (wire up LLM in lease generation)
- `backend/servers/api-server/src/services/document_generation.rs` (new)

---

##### Story 92.2: Listing Description Generation

As a **realtor**,
I want to **generate property listing descriptions using AI**,
So that **listings are professionally written**.

**Acceptance Criteria:**

**Given** property details and photos
**When** description generation is requested
**Then**:
  - AI generates compelling listing description
  - Property features are highlighted
  - Location benefits are described
  - Multiple language versions can be generated (SK, CZ, DE, EN)
  - Description fits portal character limits
**And** tone/style can be customized

**Technical Notes:**
- Reference: `ai.rs` listing description routes (currently template-based)
- Use property metadata for context
- Generate SEO-friendly descriptions
- Support regeneration with feedback

**Files to Modify:**
- `backend/servers/api-server/src/routes/ai.rs` (enhance listing description generation)
- `backend/servers/reality-server/src/routes/listings.rs` (integrate generation)

---

##### Story 92.3: Document Summarization

As a **user**,
I want to **get AI summaries of long documents**,
So that **I can quickly understand document content**.

**Acceptance Criteria:**

**Given** a document in the repository
**When** I request a summary
**Then**:
  - AI generates concise summary (configurable length)
  - Key points are extracted
  - Summary is stored for future reference
  - Original document is preserved
**And** summary generation works for PDF, DOCX, TXT

**Technical Notes:**
- Extract text from documents using existing OCR/parsing
- Chunk documents for token limits
- Use map-reduce summarization for long documents
- Store summaries in document metadata

**Files to Modify:**
- `backend/servers/api-server/src/routes/documents.rs` (add summarize endpoint)
- `backend/servers/api-server/src/services/document_generation.rs`

---

##### Story 92.4: Announcement Draft Generation

As a **property manager**,
I want to **generate announcement drafts using AI**,
So that **I can quickly create professional communications**.

**Acceptance Criteria:**

**Given** an announcement topic and key points
**When** I request a draft
**Then**:
  - AI generates well-structured announcement
  - Tone is professional and appropriate
  - Multiple drafts can be generated for selection
  - Draft is editable before sending
**And** templates for common scenarios are available

**Technical Notes:**
- Integrate with existing announcement creation flow
- Use building/organization context for personalization
- Support urgency levels in tone adjustment

**Files to Modify:**
- `backend/servers/api-server/src/routes/announcements.rs` (add AI draft endpoint)

---

#### Epic 93: Voice Assistant & OAuth Completion
**Goal:** Complete the voice assistant integration with OAuth token exchange.

**Target Apps:** api-server
**Estimate:** 3 stories, ~3 days
**Dependencies:** Epic 91 (LLM client)
**Priority:** P2 - MEDIUM

**PRD Reference:** Related to FR64 - AI chatbot capabilities

---

##### Story 93.1: Voice Assistant OAuth Token Exchange

As a **user**,
I want to **link my voice assistant device with OAuth**,
So that **my commands are securely authenticated**.

**Acceptance Criteria:**

**Given** a voice assistant link request
**When** OAuth flow completes
**Then**:
  - Access token is stored encrypted
  - Refresh token is stored for renewal
  - Token expiry is tracked
  - Token refresh happens automatically
**And** linked devices can be managed

**Technical Notes:**
- Reference: `ai.rs:1653` - Phase 2 TODO for OAuth tokens
- Implement OAuth 2.0 client for voice platforms (Alexa, Google)
- Store tokens securely with encryption
- Implement token refresh background job

**Files to Modify:**
- `backend/servers/api-server/src/routes/ai.rs` (complete voice assistant linking)
- `backend/crates/db/src/models/voice_assistant.rs` (add token fields)

---

##### Story 93.2: Voice Command Processing

As a **user**,
I want to **issue voice commands to control building features**,
So that **I can interact hands-free**.

**Acceptance Criteria:**

**Given** a linked voice assistant
**When** I issue a voice command
**Then**:
  - Command is parsed and understood
  - Appropriate action is triggered (e.g., report fault, check balance)
  - Confirmation is spoken back
  - Command history is logged
**And** common commands are supported

**Technical Notes:**
- Map voice intents to API actions
- Use existing routes for action execution
- Return voice-friendly responses

**Files to Modify:**
- `backend/servers/api-server/src/routes/ai.rs` (command processing logic)
- `backend/servers/api-server/src/services/voice_commands.rs` (new)

---

##### Story 93.3: Voice Platform Webhooks

As a **developer**,
I want to **receive webhooks from voice platforms**,
So that **commands are processed in real-time**.

**Acceptance Criteria:**

**Given** a voice platform sends a webhook
**When** the webhook is received
**Then**:
  - Request signature is verified
  - User is authenticated via OAuth token
  - Command is processed
  - Response is returned in platform format
**And** webhook endpoints are secure

**Technical Notes:**
- Implement Alexa Skills Kit webhook format
- Implement Google Actions webhook format
- Verify request signatures per platform spec

**Files to Create:**
- `backend/servers/api-server/src/routes/webhooks/voice.rs`

---

#### Epic 94: Workflow Execution Engine
**Goal:** Implement the actual async workflow execution that is currently stubbed.

**Target Apps:** api-server
**Estimate:** 4 stories, ~1 week
**Dependencies:** None
**Priority:** P2 - MEDIUM

**PRD Reference:** FR69-70 - Workflow automation configuration and event-triggered actions

---

##### Story 94.1: Workflow Action Executors

As a **developer**,
I want to **execute workflow actions asynchronously**,
So that **automated workflows actually run**.

**Acceptance Criteria:**

**Given** a workflow is triggered
**When** execution starts
**Then**:
  - Each action is executed in order
  - Action types are handled: email, notification, API call, delay
  - Execution status is updated in real-time
  - Errors are captured and logged
**And** failed actions can be retried

**Technical Notes:**
- Reference: `ai.rs:999` - TODO for async execution
- Use Tokio tasks for async execution
- Implement action executor trait with type-specific implementations
- Store execution logs for debugging

**Files to Create:**
- `backend/servers/api-server/src/services/workflow_executor.rs`
- `backend/servers/api-server/src/services/actions/mod.rs`

---

##### Story 94.2: Workflow Trigger Events

As a **property manager**,
I want to **workflows to trigger on events**,
So that **automation happens automatically**.

**Acceptance Criteria:**

**Given** a workflow is configured with a trigger
**When** the trigger event occurs
**Then**:
  - Matching workflows are identified
  - Execution is queued
  - Concurrent executions are handled
  - Trigger conditions are evaluated
**And** event types include: fault_created, vote_started, payment_overdue, meter_reading_due

**Technical Notes:**
- Hook into existing event bus (Redis pub/sub)
- Create event listener service
- Match events to workflow triggers
- Support conditional triggers

**Files to Modify:**
- `backend/servers/api-server/src/services/workflow_executor.rs`
- `backend/servers/api-server/src/routes/ai.rs` (connect triggers)

---

##### Story 94.3: Conditional Logic in Workflows

As a **property manager**,
I want to **add conditions to workflow steps**,
So that **actions only run when criteria are met**.

**Acceptance Criteria:**

**Given** a workflow with conditional steps
**When** the workflow executes
**Then**:
  - Conditions are evaluated (e.g., amount > 100, status == 'overdue')
  - Branches are followed based on results
  - Variables can be used in conditions
  - Complex expressions are supported (AND, OR, NOT)
**And** condition evaluation is logged

**Technical Notes:**
- Implement simple expression evaluator
- Support common comparison operators
- Allow accessing workflow context variables

**Files to Modify:**
- `backend/servers/api-server/src/services/workflow_executor.rs`

---

##### Story 94.4: Workflow Templates & Marketplace

As a **property manager**,
I want to **use pre-built workflow templates**,
So that **I can quickly set up common automations**.

**Acceptance Criteria:**

**Given** workflow templates exist
**When** I browse templates
**Then**:
  - Templates are categorized (onboarding, reminders, alerts)
  - Templates can be previewed
  - Templates can be imported and customized
  - My workflows can be saved as templates
**And** templates include: payment reminder, fault escalation, lease expiry alert

**Technical Notes:**
- Create template definitions in JSON/YAML
- Store templates in database
- Allow organization-specific templates

**Files to Modify:**
- `backend/servers/api-server/src/routes/ai.rs` (add template endpoints)
- `backend/crates/db/src/models/workflow_templates.rs` (new)

---

### Phase 28: Production Hardening

#### Epic 95: Production Readiness & Observability
**Goal:** Enhance production observability, error handling, and operational tooling.

**Target Apps:** api-server, reality-server
**Estimate:** 4 stories, ~1 week
**Dependencies:** None
**Priority:** P1 - HIGH

**PRD Reference:** NFR-PERF, NFR-REL - Performance and reliability requirements

---

##### Story 95.1: Distributed Tracing Implementation

As a **DevOps engineer**,
I want to **trace requests across services**,
So that **I can debug performance issues**.

**Acceptance Criteria:**

**Given** a request spans multiple services
**When** I view the trace
**Then**:
  - Complete request path is visible
  - Span timing shows bottlenecks
  - Database queries are included
  - External API calls are traced
**And** traces are queryable by trace ID

**Technical Notes:**
- Integrate OpenTelemetry
- Export to Jaeger or similar
- Add trace IDs to logs
- Instrument key operations

**Files to Modify:**
- `backend/servers/api-server/src/main.rs` (add tracing middleware)
- `backend/servers/reality-server/src/main.rs`
- Add `opentelemetry` dependencies to Cargo.toml

---

##### Story 95.2: Enhanced Error Reporting

As a **developer**,
I want to **capture detailed error context**,
So that **bugs are easier to diagnose**.

**Acceptance Criteria:**

**Given** an error occurs
**When** it is logged
**Then**:
  - Full stack trace is captured
  - Request context is included (user, org, endpoint)
  - Error is categorized by type
  - Sensitive data is redacted
**And** errors are sent to error tracking service (Sentry)

**Technical Notes:**
- Integrate Sentry for error tracking
- Add error context middleware
- Implement PII redaction
- Create error categories

**Files to Modify:**
- `backend/servers/api-server/src/middleware/error.rs`
- Add `sentry` dependency

---

##### Story 95.3: Health Check Enhancements

As a **DevOps engineer**,
I want to **comprehensive health checks**,
So that **I can monitor system dependencies**.

**Acceptance Criteria:**

**Given** the health endpoint is called
**When** checks run
**Then**:
  - Database connectivity is verified
  - Redis connectivity is verified
  - External service health is checked (if critical)
  - Degraded status is reported when appropriate
  - Response time is measured
**And** health history is persisted

**Technical Notes:**
- Reference: Epic 89 implemented feature flag storage
- Add dependency health checks
- Implement circuit breaker patterns for external services
- Store health check results

**Files to Modify:**
- `backend/servers/api-server/src/routes/infrastructure.rs`
- `backend/crates/db/src/repositories/health_checks.rs`

---

##### Story 95.4: Performance Monitoring Dashboard

As a **DevOps engineer**,
I want to **view real-time performance metrics**,
So that **I can monitor system health**.

**Acceptance Criteria:**

**Given** the system is running
**When** I view the dashboard
**Then**:
  - P95/P99 latency is displayed
  - Error rate is shown
  - Request rate is graphed
  - Database query performance is visible
  - Top slow endpoints are listed
**And** alerts are configurable

**Technical Notes:**
- Expose Prometheus metrics endpoint
- Create Grafana dashboard definitions
- Implement custom metrics for business operations
- Add alerting rules

**Files to Create:**
- `backend/servers/api-server/src/routes/metrics.rs`
- `infrastructure/grafana/dashboards/api-server.json`

---

## Summary

| Phase | Epics | Stories | Priority |
|-------|-------|---------|----------|
| 27: AI/LLM Integration | 91-94 | 16 | P1-P2 |
| 28: Production Hardening | 95 | 4 | P1 |

**Total:** 5 Epics, 20 Stories

### Implementation Order

1. **Epic 91** - AI Chat Assistant LLM Integration (P1) ~1 week
   - Stories 91.1-91.5: Core LLM infrastructure and chat functionality

2. **Epic 95** - Production Readiness (P1) ~1 week
   - Stories 95.1-95.4: Observability and monitoring

3. **Epic 92** - Intelligent Document Generation (P2) ~1 week
   - Stories 92.1-92.4: LLM-powered document creation

4. **Epic 93** - Voice Assistant OAuth (P2) ~3 days
   - Stories 93.1-93.3: Complete voice integration

5. **Epic 94** - Workflow Execution Engine (P2) ~1 week
   - Stories 94.1-94.4: Async workflow processing

### Parallel Implementation

- Epic 91 and Epic 95 can be worked in parallel (different domains)
- Epic 92 depends on Epic 91 (needs LLM client)
- Epic 93 depends on Epic 91 (needs LLM for voice responses)
- Epic 94 is independent

### Feature Flags

All LLM features should be behind feature flags:
- `llm.chat_enabled` - AI chat assistant
- `llm.document_generation` - Document generation
- `llm.voice_assistant` - Voice commands
- `llm.workflow_automation` - Automated workflows
