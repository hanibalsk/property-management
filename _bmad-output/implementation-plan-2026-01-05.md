# Implementation Plan
## Property Management System (PPT) - Gap Closure
**Date:** 2026-01-05
**Based on:** Implementation Readiness Report 2026-01-05
**Goal:** Close the 50% frontend gap and address 8 identified issues

---

## Executive Summary

This plan addresses the gaps identified in the completeness review:
- **Primary Goal:** Increase frontend coverage from 50% to 90%
- **Duration:** 8 sprints (16 weeks)
- **Issues to Resolve:** 8 (4 HIGH, 4 MEDIUM priority)

---

## Issue Tracker

| # | Issue | Priority | Sprint | Status |
|---|-------|----------|--------|--------|
| 1 | Frontend Coverage Gap (50%) | HIGH | 2-5 | 🔲 Pending |
| 2 | UC-12 Outages Not Implemented | HIGH | 1 | 🔲 Pending |
| 3 | Offline Sync Indicators Missing | HIGH | 3 | 🔲 Pending |
| 4 | Push Notification Infrastructure | HIGH | 1 | 🔲 Pending |
| 5 | Action-First UX Pattern | MEDIUM | 6 | 🔲 Pending |
| 6 | WCAG Accessibility Audit | MEDIUM | 7 | 🔲 Pending |
| 7 | AI-Assisted UI Flows | MEDIUM | 5 | 🔲 Pending |
| 8 | Epic Naming (2B/10A) | LOW | N/A | 🔲 Documentation only |

---

## Sprint Plan Overview

```
Sprint 1 (Week 1-2):   Critical Infrastructure
Sprint 2 (Week 3-4):   Core Communication Features
Sprint 3 (Week 5-6):   Resident Self-Service Features
Sprint 4 (Week 7-8):   Management & Administration
Sprint 5 (Week 9-10):  AI-Assisted Features
Sprint 6 (Week 11-12): UX Pattern Improvements
Sprint 7 (Week 13-14): Accessibility & Polish
Sprint 8 (Week 15-16): Testing & Launch Prep
```

---

## Sprint 1: Critical Infrastructure (Week 1-2)

**Goal:** Address blocking infrastructure issues

### Epic: UC-12 Utility Outages Implementation

**Story 1.1: Outages Backend Routes**
```
As a resident
I want to view current and planned utility outages
So that I can prepare for service interruptions
```

**Acceptance Criteria:**
- [ ] GET `/api/v1/outages` returns list of outages
- [ ] GET `/api/v1/outages/{id}` returns outage detail
- [ ] Outages filterable by: commodity (water, electricity, gas), status, date range
- [ ] Integration with external supplier APIs (optional Phase 2)

**Technical Tasks:**
- [ ] Create `outages` table migration
- [ ] Add `outage.rs` model in `backend/crates/db/src/models/`
- [ ] Add `outage.rs` repository in `backend/crates/db/src/repositories/`
- [ ] Add `outages.rs` routes in `backend/servers/api-server/src/routes/`
- [ ] Add OpenAPI spec for outages endpoints

**Story 1.2: Outages Frontend Feature**
```
As a resident
I want to see outages on my dashboard and receive notifications
So that I'm informed about service disruptions
```

**Technical Tasks:**
- [ ] Create `frontend/apps/ppt-web/src/features/outages/` directory
- [ ] Implement OutagesList component
- [ ] Implement OutageDetail component
- [ ] Add to navigation menu
- [ ] Add notification integration for new outages

---

### Epic: Push Notification Infrastructure Verification

**Story 1.3: Verify FCM/APNs Setup**
```
As a developer
I want to verify push notification infrastructure works end-to-end
So that users receive timely notifications
```

**Technical Tasks:**
- [ ] Verify FCM project configuration in `firebase.json`
- [ ] Verify APNs certificates are current
- [ ] Test push delivery on iOS simulator
- [ ] Test push delivery on Android emulator
- [ ] Document push notification setup in `docs/infrastructure/push-notifications.md`

**Story 1.4: Deep Link Implementation**
```
As a user receiving a push notification
I want to tap and go directly to the relevant screen
So that I can take action immediately
```

**Technical Tasks:**
- [ ] Implement deep link handlers in React Native app
- [ ] Map notification types to screen routes
- [ ] Test deep links for: faults, announcements, votes, messages
- [ ] Add analytics for deep link engagement

---

## Sprint 2: Core Communication Features (Week 3-4)

**Goal:** Implement missing communication UIs

### Epic: Direct Messaging UI

**Story 2.1: Messaging List View**
```
As a user
I want to see my message conversations
So that I can communicate with neighbors and managers
```

**Backend Status:** ✅ `messaging.rs` routes exist

**Technical Tasks:**
- [ ] Create `frontend/apps/ppt-web/src/features/messaging/` directory
- [ ] Implement ConversationList component
- [ ] Implement ConversationDetail component
- [ ] Implement MessageComposer component
- [ ] Add real-time message updates via WebSocket
- [ ] Add unread message badge to navigation

**Story 2.2: New Message Composer**
```
As a user
I want to start a new conversation with someone in my building
So that I can reach out to neighbors or managers
```

**Technical Tasks:**
- [ ] Implement recipient search/select
- [ ] Implement message drafting
- [ ] Add attachment support (photos)
- [ ] Implement send confirmation

---

### Epic: Neighbors Directory UI

**Story 2.3: Neighbors List View**
```
As a resident
I want to see who lives in my building
So that I can connect with my neighbors
```

**Backend Status:** ✅ `neighbors.rs` routes exist

**Technical Tasks:**
- [ ] Create `frontend/apps/ppt-web/src/features/neighbors/` directory
- [ ] Implement NeighborsList component with privacy settings
- [ ] Implement NeighborProfile component
- [ ] Add filtering by entrance/floor
- [ ] Implement "Invite Neighbor" flow

---

## Sprint 3: Resident Self-Service Features (Week 5-6)

**Goal:** Enable residents to manage their unit data

### Epic: Meter Readings UI

**Story 3.1: Meter Reading Submission**
```
As a resident
I want to submit my meter readings with photos
So that my utility charges are accurate
```

**Backend Status:** ✅ `meters.rs` routes exist

**Technical Tasks:**
- [ ] Create `frontend/apps/ppt-web/src/features/meters/` directory
- [ ] Implement MeterReadingForm with photo capture
- [ ] Implement MeterReadingHistory component
- [ ] Add OCR preview (show extracted value for confirmation)
- [ ] Implement submission confirmation

**Story 3.2: Offline Meter Reading (Mobile)**
```
As a resident in an area with poor connectivity
I want to submit readings offline
So that I can complete the task from my basement/meter room
```

**Technical Tasks:**
- [ ] Implement offline queue in React Native
- [ ] Add "Will sync when online" indicator (Issue #3)
- [ ] Implement background sync on connectivity restore
- [ ] Add sync status to readings list

---

### Epic: Person-Months Tracking UI

**Story 3.3: Person-Months Declaration**
```
As a unit owner
I want to declare how many people live in my unit each month
So that shared costs are fairly allocated
```

**Backend Status:** ✅ `person_months.rs` routes exist

**Technical Tasks:**
- [ ] Create `frontend/apps/ppt-web/src/features/person-months/` directory
- [ ] Implement PersonMonthsForm component
- [ ] Implement PersonMonthsHistory component
- [ ] Add reminder notifications for monthly declaration

---

### Epic: Delegation Management UI

**Story 3.4: Delegation Dashboard**
```
As a unit owner
I want to delegate rights to another person
So that they can act on my behalf
```

**Backend Status:** ✅ `delegations.rs` routes exist

**Technical Tasks:**
- [ ] Create `frontend/apps/ppt-web/src/features/delegations/` directory
- [ ] Implement DelegationsList component
- [ ] Implement CreateDelegation wizard
- [ ] Implement delegation acceptance flow
- [ ] Add delegation status indicators

---

## Sprint 4: Management & Administration (Week 7-8)

**Goal:** Complete management features

### Epic: Budget Planning UI

**Story 4.1: Budget Dashboard**
```
As a property manager
I want to view and plan building budgets
So that I can manage finances effectively
```

**Backend Status:** ✅ `budgets.rs` routes exist

**Technical Tasks:**
- [ ] Create `frontend/apps/ppt-web/src/features/budgets/` directory
- [ ] Implement BudgetOverview component
- [ ] Implement BudgetPlanningWizard
- [ ] Implement BudgetComparison (actual vs planned)
- [ ] Add budget export functionality

---

### Epic: Subscription & Billing UI

**Story 4.2: Subscription Management**
```
As an organization admin
I want to manage our subscription and billing
So that I can control costs
```

**Backend Status:** ✅ `subscriptions.rs` routes exist

**Technical Tasks:**
- [ ] Create `frontend/apps/ppt-web/src/features/subscriptions/` directory
- [ ] Implement SubscriptionOverview component
- [ ] Implement PlanSelection component
- [ ] Implement BillingHistory component
- [ ] Implement PaymentMethodManagement

---

### Epic: Onboarding & Help UI

**Story 4.3: User Onboarding Tour**
```
As a new user
I want a guided tour of the application
So that I can learn how to use it effectively
```

**Backend Status:** ✅ `onboarding.rs` routes exist

**Technical Tasks:**
- [ ] Implement onboarding tour using react-joyride
- [ ] Create tour steps for each major feature
- [ ] Implement progress tracking
- [ ] Add "Skip tour" and "Restart tour" options

---

## Sprint 5: AI-Assisted Features (Week 9-10)

**Goal:** Expose AI capabilities in frontend (Issue #7)

### Epic: AI-Assisted Fault Reporting

**Story 5.1: Photo-First Fault Report**
```
As a resident reporting a fault
I want the system to analyze my photo
So that category and priority are suggested automatically
```

**Backend Status:** ✅ `ai.rs` routes exist

**Technical Tasks:**
- [ ] Enhance FaultReportForm with photo-first flow
- [ ] Integrate with AI categorization endpoint
- [ ] Show AI suggestions with confidence indicators
- [ ] Allow user to accept/modify suggestions
- [ ] Track AI accuracy for model improvement

---

### Epic: AI Chatbot UI

**Story 5.2: AI Assistant Chat**
```
As a user
I want to ask questions to an AI assistant
So that I can get quick answers about building matters
```

**Backend Status:** ✅ `ai_chat.rs` model exists

**Technical Tasks:**
- [ ] Create `frontend/apps/ppt-web/src/features/ai-chat/` directory
- [ ] Implement ChatInterface component
- [ ] Implement message streaming display
- [ ] Add suggested questions
- [ ] Implement conversation history

---

### Epic: OCR Meter Reading Preview

**Story 5.3: OCR Value Extraction Preview**
```
As a resident submitting a meter reading
I want to see the OCR-extracted value before submitting
So that I can verify it's correct
```

**Technical Tasks:**
- [ ] Add OCR preview step in MeterReadingForm
- [ ] Show extracted value with bounding box on image
- [ ] Allow manual correction
- [ ] Store corrections for model training

---

## Sprint 6: UX Pattern Improvements (Week 11-12)

**Goal:** Implement action-first UX pattern (Issue #5)

### Epic: Action Queue Dashboard

**Story 6.1: Manager Action Queue**
```
As a property manager
I want to see a prioritized queue of items needing my attention
So that I can process tasks efficiently
```

**Technical Tasks:**
- [ ] Create ActionQueue component for manager dashboard
- [ ] Aggregate pending items: faults, approvals, votes, messages
- [ ] Implement inline actions (approve/reject without drilling down)
- [ ] Add keyboard shortcuts for power users
- [ ] Track time-to-action metrics

**Story 6.2: Resident Action Queue**
```
As a resident
I want to see what needs my attention
So that I can complete tasks quickly
```

**Technical Tasks:**
- [ ] Create resident-focused ActionQueue
- [ ] Show: pending votes, meter reading due, person-months declaration
- [ ] Implement 60-second task completion flow
- [ ] Add notification deep-link integration

---

### Epic: Command Palette

**Story 6.3: Power User Command Palette**
```
As a power user
I want to use keyboard shortcuts to navigate quickly
So that I can work more efficiently
```

**Technical Tasks:**
- [ ] Implement command palette (⌘K / Ctrl+K)
- [ ] Index all navigable pages
- [ ] Add common actions to palette
- [ ] Implement fuzzy search
- [ ] Add recently visited items

---

## Sprint 7: Accessibility & Polish (Week 13-14)

**Goal:** WCAG 2.1 AA compliance (Issue #6)

### Epic: Accessibility Audit & Fixes

**Story 7.1: Automated Accessibility Testing**
```
As a developer
I want automated accessibility checks in CI
So that regressions are caught early
```

**Technical Tasks:**
- [ ] Add axe-core to test suite
- [ ] Configure accessibility rules in CI
- [ ] Fix all critical/serious violations
- [ ] Document accessibility exceptions

**Story 7.2: Manual Accessibility Review**
```
As a user with accessibility needs
I want the application to work with screen readers and keyboard
So that I can use all features
```

**Technical Tasks:**
- [ ] Test with NVDA (Windows) and VoiceOver (Mac/iOS)
- [ ] Verify keyboard navigation for all features
- [ ] Check color contrast (4.5:1 minimum)
- [ ] Verify 200% text scaling
- [ ] Test with high contrast mode

---

### Epic: High Contrast Theme

**Story 7.3: Accessibility Theme**
```
As a user with visual impairments
I want a high contrast theme option
So that I can see the interface clearly
```

**Technical Tasks:**
- [ ] Create high-contrast color tokens
- [ ] Implement theme toggle in settings
- [ ] Persist theme preference
- [ ] Test all components in high contrast mode

---

## Sprint 8: Testing & Launch Prep (Week 15-16)

**Goal:** Quality assurance and launch preparation

### Epic: End-to-End Testing

**Story 8.1: Critical Flow E2E Tests**
```
As a QA engineer
I want comprehensive E2E tests for critical flows
So that we can confidently deploy
```

**Technical Tasks:**
- [ ] Write Playwright tests for authentication flows
- [ ] Write Playwright tests for fault reporting
- [ ] Write Playwright tests for voting
- [ ] Write Playwright tests for messaging
- [ ] Configure E2E tests in CI

---

### Epic: Performance Optimization

**Story 8.2: Bundle Size Optimization**
```
As a user on mobile
I want the app to load quickly
So that I can use it anywhere
```

**Technical Tasks:**
- [ ] Analyze bundle with webpack-bundle-analyzer
- [ ] Implement code splitting for routes
- [ ] Lazy load feature modules
- [ ] Optimize images with next/image
- [ ] Target: < 200KB gzipped initial bundle

---

### Epic: Documentation Updates

**Story 8.3: Feature Documentation**
```
As a new team member
I want up-to-date documentation
So that I can understand the system
```

**Technical Tasks:**
- [ ] Update API documentation
- [ ] Update feature documentation
- [ ] Create user guides for new features
- [ ] Update CLAUDE.md with new structure

---

## Resource Allocation

### Team Composition (Recommended)

| Role | Count | Focus |
|------|-------|-------|
| Frontend Developer | 2 | Feature implementation |
| Backend Developer | 1 | API support, bug fixes |
| UX Designer | 0.5 | Design review, accessibility |
| QA Engineer | 1 | Testing, automation |

### Sprint Velocity Targets

| Sprint | Story Points | Features |
|--------|--------------|----------|
| Sprint 1 | 20 | Outages, Push infra |
| Sprint 2 | 25 | Messaging, Neighbors |
| Sprint 3 | 25 | Meters, Person-months, Delegation |
| Sprint 4 | 20 | Budgets, Subscriptions, Onboarding |
| Sprint 5 | 25 | AI features |
| Sprint 6 | 20 | UX patterns |
| Sprint 7 | 15 | Accessibility |
| Sprint 8 | 15 | Testing, polish |

---

## Success Metrics

### Coverage Targets

| Metric | Current | Target | Sprint |
|--------|---------|--------|--------|
| Frontend coverage | 50% | 90% | Sprint 5 |
| Full-stack features | 49% | 85% | Sprint 5 |
| WCAG compliance | Unknown | AA | Sprint 7 |
| E2E test coverage | ~30% | 80% | Sprint 8 |

### Performance Targets

| Metric | Target |
|--------|--------|
| Initial bundle size | < 200KB gzipped |
| LCP | < 2.5s |
| FID | < 100ms |
| CLS | < 0.1 |

### Quality Targets

| Metric | Target |
|--------|--------|
| Accessibility violations | 0 critical/serious |
| Test coverage (unit) | > 80% |
| Test coverage (E2E) | > 80% critical flows |

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Backend API changes | Coordinate with backend team weekly |
| Scope creep | Strict sprint boundaries, defer to backlog |
| Resource constraints | Prioritize HIGH issues first |
| Technical debt | 20% sprint capacity for refactoring |

---

## Appendix: Feature Priority Matrix

### P0 - Must Have (Sprint 1-3)
- UC-12 Outages
- Push notifications
- Messaging UI
- Neighbors UI
- Meter readings UI
- Offline sync indicators

### P1 - Should Have (Sprint 4-5)
- Person-months UI
- Delegation UI
- Budget UI
- AI-assisted features
- Subscription UI

### P2 - Nice to Have (Sprint 6-7)
- Action queue dashboard
- Command palette
- High contrast theme
- Onboarding tour

### P3 - Defer
- Work orders UI
- Vendor management UI
- Insurance UI
- IoT dashboard

---

**Plan Created:** 2026-01-05
**Review Date:** Weekly sprint planning
**Owner:** Development Team Lead
