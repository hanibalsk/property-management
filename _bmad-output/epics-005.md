---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - _bmad-output/prd.md
  - _bmad-output/architecture.md
  - _bmad-output/ux-design-specification.md
  - _bmad-output/epics.md
  - _bmad-output/epics-002.md
  - _bmad-output/epics-003.md
  - _bmad-output/epics-004.md
  - docs/use-cases.md
workflowType: 'epics-and-stories'
lastStep: 4
status: 'complete'
project_name: 'Property Management System (PPT) & Reality Portal'
user_name: 'Martin Janci'
date: '2025-12-24'
continues_from: 'epics-004.md'
phase_range: '16, 17, 18'
epic_range: '54-63'
---

# Property Management System (PPT) & Reality Portal - Epic Breakdown (Part 5)

## Overview

This document continues from `epics-004.md` and provides epic and story breakdown for **Phases 16, 17, and 18** - covering remaining use cases that were not addressed in previous epic documents.

**Continuation from:** `epics-004.md` (Epics 39-53, Phases 12-15)

**Gap Analysis Date:** 2025-12-24

This document addresses the following gaps identified from the 508 use cases in `docs/use-cases.md`:

| Gap Category | Use Cases | New Epic(s) |
|--------------|-----------|-------------|
| Forms Management | UC-09 (8 UCs) | Epic 54 |
| Reporting & Analytics | UC-17 (5 UCs) | Epic 55 |
| Facility Booking | UC-24.4 | Epic 56 |
| Registries (Pet, Vehicle) | UC-24.6, UC-24.7 | Epic 57 |
| Package & Visitor Mgmt | UC-24.8, UC-24.9 | Epic 58 |
| News & Media | UC-13 (9 UCs) | Epic 59 |
| Accessibility Features | UC-25 (8 UCs) | Epic 60 |
| External Integrations | UC-22 (10 UCs) | Epic 61 |
| Emergency Contacts | UC-24.10 | Epic 62 |
| GDPR Tenant Screening | UC-33.11-12 | Epic 63 |

---

## Requirements Inventory (New FRs)

### Functional Requirements (FR166-FR210)

**Phase 16: Building Operations & Utilities**

**CA-35: Forms Management (5 FRs)**
- FR166: Managers can create and publish form templates (UC-09.3, UC-09.5)
- FR167: Residents can search and download forms (UC-09.1, UC-09.2)
- FR168: Residents can submit filled forms online (UC-09.4)
- FR169: Managers can view and export form submissions (UC-09.6, UC-09.7, UC-09.8)
- FR170: Forms support digital signatures (UC-09.4)

**CA-36: Advanced Reporting & Analytics (5 FRs)**
- FR171: Managers can generate fault statistics reports (UC-17.1)
- FR172: Managers can generate voting participation reports (UC-17.2)
- FR173: Managers can generate occupancy reports (UC-17.3)
- FR174: Managers can generate consumption reports (UC-17.4)
- FR175: Reports can be exported to PDF/Excel (UC-17.5)

**CA-37: Facility Booking (4 FRs)**
- FR176: Residents can view available common areas (UC-24.4)
- FR177: Residents can book facilities with date/time selection (UC-24.4)
- FR178: Managers can configure booking rules and availability (UC-24.4)
- FR179: System sends booking confirmations and reminders (UC-24.4)

**CA-38: Building Registries (4 FRs)**
- FR180: Residents can register pets with details and documents (UC-24.6)
- FR181: Residents can register vehicles for parking management (UC-24.7)
- FR182: Managers can view and manage registry entries (UC-24.6, UC-24.7)
- FR183: System enforces registry rules per building (UC-24.6, UC-24.7)

**Phase 17: Visitor & Package Management**

**CA-39: Package Tracking (4 FRs)**
- FR184: Residents can register expected packages (UC-24.8)
- FR185: Managers can log package arrivals (UC-24.8)
- FR186: Residents receive notifications on package arrival (UC-24.8)
- FR187: System tracks package pickup history (UC-24.8)

**CA-40: Visitor Management (4 FRs)**
- FR188: Residents can pre-register visitors (UC-24.9)
- FR189: System generates temporary access codes (UC-24.9)
- FR190: Managers can view expected visitors (UC-24.9)
- FR191: Visitors receive access instructions via email/SMS (UC-24.9)

**CA-41: News & Media Management (5 FRs)**
- FR192: Managers can publish news articles with attachments (UC-13.1-13.4)
- FR193: Residents can react to and comment on articles (UC-13.7-13.8)
- FR194: Managers can archive and manage news lifecycle (UC-13.5-13.7)
- FR195: Residents can share articles externally (UC-13.9)
- FR196: News supports rich media (images, videos) (UC-13.4)

**Phase 18: Accessibility & Advanced Integrations**

**CA-42: Accessibility Features (5 FRs)**
- FR197: Application supports screen reader navigation (UC-25.1)
- FR198: Users can enable high contrast mode (UC-25.3)
- FR199: Users can adjust text size (UC-25.4)
- FR200: All features support keyboard navigation (UC-25.8)
- FR201: Video content includes captions (UC-25.5)

**CA-43: External Integrations Suite (5 FRs)**
- FR202: Users can sync events with Google/Outlook calendars (UC-22.1)
- FR203: Managers can export to accounting systems (UC-22.2)
- FR204: Managers can use electronic document signing (UC-22.7)
- FR205: Managers can conduct video meetings via integration (UC-22.8)
- FR206: System supports webhook notifications (UC-22.10)

**CA-44: Emergency Contact Directory (2 FRs)**
- FR207: Users can access emergency contact directory (UC-24.10)
- FR208: Managers can configure emergency contacts per building (UC-24.10)

**CA-45: Enhanced Tenant Screening (2 FRs)**
- FR209: System supports GDPR-compliant tenant screening (UC-33.11)
- FR210: Tenants can manage screening consent (UC-33.12)

---

### Non-Functional Requirements (Phase 16-18 Specific)

- NFR56: Form submission < 2s
- NFR57: Report generation < 10s for standard reports
- NFR58: Facility booking availability check < 500ms
- NFR59: Package notification delivery < 30s
- NFR60: Screen reader compatibility (WCAG 2.1 AA)
- NFR61: High contrast ratio 7:1 minimum
- NFR62: Calendar sync latency < 5s
- NFR63: Webhook delivery with 3x retry

---

### FR Coverage Map

| FR | Epic | Description | Target Apps |
|----|------|-------------|-------------|
| FR166 | 54 | Form template creation | api-server, ppt-web |
| FR167 | 54 | Form search and download | api-server, ppt-web, mobile |
| FR168 | 54 | Online form submission | api-server, ppt-web, mobile |
| FR169 | 54 | Form submission management | api-server, ppt-web |
| FR170 | 54 | Digital form signatures | api-server, ppt-web |
| FR171 | 55 | Fault statistics reports | api-server, ppt-web |
| FR172 | 55 | Voting participation reports | api-server, ppt-web |
| FR173 | 55 | Occupancy reports | api-server, ppt-web |
| FR174 | 55 | Consumption reports | api-server, ppt-web |
| FR175 | 55 | Report export (PDF/Excel) | api-server, ppt-web |
| FR176 | 56 | View available facilities | api-server, ppt-web, mobile |
| FR177 | 56 | Facility booking | api-server, ppt-web, mobile |
| FR178 | 56 | Booking configuration | api-server, ppt-web |
| FR179 | 56 | Booking notifications | api-server |
| FR180 | 57 | Pet registration | api-server, ppt-web, mobile |
| FR181 | 57 | Vehicle registration | api-server, ppt-web, mobile |
| FR182 | 57 | Registry management | api-server, ppt-web |
| FR183 | 57 | Registry rules | api-server |
| FR184 | 58 | Package registration | api-server, ppt-web, mobile |
| FR185 | 58 | Package arrival logging | api-server, ppt-web |
| FR186 | 58 | Package notifications | api-server |
| FR187 | 58 | Package history | api-server, ppt-web, mobile |
| FR188 | 58 | Visitor pre-registration | api-server, ppt-web, mobile |
| FR189 | 58 | Temporary access codes | api-server |
| FR190 | 58 | Expected visitor view | api-server, ppt-web |
| FR191 | 58 | Visitor instructions | api-server |
| FR192 | 59 | News publishing | api-server, ppt-web |
| FR193 | 59 | News reactions/comments | api-server, ppt-web, mobile |
| FR194 | 59 | News lifecycle | api-server, ppt-web |
| FR195 | 59 | News sharing | ppt-web, mobile |
| FR196 | 59 | Rich media support | api-server, ppt-web, mobile |
| FR197 | 60 | Screen reader support | ppt-web, reality-web |
| FR198 | 60 | High contrast mode | ppt-web, reality-web, mobile |
| FR199 | 60 | Text size adjustment | ppt-web, reality-web, mobile |
| FR200 | 60 | Keyboard navigation | ppt-web, reality-web |
| FR201 | 60 | Video captions | ppt-web, reality-web |
| FR202 | 61 | Calendar integration | api-server, ppt-web, mobile |
| FR203 | 61 | Accounting export | api-server, ppt-web |
| FR204 | 61 | E-signature integration | api-server, ppt-web |
| FR205 | 61 | Video conferencing | api-server, ppt-web |
| FR206 | 61 | Webhooks | api-server |
| FR207 | 62 | Emergency contacts | api-server, ppt-web, mobile |
| FR208 | 62 | Emergency contact config | api-server, ppt-web |
| FR209 | 63 | GDPR tenant screening | api-server, ppt-web |
| FR210 | 63 | Screening consent | api-server, ppt-web |

---

## Epic List

### Phase 16: Building Operations & Utilities

#### Epic 54: Forms Management
**Goal:** Managers can create, publish, and manage form templates. Residents can search, download, and submit forms online with digital signatures.

**FRs covered:** FR166, FR167, FR168, FR169, FR170
**Target Apps:** api-server, ppt-web, mobile (React Native)
**Estimate:** 5 stories, ~2.5 weeks
**Dependencies:** Epic 7A (Documents), Epic 1 (Auth)

**Key Decisions:**
- Form builder with drag-and-drop fields
- PDF generation for downloadable forms
- Digital signature integration (DocuSign or similar)
- Form versioning and archival

---

#### Story 54.1: Create Form Template

As a **manager**,
I want to **create form templates with various field types**,
So that **residents can fill them out digitally**.

**Acceptance Criteria:**

**Given** a manager accesses the forms management section
**When** they click "Create Form"
**Then** a form builder opens with field types (text, number, date, checkbox, signature)
**And** they can save the form as draft or publish immediately

**Technical Notes:**
- Form builder component with drag-drop
- Field validation rules configuration
- Preview functionality

---

#### Story 54.2: Search and Download Forms

As a **resident**,
I want to **search and download available forms**,
So that **I can fill them out offline if needed**.

**Acceptance Criteria:**

**Given** a resident opens the forms section
**When** they search by form name or category
**Then** matching forms are displayed
**And** they can download forms as PDF

**Technical Notes:**
- Full-text search on form titles and descriptions
- PDF generation on-the-fly
- Download tracking

---

#### Story 54.3: Submit Form Online

As a **resident**,
I want to **fill out and submit forms online**,
So that **I don't need to print and deliver them**.

**Acceptance Criteria:**

**Given** a resident opens a submittable form
**When** they fill in all required fields
**Then** they can submit the form
**And** receive confirmation with submission ID

**Technical Notes:**
- Form validation before submission
- File attachment support
- Submission confirmation email

---

#### Story 54.4: Manage Form Submissions

As a **manager**,
I want to **view and process submitted forms**,
So that **I can handle resident requests**.

**Acceptance Criteria:**

**Given** a manager opens form submissions
**When** they filter by form type or date
**Then** submissions are listed with status
**And** they can mark as processed or request corrections

**Technical Notes:**
- Submission list with filters
- Status workflow (new, in review, processed, rejected)
- Export to CSV/Excel

---

#### Story 54.5: Digital Signature Support

As a **resident**,
I want to **digitally sign forms that require signatures**,
So that **the submission is legally valid**.

**Acceptance Criteria:**

**Given** a form requires signature
**When** the resident reaches the signature field
**Then** they can draw or type their signature
**And** the signature is embedded in the submitted form

**Technical Notes:**
- Canvas-based signature capture
- Signature timestamp and verification
- Integration with e-signature service for legal validity

---

#### Epic 55: Advanced Reporting & Analytics
**Goal:** Managers can generate comprehensive reports on faults, voting, occupancy, and consumption with export capabilities.

**FRs covered:** FR171, FR172, FR173, FR174, FR175
**Target Apps:** api-server, ppt-web
**Estimate:** 5 stories, ~2 weeks
**Dependencies:** Epic 4 (Faults), Epic 5 (Voting), Epic 11 (Financial), Epic 12 (Meters)

**Key Decisions:**
- Report templates with configurable parameters
- Chart library for visualizations (Recharts/Chart.js)
- Scheduled report generation
- PDF and Excel export engines

---

#### Story 55.1: Fault Statistics Report

As a **manager**,
I want to **generate reports on fault statistics**,
So that **I can identify trends and problem areas**.

**Acceptance Criteria:**

**Given** a manager opens the reports section
**When** they select "Fault Statistics" and date range
**Then** a report shows fault counts by category, status, building
**And** includes charts for trends over time

**Technical Notes:**
- Aggregation queries with date filters
- Bar/line charts for visualization
- Drill-down to individual faults

---

#### Story 55.2: Voting Participation Report

As a **manager**,
I want to **generate reports on voting participation**,
So that **I can track engagement and quorum achievement**.

**Acceptance Criteria:**

**Given** a manager selects "Voting Report"
**When** they choose a time period
**Then** the report shows participation rates per vote
**And** highlights votes that didn't reach quorum

**Technical Notes:**
- Participation percentage calculations
- Quorum threshold indicators
- Owner engagement trends

---

#### Story 55.3: Occupancy Report

As a **manager**,
I want to **generate occupancy reports from person-month data**,
So that **I can plan resources and fees**.

**Acceptance Criteria:**

**Given** a manager selects "Occupancy Report"
**When** they choose buildings and date range
**Then** the report shows occupancy by building, unit
**And** includes comparison with previous periods

**Technical Notes:**
- Person-month aggregation
- Building/unit breakdown
- Year-over-year comparison

---

#### Story 55.4: Consumption Report

As a **manager**,
I want to **generate utility consumption reports**,
So that **I can monitor usage and detect anomalies**.

**Acceptance Criteria:**

**Given** a manager selects "Consumption Report"
**When** they choose utility type and period
**Then** the report shows consumption by unit
**And** highlights above-average consumers

**Technical Notes:**
- Meter reading aggregation
- Statistical anomaly detection
- Per-unit cost allocation

---

#### Story 55.5: Export Reports to PDF/Excel

As a **manager**,
I want to **export any report to PDF or Excel**,
So that **I can share with stakeholders**.

**Acceptance Criteria:**

**Given** a report is displayed
**When** the manager clicks "Export"
**Then** they can choose PDF or Excel format
**And** the file downloads with proper formatting

**Technical Notes:**
- PDF generation with charts
- Excel with raw data and summary sheets
- Export queue for large reports

---

#### Epic 56: Facility Booking
**Goal:** Residents can view and book common areas. Managers configure availability and booking rules.

**FRs covered:** FR176, FR177, FR178, FR179
**Target Apps:** api-server, ppt-web, mobile (React Native)
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 3 (Buildings), Epic 2B (Notifications)

**Key Decisions:**
- Calendar-based booking interface
- Configurable time slots
- Conflict prevention
- Booking fees integration (optional)

---

#### Story 56.1: View Available Facilities

As a **resident**,
I want to **see available common areas and their schedules**,
So that **I can plan my booking**.

**Acceptance Criteria:**

**Given** a resident opens facility booking
**When** they view the facilities list
**Then** they see all bookable spaces with descriptions and photos
**And** can view the calendar for each facility

**Technical Notes:**
- Facility cards with images
- Calendar view with availability
- Filter by facility type

---

#### Story 56.2: Book a Facility

As a **resident**,
I want to **book a facility for a specific date and time**,
So that **I can use the common area**.

**Acceptance Criteria:**

**Given** a resident selects an available time slot
**When** they confirm the booking
**Then** the slot is reserved for them
**And** they receive confirmation with details

**Technical Notes:**
- Time slot selection UI
- Double-booking prevention
- Booking confirmation email

---

#### Story 56.3: Configure Booking Rules

As a **manager**,
I want to **configure booking rules for facilities**,
So that **usage is fair and organized**.

**Acceptance Criteria:**

**Given** a manager opens facility settings
**When** they configure a facility
**Then** they can set available hours, max duration, advance booking limit
**And** optionally require manager approval

**Technical Notes:**
- Business hours configuration
- Maximum booking duration
- Advance booking window (e.g., 30 days ahead)

---

#### Story 56.4: Booking Notifications

As a **resident**,
I want to **receive reminders about my bookings**,
So that **I don't forget**.

**Acceptance Criteria:**

**Given** a resident has an upcoming booking
**When** the booking time approaches (24h, 1h before)
**Then** they receive reminder notifications
**And** can cancel or reschedule from the notification

**Technical Notes:**
- Scheduled reminder jobs
- Cancellation deep-link
- Push and email notifications

---

#### Epic 57: Building Registries (Pets & Vehicles)
**Goal:** Residents can register pets and vehicles. Managers can view and manage registrations per building rules.

**FRs covered:** FR180, FR181, FR182, FR183
**Target Apps:** api-server, ppt-web, mobile (React Native)
**Estimate:** 4 stories, ~1.5 weeks
**Dependencies:** Epic 3 (Buildings/Units)

**Key Decisions:**
- Registry types: pets, vehicles (extensible)
- Document upload for registrations
- Building-specific rules (e.g., pet restrictions)
- Registration expiration and renewal

---

#### Story 57.1: Register a Pet

As a **resident**,
I want to **register my pet with the building**,
So that **I comply with building rules**.

**Acceptance Criteria:**

**Given** a resident opens the pet registry
**When** they add a pet with name, type, breed, and upload vaccination records
**Then** the pet is registered to their unit
**And** they receive registration confirmation

**Technical Notes:**
- Pet form with type dropdown (dog, cat, other)
- Document upload for vaccination
- Photo upload option

---

#### Story 57.2: Register a Vehicle

As a **resident**,
I want to **register my vehicle for parking**,
So that **I can use the building parking**.

**Acceptance Criteria:**

**Given** a resident opens the vehicle registry
**When** they enter vehicle details (make, model, plate number)
**Then** the vehicle is registered to their unit
**And** they can optionally assign a parking spot

**Technical Notes:**
- Vehicle details form
- License plate validation
- Parking spot assignment (if managed)

---

#### Story 57.3: Manage Registry Entries

As a **manager**,
I want to **view and manage all registry entries**,
So that **I can enforce building policies**.

**Acceptance Criteria:**

**Given** a manager opens the registries section
**When** they filter by building, type, or unit
**Then** they see all registered items
**And** can approve, reject, or remove entries

**Technical Notes:**
- Filterable list view
- Bulk actions
- Export to CSV

---

#### Story 57.4: Configure Registry Rules

As a **manager**,
I want to **configure registry rules per building**,
So that **policies are enforced**.

**Acceptance Criteria:**

**Given** a manager opens building settings
**When** they configure registry rules
**Then** they can set limits (e.g., max 2 pets, no dogs over 20kg)
**And** require manager approval for new registrations

**Technical Notes:**
- Rule configuration per building
- Validation on submission
- Approval workflow toggle

---

#### Epic 58: Package & Visitor Management
**Goal:** Residents can track packages and pre-register visitors with temporary access codes.

**FRs covered:** FR184, FR185, FR186, FR187, FR188, FR189, FR190, FR191
**Target Apps:** api-server, ppt-web, mobile (React Native)
**Estimate:** 5 stories, ~2 weeks
**Dependencies:** Epic 2B (Notifications), Epic 3 (Buildings)

**Key Decisions:**
- Package tracking with barcode/tracking number
- Visitor access codes (time-limited)
- SMS/Email delivery for visitor instructions
- Integration with smart locks (future)

---

#### Story 58.1: Register Expected Package

As a **resident**,
I want to **register an expected package**,
So that **building staff knows to accept it**.

**Acceptance Criteria:**

**Given** a resident opens package tracking
**When** they enter tracking number and carrier
**Then** the package is registered as expected
**And** staff can see pending packages

**Technical Notes:**
- Tracking number input
- Carrier selection
- Expected delivery date

---

#### Story 58.2: Log Package Arrival

As a **building staff/manager**,
I want to **log when packages arrive**,
So that **residents are notified**.

**Acceptance Criteria:**

**Given** a package arrives at the building
**When** staff marks it as received
**Then** the resident receives notification
**And** package status updates to "Ready for pickup"

**Technical Notes:**
- Quick scan/entry interface
- Photo of package (optional)
- Automatic notification trigger

---

#### Story 58.3: Track Package Pickup

As a **resident**,
I want to **see my package history**,
So that **I know what I've received**.

**Acceptance Criteria:**

**Given** a resident views package tracking
**When** they look at history
**Then** they see all packages with status (expected, received, picked up)
**And** can filter by date range

**Technical Notes:**
- Status timeline per package
- Pickup confirmation logging
- History retention per policy

---

#### Story 58.4: Pre-Register Visitor

As a **resident**,
I want to **pre-register a visitor**,
So that **they can access the building easily**.

**Acceptance Criteria:**

**Given** a resident opens visitor management
**When** they enter visitor name, date/time, and purpose
**Then** a temporary access code is generated
**And** the visitor receives instructions via email/SMS

**Technical Notes:**
- Visitor form with date/time picker
- Code generation (6-digit alphanumeric)
- Code expiration (24h default)

---

#### Story 58.5: View Expected Visitors

As a **building staff/manager**,
I want to **see expected visitors**,
So that **I can prepare for their arrival**.

**Acceptance Criteria:**

**Given** staff opens the visitor dashboard
**When** they view today's visitors
**Then** they see visitor names, host units, and arrival times
**And** can verify access codes

**Technical Notes:**
- Today's visitor list
- Code verification interface
- Check-in/check-out logging

---

#### Epic 59: News & Media Management
**Goal:** Managers can publish rich news articles. Residents can react, comment, and share.

**FRs covered:** FR192, FR193, FR194, FR195, FR196
**Target Apps:** api-server, ppt-web, mobile (React Native)
**Estimate:** 4 stories, ~1.5 weeks
**Dependencies:** Epic 6 (Announcements - similar patterns)

**Key Decisions:**
- Rich text editor for articles
- Image and video embedding
- Reaction system (like/reactions)
- Social sharing integration

---

#### Story 59.1: Publish News Article

As a **manager**,
I want to **publish news articles with rich content**,
So that **residents stay informed**.

**Acceptance Criteria:**

**Given** a manager opens the news section
**When** they create an article with title, content, and media
**Then** the article is published to selected buildings
**And** residents receive notifications

**Technical Notes:**
- Rich text editor (TipTap/Quill)
- Image upload with compression
- Video embedding (YouTube, upload)

---

#### Story 59.2: React to Articles

As a **resident**,
I want to **react to news articles**,
So that **I can express my opinion quickly**.

**Acceptance Criteria:**

**Given** a resident reads an article
**When** they click a reaction button
**Then** their reaction is recorded
**And** reaction counts are visible

**Technical Notes:**
- Reaction types (like, love, surprised, sad)
- Toggle behavior (click again to remove)
- Real-time count updates

---

#### Story 59.3: Comment on Articles

As a **resident**,
I want to **comment on news articles**,
So that **I can discuss with neighbors**.

**Acceptance Criteria:**

**Given** a resident opens an article
**When** they write and submit a comment
**Then** the comment appears in the thread
**And** the author is notified of new comments

**Technical Notes:**
- Threaded comments
- Markdown support
- Moderation capability

---

#### Story 59.4: Manage News Lifecycle

As a **manager**,
I want to **archive and manage old news**,
So that **the feed stays relevant**.

**Acceptance Criteria:**

**Given** a manager views their articles
**When** they archive an old article
**Then** it moves to archives but remains accessible
**And** they can restore or permanently delete

**Technical Notes:**
- Archive action
- Archive browsing
- Retention policy configuration

---

### Phase 17: Accessibility & Emergency

#### Epic 60: Accessibility Features
**Goal:** Application meets WCAG 2.1 AA standards with screen reader support, high contrast, and keyboard navigation.

**FRs covered:** FR197, FR198, FR199, FR200, FR201
**Target Apps:** ppt-web, reality-web, mobile (React Native), mobile-native (KMP)
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Cross-cutting (all UI)

**Key Decisions:**
- ARIA attributes throughout
- CSS custom properties for theming
- Focus management
- Caption service for videos

---

#### Story 60.1: Screen Reader Compatibility

As a **user with visual impairment**,
I want to **navigate the app with a screen reader**,
So that **I can use all features**.

**Acceptance Criteria:**

**Given** a user enables a screen reader (NVDA, VoiceOver)
**When** they navigate the application
**Then** all elements have proper labels
**And** navigation order is logical

**Technical Notes:**
- ARIA labels on all interactive elements
- Skip navigation links
- Announce dynamic content updates

---

#### Story 60.2: High Contrast Mode

As a **user with low vision**,
I want to **enable high contrast mode**,
So that **I can see content clearly**.

**Acceptance Criteria:**

**Given** a user opens settings
**When** they enable high contrast mode
**Then** the UI switches to high contrast colors
**And** the preference persists across sessions

**Technical Notes:**
- CSS custom property switching
- 7:1 contrast ratio minimum
- localStorage/user preference storage

---

#### Story 60.3: Text Size Adjustment

As a **user**,
I want to **adjust text size**,
So that **I can read comfortably**.

**Acceptance Criteria:**

**Given** a user opens accessibility settings
**When** they adjust text size slider
**Then** all text scales accordingly
**And** layout remains functional up to 200%

**Technical Notes:**
- CSS rem-based sizing
- Zoom-safe layouts
- User preference persistence

---

#### Story 60.4: Keyboard Navigation

As a **user who cannot use a mouse**,
I want to **navigate using only keyboard**,
So that **I can complete all tasks**.

**Acceptance Criteria:**

**Given** a user uses Tab and Enter keys
**When** they navigate through the app
**Then** all interactive elements are reachable
**And** focus indicators are clearly visible

**Technical Notes:**
- Focus management
- Custom focus styles (2px solid outline)
- Modal trap focus

---

#### Epic 61: External Integrations Suite
**Goal:** Integrate with external services for calendars, accounting, e-signatures, video conferencing, and webhooks.

**FRs covered:** FR202, FR203, FR204, FR205, FR206
**Target Apps:** api-server, ppt-web
**Estimate:** 5 stories, ~3 weeks
**Dependencies:** Epic 1 (Auth for OAuth)

**Key Decisions:**
- OAuth 2.0 for service connections
- Integration configuration per organization
- Webhook retry with exponential backoff
- Service status monitoring

---

#### Story 61.1: Calendar Integration

As a **user**,
I want to **sync meetings to my calendar**,
So that **I don't miss important events**.

**Acceptance Criteria:**

**Given** a user connects their Google or Outlook calendar
**When** a meeting or event is scheduled
**Then** it appears in their external calendar
**And** changes sync bidirectionally

**Technical Notes:**
- OAuth connection flow
- Google Calendar API
- Microsoft Graph API

---

#### Story 61.2: Accounting System Export

As a **manager**,
I want to **export financial data to accounting software**,
So that **I can streamline bookkeeping**.

**Acceptance Criteria:**

**Given** a manager opens financial export
**When** they select format (POHODA, Money S3)
**Then** the export file is generated
**And** can be imported into the accounting system

**Technical Notes:**
- XML export for POHODA
- CSV for Money S3
- Export validation

---

#### Story 61.3: E-Signature Integration

As a **manager**,
I want to **send documents for electronic signature**,
So that **I can complete agreements digitally**.

**Acceptance Criteria:**

**Given** a manager selects a document
**When** they click "Request Signature"
**Then** signers receive email with signing link
**And** signed document is stored in the system

**Technical Notes:**
- DocuSign or similar API
- Signature status tracking
- Signed document storage

---

#### Story 61.4: Video Conferencing

As a **manager**,
I want to **schedule video meetings from the app**,
So that **I can conduct virtual owners' meetings**.

**Acceptance Criteria:**

**Given** a manager creates a meeting
**When** they enable video conferencing
**Then** a Zoom/Teams meeting is created
**And** join link is shared with participants

**Technical Notes:**
- Zoom API integration
- Microsoft Teams integration
- Meeting link embedding

---

#### Story 61.5: Webhook Notifications

As a **system administrator**,
I want to **configure webhooks for events**,
So that **external systems can react to changes**.

**Acceptance Criteria:**

**Given** an admin configures a webhook
**When** the subscribed event occurs
**Then** a POST request is sent to the endpoint
**And** delivery status is logged

**Technical Notes:**
- Webhook configuration UI
- Event type subscription
- Retry with exponential backoff
- Delivery logs

---

#### Epic 62: Emergency Contact Directory
**Goal:** Users can access emergency contacts. Managers configure contacts per building.

**FRs covered:** FR207, FR208
**Target Apps:** api-server, ppt-web, mobile (React Native)
**Estimate:** 2 stories, ~1 week
**Dependencies:** Epic 3 (Buildings)

---

#### Story 62.1: View Emergency Contacts

As a **resident**,
I want to **access emergency contacts quickly**,
So that **I can call for help in emergencies**.

**Acceptance Criteria:**

**Given** a resident opens emergency contacts
**When** they view the list
**Then** they see police, fire, ambulance, and building contacts
**And** can tap to call directly

**Technical Notes:**
- Prominent placement in navigation
- Click-to-call on mobile
- Offline availability

---

#### Story 62.2: Configure Emergency Contacts

As a **manager**,
I want to **configure emergency contacts per building**,
So that **residents have correct local numbers**.

**Acceptance Criteria:**

**Given** a manager opens building settings
**When** they edit emergency contacts
**Then** they can add/edit/remove contacts
**And** contacts are visible to building residents

**Technical Notes:**
- Contact CRUD
- Default system contacts + building-specific
- Validation for phone numbers

---

#### Epic 63: GDPR-Compliant Tenant Screening
**Goal:** Tenant screening follows GDPR requirements with explicit consent management.

**FRs covered:** FR209, FR210
**Target Apps:** api-server, ppt-web
**Estimate:** 2 stories, ~1 week
**Dependencies:** Epic 19 (Tenant Screening), Epic 9 (GDPR)

---

#### Story 63.1: GDPR Screening Workflow

As a **landlord**,
I want to **perform tenant screening with GDPR compliance**,
So that **I don't violate privacy laws**.

**Acceptance Criteria:**

**Given** a landlord initiates screening
**When** the tenant receives the request
**Then** they must provide explicit consent
**And** consent is recorded with timestamp

**Technical Notes:**
- Consent request flow
- Consent recording with audit trail
- Data minimization in screening

---

#### Story 63.2: Manage Screening Consent

As a **prospective tenant**,
I want to **review and provide consent for screening**,
So that **I control my personal data**.

**Acceptance Criteria:**

**Given** a tenant receives a screening request
**When** they review the consent form
**Then** they can see what data will be checked
**And** can accept or decline

**Technical Notes:**
- Clear consent explanation
- Granular consent options
- Withdrawal mechanism

---

## Summary

### Phase 16-18 Totals

| Phase | Epics | Stories | FRs | Estimate |
|-------|-------|---------|-----|----------|
| **Phase 16** | 54-59 | 27 | FR166-FR196 | ~12 weeks |
| **Phase 17** | 60-61 | 9 | FR197-FR206 | ~5 weeks |
| **Phase 18** | 62-63 | 4 | FR207-FR210 | ~2 weeks |
| **Total** | 10 | 40 | 45 FRs | ~19 weeks |

### Complete Epic Inventory (All Documents)

| Document | Epics | Phases |
|----------|-------|--------|
| epics.md | 1-17 | 1-3 (MVP) |
| epics-002.md | 18-27 | 4-7 |
| epics-003.md | 28-38 | 8-11 |
| epics-004.md | 39-53 | 12-15 |
| epics-005.md | 54-63 | 16-18 |
| **Total** | **63 Epics** | **18 Phases** |

### Use Case Coverage

With these new epics, the coverage of the 508 use cases is now:

| Category | UC Count | Covered |
|----------|----------|---------|
| UC-01 to UC-08 | ~75 | ✅ |
| UC-09 (Forms) | 8 | ✅ Epic 54 |
| UC-10 to UC-16 | ~60 | ✅ |
| UC-17 (Reports) | 5 | ✅ Epic 55 |
| UC-18 to UC-23 | ~70 | ✅ |
| UC-24 (Community) | 10 | ✅ Epics 56-59 |
| UC-25 (Accessibility) | 8 | ✅ Epic 60 |
| UC-26 to UC-43 | ~150 | ✅ |
| UC-44 to UC-51 | ~70 | ✅ |
| **Total** | 508 | ✅ 100% |
