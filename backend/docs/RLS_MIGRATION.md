# RLS Migration Plan

## Current State

The backend uses a **repository pattern** where handlers access `state.*_repo` rather than `state.db` directly. This is good architecture, but RLS enforcement has a gap:

```
Handler → state.building_repo.create(...) → self.pool.acquire() → Query
                                              ↑
                                     No RLS context set!
```

### Available Infrastructure

1. **`RlsConnection` extractor** (`crates/api-core/src/extractors/rls_connection.rs`)
   - Sets RLS context on a dedicated connection
   - Has `release()` method to clear context before returning to pool
   - Warning on drop if not released

2. **`RlsPool` wrapper** (`crates/db/src/rls_pool.rs`)
   - Type-safe wrapper that enforces RLS on all connections
   - `acquire_with_rls(tenant_id, user_id, is_super_admin)` → `RlsGuard`
   - `acquire_public()` for unauthenticated routes
   - Guard clears context on release

3. **CI Check** (`scripts/check-rls-enforcement.sh`)
   - Detects direct pool access in handlers
   - Warns on repository patterns that may need RLS context

## Migration Strategy

### Option A: Pass Connection to Repositories (Recommended)

Change repository methods to accept a connection reference instead of using internal pool:

```rust
// Before
impl BuildingRepository {
    pub async fn create(&self, data: CreateBuilding) -> Result<Building> {
        sqlx::query_as("INSERT INTO buildings...")
            .fetch_one(&self.pool)  // Uses pool without RLS!
            .await
    }
}

// After
impl BuildingRepository {
    pub async fn create(&self, conn: &mut PgConnection, data: CreateBuilding) -> Result<Building> {
        sqlx::query_as("INSERT INTO buildings...")
            .fetch_one(conn)  // Uses RLS-enabled connection
            .await
    }
}

// Handler
async fn create_building(
    mut rls: RlsConnection,
    State(state): State<AppState>,
    Json(data): Json<CreateBuildingRequest>,
) -> Result<Json<Building>> {
    let building = state.building_repo.create(rls.conn(), data).await?;
    rls.release().await;
    Ok(Json(building))
}
```

### Option B: Replace Pool with RlsPool in AppState

Change `AppState.db: DbPool` to `AppState.db: RlsPool`:

```rust
pub struct AppState {
    pub db: RlsPool,  // Instead of DbPool
    // ...
}

// Repositories use RlsPool
impl BuildingRepository {
    pub async fn create(&self, tenant_id: Uuid, user_id: Uuid, data: CreateBuilding) -> Result<Building> {
        let mut guard = self.pool.acquire_with_rls(tenant_id, user_id, false).await?;
        let result = sqlx::query_as("INSERT INTO buildings...")
            .fetch_one(guard.conn())
            .await;
        guard.release().await;
        result
    }
}
```

## Priority Tiers

### Tier 1: HIGH RISK (Multi-Tenant Sensitive)

Must migrate first - these access organization-scoped data:

| Module | Files | Priority |
|--------|-------|----------|
| Organizations | `handlers/organizations.rs`, `repositories/organization.rs` | 1 |
| Buildings | `handlers/buildings/mod.rs`, `repositories/building.rs` | 1 |
| Units | `handlers/buildings/mod.rs`, `repositories/unit.rs` | 1 |
| Faults | `handlers/faults/mod.rs`, `repositories/fault.rs` | 2 |
| Voting | `handlers/voting/mod.rs`, `repositories/voting.rs` | 2 |
| Documents | `handlers/documents.rs`, `repositories/document.rs` | 2 |
| Financial | `handlers/budgets.rs`, `repositories/budget.rs` | 2 |
| Leases | `handlers/leases.rs`, `repositories/lease.rs` | 2 |

### Tier 2: MEDIUM RISK (User-Specific)

User preferences and settings:

| Module | Files |
|--------|-------|
| Notifications | `handlers/notifications.rs`, `repositories/notification.rs` |
| Subscriptions | `handlers/subscription.rs`, `repositories/subscription.rs` |
| MFA | `handlers/mfa.rs`, `repositories/two_factor_auth.rs` |

### Tier 3: LOW RISK (Public/System)

Public endpoints or system-wide data:

| Module | Files |
|--------|-------|
| Health | `handlers/health.rs` (no RLS needed) |
| Auth | `handlers/auth/mod.rs` (partial - login is public) |
| Public API | `handlers/public_api.rs` |

### Special Cases

**Background Jobs** - No request context available:
- `repositories/background_jobs.rs`
- Must pass explicit context or use system account

**Webhooks** - Inbound from external services:
- `handlers/portal_webhooks.rs`
- `handlers/voice_webhooks.rs`
- Context from webhook payload, not auth

**Reality Server** - Public portal:
- Most routes are public (listings search)
- User routes (favorites, saved searches) need RLS

## Migration Checklist

### Phase 1: Foundation

- [ ] Migrate `OrganizationRepository` to accept connection
- [ ] Migrate `BuildingRepository` to accept connection
- [ ] Migrate `UnitRepository` to accept connection
- [ ] Update handlers to use `RlsConnection` extractor
- [ ] Enable `--strict` mode in CI check

### Phase 2: Core Features

- [ ] Migrate `FaultRepository`
- [ ] Migrate `VotingRepository`
- [ ] Migrate `DocumentRepository`
- [ ] Migrate `BudgetRepository`
- [ ] Migrate `LeaseRepository`

### Phase 3: User Features

- [ ] Migrate `NotificationRepository`
- [ ] Migrate `SubscriptionRepository`
- [ ] Migrate `TwoFactorAuthRepository`

### Phase 4: Reality Portal

- [ ] Migrate `FavoritesRepository`
- [ ] Migrate `SavedSearchRepository`
- [ ] Migrate `InquiryRepository`

### Phase 5: Special Cases

- [ ] Refactor background job context injection
- [ ] Refactor webhook context injection
- [ ] Audit all remaining repositories

## Testing

Run RLS penetration tests after each phase:

```bash
# Run security tests
cargo test --test rls_penetration_tests -- --ignored --nocapture

# Run CI check in strict mode
./scripts/check-rls-enforcement.sh --strict
```

## Rollback

If issues arise:
1. Revert repository changes
2. Handlers continue to work (repos fall back to pool)
3. RLS policies still exist but may not be enforced
