# Backend Crates - CLAUDE.md

> **Parent:** See `backend/CLAUDE.md` for server overview.

## Shared Crates

| Crate | Purpose | Used By |
|-------|---------|---------|
| common | Core types, errors, TenantContext | All crates and servers |
| api-core | Middleware, extractors, OpenAPI utils | All servers |
| db | Database models and repositories | All servers |
| integrations | External API clients | api-server |

## Crate Details

### common
Core types shared across the entire backend:
- `TenantContext` - Multi-tenancy context
- `TenantRole` - Role enum (11 roles)
- Error types
- Utility types

### api-core
HTTP/API layer utilities:
- Axum extractors for tenant context
- Authentication middleware
- OpenAPI (utoipa) configuration
- CORS, tracing middleware

### db
Database access layer:
- SQLx connection pooling
- Entity models
- Repository pattern implementations
- Database migrations

### integrations
External service clients:
- Airbnb API client
- Booking.com API client
- Real estate portal clients
