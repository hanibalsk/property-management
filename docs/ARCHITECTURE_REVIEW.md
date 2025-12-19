# Architecture & Code Review (Deep)

This document is a **code+architecture review** of the `property-management` monorepo (PPT + Reality Portal). It focuses on **system boundaries, correctness/safety risks, contract alignment, and implementation maturity**.

## System at a glance

- **Products**
  - **Property Management (PPT)**: resident/owner/manager workflows (use-cases UC-01+)
  - **Reality Portal**: public listings + portal accounts (use-cases UC-44+)
- **Backends (Rust / Axum)**
  - `backend/servers/api-server` (Property Management API, `:8080`)
  - `backend/servers/reality-server` (Reality Portal API, `:8081`)
- **Frontends (TypeScript)**
  - `frontend/apps/ppt-web` (React + Vite SPA, dev `:3000`)
  - `frontend/apps/reality-web` (Next.js)
  - `frontend/apps/mobile` (React Native / Expo)
- **Mobile Native (KMP)**
  - `mobile-native/` (Kotlin Multiplatform for Reality Portal)

## Strengths (what’s already right)

- **Clear system boundaries**: two servers match two products, and route folders are domain-oriented (`auth`, `buildings`, `faults`, …).
- **Shared backend crates are the correct abstraction**:
  - `common`: shared types, errors, tenant context/roles
  - `api-core`: cross-cutting HTTP concerns (auth extractor, tenant middleware, OpenAPI helpers)
  - `db`: DB pool + (planned) repositories/models
  - `integrations`: external service clients
- **A use-case catalog exists** (`docs/use-cases.md`) and is already used as a naming/traceability convention (UC-xx referenced in server docs).
- **CI intent is strong**: API validation workflow compiles TypeSpec → lints OpenAPI → checks breaking changes.

## Key architectural risks / gaps (high priority)

### 1) Multi-tenancy exists on paper, but is not enforced in the servers

The repo has the right primitives:
- `common::TenantContext` + `TenantRole` (role hierarchy)
- `api-core` provides `tenant_filter` middleware and `TenantExtractor`

But neither `api-server` nor `reality-server` currently applies tenant middleware or auth middleware at the router level (no `layer(from_fn(...))` wiring).

**Impact**
- Tenant isolation is currently a **non-feature** at runtime.
- When DB work starts, it’s easy to accidentally ship cross-tenant data access.

**Recommendation**
- Introduce a shared `AppState` (at minimum: `DbPool`, config) and apply:
  - Auth extraction for protected routes
  - Tenant validation + enrichment for tenant-scoped routes
  - Role checks per endpoint (e.g., `Manager` vs `Owner`)

### 2) Authentication is scaffolded but unsafe by default

Findings:
- `api-server` auth endpoints return **placeholder tokens**.
- `api-core` auth extractor uses `JWT_SECRET` but falls back to `"dev-secret-key"` if missing.

**Impact**
- Easy to run “successfully” while actually being insecure.
- Hard to detect configuration mistakes in production.

**Recommendation**
- Fail fast if `JWT_SECRET` is missing in non-dev environments.
- Centralize auth config (issuer/audience/expiration, signing key rotation plan).
- Implement refresh token flow or remove from API until implemented (avoid “fake” security surfaces).

### 3) API contract pipeline is internally inconsistent (TypeSpec vs “by-service”)

What CI does:
- Compiles TypeSpec to `docs/api/generated/openapi.yaml` (see `docs/api/typespec/tspconfig.yaml`).

What multiple docs/scripts assume:
- `docs/api/generated/by-service/{api-server|reality-server}.yaml` exists and is the source for SDK generation.

Current reality:
- `tspconfig.yaml` only outputs the consolidated `openapi.yaml`.
- SDK packages (`frontend/packages/api-client`, `frontend/packages/reality-api-client`) and `mobile-native/CLAUDE.md` point at `docs/api/generated/by-service/*.yaml`, which currently will not be produced by `tsp compile`.

**Impact**
- Local SDK generation is confusing/broken unless manually fixed.
- Docs imply a “per-service spec” strategy that isn’t implemented.

**Recommendation (pick one path)**
- **Path A (simplest)**: Standardize everything on `docs/api/generated/openapi.yaml`.
  - Update package scripts + docs to generate from the consolidated spec.
- **Path B (cleaner long-term)**: Actually generate per-service specs.
  - Add a splitter step (or restructure TypeSpec into separate service entrypoints).
  - Ensure both TS SDK and KMP generation are aligned.

### 4) Backend implementation maturity is low vs the spec/use-case scope

Most `api-server` domain routers are TODO stubs (organizations/buildings/faults/voting/rentals/listings/integrations).
The `db` crate has a pool, but **models and repositories are placeholders**.

**Impact**
- High risk of “big bang” implementation later.
- Hard to validate architectural correctness (tenancy, auth, errors) without real endpoints.

**Recommendation**
- Implement a thin vertical slice per domain:
  - Spec → route → handler → repository → migration → integration tests
  - Start with priority domains already suggested in docs: UC-14 (Auth) + UC-27 (Org/Tenant) + UC-15 (Buildings)

## Code health / maintainability findings

### Error handling is defined but not integrated

`common::AppError` + `ErrorResponse` exist, but most routes return `StatusCode` directly.

**Recommendation**
- Implement `IntoResponse` for `AppError` (and a standard request-id extension).
- Ensure OpenAPI error responses match TypeSpec `shared/errors.tsp`.

### Duplicate / confusing frontend structure

There is a second nested tree at `frontend/frontend/` that looks like an older/duplicate monorepo snapshot.

**Recommendation**
- Remove it if unused, or document its purpose (otherwise it increases cognitive load and review risk).

### Root README is empty

`README.md` contains only the repo name.

**Recommendation**
- Add a real root README:
  - what PPT vs Reality are
  - how to run both servers + apps locally
  - how to compile TypeSpec and generate SDKs
  - where to start (priority UCs)

## Suggested prioritized roadmap (actionable)

- **P0 (Safety / correctness)**
  - Wire auth + tenant middleware; remove insecure defaults (`JWT_SECRET` fallback).
  - Standardize error handling and add request-id propagation.
  - Decide contract strategy (TypeSpec-only vs hybrid) and align scripts/docs accordingly.

- **P1 (Foundations for real work)**
  - Add `AppState` with DB pool + config; implement migrations.
  - Implement UC-14 + UC-27 + UC-15 vertical slices end-to-end (spec → DB → API → client).
  - Add integration tests and contract checks (OpenAPI-driven).

- **P2 (Scale & operability)**
  - Add observability conventions (structured logs, tracing fields: tenant_id/user_id/request_id).
  - Add rate limiting per tenant (mentioned in middleware comments).
  - Add background jobs / queues for integrations (Airbnb/Booking) as the domain grows.

## Appendix: “Where things live”

- **Use cases**: `docs/use-cases.md`
- **API specs (TypeSpec)**: `docs/api/typespec/`
- **Rust servers**: `backend/servers/{api-server,reality-server}/`
- **Shared Rust crates**: `backend/crates/{common,api-core,db,integrations}/`
- **TS SDKs**: `frontend/packages/{api-client,reality-api-client}/`
- **Apps**: `frontend/apps/{ppt-web,reality-web,mobile}/`


