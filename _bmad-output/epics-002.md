---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - _bmad-output/prd.md
  - _bmad-output/architecture.md
  - _bmad-output/ux-design-specification.md
  - _bmad-output/epics.md
  - docs/use-cases.md
workflowType: 'epics-and-stories'
lastStep: 4
status: 'complete'
project_name: 'Property Management System (PPT) & Reality Portal'
user_name: 'Martin Janci'
date: '2025-12-22'
continues_from: 'epics.md'
phase_range: '4-completion, 5, 6, 7'
epic_range: '18-27'
---

# Property Management System (PPT) & Reality Portal - Epic Breakdown (Part 2)

## Overview

This document continues from `epics.md` and provides epic and story breakdown for **Phases 4 (completion), 5, 6, and 7** - covering operations, asset management, financial planning, compliance, and Reality Portal enhancements.

**Continuation from:** `epics.md` (Epics 1-17, Phases 1-4 partial)

## Requirements Inventory (New FRs)

### Functional Requirements (FR102-FR125)

**Phase 4 Completion (Existing FRs from epics.md):**
- FR84: Airbnb/Booking.com calendar sync (UC-29)
- FR85: Guest registration for legal compliance (UC-30)
- FR86: Guest reports for authorities (UC-30)
- FR87: Tenant screening - background, credit, references (UC-33)
- FR88: Lease lifecycle management (UC-34)
- FR89: Lease expiration tracking and reminders (UC-34)

**Phase 5: Operations & Asset Management (NEW):**

**CA-16: Maintenance Scheduling (6 FRs)**
- FR102: Managers can create and schedule maintenance work orders (UC-36.1-36.3)
- FR103: Managers can track equipment service history and warranties (UC-36.4-36.5)
- FR104: System can generate preventive maintenance schedules (UC-36.6-36.8)

**CA-17: Supplier/Vendor Management (3 FRs)**
- FR105: Managers can manage vendor contacts and contracts (UC-37.1-37.3)
- FR106: Managers can assign service requests to vendors (UC-37.4-37.5)
- FR107: Managers can process and track vendor invoices (UC-37.6-37.8)

**CA-18: Insurance Management (3 FRs)**
- FR108: Managers can track insurance policies per building (UC-35.1-35.3)
- FR109: Managers can submit and track insurance claims (UC-35.4-35.6)
- FR110: System sends policy renewal reminders (UC-35.7-35.8)

**CA-19: Emergency Management (3 FRs)**
- FR111: Organizations can define emergency protocols and contacts (UC-39.1-39.3)
- FR112: Users can report incidents and track resolution (UC-39.4-39.6)
- FR113: System can broadcast emergency notifications (UC-39.7-39.8)

**Phase 6: Financial Planning & Compliance (NEW):**

**CA-20: Budget & Planning (3 FRs)**
- FR114: Managers can create annual and multi-year budgets (UC-40.1-40.3)
- FR115: Managers can track budget vs actual spending (UC-40.4-40.6)
- FR116: Managers can plan capital improvements with funding sources (UC-40.7-40.8)

**CA-21: Legal & Compliance (3 FRs)**
- FR117: Organizations can manage legal document repository (UC-38.1-38.3)
- FR118: System tracks regulatory compliance requirements (UC-38.4-38.6)
- FR119: Managers can send and track legal notices (UC-38.7-38.8)

**CA-22: Platform Subscription & Billing (3 FRs)**
- FR120: Platform operators can manage subscription plans (UC-41.1-41.4)
- FR121: System tracks usage metrics for billing (UC-41.5-41.8)
- FR122: System generates platform invoices (UC-41.9-41.12)

**Phase 7: Reality Portal Enhancements (NEW):**

**CA-23: Property Comparison (3 FRs)**
- FR123: Portal users can compare properties side-by-side (UC-48.1-48.2)
- FR124: Portal users can view market analytics and trends (UC-48.3-48.4)
- FR125: System provides AI-powered property recommendations (UC-48.5-48.6)

### Non-Functional Requirements (Applicable to New Phases)

**NFR-PERF: Performance (inherited)**
- NFR1-7: Same latency and performance targets

**NFR-NEW: Phase 5-7 Specific**
- NFR41: Work order assignment notification < 30s
- NFR42: Emergency broadcast delivery < 10s (critical)
- NFR43: Budget report generation < 5s
- NFR44: Property comparison load time < 2s
- NFR45: AI recommendations response < 3s

### Additional Requirements

**From Architecture:**
- ARCH29: Work orders integrate with fault management system
- ARCH30: Vendor invoices sync with financial module (Epic 11)
- ARCH31: Emergency broadcasts use existing notification infrastructure (Epic 2B)
- ARCH32: Budget module extends financial reporting (Epic 11)
- ARCH33: Property comparison uses existing listing data (Epic 15, 16)

**From UX Design:**
- UX28: Work order status visible in fault timeline
- UX29: Emergency alerts use distinctive visual treatment (red banner)
- UX30: Budget charts use consistent color coding with financial module
- UX31: Property comparison uses card-based side-by-side layout

### FR Coverage Map

| FR | Epic | Description |
|----|------|-------------|
| FR84 | 18 | Airbnb/Booking.com sync |
| FR85 | 18 | Guest registration |
| FR86 | 18 | Guest reports for authorities |
| FR87 | 19 | Tenant screening |
| FR88 | 19 | Lease lifecycle management |
| FR89 | 19 | Lease expiration tracking |
| FR102 | 20 | Work order creation and scheduling |
| FR103 | 20 | Equipment service history |
| FR104 | 20 | Preventive maintenance schedules |
| FR105 | 21 | Vendor contact and contract management |
| FR106 | 21 | Service request to vendor assignment |
| FR107 | 21 | Vendor invoice processing |
| FR108 | 22 | Insurance policy tracking |
| FR109 | 22 | Claims submission and tracking |
| FR110 | 22 | Policy renewal reminders |
| FR111 | 23 | Emergency protocol definitions |
| FR112 | 23 | Incident reporting and tracking |
| FR113 | 23 | Emergency notification broadcast |
| FR114 | 24 | Budget creation |
| FR115 | 24 | Budget vs actual tracking |
| FR116 | 24 | Capital improvement planning |
| FR117 | 25 | Legal document repository |
| FR118 | 25 | Regulatory compliance tracking |
| FR119 | 25 | Legal notice management |
| FR120 | 26 | Subscription plan management |
| FR121 | 26 | Usage-based metering |
| FR122 | 26 | Platform invoice generation |
| FR123 | 27 | Property comparison |
| FR124 | 27 | Market analytics |
| FR125 | 27 | AI property recommendations |

## Epic List

### Phase 4 Completion

#### Epic 18: Short-Term Rental Integration
**Goal:** Property managers can sync calendars with Airbnb/Booking.com and register guests for legal compliance.

**FRs covered:** FR84, FR85, FR86
**Target Apps:** api-server, ppt-web, mobile (React Native)
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 3 (Buildings/Units), Epic 11 (Financial)

---

#### Epic 19: Lease Management & Tenant Screening
**Goal:** Landlords can screen potential tenants and manage the full lease lifecycle from creation to termination.

**FRs covered:** FR87, FR88, FR89
**Target Apps:** api-server, ppt-web, mobile (React Native)
**Estimate:** 5 stories, ~2 weeks
**Dependencies:** Epic 3 (Buildings/Units), Epic 7A (Documents)

---

### Phase 5: Operations & Asset Management

#### Epic 20: Maintenance Scheduling & Work Orders
**Goal:** Managers can schedule preventive maintenance, create work orders, and track equipment service history.

**FRs covered:** FR102, FR103, FR104
**Target Apps:** api-server, ppt-web, mobile (React Native)
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 4 (Faults), Epic 3 (Buildings)

**Key Decisions:**
- Work orders extend fault management (same timeline UI)
- Equipment registry with warranty tracking
- Recurring schedules using cron-like patterns

---

#### Epic 21: Supplier & Vendor Management
**Goal:** Managers can manage vendor relationships, assign service requests, track contracts, and process invoices.

**FRs covered:** FR105, FR106, FR107
**Target Apps:** api-server, ppt-web
**Estimate:** 4 stories, ~1.5 weeks
**Dependencies:** Epic 20 (Work Orders), Epic 11 (Financial)

**Key Decisions:**
- Vendors linked to work orders
- Invoice integration with financial module
- Contract expiration alerts

---

#### Epic 22: Insurance Management
**Goal:** Managers can track insurance policies, submit and track claims, and receive renewal reminders.

**FRs covered:** FR108, FR109, FR110
**Target Apps:** api-server, ppt-web, mobile (React Native)
**Estimate:** 4 stories, ~1.5 weeks
**Dependencies:** Epic 3 (Buildings), Epic 7A (Documents)

**Key Decisions:**
- Policy documents stored in document management
- Claims linked to faults/incidents
- Multi-policy support per building

---

#### Epic 23: Emergency Management
**Goal:** Organizations can define emergency protocols, manage incidents, and coordinate emergency responses.

**FRs covered:** FR111, FR112, FR113
**Target Apps:** api-server, ppt-web, mobile (React Native)
**Estimate:** 4 stories, ~1.5 weeks
**Dependencies:** Epic 2B (Notifications), Epic 4 (Faults)

**Key Decisions:**
- Emergency broadcasts bypass notification preferences
- Incident tracking extends fault model
- Protocol checklists for response coordination

---

### Phase 6: Financial Planning & Compliance

#### Epic 24: Budget & Financial Planning
**Goal:** Managers can create budgets, track spending against budget, forecast expenses, and plan capital improvements.

**FRs covered:** FR114, FR115, FR116
**Target Apps:** api-server, ppt-web
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 11 (Financial)

**Key Decisions:**
- Budget categories align with financial accounts
- Multi-year planning with inflation adjustment
- Capital reserve fund tracking

---

#### Epic 25: Legal Document & Compliance
**Goal:** Organizations can manage legal documents, track regulatory compliance, and handle legal notices.

**FRs covered:** FR117, FR118, FR119
**Target Apps:** api-server, ppt-web
**Estimate:** 4 stories, ~1.5 weeks
**Dependencies:** Epic 7A (Documents), Epic 6 (Announcements)

**Key Decisions:**
- Legal documents with retention policies
- Compliance checklist templates
- Notice delivery tracking with acknowledgment

---

#### Epic 26: Platform Subscription & Billing
**Goal:** Platform operators can manage organization subscriptions, track usage, and generate invoices.

**FRs covered:** FR120, FR121, FR122
**Target Apps:** api-server, ppt-web (admin panel)
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 10B (Platform Admin), Epic 11 (Financial patterns)

**Key Decisions:**
- Subscription tiers: Free, Professional, Enterprise
- Usage metrics: users, buildings, storage, API calls
- Stripe integration for payment processing

---

### Phase 7: Reality Portal Enhancements

#### Epic 27: Property Comparison & Analytics
**Goal:** Portal users can compare properties side-by-side, view market analytics, and receive AI-powered recommendations.

**FRs covered:** FR123, FR124, FR125
**Target Apps:** reality-server, reality-web, mobile-native (KMP)
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 16 (Portal Search), Epic 15 (Listings)

**Key Decisions:**
- Comparison limited to 4 properties max
- Analytics: price trends, days on market, price per sqm
- AI recommendations based on user search history and favorites

---

## Implementation Targets by Application

| Application | Epics | Technology |
|-------------|-------|------------|
| **api-server** | 18, 19, 20, 21, 22, 23, 24, 25, 26 | Rust/Axum |
| **ppt-web** | 18, 19, 20, 21, 22, 23, 24, 25, 26 | React/Vite |
| **mobile (React Native)** | 18, 19, 20, 22, 23 | React Native |
| **reality-server** | 27 | Rust/Axum |
| **reality-web** | 27 | Next.js |
| **mobile-native (KMP)** | 27 | Kotlin Multiplatform |

## Phase Summary

| Phase | Epics | Stories | FRs | Estimate |
|-------|-------|---------|-----|----------|
| Phase 4 (Complete) | 18, 19 | 9 | 6 | ~4 weeks |
| Phase 5 | 20, 21, 22, 23 | 16 | 12 | ~6.5 weeks |
| Phase 6 | 24, 25, 26 | 12 | 9 | ~5.5 weeks |
| Phase 7 | 27 | 4 | 3 | ~2 weeks |
| **Total** | **10 Epics** | **41 Stories** | **30 FRs** | **~18 weeks** |

## Sprint Plan

### Phase 4 Completion Sprints
| Sprint | Epics | Stories | Rationale |
|--------|-------|---------|-----------|
| 4E | Epic 18 (1-4) | 4 | Short-term rental complete |
| 4F | Epic 19 (1-5) | 5 | Lease management complete |

### Phase 5 Sprints
| Sprint | Epics | Stories | Rationale |
|--------|-------|---------|-----------|
| 5A | Epic 20 (1-4) | 4 | Maintenance foundation |
| 5B | Epic 21 (1-4) | 4 | Vendor management |
| 5C | Epic 22 (1-4) | 4 | Insurance management |
| 5D | Epic 23 (1-4) | 4 | Emergency protocols |

### Phase 6 Sprints
| Sprint | Epics | Stories | Rationale |
|--------|-------|---------|-----------|
| 6A | Epic 24 (1-4) | 4 | Budget planning |
| 6B | Epic 25 (1-4) | 4 | Legal compliance |
| 6C | Epic 26 (1-4) | 4 | Platform billing |

### Phase 7 Sprints
| Sprint | Epics | Stories | Rationale |
|--------|-------|---------|-----------|
| 7A | Epic 27 (1-4) | 4 | Portal enhancements |

---

## Cumulative Project Summary (Including epics.md)

| Phase | Epics | Stories | FRs | Weeks |
|-------|-------|---------|-----|-------|
| Phase 1 (MVP) | 12 | 74 | 67 | ~20 |
| Phase 2 | 5 | 23 | 13 | ~5 |
| Phase 3 | 2 | 13 | 12 | ~4 |
| Phase 4 | 8 | 33 | 14 | ~12 |
| Phase 5 | 4 | 16 | 12 | ~6.5 |
| Phase 6 | 3 | 12 | 9 | ~5.5 |
| Phase 7 | 1 | 4 | 3 | ~2 |
| **Grand Total** | **35 Epics** | **175 Stories** | **130 FRs** | **~55 weeks** |

---

*Stories for each epic are detailed in the sections below.*

---

# Phase 4 Completion Stories

## Epic 18: Short-Term Rental Integration

**Goal:** Property managers can sync calendars with Airbnb/Booking.com and register guests for legal compliance.

### Story 18.1: Platform Connection Setup

As a **property manager**,
I want to **connect my Airbnb and Booking.com accounts**,
So that **I can sync bookings automatically**.

**Acceptance Criteria:**

**Given** a property manager navigates to rental settings
**When** they click "Connect Airbnb"
**Then** they are redirected to Airbnb OAuth flow
**And** upon authorization, the connection is saved
**And** initial calendar sync begins

**Given** an Airbnb connection already exists
**When** they view the connection status
**Then** they see last sync time and connection health
**And** can disconnect if needed

**Technical Notes:**
- Create `rental_platform_connections` table: id, unit_id, platform (airbnb, booking), access_token, refresh_token, external_id, last_sync_at
- OAuth 2.0 flow with Airbnb/Booking APIs
- Store tokens encrypted (AES-256)

---

### Story 18.2: Calendar Synchronization

As a **property manager**,
I want to **see all bookings from connected platforms in one calendar**,
So that **I can manage availability across platforms**.

**Acceptance Criteria:**

**Given** a unit has connected Airbnb and Booking.com
**When** a new booking is made on either platform
**Then** the booking appears in the unified calendar within 15 minutes
**And** the other platform's calendar is blocked for those dates

**Given** a booking is cancelled on one platform
**When** the sync runs
**Then** the blocked dates are released
**And** other platforms are updated

**Given** a property manager views the calendar
**When** they see a booking
**Then** they can identify which platform it came from (icon/color)

**Technical Notes:**
- Create `rental_bookings` table: id, unit_id, platform, external_booking_id, guest_name, check_in, check_out, status, synced_at
- Background job for sync every 15 minutes
- iCal feed generation for external calendars

---

### Story 18.3: Guest Registration

As a **property manager**,
I want to **register guests for legal compliance**,
So that **I can fulfill local regulations (e.g., tourist registration)**.

**Acceptance Criteria:**

**Given** a booking check-in date approaches (T-24h)
**When** the system sends a reminder
**Then** the property manager receives notification to complete guest registration

**Given** a property manager opens guest registration
**When** they enter guest details (name, ID, nationality, dates)
**Then** the information is saved
**And** can be exported for authority reporting

**Given** guest has ID document
**When** property manager uploads photo of ID
**Then** system extracts data via OCR (Epic 12 reuse)
**And** pre-fills the registration form

**Technical Notes:**
- Create `rental_guests` table: id, booking_id, first_name, last_name, nationality, id_type, id_number, birth_date, registered_at
- Reuse OCR service from Epic 12 for ID extraction
- Compliance with local regulations (SK: ÚHÚL, CZ: cizinecká policie)

---

### Story 18.4: Authority Reports Generation

As a **property manager**,
I want to **generate guest reports for authorities**,
So that **I can comply with legal reporting requirements**.

**Acceptance Criteria:**

**Given** a property manager needs to submit monthly guest report
**When** they select the reporting period and building
**Then** the system generates a report with all registered guests
**And** formats it per local authority requirements

**Given** the report is generated
**When** the property manager downloads it
**Then** they receive PDF and CSV formats
**And** can submit directly to authority portal (if API available)

**Technical Notes:**
- Report formats: SK (ÚHÚL CSV), CZ (cizinecká policie XML)
- Monthly summary with guest counts by nationality
- API integration with government portals (future enhancement)

---

## Epic 19: Lease Management & Tenant Screening

**Goal:** Landlords can screen potential tenants and manage the full lease lifecycle from creation to termination.

### Story 19.1: Tenant Application Intake

As a **landlord**,
I want to **receive and manage tenant applications**,
So that **I can evaluate potential tenants systematically**.

**Acceptance Criteria:**

**Given** a landlord lists a unit for rent
**When** a prospective tenant applies
**Then** an application record is created with contact info
**And** the landlord receives notification

**Given** a landlord views applications
**When** they open the applications list
**Then** they see all applications sorted by date
**And** can filter by status (new, screening, approved, rejected)

**Technical Notes:**
- Create `tenant_applications` table: id, unit_id, applicant_name, email, phone, status, applied_at
- Application form: employment info, income, references, move-in date
- Email notification to landlord on new application

---

### Story 19.2: Tenant Screening

As a **landlord**,
I want to **screen potential tenants**,
So that **I can make informed rental decisions**.

**Acceptance Criteria:**

**Given** a landlord selects an application for screening
**When** they initiate the screening process
**Then** the system collects consent from the applicant
**And** runs background and credit checks (via integration)

**Given** screening results are available
**When** the landlord views the application
**Then** they see: credit score range, background check status, reference verification status
**And** a recommendation (approve/review/decline)

**Given** the applicant doesn't consent to screening
**When** the screening is requested
**Then** the system marks as "consent pending"
**And** sends reminder to applicant

**Technical Notes:**
- Create `tenant_screenings` table: id, application_id, consent_given_at, credit_score_range, background_status, references_verified, recommendation
- Integration placeholder for screening services (Experian, TransUnion)
- GDPR: consent required, data retention limits

---

### Story 19.3: Lease Creation & Signing

As a **landlord**,
I want to **create and sign lease agreements digitally**,
So that **I can formalize rental arrangements efficiently**.

**Acceptance Criteria:**

**Given** a landlord approves an application
**When** they create a lease
**Then** they can select a lease template or upload custom
**And** fill in terms: rent, deposit, start/end dates, rules

**Given** a lease is created
**When** sent for signing
**Then** both landlord and tenant receive signing requests
**And** can sign electronically (e-signature)

**Given** both parties sign
**When** the lease is fully executed
**Then** the status updates to "active"
**And** the tenant is associated with the unit
**And** the lease document is stored in document management

**Technical Notes:**
- Create `leases` table: id, unit_id, tenant_id, landlord_id, start_date, end_date, rent_amount, deposit_amount, status, document_id
- Lease templates stored in document_templates (Epic 7B)
- E-signature: integrate with existing signature_requests (if available) or use simple checkbox consent

---

### Story 19.4: Lease Lifecycle Management

As a **landlord**,
I want to **manage lease renewals and terminations**,
So that **I can handle the full rental lifecycle**.

**Acceptance Criteria:**

**Given** a lease is approaching end date (T-60 days)
**When** the system checks active leases
**Then** the landlord receives renewal reminder notification

**Given** a landlord initiates renewal
**When** they confirm terms (same or updated rent)
**Then** a renewal lease is created linked to original
**And** sent for tenant signature

**Given** a lease needs termination
**When** the landlord or tenant initiates
**Then** they select termination type (end of term, early mutual, early breach)
**And** record termination date and reason

**Technical Notes:**
- Lease status: draft, pending_signature, active, renewing, expired, terminated
- Renewal creates new lease record with `renewed_from_id` reference
- Termination: early termination may require penalty calculation

---

### Story 19.5: Lease Expiration Tracking & Reminders

As a **landlord**,
I want to **track all lease expirations and receive reminders**,
So that **I can plan ahead for renewals or new tenants**.

**Acceptance Criteria:**

**Given** a landlord views their lease dashboard
**When** they see the expiration overview
**Then** they see leases expiring in 30, 60, 90 days
**And** can take action (renew, list for new tenant)

**Given** a lease expires without renewal
**When** the end date passes
**Then** the lease status changes to "expired"
**And** the unit status updates to "available"

**Given** automatic reminders are configured
**When** a lease is 60 days from expiration
**Then** landlord and tenant both receive reminder emails

**Technical Notes:**
- Dashboard widget showing expiration timeline
- Configurable reminder schedules: 90, 60, 30, 14, 7 days
- Auto-update unit status on lease expiration

---

# Phase 5 Stories

## Epic 20: Maintenance Scheduling & Work Orders

**Goal:** Managers can schedule preventive maintenance, create work orders, and track equipment service history.

### Story 20.1: Equipment Registry

As a **building manager**,
I want to **maintain a registry of building equipment**,
So that **I can track maintenance needs and warranties**.

**Acceptance Criteria:**

**Given** a manager navigates to equipment management
**When** they add new equipment
**Then** they can enter: name, type, location, manufacturer, model, serial number, install date, warranty expiry

**Given** equipment is registered
**When** viewing the equipment list
**Then** they see equipment grouped by building/location
**And** warranty status (active, expiring soon, expired)

**Given** equipment has associated documents
**When** viewing equipment details
**Then** they can attach manuals, warranties, service records

**Technical Notes:**
- Create `equipment` table: id, building_id, name, type, location, manufacturer, model, serial_number, install_date, warranty_expiry, status
- Create `equipment_documents` junction table
- Equipment types: HVAC, elevator, fire_system, plumbing, electrical, security, other

---

### Story 20.2: Work Order Creation

As a **building manager**,
I want to **create work orders for maintenance tasks**,
So that **I can assign and track maintenance work**.

**Acceptance Criteria:**

**Given** a manager needs to schedule maintenance
**When** they create a work order
**Then** they specify: title, description, equipment (optional), priority, due date, assigned technician

**Given** a work order is linked to a fault
**When** viewing the fault
**Then** the work order appears in the fault timeline

**Given** a work order is created
**When** the assigned technician views their tasks
**Then** they see the work order in their queue
**And** receive notification

**Technical Notes:**
- Create `work_orders` table: id, building_id, equipment_id, fault_id (nullable), title, description, priority, status, assigned_to, due_date, created_by
- Work order status: open, in_progress, on_hold, completed, cancelled
- Reuse notification infrastructure from Epic 2B

---

### Story 20.3: Preventive Maintenance Schedules

As a **building manager**,
I want to **set up recurring maintenance schedules**,
So that **equipment is serviced before problems occur**.

**Acceptance Criteria:**

**Given** a manager configures preventive maintenance
**When** they create a schedule for equipment
**Then** they specify: task description, frequency (daily, weekly, monthly, quarterly, annually), next due date

**Given** a scheduled maintenance is due
**When** the due date arrives
**Then** a work order is automatically created
**And** assigned per schedule configuration

**Given** maintenance is completed
**When** the technician marks work order done
**Then** the next occurrence is calculated
**And** service history is updated

**Technical Notes:**
- Create `maintenance_schedules` table: id, equipment_id, task_description, frequency, next_due, assigned_to, auto_create_work_order
- Cron-like scheduling: daily, weekly (day), monthly (date), quarterly, annually
- Background job to check due schedules and create work orders

---

### Story 20.4: Service History & Reporting

As a **building manager**,
I want to **view service history for all equipment**,
So that **I can track maintenance patterns and costs**.

**Acceptance Criteria:**

**Given** a manager views equipment details
**When** they open the service history tab
**Then** they see all completed work orders chronologically
**And** total maintenance costs

**Given** a manager needs maintenance reports
**When** they generate a report for a period
**Then** they see: work orders by status, average resolution time, costs by equipment type

**Technical Notes:**
- Service history from completed work_orders
- Cost tracking: add `actual_cost` to work_orders
- Report: PDF/CSV export with charts

---

## Epic 21: Supplier & Vendor Management

**Goal:** Managers can manage vendor relationships, assign service requests, track contracts, and process invoices.

### Story 21.1: Vendor Registry

As a **building manager**,
I want to **maintain a list of vendors and suppliers**,
So that **I can quickly assign work to qualified contractors**.

**Acceptance Criteria:**

**Given** a manager adds a new vendor
**When** they enter vendor details
**Then** they can specify: company name, contact person, phone, email, services provided, contract info

**Given** vendors are registered
**When** viewing the vendor list
**Then** they see vendors grouped by service type
**And** can filter by rating, contract status

**Technical Notes:**
- Create `vendors` table: id, organization_id, company_name, contact_name, phone, email, services (array), rating, contract_start, contract_end, status
- Services: plumbing, electrical, HVAC, cleaning, landscaping, security, other

---

### Story 21.2: Vendor Assignment to Work Orders

As a **building manager**,
I want to **assign vendors to work orders**,
So that **external contractors can handle specialized work**.

**Acceptance Criteria:**

**Given** a work order requires external vendor
**When** the manager assigns a vendor
**Then** the vendor receives notification with work details
**And** work order status updates to "assigned_external"

**Given** a vendor completes work
**When** they mark completion (via link or portal)
**Then** the manager receives notification for review
**And** can approve or request corrections

**Technical Notes:**
- Add `vendor_id` to work_orders table
- Vendor notification via email with secure link
- Simple vendor portal page for status updates

---

### Story 21.3: Contract Management

As a **building manager**,
I want to **track vendor contracts and renewals**,
So that **I ensure continuous service coverage**.

**Acceptance Criteria:**

**Given** a vendor has a contract
**When** the contract is approaching expiry (T-30 days)
**Then** the manager receives renewal reminder

**Given** viewing a vendor
**When** opening contract details
**Then** they see: contract dates, terms, attached documents

**Technical Notes:**
- Contract documents stored via document management
- Reminder notifications using existing infrastructure
- Contract history for audit

---

### Story 21.4: Vendor Invoice Processing

As a **building manager**,
I want to **record and track vendor invoices**,
So that **I can manage payments and costs**.

**Acceptance Criteria:**

**Given** a vendor submits an invoice
**When** the manager records it
**Then** they enter: invoice number, amount, date, linked work order(s)

**Given** invoices are recorded
**When** viewing vendor invoices
**Then** they see: pending, approved, paid status
**And** total amounts by period

**Given** an invoice is approved
**When** exported to accounting
**Then** it syncs with financial module (Epic 11)

**Technical Notes:**
- Create `vendor_invoices` table: id, vendor_id, invoice_number, amount, date, status, work_order_ids (array)
- Integration with financial transactions

---

## Epic 22: Insurance Management

**Goal:** Managers can track insurance policies, submit and track claims, and receive renewal reminders.

### Story 22.1: Policy Registry

As a **building manager**,
I want to **track all insurance policies for my buildings**,
So that **I know coverage details and expiration dates**.

**Acceptance Criteria:**

**Given** a manager adds a new policy
**When** they enter policy details
**Then** they specify: provider, policy number, type (property, liability, etc.), coverage amount, premium, start/end dates

**Given** policies are registered
**When** viewing the insurance dashboard
**Then** they see all policies with status (active, expiring, expired)
**And** total coverage and premium amounts

**Technical Notes:**
- Create `insurance_policies` table: id, building_id, provider, policy_number, type, coverage_amount, premium_amount, start_date, end_date, status
- Policy types: property, liability, flood, earthquake, equipment, umbrella

---

### Story 22.2: Policy Document Management

As a **building manager**,
I want to **store and access policy documents**,
So that **I can reference coverage details when needed**.

**Acceptance Criteria:**

**Given** a policy is registered
**When** the manager uploads policy document
**Then** it is stored and linked to the policy

**Given** viewing a policy
**When** opening documents tab
**Then** they see all related documents (policy, endorsements, certificates)

**Technical Notes:**
- Reuse document management from Epic 7A
- Create `insurance_policy_documents` junction table

---

### Story 22.3: Claims Submission & Tracking

As a **building manager**,
I want to **submit and track insurance claims**,
So that **I can recover losses from covered incidents**.

**Acceptance Criteria:**

**Given** an incident occurs (fault, damage)
**When** the manager submits a claim
**Then** they specify: policy, incident date, description, estimated loss, linked fault (if any)

**Given** a claim is submitted
**When** tracking progress
**Then** they see: status (submitted, under_review, approved, denied, paid), adjuster notes, settlement amount

**Technical Notes:**
- Create `insurance_claims` table: id, policy_id, fault_id (nullable), incident_date, description, estimated_loss, status, settlement_amount, adjuster_notes
- Claim status workflow: submitted → under_review → approved/denied → paid

---

### Story 22.4: Renewal Reminders

As a **building manager**,
I want to **receive reminders before policies expire**,
So that **I can renew coverage without gaps**.

**Acceptance Criteria:**

**Given** a policy is expiring in 60 days
**When** the reminder schedule triggers
**Then** the manager receives email notification with policy details

**Given** viewing insurance dashboard
**When** policies are expiring soon
**Then** they are highlighted with "action needed" status

**Technical Notes:**
- Configurable reminder: 90, 60, 30, 14 days before expiry
- Dashboard widget showing upcoming renewals

---

## Epic 23: Emergency Management

**Goal:** Organizations can define emergency protocols, manage incidents, and coordinate emergency responses.

### Story 23.1: Emergency Protocol Definition

As an **organization admin**,
I want to **define emergency protocols for different scenarios**,
So that **everyone knows what to do in emergencies**.

**Acceptance Criteria:**

**Given** an admin creates an emergency protocol
**When** they define the protocol
**Then** they specify: name, type (fire, flood, gas_leak, security, medical), steps, contacts, evacuation routes

**Given** protocols are defined
**When** users access emergency info
**Then** they can view protocols relevant to their building

**Technical Notes:**
- Create `emergency_protocols` table: id, organization_id, building_id (nullable), name, type, steps (JSONB), contacts (JSONB), evacuation_info
- Protocol types: fire, flood, gas_leak, power_outage, security_threat, medical, natural_disaster

---

### Story 23.2: Emergency Contact Management

As a **building manager**,
I want to **maintain emergency contact lists**,
So that **we can reach the right people quickly**.

**Acceptance Criteria:**

**Given** a manager sets up emergency contacts
**When** they add a contact
**Then** they specify: name, role, phone, email, priority order

**Given** an emergency occurs
**When** contacts are needed
**Then** they appear in priority order with click-to-call

**Technical Notes:**
- Create `emergency_contacts` table: id, building_id, name, role, phone, email, priority_order
- Roles: fire_department, police, ambulance, utility_company, building_manager, security

---

### Story 23.3: Incident Reporting

As a **building resident or manager**,
I want to **report emergency incidents**,
So that **response can be coordinated quickly**.

**Acceptance Criteria:**

**Given** an emergency occurs
**When** a user reports an incident
**Then** they specify: type, location, description, severity
**And** can attach photos

**Given** an incident is reported
**When** managers are notified
**Then** they receive immediate push/SMS notification
**And** can view incident details

**Technical Notes:**
- Create `emergency_incidents` table: id, building_id, reported_by, type, location, description, severity, status, reported_at
- High-priority notification (bypasses quiet hours)
- Incident linked to follow-up fault if needed

---

### Story 23.4: Emergency Broadcast

As a **building manager**,
I want to **broadcast emergency alerts to all residents**,
So that **everyone is informed during emergencies**.

**Acceptance Criteria:**

**Given** an emergency requires notification
**When** the manager sends emergency broadcast
**Then** all building residents receive: push notification, SMS (if phone registered), email

**Given** an emergency broadcast is sent
**When** viewing broadcast history
**Then** they see delivery status and acknowledgments

**Technical Notes:**
- Emergency broadcasts bypass notification preferences
- Delivery via all channels simultaneously
- Acknowledgment tracking: "I'm safe" button
- Rate limit: only managers can broadcast, max 1 per 5 minutes

---

# Phase 6 Stories

## Epic 24: Budget & Financial Planning

**Goal:** Managers can create budgets, track spending against budget, forecast expenses, and plan capital improvements.

### Story 24.1: Annual Budget Creation

As a **building manager**,
I want to **create annual budgets for my buildings**,
So that **I can plan expenses and set fee levels**.

**Acceptance Criteria:**

**Given** a manager creates a new budget
**When** they define the budget
**Then** they specify: year, categories (aligned with financial accounts), amounts per category

**Given** a budget is created
**When** viewing the budget
**Then** they see: total budget, breakdown by category, comparison to previous year

**Technical Notes:**
- Create `budgets` table: id, building_id, year, status (draft, approved, active)
- Create `budget_items` table: id, budget_id, category, amount, notes
- Categories: maintenance, utilities, insurance, management_fees, reserve_fund, other

---

### Story 24.2: Budget vs Actual Tracking

As a **building manager**,
I want to **track actual spending against budget**,
So that **I can identify variances and adjust**.

**Acceptance Criteria:**

**Given** a budget is active
**When** expenses are recorded (from financial module)
**Then** they are automatically categorized against budget

**Given** viewing budget tracking
**When** opening the variance report
**Then** they see: budget amount, actual spent, variance ($ and %), by category

**Given** significant variance detected (>10%)
**When** the variance threshold is exceeded
**Then** the manager receives alert notification

**Technical Notes:**
- Link to financial transactions (Epic 11) by category
- Monthly and YTD views
- Variance alerts configurable per category

---

### Story 24.3: Capital Improvement Planning

As a **building manager**,
I want to **plan capital improvements with funding**,
So that **I can schedule major projects and build reserves**.

**Acceptance Criteria:**

**Given** a manager plans a capital project
**When** they create the plan
**Then** they specify: name, description, estimated cost, target year, funding source (reserve, special assessment, loan)

**Given** capital plans exist
**When** viewing the multi-year forecast
**Then** they see: projects by year, total costs, reserve fund projections

**Technical Notes:**
- Create `capital_plans` table: id, building_id, name, description, estimated_cost, target_year, funding_source, status
- Reserve fund projection: current balance + annual contributions - planned projects

---

### Story 24.4: Financial Forecasting

As a **building manager**,
I want to **forecast future expenses**,
So that **I can plan fee increases and reserves**.

**Acceptance Criteria:**

**Given** historical data exists
**When** generating a forecast
**Then** the system projects: expenses by category (with inflation), reserve fund balance, recommended fee level

**Given** viewing the forecast
**When** adjusting parameters (inflation rate, fee increase)
**Then** the projections update in real-time

**Technical Notes:**
- 3-5 year forecast based on historical trends
- Inflation adjustment configurable (default 3%)
- Fee recommendation based on expense coverage

---

## Epic 25: Legal Document & Compliance

**Goal:** Organizations can manage legal documents, track regulatory compliance, and handle legal notices.

### Story 25.1: Legal Document Repository

As an **organization admin**,
I want to **maintain a repository of legal documents**,
So that **I can access contracts, regulations, and legal records**.

**Acceptance Criteria:**

**Given** an admin uploads a legal document
**When** they categorize it
**Then** they specify: type (contract, regulation, court_order, etc.), parties, effective date, expiry date

**Given** legal documents are stored
**When** searching
**Then** they can find documents by type, party, date range, content (full-text)

**Technical Notes:**
- Extend document management with legal-specific metadata
- Document types: contract, lease_template, regulation, court_order, legal_notice, minutes
- Retention policies per document type (GDPR compliance)

---

### Story 25.2: Regulatory Compliance Tracking

As a **building manager**,
I want to **track regulatory compliance requirements**,
So that **I ensure buildings meet all legal requirements**.

**Acceptance Criteria:**

**Given** compliance requirements are configured
**When** viewing compliance dashboard
**Then** they see: requirements by building, status (compliant, due_soon, overdue), last verified date

**Given** a compliance item is due
**When** the due date approaches (T-30 days)
**Then** the manager receives reminder notification

**Technical Notes:**
- Create `compliance_requirements` table: id, building_id, name, description, frequency, last_verified, next_due, status
- Requirements: fire_safety_inspection, elevator_certification, energy_audit, accessibility_compliance

---

### Story 25.3: Legal Notice Management

As a **building manager**,
I want to **send and track legal notices to residents**,
So that **I can document formal communications**.

**Acceptance Criteria:**

**Given** a manager needs to send a legal notice
**When** they create the notice
**Then** they specify: recipients, subject, content, delivery method (email, mail, both)

**Given** a notice is sent
**When** tracking delivery
**Then** they see: sent date, delivery status, acknowledgment (if required)

**Given** recipients must acknowledge
**When** they receive the notice
**Then** they can acknowledge receipt
**And** the system tracks acknowledgment timestamp

**Technical Notes:**
- Create `legal_notices` table: id, building_id, subject, content, recipients (array), delivery_method, sent_at
- Create `legal_notice_acknowledgments` table: id, notice_id, user_id, acknowledged_at
- Integrate with existing announcement/notification system

---

### Story 25.4: Compliance Reporting

As an **organization admin**,
I want to **generate compliance reports**,
So that **I can demonstrate regulatory adherence**.

**Acceptance Criteria:**

**Given** compliance data exists
**When** generating a compliance report
**Then** it includes: all requirements, status, verification history, upcoming due items

**Given** the report is generated
**When** exporting
**Then** available as PDF with organization branding

**Technical Notes:**
- Report includes audit trail of verifications
- Exportable for regulatory submissions

---

## Epic 26: Platform Subscription & Billing

**Goal:** Platform operators can manage organization subscriptions, track usage, and generate invoices.

### Story 26.1: Subscription Plan Management

As a **platform admin**,
I want to **define and manage subscription plans**,
So that **organizations can choose the right tier**.

**Acceptance Criteria:**

**Given** a platform admin creates a plan
**When** they define it
**Then** they specify: name, price, billing cycle (monthly, annual), features, limits (users, buildings, storage)

**Given** plans are defined
**When** viewing the plan list
**Then** they see: all plans with pricing and feature comparison

**Technical Notes:**
- Create `subscription_plans` table: id, name, price_monthly, price_annual, features (JSONB), limits (JSONB), active
- Default plans: Free (1 building, 20 users), Professional ($99/mo), Enterprise (custom)

---

### Story 26.2: Organization Subscription

As an **organization admin**,
I want to **subscribe to a plan and manage billing**,
So that **I can access platform features**.

**Acceptance Criteria:**

**Given** an organization is on Free plan
**When** they upgrade to Professional
**Then** they select payment method and confirm
**And** features unlock immediately

**Given** a subscription is active
**When** viewing billing settings
**Then** they see: current plan, usage, payment method, invoice history

**Technical Notes:**
- Create `subscriptions` table: id, organization_id, plan_id, status, started_at, current_period_start, current_period_end, payment_method_id
- Stripe integration for payments
- Proration for mid-cycle changes

---

### Story 26.3: Usage Metering

As a **platform operator**,
I want to **track usage metrics for billing**,
So that **I can implement usage-based pricing**.

**Acceptance Criteria:**

**Given** an organization uses the platform
**When** they exceed plan limits
**Then** they receive warning notifications at 80%, 100%
**And** can purchase additional capacity or upgrade

**Given** usage data is collected
**When** viewing usage dashboard
**Then** platform admins see: users, buildings, storage, API calls by organization

**Technical Notes:**
- Create `usage_metrics` table: id, organization_id, metric_type, value, recorded_at
- Metrics: active_users, buildings, storage_gb, api_calls_monthly
- Background job to aggregate daily usage

---

### Story 26.4: Invoice Generation

As a **platform operator**,
I want to **generate and send invoices**,
So that **organizations receive proper billing documentation**.

**Acceptance Criteria:**

**Given** a billing period ends
**When** the system generates invoices
**Then** each subscribing organization receives an invoice
**And** it includes: plan charges, overage charges (if any), taxes, total

**Given** an invoice is generated
**When** sent to organization
**Then** the admin receives email with PDF attachment
**And** can view in billing history

**Technical Notes:**
- Create `invoices` table: id, organization_id, subscription_id, amount, tax, total, status, due_date, paid_at
- Invoice PDF generation with platform branding
- Payment status: pending, paid, overdue, cancelled

---

# Phase 7 Stories

## Epic 27: Property Comparison & Analytics

**Goal:** Portal users can compare properties side-by-side, view market analytics, and receive AI-powered recommendations.

### Story 27.1: Property Comparison Tool

As a **portal user**,
I want to **compare properties side-by-side**,
So that **I can make informed decisions**.

**Acceptance Criteria:**

**Given** a user is browsing listings
**When** they add properties to comparison (up to 4)
**Then** the comparison counter updates
**And** they can open comparison view

**Given** comparison view is open
**When** viewing properties
**Then** they see: photos, key details (price, size, rooms), features in aligned columns
**And** can highlight differences

**Technical Notes:**
- Store comparison in session/localStorage
- Comparison attributes: price, size, rooms, floor, year_built, price_per_sqm, features
- Shareable comparison link

---

### Story 27.2: Market Analytics Dashboard

As a **portal user**,
I want to **view market analytics for areas I'm interested in**,
So that **I can understand pricing trends**.

**Acceptance Criteria:**

**Given** a user searches in a city/district
**When** they view market analytics
**Then** they see: average price, price per sqm, price trend (6 months), days on market, inventory count

**Given** viewing a specific listing
**When** they open analytics
**Then** they see: how it compares to area average, price history (if relisted)

**Technical Notes:**
- Aggregate from listing data
- Cache analytics per district (refresh daily)
- Charts: line chart for trends, bar chart for comparisons

---

### Story 27.3: AI Property Recommendations

As a **portal user**,
I want to **receive personalized property recommendations**,
So that **I discover relevant listings**.

**Acceptance Criteria:**

**Given** a user has search history and favorites
**When** they view recommendations
**Then** they see: listings matching their preferences
**And** explanation why each was recommended

**Given** a user views a listing
**When** scrolling to "Similar Properties"
**Then** they see related listings based on: location, price range, features

**Technical Notes:**
- Recommendation based on: search criteria, viewed listings, favorites, budget
- Explanation: "Because you searched for 3-room apartments in Bratislava"
- Content-based filtering (no collaborative filtering in MVP)

---

### Story 27.4: Saved Comparison & Sharing

As a **portal user**,
I want to **save and share my property comparisons**,
So that **I can discuss with family or return later**.

**Acceptance Criteria:**

**Given** a user creates a comparison
**When** they save it
**Then** it's stored in their account (if logged in)
**And** they can access it later

**Given** a user wants to share
**When** they click share
**Then** they get a shareable link
**And** recipients can view the comparison (read-only)

**Technical Notes:**
- Create `saved_comparisons` table: id, user_id (nullable for anonymous), listing_ids, created_at, share_token
- Share token for public access without login
- Expiry: 30 days for anonymous, permanent for logged-in users

---

# Validation

## FR Coverage Validation

| FR Range | Epic | Status |
|----------|------|--------|
| FR84-FR86 | Epic 18 | ✅ Covered |
| FR87-FR89 | Epic 19 | ✅ Covered |
| FR102-FR104 | Epic 20 | ✅ Covered |
| FR105-FR107 | Epic 21 | ✅ Covered |
| FR108-FR110 | Epic 22 | ✅ Covered |
| FR111-FR113 | Epic 23 | ✅ Covered |
| FR114-FR116 | Epic 24 | ✅ Covered |
| FR117-FR119 | Epic 25 | ✅ Covered |
| FR120-FR122 | Epic 26 | ✅ Covered |
| FR123-FR125 | Epic 27 | ✅ Covered |

**Total New FRs:** 30 (FR84-FR89 from existing, FR102-FR125 new)
**All FRs Mapped:** ✅ Yes

---

*End of Epic Breakdown Part 2*
