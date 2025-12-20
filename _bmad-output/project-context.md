# Project Context for AI Agents

> **Purpose:** Critical rules and patterns AI agents MUST follow when implementing code in this project. Optimized for LLM context efficiency.

---

## Technology Stack

| Layer | Technology | Version |
|-------|------------|---------|
| Backend | Rust + Axum | 2021 edition, Axum 0.7+ |
| Database | PostgreSQL + SQLx | 16+, RLS enabled |
| Cache | Redis | 7.x |
| Frontend (PPT) | React + Vite | React 18, Vite 5+ |
| Frontend (Reality) | Next.js | 14+ (SSR/SSG) |
| Mobile (PM) | React Native + Expo | RN 0.73+, Expo 50+ |
| Mobile (Reality) | Kotlin Multiplatform | Kotlin 2.1+, Ktor 3.0+ |
| State | TanStack Query + Zustand | v5+ |

---

## Critical Implementation Rules

### Multi-Tenancy (NEVER SKIP)

- **Every database query MUST include tenant context**
- Tenant ID extracted from JWT via middleware
- PostgreSQL RLS policies enforce isolation
- API handlers receive `TenantContext` as parameter
- Cross-tenant data access is an architectural violation

```rust
// CORRECT
async fn get_faults(tenant: TenantContext) -> Result<...>

// WRONG - no tenant context
async fn get_faults() -> Result<...>
```

### Naming Conventions

| Context | Convention | Example |
|---------|------------|---------|
| Database tables | snake_case plural | `fault_reports` |
| Database columns | snake_case | `created_at`, `user_id` |
| JSON/API fields | camelCase | `createdAt`, `userId` |
| React components | PascalCase.tsx | `FaultCard.tsx` |
| Rust modules | snake_case | `fault_reports.rs` |
| Kotlin packages | lowercase dot-separated | `three.two.bit.ppt.reality` |

### API Response Format (ALWAYS USE)

```json
{
  "data": { ... },
  "meta": {
    "requestId": "uuid",
    "timestamp": "2025-12-20T10:30:00Z"
  }
}
```

Error format:
```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Human-readable message",
    "details": [...],
    "requestId": "uuid"
  }
}
```

### Date/Time Handling

- API: ISO 8601 UTC with `Z` suffix (`2025-12-20T10:30:00Z`)
- Database: TIMESTAMPTZ (native PostgreSQL)
- Display: Locale-aware formatting in frontend

### Code Organization

**TypeScript (co-located tests):**
```
src/features/faults/
├── components/
│   ├── FaultCard.tsx
│   └── FaultCard.test.tsx
├── hooks/
│   └── useFaults.ts
└── types.ts
```

**Rust (separate tests/ directory):**
```
crates/api-core/
├── src/
│   └── handlers/faults.rs
└── tests/
    └── integration/faults_test.rs
```

---

## Anti-Patterns to AVOID

| ❌ Don't | ✅ Do |
|----------|-------|
| `userId` in DB columns | `user_id` |
| `/api/v1/getFaults` | `/api/v1/faults` |
| `{ error: "failed" }` | Standard error format with code |
| `2025-12-20 10:30:00` | `2025-12-20T10:30:00Z` |
| Tests in Rust `src/` | Tests in `tests/` directory |
| `any` type in TypeScript | Proper typing |
| DB queries without tenant | Include TenantContext |

---

## Package Namespace

**Root namespace:** `three.two.bit.ppt`

| Platform | Package ID |
|----------|------------|
| Android (Reality) | `three.two.bit.ppt.reality` |
| iOS (Reality) | `three.two.bit.ppt.reality` |
| Android (PM) | `three.two.bit.ppt.management` |
| iOS (PM) | `three.two.bit.ppt.management` |

---

## Development Commands

```bash
# Backend
cd backend && cargo run -p api-server    # Port 8080
cd backend && cargo run -p reality-server # Port 8081
cargo test --workspace
cargo clippy --workspace -- -D warnings

# Frontend
cd frontend && pnpm dev:ppt              # Port 5173
cd frontend && pnpm dev:reality          # Port 3000
pnpm check:fix                           # Biome lint+format
pnpm typecheck

# Mobile Native
cd mobile-native
./gradlew spotlessApply                  # Format
./gradlew :shared:build
./gradlew :androidApp:assembleDebug
```

---

## Git Conventions

**Branch naming:** `feature/{EPIC-NNN}-{description}`, `bugfix/{STORY-NNN}-{description}`

**Commit format:** `{type}({scope}): {description}`

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Example: `feat(UC-14): implement user registration`

---

## Reference Documents

For detailed patterns, see:
- `_bmad-output/architecture.md` - Full architectural decisions and patterns
- `docs/CLAUDE.md` - Use cases and PRD conventions
- `CLAUDE.md` - Root project context

---

_Generated: 2025-12-20_
