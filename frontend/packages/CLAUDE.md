# Frontend Packages - CLAUDE.md

> **Parent:** See `frontend/CLAUDE.md` for monorepo overview.

## Shared Packages

| Package | Purpose | Used By |
|---------|---------|---------|
| @ppt/api-client | API client for api-server | ppt-web, mobile |
| @ppt/reality-api-client | API client for reality-server | reality-web |
| @ppt/shared | Shared utilities and types | All apps |
| @ppt/ui-kit | Shared UI components | ppt-web, reality-web |

## Package Details

### @ppt/api-client

Generated TypeScript client for Property Management API.

```bash
# Generate from OpenAPI
pnpm --filter @ppt/api-client generate
```

Source: `docs/api/generated/by-service/api-server.yaml`

### @ppt/reality-api-client

Generated TypeScript client for Reality Portal API.

```bash
# Generate from OpenAPI
pnpm --filter @ppt/reality-api-client generate
```

Source: `docs/api/generated/by-service/reality-server.yaml`

### @ppt/shared

Common utilities, hooks, and types:
- Utility functions
- Custom hooks
- Shared TypeScript types
- Constants

### @ppt/ui-kit

Shared React components:
- Design system components
- Form components
- Layout components
- Theme configuration
