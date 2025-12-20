# Testability, Implementation Plan & Traceability

This document defines the testing strategy, implementation roadmap, and traceability framework for the Property Management System (PPT) and Reality Portal.

## Table of Contents

1. [Testability Strategy](#testability-strategy)
2. [Test Case Mapping](#test-case-mapping)
3. [Testing Pyramid](#testing-pyramid)
4. [Implementation Plan](#implementation-plan)
5. [Iteration Breakdown](#iteration-breakdown)
6. [Traceability Matrix](#traceability-matrix)
7. [Definition of Done](#definition-of-done)

---

## Testability Strategy

### Testing Principles

1. **Use Case Driven** - Every test maps to a specific use case
2. **Shift Left** - Catch issues early with unit and integration tests
3. **Automation First** - Automate all repeatable tests
4. **Risk-Based** - Higher test coverage for critical paths
5. **Continuous** - Tests run on every commit (CI/CD)

### Test Environments

| Environment | Purpose | Data | URL Pattern |
|-------------|---------|------|-------------|
| Local | Developer testing | Fixtures | `localhost:*` |
| CI | Automated tests | Fixtures + generated | N/A |
| Dev | Integration testing | Seeded | `dev.ppt.sk` |
| Staging | Pre-production | Anonymized prod copy | `staging.ppt.sk` |
| Production | Live system | Real | `app.ppt.sk`, `reality.ppt.sk` |

### Test Data Strategy

```yaml
test_data:
  fixtures:
    location: tests/fixtures/
    format: YAML + SQL
    types:
      - users (20+ personas)
      - organizations (5 types)
      - buildings (10 variations)
      - units (50+ with various states)

  factories:
    location: tests/factories/
    framework: Factory pattern (Rust/TS)
    traits:
      - with_owner
      - with_tenant
      - with_faults
      - with_votes
      - overdue

  seeds:
    dev: Realistic demo data (1000 units)
    staging: Anonymized prod snapshot
    load_test: Generated (100K units)
```

---

## Test Case Mapping

### Test Case Naming Convention

```
TC-{UC_ID}-{SCENARIO}-{TYPE}

Examples:
- TC-03.1-001-UNIT     (Fault Report - Happy path - Unit test)
- TC-03.1-002-INT      (Fault Report - With photo - Integration)
- TC-03.1-003-E2E      (Fault Report - Full flow - E2E)
- TC-03.1-ERR-001      (Fault Report - Invalid input - Error case)
```

### Test Cases by Use Case Category

#### UC-01: Notifications (6 use cases → 24 tests)

| Use Case | Test ID | Scenario | Type | Priority |
|----------|---------|----------|------|----------|
| UC-01.1 | TC-01.1-001-UNIT | Enable push - valid token | Unit | High |
| UC-01.1 | TC-01.1-002-INT | Enable push - FCM integration | Integration | High |
| UC-01.1 | TC-01.1-ERR-001 | Enable push - invalid token | Unit | Medium |
| UC-01.2 | TC-01.2-001-UNIT | Disable push - success | Unit | High |
| UC-01.3 | TC-01.3-001-UNIT | Configure preferences - all channels | Unit | High |
| UC-01.3 | TC-01.3-002-INT | Configure preferences - persistence | Integration | Medium |
| UC-01.4 | TC-01.4-001-UNIT | View history - paginated | Unit | Medium |
| UC-01.4 | TC-01.4-002-E2E | View history - filter by type | E2E | Low |
| UC-01.5 | TC-01.5-001-UNIT | Mark single as read | Unit | Medium |
| UC-01.6 | TC-01.6-001-UNIT | Mark all as read | Unit | Medium |

#### UC-02: Announcements (13 use cases → 52 tests)

| Use Case | Test ID | Scenario | Type | Priority |
|----------|---------|----------|------|----------|
| UC-02.1 | TC-02.1-001-UNIT | Create - minimal fields | Unit | High |
| UC-02.1 | TC-02.1-002-UNIT | Create - with attachments | Unit | High |
| UC-02.1 | TC-02.1-003-INT | Create - file upload | Integration | High |
| UC-02.1 | TC-02.1-ERR-001 | Create - title too long | Unit | Medium |
| UC-02.2 | TC-02.2-001-UNIT | Create meeting - with datetime | Unit | High |
| UC-02.3 | TC-02.3-001-UNIT | Search - full text | Unit | Medium |
| UC-02.3 | TC-02.3-002-INT | Search - PostgreSQL FTS | Integration | Medium |
| UC-02.4 | TC-02.4-001-UNIT | Filter by status | Unit | Medium |
| UC-02.5 | TC-02.5-001-UNIT | Filter own | Unit | Low |
| UC-02.6 | TC-02.6-001-E2E | View detail - full content | E2E | High |
| UC-02.7 | TC-02.7-001-UNIT | Add comment | Unit | Medium |
| UC-02.7 | TC-02.7-002-INT | Comment notification | Integration | Medium |
| UC-02.8 | TC-02.8-001-UNIT | View comments - paginated | Unit | Low |
| UC-02.9 | TC-02.9-001-UNIT | Edit - update text | Unit | Medium |
| UC-02.9 | TC-02.9-ERR-001 | Edit - not owner | Unit | High |
| UC-02.10 | TC-02.10-001-UNIT | Delete - soft delete | Unit | Medium |
| UC-02.11 | TC-02.11-001-UNIT | Archive | Unit | Low |
| UC-02.12 | TC-02.12-001-UNIT | Pin to top | Unit | Low |
| UC-02.13 | TC-02.13-001-UNIT | Schedule publication | Unit | Medium |
| UC-02.13 | TC-02.13-002-INT | Scheduled job execution | Integration | High |

#### UC-03: Faults (14 use cases → 70 tests)

| Use Case | Test ID | Scenario | Type | Priority |
|----------|---------|----------|------|----------|
| UC-03.1 | TC-03.1-001-UNIT | Report - minimal | Unit | Critical |
| UC-03.1 | TC-03.1-002-UNIT | Report - with photo | Unit | Critical |
| UC-03.1 | TC-03.1-003-INT | Report - S3 upload | Integration | High |
| UC-03.1 | TC-03.1-004-E2E | Report - full flow | E2E | Critical |
| UC-03.1 | TC-03.1-ERR-001 | Report - missing title | Unit | High |
| UC-03.1 | TC-03.1-ERR-002 | Report - invalid building | Unit | High |
| UC-03.2 | TC-03.2-001-UNIT | Search faults | Unit | Medium |
| UC-03.3 | TC-03.3-001-UNIT | Filter by status | Unit | Medium |
| UC-03.4 | TC-03.4-001-UNIT | Filter own faults | Unit | Medium |
| UC-03.5 | TC-03.5-001-E2E | View detail | E2E | High |
| UC-03.6 | TC-03.6-001-UNIT | Update status - in progress | Unit | Critical |
| UC-03.6 | TC-03.6-002-UNIT | Update status - resolved | Unit | Critical |
| UC-03.6 | TC-03.6-003-INT | Status change notification | Integration | High |
| UC-03.6 | TC-03.6-ERR-001 | Invalid transition | Unit | High |
| UC-03.7 | TC-03.7-001-UNIT | Add communication | Unit | High |
| UC-03.8 | TC-03.8-001-UNIT | Assign to tech manager | Unit | High |
| UC-03.8 | TC-03.8-002-INT | Assignment notification | Integration | Medium |
| UC-03.9 | TC-03.9-001-UNIT | Set priority | Unit | High |
| UC-03.10 | TC-03.10-001-UNIT | Close fault | Unit | Critical |
| UC-03.11 | TC-03.11-001-UNIT | Reopen fault | Unit | High |
| UC-03.12 | TC-03.12-001-UNIT | Escalate fault | Unit | Medium |
| UC-03.13 | TC-03.13-001-UNIT | Add photo to existing | Unit | Medium |
| UC-03.14 | TC-03.14-001-UNIT | Request update | Unit | Low |

#### UC-04: Voting (12 use cases → 60 tests)

| Use Case | Test ID | Scenario | Type | Priority |
|----------|---------|----------|------|----------|
| UC-04.1 | TC-04.1-001-UNIT | Create vote - single choice | Unit | Critical |
| UC-04.1 | TC-04.1-002-UNIT | Create vote - multiple choice | Unit | Critical |
| UC-04.1 | TC-04.1-003-INT | Vote state machine | Integration | Critical |
| UC-04.2 | TC-04.2-001-UNIT | Cast ballot - valid | Unit | Critical |
| UC-04.2 | TC-04.2-002-UNIT | Cast ballot - update | Unit | High |
| UC-04.2 | TC-04.2-ERR-001 | Cast ballot - expired | Unit | High |
| UC-04.2 | TC-04.2-ERR-002 | Cast ballot - not eligible | Unit | High |
| UC-04.3 | TC-04.3-001-UNIT | View results - owner | Unit | High |
| UC-04.3 | TC-04.3-002-E2E | View results - quorum check | E2E | High |
| UC-04.4 | TC-04.4-001-UNIT | View active votes | Unit | Medium |
| UC-04.5 | TC-04.5-001-UNIT | View completed votes | Unit | Medium |
| UC-04.6 | TC-04.6-001-UNIT | Comment on vote | Unit | Low |
| UC-04.7 | TC-04.7-001-UNIT | Grant proxy | Unit | High |
| UC-04.7 | TC-04.7-002-INT | Proxy delegation chain | Integration | High |
| UC-04.8 | TC-04.8-001-UNIT | Vote with proxy | Unit | High |
| UC-04.9 | TC-04.9-001-UNIT | Close vote | Unit | High |
| UC-04.9 | TC-04.9-002-INT | Auto-close scheduled | Integration | High |
| UC-04.10 | TC-04.10-001-UNIT | Cancel vote | Unit | Medium |
| UC-04.11 | TC-04.11-001-UNIT | Extend deadline | Unit | Medium |
| UC-04.12 | TC-04.12-001-UNIT | Export results | Unit | Low |

#### UC-14: User Account Management (10 use cases → 80 tests)

| Use Case | Test ID | Scenario | Type | Priority |
|----------|---------|----------|------|----------|
| UC-14.1 | TC-14.1-001-UNIT | Register - email/password | Unit | Critical |
| UC-14.1 | TC-14.1-002-INT | Register - email verification | Integration | Critical |
| UC-14.1 | TC-14.1-003-E2E | Register - full flow | E2E | Critical |
| UC-14.1 | TC-14.1-ERR-001 | Register - duplicate email | Unit | Critical |
| UC-14.1 | TC-14.1-ERR-002 | Register - weak password | Unit | High |
| UC-14.2 | TC-14.2-001-UNIT | Login - valid credentials | Unit | Critical |
| UC-14.2 | TC-14.2-002-UNIT | Login - with MFA | Unit | Critical |
| UC-14.2 | TC-14.2-003-INT | Login - JWT generation | Integration | Critical |
| UC-14.2 | TC-14.2-ERR-001 | Login - invalid password | Unit | Critical |
| UC-14.2 | TC-14.2-ERR-002 | Login - account locked | Unit | High |
| UC-14.3 | TC-14.3-001-UNIT | Logout - invalidate session | Unit | High |
| UC-14.3 | TC-14.3-002-INT | Logout - clear tokens | Integration | High |
| UC-14.4 | TC-14.4-001-UNIT | Password reset - request | Unit | Critical |
| UC-14.4 | TC-14.4-002-INT | Password reset - email sent | Integration | Critical |
| UC-14.4 | TC-14.4-003-UNIT | Password reset - token valid | Unit | Critical |
| UC-14.4 | TC-14.4-ERR-001 | Password reset - expired token | Unit | High |
| UC-14.5 | TC-14.5-001-UNIT | Update profile | Unit | Medium |
| UC-14.6 | TC-14.6-001-UNIT | Change password | Unit | High |
| UC-14.7 | TC-14.7-001-UNIT | Enable MFA - TOTP | Unit | High |
| UC-14.7 | TC-14.7-002-INT | Enable MFA - QR code | Integration | High |
| UC-14.8 | TC-14.8-001-UNIT | OAuth login - Google | Unit | High |
| UC-14.8 | TC-14.8-002-INT | OAuth login - token exchange | Integration | High |
| UC-14.9 | TC-14.9-001-UNIT | Delete account | Unit | Medium |
| UC-14.9 | TC-14.9-002-INT | Delete account - GDPR cascade | Integration | High |
| UC-14.10 | TC-14.10-001-UNIT | Refresh token | Unit | Critical |

#### UC-27: Multi-tenancy & Organizations (8 use cases → 48 tests)

| Use Case | Test ID | Scenario | Type | Priority |
|----------|---------|----------|------|----------|
| UC-27.1 | TC-27.1-001-UNIT | Create organization | Unit | Critical |
| UC-27.1 | TC-27.1-002-INT | Organization + subscription | Integration | Critical |
| UC-27.2 | TC-27.2-001-UNIT | Invite member | Unit | High |
| UC-27.2 | TC-27.2-002-INT | Invite email sent | Integration | High |
| UC-27.3 | TC-27.3-001-UNIT | Accept invitation | Unit | High |
| UC-27.4 | TC-27.4-001-UNIT | Assign role | Unit | Critical |
| UC-27.4 | TC-27.4-ERR-001 | Assign role - insufficient perms | Unit | Critical |
| UC-27.5 | TC-27.5-001-UNIT | Configure branding | Unit | Medium |
| UC-27.6 | TC-27.6-001-UNIT | Update settings | Unit | Medium |
| UC-27.7 | TC-27.7-001-INT | RLS isolation | Integration | Critical |
| UC-27.7 | TC-27.7-002-E2E | Cross-org access denied | E2E | Critical |
| UC-27.8 | TC-27.8-001-UNIT | Switch organization | Unit | High |

### Test Coverage Summary by Priority

| Priority | Use Cases | Test Cases | Target Coverage |
|----------|-----------|------------|-----------------|
| Critical | 45 | 180 | 100% |
| High | 120 | 360 | 95% |
| Medium | 180 | 360 | 80% |
| Low | 148 | 200 | 60% |
| **Total** | **493** | **~1,100** | **85% avg** |

---

## Testing Pyramid

```
                        ┌───────────────┐
                        │     E2E       │  5%
                        │   (~55 tests) │  (Critical flows)
                        ├───────────────┤
                        │               │
                    ┌───┤  Integration  ├───┐  20%
                    │   │  (~220 tests) │   │  (Component boundaries)
                    │   │               │   │
                ┌───┴───┴───────────────┴───┴───┐
                │                               │
                │         Unit Tests            │  75%
                │        (~825 tests)           │  (Business logic)
                │                               │
                └───────────────────────────────┘
```

### Unit Tests

**Scope:** Individual functions, modules, domain logic

**Framework:**
- Backend (Rust): `cargo test`, `mockall`
- Frontend (TypeScript): `vitest`, `@testing-library/react`
- Mobile Native (Kotlin): `kotlin.test`, `MockK`

**Examples:**
```rust
// Backend unit test example
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_fault_valid_input() {
        let input = CreateFaultInput {
            building_id: Uuid::new_v4(),
            title: "Broken elevator".to_string(),
            description: "Elevator not working on 3rd floor".to_string(),
            category: FaultCategory::Elevator,
            priority: None,
        };

        let result = validate_create_fault(&input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_fault_title_too_short() {
        let input = CreateFaultInput {
            title: "Hi".to_string(), // Too short
            ..Default::default()
        };

        let result = validate_create_fault(&input);
        assert!(matches!(result, Err(ValidationError::TitleTooShort)));
    }

    #[test]
    fn test_fault_state_transition_valid() {
        let fault = Fault::new(FaultStatus::New);
        let result = fault.transition(FaultEvent::Assign);
        assert_eq!(result.status, FaultStatus::InProgress);
    }

    #[test]
    fn test_fault_state_transition_invalid() {
        let fault = Fault::new(FaultStatus::Closed);
        let result = fault.can_transition(FaultEvent::Assign);
        assert!(!result);
    }
}
```

```typescript
// Frontend unit test example
import { render, screen, fireEvent } from '@testing-library/react';
import { FaultForm } from './FaultForm';

describe('FaultForm', () => {
  it('submits valid fault report', async () => {
    const onSubmit = vi.fn();
    render(<FaultForm onSubmit={onSubmit} buildingId="123" />);

    fireEvent.change(screen.getByLabelText('Title'), {
      target: { value: 'Broken elevator' },
    });
    fireEvent.change(screen.getByLabelText('Description'), {
      target: { value: 'Elevator not working on 3rd floor' },
    });
    fireEvent.click(screen.getByRole('button', { name: /submit/i }));

    expect(onSubmit).toHaveBeenCalledWith({
      title: 'Broken elevator',
      description: 'Elevator not working on 3rd floor',
    });
  });

  it('shows validation error for short title', async () => {
    render(<FaultForm onSubmit={vi.fn()} buildingId="123" />);

    fireEvent.change(screen.getByLabelText('Title'), {
      target: { value: 'Hi' },
    });
    fireEvent.click(screen.getByRole('button', { name: /submit/i }));

    expect(screen.getByText('Title must be at least 5 characters')).toBeInTheDocument();
  });
});
```

### Integration Tests

**Scope:** API endpoints, database operations, external services

**Framework:**
- Backend: `cargo test` with test database, `testcontainers`
- API: `httptest`, `reqwest`
- Database: SQLx test transactions

**Examples:**
```rust
// Backend integration test
#[tokio::test]
async fn test_create_fault_api() {
    let app = TestApp::spawn().await;
    let user = app.create_user("owner").await;
    let building = app.create_building(&user.org_id).await;

    let response = app
        .client
        .post("/api/v1/faults")
        .bearer_auth(&user.token)
        .json(&json!({
            "building_id": building.id,
            "title": "Broken elevator",
            "description": "Not working on 3rd floor",
            "category": "elevator"
        }))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), StatusCode::CREATED);

    let fault: FaultResponse = response.json().await.unwrap();
    assert_eq!(fault.title, "Broken elevator");
    assert_eq!(fault.status, "new");

    // Verify in database
    let db_fault = sqlx::query_as!(
        Fault,
        "SELECT * FROM property.faults WHERE id = $1",
        fault.id
    )
    .fetch_one(&app.db_pool)
    .await
    .unwrap();

    assert_eq!(db_fault.title, "Broken elevator");
}

#[tokio::test]
async fn test_rls_prevents_cross_org_access() {
    let app = TestApp::spawn().await;
    let user1 = app.create_user_in_org("org1").await;
    let user2 = app.create_user_in_org("org2").await;
    let building = app.create_building(&user1.org_id).await;

    // User from different org should not see the building
    let response = app
        .client
        .get(&format!("/api/v1/buildings/{}", building.id))
        .bearer_auth(&user2.token)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
```

### End-to-End Tests

**Scope:** Full user journeys across frontend and backend

**Framework:**
- Web: `Playwright`
- Mobile: `Detox` (React Native), `Maestro` (KMP)
- API: `Playwright` API testing

**Examples:**
```typescript
// E2E test with Playwright
import { test, expect } from '@playwright/test';

test.describe('Fault Reporting Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.fill('[name="email"]', 'owner@test.com');
    await page.fill('[name="password"]', 'TestPassword123!');
    await page.click('button[type="submit"]');
    await page.waitForURL('/dashboard');
  });

  test('owner can report a fault and receive status updates', async ({ page }) => {
    // Navigate to faults
    await page.click('[data-testid="nav-faults"]');
    await expect(page).toHaveURL('/faults');

    // Create new fault
    await page.click('[data-testid="btn-report-fault"]');
    await page.fill('[name="title"]', 'Broken elevator');
    await page.fill('[name="description"]', 'Elevator not working on 3rd floor');
    await page.selectOption('[name="category"]', 'elevator');

    // Upload photo
    await page.setInputFiles('[name="photos"]', 'tests/fixtures/elevator.jpg');

    await page.click('button[type="submit"]');

    // Verify success
    await expect(page.locator('[data-testid="toast-success"]')).toBeVisible();
    await expect(page).toHaveURL(/\/faults\/[\w-]+/);

    // Verify fault details
    await expect(page.locator('h1')).toHaveText('Broken elevator');
    await expect(page.locator('[data-testid="status-badge"]')).toHaveText('New');
  });

  test('manager can update fault status', async ({ page, context }) => {
    // Login as manager in new context
    const managerPage = await context.newPage();
    await managerPage.goto('/login');
    await managerPage.fill('[name="email"]', 'manager@test.com');
    await managerPage.fill('[name="password"]', 'TestPassword123!');
    await managerPage.click('button[type="submit"]');

    // Navigate to fault
    await managerPage.goto('/faults/test-fault-id');

    // Update status
    await managerPage.click('[data-testid="btn-update-status"]');
    await managerPage.selectOption('[name="status"]', 'in_progress');
    await managerPage.fill('[name="note"]', 'Technician assigned');
    await managerPage.click('[data-testid="btn-confirm-status"]');

    // Verify status updated
    await expect(managerPage.locator('[data-testid="status-badge"]')).toHaveText('In Progress');

    // Verify owner receives notification (check original page)
    await page.reload();
    await expect(page.locator('[data-testid="notification-badge"]')).toHaveText('1');
  });
});
```

### Performance Tests

**Scope:** Load testing, stress testing, soak testing

**Framework:** `k6`, `Grafana`

```javascript
// k6 load test script
import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  stages: [
    { duration: '1m', target: 100 },   // Ramp up
    { duration: '5m', target: 100 },   // Steady state
    { duration: '1m', target: 500 },   // Spike
    { duration: '5m', target: 500 },   // Hold spike
    { duration: '2m', target: 0 },     // Ramp down
  ],
  thresholds: {
    http_req_duration: ['p(95)<200'],  // 95% of requests under 200ms
    http_req_failed: ['rate<0.01'],    // Less than 1% failures
  },
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:8080';

export function setup() {
  // Login and get token
  const res = http.post(`${BASE_URL}/api/v1/auth/login`, JSON.stringify({
    email: 'loadtest@example.com',
    password: 'LoadTestPassword123!',
  }), { headers: { 'Content-Type': 'application/json' } });

  return { token: res.json('access_token') };
}

export default function (data) {
  const headers = {
    Authorization: `Bearer ${data.token}`,
    'Content-Type': 'application/json',
  };

  // List faults
  const listRes = http.get(`${BASE_URL}/api/v1/faults?limit=20`, { headers });
  check(listRes, {
    'list faults status is 200': (r) => r.status === 200,
    'list faults response time OK': (r) => r.timings.duration < 200,
  });

  sleep(1);

  // Create fault
  const createRes = http.post(`${BASE_URL}/api/v1/faults`, JSON.stringify({
    building_id: 'test-building-id',
    title: `Load test fault ${Date.now()}`,
    description: 'Performance test fault',
    category: 'other',
  }), { headers });

  check(createRes, {
    'create fault status is 201': (r) => r.status === 201,
    'create fault response time OK': (r) => r.timings.duration < 500,
  });

  sleep(1);
}
```

### Security Tests

**Scope:** OWASP Top 10, authentication, authorization

**Framework:** `OWASP ZAP`, custom scripts

```yaml
# Security test checklist
security_tests:
  authentication:
    - brute_force_protection
    - session_fixation
    - jwt_validation
    - mfa_bypass_attempts

  authorization:
    - idor_testing  # Insecure Direct Object Reference
    - privilege_escalation
    - cross_org_access
    - role_bypass

  injection:
    - sql_injection
    - xss_stored
    - xss_reflected
    - command_injection

  data_exposure:
    - sensitive_data_in_logs
    - sensitive_data_in_errors
    - pii_in_urls
    - api_key_exposure
```

---

## Implementation Plan

### MVP Definition

The Minimum Viable Product (MVP) focuses on core property management functionality.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              MVP SCOPE                                   │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  INCLUDED (MVP)                          EXCLUDED (Post-MVP)             │
│  ────────────────                        ──────────────────              │
│                                                                          │
│  ✓ UC-14: User Accounts (core)           ✗ UC-20: AI/ML Features         │
│  ✓ UC-27: Organizations                  ✗ UC-21: IoT & Smart Building   │
│  ✓ UC-15: Buildings & Units              ✗ UC-22: External Integrations  │
│  ✓ UC-01: Notifications                  ✗ UC-29: Short-term Rentals     │
│  ✓ UC-02: Announcements                  ✗ UC-30: Guest Registration     │
│  ✓ UC-03: Faults                         ✗ UC-31-32: Real Estate Listings│
│  ✓ UC-04: Voting                         ✗ UC-33: Tenant Screening       │
│  ✓ UC-05: Messages                       ✗ UC-34: Lease Management       │
│  ✓ UC-08: Documents (basic)              ✗ UC-35: Insurance              │
│  ✓ UC-16: Financial (basic)              ✗ UC-36: Maintenance Scheduling │
│                                          ✗ UC-37-40: Suppliers, Legal,   │
│  ~180 use cases                            Emergency, Budget             │
│  ~40% of total                           ✗ UC-44-51: Reality Portal      │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### Use Case Prioritization

| Priority | Categories | Use Cases | Rationale |
|----------|------------|-----------|-----------|
| **P0 - Foundation** | UC-14, UC-27 | 18 | Auth, multi-tenancy required for everything |
| **P1 - Core MVP** | UC-15, UC-01-05, UC-08 | 85 | Core property management features |
| **P2 - MVP Complete** | UC-16, UC-17, UC-06-07, UC-28 | 65 | Financial, reports, delegation |
| **P3 - Enhanced** | UC-23, UC-26, UC-41-42 | 50 | Security, automation, onboarding |
| **P4 - Rentals** | UC-29-30, UC-33-34 | 60 | Short-term rentals, leases |
| **P5 - Real Estate** | UC-31-32, UC-44-51 | 80 | Reality Portal features |
| **P6 - Advanced** | UC-20-22, UC-35-40 | 135 | AI/ML, IoT, operations |

### Dependency Graph

```
                    ┌─────────────────┐
                    │  UC-14: Auth    │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │ UC-27: Orgs     │
                    └────────┬────────┘
                             │
            ┌────────────────┼────────────────┐
            │                │                │
    ┌───────▼───────┐ ┌──────▼──────┐ ┌───────▼───────┐
    │UC-15: Buildings│ │UC-28: Perms │ │UC-41: Billing │
    └───────┬───────┘ └──────┬──────┘ └───────────────┘
            │                │
    ┌───────┴───────┬────────┴────────┬───────────────┐
    │               │                 │               │
┌───▼───┐     ┌─────▼─────┐     ┌─────▼─────┐   ┌─────▼─────┐
│UC-01  │     │  UC-02    │     │  UC-03    │   │  UC-04    │
│Notif. │     │Announce.  │     │  Faults   │   │  Voting   │
└───────┘     └───────────┘     └─────┬─────┘   └───────────┘
                                      │
                              ┌───────▼───────┐
                              │  UC-36: Maint │
                              │  Scheduling   │
                              └───────────────┘
```

---

## Iteration Breakdown

### Release Timeline

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         RELEASE ROADMAP                                  │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Alpha (Internal)     Beta (Pilot)      GA 1.0           GA 2.0         │
│  ────────────────     ────────────      ──────           ──────         │
│                                                                          │
│  ├─ Iteration 1-4 ─┤ ├─ Iter 5-8 ─┤ ├─ Iter 9-12 ─┤ ├─ Iter 13+ ──────┤ │
│                                                                          │
│  Foundation         Core MVP         Enhanced         Advanced           │
│  + Core features    + Financial      + Rentals        + AI/ML           │
│                     + Reports        + Real Estate    + IoT             │
│                                      + Portal                            │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### Iteration 1: Foundation (Sprint 1-2)

**Goal:** Authentication, multi-tenancy, project setup

| Use Case | Stories | Effort |
|----------|---------|--------|
| UC-14.1-3 | Register, Login, Logout | L |
| UC-14.4 | Password reset | M |
| UC-14.5-6 | Profile, Change password | S |
| UC-27.1-2 | Create org, Invite member | L |
| UC-27.3-4 | Accept invite, Assign role | M |
| Infrastructure | CI/CD, Dev environment | L |

**Deliverables:**
- [ ] User registration and login
- [ ] Organization creation
- [ ] Role-based access control
- [ ] CI/CD pipeline
- [ ] Dev/staging environments

### Iteration 2: Buildings & Notifications (Sprint 3-4)

**Goal:** Building management, notifications infrastructure

| Use Case | Stories | Effort |
|----------|---------|--------|
| UC-15.1-5 | Building CRUD | L |
| UC-15.6-10 | Unit CRUD | L |
| UC-01.1-6 | Notifications | M |
| Database | RLS policies | M |

**Deliverables:**
- [ ] Building and unit management
- [ ] Push notification infrastructure
- [ ] Row-level security
- [ ] Mobile app scaffolding

### Iteration 3: Announcements & Faults (Sprint 5-6)

**Goal:** Core communication features

| Use Case | Stories | Effort |
|----------|---------|--------|
| UC-02.1-13 | Announcements | L |
| UC-03.1-7 | Faults (basic) | L |
| UC-03.8-14 | Faults (workflow) | L |

**Deliverables:**
- [ ] Announcement CRUD with scheduling
- [ ] Fault reporting with photos
- [ ] Fault workflow (assign, resolve, close)
- [ ] State machine implementation

### Iteration 4: Voting & Messages (Sprint 7-8)

**Goal:** Democratic processes, communication

| Use Case | Stories | Effort |
|----------|---------|--------|
| UC-04.1-12 | Voting system | XL |
| UC-05.1-11 | Messaging | L |
| UC-28.1-8 | Delegation | M |

**Deliverables:**
- [ ] Vote creation and ballot casting
- [ ] Proxy delegation
- [ ] Real-time messaging (WebSocket)
- [ ] Permission delegation

### Iteration 5: Documents & Alpha Release (Sprint 9-10)

**Goal:** Document management, internal testing

| Use Case | Stories | Effort |
|----------|---------|--------|
| UC-08.1-14 | Documents | L |
| UC-42.1-8 | Onboarding | M |
| Quality | Bug fixes, polish | L |

**Deliverables:**
- [ ] Document upload with folders
- [ ] Document sharing
- [ ] Onboarding flows
- [ ] **Alpha release** (internal)

### Iteration 6: Financial Basics (Sprint 11-12)

**Goal:** Basic financial features

| Use Case | Stories | Effort |
|----------|---------|--------|
| UC-16.1-8 | Financial accounts, invoices | L |
| UC-10.1-8 | Person-months | M |
| UC-11.1-8 | Self-readings | M |

**Deliverables:**
- [ ] Invoice generation
- [ ] Payment tracking
- [ ] Person-month calculations
- [ ] Meter reading submissions

### Iteration 7: Reports & Admin (Sprint 13-14)

**Goal:** Analytics, administration

| Use Case | Stories | Effort |
|----------|---------|--------|
| UC-17.1-6 | Reports | L |
| UC-18.1-6 | System admin | M |
| UC-41.1-10 | Subscription billing | L |

**Deliverables:**
- [ ] Financial reports
- [ ] Usage analytics
- [ ] Admin dashboard
- [ ] Subscription management

### Iteration 8: Beta Release (Sprint 15-16)

**Goal:** Pilot customers, stability

| Focus | Activities | Effort |
|-------|------------|--------|
| Quality | Performance optimization | L |
| Quality | Security audit | M |
| Quality | Accessibility audit | M |
| Pilot | Customer onboarding | L |

**Deliverables:**
- [ ] **Beta release** (pilot customers)
- [ ] Performance benchmarks met
- [ ] Security audit passed
- [ ] WCAG 2.1 AA compliance

### Iteration 9-12: Enhanced Features

**Goal:** Full MVP, GA 1.0 release

| Iteration | Focus | Key Features |
|-----------|-------|--------------|
| 9 | Social | UC-06 Neighbors, UC-07 Contacts, UC-24 Community |
| 10 | Security | UC-14.7-8 MFA/OAuth, UC-23 Security features |
| 11 | Automation | UC-26 Workflow automation, UC-36 Scheduling |
| 12 | GA Prep | Bug fixes, documentation, training |

**Deliverables:**
- [ ] **GA 1.0 release**
- [ ] Full documentation
- [ ] Training materials
- [ ] SLA monitoring

### Iteration 13+: Post-GA

| Phase | Focus | Features |
|-------|-------|----------|
| GA 2.0 | Rentals | UC-29-30 Short-term rentals, Guest registration |
| GA 2.1 | Real Estate | UC-31-32 Listings, Portal integration |
| GA 2.2 | Portal | UC-44-51 Reality Portal features |
| GA 3.0 | Advanced | UC-20-22 AI/ML, IoT, Integrations |
| GA 3.x | Operations | UC-35-40 Insurance, Legal, Emergency |

---

## Traceability Matrix

### Traceability Chain

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│ Requirement │ ──► │    Code     │ ──► │    Test     │ ──► │   Release   │
│   (UC-XX)   │     │   (impl)    │     │   (TC-XX)   │     │   (v1.x)    │
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
       │                   │                   │                   │
       ▼                   ▼                   ▼                   ▼
   docs/             backend/             tests/            CHANGELOG
   use-cases.md      src/modules/         integration/      releases/
   functional-       frontend/            e2e/
   requirements.md   apps/
```

### Traceability Identifiers

| Artifact | ID Pattern | Example |
|----------|------------|---------|
| Use Case | UC-{cat}.{seq} | UC-03.1 |
| Functional Requirement | FR-{cat}.{seq} | FR-03.1 |
| API Endpoint | API-{method}-{path} | API-POST-faults |
| DTO | DTO-{name} | DTO-CreateFaultRequest |
| Database Table | DB-{schema}.{table} | DB-property.faults |
| Test Case | TC-{uc}-{seq}-{type} | TC-03.1-001-UNIT |
| Story | STORY-{epic}-{seq} | STORY-003-001 |
| Commit | feat(UC-XX): message | feat(UC-03): Add fault reporting |
| Release | v{major}.{minor}.{patch} | v1.2.0 |

### Traceability Matrix (Sample)

| Use Case | FR | API | DTO | DB | Test | Story | Release |
|----------|-----|-----|-----|-----|------|-------|---------|
| UC-03.1 Report Fault | FR-03.1 | POST /faults | CreateFaultRequest, FaultResponse | property.faults | TC-03.1-001-UNIT, TC-03.1-002-INT, TC-03.1-003-E2E | STORY-003-001 | v1.0.0 |
| UC-03.2 Search Faults | FR-03.2 | GET /faults | FaultListResponse | property.faults | TC-03.2-001-UNIT | STORY-003-002 | v1.0.0 |
| UC-03.6 Update Status | FR-03.6 | PATCH /faults/{id}/status | UpdateStatusRequest | property.faults, property.fault_status_history | TC-03.6-001-UNIT, TC-03.6-002-INT | STORY-003-006 | v1.0.0 |
| UC-04.2 Cast Ballot | FR-04.2 | POST /votes/{id}/ballots | CastBallotRequest | property.ballots | TC-04.2-001-UNIT, TC-04.2-002-E2E | STORY-004-002 | v1.0.0 |
| UC-14.1 Register | FR-14.1 | POST /auth/register | RegisterRequest | identity.users | TC-14.1-001-UNIT, TC-14.1-002-INT, TC-14.1-003-E2E | STORY-014-001 | v0.1.0 |
| UC-14.2 Login | FR-14.2 | POST /auth/login | LoginRequest, TokenResponse | identity.sessions | TC-14.2-001-UNIT, TC-14.2-002-INT | STORY-014-002 | v0.1.0 |
| UC-27.1 Create Org | FR-27.1 | POST /organizations | CreateOrgRequest | platform.organizations | TC-27.1-001-UNIT, TC-27.1-002-INT | STORY-027-001 | v0.1.0 |

### Commit Message Convention

```
{type}(UC-{id}): {description}

[optional body]

Refs: UC-{id}, STORY-{epic}-{seq}
Tests: TC-{uc}-{seq}-{type}
```

**Examples:**
```
feat(UC-03): Add fault reporting endpoint

Implement POST /api/v1/faults endpoint with:
- Input validation
- Photo upload support
- Notification to manager

Refs: UC-03.1, STORY-003-001
Tests: TC-03.1-001-UNIT, TC-03.1-002-INT
```

```
fix(UC-04): Fix vote counting with proxy delegations

Correctly count votes when proxy chain depth > 1

Refs: UC-04.7, STORY-004-007
Tests: TC-04.7-002-INT
```

### Release Notes Template

```markdown
# Release v1.2.0

## Features

### Fault Management (UC-03)
- **UC-03.1** Report Fault: Users can now report faults with photo attachments
- **UC-03.6** Update Status: Managers can update fault status with notes
- **UC-03.8** Assignment: Faults can be assigned to technical managers

### Voting (UC-04)
- **UC-04.2** Cast Ballot: Owners can cast ballots on active votes
- **UC-04.7** Proxy: Support for proxy delegation

## Bug Fixes
- Fixed UC-03.11: Reopened faults now correctly reset to "In Progress"
- Fixed UC-04.2: Ballot update now properly validates vote deadline

## Tests
- Added 45 new unit tests
- Added 12 integration tests
- E2E coverage: 85% of critical paths

## Migration Notes
- Run: `sqlx migrate run`
- New env var: `FAULT_PHOTO_MAX_SIZE`
```

### Traceability Report

Generated automatically in CI:

```yaml
# .github/workflows/traceability.yml
name: Traceability Report

on:
  push:
    branches: [main]
  schedule:
    - cron: '0 0 * * 1'  # Weekly

jobs:
  report:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Generate traceability matrix
        run: |
          python scripts/generate_traceability.py \
            --use-cases docs/use-cases.md \
            --tests tests/ \
            --output docs/traceability-report.md

      - name: Check coverage
        run: |
          python scripts/check_coverage.py \
            --matrix docs/traceability-report.md \
            --threshold 85

      - name: Upload report
        uses: actions/upload-artifact@v4
        with:
          name: traceability-report
          path: docs/traceability-report.md
```

---

## Definition of Done

### Story Definition of Done

- [ ] Code complete and reviewed
- [ ] Unit tests written and passing (≥80% coverage)
- [ ] Integration tests written for API endpoints
- [ ] Documentation updated (API docs, CLAUDE.md if needed)
- [ ] No new linting errors or warnings
- [ ] Accessibility checked (WCAG 2.1 AA)
- [ ] Security review completed (for auth/financial features)
- [ ] Traceability updated (commit message, test IDs)
- [ ] Code deployed to dev environment
- [ ] QA sign-off

### Iteration Definition of Done

- [ ] All stories meet Story DoD
- [ ] E2E tests passing for iteration features
- [ ] Performance benchmarks met
- [ ] No critical or high-severity bugs
- [ ] Release notes drafted
- [ ] Demo completed with stakeholders
- [ ] Deployed to staging environment

### Release Definition of Done

- [ ] All iterations meet Iteration DoD
- [ ] Full regression test suite passing
- [ ] Security audit completed (for major releases)
- [ ] Load testing completed and benchmarks met
- [ ] Documentation complete (user guides, API docs)
- [ ] Training materials prepared (for GA releases)
- [ ] Rollback plan documented and tested
- [ ] Monitoring and alerting configured
- [ ] Release notes published
- [ ] Deployed to production

---

## Summary

| Aspect | Details |
|--------|---------|
| **Test Cases** | ~1,100 mapped to 508 use cases |
| **Testing Pyramid** | 75% Unit, 20% Integration, 5% E2E |
| **MVP Scope** | ~180 use cases (40% of total) |
| **Iterations to GA** | 12 iterations (~24 sprints) |
| **Traceability** | UC → FR → API → DB → TC → Story → Release |
| **Coverage Target** | 85% average (100% for critical paths) |
