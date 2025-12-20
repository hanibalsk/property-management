---
stepsCompleted: [1, 2, 3, 4, 5, 6, 7, 8]
inputDocuments:
  - _bmad-output/prd.md
  - _bmad-output/ux-design-specification.md
workflowType: 'architecture'
lastStep: 8
status: 'complete'
completedAt: '2025-12-20'
project_name: 'Property Management System (PPT) & Reality Portal'
user_name: 'Martin Janci'
date: '2025-12-20'
---

# Architecture Decision Document

_This document builds collaboratively through step-by-step discovery. Sections are appended as we work through each architectural decision together._

## Project Context Analysis

### Requirements Overview

**Functional Requirements:**
The system encompasses 101 functional requirements across 15 capability areas, derived from 508 use cases spanning 51 categories. Key capability areas include:

- **Identity & Access Management** (7 FRs): OAuth 2.0 provider, 2FA, SSO between systems
- **Organization & Multi-Tenancy** (7 FRs): Row-level security, tenant isolation, data export
- **Property & Resident Management** (7 FRs): Buildings, units, residents, delegation rights
- **Communication & Notifications** (8 FRs): Push, email, in-app messaging, preference management
- **Issue & Fault Management** (7 FRs): Full lifecycle with AI categorization
- **Voting & Decision Making** (8 FRs): Immutable audit trail, quorum calculation
- **Document Management** (7 FRs): Versioning, OCR, access control
- **Financial Management** (6 FRs, Phase 2): Payment tracking, accounting exports
- **AI & Automation** (7 FRs, Phase 3): Chatbot, predictions, workflow automation
- **Real Estate & Listings** (8 FRs, Phase 4): Multi-portal syndication, search

**Non-Functional Requirements:**
Critical quality attributes that drive architectural decisions:

| Category | Key Requirements |
|----------|------------------|
| **Performance** | P95 < 200ms, 1000 RPS, LCP < 2.5s (Reality Portal) |
| **Availability** | 99.9% uptime, MTTR < 30 min, RTO < 4h, RPO < 1h |
| **Security** | Argon2id passwords, JWT sessions, AES-256 at rest, TLS 1.3 |
| **Compliance** | GDPR (24h export, 72h delete), WCAG 2.1 AA, SOC 2 (Year 2) |
| **Scalability** | 500 → 15,000 concurrent users, 100 → 5,000 RPS |
| **Localization** | sk, cs, de, en with locale-aware formatting |

**Scale & Complexity:**

| Attribute | Value |
|-----------|-------|
| Primary domain | Multi-platform PropTech SaaS |
| Complexity level | Enterprise |
| Estimated architectural components | 15+ |
| Use cases | 508 across 51 categories |
| Actor types | 20+ (cross-system roles) |

### Technical Constraints & Dependencies

**Backend Constraints:**
- Rust (Axum) for both servers - performance and safety requirements
- PostgreSQL as primary database with RLS for tenant isolation
- Redis for caching, sessions, and pub/sub
- S3-compatible storage for documents and images

**Frontend Constraints:**
- React SPA (Vite) for ppt-web - rich interactivity, offline capable
- Next.js SSR/SSG for reality-web - SEO requirements mandate server rendering
- React Native for PM mobile - code sharing with web
- Kotlin Multiplatform for Reality mobile - native performance, platform features

**Integration Constraints:**
- OAuth 2.0/OIDC: api-server as provider, reality-server as consumer
- External portals: Nehnuteľnosti.sk, Reality.sk with 99% SLA target
- Accounting: POHODA, Money S3 (async file/REST export)
- Short-term rental: Airbnb/Booking.com (iCal sync)

**UX-Driven Constraints:**
- Token-first design system (Style Dictionary) for cross-platform tokens
- Offline-first mobile with explicit sync feedback
- WebSocket for real-time updates in authenticated apps
- Push notification deep linking as primary navigation pattern
- Desktop-first for ppt-web, mobile-first for reality-web

### Cross-Cutting Concerns Identified

1. **Multi-Tenancy & Data Isolation**
   - Every database query scoped by organization_id
   - PostgreSQL RLS policies at database layer
   - Tenant context extracted from JWT in middleware
   - Architectural guarantee (not policy) against cross-tenant access

2. **Authentication & Authorization**
   - api-server as OAuth 2.0 provider
   - reality-server as SSO consumer
   - RBAC with 12+ role types across platform/org/building/unit levels
   - Permission checks in API middleware

3. **Real-Time Communication**
   - WebSocket connections for authenticated apps (PM web/mobile)
   - Push notifications via FCM/APNs with deduplication
   - Email as fallback/digest channel
   - User-controlled preferences per notification category

4. **AI/ML Integration**
   - First-class columns in data model (not external service calls)
   - OCR for meter readings with confidence scoring
   - Fault categorization and priority suggestions
   - Sentiment analysis for trend detection
   - Human-in-the-loop for low-confidence predictions

5. **Offline Capability**
   - PM mobile: Full offline with sync queue
   - Reality mobile: Read-only cached content
   - Web apps: Graceful degradation, no true offline
   - Explicit sync status feedback required

6. **SEO & SSR**
   - Reality Portal requires SSR/SSG for search engine visibility
   - LCP < 2.5s, FCP < 1.5s targets
   - Dynamic OG tags for listing sharing
   - Sitemap generation for listings

7. **GDPR & Compliance**
   - Data export within 24 hours (UC-23.4)
   - Data deletion within 72 hours (UC-23.5)
   - Audit logging for all sensitive operations
   - Consent management per processing activity
   - 10-year financial records retention

8. **Accessibility**
   - WCAG 2.1 AA compliance mandatory
   - Elderly user accommodation (48px+ touch targets)
   - Keyboard navigation for all features
   - Screen reader compatibility
   - High contrast theme option

## Starter Template Evaluation

### Primary Technology Domains

This multi-platform system spans 4 technology domains:

| Domain | App | Technology | Starter Approach |
|--------|-----|------------|------------------|
| Backend APIs | api-server, reality-server | Rust + Axum 0.8.6 | Cargo workspace |
| Web SPA | ppt-web | React 19 + Vite 6 | create-vite |
| Web SSR | reality-web | Next.js 15.5 | create-next-app |
| Mobile RN | mobile | React Native 0.83 | @react-native-community/cli |
| Mobile Native | mobile-native | KMP + Kotlin 2.3 | Android Studio wizard |

### Initialization Commands

**Backend (Cargo Workspace):**
```bash
mkdir -p backend && cd backend
cargo init --name api-server
# Configure workspace in Cargo.toml
```

**ppt-web (React SPA):**
```bash
npm create vite@latest ppt-web -- --template react-swc-ts
```

**reality-web (Next.js SSR):**
```bash
npx create-next-app@latest reality-web --typescript --tailwind --eslint --app --src-dir --turbopack
```

**mobile (React Native):**
```bash
npx @react-native-community/cli@latest init mobile --template react-native-template-typescript
```

**mobile-native (KMP):**
```
Use Android Studio "New Project" → "Kotlin Multiplatform App"
Configure shared module, Android (Compose), iOS (SwiftUI)
```

### Architectural Decisions from Starters

**Language & Runtime:**
- TypeScript 5.7 strict mode for all frontend apps
- Rust 2024 edition for backend
- Kotlin 2.3 with KSP2 for mobile-native

**Styling Solution:**
- Tailwind CSS for web apps (aligned with UX spec: Radix UI + Tailwind)
- React Native Paper for PM mobile
- Compose Material3 / SwiftUI for Reality mobile

**Build Tooling:**
- Vite 6 + SWC for ppt-web (fastest React builds)
- Turbopack for reality-web (Next.js native)
- Metro for React Native
- Gradle KTS for Kotlin Multiplatform

**Testing Framework:**
- Vitest for web apps (Vite-native, Jest-compatible API)
- Jest for React Native
- Kotest for Kotlin
- Rust's built-in test framework + Tokio test

**Code Organization:**
- Monorepo with pnpm workspaces (frontend/)
- Cargo workspace (backend/)
- Shared design tokens via Style Dictionary
- OpenAPI-generated SDK (TypeSpec → hey-api/openapi-ts)

### Version Summary (December 2025)

| Technology | Version | Source |
|------------|---------|--------|
| Axum | 0.8.6 | [GitHub](https://github.com/tokio-rs/axum) |
| Next.js | 15.5.9 | [nextjs.org](https://nextjs.org/blog/next-15-5) |
| React Native | 0.83.1 | [reactnative.dev](https://reactnative.dev/versions) |
| Kotlin | 2.3.0 | [kotlinlang.org](https://kotlinlang.org/) |
| Vite | 6.x | [vite.dev](https://vite.dev/) |
| React | 19.x | Standard with Next.js 15 / RN 0.83 |
| TypeScript | 5.7.x | Standard with starters |

**Note:** Project initialization using these commands should be the first implementation story per platform

## Core Architectural Decisions

### Decision Priority Analysis

**Critical Decisions (Already Made - From docs/architecture.md):**
- ADR-001: Modular Monolith architecture
- ADR-002: Two backend servers (api-server:8080, reality-server:8081)
- ADR-003: Rust + Axum framework
- ADR-004: PostgreSQL with RLS for multi-tenancy
- ADR-005: Event-driven with RabbitMQ/SQS
- ADR-006: Redis for caching and sessions
- ADR-007: S3-compatible object storage
- ADR-008: WebSocket for real-time features

**Important Decisions (From UX Specification):**
- Token-first design system (Style Dictionary)
- Radix UI + Tailwind CSS for web applications
- React Native Paper for PM mobile
- Compose/SwiftUI for Reality Portal mobile (KMP)
- WCAG 2.1 AA accessibility compliance

**Decisions Made in This Document:**
- Hybrid API specification (TypeSpec + utoipa)
- TanStack Query + Zustand state management
- Pessimistic offline sync with conflict detection
- Hybrid AI/ML (local + external APIs)
- 80% unit test coverage target

### Data Architecture

**Already Established (ADR-004):**
- PostgreSQL 16+ as primary database
- Row-level security (RLS) for tenant isolation
- JSONB for flexible data storage
- PostGIS for geospatial queries
- pgvector for AI embeddings (Phase 3)

**ORM Strategy:**
- SQLx for compile-time query checking
- SeaORM for complex query building
- Raw SQL for performance-critical paths

**Migration Strategy:**
- sqlx-cli for schema migrations
- Version-controlled migration files
- Rollback scripts for all migrations
- CI validation of pending migrations

### Authentication & Security

**Already Established (ADR Architecture):**
- api-server as OAuth 2.0 / OIDC provider
- reality-server as SSO consumer
- JWT with 15-minute access / 7-day refresh tokens
- Argon2id for password hashing
- TOTP-based 2FA (Google Authenticator compatible)

**Authorization Model:**
- RBAC with 12+ role types
- Permission checks in Axum middleware
- Tenant context extracted from JWT
- Audit logging for sensitive operations

**API Security:**
- Rate limiting: 100 req/min per user, 1000 req/min per org
- TLS 1.3 for all connections
- CORS with explicit origin allowlist
- Request signing for webhooks

### API & Communication Patterns

**API Specification:**
- Hybrid approach: TypeSpec (design-first) + utoipa (code-first)
- OpenAPI 3.1 as single source of truth
- Spectral for API linting
- oasdiff for breaking change detection

**SDK Generation:**
- TypeScript: @hey-api/openapi-ts
- Kotlin: openapi-generator (multiplatform)
- Automated generation in CI

**Real-Time Communication:**
- WebSocket (Axum + tokio-tungstenite) for authenticated apps
- Redis pub/sub for cross-instance messaging
- Push notifications: FCM (Android), APNs (iOS)
- Email via SendGrid with template system

### Frontend Architecture

**State Management:**
- Server State: TanStack Query (React Query)
- Client State: Zustand (lightweight)
- Form State: React Hook Form + Zod validation
- Theme/Auth: React Context

**Component Architecture:**
- Radix UI primitives (headless, accessible)
- Tailwind CSS utility classes
- Style Dictionary design tokens
- Storybook for component documentation

**Performance Optimization:**
- ppt-web: Code splitting, lazy routes
- reality-web: SSR/SSG, ISR for listings
- Image optimization: Next.js Image, responsive srcset
- Bundle analysis in CI

### Offline & Sync Architecture

**PM Mobile (React Native):**
- SQLite (WatermelonDB) for local storage
- Sync queue for pending operations
- Conflict detection: version vectors
- Conflict resolution: last-write-wins + user prompt for critical ops
- Background sync on reconnect

**Reality Mobile (KMP):**
- Read-only offline cache
- Room (Android) / Core Data (iOS) for favorites
- Pull-to-refresh sync model

### AI/ML Architecture

**Approach:** Hybrid (local + external)

| Feature | Approach | Storage |
|---------|----------|---------|
| OCR Meter Reading | External API → Tesseract/Cloud Vision | `meter_reading.ocr_extracted_value`, `ocr_confidence` |
| Fault Categorization | Rule engine + lightweight classifier | `fault.ai_category`, `ai_priority_suggestion` |
| Sentiment Analysis | Local Rust crate (vader-sentiment) | `message.sentiment_score` |
| Chatbot | External LLM + RAG on local docs | Ephemeral |
| Predictive Maintenance | ML model (Phase 3) | `equipment.predicted_failure_date` |

**AI Confidence Thresholds:**
- High confidence (>90%): Auto-apply
- Medium (70-90%): Apply with "AI suggested" indicator
- Low (<70%): Human review required

### Infrastructure & Deployment

**Hosting Strategy (Initial):**
- Single region (EU-central)
- Kubernetes or Railway/Fly.io
- Managed PostgreSQL (Supabase/Neon/RDS)
- Managed Redis (Upstash/ElastiCache)

**CI/CD Pipeline:**
- GitHub Actions
- Trunk-based development
- Automated testing on PR
- Preview deployments for frontend
- Blue-green deployment for production

**Monitoring & Observability:**
- Prometheus + Grafana for metrics
- Structured JSON logs → Loki/ELK
- OpenTelemetry for distributed tracing
- Sentry for error tracking
- Uptime monitoring (external)

**Scaling Path:**
- Year 1: Vertical scaling, read replicas
- Year 2: Horizontal pods, CDN, edge caching
- Year 3: Multi-region, database sharding

### Testing Strategy

| Layer | Tool | Target |
|-------|------|--------|
| Backend Unit | Rust test + tokio-test | 80% |
| Backend Integration | testcontainers-rs | Critical paths |
| API Contract | OpenAPI validation | 100% |
| Web Unit | Vitest | 80% |
| Web Component | React Testing Library | 70% |
| Web E2E | Playwright | Critical flows |
| Mobile Unit | Jest | 70% |
| Mobile E2E | Detox | Critical flows |
| KMP Unit | Kotest | 70% |
| Accessibility | axe-core (CI) | WCAG 2.1 AA |

### Decision Impact Analysis

**Implementation Sequence:**
1. Backend foundation (auth, multi-tenancy, core entities)
2. API specification and SDK generation
3. Web apps with shared design system
4. Mobile apps with offline capability
5. AI/ML features (Phase 3)
6. Reality Portal and portal integrations (Phase 4)

**Cross-Component Dependencies:**
- Design tokens → All frontends
- OpenAPI spec → SDK generation → All clients
- Auth service → All authenticated endpoints
- Multi-tenancy middleware → All API routes
- WebSocket server → Real-time features in all apps

## Implementation Patterns & Consistency Rules

### Naming Patterns

**Database Naming Conventions:**

| Element | Convention | Example |
|---------|------------|---------|
| Tables | snake_case, plural | `users`, `buildings`, `fault_reports` |
| Columns | snake_case | `user_id`, `created_at`, `organization_id` |
| Primary Keys | `id` (uuid) | `id UUID PRIMARY KEY DEFAULT gen_random_uuid()` |
| Foreign Keys | `{table_singular}_id` | `user_id`, `building_id` |
| Indexes | `idx_{table}_{columns}` | `idx_users_email`, `idx_faults_building_id_status` |
| Constraints | `{type}_{table}_{desc}` | `chk_users_email_format`, `unq_users_email` |
| Enums | PascalCase | `FaultStatus`, `VotingType` |

**API Naming Conventions:**

| Element | Convention | Example |
|---------|------------|---------|
| Endpoints | kebab-case, plural nouns | `/api/v1/fault-reports`, `/api/v1/buildings` |
| Route params | camelCase | `/api/v1/buildings/{buildingId}/units` |
| Query params | camelCase | `?pageSize=20&sortBy=createdAt` |
| Request body | camelCase | `{ "buildingId": "...", "faultType": "..." }` |
| Response body | camelCase | `{ "data": {...}, "pagination": {...} }` |
| Headers | Title-Case | `Authorization`, `X-Request-Id`, `X-Tenant-Id` |

**Code Naming Conventions:**

*Rust (Backend):*

| Element | Convention | Example |
|---------|------------|---------|
| Modules | snake_case | `fault_reports`, `auth_middleware` |
| Structs | PascalCase | `FaultReport`, `TenantContext` |
| Functions | snake_case | `get_fault_by_id`, `create_building` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_UPLOAD_SIZE`, `JWT_EXPIRY` |
| Traits | PascalCase | `Repository`, `EventPublisher` |

*TypeScript (Frontend):*

| Element | Convention | Example |
|---------|------------|---------|
| Files (components) | PascalCase.tsx | `FaultCard.tsx`, `VotingModal.tsx` |
| Files (utilities) | camelCase.ts | `formatDate.ts`, `apiClient.ts` |
| Files (hooks) | use*.ts | `useFaults.ts`, `useAuth.ts` |
| Components | PascalCase | `FaultCard`, `BuildingList` |
| Functions | camelCase | `getFaultById`, `formatCurrency` |
| Constants | SCREAMING_SNAKE_CASE | `API_BASE_URL`, `MAX_FILE_SIZE` |
| Types/Interfaces | PascalCase | `FaultReport`, `User`, `ApiResponse<T>` |

*Kotlin (KMP):*

| Element | Convention | Example |
|---------|------------|---------|
| Packages | lowercase | `three.two.bit.ppt.reality.data` |
| Classes | PascalCase | `ListingRepository`, `FavoriteViewModel` |
| Functions | camelCase | `getListingById`, `toggleFavorite` |
| Properties | camelCase | `isLoading`, `currentUser` |

### Structure Patterns

**Test Location by Platform:**

| Platform | Location | Pattern |
|----------|----------|---------|
| Rust | `tests/` directory at crate level | `tests/integration/auth_test.rs` |
| TypeScript | Co-located with source | `FaultCard.test.tsx` next to `FaultCard.tsx` |
| React Native | `__tests__/` directory | `__tests__/FaultCard.test.tsx` |
| KMP | `commonTest`, `androidTest`, `iosTest` | Standard Gradle convention |

**Component Organization (TypeScript):**

```
src/
├── components/           # Shared UI components
│   ├── ui/              # Primitive UI (buttons, inputs)
│   └── domain/          # Domain-specific (FaultCard, VoteWidget)
├── features/            # Feature modules
│   ├── faults/
│   │   ├── components/  # Feature-specific components
│   │   ├── hooks/       # Feature-specific hooks
│   │   ├── api/         # API calls for this feature
│   │   └── types.ts     # Feature types
│   └── voting/
├── hooks/               # Shared hooks
├── lib/                 # Utilities, helpers
├── stores/              # Zustand stores
└── types/               # Global types
```

**Service Organization (Rust):**

```
src/
├── api/                 # HTTP handlers
│   ├── handlers/        # Route handlers by domain
│   ├── middleware/      # Request middleware
│   └── extractors/      # Custom extractors
├── domain/              # Business logic
│   ├── models/          # Domain models
│   ├── services/        # Domain services
│   └── events/          # Domain events
├── infra/               # Infrastructure
│   ├── db/              # Database access
│   ├── cache/           # Redis access
│   └── external/        # External API clients
└── config/              # Configuration
```

### Format Patterns

**API Response Formats:**

*Success Response:*
```json
{
  "data": { ... },
  "meta": {
    "requestId": "uuid",
    "timestamp": "2025-12-20T10:30:00Z"
  }
}
```

*Paginated Response:*
```json
{
  "data": [...],
  "pagination": {
    "page": 1,
    "pageSize": 20,
    "total": 150,
    "totalPages": 8,
    "hasMore": true
  },
  "meta": { ... }
}
```

*Error Response:*
```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Human-readable message",
    "details": [
      { "field": "email", "code": "INVALID_FORMAT", "message": "Invalid email" }
    ],
    "requestId": "uuid"
  }
}
```

**Date/Time Formats:**

| Context | Format | Example |
|---------|--------|---------|
| API JSON | ISO 8601 UTC | `2025-12-20T10:30:00Z` |
| Database | TIMESTAMPTZ | Native PostgreSQL |
| Display (SK/CZ) | `d.M.yyyy HH:mm` | `20.12.2025 10:30` |
| Display (EN) | `MMM d, yyyy h:mm a` | `Dec 20, 2025 10:30 AM` |

**HTTP Status Codes:**

| Status | When to Use |
|--------|-------------|
| 200 | Successful GET, PUT, PATCH |
| 201 | Successful POST (resource created) |
| 204 | Successful DELETE |
| 400 | Validation error, malformed request |
| 401 | Missing or invalid authentication |
| 403 | Authenticated but not authorized |
| 404 | Resource not found |
| 409 | Conflict (duplicate, version mismatch) |
| 422 | Business rule violation |
| 429 | Rate limit exceeded |
| 500 | Server error (never expose details) |

### Communication Patterns

**Event Naming:**

| Format | Example |
|--------|---------|
| Domain events | `{aggregate}.{action}` in past tense | `fault.created`, `vote.cast`, `user.registered` |
| UI events | `on{Action}` | `onFaultSubmit`, `onVoteCast` |
| WebSocket | `{domain}:{action}` | `fault:updated`, `voting:closed` |

**Event Payload Structure:**
```json
{
  "eventId": "uuid",
  "eventType": "fault.created",
  "timestamp": "2025-12-20T10:30:00Z",
  "aggregateId": "fault-uuid",
  "tenantId": "org-uuid",
  "data": { ... },
  "metadata": {
    "userId": "user-uuid",
    "correlationId": "request-uuid"
  }
}
```

**State Management Patterns (Frontend):**

*TanStack Query Keys:*
```typescript
// Pattern: [domain, action, ...params]
queryKey: ['faults', 'list', buildingId]
queryKey: ['faults', 'detail', faultId]
queryKey: ['voting', 'active', buildingId]
```

*Zustand Store Pattern:*
```typescript
// Pattern: Slice per domain
interface FaultSlice {
  selectedFaultId: string | null;
  setSelectedFault: (id: string | null) => void;
}
```

### Process Patterns

**Error Handling:**

*Backend (Rust):*
```rust
// Use Result<T, AppError> everywhere
// AppError maps to HTTP status codes
pub enum AppError {
    NotFound(String),
    Unauthorized(String),
    Forbidden(String),
    Validation(Vec<ValidationError>),
    Internal(anyhow::Error),
}
```

*Frontend (TypeScript):*
- Use Error Boundaries for React component errors
- Use TanStack Query error handling for API errors
- Show toast for recoverable errors, error page for fatal errors

**Loading States:**

| State | UI Pattern |
|-------|------------|
| Initial load | Skeleton placeholder |
| Refetch | Subtle indicator (spinner in header) |
| Mutation | Button loading state |
| Infinite scroll | Loader at bottom |
| Optimistic update | Immediate UI + rollback on error |

**Validation Timing:**

| Layer | When | What |
|-------|------|------|
| Client | On blur + submit | Format, required fields |
| API | On request | Full validation, business rules |
| Database | On insert/update | Constraints, RLS |

### Enforcement Guidelines

**All AI Agents MUST:**
1. Follow naming conventions exactly as specified (no exceptions)
2. Use the standard API response format for all endpoints
3. Place tests according to platform conventions
4. Use camelCase for JSON fields in API requests/responses
5. Include `requestId` in all API responses
6. Use ISO 8601 UTC for all date/time in APIs
7. Apply tenant context to all database queries
8. Log with structured JSON format

**Pattern Enforcement:**
- ESLint/Biome rules for TypeScript naming
- Clippy lints for Rust naming
- Spectral rules for API consistency
- PR review checklist includes pattern compliance
- CI fails on pattern violations

### Pattern Examples

**Good Example - Rust Handler:**
```rust
pub async fn get_fault_by_id(
    State(ctx): State<AppContext>,
    tenant: TenantContext,
    Path(fault_id): Path<Uuid>,
) -> Result<Json<ApiResponse<FaultResponse>>, AppError> {
    let fault = ctx.fault_service
        .get_by_id(&tenant, fault_id)
        .await?
        .ok_or(AppError::NotFound("Fault not found".into()))?;

    Ok(Json(ApiResponse::ok(fault.into())))
}
```

**Good Example - TypeScript Component:**
```typescript
// src/features/faults/components/FaultCard.tsx
export function FaultCard({ fault }: FaultCardProps) {
  const { mutate: updateStatus, isPending } = useUpdateFaultStatus();

  return (
    <Card>
      <CardHeader>{fault.title}</CardHeader>
      <CardContent>
        <StatusBadge status={fault.status} />
        <Button
          loading={isPending}
          onClick={() => updateStatus(fault.id)}
        >
          Mark Resolved
        </Button>
      </CardContent>
    </Card>
  );
}
```

**Anti-Patterns to Avoid:**
- ❌ `userId` in database columns (use `user_id`)
- ❌ `/api/v1/getFaults` (use `/api/v1/faults`)
- ❌ `{ error: "Something went wrong" }` (use standard error format)
- ❌ `2025-12-20 10:30:00` in JSON (use ISO 8601 with Z)
- ❌ Co-locating tests in Rust (use `tests/` directory)
- ❌ Using `any` type in TypeScript
- ❌ Direct database queries without tenant context

## Project Structure & Boundaries

### Complete Project Directory Structure

```
property-management/
├── CLAUDE.md                         # Root context: namespace, architecture
├── VERSION                           # Single source of truth for version
│
├── docs/                             # Documentation
│   ├── CLAUDE.md                    # Docs context
│   ├── spec1.0.md                   # Original specification
│   ├── use-cases.md                 # 508 use cases catalog
│   ├── functional-requirements.md   # FR by use case
│   ├── non-functional-requirements.md
│   ├── architecture.md              # ADRs, service boundaries
│   ├── technical-design.md          # API endpoints, DTOs, state machines
│   ├── domain-model.md              # Entities, aggregates
│   ├── project-structure.md         # Structure reference
│   └── api/                         # API specifications
│       ├── README.md
│       ├── typespec/                # TypeSpec sources
│       │   ├── main.tsp
│       │   ├── tspconfig.yaml
│       │   ├── domains/             # Domain-specific specs
│       │   └── shared/              # Shared models
│       ├── generated/               # Generated OpenAPI
│       │   ├── openapi.yaml         # Full consolidated spec
│       │   └── by-service/
│       │       ├── api-server.yaml
│       │       └── reality-server.yaml
│       └── rules/
│           └── spectral.yaml        # API linting rules
│
├── backend/                          # Rust Backend
│   ├── CLAUDE.md                    # Backend context
│   ├── Cargo.toml                   # Workspace configuration
│   ├── Cargo.lock
│   ├── .sqlx/                       # SQLx offline cache
│   ├── crates/                      # Shared libraries
│   │   ├── common/                  # Core types (TenantContext, AppError, ApiResponse)
│   │   ├── api-core/                # HTTP middleware, extractors
│   │   ├── db/                      # Database layer, repositories
│   │   └── integrations/            # External APIs (email, push, storage, portals)
│   ├── servers/                     # Backend servers
│   │   ├── api-server/              # Property Management API (Port 8080)
│   │   └── reality-server/          # Reality Portal API (Port 8081)
│   └── migrations/                  # Database migrations
│
├── frontend/                         # TypeScript Frontend
│   ├── CLAUDE.md                    # Frontend context
│   ├── package.json                 # pnpm workspace root
│   ├── pnpm-workspace.yaml
│   ├── turbo.json                   # Turborepo config
│   ├── packages/                    # Shared packages
│   │   ├── api-client/              # @ppt/api-client (api-server SDK)
│   │   ├── reality-api-client/      # @ppt/reality-api-client (reality-server SDK)
│   │   ├── shared/                  # @ppt/shared (utilities)
│   │   ├── ui-kit/                  # @ppt/ui-kit (components)
│   │   └── design-tokens/           # @ppt/design-tokens (Style Dictionary)
│   └── apps/                        # Applications
│       ├── ppt-web/                 # @ppt/web - React SPA (Property Mgmt)
│       ├── reality-web/             # @ppt/reality-web - Next.js SSR (Reality Portal)
│       └── mobile/                  # @ppt/mobile - React Native (Property Mgmt)
│
├── mobile-native/                    # Kotlin Multiplatform (Reality Portal)
│   ├── CLAUDE.md                    # KMP context
│   ├── build.gradle.kts
│   ├── shared/                      # Common code (data, domain, presentation)
│   ├── androidApp/                  # three.two.bit.ppt.reality
│   └── iosApp/                      # three.two.bit.ppt.reality
│
├── infra/                            # Infrastructure as Code
│   ├── docker-compose.yml           # Local development
│   ├── kubernetes/                  # K8s manifests
│   └── terraform/                   # Cloud provisioning
│
├── scripts/                          # Build and deployment scripts
│   ├── bump-version.sh
│   ├── install-hooks.sh
│   ├── generate-sdk.sh
│   └── dev-setup.sh
│
└── .github/workflows/                # CI/CD
    ├── api-validation.yml
    ├── backend.yml
    ├── frontend.yml
    └── mobile-native.yml
```

### Architectural Boundaries

**API Boundaries:**

| Boundary | Endpoint | Auth | Consumers |
|----------|----------|------|-----------|
| api-server external | `/api/v1/*` | JWT + Tenant | ppt-web, mobile (RN) |
| reality-server external | `/api/v1/*` | Optional JWT | reality-web, mobile-native |
| WebSocket | `/ws` | JWT | ppt-web, mobile (RN) |
| OAuth | `/oauth/*` | None → JWT | All clients |

**Service Boundaries:**

| Service | Responsibility | Dependencies |
|---------|---------------|--------------|
| api-server | PM business logic, OAuth provider | PostgreSQL, Redis, S3 |
| reality-server | Portal listings, search, SSO consumer | PostgreSQL, Redis, api-server |
| worker (future) | Async jobs (notifications, AI) | RabbitMQ/SQS, external APIs |

**Data Boundaries:**

| Schema | Owner | Access Pattern |
|--------|-------|----------------|
| `auth.*` | api-server | Write: api-server, Read: both |
| `organizations.*` | api-server | Both servers via RLS |
| `buildings.*` | api-server | Both servers via RLS |
| `listings.*` | reality-server | Write: reality-server, Read: both |
| `portal_users.*` | reality-server | reality-server only |

### Requirements to Structure Mapping

**Use Case Category → Location:**

| Category | Backend | Frontend |
|----------|---------|----------|
| UC-01 Notifications | `api-server/handlers/notifications.rs` | `ppt-web/features/notifications/` |
| UC-03 Faults | `api-server/handlers/faults.rs` | `ppt-web/features/faults/` |
| UC-04 Voting | `api-server/handlers/voting.rs` | `ppt-web/features/voting/` |
| UC-14 User Accounts | `api-server/handlers/auth.rs` | `ppt-web/features/auth/` |
| UC-15 Buildings | `api-server/handlers/buildings.rs` | `ppt-web/features/buildings/` |
| UC-27 Organizations | `crates/common/tenant.rs` | `ppt-web/features/organization/` |
| UC-31 Listings | `reality-server/handlers/listings.rs` | `reality-web/app/listings/` |
| UC-44 Favorites | `reality-server/handlers/favorites.rs` | `mobile-native/shared/domain/` |

**Cross-Cutting Concerns:**

| Concern | Location |
|---------|----------|
| Authentication | `crates/api-core/middleware/auth.rs` |
| Multi-tenancy | `crates/api-core/extractors/tenant.rs` |
| Error handling | `crates/common/error.rs` |
| API response format | `crates/common/response.rs` |
| Design tokens | `frontend/packages/design-tokens/` |
| UI components | `frontend/packages/ui-kit/` |
| API clients | `frontend/packages/api-client/`, `reality-api-client/` |

### Integration Points

**Internal Communication:**

```
ppt-web, mobile (RN) ──────► api-server:8080 ◄──┐
                                    │           │ (shared DB)
reality-web, mobile-native ────► reality-server:8081
```

**External Integrations:**

| Integration | Location | Use Cases |
|-------------|----------|-----------|
| SendGrid | `crates/integrations/email/` | UC-01.1, UC-01.4 |
| FCM/APNs | `crates/integrations/push/` | UC-01.2, UC-01.3 |
| S3 | `crates/integrations/storage/` | UC-08.1, UC-08.2 |
| Portals | `crates/integrations/portals/` | UC-32.1 |

### Development Workflow

**Local Development:**
```bash
docker-compose up -d                    # PostgreSQL, Redis, MinIO
cd backend && cargo run --bin api-server
cd frontend && pnpm dev:ppt             # :5173
cd frontend && pnpm dev:reality         # :3000
```

**Build Process:**
```bash
cargo build --release                   # Backend
pnpm build                              # Frontend (Turborepo)
./gradlew assembleRelease               # Android
```

## Architecture Validation Results

### Coherence Validation ✅

**Decision Compatibility:**
All technology choices are compatible and work together:
- Rust (Axum 0.8.6) + PostgreSQL + Redis - proven stack for high-performance backends
- React 19 + Vite 6 + TanStack Query - modern frontend stack with excellent DX
- Next.js 15.5 + Tailwind - SSR-optimized for Reality Portal SEO requirements
- React Native 0.83 New Architecture - aligned with React 19 for code sharing
- Kotlin Multiplatform 2.3 - stable for cross-platform Reality mobile

No version conflicts detected. All dependencies are on current stable releases (December 2025).

**Pattern Consistency:**
- Naming conventions consistent across all platforms (snake_case DB, camelCase JSON, PascalCase components)
- API response format standardized across both servers
- Error handling patterns align with HTTP status code conventions
- Event payload structure consistent for all domain events

**Structure Alignment:**
- Monorepo structure (Cargo workspace, pnpm workspace) supports code sharing patterns
- Shared crates (`common`, `api-core`, `db`) prevent duplication
- Design tokens shared via `@ppt/design-tokens` enable consistent UI
- OpenAPI-generated SDKs ensure API contract compliance

### Requirements Coverage Validation ✅

**Use Case Coverage:**
All 51 use case categories have architectural support:

| Phase | Categories | Architectural Coverage |
|-------|------------|----------------------|
| Phase 1 | UC-01 to UC-15, UC-27 | api-server handlers, ppt-web features |
| Phase 2 | UC-16 to UC-18 | Financial module, accounting integrations |
| Phase 3 | UC-19 to UC-26 | AI/ML crates, real-time WebSocket |
| Phase 4 | UC-29 to UC-34, UC-44 to UC-51 | reality-server, mobile-native |

**Non-Functional Requirements:**

| NFR | Architectural Support |
|-----|----------------------|
| P95 < 200ms | Rust + Axum, Redis caching, PostgreSQL optimization |
| 99.9% uptime | Kubernetes, health checks, graceful degradation |
| GDPR compliance | Audit logging, data export/delete endpoints, RLS |
| WCAG 2.1 AA | Radix UI (accessible primitives), design tokens |
| LCP < 2.5s | Next.js SSR/SSG, image optimization, CDN |

### Implementation Readiness Validation ✅

**Decision Completeness:**
- ✅ All critical decisions documented with specific versions
- ✅ Technology stack fully specified for all 4 platforms
- ✅ Integration patterns defined (OAuth, WebSocket, push, external APIs)
- ✅ Testing strategy with coverage targets

**Structure Completeness:**
- ✅ Complete directory structure with all major files
- ✅ Clear service boundaries (api-server vs reality-server)
- ✅ Shared crate organization defined
- ✅ Frontend monorepo packages specified

**Pattern Completeness:**
- ✅ Database naming conventions with examples
- ✅ API naming conventions with examples
- ✅ Code naming conventions per platform
- ✅ Event naming and payload structure
- ✅ Error handling patterns with code examples

### Gap Analysis Results

**No Critical Gaps Found**

**Important Gaps (Addressed by existing docs):**
- Database schema details → `docs/domain-model.md`
- API endpoint specifications → `docs/technical-design.md`
- State machine definitions → `docs/technical-design.md`
- Security details → `docs/architecture.md` ADRs

**Nice-to-Have (Future Enhancement):**
- Detailed Kubernetes helm charts
- Performance benchmarking scripts
- Multi-region deployment architecture (Year 3)

### Architecture Completeness Checklist

**✅ Requirements Analysis**
- [x] Project context thoroughly analyzed (508 use cases, 51 categories)
- [x] Scale and complexity assessed (Enterprise, 15+ components)
- [x] Technical constraints identified (Rust, PostgreSQL, multi-platform)
- [x] Cross-cutting concerns mapped (8 concerns identified)

**✅ Architectural Decisions**
- [x] Critical decisions documented with versions (December 2025)
- [x] Technology stack fully specified (4 platforms)
- [x] Integration patterns defined (OAuth, WebSocket, external APIs)
- [x] Performance considerations addressed (caching, SSR, CDN)

**✅ Implementation Patterns**
- [x] Naming conventions established (DB, API, code per platform)
- [x] Structure patterns defined (tests, components, services)
- [x] Communication patterns specified (events, state, real-time)
- [x] Process patterns documented (errors, loading, validation)

**✅ Project Structure**
- [x] Complete directory structure defined
- [x] Component boundaries established
- [x] Integration points mapped
- [x] Requirements to structure mapping complete

### Architecture Readiness Assessment

**Overall Status:** ✅ READY FOR IMPLEMENTATION

**Confidence Level:** HIGH

**Key Strengths:**
1. Existing ADRs provide solid architectural foundation
2. Modular monolith simplifies initial development
3. Shared database with RLS provides data consistency
4. Token-first design system enables cross-platform UI consistency
5. OpenAPI-generated SDKs ensure API contract compliance

**Areas for Future Enhancement:**
- Detailed Kubernetes deployment manifests
- Performance benchmarking baseline
- Multi-region architecture (Year 3+)

### Implementation Handoff

**AI Agent Guidelines:**
1. Follow all architectural decisions exactly as documented
2. Use implementation patterns consistently across all components
3. Respect project structure and boundaries
4. Refer to this document for all architectural questions
5. Check `docs/architecture.md` for existing ADRs
6. Use `docs/technical-design.md` for API specifications

**First Implementation Priority:**
```bash
# 1. Initialize project structure
./scripts/dev-setup.sh

# 2. Backend foundation
cd backend && cargo build

# 3. Frontend packages
cd frontend && pnpm install && pnpm build

# 4. Database migrations
sqlx migrate add initial_schema
```

**Recommended Epic Sequence:**
1. EPIC-001: User Authentication & Multi-tenancy (UC-14, UC-27)
2. EPIC-002: Building & Unit Management (UC-15)
3. EPIC-003: Fault Reporting (UC-03)
4. EPIC-004: Voting System (UC-04)
5. EPIC-005: Document Management (UC-08)

