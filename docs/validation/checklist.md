# Use Case Validation Checklist

> **Parent:** See `docs/CLAUDE.md` for use case overview.

This checklist tracks validation progress with stakeholders and ensures all scenarios are reviewed before implementation.

---

## Stakeholder Validation

### Property Managers

**Contact:** [TBD]
**Review Date:** [TBD]

| Category | Use Cases | Status | Notes |
|----------|-----------|--------|-------|
| UC-03: Faults | 14 | [ ] Pending | Priority: High |
| UC-15: Buildings | 10 | [ ] Pending | Priority: High |
| UC-36: Maintenance | 8 | [ ] Pending | Priority: Medium |
| UC-37: Suppliers | 8 | [ ] Pending | Priority: Medium |

**Key Questions:**
- [ ] Are fault status transitions correct?
- [ ] Are all reporting fields captured?
- [ ] Is the escalation workflow correct?
- [ ] Are maintenance scheduling options sufficient?

---

### Building Owners

**Contact:** [TBD]
**Review Date:** [TBD]

| Category | Use Cases | Status | Notes |
|----------|-----------|--------|-------|
| UC-04: Voting | 14 | [ ] Pending | Priority: High |
| UC-16: Financial | 10 | [ ] Pending | Priority: High |
| UC-40: Budget | 8 | [ ] Pending | Priority: Medium |
| UC-28: Delegation | 10 | [ ] Pending | Priority: Medium |

**Key Questions:**
- [ ] Are voting rules aligned with housing cooperative laws?
- [ ] Is proxy voting workflow correct?
- [ ] Are financial statement formats acceptable?
- [ ] Are delegation permissions granular enough?

---

### Short-term Rental Managers

**Contact:** [TBD]
**Review Date:** [TBD]

| Category | Use Cases | Status | Notes |
|----------|-----------|--------|-------|
| UC-29: Rental Management | 15 | [ ] Pending | Priority: High |
| UC-30: Guest Registration | 10 | [ ] Pending | Priority: High |

**Key Questions:**
- [ ] Are Airbnb/Booking.com sync intervals acceptable?
- [ ] Is access code workflow correct?
- [ ] Are police registration requirements met?
- [ ] Is guest communication automated enough?

---

### Real Estate Agents

**Contact:** [TBD]
**Review Date:** [TBD]

| Category | Use Cases | Status | Notes |
|----------|-----------|--------|-------|
| UC-31: Listings | 14 | [ ] Pending | Priority: High |
| UC-32: Portal Integration | 10 | [ ] Pending | Priority: High |
| UC-33: Tenant Screening | 12 | [ ] Pending | Priority: Medium |
| UC-34: Lease Management | 10 | [ ] Pending | Priority: Medium |

**Key Questions:**
- [ ] Are listing fields sufficient for local market?
- [ ] Are required portal integrations covered?
- [ ] Is screening workflow GDPR-compliant?
- [ ] Are lease templates flexible enough?

---

### Reality Portal Users

**Contact:** [TBD]
**Review Date:** [TBD]

| Category | Use Cases | Status | Notes |
|----------|-----------|--------|-------|
| UC-44: Favorites | 6 | [ ] Pending | Priority: Medium |
| UC-45: Saved Searches | 8 | [ ] Pending | Priority: High |
| UC-46: Inquiries | 6 | [ ] Pending | Priority: High |
| UC-47: User Accounts | 15 | [ ] Pending | Priority: High |
| UC-48: Comparison | 5 | [ ] Pending | Priority: Low |
| UC-49: Agency Management | 10 | [ ] Pending | Priority: Medium |
| UC-50: Property Import | 10 | [ ] Pending | Priority: Medium |
| UC-51: Realtor Profile | 12 | [ ] Pending | Priority: Medium |

**Key Questions:**
- [ ] Are search filters sufficient?
- [ ] Are alert notification options adequate?
- [ ] Is inquiry workflow intuitive?
- [ ] Are SSO options (Google, Apple, Facebook) correct?

---

### IT/Security Team

**Contact:** [TBD]
**Review Date:** [TBD]

| Category | Use Cases | Status | Notes |
|----------|-----------|--------|-------|
| UC-14: User Accounts | 12 | [ ] Pending | Priority: Critical |
| UC-23: Security | 12 | [ ] Pending | Priority: Critical |
| UC-27: Multi-tenancy | 10 | [ ] Pending | Priority: Critical |

**Key Questions:**
- [ ] Is MFA implementation correct (TOTP, SMS)?
- [ ] Are SSO protocols supported (SAML, OIDC)?
- [ ] Is tenant isolation verified?
- [ ] Are rate limiting thresholds appropriate?
- [ ] Is audit logging comprehensive?

---

### Legal/Compliance Team

**Contact:** [TBD]
**Review Date:** [TBD]

| Category | Use Cases | Status | Notes |
|----------|-----------|--------|-------|
| UC-23: GDPR/Compliance | 12 | [ ] Pending | Priority: Critical |
| UC-38: Legal | 8 | [ ] Pending | Priority: High |
| UC-33: Tenant Screening | 12 | [ ] Pending | Priority: High |

**Key Questions:**
- [ ] Is GDPR data export format compliant?
- [ ] Is data retention policy correct by region?
- [ ] Are consent flows legally valid?
- [ ] Are screening practices region-compliant?

---

## Technical Review Checklist

### For Each Use Case

| Check | Status |
|-------|--------|
| Actor clearly defined | [ ] |
| Platform scope defined (ppt-web, mobile, etc.) | [ ] |
| Edge cases documented (see edge-cases.md) | [ ] |
| Error paths defined | [ ] |
| Multi-tenancy isolation verified | [ ] |
| GDPR implications reviewed | [ ] |
| Offline capability requirement identified | [ ] |
| API endpoint mapped | [ ] |
| UI/UX mockup linked | [ ] |

### Category Completion

| Category | Edge Cases | Error Paths | API Spec | UI Mockup | Tests |
|----------|------------|-------------|----------|-----------|-------|
| UC-01: Notifications | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-02: Announcements | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-03: Faults | [x] | [x] | [ ] | [ ] | [ ] |
| UC-04: Voting | [x] | [x] | [ ] | [ ] | [ ] |
| UC-05: Messages | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-06: Neighbors | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-07: Contacts | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-08: Documents | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-09: Forms | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-10: Person-Months | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-11: Self-Readings | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-12: Outages | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-13: News | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-14: User Accounts | [x] | [x] | [ ] | [ ] | [ ] |
| UC-15: Buildings | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-16: Financial | [x] | [x] | [ ] | [ ] | [ ] |
| UC-17: Reports | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-18: Admin | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-19: Real-time | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-20: AI/ML | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-21: IoT | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-22: Integrations | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-23: Security | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-24: Community | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-25: Accessibility | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-26: Automation | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-27: Multi-tenancy | [x] | [x] | [ ] | [ ] | [ ] |
| UC-28: Delegation | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-29: Short-term Rental | [x] | [x] | [ ] | [ ] | [ ] |
| UC-30: Guest Reg | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-31: Listings | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-32: Portal Integration | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-33: Screening | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-34: Lease | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-35: Insurance | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-36: Maintenance | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-37: Suppliers | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-38: Legal | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-39: Emergency | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-40: Budget | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-41: Subscription | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-42: Onboarding | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-43: Mobile Features | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-44: Favorites | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-45: Saved Searches | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-46: Inquiries | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-47: Portal Accounts | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-48: Comparison | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-49: Agency | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-50: Import | [ ] | [ ] | [ ] | [ ] | [ ] |
| UC-51: Realtor Profile | [ ] | [ ] | [ ] | [ ] | [ ] |

---

## Sign-off Tracking

### Phase 1: Core (UC-14, UC-27, UC-15)

| Milestone | Stakeholder | Status | Date |
|-----------|-------------|--------|------|
| Requirements review | IT/Security | [ ] Pending | |
| Edge cases approved | Development | [ ] Pending | |
| API spec approved | Frontend/Backend | [ ] Pending | |
| UI mockups approved | UX Team | [ ] Pending | |
| Implementation complete | Development | [ ] Pending | |
| QA testing complete | QA Team | [ ] Pending | |
| UAT approved | Business Owner | [ ] Pending | |

### Phase 2: Core Features (UC-01 to UC-13)

| Milestone | Stakeholder | Status | Date |
|-----------|-------------|--------|------|
| Requirements review | Property Managers | [ ] Pending | |
| Requirements review | Building Owners | [ ] Pending | |
| Edge cases approved | Development | [ ] Pending | |
| API spec approved | Frontend/Backend | [ ] Pending | |
| Implementation complete | Development | [ ] Pending | |
| UAT approved | Business Owner | [ ] Pending | |

### Phase 3: Advanced Features (UC-19 to UC-26)

| Milestone | Stakeholder | Status | Date |
|-----------|-------------|--------|------|
| Technical feasibility | IT/Security | [ ] Pending | |
| AI/ML requirements | Data Team | [ ] Pending | |
| IoT integration scope | Hardware Team | [ ] Pending | |
| Implementation complete | Development | [ ] Pending | |
| UAT approved | Business Owner | [ ] Pending | |

### Phase 4: Rental & Real Estate (UC-29 to UC-34)

| Milestone | Stakeholder | Status | Date |
|-----------|-------------|--------|------|
| Requirements review | Rental Managers | [ ] Pending | |
| Requirements review | Real Estate Agents | [ ] Pending | |
| Legal compliance | Legal Team | [ ] Pending | |
| Implementation complete | Development | [ ] Pending | |
| UAT approved | Business Owner | [ ] Pending | |

### Phase 5: Reality Portal (UC-44 to UC-51)

| Milestone | Stakeholder | Status | Date |
|-----------|-------------|--------|------|
| Requirements review | Portal Users | [ ] Pending | |
| Requirements review | Agencies | [ ] Pending | |
| Implementation complete | Development | [ ] Pending | |
| UAT approved | Business Owner | [ ] Pending | |

---

## Issue Tracking

### Open Issues

| ID | Category | Description | Priority | Assignee | Status |
|----|----------|-------------|----------|----------|--------|
| V-001 | UC-14 | SSO SAML configuration unclear | High | [TBD] | Open |
| V-002 | UC-33 | Regional screening requirements | Medium | [TBD] | Open |
| V-003 | UC-41 | Usage-based billing formula | Medium | [TBD] | Open |

### Resolved Issues

| ID | Category | Description | Resolution | Date |
|----|----------|-------------|------------|------|
| | | | | |

---

## Revision History

| Date | Author | Changes |
|------|--------|---------|
| 2024-12-20 | Initial | Created validation checklist structure |
