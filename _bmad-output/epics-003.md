---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - _bmad-output/prd.md
  - _bmad-output/architecture.md
  - _bmad-output/ux-design-specification.md
  - _bmad-output/epics.md
  - _bmad-output/epics-002.md
  - docs/use-cases.md
  - docs/functional-requirements.md
  - docs/non-functional-requirements.md
workflowType: 'epics-and-stories'
lastStep: 4
status: 'complete'
project_name: 'Property Management System (PPT) & Reality Portal'
user_name: 'Martin Janci'
date: '2025-12-23'
continues_from: 'epics-002.md'
phase_range: '8, 9, 10, 11'
epic_range: '28-40'
---

# Property Management System (PPT) & Reality Portal - Epic Breakdown (Part 3)

## Overview

This document continues from `epics-002.md` and provides epic and story breakdown for **Phases 8, 9, 10, and 11** - covering deferred items, Reality Portal enhancements, advanced integrations, and mobile-native features.

**Continuation from:** `epics-002.md` (Epics 18-27, Phases 4-7)

**Implementation Status (as of 2025-12-23):**
- **Completed:** Epics 1-25 (all implemented)
- **In Progress:** Epic 26 (Platform Subscription & Billing) - PR pending
- **Remaining from Part 2:** Epic 27 (Property Comparison & Analytics)

---

## Deferred Items Review

### Items Deferred from Previous Epics

| Epic | Deferred Item | Original Note | New Epic |
|------|---------------|---------------|----------|
| 7A | OCR and full-text search for documents | "deferred to Epic 7B" | Epic 28 |
| 8A | Granular notification preferences | "deferred to Epic 8B" | Epic 29 |
| 10A | SSO consumer on reality-server | "deferred to Phase 4" | ✅ Done (10A-SSO) |
| 18.4 | Government portal API integration | "future enhancement" | Epic 30 |
| 13 | Real ML for AI features | "Initial: keyword matching, real ML in Phase 3" | Epic 31 |

---

## Requirements Inventory (New FRs)

### Functional Requirements (FR126-FR165)

**Phase 8: Deferred & Enhanced Features (NEW):**

**CA-24: Document Intelligence (4 FRs)**
- FR126: System can extract text from documents using OCR (UC-20.2)
- FR127: Users can full-text search document contents (UC-08.1-enhanced)
- FR128: System can auto-classify documents by content (UC-20.7)
- FR129: System can summarize long documents (UC-20.8)

**CA-25: Advanced Notifications (4 FRs)**
- FR130: Users can configure granular notification preferences per category (UC-01.3-enhanced)
- FR131: Users can set quiet hours and do-not-disturb schedules (UC-01.3)
- FR132: Users can configure notification digest (hourly/daily summary) (UC-01)
- FR133: System provides smart notification grouping and priority (UC-01)

**CA-26: Government Integration (3 FRs)**
- FR134: System can submit guest reports to government portals (SK: ÚHÚL) (UC-30.5)
- FR135: System can submit guest reports to CZ foreign police (UC-30.5)
- FR136: System supports automatic compliance verification (UC-38)

**Phase 9: Reality Portal Professional (NEW):**

**CA-27: Portal Favorites & Alerts (4 FRs)**
- FR137: Portal users can save listings to favorites (UC-44.1-44.6)
- FR138: Portal users can create saved searches with alerts (UC-45.1-45.8)
- FR139: System sends price change alerts to interested users (UC-45.7-45.8)
- FR140: Users can share and export favorites (UC-44.4-44.5)

**CA-28: Agency Management (4 FRs)**
- FR141: Agency owners can create and manage reality agencies (UC-49.1-49.10)
- FR142: Agencies can invite and manage realtors (UC-47.13-47.15)
- FR143: Agencies can view performance dashboards (UC-49.7)
- FR144: Agencies can configure branding and settings (UC-49.3, UC-49.10)

**CA-29: Realtor Tools (4 FRs)**
- FR145: Realtors can create professional profiles (UC-51.1-51.2)
- FR146: Realtors can manage their listings (UC-51.3-51.7)
- FR147: Realtors can respond to inquiries and schedule viewings (UC-51.8-51.9, UC-46.5-46.6)
- FR148: Realtors can view listing analytics (UC-51.10)

**CA-30: Property Import (4 FRs)**
- FR149: Users can import listings from CSV/Excel (UC-50.8)
- FR150: Users can connect external CRM systems (UC-50.1-50.3)
- FR151: Agencies can configure automatic sync schedules (UC-50.4)
- FR152: System can import from XML/RSS feeds (UC-50.7)

**Phase 10: Mobile Native & Advanced UX (NEW):**

**CA-31: Mobile Native Features (4 FRs)**
- FR153: Users can add home screen widgets (UC-43.1)
- FR154: Users can use voice assistant integration (UC-43.3)
- FR155: Users can scan QR codes for quick access (UC-43.4)
- FR156: Users can use NFC for building access (UC-43.5)

**CA-32: Onboarding & Help (4 FRs)**
- FR157: New users complete interactive onboarding tour (UC-42.1)
- FR158: Users can access contextual help and tooltips (UC-42.2)
- FR159: Users can search FAQ and watch tutorials (UC-42.3-42.4)
- FR160: Users can submit feedback and report bugs (UC-42.6-42.8)

**Phase 11: Community & Social (NEW):**

**CA-33: Community Features (4 FRs)**
- FR161: Residents can create and join community groups (UC-24.1-24.3)
- FR162: Residents can post and share in community feed (UC-24.4-24.6)
- FR163: Residents can organize community events (UC-24.7-24.8)
- FR164: Residents can trade/share items in marketplace (UC-24.9-24.10)

**CA-34: Advanced Workflow Automation (1 FR)**
- FR165: Managers can create custom workflow automation rules (UC-26.1-26.8)

---

### Non-Functional Requirements (Phase 8-11 Specific)

- NFR46: OCR text extraction < 5s per page
- NFR47: Document full-text search < 500ms
- NFR48: Notification digest generation < 30s
- NFR49: Government portal submission with retry/fallback
- NFR50: Portal favorites sync < 1s
- NFR51: Agency dashboard load < 2s
- NFR52: CSV import: 1000 listings < 60s
- NFR53: Widget update refresh < 5s
- NFR54: NFC authentication < 1s
- NFR55: Onboarding tour completion rate > 70%

---

### FR Coverage Map

| FR | Epic | Description | Target Apps |
|----|------|-------------|-------------|
| FR126 | 28 | Document OCR | api-server, ppt-web |
| FR127 | 28 | Document full-text search | api-server, ppt-web, mobile |
| FR128 | 28 | Document auto-classification | api-server |
| FR129 | 28 | Document summarization | api-server, ppt-web |
| FR130 | 29 | Granular notification preferences | api-server, ppt-web, mobile |
| FR131 | 29 | Quiet hours / DND | api-server, ppt-web, mobile |
| FR132 | 29 | Notification digest | api-server |
| FR133 | 29 | Smart notification grouping | api-server, ppt-web, mobile |
| FR134 | 30 | SK ÚHÚL integration | api-server |
| FR135 | 30 | CZ foreign police integration | api-server |
| FR136 | 30 | Compliance verification | api-server, ppt-web |
| FR137 | 31 | Portal favorites | reality-server, reality-web, mobile-native |
| FR138 | 31 | Saved searches with alerts | reality-server, reality-web, mobile-native |
| FR139 | 31 | Price change alerts | reality-server |
| FR140 | 31 | Favorites share/export | reality-web, mobile-native |
| FR141 | 32 | Agency creation/management | reality-server, reality-web |
| FR142 | 32 | Realtor invitations | reality-server, reality-web |
| FR143 | 32 | Agency dashboards | reality-server, reality-web |
| FR144 | 32 | Agency branding | reality-server, reality-web |
| FR145 | 33 | Realtor profiles | reality-server, reality-web, mobile-native |
| FR146 | 33 | Listing management | reality-server, reality-web, mobile-native |
| FR147 | 33 | Inquiry handling | reality-server, reality-web, mobile-native |
| FR148 | 33 | Listing analytics | reality-server, reality-web |
| FR149 | 34 | CSV/Excel import | reality-server, reality-web |
| FR150 | 34 | CRM integration | reality-server, reality-web |
| FR151 | 34 | Automatic sync | reality-server |
| FR152 | 34 | XML/RSS import | reality-server |
| FR153 | 35 | Home screen widgets | mobile, mobile-native |
| FR154 | 35 | Voice assistant | mobile, mobile-native |
| FR155 | 35 | QR code scanning | mobile, mobile-native |
| FR156 | 35 | NFC access | mobile, mobile-native |
| FR157 | 36 | Onboarding tour | ppt-web, reality-web, mobile, mobile-native |
| FR158 | 36 | Contextual help | ppt-web, reality-web, mobile, mobile-native |
| FR159 | 36 | FAQ and tutorials | ppt-web, reality-web |
| FR160 | 36 | Feedback/bug reports | ppt-web, reality-web, mobile, mobile-native |
| FR161 | 37 | Community groups | api-server, ppt-web, mobile |
| FR162 | 37 | Community feed | api-server, ppt-web, mobile |
| FR163 | 37 | Community events | api-server, ppt-web, mobile |
| FR164 | 37 | Item marketplace | api-server, ppt-web, mobile |
| FR165 | 38 | Workflow automation | api-server, ppt-web |

---

## Epic List

### Phase 8: Deferred & Enhanced Features

#### Epic 28: Document Intelligence
**Goal:** Enable OCR text extraction, full-text search, auto-classification, and AI summarization for documents.

**FRs covered:** FR126, FR127, FR128, FR129
**Target Apps:** api-server, ppt-web, mobile (React Native)
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 7A (Documents), Epic 13 (AI)

---

#### Epic 29: Advanced Notification Preferences
**Goal:** Users can configure granular notification preferences, quiet hours, digests, and smart grouping.

**FRs covered:** FR130, FR131, FR132, FR133
**Target Apps:** api-server, ppt-web, mobile (React Native)
**Estimate:** 4 stories, ~1.5 weeks
**Dependencies:** Epic 2B (Notifications), Epic 8A (Preferences)

---

#### Epic 30: Government Portal Integration
**Goal:** Automated submission of guest reports to government portals (SK, CZ) for legal compliance.

**FRs covered:** FR134, FR135, FR136
**Target Apps:** api-server, ppt-web
**Estimate:** 3 stories, ~1.5 weeks
**Dependencies:** Epic 18 (Short-term Rental), Epic 25 (Legal Compliance)

---

### Phase 9: Reality Portal Professional

#### Epic 31: Portal Favorites & Search Alerts
**Goal:** Portal users can save favorites, create saved searches, and receive alerts for new listings and price changes.

**FRs covered:** FR137, FR138, FR139, FR140
**Target Apps:** reality-server, reality-web, mobile-native (KMP)
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 16 (Portal Search), Epic 27 (Property Comparison)

---

#### Epic 32: Agency Management
**Goal:** Reality agencies can manage their organization, invite realtors, view performance, and configure branding.

**FRs covered:** FR141, FR142, FR143, FR144
**Target Apps:** reality-server, reality-web
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 15 (Listings), Epic 16 (Portal)

---

#### Epic 33: Realtor Tools
**Goal:** Realtors can create profiles, manage listings, respond to inquiries, and view analytics.

**FRs covered:** FR145, FR146, FR147, FR148
**Target Apps:** reality-server, reality-web, mobile-native (KMP)
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 32 (Agency), Epic 15 (Listings)

---

#### Epic 34: Property Import
**Goal:** Realtors and agencies can import listings from CSV, CRM systems, and XML/RSS feeds.

**FRs covered:** FR149, FR150, FR151, FR152
**Target Apps:** reality-server, reality-web
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 15 (Listings), Epic 32 (Agency)

---

### Phase 10: Mobile Native & Advanced UX

#### Epic 35: Mobile Native Features
**Goal:** Enhanced mobile experience with widgets, voice assistant, QR scanning, and NFC access.

**FRs covered:** FR153, FR154, FR155, FR156
**Target Apps:** mobile (React Native), mobile-native (KMP)
**Estimate:** 4 stories, ~2.5 weeks
**Dependencies:** Epic 1 (Auth), Epic 14 (IoT - for NFC)

---

#### Epic 36: Onboarding & Help System
**Goal:** Interactive onboarding, contextual help, FAQs, tutorials, and feedback collection.

**FRs covered:** FR157, FR158, FR159, FR160
**Target Apps:** ppt-web, reality-web, mobile (React Native), mobile-native (KMP)
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** None (cross-cutting)

---

### Phase 11: Community & Automation

#### Epic 37: Community & Social Features
**Goal:** Residents can participate in community groups, events, and item marketplace.

**FRs covered:** FR161, FR162, FR163, FR164
**Target Apps:** api-server, ppt-web, mobile (React Native)
**Estimate:** 4 stories, ~2.5 weeks
**Dependencies:** Epic 2A (Organizations), Epic 6 (Announcements)

---

#### Epic 38: Advanced Workflow Automation
**Goal:** Managers can create custom automation rules for recurring tasks and event-driven actions.

**FRs covered:** FR165
**Target Apps:** api-server, ppt-web
**Estimate:** 3 stories, ~1.5 weeks
**Dependencies:** Epic 13 (AI), Epic 2B (Notifications)

---

## Implementation Targets by Application

| Application | Epics | Technology |
|-------------|-------|------------|
| **api-server** | 28, 29, 30, 37, 38 | Rust/Axum |
| **ppt-web** | 28, 29, 30, 36, 37, 38 | React/Vite |
| **mobile (React Native)** | 28, 29, 35, 36, 37 | React Native |
| **reality-server** | 31, 32, 33, 34 | Rust/Axum |
| **reality-web** | 31, 32, 33, 34, 36 | Next.js |
| **mobile-native (KMP)** | 31, 33, 35, 36 | Kotlin Multiplatform |

---

## Phase Summary

| Phase | Epics | Stories | FRs | Estimate |
|-------|-------|---------|-----|----------|
| Phase 8 (Deferred) | 28, 29, 30 | 11 | 11 | ~5 weeks |
| Phase 9 (Portal Pro) | 31, 32, 33, 34 | 16 | 16 | ~8 weeks |
| Phase 10 (Mobile/UX) | 35, 36 | 8 | 8 | ~4.5 weeks |
| Phase 11 (Community) | 37, 38 | 7 | 5 | ~4 weeks |
| **Total Part 3** | **11 Epics** | **42 Stories** | **40 FRs** | **~21.5 weeks** |

---

## Sprint Plan

### Phase 8 Sprints
| Sprint | Epics | Stories | Rationale |
|--------|-------|---------|-----------|
| 8A | Epic 28 (1-4) | 4 | Document intelligence |
| 8B | Epic 29 (1-4) | 4 | Advanced notifications |
| 8C | Epic 30 (1-3) | 3 | Government integration |

### Phase 9 Sprints
| Sprint | Epics | Stories | Rationale |
|--------|-------|---------|-----------|
| 9A | Epic 31 (1-4) | 4 | Favorites & alerts |
| 9B | Epic 32 (1-4) | 4 | Agency management |
| 9C | Epic 33 (1-4) | 4 | Realtor tools |
| 9D | Epic 34 (1-4) | 4 | Property import |

### Phase 10 Sprints
| Sprint | Epics | Stories | Rationale |
|--------|-------|---------|-----------|
| 10A | Epic 35 (1-4) | 4 | Mobile native features |
| 10B | Epic 36 (1-4) | 4 | Onboarding & help |

### Phase 11 Sprints
| Sprint | Epics | Stories | Rationale |
|--------|-------|---------|-----------|
| 11A | Epic 37 (1-4) | 4 | Community features |
| 11B | Epic 38 (1-3) | 3 | Workflow automation |

---

## Cumulative Project Summary (All Parts)

| Phase | Epics | Stories | FRs | Weeks |
|-------|-------|---------|-----|-------|
| Phase 1 (MVP) | 12 | 74 | 67 | ~20 |
| Phase 2 | 5 | 23 | 13 | ~5 |
| Phase 3 | 2 | 13 | 12 | ~4 |
| Phase 4 | 8 | 33 | 14 | ~12 |
| Phase 5 | 4 | 16 | 12 | ~6.5 |
| Phase 6 | 3 | 12 | 9 | ~5.5 |
| Phase 7 | 1 | 4 | 3 | ~2 |
| Phase 8 | 3 | 11 | 11 | ~5 |
| Phase 9 | 4 | 16 | 16 | ~8 |
| Phase 10 | 2 | 8 | 8 | ~4.5 |
| Phase 11 | 2 | 7 | 5 | ~4 |
| **Grand Total** | **46 Epics** | **217 Stories** | **170 FRs** | **~76.5 weeks** |

---

# Phase 8 Stories

## Epic 28: Document Intelligence

**Goal:** Enable OCR text extraction, full-text search, auto-classification, and AI summarization for documents.

### Story 28.1: Document OCR Text Extraction

As a **document uploader**,
I want to **have text automatically extracted from uploaded documents**,
So that **document contents become searchable**.

**Acceptance Criteria:**

**Given** a user uploads a PDF or image document
**When** the upload completes
**Then** the system queues the document for OCR processing
**And** extracts text within 5 seconds per page
**And** stores extracted text for search indexing

**Given** OCR processing fails
**When** the text cannot be extracted
**Then** the document is marked as "text unavailable"
**And** an admin notification is sent if failure rate > 5%

**Technical Notes:**
- Use Tesseract OCR or cloud service (AWS Textract, Google Vision)
- Add `extracted_text`, `ocr_status`, `ocr_processed_at` to documents table
- Background job for async processing
- Support: PDF, JPG, PNG, TIFF

---

### Story 28.2: Document Full-Text Search

As a **user**,
I want to **search within document contents**,
So that **I can find documents by their text, not just titles**.

**Acceptance Criteria:**

**Given** a user enters search terms in document search
**When** they submit the search
**Then** the system searches both titles AND extracted text
**And** returns results ranked by relevance
**And** highlights matching text snippets

**Given** search returns results
**When** viewing search results
**Then** each result shows: title, matched snippet, document type, date

**Technical Notes:**
- PostgreSQL full-text search with `tsvector`/`tsquery`
- Create `document_search_index` GIN index
- Relevance ranking using `ts_rank`
- Snippet extraction with `ts_headline`

---

### Story 28.3: Document Auto-Classification

As a **manager**,
I want to **documents to be automatically categorized**,
So that **organization happens without manual effort**.

**Acceptance Criteria:**

**Given** a document is uploaded
**When** text extraction completes
**Then** the system suggests a category based on content
**And** displays confidence level

**Given** auto-classification suggests a category
**When** the user views the document
**Then** they can accept, reject, or change the suggested category

**Technical Notes:**
- ML model trained on existing categorized documents
- Categories: invoice, contract, meeting_minutes, policy, manual, correspondence
- Confidence threshold: auto-apply if > 80%, suggest if > 50%

---

### Story 28.4: Document Summarization

As a **user**,
I want to **see AI-generated summaries of long documents**,
So that **I can quickly understand document contents**.

**Acceptance Criteria:**

**Given** a document has extracted text > 500 words
**When** the user requests a summary
**Then** the system generates a 2-3 sentence summary
**And** displays key points and topics

**Given** a summary is generated
**When** viewing document details
**Then** the summary appears at the top
**And** user can expand to full text

**Technical Notes:**
- Use LLM API (OpenAI, Claude) for summarization
- Cache summaries (regenerate on document update)
- Add `ai_summary`, `summary_generated_at` to documents

---

## Epic 29: Advanced Notification Preferences

**Goal:** Users can configure granular notification preferences, quiet hours, digests, and smart grouping.

### Story 29.1: Granular Notification Preferences

As a **user**,
I want to **configure notification preferences per category**,
So that **I only receive notifications I care about**.

**Acceptance Criteria:**

**Given** a user opens notification settings
**When** they view preferences
**Then** they see toggles for each category: announcements, faults, votes, messages, documents, financial, emergencies
**And** each category has channel options: push, email, in-app

**Given** a user disables a category
**When** an event of that type occurs
**Then** they do not receive notifications for that category
**Except** emergencies which cannot be fully disabled

**Technical Notes:**
- Create `notification_preferences` table: user_id, category, push_enabled, email_enabled, in_app_enabled
- Emergency category: can disable push/email but in-app always on
- Default: all enabled

---

### Story 29.2: Quiet Hours Configuration

As a **user**,
I want to **set quiet hours when I won't receive notifications**,
So that **I'm not disturbed during sleep or meetings**.

**Acceptance Criteria:**

**Given** a user configures quiet hours
**When** they set start and end time (e.g., 22:00 - 07:00)
**Then** non-emergency notifications are held during those hours
**And** delivered when quiet hours end

**Given** an emergency notification arrives during quiet hours
**When** the notification is sent
**Then** it bypasses quiet hours and delivers immediately

**Technical Notes:**
- Add `quiet_hours_start`, `quiet_hours_end`, `quiet_hours_timezone` to user_preferences
- Queue non-emergency notifications during quiet hours
- Background job to release held notifications

---

### Story 29.3: Notification Digest

As a **user**,
I want to **receive notification summaries instead of individual alerts**,
So that **I'm not overwhelmed by frequent notifications**.

**Acceptance Criteria:**

**Given** a user enables digest mode
**When** they configure frequency (hourly, daily, weekly)
**Then** individual notifications are aggregated
**And** a single digest email/notification is sent at the configured interval

**Given** digest is generated
**When** the user views it
**Then** they see: grouped by category, counts, and most recent items

**Technical Notes:**
- Add `digest_enabled`, `digest_frequency` to notification_preferences
- Background job to compile and send digests
- Digest email template with category groupings

---

### Story 29.4: Smart Notification Grouping

As a **user**,
I want to **similar notifications grouped together**,
So that **my notification center isn't cluttered**.

**Acceptance Criteria:**

**Given** multiple notifications of the same type arrive
**When** viewing the notification center
**Then** they are grouped (e.g., "5 new comments on your fault report")
**And** can be expanded to see individual items

**Given** notifications from the same thread/context arrive
**When** displayed
**Then** they are stacked with the most recent on top

**Technical Notes:**
- Group by: entity_type + entity_id (e.g., fault_123)
- Max group size: 10 (then summarize)
- Client-side grouping for real-time updates

---

## Epic 30: Government Portal Integration

**Goal:** Automated submission of guest reports to government portals (SK, CZ) for legal compliance.

### Story 30.1: Slovak ÚHÚL Integration

As a **property manager in Slovakia**,
I want to **automatically submit guest reports to ÚHÚL**,
So that **I comply with tourism registration requirements**.

**Acceptance Criteria:**

**Given** a manager has registered guests
**When** they initiate ÚHÚL submission
**Then** the system formats data per ÚHÚL specifications
**And** submits via API or generates required file format
**And** tracks submission status

**Given** submission fails
**When** an error occurs
**Then** the system retries 3 times
**And** notifies the manager of the failure
**And** provides manual submission fallback

**Technical Notes:**
- ÚHÚL API integration (if available) or file generation
- Required fields: guest name, nationality, ID, dates, accommodation type
- Submission log for audit trail

---

### Story 30.2: Czech Foreign Police Integration

As a **property manager in Czech Republic**,
I want to **automatically submit guest reports to cizinecká policie**,
So that **I comply with foreign national registration requirements**.

**Acceptance Criteria:**

**Given** a manager has foreign guests registered
**When** they initiate police submission
**Then** the system formats data per Czech police requirements
**And** generates XML file in required format
**And** can submit electronically if API available

**Given** manual submission is needed
**When** the manager downloads the report
**Then** they receive properly formatted XML and PDF

**Technical Notes:**
- XML format per cizinecká policie specification
- Required fields differ from SK (additional: purpose of stay)
- Support both API and manual file download

---

### Story 30.3: Compliance Verification Dashboard

As a **property manager**,
I want to **see which guests have been reported and which are pending**,
So that **I can ensure complete compliance**.

**Acceptance Criteria:**

**Given** a manager views the compliance dashboard
**When** they check guest registration status
**Then** they see: registered guests, submitted reports, pending submissions, failures

**Given** submissions are pending
**When** the deadline approaches
**Then** the manager receives reminder notifications

**Technical Notes:**
- Dashboard widget showing compliance status
- Deadline tracking per country regulation
- Batch submission for multiple guests

---

# Phase 9 Stories

## Epic 31: Portal Favorites & Search Alerts

**Goal:** Portal users can save favorites, create saved searches, and receive alerts for new listings and price changes.

### Story 31.1: Favorites Management

As a **portal user**,
I want to **save listings to my favorites**,
So that **I can easily find properties I'm interested in**.

**Acceptance Criteria:**

**Given** a user views a listing
**When** they click the heart/favorite icon
**Then** the listing is added to their favorites
**And** the icon shows filled/active state

**Given** a user views their favorites
**When** they open the favorites page
**Then** they see all saved listings with thumbnails, prices, and status
**And** can remove items from favorites

**Technical Notes:**
- Create `portal_favorites` table: id, user_id, listing_id, created_at
- Heart icon on listing cards and detail pages
- Sync across devices for logged-in users

---

### Story 31.2: Saved Searches

As a **portal user**,
I want to **save my search criteria**,
So that **I can quickly run the same search again**.

**Acceptance Criteria:**

**Given** a user performs a search
**When** they click "Save this search"
**Then** the search criteria are saved with a name
**And** appear in their saved searches list

**Given** a user views saved searches
**When** they click on one
**Then** the search runs with the saved criteria

**Technical Notes:**
- Create `saved_searches` table: id, user_id, name, criteria (JSONB), created_at, alert_enabled
- Criteria: location, price_range, rooms, size, property_type, etc.

---

### Story 31.3: New Listing Alerts

As a **portal user**,
I want to **receive alerts when new listings match my saved search**,
So that **I don't miss new properties**.

**Acceptance Criteria:**

**Given** a user enables alerts on a saved search
**When** a new listing matches the criteria
**Then** the user receives a notification (push/email per preferences)
**And** the notification links to the new listing

**Given** alert frequency is configured
**When** set to daily digest
**Then** matching listings are bundled in a daily email

**Technical Notes:**
- Background job to check new listings against saved searches
- Configurable frequency: instant, daily, weekly
- Add `alert_frequency` to saved_searches

---

### Story 31.4: Price Change Alerts

As a **portal user**,
I want to **receive alerts when a favorite listing's price changes**,
So that **I can act on price reductions**.

**Acceptance Criteria:**

**Given** a user has favorites with price alerts enabled
**When** a listing's price changes
**Then** the user receives notification showing old and new price

**Given** price increases
**When** the user views the alert
**Then** they see the change clearly indicated (up/down arrow, percentage)

**Technical Notes:**
- Track `price_history` on listings
- Add `price_alert_enabled` to portal_favorites
- Calculate percentage change for display

---

## Epic 32: Agency Management

**Goal:** Reality agencies can manage their organization, invite realtors, view performance, and configure branding.

### Story 32.1: Agency Creation

As an **agency owner**,
I want to **create a reality agency on the portal**,
So that **I can manage my real estate business**.

**Acceptance Criteria:**

**Given** a user wants to create an agency
**When** they complete the agency registration form
**Then** the agency is created with: name, logo, contact info, description
**And** the user becomes the agency owner

**Given** an agency is created
**When** viewing the agency profile
**Then** it appears in the portal's agency directory

**Technical Notes:**
- Create `reality_agencies` table: id, name, logo_url, description, contact_email, phone, website, created_at
- Create `agency_members` table: id, agency_id, user_id, role (owner, manager, realtor)

---

### Story 32.2: Realtor Invitations

As an **agency owner or manager**,
I want to **invite realtors to join my agency**,
So that **they can list properties under our brand**.

**Acceptance Criteria:**

**Given** a manager wants to invite a realtor
**When** they enter the realtor's email
**Then** an invitation is sent with a unique link
**And** the invitation appears in pending invitations

**Given** a realtor clicks the invitation
**When** they accept
**Then** they are added to the agency
**And** can create listings under the agency

**Technical Notes:**
- Create `agency_invitations` table: id, agency_id, email, token, status, invited_by, expires_at
- Email template for invitation
- Token expiry: 7 days

---

### Story 32.3: Agency Dashboard

As an **agency owner or manager**,
I want to **view agency performance metrics**,
So that **I can track our business success**.

**Acceptance Criteria:**

**Given** a manager views the agency dashboard
**When** they access the analytics section
**Then** they see: total listings, active listings, total views, total inquiries, conversions

**Given** metrics are displayed
**When** viewing trends
**Then** they see charts showing: listings over time, inquiries over time, performance by realtor

**Technical Notes:**
- Aggregate from listing_analytics
- Cache dashboard data (refresh hourly)
- Realtor comparison charts

---

### Story 32.4: Agency Branding

As an **agency owner**,
I want to **configure my agency's branding**,
So that **our listings are recognizable**.

**Acceptance Criteria:**

**Given** an owner configures branding
**When** they upload logo and set colors
**Then** listings show agency branding (logo watermark on photos, agency info)

**Given** branding is configured
**When** viewing agency listings
**Then** they show: agency logo, contact info, consistent styling

**Technical Notes:**
- Add `primary_color`, `logo_watermark_position` to reality_agencies
- Image processing to add watermark to listing photos
- Agency badge on listing cards

---

## Epic 33: Realtor Tools

**Goal:** Realtors can create profiles, manage listings, respond to inquiries, and view analytics.

### Story 33.1: Realtor Profile

As a **realtor**,
I want to **create a professional profile**,
So that **clients can learn about me and my credentials**.

**Acceptance Criteria:**

**Given** a realtor sets up their profile
**When** they enter information
**Then** they can add: photo, bio, specializations, years of experience, languages, license number

**Given** a profile is complete
**When** users view the realtor's listings
**Then** they can click to see the realtor's full profile

**Technical Notes:**
- Create `realtor_profiles` table: id, user_id, photo_url, bio, specializations (array), experience_years, languages, license_number, verified_at
- Profile completeness indicator

---

### Story 33.2: Listing Management

As a **realtor**,
I want to **create and manage property listings**,
So that **I can market properties for sale or rent**.

**Acceptance Criteria:**

**Given** a realtor creates a listing
**When** they enter property details
**Then** they can add: title, description, price, property type, size, rooms, features, photos, location

**Given** a listing exists
**When** the realtor manages it
**Then** they can: edit, set status (active, pending, sold, withdrawn), feature, archive

**Technical Notes:**
- Extend existing listings table for realtor-specific fields
- Photo upload with drag-and-drop reordering
- Status workflow: draft → active → pending → sold/withdrawn

---

### Story 33.3: Inquiry Management

As a **realtor**,
I want to **respond to inquiries about my listings**,
So that **I can convert leads to clients**.

**Acceptance Criteria:**

**Given** a realtor receives an inquiry
**When** they view their inbox
**Then** they see all inquiries with: listing, requester info, message, date

**Given** the realtor responds
**When** they send a reply
**Then** the requester receives the response
**And** the inquiry status updates to "responded"

**Technical Notes:**
- Create `listing_inquiries` table: id, listing_id, user_id, message, status, created_at
- In-app messaging for inquiry threads
- Email notification on new inquiry

---

### Story 33.4: Listing Analytics

As a **realtor**,
I want to **view analytics for my listings**,
So that **I can optimize my marketing**.

**Acceptance Criteria:**

**Given** a realtor views listing analytics
**When** they select a listing
**Then** they see: views, favorites, inquiries, days on market

**Given** analytics are displayed
**When** viewing trends
**Then** they see: views over time, inquiry sources, comparison to similar listings

**Technical Notes:**
- Create `listing_analytics` table: listing_id, date, views, favorites_added, inquiries
- Daily aggregation job
- Benchmark against similar listings in area

---

## Epic 34: Property Import

**Goal:** Realtors and agencies can import listings from CSV, CRM systems, and XML/RSS feeds.

### Story 34.1: CSV/Excel Import

As a **realtor**,
I want to **bulk import listings from a spreadsheet**,
So that **I can migrate existing data quickly**.

**Acceptance Criteria:**

**Given** a realtor uploads a CSV/Excel file
**When** the system processes it
**Then** it validates the data and shows preview
**And** highlights errors or missing fields

**Given** validation passes
**When** the realtor confirms import
**Then** listings are created in draft status
**And** summary shows: imported, failed, skipped

**Technical Notes:**
- Template download with required/optional columns
- Validation: required fields, data types, location lookup
- Batch processing for large files (1000+ rows)

---

### Story 34.2: CRM Integration

As an **agency owner**,
I want to **connect my CRM system**,
So that **listings sync automatically**.

**Acceptance Criteria:**

**Given** an agency configures CRM connection
**When** they enter API credentials
**Then** the system validates the connection
**And** displays available data for mapping

**Given** field mapping is configured
**When** sync runs
**Then** new listings are imported
**And** existing listings are updated

**Technical Notes:**
- Support: major real estate CRMs (API adapters)
- Create `crm_connections` table: id, agency_id, crm_type, credentials (encrypted), field_mapping (JSONB)
- OAuth where supported

---

### Story 34.3: Automatic Sync Schedule

As an **agency owner**,
I want to **schedule automatic syncs from my CRM**,
So that **listings stay up-to-date without manual work**.

**Acceptance Criteria:**

**Given** a CRM connection is configured
**When** the owner sets a sync schedule
**Then** they can choose frequency: hourly, daily, real-time

**Given** scheduled sync runs
**When** changes are detected
**Then** listings are updated/created
**And** sync log is maintained

**Technical Notes:**
- Background job scheduler
- Sync log with: timestamp, items synced, errors
- Conflict resolution: last-write-wins or manual review

---

### Story 34.4: XML/RSS Feed Import

As an **agency owner**,
I want to **import listings from an XML or RSS feed**,
So that **I can aggregate listings from multiple sources**.

**Acceptance Criteria:**

**Given** an owner configures a feed URL
**When** the system fetches the feed
**Then** it parses listings per the feed format
**And** creates/updates listings accordingly

**Given** feed is configured
**When** automatic refresh runs
**Then** new listings are added
**And** removed listings are marked inactive

**Technical Notes:**
- Support: RSS 2.0, Atom, custom XML with mapping
- Feed validation and error handling
- Deduplication based on external ID

---

# Phase 10 Stories

## Epic 35: Mobile Native Features

**Goal:** Enhanced mobile experience with widgets, voice assistant, QR scanning, and NFC access.

### Story 35.1: Home Screen Widgets

As a **mobile user**,
I want to **add widgets to my home screen**,
So that **I can see important information at a glance**.

**Acceptance Criteria:**

**Given** a user configures a widget
**When** they add it to home screen
**Then** they can choose: notifications count, latest announcement, fault status, upcoming events

**Given** a widget is on home screen
**When** data updates
**Then** the widget refreshes within 5 seconds
**And** tapping opens the relevant app section

**Technical Notes:**
- iOS: WidgetKit
- Android: App Widgets
- KMP: Platform-specific widget implementations
- Widget types: small (count), medium (preview), large (list)

---

### Story 35.2: Voice Assistant Integration

As a **mobile user**,
I want to **use voice commands with Siri/Google Assistant**,
So that **I can interact hands-free**.

**Acceptance Criteria:**

**Given** a user has enabled voice integration
**When** they say "Report a fault in the elevator"
**Then** the app opens fault reporting pre-filled with location

**Given** voice command is recognized
**When** action is performed
**Then** user receives voice confirmation

**Technical Notes:**
- iOS: SiriKit intents
- Android: App Actions
- Supported commands: report fault, check announcements, view balance
- Voice feedback via TTS

---

### Story 35.3: QR Code Scanning

As a **mobile user**,
I want to **scan QR codes for quick access**,
So that **I can quickly access features or information**.

**Acceptance Criteria:**

**Given** a user scans a QR code
**When** it contains app deep link
**Then** the app opens to the relevant screen (document, announcement, event)

**Given** QR code is for guest check-in
**When** scanned
**Then** the guest registration form pre-fills with booking info

**Technical Notes:**
- Native camera integration
- Deep link handling for: documents, announcements, bookings, events
- Generate QR codes for sharing

---

### Story 35.4: NFC Building Access

As a **resident**,
I want to **use my phone for building access**,
So that **I don't need a separate key fob**.

**Acceptance Criteria:**

**Given** NFC access is configured for building
**When** user taps phone on reader
**Then** access is granted if authorized
**And** access log is recorded

**Given** user is not authorized
**When** they attempt access
**Then** access is denied
**And** user is notified

**Technical Notes:**
- NFC HCE (Host Card Emulation) for Android
- Apple Wallet pass for iOS
- Integration with building access control system
- Secure credential storage

---

## Epic 36: Onboarding & Help System

**Goal:** Interactive onboarding, contextual help, FAQs, tutorials, and feedback collection.

### Story 36.1: Interactive Onboarding Tour

As a **new user**,
I want to **complete an onboarding tour**,
So that **I understand how to use the application**.

**Acceptance Criteria:**

**Given** a user logs in for the first time
**When** they start the tour
**Then** they are guided through key features with tooltips
**And** can skip or pause the tour

**Given** the tour completes
**When** the user finishes
**Then** their profile is marked as onboarded
**And** they receive a "Getting Started" checklist

**Technical Notes:**
- Tour library: react-joyride (web), custom for mobile
- Steps: navigation, key features, settings
- Progress saved across sessions

---

### Story 36.2: Contextual Help

As a **user**,
I want to **access help for the current screen**,
So that **I can understand features without leaving the page**.

**Acceptance Criteria:**

**Given** a user clicks the help icon
**When** on any screen
**Then** they see help content relevant to that screen
**And** can expand for more details

**Given** help is displayed
**When** viewing it
**Then** it includes: description, common actions, tips

**Technical Notes:**
- Help content keyed by route/screen
- Markdown content with expandable sections
- CMS for help content management

---

### Story 36.3: FAQ and Tutorials

As a **user**,
I want to **search FAQs and watch tutorials**,
So that **I can learn at my own pace**.

**Acceptance Criteria:**

**Given** a user opens the help center
**When** they search FAQs
**Then** they see relevant questions and answers

**Given** tutorials are available
**When** the user watches one
**Then** they see a video walkthrough of the feature
**And** can mark as completed

**Technical Notes:**
- FAQ database with full-text search
- Video hosting: YouTube/Vimeo embed or self-hosted
- Tutorial progress tracking

---

### Story 36.4: Feedback and Bug Reports

As a **user**,
I want to **submit feedback and report bugs**,
So that **I can help improve the application**.

**Acceptance Criteria:**

**Given** a user wants to submit feedback
**When** they open the feedback form
**Then** they can enter: type (feedback, bug, feature request), description, optional screenshot

**Given** a bug is reported
**When** submitted
**Then** it includes: device info, app version, current screen
**And** user receives confirmation

**Technical Notes:**
- Create `user_feedback` table: id, user_id, type, description, screenshot_url, device_info, created_at
- Screenshot capture integration
- Admin dashboard for feedback review

---

# Phase 11 Stories

## Epic 37: Community & Social Features

**Goal:** Residents can participate in community groups, events, and item marketplace.

### Story 37.1: Community Groups

As a **resident**,
I want to **join and participate in community groups**,
So that **I can connect with neighbors who share interests**.

**Acceptance Criteria:**

**Given** a resident views available groups
**When** they browse
**Then** they see: group name, description, member count, category

**Given** a resident joins a group
**When** they are a member
**Then** they can: post, comment, see member list

**Technical Notes:**
- Create `community_groups` table: id, building_id, name, description, category, privacy (public, members_only)
- Create `group_members` table: id, group_id, user_id, role (admin, member)
- Categories: hobbies, sports, families, pets, services

---

### Story 37.2: Community Feed

As a **resident**,
I want to **post and interact in the community feed**,
So that **I can share and stay informed**.

**Acceptance Criteria:**

**Given** a resident views the feed
**When** they open community section
**Then** they see posts from their groups and building
**And** can filter by: all, my groups, building-wide

**Given** a resident creates a post
**When** they submit
**Then** it appears in relevant feeds
**And** members can like and comment

**Technical Notes:**
- Create `community_posts` table: id, group_id, user_id, content, media (array), created_at
- Create `post_interactions` table: id, post_id, user_id, type (like, comment), content

---

### Story 37.3: Community Events

As a **resident**,
I want to **organize and attend community events**,
So that **I can participate in building activities**.

**Acceptance Criteria:**

**Given** a resident creates an event
**When** they fill in details
**Then** they specify: title, description, date/time, location, capacity

**Given** an event exists
**When** residents view it
**Then** they can: RSVP, see attendees, add to calendar

**Technical Notes:**
- Create `community_events` table: id, group_id, organizer_id, title, description, date_time, location, capacity
- Create `event_rsvps` table: id, event_id, user_id, status (going, maybe, not_going)
- Calendar integration (iCal export)

---

### Story 37.4: Item Marketplace

As a **resident**,
I want to **buy, sell, or share items with neighbors**,
So that **I can declutter and find deals locally**.

**Acceptance Criteria:**

**Given** a resident lists an item
**When** they create the listing
**Then** they specify: title, description, photos, price (or "free"), category

**Given** items are listed
**When** neighbors browse
**Then** they can: filter by category, contact seller, mark as interested

**Technical Notes:**
- Create `marketplace_items` table: id, user_id, building_id, title, description, photos, price, category, status
- Categories: furniture, electronics, clothing, services, free
- Contact via in-app messaging

---

## Epic 38: Advanced Workflow Automation

**Goal:** Managers can create custom automation rules for recurring tasks and event-driven actions.

### Story 38.1: Automation Rule Builder

As a **manager**,
I want to **create automation rules**,
So that **routine tasks happen automatically**.

**Acceptance Criteria:**

**Given** a manager opens automation settings
**When** they create a new rule
**Then** they can define: trigger, conditions, actions

**Given** triggers available
**When** selecting
**Then** options include: time-based, event-based (fault created, payment received, lease expiring)

**Technical Notes:**
- Create `automation_rules` table: id, organization_id, name, trigger (JSONB), conditions (JSONB), actions (JSONB), enabled
- Visual rule builder UI
- Trigger types: schedule (cron), event

---

### Story 38.2: Automation Actions

As a **manager**,
I want to **configure what happens when automation triggers**,
So that **the right actions occur automatically**.

**Acceptance Criteria:**

**Given** a rule is triggered
**When** conditions are met
**Then** configured actions execute in order

**Given** actions available
**When** configuring
**Then** options include: send notification, create task, update status, send email, assign to user

**Technical Notes:**
- Action handlers for each action type
- Action chaining with error handling
- Execution log for audit

---

### Story 38.3: Automation Monitoring

As a **manager**,
I want to **monitor automation execution**,
So that **I can ensure rules are working correctly**.

**Acceptance Criteria:**

**Given** automations are running
**When** the manager views the dashboard
**Then** they see: active rules, recent executions, success/failure rates

**Given** an automation fails
**When** viewing the log
**Then** they see: error details, affected entities, retry options

**Technical Notes:**
- Create `automation_executions` table: id, rule_id, triggered_at, status, error_message, execution_details
- Dashboard with execution metrics
- Alert on repeated failures

---

# Validation

## FR Coverage Validation

| FR Range | Epic | Status |
|----------|------|--------|
| FR126-FR129 | Epic 28 | ✅ Covered |
| FR130-FR133 | Epic 29 | ✅ Covered |
| FR134-FR136 | Epic 30 | ✅ Covered |
| FR137-FR140 | Epic 31 | ✅ Covered |
| FR141-FR144 | Epic 32 | ✅ Covered |
| FR145-FR148 | Epic 33 | ✅ Covered |
| FR149-FR152 | Epic 34 | ✅ Covered |
| FR153-FR156 | Epic 35 | ✅ Covered |
| FR157-FR160 | Epic 36 | ✅ Covered |
| FR161-FR164 | Epic 37 | ✅ Covered |
| FR165 | Epic 38 | ✅ Covered |

**Total New FRs:** 40 (FR126-FR165)
**All FRs Mapped:** ✅ Yes

---

## Use Case Coverage

| UC Category | Epic(s) | Status |
|-------------|---------|--------|
| UC-42: Onboarding & Help | Epic 36 | ✅ Covered |
| UC-43: Mobile App Features | Epic 35 | ✅ Covered |
| UC-44: Favorites Management | Epic 31 | ✅ Covered |
| UC-45: Saved Searches & Alerts | Epic 31 | ✅ Covered |
| UC-49: Agency Management | Epic 32 | ✅ Covered |
| UC-50: Property Import | Epic 34 | ✅ Covered |
| UC-51: Realtor Profile & Listings | Epic 33 | ✅ Covered |
| UC-24: Community & Social | Epic 37 | ✅ Covered |
| UC-26: Workflow Automation | Epic 38 | ✅ Covered |

---

*End of Epic Breakdown Part 3*
