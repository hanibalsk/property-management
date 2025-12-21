# Documentation Index

Property Management System (PPT) and Reality Portal documentation.

## Core Documentation

### Product Specification

- **[spec1.0.md](./spec1.0.md)** - Original system specification with UI/feature details
- **[use-cases.md](./use-cases.md)** - Complete use case catalog (508 UCs, 51 categories)
- **[functional-requirements.md](./functional-requirements.md)** - Business rules and acceptance criteria

### Domain & Architecture

- **[domain-model.md](./domain-model.md)** - DDD entities, aggregates, value objects
- **[architecture.md](./architecture.md)** - System architecture, ADRs, service boundaries
- **[sequence-diagrams.md](./sequence-diagrams.md)** - 23 interaction flows (sync/async patterns)

### Technical Design

- **[technical-design.md](./technical-design.md)** - API endpoints (~145), DTOs (~127), state machines
- **[non-functional-requirements.md](./non-functional-requirements.md)** - Performance, security, SEO, caching
- **[testability-and-implementation.md](./testability-and-implementation.md)** - Test strategy, 12-iteration roadmap

### Project Reference

- **[project-structure.md](./project-structure.md)** - Full monorepo directory structure
- **[CLAUDE.md](./CLAUDE.md)** - AI assistant context and conventions

---

## Reviews & Analysis

- **[ARCHITECTURE_REVIEW.md](./ARCHITECTURE_REVIEW.md)** - Code/architecture review and gap analysis
- **[DOCUMENTATION_DEEP_DIVE.md](./DOCUMENTATION_DEEP_DIVE.md)** - Doc audit, source-of-truth model

---

## api/

API specifications using TypeSpec and OpenAPI.

- **[README.md](./api/README.md)** - API documentation quick start

### api/typespec/

- **[main.tsp](./api/typespec/main.tsp)** - TypeSpec entry point (v0.1.68)

### api/typespec/shared/

- **[models.tsp](./api/typespec/shared/models.tsp)** - Common types (Address, Money, Attachment)
- **[errors.tsp](./api/typespec/shared/errors.tsp)** - Standard error responses
- **[pagination.tsp](./api/typespec/shared/pagination.tsp)** - Pagination patterns
- **[tenant.tsp](./api/typespec/shared/tenant.tsp)** - Multi-tenant context and roles

### api/typespec/domains/

- **[auth.tsp](./api/typespec/domains/auth.tsp)** - Authentication, registration, MFA (UC-14)
- **[organizations.tsp](./api/typespec/domains/organizations.tsp)** - Multi-tenancy, members (UC-27)
- **[buildings.tsp](./api/typespec/domains/buildings.tsp)** - Buildings, floors, areas (UC-15)
- **[units.tsp](./api/typespec/domains/units.tsp)** - Properties, units, ownership
- **[faults.tsp](./api/typespec/domains/faults.tsp)** - Fault reporting workflow (UC-03)
- **[voting.tsp](./api/typespec/domains/voting.tsp)** - Voting, polls, proxy (UC-04)
- **[documents.tsp](./api/typespec/domains/documents.tsp)** - Document management (UC-08)
- **[rentals.tsp](./api/typespec/domains/rentals.tsp)** - Short-term rentals, guests (UC-29-30)
- **[listings.tsp](./api/typespec/domains/listings.tsp)** - Real estate listings (UC-31-32)
- **[compliance.tsp](./api/typespec/domains/compliance.tsp)** - GDPR data export/deletion (UC-26)

### api/rules/

- **[spectral.yaml](./api/rules/spectral.yaml)** - OpenAPI linting rules

---

## validation/

- **[checklist.md](./validation/checklist.md)** - Stakeholder validation and review tracking
- **[edge-cases.md](./validation/edge-cases.md)** - Edge cases, error paths, exception handling

---

## Statistics

| Metric | Count |
|--------|-------|
| Use Cases | 508 |
| Categories | 51 |
| API Endpoints | ~145 |
| DTOs | ~127 |
| State Machines | 8 |

---

*Last updated: 2024-12-21*
