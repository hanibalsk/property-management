# Property Management System (PPT)

![Version](https://img.shields.io/badge/version-0.2.201-blue)
![Rust](https://img.shields.io/badge/rust-1.75+-orange)
![TypeScript](https://img.shields.io/badge/typescript-5.0+-blue)

A comprehensive property management platform with two products:

- **Property Management (PPT)**: Manager/resident/owner workflows for housing cooperatives
- **Reality Portal**: Public real estate listings platform

## Quick Start

### Prerequisites

- Rust 1.75+
- Node.js 20+
- pnpm 8+
- PostgreSQL 15+

### Backend

```bash
cd backend
cargo build
cargo run -p api-server      # Property Management API (:8080)
cargo run -p reality-server  # Reality Portal API (:8081)
```

### Frontend

```bash
cd frontend
pnpm install
pnpm dev:ppt      # Property Management web (:3000)
pnpm dev:reality  # Reality Portal web (:3001)
```

### Mobile Native (Reality Portal)

```bash
cd mobile-native

# Android
./gradlew :androidApp:assembleDebug

# iOS (requires Xcode)
cd iosApp && open iosApp.xcodeproj
```

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        PROPERTY MANAGEMENT                          │
├─────────────────────────────────────────────────────────────────────┤
│  ppt-web (React SPA)     │  mobile (React Native)                  │
├─────────────────────────────────────────────────────────────────────┤
│                         api-server (Rust)                           │
│                           Port 8080                                 │
└─────────────────────────────────────────────────────────────────────┘
                                  │
                           Shared Database
                                  │
┌─────────────────────────────────────────────────────────────────────┐
│                         reality-server (Rust)                       │
│                           Port 8081                                 │
├─────────────────────────────────────────────────────────────────────┤
│  reality-web (Next.js)     │  mobile-native (Kotlin)               │
├─────────────────────────────────────────────────────────────────────┤
│                          REALITY PORTAL                             │
└─────────────────────────────────────────────────────────────────────┘
```

## Project Structure

```
property-management/
├── backend/              # Rust workspace
│   ├── crates/           # Shared libraries
│   │   ├── common/       # Types, errors, tenant context
│   │   ├── api-core/     # HTTP middleware, extractors
│   │   ├── db/           # Database models, repositories
│   │   └── integrations/ # External API clients
│   └── servers/          # HTTP servers
│       ├── api-server/   # Property Management API
│       └── reality-server/ # Reality Portal API
├── frontend/             # TypeScript monorepo (pnpm)
│   ├── apps/             # Applications
│   │   ├── ppt-web/      # Property Management SPA
│   │   ├── reality-web/  # Reality Portal (Next.js)
│   │   └── mobile/       # React Native app
│   └── packages/         # Shared packages
│       ├── api-client/   # Generated TS SDK
│       ├── shared/       # Utilities
│       └── ui-kit/       # UI components
├── mobile-native/        # Kotlin Multiplatform
│   ├── shared/           # Shared KMP code
│   ├── androidApp/       # Android application
│   └── iosApp/           # iOS application (SwiftUI)
└── docs/                 # Documentation
    ├── use-cases.md      # 508 use cases catalog
    ├── api/              # API specifications
    │   └── typespec/     # TypeSpec definitions
    └── ARCHITECTURE_REVIEW.md
```

## API Development

### Compile TypeSpec to OpenAPI

```bash
cd docs/api/typespec
npx tsp compile .
```

### Generate TypeScript SDK

```bash
cd frontend
pnpm generate-api
```

## Versioning

Single source of truth: `VERSION` file

```bash
# Auto-bumps patch on every commit via pre-commit hook
# Manual bumps:
./scripts/bump-version.sh minor
./scripts/bump-version.sh major

# Install hooks
./scripts/install-hooks.sh
```

## Documentation

See **[docs/index.md](docs/index.md)** for the full documentation index.

| Document | Description |
|----------|-------------|
| [CLAUDE.md](CLAUDE.md) | Architecture, conventions, quick reference |
| [docs/use-cases.md](docs/use-cases.md) | 508 use cases across 51 categories |
| [docs/technical-design.md](docs/technical-design.md) | ~145 API endpoints, ~127 DTOs |

## Contributing

1. Create feature branch: `feature/epic-{N}-{description}`
2. Implement stories with commits: `feat(epic-N): story N.M - description`
3. Run code review: `/bmad:bmm:workflows:code-review`
4. Create PR via `gh pr create`

## License

Proprietary - All rights reserved
