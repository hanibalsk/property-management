# Property Management System (PPT)

## Namespace

**Package namespace:** `three.two.bit.ppt`

| Platform | Package/Bundle ID |
|----------|-------------------|
| Android (Reality Portal) | `bit.two.three.ppt.reality` |
| iOS (Reality Portal) | `bit.two.three.ppt.reality` |
| Android (Property Mgmt) | `bit.two.three.ppt.management` |
| iOS (Property Mgmt) | `bit.two.three.ppt.management` |

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        PROPERTY MANAGEMENT                          │
├─────────────────────────────────────────────────────────────────────┤
│  ppt-web (React SPA)     │  mobile (React Native)                  │
│  - Manager dashboard     │  - Android: bit.two.three.ppt.management│
│  - Building management   │  - iOS: bit.two.three.ppt.management    │
│  - Faults, Voting, etc   │                                         │
├─────────────────────────────────────────────────────────────────────┤
│                         api-server (Rust)                           │
│  Port 8080 │ UC-01 to UC-32 │ OAuth Provider                       │
└─────────────────────────────────────────────────────────────────────┘
                                  │
                           Shared Database
                                  │
┌─────────────────────────────────────────────────────────────────────┐
│                         reality-server (Rust)                       │
│  Port 8081 │ Public listings │ SSO Consumer                        │
├─────────────────────────────────────────────────────────────────────┤
│  reality-web (Next.js SSR)   │  mobile-native (KMP)                │
│  - Public listings           │  - Android: bit.two.three.ppt.reality│
│  - Search, filters           │  - iOS: bit.two.three.ppt.reality   │
│  - i18n (sk, cs, de, en)     │                                     │
├─────────────────────────────────────────────────────────────────────┤
│                          REALITY PORTAL                             │
└─────────────────────────────────────────────────────────────────────┘
```

## Platform Matrix

| Platform | App | Technology | Backend |
|----------|-----|------------|---------|
| Web | Property Management | React SPA (Vite) | api-server |
| Web | Reality Portal | Next.js (SSR + ISR) | reality-server |
| Mobile | Property Management | React Native | api-server |
| Mobile | Reality Portal | Kotlin Multiplatform | reality-server |

## Project Structure

See `docs/project-structure.md` for full directory tree.

```
property-management/
├── docs/                 # Documentation, API specs
├── backend/              # Rust: api-server, reality-server
├── frontend/             # TypeScript: ppt-web, reality-web, mobile
└── mobile-native/        # Kotlin: Reality Portal (Android/iOS)
```

## Git Conventions

### Branch Naming

```
feature/{EPIC-NNN}-{description}
bugfix/{STORY-NNN}-{description}
hotfix/{issue-description}
```

### Commit Messages

```
{type}({scope}): {description}
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

**Examples:**
- `feat(UC-14): implement user registration`
- `fix(api-server): correct tenant context extraction`
- `docs(reality-portal): add i18n documentation`

## Quick Start

```bash
# Backend
cd backend && cargo build

# Frontend
cd frontend && pnpm install && pnpm dev:ppt

# Reality Portal
cd frontend && pnpm dev:reality

# Mobile Native (Reality)
cd mobile-native && ./gradlew build
```

## Documentation Index

| File | Description |
|------|-------------|
| `docs/CLAUDE.md` | Use cases, PRD/Epic/Story conventions |
| `docs/spec1.0.md` | Original system specification |
| `docs/use-cases.md` | 407 use cases catalog |
| `docs/project-structure.md` | Full directory structure |
| `docs/api/README.md` | API specification index |
