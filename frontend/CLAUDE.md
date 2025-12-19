# Frontend - CLAUDE.md

> **Parent:** See root `CLAUDE.md` for namespace and architecture.

## Overview

TypeScript frontend monorepo using pnpm workspaces.

## Apps

| App | Package | Technology | Backend |
|-----|---------|------------|---------|
| ppt-web | @ppt/web | React + Vite (SPA) | api-server |
| reality-web | @ppt/reality-web | Next.js (SSR) | reality-server |
| mobile | @ppt/mobile | React Native (Expo) | api-server |

## Quick Commands

```bash
# Install dependencies
pnpm install

# Development
pnpm dev:ppt         # Property Management web
pnpm dev:reality     # Reality Portal web
pnpm dev:mobile      # React Native

# Build
pnpm build           # Build all

# Type check
pnpm typecheck       # Check all packages

# Lint
pnpm lint            # Lint all packages

# Generate API clients
pnpm generate-api           # @ppt/api-client
pnpm generate-reality-api   # @ppt/reality-api-client
```

## Workspace Structure

```
frontend/
├── package.json       # Monorepo root
├── packages/          # Shared packages (see packages/CLAUDE.md)
│   ├── api-client/
│   ├── reality-api-client/
│   ├── shared/
│   └── ui-kit/
└── apps/              # Applications (see apps/CLAUDE.md)
    ├── ppt-web/
    ├── reality-web/
    └── mobile/
```

## Package Naming

All packages use `@ppt/` scope:
- `@ppt/web` - Property Management web
- `@ppt/reality-web` - Reality Portal web
- `@ppt/mobile` - Property Management mobile
- `@ppt/api-client` - API client for api-server
- `@ppt/reality-api-client` - API client for reality-server
- `@ppt/shared` - Shared utilities
- `@ppt/ui-kit` - UI components

## Technology Stack

- **React 18** - UI library
- **TypeScript 5** - Type safety
- **TanStack Query** - Server state management
- **Vite** - Build tool (ppt-web)
- **Next.js 14** - Framework (reality-web)
- **Expo** - React Native framework
