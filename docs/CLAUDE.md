# Property Management System - Project Documentation

## Overview

This repository contains the requirements specification for a comprehensive **Property Management System** designed for housing cooperatives, property management companies, and building administrators.

## Documents

| File | Description |
|------|-------------|
| `spec1.0.md` | Original system specification with UI/feature details |
| `use-cases.md` | Complete use case catalog (407 use cases, 43 categories) |

## Use Cases Summary

### Statistics
- **Total Use Cases:** 407
- **Categories:** 43
- **Actors:** 16

### Actor Hierarchy

```
Super Administrator (Platform Level)
└── Organization (Housing Cooperative / Property Management Company)
        ├── Organization Admin
        ├── Manager
        ├── Technical Manager
        └── Building
                └── Unit
                        ├── Owner
                        │       └── Owner Delegate
                        ├── Tenant
                        ├── Resident
                        ├── Property Manager
                        │       └── Guest
                        └── Real Estate Agent
```

### Categories by Domain

#### Core Property Management (UC-01 to UC-18)
- Notifications, Announcements, Faults, Voting, Messages
- Neighbors, Contacts, Documents, Forms
- Person-Months, Self-Readings, Outages, News
- User Accounts, Building Management, Financial, Reports, Admin

#### Modern Technology (UC-19 to UC-26)
- Real-time & Mobile Features
- AI/ML Features (chatbot, OCR, predictions)
- IoT & Smart Building
- External Integrations
- Security & Compliance (GDPR, 2FA)
- Community & Social
- Accessibility (WCAG)
- Workflow Automation

#### Multi-tenancy & Rental (UC-27 to UC-34)
- Organizations (multi-tenancy)
- Delegation & Permissions
- Short-term Rental (Airbnb/Booking)
- Guest Registration System
- Real Estate & Listings
- Portal Integration (API)
- Tenant Screening
- Lease Management

#### Operations & Support (UC-35 to UC-43)
- Insurance Management
- Maintenance Scheduling
- Supplier/Vendor Management
- Legal & Compliance
- Emergency Management
- Budget & Planning
- Subscription & Billing
- Onboarding & Help
- Mobile App Features

## Key Features

### AI/ML Capabilities
- AI Chatbot for common questions
- OCR for meter readings and ID documents
- Predictive maintenance
- Sentiment analysis
- Smart search with NLP
- Automated document summarization

### Integrations
- Airbnb / Booking.com sync
- Real estate portal API
- Calendar (Google/Outlook)
- Video conferencing (Zoom/Teams)
- E-signatures (DocuSign)
- SMS gateway
- Government portals

### Security
- Two-factor authentication
- Biometric login
- GDPR compliance (data export/deletion)
- End-to-end encryption
- Audit trails

## Development Notes

### When implementing use cases:
1. Each use case has a unique ID (e.g., UC-03.5)
2. Actors define who can perform the action
3. Descriptions are intentionally brief - expand as needed

### Recommended implementation order:
1. **Priority 1:** UC-14 (User Accounts), UC-27 (Organizations), UC-15 (Buildings)
2. **Priority 2:** UC-01 to UC-13 (Core features)
3. **Priority 3:** UC-16 to UC-18 (Financial, Reports, Admin)
4. **Priority 4:** Modern tech features (UC-19 to UC-26)
5. **Priority 5:** Rental & Real Estate (UC-29 to UC-34)
6. **Priority 6:** Operations (UC-35 to UC-43)

## File Conventions

- All documentation is in English
- Use Markdown format
- Use case IDs follow pattern: `UC-XX.Y` where XX is category, Y is sequence

## Naming Conventions

### Product Requirements Documents (PRD)

```
PRD-{NNN}-{feature-name}.md
```

**Examples:**
- `PRD-001-user-authentication.md`
- `PRD-002-fault-management.md`
- `PRD-003-airbnb-integration.md`

**Structure:**
1. Overview
2. Problem Statement
3. Goals & Success Metrics
4. User Stories
5. Requirements (Functional / Non-functional)
6. Out of Scope
7. Dependencies
8. Timeline

### Epics

```
EPIC-{NNN}-{epic-name}.md
```

**Examples:**
- `EPIC-001-user-management.md`
- `EPIC-002-voting-system.md`
- `EPIC-003-short-term-rentals.md`

**Mapping to Use Cases:**
| Epic | Use Case Categories |
|------|---------------------|
| EPIC-001 | UC-14 (User Accounts), UC-27 (Organizations) |
| EPIC-002 | UC-04 (Voting), UC-28 (Delegation) |
| EPIC-003 | UC-29 (Rentals), UC-30 (Guest Registration) |

**Structure:**
1. Epic Summary
2. Business Value
3. Acceptance Criteria
4. Stories List
5. Dependencies
6. Risks

### Stories

```
STORY-{EPIC}-{NNN}-{story-description}.md
```

**Examples:**
- `STORY-001-001-user-registration.md`
- `STORY-001-002-password-reset.md`
- `STORY-002-001-create-vote.md`

**Alternative format (with use case reference):**
```
STORY-UC{XX}-{Y}-{description}.md
```
- `STORY-UC14-1-register-account.md`
- `STORY-UC04-7-create-vote.md`

**Structure:**
1. User Story (As a... I want... So that...)
2. Acceptance Criteria (Given/When/Then)
3. Technical Notes
4. UI/UX Notes
5. Test Cases
6. Definition of Done

### Directory Structure

```
docs/
├── CLAUDE.md                    # This file
├── spec1.0.md                   # Original specification
├── use-cases.md                 # Use case catalog
├── prd/                         # Product Requirements
│   ├── PRD-001-user-auth.md
│   └── PRD-002-fault-mgmt.md
├── epics/                       # Epics
│   ├── EPIC-001-user-mgmt.md
│   └── EPIC-002-voting.md
└── stories/                     # User Stories
    ├── EPIC-001/
    │   ├── STORY-001-001-registration.md
    │   └── STORY-001-002-login.md
    └── EPIC-002/
        └── STORY-002-001-create-vote.md
```

### Branch Naming

```
feature/{EPIC-NNN}-{description}
bugfix/{STORY-NNN}-{description}
hotfix/{issue-description}
```

**Examples:**
- `feature/EPIC-001-user-management`
- `feature/STORY-001-003-2fa-authentication`
- `bugfix/STORY-002-001-vote-count-fix`

### Commit Messages

```
{type}({scope}): {description}

[optional body]
[optional footer]
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

**Examples:**
- `feat(UC-14): implement user registration`
- `fix(UC-04): correct vote counting logic`
- `docs(PRD-001): add security requirements`

---

## API Strategy

### Single Source of Truth: OpenAPI

The API specification serves as the single source of truth for all clients and services. We use a **hybrid API-first approach**:

| Approach | Use For |
|----------|---------|
| **Design-First** | Multi-tenancy, authentication, GDPR, external integrations |
| **Code-First** | Internal CRUD, business logic, rapidly evolving features |

### Technology Stack

#### API Specifications
| Component | Tool | Purpose |
|-----------|------|---------|
| Spec Language | **TypeSpec** | Design-first, code-like syntax |
| OpenAPI Version | **3.1** | Latest standard |
| Linting | **Spectral** | Enforce naming, security rules |
| Breaking Changes | **oasdiff** | Detect breaking changes in CI |
| Documentation | **Stoplight/ReDoc** | Interactive API docs |

#### Backend (Rust)
| Component | Tool | Purpose |
|-----------|------|---------|
| Framework | **Axum** | High-performance, type-safe |
| OpenAPI | **utoipa** | Derive specs from Rust code |
| Validation | **validator** | Request validation with derive |
| Database | **sqlx** / **sea-orm** | Async database access |

#### Frontend (TypeScript)
| Component | Tool | Purpose |
|-----------|------|---------|
| Web | **React** | SPA web application |
| Mobile | **React Native** | Cross-platform mobile |
| SDK Generation | **@hey-api/openapi-ts** | Type-safe API client |
| State | **TanStack Query** | Server state + caching |

#### Mobile Native (Kotlin)
| Component | Tool | Purpose |
|-----------|------|---------|
| Framework | **Kotlin Multiplatform** | Shared business logic |
| Networking | **Ktor Client** | HTTP client |
| SDK Generation | **openapi-generator** | Kotlin API client |

### API Specifications Location

```
docs/api/
├── README.md                    # API documentation index
├── typespec/
│   ├── main.tsp                # Entry point
│   ├── tspconfig.yaml
│   ├── domains/
│   │   ├── auth.tsp            # UC-14: Authentication
│   │   ├── organizations.tsp   # UC-27: Multi-tenancy
│   │   ├── buildings.tsp       # UC-15: Buildings
│   │   ├── units.tsp           # Properties, Units
│   │   ├── faults.tsp          # UC-03: Fault reporting
│   │   ├── voting.tsp          # UC-04: Voting
│   │   ├── documents.tsp       # UC-08: Documents
│   │   ├── rentals.tsp         # UC-29-30: Airbnb/Booking
│   │   ├── listings.tsp        # UC-31-32: Real estate
│   │   └── compliance.tsp      # UC-26: GDPR
│   └── shared/
│       ├── models.tsp          # Common data types
│       ├── errors.tsp          # Error responses
│       ├── pagination.tsp      # List patterns
│       └── tenant.tsp          # Multi-tenant context
├── generated/
│   ├── openapi.yaml            # Full consolidated spec
│   └── by-service/
│       ├── auth-api.yaml
│       ├── property-api.yaml
│       └── integration-api.yaml
└── rules/
    └── spectral.yaml           # Linting rules
```

### Multi-Tenancy Pattern

All API operations require tenant context:

```yaml
# OpenAPI Security Scheme
components:
  securitySchemes:
    OAuth2:
      type: oauth2
      flows:
        authorizationCode:
          scopes:
            'tenant:read': Read tenant data
            'tenant:write': Write tenant data
            'gdpr:export': Export personal data
            'gdpr:delete': Delete personal data
```

```rust
// Rust TenantContext
#[derive(Debug, Clone, ToSchema)]
pub struct TenantContext {
    pub tenant_id: String,
    pub role: TenantRole,
}
```

### Versioning Strategy

- **URI-based**: `/api/v1/properties`, `/api/v2/properties`
- **Max 2 active versions** at any time
- **12-month deprecation window** for breaking changes
- **Response headers**: `Deprecation`, `Sunset`, `Link`

### SDK Generation

```bash
# TypeScript (React, React Native)
npx @hey-api/openapi-ts \
  -i docs/api/generated/openapi.yaml \
  -o frontend/packages/api-client/src/generated

# Kotlin (KMP)
openapi-generator generate \
  -i docs/api/generated/openapi.yaml \
  -g kotlin \
  -o mobile-native/shared/src/commonMain/kotlin/api \
  --additional-properties=library=multiplatform
```

---

## Architecture

### Backend (Rust Multi-Server)

```
backend/
├── Cargo.toml                   # Workspace root
├── crates/                      # Shared libraries
│   ├── common/                  # Core types, errors, TenantContext
│   ├── api-core/               # OpenAPI, extractors, middleware
│   ├── db/                     # Database layer, models, repositories
│   └── integrations/           # Airbnb, Booking, real estate portals
└── servers/                     # Individual servers
    ├── auth-server/            # Authentication service
    ├── property-server/        # Core property management
    └── integration-server/     # External integrations
```

### Frontend (TypeScript Monorepo)

```
frontend/
├── packages/
│   ├── api-client/             # Generated TypeScript SDK
│   ├── shared/                 # Shared components/logic
│   └── ui-kit/                 # Design system
└── apps/
    ├── web/                    # React web app
    └── mobile/                 # React Native
```

### Mobile Native (Kotlin Multiplatform)

```
mobile-native/
├── shared/                      # KMP shared code
│   └── src/
│       ├── commonMain/         # Shared Kotlin (API client)
│       ├── androidMain/        # Android-specific
│       └── iosMain/            # iOS-specific
├── androidApp/                 # Android application
└── iosApp/                     # iOS application
```

---

## CI/CD Pipeline

### API Validation Workflow

```yaml
name: API Validation
on: [push, pull_request]

jobs:
  validate:
    steps:
      - uses: actions/checkout@v4
      - name: Compile TypeSpec
        run: npx tsp compile docs/api/typespec
      - name: Lint OpenAPI
        run: npx spectral lint docs/api/generated/openapi.yaml
      - name: Check Breaking Changes
        run: npx oasdiff breaking origin/main docs/api/generated/openapi.yaml
      - name: Generate SDKs
        run: |
          npx @hey-api/openapi-ts -i docs/api/generated/openapi.yaml -o frontend/packages/api-client/src/generated
```

### Success Metrics

| Metric | Target |
|--------|--------|
| API contract mismatches | 80% reduction |
| Documentation freshness | < 1 hour after code change |
| Cross-tenant data leaks | Zero |
| Integration failure rate | < 1% |
| GDPR compliance | 100% |
