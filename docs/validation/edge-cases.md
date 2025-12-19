# Use Case Edge Cases & Error Paths

> **Parent:** See `docs/CLAUDE.md` for use case overview.

This document captures edge cases, error paths, and exception handling for critical use cases. Use this as a reference during implementation to ensure robust handling of non-happy-path scenarios.

---

## UC-14: User Account Management

### UC-14.1: Register Account

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| Expired invitation | Link clicked after 7-day expiry | Show "Invitation expired" with option to request new one |
| Invalid code | Registration code malformed or not found | Show "Invalid registration code" error |
| Email exists | Email already registered in system | Show "Email already in use" with login/reset options |
| Concurrent registration | Same invitation used twice simultaneously | First request succeeds, second shows "Already registered" |
| Organization inactive | Registering to suspended organization | Block registration, show "Organization unavailable" |

**Error Handling:**
- Never reveal whether an email exists for security (password reset flow)
- Rate limit registration attempts per IP (5/hour)
- Log all failed registration attempts for security audit

### UC-14.2: Login

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| Account locked | User exceeds 5 failed attempts | Lock for 15 min, show "Account temporarily locked" |
| Password expired | Policy requires password change | Force redirect to password change screen |
| Organization suspended | User's org is suspended | Block login, show "Contact administrator" |
| Session conflict | User already logged in elsewhere | Allow new login, optionally notify of other sessions |
| MFA required | Organization requires 2FA | Redirect to MFA setup if not configured |

**Error Handling:**
- Generic "Invalid credentials" for wrong email/password (don't reveal which is wrong)
- Increment lockout counter on each failure
- Log IP and device for failed attempts

### UC-14.4: Reset Password

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| User not found | Email doesn't exist in system | Show generic "Check your email" (don't reveal) |
| Token expired | Reset link clicked after 1 hour | Show "Link expired" with option to request new one |
| Token already used | Reset link used twice | Show "Link already used" with option to request new one |
| Concurrent reset | Multiple reset requests sent | Only most recent token is valid |
| Password reuse | New password same as previous 3 | Reject with "Cannot reuse recent passwords" |

### UC-14.8: Delete Account

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| Active lease | User has current lease agreement | Block deletion, show "Active lease must be terminated first" |
| Only owner | User is sole owner of a unit | Block deletion, show "Transfer ownership before deleting" |
| Pending payments | Outstanding balance exists | Block deletion, show "Clear outstanding balance first" |
| GDPR request | User invokes right to deletion | Schedule 30-day deletion window, anonymize immediately |
| System admin | Last system administrator | Block deletion, show "Assign another admin first" |

---

## UC-03: Faults

### UC-03.1: Report Fault

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| Photo too large | Upload exceeds 10MB limit | Show "File too large" with size guidance |
| Invalid file type | User uploads non-image file | Show "Only images allowed (JPG, PNG)" |
| Network offline | Reporting while disconnected | Queue locally, sync when online |
| Location invalid | Building/unit not recognized | Prompt to select from list |
| Duplicate fault | Similar fault reported recently | Suggest "View existing fault?" option |

**Offline Queue Behavior:**
- Store fault data locally with timestamp
- Retry on network reconnect with exponential backoff
- Show "Pending sync" indicator to user
- Expire queued items after 7 days

### UC-03.6: Update Fault Status

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| Concurrent update | Two managers update simultaneously | Optimistic locking, reject second update |
| Invalid transition | Status change violates workflow | Show valid transitions only |
| Assigned user gone | Technical manager no longer exists | Unassign and notify manager |
| Escalation triggered | SLA breach on status change | Auto-notify supervisor |

**Status Transitions:**
```
New -> In Progress -> Resolved -> Closed
     -> Escalated  -> In Progress
Closed -> Reopened -> In Progress
```

### UC-03.10: Close Fault

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| Reporter disagrees | User marks as not resolved | Keep open, notify manager |
| Reopen threshold | Reopened 3+ times | Escalate to organization admin |
| Auto-close | No response after 14 days | Auto-close with notification |

### UC-03.11: Reopen Fault

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| Fault archived | Fault older than 90 days | Block reopen, suggest new fault |
| Reporter deleted | Original reporter no longer exists | Allow any unit resident to reopen |
| Already reopened | Fault in reopened state | Show "Already open" message |

---

## UC-04: Voting and Polls

### UC-04.4: Cast Vote

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| Vote already cast | User tries to vote again | Show "Already voted" with current selection |
| Deadline passed | Vote submitted after deadline | Reject with "Voting closed" message |
| Mid-vote deadline | Deadline passes during form fill | Reject on submit with "Deadline passed" |
| Proxy vote active | User delegated to someone else | Show "Vote delegated to [name]" |
| Not eligible | User not an owner for this vote | Hide vote option, show "Owners only" |

### UC-04.11: Delegate Vote (Proxy Voting)

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| Delegate already voting | Target user has own vote in progress | Block with "User has their own vote" |
| Circular delegation | A -> B -> A delegation loop | Prevent with "Circular delegation not allowed" |
| Delegate not eligible | Target user not an owner | Block with "Only owners can receive delegations" |
| Multiple delegations | User tries to delegate to multiple | Replace previous delegation |
| Delegation expired | Vote ends before delegation used | Notify both parties of unused delegation |

**Circular Detection:**
```
Check delegation chain before creating:
- If target has delegation to source, reject
- If target's delegate chain includes source, reject
- Max chain depth: 3 levels
```

### UC-04.14: Change Vote

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| Changes not allowed | Poll configured as immutable | Show "Changes not allowed for this poll" |
| Deadline passed | Change attempted after close | Reject with "Voting closed" |
| Results published | Vote closed and results shown | Reject with "Results already published" |

---

## UC-27: Multi-tenancy & Organizations

### UC-27.1: Create Organization

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| Duplicate name | Organization name exists | Suggest alternative or add location suffix |
| Invalid VAT | VAT number format incorrect | Show format requirements by country |
| Platform limit | Max organizations reached | Show "Contact sales to upgrade platform" |
| Invalid domain | Email domain blacklisted | Reject with security warning |

### UC-27.3: Delete Organization

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| Has buildings | Organization has active buildings | Block with "Remove buildings first" and list |
| Unpaid invoices | Outstanding platform charges | Block with "Clear balance first" |
| Active users | Users still assigned | Show user count, offer to notify/transfer |
| Data export pending | GDPR export in progress | Block until export completes |

**Deletion Process:**
1. Mark as "pending deletion"
2. Notify all organization admins
3. 30-day grace period
4. Export all data for compliance
5. Anonymize personal data
6. Archive for 7 years (legal requirement)
7. Permanently delete non-essential data

### UC-27.7: Switch Organization Context

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| No access | User removed from target org | Show "Access denied" with contact info |
| Org suspended | Target organization suspended | Show "Organization temporarily unavailable" |
| Session expired | JWT expired during switch | Re-authenticate with target org context |
| Permissions changed | Role changed since last access | Refresh permissions, show any restrictions |

---

## UC-29: Short-term Rental Management

### UC-29.1: Connect Airbnb Account

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| OAuth expired | Token refresh fails | Prompt to re-authenticate |
| Already linked | Account connected by another user | Show "Account already linked to [email]" |
| Rate limited | Too many API requests | Queue operations, retry with backoff |
| API unavailable | Airbnb API down | Show "Sync temporarily unavailable", use cache |
| Wrong account | User connects wrong Airbnb | Offer "Disconnect and try again" |

### UC-29.3: Sync Reservations

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| Duplicate reservation | Same booking imported twice | Skip with "Already exists" log |
| Date conflict | Overlapping reservations | Flag conflict, alert property manager |
| Missing data | Incomplete reservation info | Import partial, flag for manual review |
| Currency mismatch | Foreign currency booking | Convert to org currency, store original |
| Cancelled reservation | Booking cancelled on platform | Update local status, notify manager |

**Conflict Resolution:**
```
Priority order for date conflicts:
1. Existing local bookings
2. Earlier created external booking
3. Manual override flag
```

### UC-29.6: Generate Access Code

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| Smart lock offline | Device not responding | Generate code anyway, flag "Pending sync" |
| Code generation fails | Random generation error | Retry 3x, then fall back to manual code |
| Code already assigned | Time slot overlap | Invalidate old code, generate new |
| Guest early arrival | Check-in before start time | Configurable: block or allow early access |
| Multiple guests | Group booking | Generate single code or individual codes |

---

## UC-16: Financial Management

### UC-16.3: Make Payment

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| Gateway down | Payment processor unavailable | Show "Try again later", offer bank transfer |
| Insufficient funds | Card declined for balance | Show generic "Payment declined" |
| Duplicate payment | Double-click on submit | Idempotency key prevents duplicate |
| Partial payment | Amount less than due | Accept partial, update remaining balance |
| Currency conversion | Payment in foreign currency | Show converted amount before confirm |

**Duplicate Prevention:**
```
1. Generate idempotency key on form load
2. Store with payment request
3. If key exists, return previous result
4. Key expires after 24 hours
```

### UC-16.4: Generate Invoice

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| Invalid VAT rate | Rate doesn't match country | Show "Review tax settings" warning |
| Missing billing info | Owner has incomplete profile | Block generation, list missing fields |
| Tax calculation error | Complex multi-rate scenario | Log error, flag for manual review |
| Negative balance | Owner has credit | Show credit, offer to generate credit note |

### UC-16.6: View Annual Settlement

| Scenario | Edge Case | Expected Behavior |
|----------|-----------|-------------------|
| Period incomplete | Fiscal year not closed | Show "Preliminary" watermark |
| Data migration | Historical data being imported | Show loading state, estimate completion |
| Multiple owners | Unit changed ownership mid-year | Show split by ownership period |

---

## Cross-Cutting Concerns

### Multi-tenancy Isolation

Every database query must include tenant context:

```sql
-- CORRECT: Always filter by tenant
SELECT * FROM faults WHERE tenant_id = $1 AND id = $2;

-- WRONG: Missing tenant filter
SELECT * FROM faults WHERE id = $1;
```

**Sharing Edge Cases:**
| Scenario | Behavior |
|----------|----------|
| Share document to other org | Block, show "Cannot share outside organization" |
| Forward message to other org | Allow, but strip attachments |
| Export data with tenant refs | Include only own tenant data |

### Rate Limiting

| Endpoint Type | Rate Limit | Window |
|---------------|------------|--------|
| Authentication | 5 attempts | 15 min |
| API read | 100 requests | 1 min |
| API write | 20 requests | 1 min |
| File upload | 10 uploads | 1 hour |
| Bulk operations | 1 operation | 5 min |

**Response:**
- 429 Too Many Requests
- `Retry-After` header with seconds
- Log for security monitoring

### GDPR Compliance

Every use case involving personal data must handle:

| Right | Implementation |
|-------|----------------|
| Access | Export all user data in JSON/CSV |
| Rectification | User can edit profile data |
| Erasure | Anonymize or delete (see UC-14.8) |
| Portability | Export in machine-readable format |
| Restriction | Disable account without deletion |

### Offline Support

| Use Case | Offline Behavior |
|----------|------------------|
| UC-03.1: Report Fault | Queue with photos, sync later |
| UC-11.1: Submit Meter Reading | Queue reading, sync later |
| UC-05.5: Send Message | Queue message, show "Pending" |
| UC-01.4: View Notifications | Show cached, mark "Last sync: X" |
| UC-04.4: Cast Vote | Block, require online for voting |

**Conflict Resolution:**
```
1. Server timestamp wins for create/update conflicts
2. User notified of conflicts on sync
3. Original offline data preserved in conflict log
4. Manual resolution option for complex conflicts
```

---

## Testing Recommendations

### Priority Test Cases

For each edge case documented above:

1. **Unit Tests**
   - Input validation for edge values
   - Error message verification
   - State transition validation

2. **Integration Tests**
   - Multi-user concurrent scenarios
   - External API failure handling
   - Database constraint enforcement

3. **E2E Tests**
   - User journey through error states
   - Offline/online transitions
   - Cross-tenant isolation verification

### Security Test Cases

- SQL injection in all text inputs
- IDOR (Insecure Direct Object Reference) for tenant isolation
- CSRF for state-changing operations
- Rate limit bypass attempts
- Session fixation/hijacking

---

## Revision History

| Date | Author | Changes |
|------|--------|---------|
| 2024-12-20 | Initial | Created edge cases for UC-14, UC-03, UC-04, UC-27, UC-29, UC-16 |
