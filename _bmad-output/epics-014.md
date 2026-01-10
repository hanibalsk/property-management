---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - _bmad-output/prd.md
  - _bmad-output/architecture.md
  - _bmad-output/epics-013.md
  - _bmad-output/research/gap-analysis.md
workflowType: 'epics-and-stories'
lastStep: 4
status: 'ready'
project_name: 'Property Management System (PPT) & Reality Portal'
user_name: 'Claude'
date: '2026-01-09'
continues_from: 'epics-013.md'
phase_range: '36-38'
epic_range: '121-131'
---

# Property Management System (PPT) & Reality Portal - Epic Breakdown (Part 14)

## Overview

This document continues from `epics-013.md` and addresses remaining implementation gaps identified in the gap analysis report. These epics focus on:

1. **Phase 36: Missing Core Features** - UC-12 Outages, Push Infrastructure
2. **Phase 37: UX Improvements** - Action-first dashboards, Accessibility
3. **Phase 38: AI UI Integration** - Photo-first flows, Chatbot UI

**Continuation from:** `epics-013.md` (Epics 107-110, Phases 34-35)

**Source:** Gap analysis report (2026-01-09)

---

## Phase 36: Missing Core Features

### Epic 121: Utility Outages (UC-12)

**Goal:** Implement the complete utility outages feature - backend routes, database models, and frontend UI for viewing and managing utility service interruptions.

**Target Apps:** api-server, ppt-web, mobile
**Estimate:** 4 stories, ~3-4 days
**Dependencies:** None (greenfield feature)
**Priority:** P0 - CRITICAL (only missing UC category)

**PRD Reference:** UC-12.1 through UC-12.8

---

#### Story 121.1: Outages Database Schema & Model

As a **developer**,
I want to **have a proper database schema for utility outages**,
So that **outage data can be stored and queried efficiently**.

**Acceptance Criteria:**

**Given** the database migration system
**When** the outages migration is applied
**Then**:
  - `outages` table is created with all required fields
  - Proper indexes exist for common queries
  - RLS policies enforce tenant isolation
  - The model supports: water, electricity, gas, heating commodities
  - Both planned and emergency outage types are supported
**And** the model integrates with the notification system

**Technical Notes:**
- Create `outages` migration
- Add `outage.rs` model in `backend/crates/db/src/models/`
- Add `outage.rs` repository in `backend/crates/db/src/repositories/`
- Include status enum: planned, in_progress, resolved, cancelled

**Database Schema:**
```sql
CREATE TABLE outages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id),
    building_id UUID REFERENCES buildings(id),  -- NULL = affects all buildings

    -- Outage details
    title VARCHAR(200) NOT NULL,
    description TEXT,
    commodity VARCHAR(50) NOT NULL,  -- 'water', 'electricity', 'gas', 'heating', 'internet'
    outage_type VARCHAR(50) NOT NULL,  -- 'planned', 'emergency'
    status VARCHAR(50) NOT NULL DEFAULT 'planned',  -- 'planned', 'in_progress', 'resolved', 'cancelled'

    -- Timing
    scheduled_start TIMESTAMPTZ NOT NULL,
    scheduled_end TIMESTAMPTZ NOT NULL,
    actual_start TIMESTAMPTZ,
    actual_end TIMESTAMPTZ,

    -- Source
    source VARCHAR(100),  -- 'supplier', 'internal', 'external'
    supplier_reference VARCHAR(100),
    contact_info TEXT,

    -- Notification
    notify_residents BOOLEAN DEFAULT true,
    notification_sent_at TIMESTAMPTZ,

    -- Metadata
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_outages_organization ON outages(organization_id);
CREATE INDEX idx_outages_building ON outages(building_id);
CREATE INDEX idx_outages_status ON outages(status);
CREATE INDEX idx_outages_scheduled ON outages(scheduled_start, scheduled_end);

-- RLS Policy
ALTER TABLE outages ENABLE ROW LEVEL SECURITY;

CREATE POLICY outages_organization_policy ON outages
    FOR ALL
    USING (organization_id = current_setting('app.current_organization_id')::UUID);
```

**Files to Create:**
- `backend/crates/db/src/migrations/YYYYMMDD_create_outages.sql`
- `backend/crates/db/src/models/outage.rs`
- `backend/crates/db/src/repositories/outage.rs`

**Story Points:** 3

---

#### Story 121.2: Outages API Routes

As a **frontend developer**,
I want to **have REST API endpoints for outages**,
So that **the frontend can display and manage outages**.

**Acceptance Criteria:**

**Given** the outages model exists
**When** API endpoints are implemented
**Then**:
  - GET `/api/v1/outages` returns paginated list with filters
  - GET `/api/v1/outages/{id}` returns outage detail
  - POST `/api/v1/outages` creates new outage (manager only)
  - PUT `/api/v1/outages/{id}` updates outage (manager only)
  - DELETE `/api/v1/outages/{id}` cancels outage (manager only)
  - GET `/api/v1/outages/active` returns current/upcoming outages
**And** OpenAPI documentation is generated

**Technical Notes:**
- Add `outages.rs` in `backend/servers/api-server/src/routes/`
- Wire routes in `main.rs`
- Add utoipa annotations for OpenAPI
- Include filtering by: commodity, status, building_id, date range

**Endpoints:**
```
GET    /api/v1/outages           - List outages (with filters)
GET    /api/v1/outages/active    - Get active/upcoming outages
GET    /api/v1/outages/{id}      - Get outage detail
POST   /api/v1/outages           - Create outage (Manager+)
PUT    /api/v1/outages/{id}      - Update outage (Manager+)
POST   /api/v1/outages/{id}/start - Mark outage as started
POST   /api/v1/outages/{id}/resolve - Mark outage as resolved
DELETE /api/v1/outages/{id}      - Cancel outage (Manager+)
```

**Files to Create:**
- `backend/servers/api-server/src/routes/outages.rs`

**Files to Modify:**
- `backend/servers/api-server/src/routes/mod.rs`
- `backend/servers/api-server/src/main.rs`

**Story Points:** 3

---

#### Story 121.3: Outages Frontend Feature (ppt-web)

As a **resident**,
I want to **view current and upcoming utility outages**,
So that **I can prepare for service interruptions**.

**Acceptance Criteria:**

**Given** the outages API exists
**When** I navigate to the outages page
**Then**:
  - I see a list of current and upcoming outages
  - Outages are color-coded by commodity (water=blue, electricity=yellow, etc.)
  - I can filter by commodity type
  - I can see outage timeline (start/end times)
  - Active outages are highlighted
  - I receive notifications for new outages
**And** the UI matches the existing design system

**Technical Notes:**
- Create `frontend/apps/ppt-web/src/features/outages/` directory
- Use existing TanStack Query patterns
- Generate SDK client from OpenAPI

**Components to Create:**
```
features/outages/
├── pages/
│   ├── OutagesPage.tsx         - List view with filters
│   └── OutageDetailPage.tsx    - Detail view
├── components/
│   ├── OutageCard.tsx          - Outage summary card
│   ├── OutageTimeline.tsx      - Visual timeline
│   └── OutageFilter.tsx        - Commodity/status filter
├── hooks/
│   └── useOutages.ts           - TanStack Query hooks
└── index.ts
```

**Files to Create:**
- `frontend/apps/ppt-web/src/features/outages/` (entire directory)

**Files to Modify:**
- `frontend/apps/ppt-web/src/routes.tsx` (add routes)
- `frontend/apps/ppt-web/src/components/Navigation.tsx` (add menu item)

**Story Points:** 5

---

#### Story 121.4: Outages Management UI (Manager)

As a **property manager**,
I want to **create and manage utility outages**,
So that **residents are informed about service interruptions**.

**Acceptance Criteria:**

**Given** I am a manager
**When** I access outage management
**Then**:
  - I can create a new planned outage
  - I can create an emergency outage
  - I can update outage details
  - I can mark an outage as started/resolved
  - I can cancel an outage
  - I can choose whether to notify residents
**And** notifications are sent when specified

**Components to Add:**
```
features/outages/
├── pages/
│   └── CreateOutagePage.tsx    - Create/edit form
└── components/
    ├── OutageForm.tsx          - Outage form
    └── OutageActions.tsx       - Status change actions
```

**Story Points:** 3

---

### Epic 122: Push Notification Deep Links

**Goal:** Verify and enhance push notification infrastructure with proper deep linking so users can navigate directly from notifications to relevant screens.

**Target Apps:** mobile, mobile-native
**Estimate:** 3 stories, ~2-3 days
**Dependencies:** Existing notification system
**Priority:** P0 - CRITICAL (UX requirement)

**PRD Reference:** UX Specification - Notification Deep Links

---

#### Story 122.1: Verify Push Notification Infrastructure

As a **developer**,
I want to **verify FCM and APNs configuration**,
So that **push notifications work reliably across platforms**.

**Acceptance Criteria:**

**Given** the notification system exists
**When** push notification infrastructure is verified
**Then**:
  - FCM project configuration is correct in `firebase.json`
  - APNs certificates are current and valid
  - Push delivery works on iOS simulator
  - Push delivery works on Android emulator
  - Notification payloads include all required metadata
**And** infrastructure is documented

**Technical Tasks:**
- [ ] Audit FCM configuration
- [ ] Verify APNs certificate expiry
- [ ] Test push on iOS simulator
- [ ] Test push on Android emulator
- [ ] Document setup in `docs/infrastructure/push-notifications.md`

**Story Points:** 2

---

#### Story 122.2: Deep Link Navigation (React Native)

As a **mobile user**,
I want to **tap a notification and go directly to the relevant screen**,
So that **I can take immediate action**.

**Acceptance Criteria:**

**Given** I receive a push notification
**When** I tap it
**Then**:
  - Fault notifications navigate to FaultDetailPage
  - Announcement notifications navigate to AnnouncementDetailPage
  - Vote notifications navigate to VoteDetailPage
  - Message notifications navigate to ConversationPage
  - Outage notifications navigate to OutageDetailPage
**And** the app state is properly initialized

**Technical Notes:**
- Implement deep link handlers in React Native
- Map notification types to screen routes
- Handle app cold start vs background resume

**Files to Modify:**
- `frontend/apps/mobile/src/navigation/linking.ts`
- `frontend/apps/mobile/src/services/notifications.ts`

**Story Points:** 5

---

#### Story 122.3: Deep Link Navigation (Kotlin Multiplatform)

As a **Reality Portal mobile user**,
I want to **tap notifications and navigate to relevant screens**,
So that **I can view listing updates immediately**.

**Acceptance Criteria:**

**Given** I receive a notification about a listing
**When** I tap it
**Then**:
  - New listing notifications navigate to ListingDetailScreen
  - Price change notifications navigate to ListingDetailScreen
  - Saved search alerts navigate to SearchResultsScreen
**And** navigation works on both Android and iOS

**Technical Notes:**
- Implement deep links in KMP shared module
- Configure Android manifest for deep links
- Configure iOS Info.plist for Universal Links

**Files to Modify:**
- `mobile-native/shared/src/commonMain/kotlin/navigation/DeepLinks.kt`
- `mobile-native/androidApp/src/main/AndroidManifest.xml`
- `mobile-native/iosApp/iosApp/Info.plist`

**Story Points:** 3

---

### Epic 123: Offline Sync with Progress Indicators

**Goal:** Implement clear offline sync indicators in mobile apps so users know when actions will be synced.

**Target Apps:** mobile
**Estimate:** 3 stories, ~2-3 days
**Dependencies:** Existing offline queue
**Priority:** P1 - HIGH (UX requirement)

**PRD Reference:** UX Specification - Offline Sync Indicators

---

#### Story 123.1: Offline Detection & Queue UI

As a **mobile user with poor connectivity**,
I want to **see when I'm offline and have pending actions**,
So that **I know my actions will be saved when connectivity returns**.

**Acceptance Criteria:**

**Given** I'm using the mobile app
**When** I lose connectivity
**Then**:
  - An offline banner appears at the top of the screen
  - Pending actions show "Will sync when online" indicator
  - The pending action count is visible
  - Actions that can be performed offline still work
**And** the UI clearly communicates the offline state

**Technical Notes:**
- Use NetInfo to detect connectivity
- Implement offline banner component
- Add pending action indicators to form submissions

**Components to Create:**
```
components/
├── OfflineBanner.tsx           - Connectivity status banner
├── PendingSyncIndicator.tsx    - "Will sync" indicator
└── SyncStatusBadge.tsx         - Badge showing pending count
```

**Story Points:** 3

---

#### Story 123.2: Background Sync Progress

As a **mobile user**,
I want to **see sync progress when connectivity returns**,
So that **I know my actions are being processed**.

**Acceptance Criteria:**

**Given** I have pending actions and connectivity returns
**When** background sync starts
**Then**:
  - A sync progress indicator appears
  - Individual items show sync status (pending/syncing/synced/failed)
  - Failed syncs are highlighted with retry option
  - Success confirmation is shown when sync completes
**And** sync happens automatically without user action

**Technical Notes:**
- Implement background sync service
- Add sync status to queued items
- Show toast notifications for sync completion

**Story Points:** 3

---

#### Story 123.3: Offline Meter Reading Flow

As a **resident in an area with poor connectivity**,
I want to **submit meter readings offline**,
So that **I can complete the task from my basement/meter room**.

**Acceptance Criteria:**

**Given** I'm offline and need to submit a meter reading
**When** I capture the reading
**Then**:
  - Photo is saved locally
  - Reading is queued for sync
  - "Will sync when online" message is shown
  - The reading appears in my history as "pending"
**And** it syncs automatically when online

**Technical Notes:**
- Extend offline queue for meter readings
- Store photos locally until synced
- Update meter reading list with sync status

**Story Points:** 2

---

## Phase 37: UX Improvements

### Epic 124: Action-First Dashboard UX

**Goal:** Refactor dashboards to follow action-first UX pattern with task queues instead of navigation-heavy pages.

**Target Apps:** ppt-web, mobile
**Estimate:** 4 stories, ~4-5 days
**Dependencies:** None
**Priority:** P1 - HIGH (UX improvement)

**PRD Reference:** UX Specification - Action-First Design

---

#### Story 124.1: Manager Action Queue

As a **property manager**,
I want to **see a prioritized queue of items needing my attention**,
So that **I can process tasks efficiently**.

**Acceptance Criteria:**

**Given** I'm logged in as a manager
**When** I view my dashboard
**Then**:
  - I see a prioritized action queue
  - Items include: pending faults, pending approvals, active votes, unread messages
  - Each item shows urgency indicator
  - I can take inline actions (approve/reject) without drilling down
  - I can filter by type or urgency
**And** completing an action removes it from the queue

**Technical Notes:**
- Create ActionQueue component
- Aggregate data from multiple endpoints
- Implement inline action buttons
- Add keyboard shortcuts for power users

**Components to Create:**
```
features/dashboard/
├── components/
│   ├── ActionQueue.tsx         - Main action queue
│   ├── ActionItem.tsx          - Individual action item
│   ├── ActionFilters.tsx       - Filter by type/urgency
│   └── InlineActions.tsx       - Quick action buttons
└── hooks/
    └── useActionQueue.ts       - Aggregate action data
```

**Story Points:** 5

---

#### Story 124.2: Resident Action Queue

As a **resident**,
I want to **see what needs my attention**,
So that **I can complete tasks quickly**.

**Acceptance Criteria:**

**Given** I'm logged in as a resident
**When** I view my dashboard
**Then**:
  - I see pending items: votes to cast, meter readings due, person-months declaration
  - Items are sorted by urgency (deadlines first)
  - I can complete most tasks in < 60 seconds
  - Completed items animate out of the queue
**And** the dashboard feels responsive and efficient

**Technical Notes:**
- Create resident-focused ActionQueue variant
- Implement 60-second flow optimization
- Track task completion time analytics

**Story Points:** 3

---

#### Story 124.3: Notification Deep Link Integration

As a **user**,
I want to **tap a notification and land in the action queue**,
So that **I can process the notification quickly**.

**Acceptance Criteria:**

**Given** I receive a notification
**When** I tap it
**Then**:
  - I land on the action queue with the relevant item highlighted
  - The item is expanded or focused
  - I can take action immediately
**And** the transition feels seamless

**Story Points:** 2

---

#### Story 124.4: Keyboard Shortcuts

As a **power user**,
I want to **use keyboard shortcuts for common actions**,
So that **I can work faster**.

**Acceptance Criteria:**

**Given** I'm on the dashboard
**When** I use keyboard shortcuts
**Then**:
  - `j/k` navigate between action items
  - `Enter` opens the selected item
  - `a` approves (where applicable)
  - `r` rejects (where applicable)
  - `?` shows keyboard shortcut help
**And** shortcuts are discoverable

**Story Points:** 2

---

### Epic 125: WCAG 2.1 AA Accessibility

**Goal:** Achieve WCAG 2.1 AA compliance across all frontend applications.

**Target Apps:** ppt-web, reality-web, mobile
**Estimate:** 4 stories, ~4-5 days
**Dependencies:** None
**Priority:** P1 - HIGH (Compliance requirement)

**PRD Reference:** UC-25 Accessibility

---

#### Story 125.1: Automated Accessibility Testing

As a **developer**,
I want to **automated accessibility checks in CI**,
So that **regressions are caught early**.

**Acceptance Criteria:**

**Given** the CI pipeline
**When** tests run
**Then**:
  - axe-core scans all pages
  - Critical/serious violations fail the build
  - Reports are generated for each run
  - Violations are documented with fix suggestions
**And** new violations are caught before merge

**Technical Tasks:**
- [ ] Add @axe-core/playwright to test suite
- [ ] Configure accessibility rules
- [ ] Add CI job for accessibility checks
- [ ] Create violation report format

**Story Points:** 3

---

#### Story 125.2: Manual Accessibility Audit

As a **user with accessibility needs**,
I want to **use the application with screen readers and keyboard**,
So that **I can access all features**.

**Acceptance Criteria:**

**Given** the application
**When** I use assistive technology
**Then**:
  - All interactive elements are keyboard accessible
  - Screen readers announce content correctly
  - Focus management is logical
  - Color contrast meets 4.5:1 minimum
  - Text scales to 200% without breaking
  - High contrast mode is supported
**And** the experience is comparable to non-assistive use

**Technical Tasks:**
- [ ] Test with NVDA (Windows)
- [ ] Test with VoiceOver (Mac/iOS)
- [ ] Test with TalkBack (Android)
- [ ] Verify keyboard navigation
- [ ] Check color contrast
- [ ] Test 200% text scaling

**Story Points:** 5

---

#### Story 125.3: Fix Accessibility Violations

As a **developer**,
I want to **fix identified accessibility violations**,
So that **the application is compliant**.

**Acceptance Criteria:**

**Given** accessibility audit findings
**When** violations are fixed
**Then**:
  - All critical violations are resolved
  - All serious violations are resolved
  - Moderate violations are tracked
  - Documentation explains any exceptions
**And** automated tests pass

**Story Points:** 5

---

#### Story 125.4: High Contrast Theme

As a **user with visual impairments**,
I want to **use a high contrast theme**,
So that **I can see the interface clearly**.

**Acceptance Criteria:**

**Given** the application
**When** I enable high contrast mode
**Then**:
  - All text has 7:1 contrast ratio
  - Borders and controls are clearly visible
  - Icons have sufficient contrast
  - Focus indicators are highly visible
  - Theme persists across sessions
**And** the theme works with system high contrast mode

**Technical Tasks:**
- [ ] Create high-contrast color tokens
- [ ] Implement theme toggle in settings
- [ ] Test all components in high contrast
- [ ] Respect system contrast preference

**Story Points:** 3

---

## Phase 38: AI UI Integration

### Epic 126: AI-Assisted Fault Reporting UI

**Goal:** Implement photo-first fault reporting with AI-powered category and priority suggestions.

**Target Apps:** ppt-web, mobile
**Estimate:** 3 stories, ~3-4 days
**Dependencies:** Backend AI routes exist
**Priority:** P2 - MEDIUM (Enhancement)

**PRD Reference:** UC-20 AI/ML Features

---

#### Story 126.1: Photo-First Fault Report Flow

As a **resident**,
I want to **start a fault report by taking a photo**,
So that **the system can help categorize the issue**.

**Acceptance Criteria:**

**Given** I want to report a fault
**When** I take a photo
**Then**:
  - The photo is analyzed by AI
  - Category suggestions are shown with confidence scores
  - Priority is suggested based on severity
  - I can accept or modify suggestions
  - Description is pre-filled based on AI analysis
**And** the flow is faster than manual entry

**Technical Notes:**
- Integrate with existing AI endpoint
- Show loading state during analysis
- Display confidence indicators

**Components to Create:**
```
features/faults/
├── components/
│   ├── PhotoCapture.tsx        - Camera/upload interface
│   ├── AIAnalysisResult.tsx    - Show AI suggestions
│   └── ConfidenceIndicator.tsx - Visual confidence display
└── hooks/
    └── useAIAnalysis.ts        - AI analysis hook
```

**Story Points:** 5

---

#### Story 126.2: AI Suggestion Feedback Loop

As a **system**,
I want to **learn from user corrections**,
So that **AI suggestions improve over time**.

**Acceptance Criteria:**

**Given** AI makes a suggestion
**When** the user modifies it
**Then**:
  - The correction is recorded
  - Feedback is sent for model training
  - Analytics track suggestion accuracy
**And** the model can be improved

**Technical Tasks:**
- [ ] Track AI suggestion acceptance rate
- [ ] Record user corrections
- [ ] Implement feedback endpoint
- [ ] Add analytics dashboard

**Story Points:** 2

---

#### Story 126.3: Mobile Photo-First Flow

As a **mobile user**,
I want to **quickly report faults from my phone**,
So that **I can report issues as I encounter them**.

**Acceptance Criteria:**

**Given** I'm using the mobile app
**When** I tap "Report Fault"
**Then**:
  - Camera opens immediately
  - Photo is sent for AI analysis
  - Suggestions appear while I add details
  - Submission takes < 60 seconds
**And** the experience is optimized for mobile

**Story Points:** 3

---

### Epic 127: AI Chatbot Interface

**Goal:** Implement the frontend UI for the AI chatbot that answers common building questions.

**Target Apps:** ppt-web, mobile
**Estimate:** 3 stories, ~3-4 days
**Dependencies:** Backend ai_chat.rs exists
**Priority:** P2 - MEDIUM (Enhancement)

**PRD Reference:** UC-20.1 AI Chatbot

---

#### Story 127.1: Chat Interface Component

As a **user**,
I want to **chat with an AI assistant**,
So that **I can get quick answers about building matters**.

**Acceptance Criteria:**

**Given** I access the AI chat
**When** I ask a question
**Then**:
  - My message appears in the chat
  - AI response streams in real-time
  - Markdown formatting is rendered
  - I can ask follow-up questions
  - Chat history is preserved
**And** the interface feels conversational

**Components to Create:**
```
features/ai-chat/
├── pages/
│   └── ChatPage.tsx            - Main chat page
├── components/
│   ├── ChatInterface.tsx       - Chat container
│   ├── MessageBubble.tsx       - Individual message
│   ├── MessageInput.tsx        - Input with send button
│   ├── StreamingText.tsx       - Animated text streaming
│   └── SuggestedQuestions.tsx  - Quick question chips
└── hooks/
    └── useChat.ts              - Chat state management
```

**Story Points:** 5

---

#### Story 127.2: Suggested Questions

As a **new user**,
I want to **see suggested questions**,
So that **I know what I can ask**.

**Acceptance Criteria:**

**Given** I open the chat
**When** it's empty or I complete a conversation
**Then**:
  - Suggested questions are displayed
  - Questions are relevant to my role (resident vs manager)
  - Tapping a suggestion sends it as a message
  - Suggestions are contextual to current page
**And** users are guided to useful interactions

**Story Points:** 2

---

#### Story 127.3: Chat History & Persistence

As a **user**,
I want to **see my previous conversations**,
So that **I can reference past answers**.

**Acceptance Criteria:**

**Given** I've had previous conversations
**When** I access chat
**Then**:
  - Recent conversations are listed
  - I can continue a previous conversation
  - Old conversations can be deleted
  - Search works across conversations
**And** context is maintained within conversations

**Story Points:** 3

---

### Epic 128: OCR Meter Reading Preview

**Goal:** Show users the OCR-extracted meter reading value before submission for verification.

**Target Apps:** ppt-web, mobile
**Estimate:** 2 stories, ~2 days
**Dependencies:** Backend OCR exists
**Priority:** P2 - MEDIUM (Enhancement)

**PRD Reference:** UC-20.3 OCR Meter Readings

---

#### Story 128.1: OCR Preview UI

As a **resident submitting a meter reading**,
I want to **see the OCR-extracted value**,
So that **I can verify it's correct before submitting**.

**Acceptance Criteria:**

**Given** I take a photo of my meter
**When** OCR processing completes
**Then**:
  - The extracted value is displayed prominently
  - A bounding box shows where the value was found in the image
  - Confidence level is indicated
  - I can easily correct if wrong
  - Both values (OCR and corrected) are recorded
**And** submission only proceeds after my confirmation

**Components to Create:**
```
features/meters/
├── components/
│   ├── OCRPreview.tsx          - Preview with bounding box
│   ├── ValueConfirmation.tsx   - Confirm/correct UI
│   └── OCRConfidenceBar.tsx    - Visual confidence indicator
└── hooks/
    └── useOCR.ts               - OCR processing hook
```

**Story Points:** 4

---

#### Story 128.2: Correction Tracking for Model Training

As a **system**,
I want to **track OCR corrections**,
So that **the model can be improved**.

**Acceptance Criteria:**

**Given** a user corrects an OCR value
**When** the correction is submitted
**Then**:
  - Original OCR value is recorded
  - Corrected value is recorded
  - Image is flagged for training
  - Accuracy metrics are tracked
**And** training data is available for model improvement

**Story Points:** 2

---

## Phase 39: Quality & Polish

### Epic 129: Command Palette & Keyboard Shortcuts

**Goal:** Add power user features for fast navigation and action execution.

**Target Apps:** ppt-web
**Estimate:** 2 stories, ~2 days
**Dependencies:** None
**Priority:** P3 - LOW (Enhancement)

**PRD Reference:** UX Specification - Power User Features

---

#### Story 129.1: Command Palette

As a **power user**,
I want to **use a command palette**,
So that **I can quickly navigate and execute actions**.

**Acceptance Criteria:**

**Given** I'm using ppt-web
**When** I press `Cmd/Ctrl + K`
**Then**:
  - Command palette opens
  - I can search for pages, actions, and entities
  - Fuzzy search finds partial matches
  - Recent items are shown first
  - Selected item executes on Enter
**And** the palette feels fast and responsive

**Components to Create:**
```
components/
├── CommandPalette/
│   ├── CommandPalette.tsx      - Main component
│   ├── CommandList.tsx         - Results list
│   ├── CommandItem.tsx         - Individual result
│   └── useCommands.ts          - Command registry
```

**Story Points:** 5

---

#### Story 129.2: Global Keyboard Shortcuts

As a **power user**,
I want to **use keyboard shortcuts throughout the app**,
So that **I can work without a mouse**.

**Acceptance Criteria:**

**Given** keyboard shortcuts are enabled
**When** I use them
**Then**:
  - `g h` goes to Home
  - `g f` goes to Faults
  - `g v` goes to Votes
  - `g m` goes to Messages
  - `?` shows all shortcuts
**And** shortcuts don't conflict with browser defaults

**Story Points:** 2

---

### Epic 130: Performance Optimization

**Goal:** Optimize bundle size and loading performance.

**Target Apps:** ppt-web, reality-web
**Estimate:** 2 stories, ~2 days
**Dependencies:** None
**Priority:** P3 - LOW (Enhancement)

**PRD Reference:** NFR - Performance

---

#### Story 130.1: Bundle Size Optimization

As a **mobile user**,
I want to **the app to load quickly**,
So that **I can use it on slow connections**.

**Acceptance Criteria:**

**Given** the current bundle
**When** optimizations are applied
**Then**:
  - Initial bundle is < 200KB gzipped
  - Routes are code-split
  - Feature modules are lazy-loaded
  - Images are optimized
  - Tree-shaking removes unused code
**And** Lighthouse performance score > 90

**Technical Tasks:**
- [ ] Analyze bundle with webpack-bundle-analyzer
- [ ] Implement route-based code splitting
- [ ] Lazy load feature modules
- [ ] Optimize images with next/image
- [ ] Enable tree-shaking for all imports

**Story Points:** 5

---

#### Story 130.2: Core Web Vitals Optimization

As a **user**,
I want to **the app to feel fast**,
So that **interactions are immediate**.

**Acceptance Criteria:**

**Given** the application
**When** measuring Core Web Vitals
**Then**:
  - LCP < 2.5s
  - FID < 100ms
  - CLS < 0.1
  - TTFB < 600ms
**And** metrics are monitored in production

**Story Points:** 3

---

### Epic 131: E2E Test Suite

**Goal:** Implement comprehensive end-to-end tests for critical flows.

**Target Apps:** ppt-web
**Estimate:** 3 stories, ~3-4 days
**Dependencies:** None
**Priority:** P3 - LOW (Quality)

**PRD Reference:** Testability Requirements

---

#### Story 131.1: E2E Test Infrastructure

As a **developer**,
I want to **have E2E test infrastructure**,
So that **critical flows are automatically tested**.

**Acceptance Criteria:**

**Given** the CI pipeline
**When** E2E tests are configured
**Then**:
  - Playwright is installed and configured
  - Test database is seeded with fixtures
  - Tests run in CI on every PR
  - Test reports are generated
  - Flaky test detection is enabled
**And** tests are reliable and fast

**Technical Tasks:**
- [ ] Add Playwright to devDependencies
- [ ] Create test fixtures and seed data
- [ ] Configure CI job for E2E tests
- [ ] Set up test database isolation

**Story Points:** 3

---

#### Story 131.2: Authentication Flow Tests

As a **developer**,
I want to **E2E tests for authentication**,
So that **login/logout flows work correctly**.

**Acceptance Criteria:**

**Given** E2E infrastructure
**When** auth tests run
**Then**:
  - User registration works
  - Email verification works
  - Login with email/password works
  - Login with OAuth works
  - Password reset works
  - Session expiry is handled
**And** all auth scenarios are covered

**Story Points:** 3

---

#### Story 131.3: Critical Feature Flow Tests

As a **developer**,
I want to **E2E tests for critical features**,
So that **core functionality works correctly**.

**Acceptance Criteria:**

**Given** E2E infrastructure
**When** feature tests run
**Then**:
  - Fault reporting flow works
  - Voting flow works
  - Message sending works
  - Document upload works
  - Meter reading submission works
**And** 80% of critical flows are covered

**Story Points:** 5

---

## Summary

### Epics Created

| Epic | Title | Priority | Stories | Est. Days |
|------|-------|----------|---------|-----------|
| 121 | Utility Outages (UC-12) | P0 | 4 | 3-4 |
| 122 | Push Notification Deep Links | P0 | 3 | 2-3 |
| 123 | Offline Sync Progress | P1 | 3 | 2-3 |
| 124 | Action-First Dashboard | P1 | 4 | 4-5 |
| 125 | WCAG 2.1 AA Accessibility | P1 | 4 | 4-5 |
| 126 | AI Fault Reporting UI | P2 | 3 | 3-4 |
| 127 | AI Chatbot Interface | P2 | 3 | 3-4 |
| 128 | OCR Meter Preview | P2 | 2 | 2 |
| 129 | Command Palette | P3 | 2 | 2 |
| 130 | Performance Optimization | P3 | 2 | 2 |
| 131 | E2E Test Suite | P3 | 3 | 3-4 |

**Total:** 11 epics, 33 stories

### Implementation Order

1. **Week 1:** Epic 121 (Outages) + Epic 122 (Push Deep Links)
2. **Week 2:** Epic 123 (Offline Sync) + Epic 124 (Action Dashboard)
3. **Week 3:** Epic 125 (Accessibility)
4. **Week 4:** Epic 126 (AI Faults) + Epic 127 (AI Chat)
5. **Week 5:** Epic 128 (OCR) + Epic 129 (Command Palette)
6. **Week 6:** Epic 130 (Performance) + Epic 131 (E2E Tests)

---

**Document Status:** READY FOR IMPLEMENTATION
**Next Step:** Run implementation readiness check, then start `/bmad-loop`
