# Documentation Index

Property Management System (PPT) and Reality Portal documentation.

## Quick Links

| Document | Description |
|----------|-------------|
| [CLAUDE.md](./CLAUDE.md) | AI assistant context and conventions |
| [use-cases.md](./use-cases.md) | Complete use case catalog (508 UCs) |
| [architecture.md](./architecture.md) | System architecture and ADRs |

---

## Core Documentation

### Product Specification

- **[spec1.0.md](./spec1.0.md)** - Original system specification with UI/feature details
- **[use-cases.md](./use-cases.md)** - Complete use case catalog (508 use cases, 51 categories)
- **[functional-requirements.md](./functional-requirements.md)** - Business rules and acceptance criteria for all use cases

### Domain & Architecture

- **[domain-model.md](./domain-model.md)** - Entities, aggregates, value objects, and relationships (DDD)
- **[architecture.md](./architecture.md)** - System architecture, 10 ADRs, service boundaries
- **[sequence-diagrams.md](./sequence-diagrams.md)** - 23 interaction flows with sync/async patterns

### Technical Design

- **[technical-design.md](./technical-design.md)** - API endpoints (~145), DTOs (~127), 8 state machines
- **[non-functional-requirements.md](./non-functional-requirements.md)** - Performance, security, SEO, caching, SSR/SSG
- **[testability-and-implementation.md](./testability-and-implementation.md)** - Test strategy, MVP priorities, 12-iteration roadmap

### Project Reference

- **[project-structure.md](./project-structure.md)** - Full monorepo directory structure
- **[CLAUDE.md](./CLAUDE.md)** - AI assistant context, conventions, API strategy

---

## Reviews & Analysis

- **[ARCHITECTURE_REVIEW.md](./ARCHITECTURE_REVIEW.md)** - Code/architecture review, gaps, and prioritized roadmap
- **[DOCUMENTATION_DEEP_DIVE.md](./DOCUMENTATION_DEEP_DIVE.md)** - Doc audit, contradictions resolved, source-of-truth model

---

## API Specifications

### [api/](./api/)

- **[README.md](./api/README.md)** - API documentation index and quick start guide

### TypeSpec Source ([api/typespec/](./api/typespec/))

#### Configuration

- **[main.tsp](./api/typespec/main.tsp)** - TypeSpec entry point
- **[tspconfig.yaml](./api/typespec/tspconfig.yaml)** - TypeSpec compiler configuration

#### Shared Models ([api/typespec/shared/](./api/typespec/shared/))

- **[models.tsp](./api/typespec/shared/models.tsp)** - Common types (Address, Money, Attachment)
- **[errors.tsp](./api/typespec/shared/errors.tsp)** - Standard error responses
- **[pagination.tsp](./api/typespec/shared/pagination.tsp)** - Pagination patterns
- **[tenant.tsp](./api/typespec/shared/tenant.tsp)** - Multi-tenant context and roles

#### Domain Specifications ([api/typespec/domains/](./api/typespec/domains/))

| File | Use Cases | Description |
|------|-----------|-------------|
| **[auth.tsp](./api/typespec/domains/auth.tsp)** | UC-14 | Authentication, registration, MFA, OAuth |
| **[organizations.tsp](./api/typespec/domains/organizations.tsp)** | UC-27 | Multi-tenancy, organizations, members |
| **[buildings.tsp](./api/typespec/domains/buildings.tsp)** | UC-15 | Buildings, floors, common areas |
| **[units.tsp](./api/typespec/domains/units.tsp)** | - | Properties, units, ownership |
| **[faults.tsp](./api/typespec/domains/faults.tsp)** | UC-03 | Fault reporting and workflow |
| **[voting.tsp](./api/typespec/domains/voting.tsp)** | UC-04 | Voting, polls, proxy delegation |
| **[documents.tsp](./api/typespec/domains/documents.tsp)** | UC-08 | Document management, folders, versions |
| **[rentals.tsp](./api/typespec/domains/rentals.tsp)** | UC-29-30 | Short-term rentals, guest registration |
| **[listings.tsp](./api/typespec/domains/listings.tsp)** | UC-31-32 | Real estate listings, search |
| **[compliance.tsp](./api/typespec/domains/compliance.tsp)** | UC-26 | GDPR, data export/deletion |

### Generated Output ([api/generated/](./api/generated/))

- OpenAPI specs generated from TypeSpec (run `npx tsp compile .`)

### Linting Rules ([api/rules/](./api/rules/))

- **[spectral.yaml](./api/rules/spectral.yaml)** - OpenAPI linting rules

---

## Validation

### [validation/](./validation/)

- **[checklist.md](./validation/checklist.md)** - Stakeholder validation checklist and review tracking
- **[edge-cases.md](./validation/edge-cases.md)** - Edge cases, error paths, and exception handling

---

## Statistics

| Metric | Count |
|--------|-------|
| Use Cases | 508 |
| Categories | 51 |
| API Endpoints | ~145 |
| DTOs | ~127 |
| State Machines | 8 |
| Planned Test Cases | ~1,100 |

---

## Document Relationships

```
spec1.0.md (original spec)
    │
    ▼
use-cases.md (508 UCs)
    │
    ├──► functional-requirements.md (BRs, acceptance criteria)
    │
    ├──► domain-model.md (entities, aggregates)
    │
    ├──► sequence-diagrams.md (interaction flows)
    │
    └──► api/typespec/** (canonical API contract)
              │
              ▼
         technical-design.md (DTOs, state machines)
              │
              ▼
         testability-and-implementation.md (tests, roadmap)
```

---

*Last updated: 2024-12-20*
