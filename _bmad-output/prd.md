---
stepsCompleted: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
inputDocuments:
  - docs/use-cases.md
  - docs/non-functional-requirements.md
  - docs/functional-requirements.md
  - docs/spec1.0.md
  - docs/architecture.md
  - docs/technical-design.md
  - docs/domain-model.md
  - docs/sequence-diagrams.md
  - docs/testability-and-implementation.md
  - docs/project-structure.md
  - docs/CLAUDE.md
  - docs/index.md
  - docs/DOCUMENTATION_DEEP_DIVE.md
  - docs/ARCHITECTURE_REVIEW.md
documentCounts:
  briefs: 0
  research: 0
  brainstorming: 0
  projectDocs: 14
workflowType: 'prd'
lastStep: 11
project_name: 'Property Management System (PPT) & Reality Portal'
user_name: 'Martin Janci'
date: '2025-12-20'
---

# Product Requirements Document - Property Management System (PPT) & Reality Portal

**Author:** Martin Janci
**Date:** 2025-12-20

## Executive Summary

### The Problem

Property management in Central Europe remains fragmented. Building administrators juggle disconnected systems for tenant communication, fault tracking, financial management, and utility readings. When owners want to sell or rent, they start from scratch on separate real estate portals. Meanwhile, smart building technology and AI capabilities remain inaccessible to all but the largest property management companies.

**The result:** Inefficient operations, frustrated residents, and missed opportunities for property owners.

### Our Solution

The Property Management System (PPT) and Reality Portal create a unified ecosystem that fuses:
- **Property Operations** ↔ **Real Estate Marketplace** - Seamless transition from managing to listing
- **Traditional Workflows** ↔ **AI/IoT Intelligence** - Smart automation accessible to every building

**Vision:** Every property manager, owner, and tenant operates on a single intelligent platform - from daily maintenance requests to property transactions - with AI handling routine tasks and IoT providing real-time building insights.

### Platform Overview

| System | Purpose | Technology |
|--------|---------|------------|
| **Property Management** (ppt-web, mobile) | Building administration for cooperatives & management companies | React SPA, React Native |
| **Reality Portal** (reality-web, mobile-native) | Public real estate marketplace with agency management | Next.js SSR, Kotlin Multiplatform |
| **Backend Services** | api-server:8080, reality-server:8081 | Rust (Axum) |

**Scope:** 508 use cases across 51 functional categories, serving 20+ actor types.

### What Makes This Special

#### The Fundamental Insight

Property lifecycle is continuous: buildings are managed, units are listed, tenants move in, buildings are managed again. Yet the software industry treats each phase as separate products requiring separate data, separate logins, separate workflows.

**PPT is architected differently.** A single data model spans the entire property lifecycle.

#### Architectural Differentiators (Not Feature Counts)

**1. Unified Data Model**
An owner's unit in Property Management IS the same entity as a listing on Reality Portal. No sync. No duplicate entry. No data drift.

*Technical reality:* Shared PostgreSQL with tenant isolation. `unit_id` is the foreign key across both systems.

**2. AI/IoT as First-Class Citizens**
AI capabilities aren't bolted-on API calls. They're embedded in the domain model:
- `meter_reading` table has `ocr_extracted_value` and `ocr_confidence` columns
- `fault` table has `ai_category`, `ai_priority_suggestion`, `ai_resolution_prediction`
- `message` table has `sentiment_score` for trend detection

*This means:* AI improves over time using YOUR data, not generic models.

**3. Multi-Tenant from Ground Up**
Every query is tenant-scoped. This isn't a feature flag - it's the architecture.
- Organization isolation is guaranteed at the database layer
- Cross-tenant data leaks are architecturally impossible (not just policy-forbidden)

**4. Regional Compliance Built-In**
GDPR data export/deletion aren't afterthought endpoints. They're use cases (UC-23.4, UC-23.5) with full functional requirements.

#### Honest Differentiator Assessment

| Claim | Status | Evidence |
|-------|--------|----------|
| Unified property-to-listing data | ✅ Verified | Architecture docs confirm shared DB |
| AI embedded in data model | ✅ Verified | Domain model shows AI columns |
| Multi-tenant isolation | ✅ Verified | NFR mandates tenant context |
| Central European compliance | ✅ Verified | GDPR use cases documented |
| "No competitor does this" | ⚠️ Unverified | Requires market research |
| "50% efficiency gain" | ⚠️ Aspirational | No baseline to measure against |

### User Benefits by Role

| Role | Current Pain | PPT Solution |
|------|--------------|--------------|
| **Owner** | Multiple logins, no visibility into building finances, must attend meetings to vote | Single app, real-time balance, remote voting (UC-04.4), personalized notifications for your unit only |
| **Tenant** | Fault reports lost in email, no status updates, rent tracking separate | Mobile fault reporting with photo, status tracking (UC-03), rent payment tracking, privacy-protected data visible only to authorized managers |
| **Property Manager** | 5+ systems, manual data re-entry, WhatsApp chaos | Unified dashboard, automated notifications, AI assistance, accounting system integration (UC-22.2) |
| **Realtor** | Manual listing creation on multiple portals, no property data access | One-click listing from existing unit data (UC-31), multi-portal sync - update once, publish to Nehnuteľnosti.sk, Reality.sk, and more (UC-32) |

### Data Privacy by Role

| Role | What You Can See | What Others See About You |
|------|------------------|---------------------------|
| **Owner** | Your unit, your payments, your votes, building-wide announcements | Name visible to neighbors (configurable), vote choices anonymous |
| **Tenant** | Your lease, your faults, your communications | Manager sees contact info and lease terms only |
| **Manager** | All units in assigned buildings, all residents, all transactions | Residents see name and contact info |
| **Realtor** | Your listings, your inquiries, agency-shared listings | Public sees listing, not personal details |

Full GDPR rights (export, deletion) available to all users. See UC-23.4, UC-23.5.

### Success Vision

When PPT succeeds:
- Property managers spend 50% less time on routine communication
- Owners list properties in minutes, not hours
- Tenants get answers instantly, not after waiting days
- Buildings become smarter without expensive retrofits

### Implementation Approach

This PRD covers 508 use cases. Implementation follows a phased approach defined in `docs/testability-and-implementation.md`:

| Phase | Focus | Key Use Cases |
|-------|-------|---------------|
| **MVP** | Core property management | UC-14 (Auth), UC-27 (Organizations), UC-15 (Buildings), UC-01-13 (Core features) |
| **Phase 2** | Financial & Reporting | UC-16-18 |
| **Phase 3** | Modern Tech | UC-19-26 (AI/ML, IoT, Real-time) |
| **Phase 4** | Rental & Real Estate | UC-29-34, UC-44-51 (Reality Portal) |

> **Note:** Detailed prioritization and MVP scope in `docs/testability-and-implementation.md`

### System Boundaries

| Capability | Property Management (api-server) | Reality Portal (reality-server) |
|------------|----------------------------------|--------------------------------|
| **Users** | Managers, Owners, Tenants | Portal Users, Realtors, Agencies |
| **Access** | Authenticated only | Public + Authenticated |
| **Use Cases** | UC-01 to UC-43 | UC-44 to UC-51, UC-31-32 (shared) |
| **Auth Model** | OAuth Provider | SSO Consumer (from api-server) |
| **Rendering** | SPA (React) | SSR/SSG (Next.js) for SEO |

**Shared Database:** Both servers share PostgreSQL but have distinct access patterns. Reality Portal has read-heavy public queries; Property Management has write-heavy transactional operations.

### Critical Non-Functional Requirements

These NFRs are non-negotiable. Full details in `docs/non-functional-requirements.md`.

| Requirement | Target | Consequence of Missing |
|-------------|--------|------------------------|
| **P95 Latency** | < 200ms | User abandonment, poor UX |
| **Uptime** | 99.9% | 8.76h max downtime/year |
| **Error Rate** | < 0.1% | Trust erosion |
| **GDPR Compliance** | 100% | Legal liability, fines |
| **WCAG 2.1 AA** | Full compliance | Accessibility lawsuits, exclusion |
| **Reality Portal LCP** | < 2.5s | SEO ranking penalty |

### Actor Model Summary

Full actor definitions in `docs/use-cases.md`. Key hierarchy:

```
Platform Level
└── Super Administrator (global platform admin)

Organization Level (per housing cooperative / property management company)
├── Organization Admin
├── Manager (property management rep)
└── Technical Manager (maintenance staff)

Unit Level (per apartment/unit)
├── Owner → Owner Delegate (delegated rights)
├── Tenant (renter)
├── Resident (family member, roommate)
└── Property Manager (short-term rental) → Guest

Reality Portal Level
├── Portal User (anonymous browser)
└── Agency → Agency Owner / Agency Manager / Realtor
```

**Authorization Rule:** All operations are scoped to tenant context. Cross-tenant data access is forbidden.

### Adoption Considerations

| Concern | Approach |
|---------|----------|
| **Data Migration** | Import tools for common formats (Excel, CSV). Historical data preserved. See `docs/technical-design.md` for migration APIs. |
| **Training** | Onboarding tour (UC-42.1), contextual help (UC-42.2), video tutorials (UC-42.3) built into the platform |
| **Offline Access** | Mobile apps support offline mode with background sync (UC-19.6, UC-19.7) |
| **Gradual Rollout** | Can onboard building-by-building, not all-or-nothing |
| **Accounting Integration** | Syncs with POHODA, Money S3, and other regional systems (UC-22.2) |

### Business Context

**Target Segments:**
- Housing cooperatives (bytové družstvá) - 50-500 units
- Property management companies (správcovské spoločnosti) - managing multiple buildings
- Individual landlords with 5+ units
- Real estate agencies seeking integrated listing management

**Monetization Model:**
Platform subscription with tiers based on unit count. Details in business plan (outside PRD scope).

**Success Metrics (Measurable):**

| Metric | Baseline | Target | Measurement |
|--------|----------|--------|-------------|
| Manager response time to faults | TBD (measure at pilot) | 50% reduction | Fault lifecycle tracking |
| Owner app engagement | N/A (new) | 60% monthly active | Analytics |
| Meter reading submission rate | ~40% on-time (industry) | 85% on-time | Submission timestamps |
| Listing time-to-publish | ~2 hours (manual) | < 10 minutes | Listing creation flow |

> **Note:** Baseline metrics to be established during pilot phase.

## Project Classification

| Attribute | Value |
|-----------|-------|
| **Technical Type** | Multi-platform SaaS |
| **Domain** | PropTech (Property Management + Real Estate) |
| **Complexity** | High |
| **Project Context** | Brownfield - comprehensive documentation exists |

**Complexity Drivers:** Multi-tenant architecture, GDPR compliance, financial transactions, 99.9% uptime SLA, multi-region scalability path.

## Source of Truth Documents

| Document | Purpose | Location |
|----------|---------|----------|
| Use Cases | 508 use cases across 51 categories | `docs/use-cases.md` |
| Functional Requirements | Inputs, outputs, business rules | `docs/functional-requirements.md` |
| Non-Functional Requirements | Performance, security, scalability, SEO | `docs/non-functional-requirements.md` |
| Architecture | System design, ADRs, service boundaries | `docs/architecture.md` |
| Technical Design | API endpoints, DTOs, state machines | `docs/technical-design.md` |
| Domain Model | Entities, aggregates, relationships | `docs/domain-model.md` |

## Success Criteria

### User Success

Success is achieved when users experience tangible improvements over their current fragmented workflows.

| User Type | Current State | Success State | "Aha!" Moment |
|-----------|---------------|---------------|---------------|
| **Owner** | Visits manager office for balance info, attends meetings to vote, uses separate portal to list property | Checks balance on phone, votes remotely, lists property in 3 clicks | "I voted from my couch and saw results in real-time" |
| **Tenant** | Reports faults via email/phone with no visibility, tracks rent separately | Reports fault with photo, sees status updates, views rent history | "I reported the leak and got notified when the plumber was scheduled" |
| **Property Manager** | Juggles 5+ systems, re-enters data manually, uses WhatsApp for communication | Single dashboard, automated notifications, AI-assisted meter readings | "I processed 200 meter readings in 10 minutes instead of 2 days" |
| **Realtor** | Creates listings manually on 4+ portals, updates each separately | One-click listing from property data, multi-portal sync | "I updated the price once and it changed everywhere" |

**User Success Metrics:**

| Metric | Baseline | Target | Measurement Method |
|--------|----------|--------|-------------------|
| Task completion rate | N/A | > 90% for core flows | Analytics: funnel completion |
| Time to report fault | ~10 min (phone/email) | < 2 min (app) | Flow timing analytics |
| Remote voting participation | ~30% (in-person only) | > 70% | Vote submission records |
| Meter reading submission on-time | ~40% (industry avg) | > 85% | Submission timestamps |
| User-reported satisfaction (NPS) | N/A | > 40 | Quarterly surveys |

### Business Success

**Phase-Based Success Milestones:**

| Timeframe | Milestone | Success Indicator |
|-----------|-----------|-------------------|
| **3 months (MVP)** | Pilot launch | 3-5 pilot organizations, 500+ units, core features stable |
| **6 months** | Early adoption | 15+ organizations, 3,000+ units, < 5% churn |
| **12 months** | Market validation | 50+ organizations, 15,000+ units, positive unit economics |
| **24 months** | Growth phase | 200+ organizations, 50,000+ units, Reality Portal live |

**Key Business Metrics:**

| Metric | Year 1 Target | Year 3 Target | Measurement |
|--------|---------------|---------------|-------------|
| Organizations onboarded | 50 | 500 | CRM tracking |
| Total units managed | 15,000 | 250,000 | Platform data |
| Monthly recurring revenue (MRR) | €25,000 | €300,000 | Billing system |
| Customer acquisition cost (CAC) | < €500/org | < €300/org | Marketing spend / new orgs |
| Lifetime value (LTV) | > €3,000/org | > €5,000/org | Revenue / churn |
| LTV:CAC ratio | > 6:1 | > 10:1 | Calculated |
| Net revenue retention | > 100% | > 110% | Expansion revenue |
| Churn rate (monthly) | < 3% | < 1% | Cancellations / total |

**Reality Portal Business Metrics (Phase 4+):**

| Metric | Year 2 Target | Year 3 Target |
|--------|---------------|---------------|
| Active listings | 5,000 | 50,000 |
| Registered agencies | 50 | 300 |
| Monthly unique visitors | 100,000 | 500,000 |
| Inquiry conversion rate | > 5% | > 8% |

### Technical Success

Technical success ensures the platform can deliver on user and business promises.

**Performance Targets (from NFR):**

| Metric | Target | Alert Threshold | Source |
|--------|--------|-----------------|--------|
| API P95 latency | < 200ms | > 500ms | `docs/non-functional-requirements.md` |
| API P99 latency | < 500ms | > 1s | NFR |
| Uptime | 99.9% | < 99.5% | NFR |
| Error rate (5xx) | < 0.1% | > 1% | NFR |
| Reality Portal LCP | < 2.5s | > 3s | NFR |
| Reality Portal FCP | < 1.5s | > 2s | NFR |
| Database query P95 | < 100ms | > 200ms | NFR |

**Scalability Targets:**

| Metric | MVP | Year 1 | Year 3 | Source |
|--------|-----|--------|--------|--------|
| Concurrent users | 500 | 5,000 | 15,000 | NFR |
| API requests/sec | 100 | 1,000 | 5,000 | NFR |
| Database size | 10 GB | 100 GB | 500 GB | NFR |
| File storage | 50 GB | 500 GB | 2 TB | NFR |

**Security & Compliance:**

| Requirement | Target | Validation Method | Source |
|-------------|--------|-------------------|--------|
| GDPR compliance | 100% | Annual audit | NFR |
| Data export (UC-23.4) | < 24h | Automated testing | FR |
| Data deletion (UC-23.5) | < 72h | Automated testing | FR |
| WCAG 2.1 AA | Full | Accessibility audit | NFR |
| Penetration testing | Annual | Third-party audit | NFR |
| SOC 2 Type II | Year 2 | Certification | NFR |

**Code Quality:**

| Metric | Target | Measurement |
|--------|--------|-------------|
| Test coverage | > 80% | CI pipeline |
| Critical bugs in production | 0 | Bug tracking |
| Mean time to recovery (MTTR) | < 30 min | Incident logs |
| Deployment frequency | Daily capable | CI/CD metrics |

### Measurable Outcomes

**North Star Metrics:**

| System | North Star | Why It Matters |
|--------|------------|----------------|
| **Property Management** | Monthly Active Organizations (MAO) | Measures real adoption, not just signups |
| **Reality Portal** | Listings with inquiries / month | Measures marketplace health |

**Leading Indicators (predict success):**

| Indicator | Target | Predicts |
|-----------|--------|----------|
| 7-day activation rate | > 60% | Long-term retention |
| Features used per session | > 3 | Engagement depth |
| Manager login frequency | > 3x/week | Platform stickiness |
| Owner app installs / org | > 40% | User adoption |

**Lagging Indicators (confirm success):**

| Indicator | Target | Confirms |
|-----------|--------|----------|
| 90-day retention | > 85% | Product-market fit |
| Expansion revenue % | > 20% | Value delivery |
| Referral rate | > 30% | User satisfaction |
| Support ticket volume/user | Decreasing | UX improvement |

## Product Scope

### MVP - Minimum Viable Product

**Goal:** Prove core value proposition with 3-5 pilot organizations.

**MVP Use Cases (from `docs/testability-and-implementation.md`):**

| Category | Use Cases | Priority |
|----------|-----------|----------|
| **Authentication** | UC-14.1-14.12 (Registration, Login, Password, Sessions) | P1 |
| **Organizations** | UC-27.1-27.8 (Multi-tenancy, Org management) | P1 |
| **Buildings** | UC-15.1-15.10 (Building, Unit, Resident management) | P1 |
| **Announcements** | UC-02.1-02.12 (Create, View, Comment) | P1 |
| **Faults** | UC-03.1-03.12 (Report, Track, Resolve) | P1 |
| **Voting** | UC-04.1-04.12 (Create, Vote, Results) | P1 |
| **Messages** | UC-05.1-05.8 (Direct messaging) | P2 |
| **Documents** | UC-08.1-08.10 (Upload, Share, Organize) | P2 |
| **Notifications** | UC-01.1-01.6 (Push, Email, Preferences) | P1 |

**MVP Exclusions (explicitly out of scope):**
- AI/ML features (Phase 3)
- IoT integrations (Phase 3)
- Reality Portal (Phase 4)
- Short-term rental / Airbnb integration (Phase 4)
- Financial transactions / payments (Phase 2)
- Advanced reporting (Phase 2)

**MVP Success Criteria:**
- [ ] 3+ organizations actively using the platform
- [ ] 500+ units managed
- [ ] < 5 critical bugs in first 30 days
- [ ] > 50% of owners have logged in at least once
- [ ] > 80% of reported faults tracked through resolution
- [ ] NPS > 30 from pilot users

### Growth Features (Post-MVP)

**Phase 2: Financial & Reporting (Months 4-8)**

| Category | Use Cases | Business Value |
|----------|-----------|----------------|
| Financial Management | UC-16.1-16.15 | Payment tracking, invoicing |
| Reports | UC-17.1-17.12 | Operational insights |
| Admin Tools | UC-18.1-18.10 | Platform management |
| Person-Months | UC-10.1-10.6 | Fee allocation basis |
| Self-Readings | UC-11.1-11.8 | Meter management |

**Phase 2 Success Criteria:**
- [ ] Payment reminders reduce late payments by 30%
- [ ] Report generation time < 5 seconds
- [ ] 90% of meter readings submitted digitally

**Phase 3: Modern Technology (Months 9-14)**

| Category | Use Cases | Business Value |
|----------|-----------|----------------|
| Real-time Features | UC-19.1-19.10 | Live updates, notifications |
| AI/ML Features | UC-20.1-20.12 | Automation, predictions |
| IoT Integration | UC-21.1-21.10 | Smart building data |
| External Integrations | UC-22.1-22.10 | Accounting, calendar |
| Security & Compliance | UC-23.1-23.8 | GDPR, 2FA |
| Accessibility | UC-25.1-25.6 | WCAG compliance |
| Workflow Automation | UC-26.1-26.8 | Process automation |

**Phase 3 Success Criteria:**
- [ ] OCR meter reading accuracy > 95%
- [ ] AI chatbot resolves > 40% of queries without human
- [ ] 2FA adoption > 50% of managers

### Vision (Future)

**Phase 4: Reality Portal & Rental (Months 15-24)**

| Category | Use Cases | Business Value |
|----------|-----------|----------------|
| Short-term Rental | UC-29.1-29.12 | Airbnb/Booking integration |
| Guest Registration | UC-30.1-30.10 | Legal compliance |
| Real Estate Listings | UC-31.1-31.12 | Property marketplace |
| Portal Integration | UC-32.1-32.8 | Multi-portal sync |
| Tenant Screening | UC-33.1-33.8 | Risk reduction |
| Lease Management | UC-34.1-34.10 | Contract lifecycle |
| Reality Portal | UC-44-51 | Public marketplace |

**Long-term Vision (Year 3+):**

| Capability | Description | Dependency |
|------------|-------------|------------|
| Multi-region deployment | EU → Global expansion | Scale validation |
| White-label offering | Platform for other property managers | API maturity |
| Marketplace integrations | Insurance, maintenance vendors | User base |
| Predictive analytics | Maintenance forecasting, price optimization | Data volume |
| Mobile-native Reality Portal | KMP apps for iOS/Android | Reality Portal success |

**Vision Success Criteria:**
- [ ] Reality Portal in top 5 Slovak real estate portals by traffic
- [ ] > 50% of listings come from Property Management users
- [ ] International expansion to Czech Republic
- [ ] Platform profitable with positive EBITDA

## User Journeys

### Journey 1: Eva Horáková - The Overwhelmed Property Manager

**Who She Is:**
Eva, 42, has managed 8 residential buildings (320 units) for a housing cooperative in Bratislava for 12 years. She's competent but exhausted - her phone never stops, her inbox overflows with tenant complaints, and annual meter reading season gives her nightmares. She dreams of actually leaving work at 5 PM.

**Her Current Pain:**
Every morning, Eva opens five different applications: email for tenant communication, Excel for meter readings, a legacy DOS-based accounting system, WhatsApp for urgent messages, and paper folders for documents. She spends 2 hours daily just copying data between systems. Last month, she missed a critical water leak report buried in her inbox - the damage cost €15,000 to repair.

**How PPT Changes Her Story:**

Eva's colleague from another cooperative mentions PPT at a conference. Skeptical but desperate, she signs up for a pilot. On her first Monday with PPT, she opens a single dashboard and sees:
- 3 new fault reports (prioritized by AI - the leak is flagged as urgent)
- 12 meter reading submissions overnight (OCR already extracted values)
- 2 voting sessions ending this week (participation tracking shows she needs to remind Building 3)

The breakthrough moment comes during annual meter reading season. Instead of her usual 3-week marathon of calling residents, transcribing photos, and fixing OCR errors, Eva sends one push notification. Within a week, 85% of readings are in - verified by AI, ready for billing export to POHODA.

**Six months later:** Eva leaves work at 5 PM most days. She handles 15% more units with less stress. Her cooperative's NPS has jumped from 23 to 48 because issues get resolved before residents complain twice.

**Journey Requirements Revealed:**
- Unified dashboard with prioritized task view
- AI-powered fault categorization and prioritization
- OCR meter reading with confidence scoring
- Push notification campaigns
- Voting participation tracking
- Accounting system integration (POHODA export)
- Mobile app for on-site inspections

---

### Journey 2: Ján Kováč - The Frustrated Owner Who Wants Answers

**Who He Is:**
Ján, 58, is a retired engineer who owns a 3-room apartment in a Košice cooperative where he's lived for 25 years. He's detail-oriented, slightly grumpy, and believes the building management is hiding information from him. He attends every building meeting, always with questions.

**His Current Pain:**
Ján wants to know exactly where his €180 monthly fees go. The annual financial report is a 40-page PDF that arrives by post - he can't find the breakdown for elevator maintenance costs that seem suspiciously high. When he calls the manager, she's never available. When he emails, responses take weeks. He suspects something is wrong but can't prove it.

**How PPT Changes His Story:**

Ján's daughter Zuzana downloads PPT for him and sets up his account. Initially dismissive ("Another app I don't need"), he opens it after receiving a push notification about a vote on elevator renovation.

For the first time, he can:
- See his payment history and exactly what each line item covers
- Read the elevator maintenance contract and vendor invoices (documents section)
- View the proposed renovation vote with cost breakdown AND vote from his phone
- Ask a question in the vote discussion thread instead of waiting for a meeting

The aha moment: During the meeting he's watching from his living room (hybrid attendance), his question about elevator costs gets answered with a linked document. He votes "Yes" with confidence for the first time - he understands what he's approving.

**Three months later:** Ján has stopped calling the manager with complaints. He checks his balance weekly, reads announcements when notified, and has become a PPT evangelist to other skeptical retirees in his building. His satisfaction score went from "likely to complain" to "promoter."

**Journey Requirements Revealed:**
- Transparent financial breakdown per unit
- Document access with version history
- Remote/hybrid voting with discussion threads
- Push notifications for votes and announcements
- Question/comment functionality on announcements
- Mobile-friendly interface for older users
- Balance and payment history view

---

### Journey 3: Michaela Novotná - The Tenant Who Just Wants Things Fixed

**Who She Is:**
Michaela, 29, is a marketing specialist who moved to Žilina for work. She rents a studio apartment and has zero interest in building politics - she just wants functioning heating, hot water, and a working elevator. She's never met her landlord and wouldn't recognize the property manager if they passed on the street.

**Her Current Pain:**
Two weeks ago, her bathroom tap started leaking. She called the number on the building notice board - voicemail. Emailed the address she found online - auto-reply saying the manager is on vacation. Texted her landlord - he said "contact the manager." The leak is getting worse, she's putting a bucket under it, and she's frustrated enough to consider moving when her lease ends.

**How PPT Changes Her Story:**

Michaela finds a QR code on the building's notice board linking to PPT. She downloads the app, creates an account with her lease number, and within 2 minutes:
- Takes 3 photos of the leaking tap
- Selects "Plumbing" from the fault category
- Adds a note: "Leak getting worse, bucket overflowing every 4 hours"
- Hits submit

She receives a push notification 4 hours later: "Plumber scheduled for tomorrow 2-4 PM." The next day, another notification: "Fault resolved - tap replaced." She rates the experience 5 stars.

**The breakthrough:** When her heating fails in November, she doesn't panic - she opens PPT, reports the fault, and gets a callback within 30 minutes. The technical manager was already in the building fixing another unit. Michaela renews her lease.

**Journey Requirements Revealed:**
- QR code onboarding flow for tenants
- Photo attachment for fault reports
- Fault category selection (guided)
- Real-time status updates via push
- Scheduled appointment notifications
- Fault resolution confirmation and rating
- Minimal data collection (privacy for tenants)

---

### Journey 4: Martin Hlinka - The Realtor Racing Against Time

**Who He Is:**
Martin, 35, works for a mid-sized real estate agency in Bratislava. He's been in the business for 5 years and manages about 40 active listings at any time. He's ambitious, tech-savvy, and frustrated by manual work that keeps him from what he does best: showing properties and closing deals.

**His Current Pain:**
When a client lists their 2-bedroom apartment for sale, Martin spends 2 hours:
- Visiting the property to take measurements (the seller doesn't know the exact m²)
- Taking 50 photos, then editing down to 20
- Writing descriptions for 4 different portals (each has different character limits)
- Uploading to Nehnuteľnosti.sk, Reality.sk, Bazos.sk, and his agency website
- When the seller drops the price by €5,000, he updates all 4 portals manually

Last week, he forgot to update one portal - a buyer saw the old price and complained to his manager.

**How PPT Changes His Story:**

Martin's agency partners with a housing cooperative using PPT. When an owner in that cooperative wants to sell, Martin receives an inquiry through Reality Portal. He opens the listing creation wizard and:
- Property data auto-populates: 68m², 3rd floor, built 1985, gas heating
- Unit history shows: new windows in 2019, renovated bathroom in 2021
- Energy certificate is already uploaded from building documents
- He adds his 20 photos and a description

He hits "Publish to all portals" - Reality Portal, Nehnuteľnosti.sk, and Reality.sk all get the listing simultaneously. When the seller wants to drop the price, Martin changes it once - all portals update within minutes.

**The aha moment:** A buyer asks about building financials before making an offer. Instead of playing phone tag with the manager, Martin shares a "building info package" generated directly from PPT - maintenance fund balance, planned renovations, no outstanding debts. The buyer makes an offer the same day.

**Journey Requirements Revealed:**
- Property data pre-population from unit records
- Document inheritance from building (energy certificates, etc.)
- Multi-portal syndication (one-click publish)
- Price sync across portals
- Building info package generation
- Inquiry management through Reality Portal
- Agency dashboard for listing management

---

### Journey 5: Super Admin Lucia - Platform Operations at Scale

**Who She Is:**
Lucia, 38, is the technical operations lead at the company behind PPT. She's responsible for platform health, onboarding new organizations, and handling escalated issues. She manages a team of 3 support agents and reports to the CTO.

**Her Typical Day:**

8:00 AM - Lucia opens her admin dashboard and sees:
- 47 organizations active (12,500 units total)
- 3 new organization signups pending verification
- 1 escalated support ticket: "Manager locked out, forgot 2FA recovery"
- System health: All green, P95 latency at 145ms

She processes the new signups:
- Housing cooperative "BD Ružinov 47" - verifies business registration, approves
- Property management company "Správa SK" - flags for additional verification (unusually large unit count claim)
- Individual landlord - auto-approved based on low unit count

The escalated ticket requires her to:
- Verify the manager's identity through organization admin
- Issue a temporary 2FA bypass code with 1-hour expiry
- Log the incident for security audit

**Afternoon emergency:** At 2 PM, she gets an alert - one organization's API usage is spiking (possible bug or abuse). She:
- Views the organization's recent activity log
- Identifies a misconfigured integration that's polling every second instead of every minute
- Contacts the organization admin with a fix suggestion
- Temporarily rate-limits the organization to protect platform stability

**Journey Requirements Revealed:**
- Platform health dashboard (latency, uptime, usage)
- Organization lifecycle management (signup, verification, suspension)
- Multi-organization view with search and filtering
- Support ticket escalation workflow
- 2FA recovery procedures with audit logging
- Per-organization usage monitoring and rate limiting
- Activity log access for troubleshooting
- Security incident logging

---

### Journey 6: Edge Case - Voting Dispute Resolution

**The Scenario:**
A building in Trenčín holds a vote on whether to renovate the common areas (estimated cost: €80,000, special assessment of €400 per unit). The vote closes with 51% in favor, but 3 owners dispute the results, claiming their votes weren't counted.

**The Journey:**

**Hour 0 - Vote Closes:**
The building manager, Peter, sees the final tally: 51% for, 46% against, 3% abstain. He prepares to announce the results, but immediately receives 3 messages claiming "my vote wasn't counted."

**Hour 1 - Investigation Begins:**
Peter opens the voting audit log in PPT and sees:
- Owner A: Vote cast at 14:32, "For" - counted ✓
- Owner B: Vote cast at 23:58 (1 minute before close), "Against" - counted ✓
- Owner C: No vote recorded

He contacts Owner C, who insists they voted. Peter checks the activity log:
- Owner C logged in at 22:45
- Opened voting page at 22:47
- No "submit vote" action recorded
- Session timeout at 23:15

**Resolution:**
Peter calls Owner C, who realizes they were interrupted by a phone call and forgot to hit "Submit." The vote was never cast. Peter shares the timestamped activity log (anonymized) with the disputing owners. Owners A and B accept their votes were counted; Owner C acknowledges their error.

**Outcome:**
The vote result stands. Peter generates a compliance report with full audit trail for the organization's records. No lawyers needed.

**Journey Requirements Revealed:**
- Timestamped vote audit logs (immutable)
- Activity log per user (login, actions, session events)
- Vote status visibility (cast vs. started vs. never opened)
- Anonymized audit log export for disputes
- Compliance report generation
- Vote close timestamp enforcement

---

### Journey Requirements Summary

| Journey | User Type | Key Capability Areas |
|---------|-----------|---------------------|
| Eva (Property Manager) | Manager | Dashboard, AI prioritization, OCR, notifications, integrations |
| Ján (Owner) | Owner | Transparency, documents, remote voting, mobile access |
| Michaela (Tenant) | Tenant | Fault reporting, status tracking, minimal friction |
| Martin (Realtor) | Realtor | Property data sync, multi-portal publishing, inquiry management |
| Lucia (Super Admin) | Platform Ops | Health monitoring, org management, support escalation, security |
| Voting Dispute (Edge) | Manager/Owner | Audit logs, activity tracking, compliance reporting |

**Capabilities by Priority:**

| Priority | Capabilities | Source Journeys |
|----------|--------------|-----------------|
| **P1 - MVP** | Fault reporting, voting, notifications, basic dashboard, mobile access | Eva, Ján, Michaela |
| **P2 - Growth** | AI prioritization, OCR, document management, integrations | Eva, Ján |
| **P3 - Scale** | Multi-portal sync, audit logs, platform ops dashboard | Martin, Lucia, Voting Dispute |
| **P4 - Vision** | Reality Portal, agency management, inquiry routing | Martin |

## Domain-Specific Requirements

### PropTech Compliance & Regulatory Overview

The Property Management System operates in the PropTech domain with specific regulatory requirements across multiple jurisdictions (Slovakia, Czech Republic, Germany, Austria). The platform handles sensitive personal data (residents, owners, tenants), financial transactions, and must comply with EU-wide regulations.

**Domain Complexity Drivers:**
- Multi-tenant SaaS with organization-level data isolation
- Financial transactions (payments, invoices, fund management)
- Personal data processing under GDPR
- Accessibility requirements for public-facing services
- Real estate listing regulations per country

### Key Domain Concerns

#### 1. Data Protection & Privacy (GDPR)

**Regulatory Context:**
As an EU-based platform processing personal data of EU residents, PPT must comply with GDPR (General Data Protection Regulation 2016/679).

**Key Requirements:**

| Requirement | Use Case | Implementation |
|-------------|----------|----------------|
| **Lawful Basis** | All data processing | Consent for optional features, legitimate interest for core operations, contract for service delivery |
| **Data Minimization** | UC-all | Collect only what's necessary for each function |
| **Right to Access** | UC-23.4 | Export all personal data within 24 hours |
| **Right to Erasure** | UC-23.5 | Delete personal data within 72 hours (with legal retention exceptions) |
| **Data Portability** | UC-23.4 | Machine-readable export format (JSON/CSV) |
| **Breach Notification** | NFR | Notify authorities within 72 hours, affected users without undue delay |
| **Privacy by Design** | Architecture | Data protection built into system design, not bolted on |

**Implementation Approach:**
- Privacy settings per user with granular controls
- Consent management for each data processing activity
- Audit logging for all personal data access
- Automated data retention policies
- DPO (Data Protection Officer) contact in platform

#### 2. Financial Compliance

**Regulatory Context:**
PPT processes financial data (payment records, invoices, fund balances) but is not a payment processor or financial institution.

**Key Requirements:**

| Requirement | Scope | Implementation |
|-------------|-------|----------------|
| **No Payment Processing** | Phase 2+ | PPT tracks payments but does not process credit cards directly - integrates with licensed payment providers (if needed) |
| **Financial Records Retention** | Slovak law | 10-year retention for accounting documents |
| **Audit Trail** | All financial transactions | Immutable logs with timestamps, user IDs, IP addresses |
| **Invoicing Compliance** | Slovak/EU | Valid invoice format per local tax regulations |
| **Fund Transparency** | Building maintenance funds | Clear breakdown of contributions and expenditures |

**Implementation Approach:**
- Integration with accounting systems (POHODA, Money S3) rather than replacing them
- Read-only financial displays for owners (source of truth remains in accounting system)
- Export capabilities for auditors

#### 3. Multi-Tenant Data Isolation

**Regulatory Context:**
Each organization (housing cooperative, property management company) must have complete data isolation. Cross-tenant data access would be a severe compliance and trust violation.

**Key Requirements:**

| Requirement | Implementation |
|-------------|----------------|
| **Tenant Context in Every Query** | Database queries always include organization_id filter |
| **No Cross-Tenant Joins** | API layer enforces tenant boundaries before database |
| **Tenant ID in Audit Logs** | All operations logged with tenant context |
| **Separate Encryption Keys** | Per-tenant encryption for sensitive data (Phase 2+) |
| **Tenant Admin Cannot Access Other Tenants** | Role-based access strictly scoped |

**Implementation Approach:**
- Middleware extracts and validates tenant context from JWT
- Database row-level security policies (PostgreSQL RLS)
- Automated testing for cross-tenant access attempts
- Penetration testing specifically targeting tenant isolation

#### 4. Accessibility (WCAG 2.1 AA)

**Regulatory Context:**
Public-facing web applications in the EU should comply with Web Accessibility Directive (EU 2016/2102). While PPT is a private platform, accessibility ensures:
- Elderly owners can use the platform
- Users with disabilities are not excluded
- Legal risk mitigation for public sector clients

**Key Requirements:**

| WCAG Criterion | PPT Implementation |
|----------------|-------------------|
| **Perceivable** | Alt text for images, sufficient color contrast, resizable text |
| **Operable** | Keyboard navigation, no time limits (or extendable), skip links |
| **Understandable** | Consistent navigation, error identification, input assistance |
| **Robust** | Valid HTML, ARIA labels, works with assistive technologies |

**Implementation Approach:**
- Accessibility audit before MVP launch
- Automated accessibility testing in CI (axe-core)
- Manual testing with screen readers (NVDA, VoiceOver)
- Accessibility statement in platform

#### 5. Real Estate & Listing Regulations

**Regulatory Context:**
Reality Portal publishes property listings, which are subject to advertising regulations and real estate industry standards.

**Key Requirements:**

| Requirement | Country | Implementation |
|-------------|---------|----------------|
| **Truthful Advertising** | All | Listings must accurately represent property (m², amenities) |
| **Energy Certificates** | EU | Mandatory display of energy rating for sales/rentals |
| **Anti-Discrimination** | EU | No discriminatory language in listings |
| **Agency Licensing** | SK/CZ | Verify realtor credentials before allowing listings |
| **Price Transparency** | SK | Display price with VAT status clearly |

**Implementation Approach:**
- Property data validation against registered records
- Required fields for energy certificate, price, size
- Content moderation for discriminatory language (AI-assisted)
- Agency verification workflow before onboarding

### Compliance Requirements Matrix

| Regulation | Scope | Phase | Validation Method |
|------------|-------|-------|-------------------|
| **GDPR** | All | MVP | Annual audit, automated testing |
| **Slovak Data Protection Act** | SK users | MVP | Legal review, DPO oversight |
| **ePrivacy Directive** | Cookies, tracking | MVP | Consent banner, preference center |
| **WCAG 2.1 AA** | All web interfaces | MVP | Accessibility audit |
| **Real Estate Advertising** | Reality Portal | Phase 4 | Content moderation |
| **Financial Records Retention** | Financial module | Phase 2 | 10-year archival |
| **Building Management Act (SK)** | Voting, meetings | MVP | Legal compliance review |

### Industry Standards & Best Practices

| Standard | Description | PPT Application |
|----------|-------------|-----------------|
| **ISO 27001** | Information security management | Target for Year 2 certification |
| **SOC 2 Type II** | Service organization controls | Target for Year 2 audit |
| **OWASP Top 10** | Web application security | Security testing against all 10 categories |
| **OpenAPI 3.1** | API specification | All APIs documented, SDK generated |
| **OAuth 2.0 / OIDC** | Authentication standard | api-server as OAuth provider |

### Required Expertise & Validation

**Domain Expertise Needed:**

| Expertise | Role | When Needed |
|-----------|------|-------------|
| **Data Protection Officer (DPO)** | GDPR compliance oversight | Before MVP launch |
| **Legal Counsel (SK/CZ)** | Contract templates, terms of service | Before MVP launch |
| **Accessibility Consultant** | WCAG audit | Before MVP launch |
| **Security Auditor** | Penetration testing | Before MVP launch, then annually |
| **Real Estate Industry Advisor** | Reality Portal compliance | Before Phase 4 |

**Validation Milestones:**

| Milestone | Validation | Timing |
|-----------|------------|--------|
| MVP Launch | GDPR compliance review, security audit, accessibility audit | Pre-launch |
| Phase 2 (Financial) | Accounting integration certification | Pre-release |
| Phase 3 (AI/ML) | AI model bias testing, OCR accuracy validation | Pre-release |
| Phase 4 (Reality Portal) | Real estate advertising compliance review | Pre-release |
| Annual | Penetration testing, GDPR audit, accessibility review | Ongoing |

### Implementation Considerations

**Architecture Implications:**

| Concern | Architectural Response |
|---------|------------------------|
| GDPR data portability | Export API with comprehensive data retrieval |
| GDPR erasure | Soft delete with 30-day recovery, then hard delete |
| Multi-tenant isolation | PostgreSQL RLS, tenant context middleware |
| Audit logging | Append-only audit log table, separate from operational data |
| Accessibility | Design system with accessibility built-in |

**Development Process Implications:**

| Concern | Process Response |
|---------|------------------|
| Privacy by Design | Privacy impact assessment for new features |
| Security | Security review in PR checklist, automated SAST |
| Accessibility | Accessibility testing in CI, manual review for new components |
| Compliance | Compliance officer sign-off for features touching regulated areas |

**Operational Implications:**

| Concern | Operational Response |
|---------|---------------------|
| Breach notification | Incident response runbook, 72-hour SLA |
| Data subject requests | Self-service for export/delete, manual escalation path |
| Audit readiness | Continuous audit logging, quarterly log review |
| Accessibility | User feedback channel for accessibility issues |

## Innovation & Novel Patterns

### Detected Innovation Areas

PPT introduces several innovative patterns that differentiate it from traditional property management and real estate platforms:

#### 1. Property-to-Market Fusion Architecture

**Innovation:** Unified data model spanning property operations and real estate transactions.

**What's Novel:**
Traditional property management systems and real estate portals are separate products from different vendors. PPT's architecture assumes from day one that a managed unit may become a listing, and a listing may become a managed unit - with no data migration required.

| Traditional Approach | PPT Innovation |
|---------------------|----------------|
| Separate databases for PM and RE | Single PostgreSQL with shared entities |
| Manual data re-entry to list property | One-click listing from unit record |
| No visibility into building health for buyers | Building info package with maintenance history |
| Owner switches between 3+ apps | Single platform for entire lifecycle |

**Validation Approach:**
- Pilot with 2-3 organizations that do both PM and RE
- Measure time-to-list for units already in system vs. external listings
- Track buyer inquiry-to-close conversion with building transparency

**Risk Mitigation:**
- If fusion proves too complex, systems can operate independently (shared DB doesn't require shared UI)
- Fallback: manual listing creation still available

#### 2. AI/IoT as First-Class Data Model Citizens

**Innovation:** AI capabilities embedded directly in database schema, not as external service calls.

**What's Novel:**
Most property management platforms add AI as integrations - calling external APIs for OCR, NLP, predictions. PPT embeds AI outputs directly in the data model:

| Entity | AI Column | Purpose |
|--------|-----------|---------|
| `meter_reading` | `ocr_extracted_value`, `ocr_confidence` | Automated meter reading extraction |
| `fault` | `ai_category`, `ai_priority_suggestion` | Intelligent fault triage |
| `message` | `sentiment_score` | Resident satisfaction tracking |
| `equipment` | `predicted_failure_date` | Predictive maintenance |

**Benefits:**
- AI improves over time using organization-specific data
- Historical AI predictions are auditable
- No external API latency for AI-enhanced queries
- AI features work even if third-party service changes

**Validation Approach:**
- OCR accuracy testing: 95% target on meter reading photos
- A/B test AI-prioritized fault queue vs. manual prioritization
- Track sentiment trends against NPS scores

**Risk Mitigation:**
- AI columns are optional - system works without them
- Human override always available for AI suggestions
- Fallback to basic CRUD if AI features underperform

#### 3. Unified Multi-Tenant + Multi-Language Platform

**Innovation:** Same codebase serves cooperatives in Slovakia, Czech Republic, Germany, and Austria with full localization.

**What's Novel:**
Most PropTech solutions are single-country, requiring separate vendors per market. PPT's multi-tenant architecture supports:
- Per-organization language preference
- Multi-currency display (EUR, CZK)
- Country-specific compliance (Slovak Building Management Act, German WEG)
- Regional integrations (POHODA SK, Money S3 CZ)

**Validation Approach:**
- Pilot in one country first (Slovakia)
- Expand to Czech Republic with same codebase
- Measure localization effort vs. building separate systems

**Risk Mitigation:**
- Modular compliance: country-specific rules isolated in configuration
- Language files separate from business logic
- Can operate single-country if multi-country proves too complex

### Market Context & Competitive Landscape

**Current Market State (Central Europe):**

| Segment | Dominant Players | Gap PPT Fills |
|---------|------------------|---------------|
| Property Management | Domus (SK), Domea (CZ) | Legacy systems, no mobile, no AI |
| Real Estate Portals | Nehnuteľnosti.sk, Sreality.cz | No connection to PM systems |
| Smart Building | Siemens, Honeywell | Enterprise-only, expensive |
| AI in Real Estate | Zillow (US), Zoopla (UK) | Not available in CEE market |

**Competitive Moat:**
- First-mover in PM + RE fusion for Central Europe
- Local compliance built-in (not localized from US/UK product)
- AI accessible to small cooperatives, not just enterprises

**Research Needed (Future):**
- Detailed competitive analysis of regional players
- User research on willingness to switch from legacy systems
- Integration partnership opportunities (banks, utilities)

### Validation Approach

| Innovation | Validation Method | Success Metric | Timeline |
|------------|-------------------|----------------|----------|
| PM + RE Fusion | Pilot with mixed-use cooperative | > 30% of listings from PM users | Month 6 |
| AI Meter Reading | OCR accuracy testing | > 95% accuracy | Month 9 |
| AI Fault Triage | A/B test with pilot orgs | 30% faster resolution | Month 12 |
| Multi-Country | Czech expansion pilot | Same codebase, < 20% localization effort | Month 18 |

### Risk Mitigation

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| **Fusion too complex** | Medium | High | Decouple PM and RE if needed - shared DB doesn't require shared UI |
| **AI accuracy insufficient** | Medium | Medium | Human override always available; train on more data |
| **Multi-country compliance burden** | Low | High | Start single-country; expand only when validated |
| **Market not ready for fusion** | Low | High | Can market PM and RE as separate products to different segments |
| **Legacy system integration fails** | Medium | Medium | Build import tools; don't require full migration |

### Innovation vs. Execution Balance

PPT's innovation is **architectural**, not **feature-based**. The individual features (fault reporting, voting, listings) exist in other products. The innovation is in:

1. **How they're connected** (unified data model)
2. **How AI is embedded** (first-class citizen, not integration)
3. **Who can access them** (small cooperatives, not just enterprises)

This means:
- Lower technical risk than pure innovation plays
- Faster time to market (proven features, novel architecture)
- Competitive moat is structural, not easily copied

## Multi-Platform SaaS Specific Requirements

### Project-Type Overview

PPT is a multi-platform B2B SaaS product serving the PropTech vertical. It combines:
- **SaaS B2B**: Multi-tenant architecture, organization-level permissions, subscription billing
- **Web Application**: React SPA (ppt-web), Next.js SSR (reality-web)
- **Mobile Application**: React Native (property management), Kotlin Multiplatform (Reality Portal)

This hybrid nature requires addressing requirements across all three project types while maintaining architectural consistency.

### Multi-Tenancy Model

**Tenant Definition:**
A tenant is an **Organization** - a housing cooperative, property management company, or individual landlord.

| Aspect | Implementation |
|--------|----------------|
| **Tenant Isolation** | Row-level security (PostgreSQL RLS) with `organization_id` in every table |
| **Tenant Context** | Extracted from JWT, validated in middleware, passed to all queries |
| **Cross-Tenant Access** | Architecturally forbidden - no admin override |
| **Tenant Provisioning** | Self-service signup with verification workflow |
| **Tenant Offboarding** | Data export + 30-day retention + permanent deletion |

**Tenant Hierarchy:**
```
Organization (Tenant)
├── Buildings[]
│   ├── Units[]
│   │   ├── Residents[]
│   │   └── Assignments[]
│   └── Common Areas[]
├── Users[]
│   └── Roles[]
├── Documents[]
├── Votes[]
└── Faults[]
```

**Shared Resources (Cross-Tenant):**
- Super Administrator accounts (platform level)
- System configuration
- Feature flags
- Billing/subscription data

### Role-Based Access Control (RBAC) Matrix

**Role Hierarchy:**

| Level | Role | Scope | Description |
|-------|------|-------|-------------|
| Platform | Super Administrator | All tenants | Platform operations, support |
| Organization | Organization Admin | Single tenant | Full org management |
| Organization | Manager | Single tenant | Day-to-day operations |
| Organization | Technical Manager | Single tenant | Maintenance focus |
| Building | Building Admin | Single building | Building-specific admin |
| Unit | Owner | Own unit(s) | Unit owner capabilities |
| Unit | Owner Delegate | Delegated unit(s) | Acting on behalf of owner |
| Unit | Tenant | Leased unit | Renter capabilities |
| Unit | Resident | Associated unit | Family member, roommate |
| Portal | Portal User | Public | Anonymous browsing |
| Portal | Realtor | Agency listings | Real estate agent |
| Portal | Agency Manager | Agency | Agency operations |
| Portal | Agency Owner | Agency | Agency administration |

**Permission Matrix (Key Operations):**

| Operation | Super Admin | Org Admin | Manager | Tech Mgr | Owner | Tenant |
|-----------|-------------|-----------|---------|----------|-------|--------|
| Create Organization | ✓ | - | - | - | - | - |
| Manage Org Settings | ✓ | ✓ | - | - | - | - |
| Add/Remove Users | ✓ | ✓ | ✓ | - | - | - |
| Create Announcements | ✓ | ✓ | ✓ | ✓ | - | - |
| Create Votes | ✓ | ✓ | ✓ | - | - | - |
| Cast Vote | - | ✓ | - | - | ✓ | - |
| Report Fault | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Resolve Fault | ✓ | ✓ | ✓ | ✓ | - | - |
| View Own Balance | - | ✓ | - | - | ✓ | ✓ |
| View All Balances | ✓ | ✓ | ✓ | - | - | - |
| Export Personal Data | ✓ | ✓ | ✓ | - | ✓ | ✓ |
| Delete Account | ✓ | ✓ | - | - | ✓ | ✓ |

**Permission Implementation:**
- Permissions stored as role → permission mappings
- Role assignments scoped to organization/building/unit
- Permission checks in API middleware before business logic
- Audit log for all permission-sensitive operations

### Subscription & Billing Model

**Subscription Tiers (Conceptual):**

| Tier | Unit Limit | Features | Target Segment |
|------|------------|----------|----------------|
| **Starter** | ≤ 50 units | Core PM (faults, votes, announcements) | Individual landlords |
| **Professional** | ≤ 200 units | + Financial, Reports, Integrations | Small cooperatives |
| **Business** | ≤ 500 units | + AI/ML, IoT, Priority Support | Mid-size PM companies |
| **Enterprise** | Unlimited | + Custom integrations, SLA, Dedicated support | Large PM companies |

**Billing Considerations:**
- Per-unit pricing model
- Monthly/annual billing cycles
- Upgrade/downgrade prorated
- Usage-based add-ons (AI features, storage)

> **Note:** Detailed pricing in business plan (outside PRD scope). Platform must support flexible billing models.

**Technical Billing Requirements:**
- Subscription state machine (trial → active → past_due → cancelled)
- Usage metering for add-on features
- Invoice generation with local tax compliance
- Payment provider integration (Stripe, local alternatives)
- Dunning management for failed payments

### Integration Architecture

**Integration Categories:**

| Category | Integrations | Priority | Implementation |
|----------|--------------|----------|----------------|
| **Accounting** | POHODA, Money S3, custom CSV | Phase 2 | REST API export, file generation |
| **Calendar** | Google Calendar, Outlook | Phase 3 | OAuth + API |
| **Communication** | Email (SendGrid), SMS, Push | MVP | Provider abstraction layer |
| **Real Estate Portals** | Nehnuteľnosti.sk, Reality.sk | Phase 4 | Portal-specific adapters |
| **Short-term Rental** | Airbnb, Booking.com | Phase 4 | iCal sync, API where available |
| **Document Signing** | DocuSign, local alternatives | Phase 3 | Embedded signing flow |
| **Payment** | Stripe, GoPay (SK), Comgate (CZ) | Phase 2 | Provider abstraction |

**Integration Patterns:**

| Pattern | Use Case | Implementation |
|---------|----------|----------------|
| **Webhook Inbound** | Real-time events from external systems | Signature verification, idempotency |
| **Webhook Outbound** | Notify external systems of PPT events | Retry with exponential backoff |
| **OAuth Consumer** | Access third-party APIs on behalf of user | Token storage, refresh handling |
| **File Export** | Generate files for import elsewhere | Async job queue, download link |
| **iCal Sync** | Calendar synchronization | Standard iCal format |

**Integration Security:**
- API keys stored encrypted
- OAuth tokens per-user, not per-org
- Webhook signatures verified
- Rate limiting on outbound calls
- Integration audit logging

### Compliance Requirements Summary

| Requirement | Scope | Implementation | Validation |
|-------------|-------|----------------|------------|
| **GDPR** | All EU users | Privacy by design, consent, export/delete | Annual audit |
| **Multi-Tenancy** | All data | RLS, tenant context, no cross-access | Pen testing |
| **Authentication** | All users | OAuth 2.0, 2FA optional, session management | Security audit |
| **Authorization** | All operations | RBAC, permission checks, audit logging | Code review |
| **Data Retention** | All data | Configurable per data type, legal minimums | Policy review |
| **Accessibility** | Web interfaces | WCAG 2.1 AA | Accessibility audit |

### Technical Architecture Considerations

**Backend Architecture:**

| Component | Technology | Rationale |
|-----------|------------|-----------|
| API Server | Rust (Axum) | Performance, safety, async |
| Reality Server | Rust (Axum) | Consistency, shared code |
| Database | PostgreSQL | RLS, JSON, full-text search |
| Cache | Redis | Session, rate limiting, pub/sub |
| Queue | Redis/RabbitMQ | Background jobs, webhooks |
| Search | PostgreSQL FTS / Meilisearch | Listing search (Reality Portal) |
| File Storage | S3-compatible | Documents, images |

**Frontend Architecture:**

| App | Technology | Rendering | Rationale |
|-----|------------|-----------|-----------|
| ppt-web | React (Vite) | SPA | Rich interactivity, offline capable |
| reality-web | Next.js | SSR/SSG | SEO, performance, public pages |
| mobile | React Native | Native | Cross-platform, code sharing |
| mobile-native | Kotlin Multiplatform | Native | Performance, platform features |

**API Strategy:**

| Aspect | Approach |
|--------|----------|
| **Specification** | OpenAPI 3.1 (TypeSpec source) |
| **Versioning** | URI-based (/api/v1/), max 2 active versions |
| **Authentication** | OAuth 2.0 / OIDC (api-server as provider) |
| **Authorization** | JWT with tenant context, permission claims |
| **Rate Limiting** | Per-tenant, per-endpoint limits |
| **SDK Generation** | TypeScript (hey-api), Kotlin (openapi-generator) |

### Implementation Considerations

**Development Workflow:**

| Phase | Focus | Key Deliverables |
|-------|-------|------------------|
| Foundation | Auth, multi-tenancy, core entities | api-server skeleton, ppt-web shell |
| MVP | Core PM features | Faults, votes, announcements, notifications |
| Growth | Financial, integrations | Payments, accounting sync, reports |
| Scale | AI/ML, IoT, Reality Portal | OCR, predictions, listings, portal |

**DevOps Requirements:**

| Requirement | Implementation |
|-------------|----------------|
| **CI/CD** | GitHub Actions, automated testing, deployment |
| **Environments** | Dev, Staging, Production (per-region) |
| **Monitoring** | Prometheus metrics, Grafana dashboards |
| **Logging** | Structured JSON, centralized (ELK/Loki) |
| **Alerting** | PagerDuty/Opsgenie for critical issues |
| **Backup** | Automated daily, tested recovery |

**Scalability Path:**

| Scale Point | Trigger | Action |
|-------------|---------|--------|
| 100 orgs | Year 1 | Single region, vertical scaling |
| 500 orgs | Year 2 | Read replicas, CDN, caching |
| 2000 orgs | Year 3 | Horizontal scaling, multi-region |
| 5000+ orgs | Year 4+ | Sharding by region, edge deployment |

## Scoping & MVP Strategy

### MVP Strategy & Philosophy

**Platform MVP Philosophy:**
PPT takes a **Problem-Solving MVP** approach rather than a feature-minimal approach. The MVP must solve the complete core problem for a small set of pilot organizations - not partially solve problems for many users.

**The Core Problem Being Solved:**
Property managers juggle 5+ disconnected systems. Owners lack visibility. Tenants can't get answers. The MVP must eliminate this fragmentation for core workflows.

**What MVP Includes:**
- Complete fault reporting → resolution workflow
- Complete voting workflow with audit trail
- Complete announcement/communication workflow
- Basic document management
- User authentication with org-level isolation

**What MVP Excludes (Even If Users Want It):**
- Financial transactions (requires compliance, payment integration)
- AI/ML features (requires data volume, model training)
- Reality Portal (requires PM success first)
- IoT integrations (requires hardware partnerships)

### Resource Requirements by Phase

| Phase | Duration | Team Size | Key Roles |
|-------|----------|-----------|-----------|
| **MVP** | 4 months | 4-5 | 2 Full-stack (Rust/React), 1 Mobile, 1 DevOps, 0.5 PM |
| **Phase 2** | 4 months | 6-7 | + 1 Backend (Financial), 1 Frontend |
| **Phase 3** | 6 months | 8-10 | + 1 ML Engineer, 1 IoT Specialist |
| **Phase 4** | 6 months | 10-12 | + 2 Full-stack (Reality Portal), 1 KMP |

### MVP Feature Set

**Priority 1 - Authentication & Multi-tenancy:**
- UC-14.1-14.12: User registration, login, password management, session handling
- UC-27.1-27.8: Organization management, multi-tenant data isolation

**Priority 1 - Core Property Management:**
- UC-15.1-15.10: Building management, unit management, resident associations
- UC-01.1-01.6: Notification system (push, email, preferences)
- UC-02.1-02.12: Announcements (create, view, comment, pin)
- UC-03.1-03.12: Fault reporting (submit, track, resolve, rate)
- UC-04.1-04.12: Voting (create, vote, discuss, results, audit)

**Priority 2 - Communication & Documents:**
- UC-05.1-05.8: Direct messaging between residents and managers
- UC-08.1-08.10: Document upload, sharing, organization

### Post-MVP Features

**Phase 2 - Financial & Reporting:**
- UC-10.1-10.6: Person-months tracking
- UC-11.1-11.8: Meter self-readings
- UC-16.1-16.15: Financial management
- UC-17.1-17.12: Reporting and analytics
- UC-18.1-18.10: Platform administration

**Phase 3 - Modern Technology:**
- UC-19.1-19.10: Real-time features, WebSocket updates
- UC-20.1-20.12: AI/ML (chatbot, OCR, predictions, sentiment)
- UC-21.1-21.10: IoT integrations, smart building data
- UC-22.1-22.10: External integrations (accounting, calendar)
- UC-23.1-23.8: Security & GDPR compliance features
- UC-25.1-25.6: WCAG accessibility compliance
- UC-26.1-26.8: Workflow automation

**Phase 4 - Reality Portal & Rental:**
- UC-29.1-29.12: Short-term rental (Airbnb/Booking)
- UC-30.1-30.10: Guest registration system
- UC-31.1-31.12: Real estate listings
- UC-32.1-32.8: Multi-portal syndication
- UC-33.1-33.8: Tenant screening
- UC-34.1-34.10: Lease management
- UC-44-51: Reality Portal features

### Progressive Development Roadmap

| Milestone | Timeline | Key Deliverables | Success Criteria |
|-----------|----------|------------------|------------------|
| **Foundation** | Month 1-2 | Auth, multi-tenancy, core entities, API skeleton | 0 security vulnerabilities, 100% tenant isolation tests pass |
| **MVP Beta** | Month 3 | Core PM features, mobile app | 3 internal testers using daily |
| **MVP Launch** | Month 4 | Pilot deployment, bug fixes | 3-5 pilot orgs, 500+ units |
| **Phase 2 Start** | Month 5 | Financial module design | Integration specs approved |
| **Phase 2 Complete** | Month 8 | Financial features, reporting | Accounting export working |
| **Phase 3 Start** | Month 9 | AI/ML infrastructure | OCR prototype > 90% accuracy |
| **Phase 3 Complete** | Month 14 | AI features, IoT ready | AI chatbot handling 40% queries |
| **Phase 4 Start** | Month 15 | Reality Portal design | UX approved |
| **Phase 4 Complete** | Month 20 | Reality Portal launch | 5,000 listings, 50 agencies |

### Risk Mitigation Strategy

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| **Pilot org churn** | Medium | High | Weekly check-ins, rapid bug fixes, success manager |
| **Scope creep in MVP** | High | Medium | Strict backlog grooming, "not MVP" tag, PM veto power |
| **Multi-tenancy bugs** | Low | Critical | Automated cross-tenant tests, pen testing |
| **Team scaling delays** | Medium | Medium | Start hiring for Phase 2 in Month 3 |
| **Integration complexity** | Medium | Medium | Accounting integration as Phase 2 priority, not MVP |
| **AI model underperformance** | Medium | Low | Human fallback always available |

### Scoping Decisions

| Capability | Decision | Rationale |
|------------|----------|-----------|
| **Financial transactions** | Phase 2, not MVP | Requires payment provider integration, compliance |
| **AI meter reading** | Phase 3, not MVP | Needs training data from MVP usage |
| **Reality Portal** | Phase 4 | Depends on PM success and data volume |
| **White-label** | Year 3+ | Requires API maturity and proven scale |
| **Multi-region** | Year 2+ | Single region until 500+ orgs |

> **Reference:** Full use case details in `docs/use-cases.md`, implementation prioritization in `docs/testability-and-implementation.md`

## Functional Requirements

> **Source of Truth:** Complete functional requirements for all 508 use cases are defined in `docs/functional-requirements.md`. This section provides a capability-area summary with traceability to use case IDs.

### Capability Area 1: Identity & Access Management

| FR | Capability | Use Cases |
|----|------------|-----------|
| FR1 | Users can register accounts with email verification | UC-14.1, UC-14.2 |
| FR2 | Users can authenticate via email/password with session management | UC-14.3, UC-14.4 |
| FR3 | Users can reset forgotten passwords securely | UC-14.5, UC-14.6 |
| FR4 | Users can enable two-factor authentication | UC-23.1 |
| FR5 | Users can manage active sessions across devices | UC-14.11, UC-14.12 |
| FR6 | Platform operators can manage user lifecycle (invite, suspend, delete) | UC-14.7-14.10 |
| FR7 | Users can authenticate via SSO from Property Management to Reality Portal | UC-46.3 |

### Capability Area 2: Organization & Multi-Tenancy

| FR | Capability | Use Cases |
|----|------------|-----------|
| FR8 | Organizations can be created with complete isolation from other tenants | UC-27.1 |
| FR9 | Organization admins can configure organization settings and branding | UC-27.2, UC-27.3 |
| FR10 | Organization admins can manage members and assign roles | UC-27.4, UC-27.5 |
| FR11 | Organizations can define custom role permissions within RBAC framework | UC-27.6 |
| FR12 | Super admins can view and manage all organizations | UC-18.1-18.10 |
| FR13 | Organizations can export all their data for migration | UC-27.7 |
| FR14 | Organizations can be deactivated with data retention per policy | UC-27.8 |

### Capability Area 3: Property & Resident Management

| FR | Capability | Use Cases |
|----|------------|-----------|
| FR15 | Managers can create and configure buildings with address and metadata | UC-15.1 |
| FR16 | Managers can define units within buildings with ownership/rental status | UC-15.2 |
| FR17 | Managers can associate residents (owners, tenants) with units | UC-15.3, UC-15.4 |
| FR18 | Owners can delegate rights to other users | UC-28.1-28.4 |
| FR19 | Managers can track person-months for fee allocation | UC-10.1-10.6 |
| FR20 | Residents can view their unit details and associated information | UC-15.5 |
| FR21 | Managers can manage common areas and shared facilities | UC-15.6 |

### Capability Area 4: Communication & Notifications

| FR | Capability | Use Cases |
|----|------------|-----------|
| FR22 | Managers can create announcements visible to specific buildings/units | UC-02.1-02.4 |
| FR23 | Residents can view, comment on, and acknowledge announcements | UC-02.5-02.8 |
| FR24 | Managers can pin important announcements | UC-02.9 |
| FR25 | Users can send direct messages to other users within their organization | UC-05.1-05.4 |
| FR26 | Users receive push notifications for relevant events | UC-01.1-01.3 |
| FR27 | Users can configure notification preferences by channel and category | UC-01.4-01.6 |
| FR28 | System can send email notifications for offline users | UC-01.2 |
| FR29 | Users can view neighbor information based on privacy settings | UC-06.1-06.4 |

### Capability Area 5: Issue & Fault Management

| FR | Capability | Use Cases |
|----|------------|-----------|
| FR30 | Residents can report faults with description, category, and photos | UC-03.1-03.3 |
| FR31 | Managers can view, triage, and assign faults to technical staff | UC-03.4-03.6 |
| FR32 | Technical managers can update fault status through resolution workflow | UC-03.7-03.9 |
| FR33 | Residents can track status of their reported faults | UC-03.10 |
| FR34 | Residents can rate resolved faults | UC-03.11 |
| FR35 | System can suggest fault category and priority based on description (AI-assisted) | UC-20.3 |
| FR36 | Managers can generate fault reports and analytics | UC-17.4 |

### Capability Area 6: Voting & Decision Making

| FR | Capability | Use Cases |
|----|------------|-----------|
| FR37 | Managers can create votes with multiple question types (yes/no, multiple choice, ranked) | UC-04.1-04.3 |
| FR38 | Owners can cast votes during voting period | UC-04.4 |
| FR39 | Owners can delegate voting rights | UC-28.2 |
| FR40 | Users can discuss votes in associated threads | UC-04.5 |
| FR41 | System calculates and displays results based on configured quorum | UC-04.6-04.7 |
| FR42 | System maintains immutable audit trail for all voting activity | UC-04.8 |
| FR43 | Managers can generate compliance reports for votes | UC-04.9 |
| FR44 | System supports hybrid attendance (in-person + remote) | UC-04.10 |

### Capability Area 7: Document Management

| FR | Capability | Use Cases |
|----|------------|-----------|
| FR45 | Users can upload documents with metadata and categorization | UC-08.1-08.3 |
| FR46 | Managers can organize documents in folder structures | UC-08.4 |
| FR47 | Users can view documents based on their access permissions | UC-08.5 |
| FR48 | System maintains version history for documents | UC-08.6 |
| FR49 | Users can search documents by content and metadata | UC-08.7 |
| FR50 | Managers can share documents with specific users or groups | UC-08.8 |
| FR51 | System can extract text from uploaded documents (OCR) | UC-20.2 |

### Capability Area 8: Financial Management (Phase 2)

| FR | Capability | Use Cases |
|----|------------|-----------|
| FR52 | Owners can view their payment history and current balance | UC-16.1-16.3 |
| FR53 | Managers can record payments and generate invoices | UC-16.4-16.6 |
| FR54 | Managers can manage building maintenance fund | UC-16.7-16.9 |
| FR55 | System can generate financial reports by period, building, category | UC-17.1-17.3 |
| FR56 | System can export financial data to accounting systems (POHODA, Money S3) | UC-22.2 |
| FR57 | Managers can send payment reminders | UC-16.10 |

### Capability Area 9: Meter Readings & Utilities (Phase 2)

| FR | Capability | Use Cases |
|----|------------|-----------|
| FR58 | Residents can submit meter readings with photos | UC-11.1-11.3 |
| FR59 | System can extract meter values from photos (OCR) | UC-20.1 |
| FR60 | Managers can view and validate submitted readings | UC-11.4-11.5 |
| FR61 | System can detect anomalous readings | UC-11.6 |
| FR62 | Managers can generate utility reports | UC-11.7 |
| FR63 | System can track outages and service interruptions | UC-12.1-12.4 |

### Capability Area 10: AI & Automation (Phase 3)

| FR | Capability | Use Cases |
|----|------------|-----------|
| FR64 | Users can interact with AI chatbot for common questions | UC-20.4 |
| FR65 | System can analyze message sentiment for trend detection | UC-20.5 |
| FR66 | System can predict maintenance needs based on equipment age and history | UC-20.6 |
| FR67 | System can summarize long documents automatically | UC-20.7 |
| FR68 | System can provide smart search with natural language queries | UC-20.8 |
| FR69 | Managers can configure workflow automations | UC-26.1-26.4 |
| FR70 | System can trigger automated actions based on events | UC-26.5-26.8 |

### Capability Area 11: IoT & Smart Building (Phase 3)

| FR | Capability | Use Cases |
|----|------------|-----------|
| FR71 | System can ingest data from IoT sensors | UC-21.1-21.3 |
| FR72 | Users can view real-time sensor data dashboards | UC-21.4 |
| FR73 | System can alert on threshold violations | UC-21.5 |
| FR74 | System can correlate sensor data with fault reports | UC-21.6 |
| FR75 | Managers can configure sensor thresholds and alerts | UC-21.7 |

### Capability Area 12: Real Estate & Listings (Phase 4)

| FR | Capability | Use Cases |
|----|------------|-----------|
| FR76 | Owners can create property listings from existing unit data | UC-31.1-31.3 |
| FR77 | Realtors can manage listings with photos, descriptions, pricing | UC-31.4-31.6 |
| FR78 | System can syndicate listings to external portals | UC-32.1-32.4 |
| FR79 | Portal users can search and filter property listings | UC-44.1-44.4 |
| FR80 | Portal users can save favorite listings | UC-44.5-44.8 |
| FR81 | Portal users can contact listing agents | UC-45.1-45.4 |
| FR82 | Agencies can manage realtors and shared listings | UC-49.1-49.6 |
| FR83 | Realtors can import listings from external sources | UC-50.1-50.4 |

### Capability Area 13: Rental Management (Phase 4)

| FR | Capability | Use Cases |
|----|------------|-----------|
| FR84 | Property managers can sync with Airbnb/Booking.com | UC-29.1-29.4 |
| FR85 | Property managers can register guests for legal compliance | UC-30.1-30.4 |
| FR86 | System can generate guest reports for authorities | UC-30.5-30.6 |
| FR87 | Landlords can screen potential tenants | UC-33.1-33.4 |
| FR88 | Landlords can manage lease lifecycle (create, renew, terminate) | UC-34.1-34.6 |
| FR89 | System can track lease expirations and send reminders | UC-34.7-34.8 |

### Capability Area 14: Compliance & Privacy

| FR | Capability | Use Cases |
|----|------------|-----------|
| FR90 | Users can export all their personal data (GDPR) | UC-23.4 |
| FR91 | Users can request deletion of their personal data (GDPR) | UC-23.5 |
| FR92 | Users can configure privacy settings for profile visibility | UC-23.1-23.3 |
| FR93 | System maintains audit logs for compliance-sensitive operations | UC-23.6 |
| FR94 | Managers can generate compliance reports | UC-23.7 |
| FR95 | System enforces data retention policies per regulation | UC-23.8 |

### Capability Area 15: Platform Operations

| FR | Capability | Use Cases |
|----|------------|-----------|
| FR96 | Super admins can view platform health metrics | UC-18.1 |
| FR97 | Super admins can manage feature flags | UC-18.2 |
| FR98 | Super admins can broadcast system announcements | UC-18.3 |
| FR99 | Support staff can access organization data for troubleshooting | UC-18.4 |
| FR100 | System provides onboarding tour for new users | UC-42.1 |
| FR101 | System provides contextual help and documentation | UC-42.2 |

### Functional Requirements Summary

| Capability Area | FR Count | MVP | Phase 2 | Phase 3 | Phase 4 |
|-----------------|----------|-----|---------|---------|---------|
| Identity & Access | 7 | ✓ | | | |
| Organization & Multi-Tenancy | 7 | ✓ | | | |
| Property & Resident Management | 7 | ✓ | | | |
| Communication & Notifications | 8 | ✓ | | | |
| Issue & Fault Management | 7 | ✓ | | | |
| Voting & Decision Making | 8 | ✓ | | | |
| Document Management | 7 | ✓ | | | |
| Financial Management | 6 | | ✓ | | |
| Meter Readings & Utilities | 6 | | ✓ | | |
| AI & Automation | 7 | | | ✓ | |
| IoT & Smart Building | 5 | | | ✓ | |
| Real Estate & Listings | 8 | | | | ✓ |
| Rental Management | 6 | | | | ✓ |
| Compliance & Privacy | 6 | ✓ | | | |
| Platform Operations | 6 | ✓ | | | |
| **Total** | **101** | **57** | **12** | **12** | **14** |

> **Complete Details:** Each use case referenced above has full functional requirements (inputs, outputs, business rules, preconditions, postconditions) defined in `docs/functional-requirements.md`

## Non-Functional Requirements

> **Source of Truth:** Comprehensive NFRs are defined in `docs/non-functional-requirements.md`. This section summarizes critical quality attributes with traceability.

### Performance

| Requirement | Target | Alert Threshold | Measurement |
|-------------|--------|-----------------|-------------|
| **API P95 Latency** | < 200ms | > 500ms | APM monitoring |
| **API P99 Latency** | < 500ms | > 1s | APM monitoring |
| **Database Query P95** | < 100ms | > 200ms | Query analytics |
| **Reality Portal LCP** | < 2.5s | > 3s | Core Web Vitals |
| **Reality Portal FCP** | < 1.5s | > 2s | Core Web Vitals |
| **Mobile App Launch** | < 3s | > 5s | App analytics |
| **Push Notification Delivery** | < 5s | > 15s | Delivery tracking |

**Capacity Targets:**

| Metric | MVP | Year 1 | Year 3 |
|--------|-----|--------|--------|
| Concurrent Users | 500 | 5,000 | 15,000 |
| API Requests/sec | 100 | 1,000 | 5,000 |
| WebSocket Connections | 1,000 | 10,000 | 50,000 |

### Security

**Authentication & Authorization:**

| Requirement | Specification |
|-------------|---------------|
| Password Hashing | Argon2id with secure parameters |
| Session Management | JWT with 15-minute access, 7-day refresh |
| 2FA Support | TOTP (Google Authenticator), SMS backup |
| OAuth 2.0 Provider | api-server serves as OAuth provider for SSO |
| Rate Limiting | 100 req/min per user, 1000 req/min per org |

**Data Protection:**

| Requirement | Specification |
|-------------|---------------|
| Encryption at Rest | AES-256 for all database fields containing PII |
| Encryption in Transit | TLS 1.3 for all API communication |
| Multi-Tenant Isolation | PostgreSQL RLS with organization_id |
| Audit Logging | Immutable logs for all sensitive operations |
| Secret Management | No secrets in code, vault-based key management |

**Compliance:**

| Standard | Requirement | Validation |
|----------|-------------|------------|
| GDPR | Full compliance | Annual audit |
| Data Export (UC-23.4) | < 24 hours | Automated testing |
| Data Deletion (UC-23.5) | < 72 hours | Automated testing |
| Penetration Testing | Annual | Third-party audit |
| SOC 2 Type II | Year 2 target | External certification |

### Scalability

**Horizontal Scaling Path:**

| Scale Point | Users | Organizations | Strategy |
|-------------|-------|---------------|----------|
| MVP | 500 | 5 | Single region, vertical |
| Year 1 | 5,000 | 50 | Read replicas, CDN |
| Year 2 | 15,000 | 200 | Horizontal pods, caching |
| Year 3 | 50,000 | 500 | Multi-region, sharding |

**Resource Scaling:**

| Resource | MVP | Year 1 | Year 3 |
|----------|-----|--------|--------|
| Database Size | 10 GB | 100 GB | 500 GB |
| File Storage | 50 GB | 500 GB | 2 TB |
| Redis Cache | 1 GB | 10 GB | 50 GB |

**Elasticity Requirements:**
- Auto-scale API pods at 70% CPU utilization
- Scale-to-zero for non-production environments
- Handle 10x traffic spikes within 5 minutes

### Reliability

| Requirement | Target | Measurement |
|-------------|--------|-------------|
| **Uptime SLA** | 99.9% | External monitoring |
| **Max Planned Downtime** | 4h/month | Maintenance windows |
| **Error Rate (5xx)** | < 0.1% | APM monitoring |
| **MTTR** | < 30 minutes | Incident logs |
| **RTO** | < 4 hours | DR testing |
| **RPO** | < 1 hour | Backup frequency |

**Backup & Recovery:**

| Data Type | Backup Frequency | Retention |
|-----------|------------------|-----------|
| Database | Hourly | 30 days |
| File Storage | Daily | 90 days |
| Audit Logs | Real-time replication | 7 years |
| Configuration | On change | 1 year |

**Disaster Recovery:**
- Multi-AZ deployment in primary region
- Cross-region backup replication
- Automated failover for database
- Documented DR runbooks with quarterly testing

### Accessibility

| Standard | Requirement | Validation |
|----------|-------------|------------|
| **WCAG 2.1 AA** | Full compliance | Accessibility audit |
| **Keyboard Navigation** | All features accessible | Manual testing |
| **Screen Reader** | NVDA, VoiceOver compatible | Manual testing |
| **Color Contrast** | 4.5:1 minimum ratio | Automated testing |
| **Text Scaling** | Up to 200% without loss | Manual testing |

**Accessibility Testing:**
- axe-core in CI pipeline for automated checks
- Quarterly manual audit with assistive technologies
- User feedback channel for accessibility issues
- Accessibility statement in platform footer

### Integration

**External System Requirements:**

| Integration | Protocol | SLA | Retry Policy |
|-------------|----------|-----|--------------|
| POHODA Export | File/REST | Async | 3 retries, exponential backoff |
| Money S3 Export | CSV | Async | 3 retries, exponential backoff |
| Nehnuteľnosti.sk | REST API | 99% | 5 retries, circuit breaker |
| Reality.sk | REST API | 99% | 5 retries, circuit breaker |
| Airbnb iCal | iCal sync | Best effort | Hourly sync |
| SendGrid Email | REST API | 99.9% | Queue-based |
| FCM Push | REST API | 99.9% | Queue-based |

**API Requirements:**

| Requirement | Specification |
|-------------|---------------|
| API Versioning | URI-based (/api/v1/, /api/v2/) |
| Max Active Versions | 2 |
| Deprecation Window | 12 months |
| Rate Limiting | Per-tenant, per-endpoint |
| SDK Generation | TypeScript, Kotlin |
| OpenAPI Spec | 3.1 |

### Observability

**Monitoring Requirements:**

| Component | Tool | Alerting |
|-----------|------|----------|
| APM | Prometheus + Grafana | PagerDuty |
| Logs | Structured JSON → ELK/Loki | Log-based alerts |
| Traces | OpenTelemetry | Trace-based alerts |
| Uptime | External monitor | SMS + PagerDuty |

**SLO Dashboard:**
- Real-time P95 latency by endpoint
- Error rate trends
- Uptime percentage
- Active user count
- Organization health scores

### Localization

| Requirement | Specification |
|-------------|---------------|
| **Languages** | Slovak (sk), Czech (cs), German (de), English (en) |
| **Date/Time** | Localized per user preference |
| **Currency** | EUR, CZK display |
| **Number Format** | Locale-aware |
| **Translation** | i18n framework, professional translation |

### Mobile-Specific

| Requirement | Specification |
|-------------|---------------|
| **Offline Mode** | Core features available offline |
| **Background Sync** | Queue actions when online |
| **Push Notifications** | FCM (Android), APNs (iOS) |
| **Deep Linking** | Universal links support |
| **App Size** | < 50MB initial download |
| **Battery** | < 3% drain per hour active use |

### NFR Summary by Phase

| Category | MVP Critical | Phase 2+ |
|----------|--------------|----------|
| P95 Latency < 200ms | ✓ | ✓ |
| 99.9% Uptime | ✓ | ✓ |
| GDPR Compliance | ✓ | ✓ |
| Multi-tenant Isolation | ✓ | ✓ |
| WCAG 2.1 AA | ✓ | ✓ |
| 1000 RPS | | ✓ |
| Multi-region | | ✓ |
| SOC 2 Certification | | ✓ |

> **Complete Details:** Full NFR specifications including test criteria in `docs/non-functional-requirements.md`
