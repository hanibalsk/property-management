# API Documentation

This directory contains the API specifications for the Property Management System.

## Structure

```
api/
├── typespec/           # TypeSpec source files
│   ├── main.tsp       # Entry point
│   ├── domains/       # Domain-specific specs
│   └── shared/        # Shared models
├── generated/         # Generated OpenAPI specs
│   ├── openapi.yaml   # Full consolidated spec
│   └── by-service/    # Per-server specs
└── rules/             # Linting rules
    └── spectral.yaml
```

## Quick Start

### Generate OpenAPI from TypeSpec

```bash
cd docs/api/typespec
npx tsp compile .
```

### Lint OpenAPI Spec

```bash
npx spectral lint docs/api/generated/openapi.yaml
```

### Generate SDKs

```bash
# TypeScript
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

## Domain Specifications

| File | Use Cases | Description |
|------|-----------|-------------|
| `auth.tsp` | UC-14 | Authentication, registration, 2FA |
| `organizations.tsp` | UC-27 | Multi-tenancy, organizations |
| `buildings.tsp` | UC-15 | Buildings, floors, common areas |
| `units.tsp` | - | Properties, units, owners |
| `faults.tsp` | UC-03 | Fault reporting and tracking |
| `voting.tsp` | UC-04 | Voting, polls, assemblies |
| `documents.tsp` | UC-08 | Document management |
| `rentals.tsp` | UC-29-30 | Airbnb/Booking integration |
| `listings.tsp` | UC-31-32 | Real estate listings |
| `compliance.tsp` | UC-26 | GDPR, data export/deletion |

## Shared Models

| File | Description |
|------|-------------|
| `models.tsp` | Common data types (Address, Money, etc.) |
| `errors.tsp` | Standard error responses |
| `pagination.tsp` | Pagination patterns |
| `tenant.tsp` | Multi-tenant context |

## API Versioning

- URI-based: `/api/v1/`, `/api/v2/`
- Max 2 active versions
- 12-month deprecation window
