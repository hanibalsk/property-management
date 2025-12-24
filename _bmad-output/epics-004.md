---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - _bmad-output/prd.md
  - _bmad-output/architecture.md
  - _bmad-output/ux-design-specification.md
  - _bmad-output/epics.md
  - _bmad-output/epics-002.md
  - _bmad-output/epics-003.md
  - docs/use-cases.md
  - docs/functional-requirements.md
  - docs/non-functional-requirements.md
workflowType: 'epics-and-stories'
lastStep: 4
status: 'complete'
project_name: 'Property Management System (PPT) & Reality Portal'
user_name: 'Martin Janci'
date: '2025-12-23'
continues_from: 'epics-003.md'
phase_range: '12, 13, 14, 15'
epic_range: '39-53'
---

# Property Management System (PPT) & Reality Portal - Epic Breakdown (Part 4)

## Overview

This document continues from `epics-003.md` and provides epic and story breakdown for **Phases 12, 13, 14, and 15** - covering frontend implementations, Reality Portal development, mobile apps, and advanced platform features.

**Continuation from:** `epics-003.md` (Epics 28-38, Phases 8-11)

**Implementation Status (as of 2025-12-23):**
- **Completed Backend (api-server):** Epics 1-38 (all API routes implemented)
- **Completed:** Database migrations, models, repositories for all features
- **Remaining:** Frontend apps, mobile apps, Reality Portal, advanced integrations

---

## Gap Analysis

### Backend Completeness Review

| Epic | Backend Status | Frontend Status | Mobile Status |
|------|---------------|-----------------|---------------|
| 1-10 (Core) | ✅ Complete | ⚠️ Partial | ⚠️ Partial |
| 11-15 (Financial, Meters, AI, IoT, Listings) | ✅ Complete | ❌ Not Started | ❌ Not Started |
| 16-20 (Agencies, Screening, Leases, Work Orders, Vendors) | ✅ Complete | ❌ Not Started | ❌ Not Started |
| 21-26 (Insurance, Emergency, Budgets, Legal, Subscriptions) | ✅ Complete | ❌ Not Started | ❌ Not Started |
| 27 (Property Comparison) | ⚠️ Deferred | ❌ Not Started | ❌ Not Started |
| 28-38 (Advanced) | ✅ Complete | ❌ Not Started | ❌ Not Started |

### Reality Portal Status

| Component | Status |
|-----------|--------|
| reality-server (backend) | ⚠️ Partial (basic routes) |
| reality-web (Next.js) | ❌ Not Started |
| mobile-native (KMP) | ❌ Not Started |

### Deferred Items from Previous Epics

| Item | Original Epic | Status | New Epic |
|------|--------------|--------|----------|
| Document OCR/Full-text search | Epic 28 | Backend ✅ | Epic 39 (Frontend) |
| Advanced Notifications UI | Epic 29 | Backend ✅ | Epic 40 (Frontend) |
| Government Portal Integration | Epic 30 | Backend ✅ | Epic 41 (Frontend) |
| Portal Favorites & Alerts | Epic 31 | Backend ⚠️ | Epic 42 (Reality Portal) |
| Agency Management | Epic 32 | Backend ⚠️ | Epic 43 (Reality Portal) |
| Realtor Tools | Epic 33 | Backend ⚠️ | Epic 44 (Reality Portal) |
| Property Import | Epic 34 | Backend ⚠️ | Epic 45 (Reality Portal) |
| Mobile Native Features | Epic 35 | Backend ✅ | Epic 46 (Mobile) |
| Onboarding & Help | Epic 36 | Backend ✅ | Epic 47 (All Apps) |
| Community Features UI | Epic 37 | Backend ✅ | Epic 48 (Frontend) |
| Workflow Automation UI | Epic 38 | Backend ✅ | Epic 49 (Frontend) |

---

## Phase 12: PPT Frontend Implementation

#### Epic 39: Document Intelligence UI (ppt-web, mobile)

**Goal:** Build frontend interfaces for document OCR, full-text search, classification, and summarization.

**Backend Dependencies:** Epic 28 (API complete)
**Target Apps:** ppt-web, mobile (React Native)
**Estimate:** 4 stories, ~2 weeks

#### Story 39.1: Document Search Interface

As a **user**,
I want to **search within document contents**,
So that **I can find documents by their text, not just titles**.

**Acceptance Criteria:**

**Given** a user opens the document search
**When** they enter search terms
**Then** results show matching documents with highlighted snippets
**And** results are ranked by relevance

**Technical Notes:**
- Enhanced search input with filters (date, type, category)
- Search result cards with text snippet highlighting
- Integration with `/api/v1/documents/search` endpoint

---

#### Story 39.2: OCR Processing Status

As a **document uploader**,
I want to **see the OCR processing status of my documents**,
So that **I know when text extraction is complete**.

**Acceptance Criteria:**

**Given** a user uploads a document
**When** OCR is processing
**Then** a status indicator shows "Processing OCR..."
**And** changes to "Text extracted" when complete

**Technical Notes:**
- Status badge on document cards
- Progress indicator during upload/processing
- Polling or WebSocket for real-time status updates

---

#### Story 39.3: Document Classification UI

As a **manager**,
I want to **see AI-suggested categories for documents**,
So that **I can accept or modify the classification**.

**Acceptance Criteria:**

**Given** a document is uploaded and processed
**When** AI suggests a category
**Then** the suggestion appears with confidence level
**And** user can accept, reject, or change

**Technical Notes:**
- Category suggestion chips
- Confidence percentage display
- Quick-accept/reject buttons

---

#### Story 39.4: Document Summary View

As a **user**,
I want to **see AI-generated summaries of long documents**,
So that **I can quickly understand document contents**.

**Acceptance Criteria:**

**Given** a document has more than 500 words
**When** the user views the document detail
**Then** an AI summary appears at the top
**And** key points are listed in bullets

**Technical Notes:**
- Collapsible summary section
- "Regenerate summary" button
- Summary caching

---

#### Epic 40: Advanced Notifications UI (ppt-web, mobile)

**Goal:** Build frontend for granular notification preferences, quiet hours, and digests.

**Backend Dependencies:** Epic 29 (API complete)
**Target Apps:** ppt-web, mobile (React Native)
**Estimate:** 4 stories, ~1.5 weeks

#### Story 40.1: Granular Preferences Settings

As a **user**,
I want to **configure notification preferences per category**,
So that **I only receive notifications I care about**.

**Acceptance Criteria:**

**Given** a user opens notification settings
**When** they view the preferences page
**Then** they see toggles for each category
**And** each category has channel options (push, email, in-app)

**Technical Notes:**
- Category list with toggle switches
- Channel selector per category
- Save confirmation toast

---

#### Story 40.2: Quiet Hours Configuration

As a **user**,
I want to **set quiet hours for notifications**,
So that **I'm not disturbed during sleep**.

**Acceptance Criteria:**

**Given** a user enables quiet hours
**When** they set start and end times
**Then** non-emergency notifications are held
**And** a schedule preview is shown

**Technical Notes:**
- Time picker for start/end
- Timezone selector
- Visual schedule preview

---

#### Story 40.3: Digest Preferences

As a **user**,
I want to **receive notification summaries instead of individual alerts**,
So that **I'm not overwhelmed**.

**Acceptance Criteria:**

**Given** a user enables digest mode
**When** they select frequency (hourly, daily, weekly)
**Then** notifications are aggregated
**And** a preview shows expected delivery times

**Technical Notes:**
- Frequency selector
- Delivery time picker for daily/weekly
- Sample digest preview

---

#### Story 40.4: Smart Notification Grouping

As a **user**,
I want to **grouped notifications in the notification center**,
So that **my notification list isn't cluttered**.

**Acceptance Criteria:**

**Given** multiple notifications of the same type arrive
**When** viewing the notification center
**Then** they are grouped (e.g., "5 new comments")
**And** can be expanded to see individual items

**Technical Notes:**
- Grouped notification cards
- Expand/collapse animation
- "Mark all as read" per group

---

#### Epic 41: Government Portal UI (ppt-web)

**Goal:** Build frontend for government portal submissions and compliance tracking.

**Backend Dependencies:** Epic 30 (API complete)
**Target Apps:** ppt-web
**Estimate:** 3 stories, ~1 week

#### Story 41.1: Guest Report Submission

As a **property manager**,
I want to **submit guest reports to government portals**,
So that **I comply with registration requirements**.

**Acceptance Criteria:**

**Given** a manager has registered guests
**When** they click "Submit to ÚHÚL/Police"
**Then** they can preview the data to be submitted
**And** submit with a single click

**Technical Notes:**
- Preview modal with data summary
- Country selector (SK/CZ)
- Submission progress indicator

---

#### Story 41.2: Submission Status Tracking

As a **property manager**,
I want to **track the status of my government submissions**,
So that **I know if they succeeded or failed**.

**Acceptance Criteria:**

**Given** submissions have been made
**When** viewing the compliance dashboard
**Then** status is shown (pending, submitted, failed)
**And** failure reasons are displayed

**Technical Notes:**
- Status badges (success/pending/failed)
- Error details tooltip
- Retry button for failures

---

#### Story 41.3: Compliance Dashboard

As a **property manager**,
I want to **see which guests are pending registration**,
So that **I can ensure complete compliance**.

**Acceptance Criteria:**

**Given** guests are registered in the system
**When** viewing the dashboard
**Then** pending, submitted, and overdue counts are shown
**And** deadlines are highlighted

**Technical Notes:**
- Dashboard widgets
- Deadline countdown
- Batch submission capability

---

#### Epic 42: Community Features UI (ppt-web, mobile)

**Goal:** Build frontend for community groups, posts, events, and marketplace.

**Backend Dependencies:** Epic 37 (API complete)
**Target Apps:** ppt-web, mobile (React Native)
**Estimate:** 4 stories, ~2.5 weeks

#### Story 42.1: Community Groups

As a **resident**,
I want to **browse and join community groups**,
So that **I can connect with neighbors**.

**Acceptance Criteria:**

**Given** groups exist in the building
**When** a resident views the groups page
**Then** they see group cards with name, description, member count
**And** can join with a single click

**Technical Notes:**
- Group directory with filters
- Join/Leave buttons
- Member list view

---

#### Story 42.2: Community Feed

As a **resident**,
I want to **post and interact in the community feed**,
So that **I can share and stay informed**.

**Acceptance Criteria:**

**Given** a resident is a group member
**When** they view the feed
**Then** they see posts from their groups
**And** can like, comment, and share

**Technical Notes:**
- Post composer with media upload
- Like/comment interactions
- Feed filtering (all, my groups, building)

---

#### Story 42.3: Community Events

As a **resident**,
I want to **create and RSVP to community events**,
So that **I can participate in building activities**.

**Acceptance Criteria:**

**Given** an event is created
**When** residents view it
**Then** they can RSVP (going/maybe/not going)
**And** see attendee list

**Technical Notes:**
- Event card with date/time/location
- RSVP button group
- Calendar integration (iCal export)

---

#### Story 42.4: Item Marketplace

As a **resident**,
I want to **list and browse items for sale/share**,
So that **I can trade locally**.

**Acceptance Criteria:**

**Given** items are listed
**When** a resident browses
**Then** they see item cards with photos, price, category
**And** can contact the seller

**Technical Notes:**
- Item listing form with photos
- Category filter sidebar
- In-app messaging integration

---

#### Epic 43: Workflow Automation UI (ppt-web)

**Goal:** Build frontend for automation rules, templates, and monitoring.

**Backend Dependencies:** Epic 38 (API complete)
**Target Apps:** ppt-web
**Estimate:** 3 stories, ~1.5 weeks

#### Story 43.1: Automation Rule Builder

As a **manager**,
I want to **create automation rules visually**,
So that **I can automate routine tasks**.

**Acceptance Criteria:**

**Given** a manager opens automation settings
**When** they create a new rule
**Then** they can select trigger, conditions, and actions
**And** see a visual preview of the workflow

**Technical Notes:**
- Visual rule builder with drag-and-drop
- Trigger type selector
- Condition builder with operators
- Action configuration panel

---

#### Story 43.2: Template Library

As a **manager**,
I want to **create rules from templates**,
So that **I can quickly set up common automations**.

**Acceptance Criteria:**

**Given** templates are available
**When** selecting a template
**Then** the rule is pre-configured
**And** can be customized before saving

**Technical Notes:**
- Template gallery with previews
- One-click create from template
- Template customization modal

---

#### Story 43.3: Execution Monitoring

As a **manager**,
I want to **monitor automation executions**,
So that **I can ensure rules work correctly**.

**Acceptance Criteria:**

**Given** automations are running
**When** viewing the dashboard
**Then** recent executions are shown with status
**And** failures show error details

**Technical Notes:**
- Execution log table
- Status filters
- Retry failed executions button
- Alert configuration for failures

---

## Phase 13: Reality Portal Development

#### Epic 44: Reality Portal Foundation (reality-web)

**Goal:** Build the Reality Portal public website with Next.js SSR/SSG.

**Target Apps:** reality-web (Next.js)
**Estimate:** 6 stories, ~3 weeks

#### Story 44.1: Homepage & Navigation

As a **portal visitor**,
I want to **see a professional homepage with search**,
So that **I can start browsing listings**.

**Acceptance Criteria:**

**Given** a user visits the portal
**When** the homepage loads
**Then** they see: hero search, featured listings, categories
**And** navigation to sale/rent sections

**Technical Notes:**
- Next.js App Router with SSG
- Hero section with search form
- Featured listings carousel
- SEO meta tags

---

#### Story 44.2: Listing Search & Filters

As a **portal user**,
I want to **search and filter property listings**,
So that **I can find properties matching my criteria**.

**Acceptance Criteria:**

**Given** listings exist
**When** a user applies filters
**Then** results update in real-time
**And** URL reflects filter state for sharing

**Technical Notes:**
- Filter sidebar (price, rooms, size, type)
- Map/list view toggle
- URL query string sync
- Infinite scroll or pagination

---

#### Story 44.3: Listing Detail Page

As a **portal user**,
I want to **view complete listing details**,
So that **I can make an informed decision**.

**Acceptance Criteria:**

**Given** a user clicks a listing
**When** the detail page loads
**Then** they see: photos, description, features, location map
**And** contact form for inquiries

**Technical Notes:**
- SSR for SEO
- Photo gallery with lightbox
- Map integration (Mapbox/Google)
- JSON-LD structured data

---

#### Story 44.4: Portal User Authentication

As a **portal user**,
I want to **create an account and log in**,
So that **I can save favorites and track inquiries**.

**Acceptance Criteria:**

**Given** a user registers or logs in
**When** authentication completes
**Then** they have access to favorites, saved searches
**And** their inquiry history

**Technical Notes:**
- Registration/login forms
- OAuth (Google, Apple, Facebook)
- JWT token management
- Protected routes

---

#### Story 44.5: Favorites & Saved Searches

As a **portal user**,
I want to **save listings and searches**,
So that **I can easily find them later**.

**Acceptance Criteria:**

**Given** a user is logged in
**When** they favorite a listing or save a search
**Then** it appears in their dashboard
**And** alerts can be enabled

**Technical Notes:**
- Heart icon on listing cards
- Saved searches dashboard
- Alert toggle per saved search

---

#### Story 44.6: Contact & Inquiry Forms

As a **portal user**,
I want to **contact listing owners/agents**,
So that **I can schedule viewings or ask questions**.

**Acceptance Criteria:**

**Given** a user views a listing
**When** they submit an inquiry
**Then** the message is sent to the listing agent
**And** appears in the user's inquiry history

**Technical Notes:**
- Contact form with validation
- Viewing request scheduler
- Email notification to agent

---

#### Epic 45: Reality Portal Agency Features (reality-web)

**Goal:** Build agency management and realtor tools for Reality Portal.

**Target Apps:** reality-web
**Estimate:** 4 stories, ~2 weeks

#### Story 45.1: Agency Dashboard

As an **agency owner**,
I want to **view my agency's performance**,
So that **I can track business metrics**.

**Acceptance Criteria:**

**Given** an agency exists
**When** the owner views the dashboard
**Then** they see: listings, inquiries, views, conversions
**And** realtor performance comparison

**Technical Notes:**
- Dashboard with charts
- Date range selector
- Realtor leaderboard

---

#### Story 45.2: Realtor Management

As an **agency owner**,
I want to **invite and manage realtors**,
So that **they can list properties under our brand**.

**Acceptance Criteria:**

**Given** an agency is active
**When** the owner invites a realtor
**Then** an invitation email is sent
**And** the realtor appears in the team list upon acceptance

**Technical Notes:**
- Invitation form with email
- Pending invitations list
- Realtor cards with actions

---

#### Story 45.3: Realtor Listing Management

As a **realtor**,
I want to **create and manage my listings**,
So that **I can market properties effectively**.

**Acceptance Criteria:**

**Given** a realtor is part of an agency
**When** they create a listing
**Then** it's associated with their profile and agency
**And** appears in public search

**Technical Notes:**
- Listing form with photo upload
- Status management (draft, active, sold)
- Analytics per listing

---

#### Story 45.4: Agency Branding

As an **agency owner**,
I want to **customize my agency's branding**,
So that **our listings are recognizable**.

**Acceptance Criteria:**

**Given** an agency is active
**When** the owner uploads logo and sets colors
**Then** branding appears on all agency listings
**And** on the agency profile page

**Technical Notes:**
- Logo upload with preview
- Color picker for primary/secondary
- Branding preview

---

#### Epic 46: Reality Portal Import (reality-web)

**Goal:** Build property import functionality from CSV, CRM, and feeds.

**Target Apps:** reality-web
**Estimate:** 4 stories, ~2 weeks

#### Story 46.1: CSV Import

As a **realtor**,
I want to **bulk import listings from CSV**,
So that **I can migrate data quickly**.

**Acceptance Criteria:**

**Given** a realtor uploads a CSV
**When** the system processes it
**Then** they see a preview with validation
**And** can confirm to create listings

**Technical Notes:**
- File upload with drag-and-drop
- Column mapping interface
- Validation error display
- Import progress indicator

---

#### Story 46.2: CRM Connection

As an **agency owner**,
I want to **connect my CRM system**,
So that **listings sync automatically**.

**Acceptance Criteria:**

**Given** an agency configures CRM
**When** credentials are validated
**Then** connection status is shown
**And** field mapping can be configured

**Technical Notes:**
- CRM type selector
- API credentials form
- Connection test button
- Field mapping UI

---

#### Story 46.3: Automatic Sync Schedule

As an **agency owner**,
I want to **schedule automatic syncs**,
So that **listings stay up-to-date**.

**Acceptance Criteria:**

**Given** a CRM is connected
**When** the owner sets a schedule
**Then** syncs run automatically
**And** sync history is available

**Technical Notes:**
- Schedule frequency selector
- Sync history log
- Manual sync trigger button

---

#### Story 46.4: XML/RSS Feed Import

As an **agency owner**,
I want to **import from XML/RSS feeds**,
So that **I can aggregate multiple sources**.

**Acceptance Criteria:**

**Given** a feed URL is configured
**When** the system fetches it
**Then** listings are parsed and previewed
**And** can be imported with one click

**Technical Notes:**
- Feed URL input
- Feed validation
- Preview parsed listings
- Deduplication handling

---

## Phase 14: Mobile Applications

#### Epic 47: PPT Mobile App (mobile - React Native)

**Goal:** Complete the Property Management mobile app with all features.

**Target Apps:** mobile (React Native)
**Estimate:** 8 stories, ~4 weeks

#### Story 47.1: Mobile Authentication

As a **mobile user**,
I want to **log in securely on my phone**,
So that **I can access property management features**.

**Acceptance Criteria:**

**Given** a user opens the app
**When** they log in with email/password or biometrics
**Then** they're authenticated
**And** can access their buildings

**Technical Notes:**
- Login form
- Biometric authentication (Face ID/Touch ID)
- Secure token storage
- Remember me functionality

---

#### Story 47.2: Mobile Dashboard

As a **mobile user**,
I want to **see an overview of my building**,
So that **I can stay informed on the go**.

**Acceptance Criteria:**

**Given** a user opens the app
**When** they view the dashboard
**Then** they see: notifications, announcements, pending actions
**And** can navigate to any section

**Technical Notes:**
- Dashboard cards
- Pull-to-refresh
- Bottom navigation
- Badge counts

---

#### Story 47.3: Mobile Fault Reporting

As a **mobile user**,
I want to **report faults with photos**,
So that **I can quickly document issues**.

**Acceptance Criteria:**

**Given** a user wants to report a fault
**When** they fill the form and attach photos
**Then** the fault is submitted
**And** appears in their fault list

**Technical Notes:**
- Camera integration
- Photo gallery picker
- Location auto-detection
- Offline queue for poor connectivity

---

#### Story 47.4: Mobile Announcements

As a **mobile user**,
I want to **view and comment on announcements**,
So that **I stay informed about building news**.

**Acceptance Criteria:**

**Given** announcements exist
**When** a user views the list
**Then** they see announcement cards with previews
**And** can read full content and comment

**Technical Notes:**
- Announcement list with cards
- Pull-to-refresh
- Comment form
- Attachment viewer

---

#### Story 47.5: Mobile Voting

As a **owner on mobile**,
I want to **cast votes on the go**,
So that **I can participate from anywhere**.

**Acceptance Criteria:**

**Given** active votes exist
**When** an owner views them
**Then** they can see options and cast their vote
**And** see results when voting closes

**Technical Notes:**
- Vote cards with countdown
- Option selection
- Confirmation dialog
- Results charts

---

#### Story 47.6: Mobile Documents

As a **mobile user**,
I want to **browse and download documents**,
So that **I can access them anytime**.

**Acceptance Criteria:**

**Given** documents are available
**When** a user browses the document tree
**Then** they can navigate folders and download files
**And** view PDFs in-app

**Technical Notes:**
- Folder tree navigation
- File download
- PDF viewer integration
- Offline document cache

---

#### Story 47.7: Mobile Push Notifications

As a **mobile user**,
I want to **receive push notifications**,
So that **I'm alerted to important events**.

**Acceptance Criteria:**

**Given** a user has enabled notifications
**When** events occur (new announcement, fault update)
**Then** they receive push notifications
**And** tapping opens the relevant screen

**Technical Notes:**
- FCM/APNs integration
- Deep linking
- Notification preferences sync
- Badge count updates

---

#### Story 47.8: Mobile Offline Support

As a **mobile user**,
I want to **access cached data offline**,
So that **I can use the app without internet**.

**Acceptance Criteria:**

**Given** a user has viewed content
**When** they go offline
**Then** cached content is still accessible
**And** actions queue for sync when online

**Technical Notes:**
- SQLite local storage
- Action queue for offline mutations
- Sync indicator
- Conflict resolution

---

#### Epic 48: Reality Portal Mobile (mobile-native - KMP)

**Goal:** Build the Reality Portal mobile app with Kotlin Multiplatform.

**Target Apps:** mobile-native (KMP - Android/iOS)
**Estimate:** 6 stories, ~3 weeks

#### Story 48.1: Portal Mobile Search

As a **mobile portal user**,
I want to **search for properties on my phone**,
So that **I can browse listings anywhere**.

**Acceptance Criteria:**

**Given** the app is open
**When** a user enters search criteria
**Then** matching listings appear
**And** can be filtered and sorted

**Technical Notes:**
- KMP shared business logic
- Compose UI (Android)
- SwiftUI (iOS)
- Location-based search

---

#### Story 48.2: Portal Mobile Listing View

As a **mobile portal user**,
I want to **view listing details on my phone**,
So that **I can see photos and information**.

**Acceptance Criteria:**

**Given** a listing is selected
**When** the detail screen opens
**Then** all photos, details, and map are shown
**And** contact options are available

**Technical Notes:**
- Photo gallery with swipe
- Native map integration
- Share functionality
- Call/email agent buttons

---

#### Story 48.3: Portal Mobile Favorites

As a **mobile portal user**,
I want to **save favorites on my phone**,
So that **I can quickly access them later**.

**Acceptance Criteria:**

**Given** a user is logged in
**When** they favorite a listing
**Then** it appears in their favorites tab
**And** syncs across devices

**Technical Notes:**
- Heart button on listings
- Favorites tab
- Local + remote sync
- Offline favorites access

---

#### Story 48.4: Portal Mobile Alerts

As a **mobile portal user**,
I want to **receive alerts for new listings**,
So that **I don't miss opportunities**.

**Acceptance Criteria:**

**Given** alerts are enabled for a saved search
**When** new matching listings appear
**Then** a push notification is sent
**And** opens the listing when tapped

**Technical Notes:**
- FCM/APNs push
- Alert configuration UI
- Deep linking to listings

---

#### Story 48.5: Portal Mobile Account

As a **mobile portal user**,
I want to **manage my account on mobile**,
So that **I can update preferences**.

**Acceptance Criteria:**

**Given** a user is logged in
**When** they access profile settings
**Then** they can update info, preferences, and logout
**And** manage linked social accounts

**Technical Notes:**
- Profile edit form
- OAuth account linking
- Logout with confirmation

---

#### Story 48.6: Portal Mobile Inquiries

As a **mobile portal user**,
I want to **send and track inquiries**,
So that **I can contact agents easily**.

**Acceptance Criteria:**

**Given** a user views a listing
**When** they send an inquiry
**Then** it appears in their inquiry history
**And** they're notified of responses

**Technical Notes:**
- Inquiry form
- Inquiry history list
- Push notifications for responses

---

#### Epic 49: Mobile Native Features (mobile, mobile-native)

**Goal:** Implement platform-specific mobile features (widgets, voice, NFC).

**Target Apps:** mobile (React Native), mobile-native (KMP)
**Estimate:** 4 stories, ~2.5 weeks

#### Story 49.1: Home Screen Widgets

As a **mobile user**,
I want to **add widgets to my home screen**,
So that **I see important info at a glance**.

**Acceptance Criteria:**

**Given** a user configures a widget
**When** added to home screen
**Then** it shows: notifications count, latest announcement, etc.
**And** updates automatically

**Technical Notes:**
- iOS WidgetKit
- Android App Widgets
- Widget update scheduler
- Deep link on tap

---

#### Story 49.2: Voice Assistant Integration

As a **mobile user**,
I want to **use voice commands**,
So that **I can interact hands-free**.

**Acceptance Criteria:**

**Given** voice integration is enabled
**When** a user says "Report elevator fault"
**Then** the app opens fault reporting pre-filled
**And** confirms action via voice

**Technical Notes:**
- SiriKit intents (iOS)
- Google Assistant App Actions (Android)
- Voice command parsing

---

#### Story 49.3: QR Code Scanning

As a **mobile user**,
I want to **scan QR codes**,
So that **I can quickly access features**.

**Acceptance Criteria:**

**Given** a user scans a QR code
**When** it contains an app deep link
**Then** the app opens to the relevant screen
**And** pre-fills any included data

**Technical Notes:**
- Native camera QR scanning
- Deep link handler
- QR code generator for sharing

---

#### Story 49.4: NFC Building Access

As a **resident**,
I want to **use my phone for building access**,
So that **I don't need a separate key fob**.

**Acceptance Criteria:**

**Given** NFC is configured for the building
**When** a user taps their phone on the reader
**Then** access is granted if authorized
**And** logged for security

**Technical Notes:**
- Android HCE implementation
- Apple Wallet pass (iOS)
- Secure credential storage
- Access control integration

---

## Phase 15: Advanced Platform Features

#### Epic 50: Onboarding & Help (All Apps)

**Goal:** Implement interactive onboarding and help system across all apps.

**Target Apps:** ppt-web, reality-web, mobile, mobile-native
**Estimate:** 4 stories, ~2 weeks

#### Story 50.1: Interactive Onboarding Tour

As a **new user**,
I want to **complete an onboarding tour**,
So that **I understand how to use the app**.

**Acceptance Criteria:**

**Given** a user logs in for the first time
**When** the tour starts
**Then** they're guided through key features
**And** can skip or pause anytime

**Technical Notes:**
- react-joyride (web)
- Custom overlay (mobile)
- Progress saving
- Completion tracking

---

#### Story 50.2: Contextual Help

As a **user**,
I want to **access help for my current screen**,
So that **I understand features without leaving**.

**Acceptance Criteria:**

**Given** a user clicks the help icon
**When** on any screen
**Then** relevant help content appears
**And** can expand for more details

**Technical Notes:**
- Help icon in header
- Slide-out help panel
- Screen-keyed content
- Links to full documentation

---

#### Story 50.3: FAQ & Tutorials

As a **user**,
I want to **search FAQs and watch tutorials**,
So that **I can learn at my own pace**.

**Acceptance Criteria:**

**Given** a user opens the help center
**When** they search FAQs
**Then** relevant Q&A appears
**And** video tutorials are available

**Technical Notes:**
- FAQ search with full-text
- Video embed (YouTube/Vimeo)
- Tutorial completion tracking

---

#### Story 50.4: Feedback & Bug Reports

As a **user**,
I want to **submit feedback and bug reports**,
So that **I can help improve the app**.

**Acceptance Criteria:**

**Given** a user wants to report an issue
**When** they fill the feedback form
**Then** device info is auto-attached
**And** optional screenshot can be added

**Technical Notes:**
- Feedback form
- Screenshot capture
- Device/app version auto-fill
- Admin dashboard for review

---

#### Epic 51: Property Comparison (reality-web, mobile-native)

**Goal:** Implement property comparison feature for Reality Portal.

**Target Apps:** reality-web, mobile-native
**Estimate:** 3 stories, ~1.5 weeks

#### Story 51.1: Add to Comparison

As a **portal user**,
I want to **add listings to comparison**,
So that **I can compare them side by side**.

**Acceptance Criteria:**

**Given** listings are being browsed
**When** a user clicks "Compare"
**Then** the listing is added to comparison tray
**And** max 4 listings can be compared

**Technical Notes:**
- Compare button on listing cards
- Floating comparison tray
- Max limit enforcement

---

#### Story 51.2: Comparison View

As a **portal user**,
I want to **see listings compared side by side**,
So that **I can evaluate differences**.

**Acceptance Criteria:**

**Given** listings are in comparison
**When** the user opens comparison view
**Then** properties are shown in columns
**And** key features are aligned for comparison

**Technical Notes:**
- Column layout
- Feature rows (price, size, rooms, etc.)
- Highlight differences
- Remove from comparison

---

#### Story 51.3: Share Comparison

As a **portal user**,
I want to **share my comparison**,
So that **others can see my shortlist**.

**Acceptance Criteria:**

**Given** a comparison is created
**When** the user shares it
**Then** a shareable link is generated
**And** can be exported to PDF

**Technical Notes:**
- Share link generation
- PDF export
- Email sharing

---

#### Epic 52: Advanced Financial UI (ppt-web)

**Goal:** Build advanced financial management interfaces.

**Target Apps:** ppt-web
**Estimate:** 4 stories, ~2 weeks

#### Story 52.1: Financial Dashboard

As a **manager**,
I want to **see financial overview for buildings**,
So that **I can track income and expenses**.

**Acceptance Criteria:**

**Given** financial data exists
**When** the manager views the dashboard
**Then** they see: total balance, pending payments, overdue
**And** charts showing trends

**Technical Notes:**
- Dashboard cards with metrics
- Line/bar charts
- Date range selector
- Building filter

---

#### Story 52.2: Invoice Management

As a **manager**,
I want to **create and send invoices**,
So that **owners receive proper billing**.

**Acceptance Criteria:**

**Given** a manager creates an invoice
**When** they fill in details and send
**Then** the invoice is emailed to the owner
**And** appears in their payment history

**Technical Notes:**
- Invoice form with line items
- PDF generation
- Email sending
- Status tracking

---

#### Story 52.3: Payment Reconciliation

As a **manager**,
I want to **reconcile bank payments with invoices**,
So that **accounts are accurate**.

**Acceptance Criteria:**

**Given** bank payments are received
**When** the manager reconciles
**Then** payments match to invoices
**And** balances update automatically

**Technical Notes:**
- Payment matching interface
- Auto-match suggestions
- Manual match override
- Reconciliation history

---

#### Story 52.4: Budget Tracking

As a **manager**,
I want to **track budget vs actual spending**,
So that **I can manage building finances**.

**Acceptance Criteria:**

**Given** a budget is created
**When** expenses are recorded
**Then** budget vs actual is shown
**And** variance is highlighted

**Technical Notes:**
- Budget form with categories
- Expense tracking
- Variance charts
- Alert on budget overrun

---

#### Epic 53: Advanced Reports UI (ppt-web)

**Goal:** Build comprehensive reporting and analytics interfaces.

**Target Apps:** ppt-web
**Estimate:** 4 stories, ~2 weeks

#### Story 53.1: Report Builder

As a **manager**,
I want to **create custom reports**,
So that **I can analyze data my way**.

**Acceptance Criteria:**

**Given** data sources are available
**When** the manager configures a report
**Then** they can select fields, filters, and groupings
**And** preview the report

**Technical Notes:**
- Drag-and-drop field selector
- Filter builder
- Grouping options
- Preview pane

---

#### Story 53.2: Scheduled Reports

As a **manager**,
I want to **schedule automatic report delivery**,
So that **I receive reports regularly**.

**Acceptance Criteria:**

**Given** a report is created
**When** the manager sets a schedule
**Then** reports are generated automatically
**And** delivered via email

**Technical Notes:**
- Schedule configuration
- Email delivery settings
- Report history

---

#### Story 53.3: Dashboard Analytics

As a **manager**,
I want to **view building analytics dashboards**,
So that **I understand building performance**.

**Acceptance Criteria:**

**Given** analytics data exists
**When** the manager views dashboards
**Then** they see: occupancy, faults, payments, consumption
**And** can drill down into details

**Technical Notes:**
- KPI cards
- Interactive charts
- Drill-down capability
- Export to PDF

---

#### Story 53.4: Trend Analysis

As a **manager**,
I want to **analyze trends over time**,
So that **I can identify patterns and plan**.

**Acceptance Criteria:**

**Given** historical data exists
**When** the manager views trends
**Then** they see time-series charts
**And** can compare periods

**Technical Notes:**
- Time series charts
- Period comparison
- Trend line indicators
- Anomaly highlighting

---

## Implementation Targets by Application

| Application | Epics | Technology |
|-------------|-------|------------|
| **ppt-web** | 39, 40, 41, 42, 43, 50, 52, 53 | React/Vite |
| **mobile (React Native)** | 39, 40, 42, 47, 49, 50 | React Native |
| **reality-web** | 44, 45, 46, 50, 51 | Next.js |
| **mobile-native (KMP)** | 48, 49, 50, 51 | Kotlin Multiplatform |

---

## Phase Summary

| Phase | Epics | Stories | Estimate |
|-------|-------|---------|----------|
| Phase 12 (PPT Frontend) | 39, 40, 41, 42, 43 | 18 | ~8.5 weeks |
| Phase 13 (Reality Portal) | 44, 45, 46 | 14 | ~7 weeks |
| Phase 14 (Mobile Apps) | 47, 48, 49 | 18 | ~9.5 weeks |
| Phase 15 (Advanced) | 50, 51, 52, 53 | 15 | ~7.5 weeks |
| **Total Part 4** | **15 Epics** | **65 Stories** | **~32.5 weeks** |

---

## Sprint Plan

### Phase 12 Sprints
| Sprint | Epics | Stories | Rationale |
|--------|-------|---------|-----------|
| 12A | Epic 39 (1-4) | 4 | Document Intelligence UI |
| 12B | Epic 40 (1-4) | 4 | Notifications UI |
| 12C | Epic 41 (1-3) | 3 | Government Portal UI |
| 12D | Epic 42 (1-4) | 4 | Community Features UI |
| 12E | Epic 43 (1-3) | 3 | Automation UI |

### Phase 13 Sprints
| Sprint | Epics | Stories | Rationale |
|--------|-------|---------|-----------|
| 13A | Epic 44 (1-3) | 3 | Portal Foundation |
| 13B | Epic 44 (4-6) | 3 | Portal User Features |
| 13C | Epic 45 (1-4) | 4 | Agency Features |
| 13D | Epic 46 (1-4) | 4 | Import Features |

### Phase 14 Sprints
| Sprint | Epics | Stories | Rationale |
|--------|-------|---------|-----------|
| 14A | Epic 47 (1-4) | 4 | PPT Mobile Core |
| 14B | Epic 47 (5-8) | 4 | PPT Mobile Features |
| 14C | Epic 48 (1-3) | 3 | Portal Mobile Core |
| 14D | Epic 48 (4-6) | 3 | Portal Mobile Features |
| 14E | Epic 49 (1-4) | 4 | Native Features |

### Phase 15 Sprints
| Sprint | Epics | Stories | Rationale |
|--------|-------|---------|-----------|
| 15A | Epic 50 (1-4) | 4 | Onboarding & Help |
| 15B | Epic 51 (1-3) | 3 | Property Comparison |
| 15C | Epic 52 (1-4) | 4 | Financial UI |
| 15D | Epic 53 (1-4) | 4 | Reports UI |

---

## Cumulative Project Summary (All Parts)

| Part | Phases | Epics | Stories | Weeks |
|------|--------|-------|---------|-------|
| Part 1 | 1-3 | 17 | 97 | ~29 |
| Part 2 | 4-7 | 16 | 68 | ~31 |
| Part 3 | 8-11 | 11 | 42 | ~21.5 |
| Part 4 | 12-15 | 15 | 65 | ~32.5 |
| **Grand Total** | **15 Phases** | **59 Epics** | **272 Stories** | **~114 weeks** |

---

## Priority Recommendations

### Immediate Priority (Next Sprint)
1. **Epic 44: Reality Portal Foundation** - Enables public listing visibility
2. **Epic 47: PPT Mobile App** - Critical for user adoption

### High Priority (Next Quarter)
1. **Epic 42: Community Features UI** - High engagement feature
2. **Epic 48: Reality Portal Mobile** - Expands market reach
3. **Epic 52: Financial UI** - Core business functionality

### Medium Priority (This Year)
1. **Epic 39-41: Document, Notifications, Government UI**
2. **Epic 45-46: Agency and Import Features**
3. **Epic 50: Onboarding & Help**

### Lower Priority (Future)
1. **Epic 43: Automation UI** - Advanced feature
2. **Epic 49: Native Features** - Enhancement
3. **Epic 51, 53: Comparison, Reports** - Nice-to-have

---

*End of Epic Breakdown Part 4*
