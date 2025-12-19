# Project Structure

## Overview

Property Management System (PPT) + Reality Portal

**Namespace:** `three.two.bit.ppt`

## Directory Tree

```
property-management/
├── CLAUDE.md                         # Root: Namespace, architecture, conventions
│
├── docs/                             # Documentation
│   ├── CLAUDE.md                    # Docs context
│   ├── spec1.0.md                   # Original specification
│   ├── use-cases.md                 # 407 use cases
│   ├── project-structure.md         # This file
│   └── api/                         # API specifications
│       ├── README.md
│       ├── typespec/                # TypeSpec sources
│       │   ├── main.tsp
│       │   ├── domains/             # Domain-specific specs
│       │   └── shared/              # Shared models
│       ├── generated/               # Generated OpenAPI
│       │   ├── openapi.yaml         # Full spec
│       │   └── by-service/
│       │       ├── api-server.yaml
│       │       └── reality-server.yaml
│       └── rules/
│           └── spectral.yaml        # Linting rules
│
├── backend/                          # Rust Backend
│   ├── CLAUDE.md                    # Backend context
│   ├── Cargo.toml                   # Workspace
│   ├── crates/                      # Shared libraries
│   │   ├── CLAUDE.md
│   │   ├── common/                  # Core types, TenantContext
│   │   ├── api-core/                # Middleware, extractors
│   │   ├── db/                      # Database layer
│   │   └── integrations/            # External APIs
│   └── servers/                     # Backend servers
│       ├── CLAUDE.md
│       ├── api-server/              # Property Management API (Port 8080)
│       └── reality-server/          # Reality Portal API (Port 8081)
│
├── frontend/                         # TypeScript Frontend
│   ├── CLAUDE.md                    # Frontend context
│   ├── package.json                 # pnpm monorepo
│   ├── packages/                    # Shared packages
│   │   ├── CLAUDE.md
│   │   ├── api-client/              # @ppt/api-client (api-server SDK)
│   │   ├── reality-api-client/      # @ppt/reality-api-client (reality-server SDK)
│   │   ├── shared/                  # @ppt/shared (utilities)
│   │   └── ui-kit/                  # @ppt/ui-kit (components)
│   └── apps/                        # Applications
│       ├── CLAUDE.md
│       ├── ppt-web/                 # @ppt/web - React SPA (Property Mgmt)
│       ├── reality-web/             # @ppt/reality-web - Next.js (Reality Portal)
│       └── mobile/                  # @ppt/mobile - React Native (Property Mgmt)
│
├── mobile-native/                    # Kotlin Multiplatform (Reality Portal)
│   ├── CLAUDE.md                    # KMP context
│   ├── build.gradle.kts
│   ├── shared/                      # Common code
│   ├── androidApp/                  # three.two.bit.ppt.reality
│   └── iosApp/                      # three.two.bit.ppt.reality
│
└── .github/workflows/                # CI/CD
    ├── api-validation.yml
    ├── backend.yml
    ├── frontend.yml
    └── mobile-native.yml
```

## Backend Servers

| Server | Port | Purpose | Consumers |
|--------|------|---------|-----------|
| api-server | 8080 | Property Management API | ppt-web, mobile (RN) |
| reality-server | 8081 | Reality Portal API | reality-web, mobile-native (KMP) |

## Frontend Apps

| App | Package | Technology | Backend |
|-----|---------|------------|---------|
| ppt-web | @ppt/web | React + Vite (SPA) | api-server |
| reality-web | @ppt/reality-web | Next.js (SSR) | reality-server |
| mobile | @ppt/mobile | React Native | api-server |

## Mobile Native Apps

| App | Package ID | Technology | Backend |
|-----|------------|------------|---------|
| Reality Portal Android | three.two.bit.ppt.reality | Kotlin Multiplatform | reality-server |
| Reality Portal iOS | three.two.bit.ppt.reality | Kotlin Multiplatform | reality-server |

## Shared Code

| Location | Purpose |
|----------|---------|
| backend/crates/* | Rust shared libraries |
| frontend/packages/* | TypeScript shared packages |
| mobile-native/shared | KMP shared module |

## API SDK Generation

```bash
# Property Management API client
pnpm --filter @ppt/api-client generate

# Reality Portal API client
pnpm --filter @ppt/reality-api-client generate

# Kotlin (KMP)
openapi-generator generate \
  -i docs/api/generated/by-service/reality-server.yaml \
  -g kotlin \
  -o mobile-native/shared/src/commonMain/kotlin/api
```
