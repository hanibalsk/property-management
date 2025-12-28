---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - _bmad-output/prd.md
  - _bmad-output/architecture.md
  - _bmad-output/epics-005.md
  - docs/use-cases.md
workflowType: 'epics-and-stories'
lastStep: 4
status: 'complete'
project_name: 'Property Management System (PPT) & Reality Portal'
user_name: 'Martin Janci'
date: '2025-12-27'
continues_from: 'epics-005.md'
phase_range: '19, 20, 21'
epic_range: '64-78'
---

# Property Management System (PPT) & Reality Portal - Epic Breakdown (Part 6)

## Overview

This document continues from `epics-005.md` and provides epic and story breakdown for **Phases 19, 20, and 21** - covering advanced platform capabilities, compliance, and operational features.

**Continuation from:** `epics-005.md` (Epics 54-63, Phases 16-18)

**Note:** Epics 64-65 (AI/LLM and Energy) were implemented in PR #80.

---

## Epic List

### Phase 19: Platform Evolution

#### Epic 66: Platform Migration & Data Import
**Goal:** Enable organizations to migrate from legacy systems with comprehensive data import/export capabilities.

**Target Apps:** api-server, ppt-web
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 3 (Buildings), Epic 11 (Financial)

---

##### Story 66.1: Define Import Templates

As a **platform administrator**,
I want to **create standardized import templates for common data types**,
So that **organizations can prepare their data for migration**.

**Acceptance Criteria:**

**Given** an admin accesses the import template builder
**When** they define field mappings for buildings, units, residents, financials
**Then** templates are saved and downloadable as CSV/Excel
**And** include validation rules and example data

**Technical Notes:**
- Template builder UI with field mapping
- Export template as CSV with headers
- Validation rules embedded in template metadata

---

##### Story 66.2: Bulk Data Import

As a **manager**,
I want to **import data from spreadsheets**,
So that **I can migrate from legacy systems quickly**.

**Acceptance Criteria:**

**Given** a manager uploads a filled import template
**When** the system processes the file
**Then** data is validated against schema
**And** errors are reported with row numbers
**And** valid data is imported in a transaction

**Technical Notes:**
- Async import processing with job queue
- Transaction rollback on critical errors
- Progress tracking via WebSocket
- Import history and audit log

---

##### Story 66.3: Data Export for Migration

As a **manager**,
I want to **export all organization data**,
So that **I can migrate to another system or create backups**.

**Acceptance Criteria:**

**Given** a manager requests full data export
**When** they select data categories
**Then** a ZIP file is generated with CSV files
**And** includes relationships and references
**And** is GDPR-compliant (anonymization options)

**Technical Notes:**
- Background export job
- Chunked file generation for large datasets
- Include metadata file with schema definitions

---

##### Story 66.4: Import Validation & Preview

As a **manager**,
I want to **preview import results before committing**,
So that **I can verify data accuracy**.

**Acceptance Criteria:**

**Given** a manager uploads import data
**When** validation completes
**Then** they see a preview with:
  - Record counts by type
  - Validation warnings/errors
  - Duplicate detection
  - Sample records
**And** can approve or cancel the import

**Technical Notes:**
- Dry-run mode for import
- Duplicate detection algorithms
- Preview limited to first 100 records

---

#### Epic 67: Advanced Compliance (AML/DSA)
**Goal:** Implement Anti-Money Laundering and Digital Services Act compliance features.

**Target Apps:** api-server, ppt-web
**Estimate:** 4 stories, ~2.5 weeks
**Dependencies:** Epic 9 (GDPR), Epic 19 (Tenant Screening)

---

##### Story 67.1: AML Risk Assessment

As a **landlord**,
I want to **assess AML risk for high-value transactions**,
So that **I comply with anti-money laundering regulations**.

**Acceptance Criteria:**

**Given** a transaction exceeds the AML threshold (€10,000+)
**When** the system evaluates the parties
**Then** a risk score is calculated based on:
  - Party identification completeness
  - Source of funds documentation
  - Country risk ratings
  - Transaction patterns
**And** high-risk cases are flagged for review

**Technical Notes:**
- Risk scoring algorithm
- Country risk database
- Suspicious activity detection
- Audit trail for compliance

---

##### Story 67.2: Enhanced Due Diligence

As a **manager**,
I want to **perform enhanced due diligence for flagged parties**,
So that **I can document compliance efforts**.

**Acceptance Criteria:**

**Given** a party is flagged as high-risk
**When** the manager initiates EDD
**Then** they can:
  - Request additional documentation
  - Record source of wealth
  - Document beneficial ownership
  - Add compliance notes
**And** the EDD record is timestamped and immutable

**Technical Notes:**
- EDD workflow with required fields
- Document upload with verification status
- Immutable audit records

---

##### Story 67.3: DSA Transparency Reports

As a **platform administrator**,
I want to **generate DSA transparency reports**,
So that **the platform complies with Digital Services Act**.

**Acceptance Criteria:**

**Given** DSA reporting period ends
**When** admin generates transparency report
**Then** report includes:
  - Content moderation actions
  - User reports and resolutions
  - Automated decision metrics
  - Appeal outcomes
**And** report is downloadable in required format

**Technical Notes:**
- Aggregated statistics from moderation logs
- PDF generation with DSA template
- Scheduled report generation

---

##### Story 67.4: Content Moderation Dashboard

As a **platform administrator**,
I want to **review and moderate user-generated content**,
So that **illegal or harmful content is removed promptly**.

**Acceptance Criteria:**

**Given** content is reported or flagged
**When** moderator reviews the queue
**Then** they can:
  - View reported content with context
  - Take action (remove, warn, ignore)
  - Document decision rationale
  - Track appeal status
**And** decisions are logged for DSA compliance

**Technical Notes:**
- Moderation queue with prioritization
- Action templates for common violations
- Appeal workflow

---

#### Epic 68: Service Provider Marketplace
**Goal:** Create a marketplace where building managers can find and hire verified service providers.

**Target Apps:** api-server, ppt-web, mobile
**Estimate:** 5 stories, ~3 weeks
**Dependencies:** Epic 21 (Vendors), Epic 4 (Faults)

---

##### Story 68.1: Service Provider Profiles

As a **service provider**,
I want to **create a marketplace profile**,
So that **property managers can find and hire me**.

**Acceptance Criteria:**

**Given** a service provider registers
**When** they complete their profile
**Then** profile includes:
  - Company information and certifications
  - Service categories and coverage area
  - Portfolio with photos
  - Pricing structure (hourly/project)
  - Availability calendar
**And** profile is searchable in marketplace

**Technical Notes:**
- Profile builder with sections
- Image upload for portfolio
- Service category taxonomy
- Coverage area (postal codes/regions)

---

##### Story 68.2: Search & Discovery

As a **manager**,
I want to **search for service providers by need**,
So that **I can find qualified vendors quickly**.

**Acceptance Criteria:**

**Given** a manager needs a service
**When** they search the marketplace
**Then** results are filtered by:
  - Service category
  - Location/coverage
  - Ratings and reviews
  - Availability
  - Price range
**And** sorted by relevance/rating

**Technical Notes:**
- Full-text search with filters
- Geo-based filtering
- Rating aggregation
- Sort options

---

##### Story 68.3: Request for Quote (RFQ)

As a **manager**,
I want to **request quotes from multiple providers**,
So that **I can compare offers**.

**Acceptance Criteria:**

**Given** a manager selects providers
**When** they submit an RFQ
**Then** selected providers receive notification
**And** can submit quotes with:
  - Price breakdown
  - Timeline
  - Terms
**And** manager can compare quotes side-by-side

**Technical Notes:**
- RFQ form with job details
- Multi-provider notifications
- Quote comparison view
- Quote expiration handling

---

##### Story 68.4: Provider Verification

As a **platform administrator**,
I want to **verify service provider credentials**,
So that **only qualified providers appear in marketplace**.

**Acceptance Criteria:**

**Given** a provider submits verification documents
**When** admin reviews them
**Then** they can:
  - Verify business registration
  - Confirm insurance coverage
  - Validate certifications
  - Award verification badges
**And** verified status appears on profile

**Technical Notes:**
- Document upload for verification
- Manual review queue
- Verification badge system
- Expiration tracking for time-limited docs

---

##### Story 68.5: Reviews & Ratings

As a **manager**,
I want to **rate and review service providers**,
So that **others can make informed decisions**.

**Acceptance Criteria:**

**Given** a job is completed
**When** the manager submits a review
**Then** they rate:
  - Quality (1-5 stars)
  - Timeliness (1-5 stars)
  - Communication (1-5 stars)
  - Value (1-5 stars)
**And** can add written review
**And** provider can respond to review

**Technical Notes:**
- Multi-dimension rating
- Review text with moderation
- Provider response capability
- Aggregate rating calculation

---

#### Epic 69: Public API & Developer Ecosystem
**Goal:** Provide a public API for third-party integrations with developer portal and documentation.

**Target Apps:** api-server, developer-portal (new)
**Estimate:** 5 stories, ~3 weeks
**Dependencies:** Epic 1 (Auth)

---

##### Story 69.1: API Key Management

As a **developer**,
I want to **generate and manage API keys**,
So that **my application can access the API**.

**Acceptance Criteria:**

**Given** a developer registers for API access
**When** they create an API key
**Then** they can:
  - Name the key for identification
  - Set scope/permissions
  - Set rate limits
  - View usage statistics
  - Revoke keys
**And** keys are securely hashed in storage

**Technical Notes:**
- API key generation with prefix
- Scope-based permissions
- Rate limiting per key
- Key rotation support

---

##### Story 69.2: Interactive API Documentation

As a **developer**,
I want to **explore API endpoints interactively**,
So that **I can understand and test the API**.

**Acceptance Criteria:**

**Given** a developer visits the API docs
**When** they browse endpoints
**Then** they see:
  - OpenAPI-generated documentation
  - Request/response examples
  - Interactive "Try it" feature
  - Authentication setup guide
**And** can test endpoints with their API key

**Technical Notes:**
- Swagger UI / Redoc integration
- OpenAPI 3.1 spec generation
- Sandbox environment for testing

---

##### Story 69.3: Webhook Subscriptions

As a **developer**,
I want to **subscribe to webhook events**,
So that **my app receives real-time updates**.

**Acceptance Criteria:**

**Given** a developer configures webhooks
**When** they subscribe to event types
**Then** they can:
  - Select events (fault.created, payment.received, etc.)
  - Specify endpoint URL
  - Set secret for signature verification
  - Test webhook delivery
**And** failed deliveries are retried with backoff

**Technical Notes:**
- Event type catalog
- HMAC signature for security
- Retry with exponential backoff
- Delivery logs and debugging

---

##### Story 69.4: Rate Limiting & Quotas

As a **platform administrator**,
I want to **enforce API rate limits**,
So that **the platform remains stable**.

**Acceptance Criteria:**

**Given** API requests are made
**When** rate limits are exceeded
**Then** 429 responses are returned
**And** headers indicate:
  - Rate limit ceiling
  - Remaining requests
  - Reset time
**And** different tiers have different limits

**Technical Notes:**
- Redis-based rate limiting
- Sliding window algorithm
- Tier-based quotas
- Burst allowance

---

##### Story 69.5: SDK Generation

As a **developer**,
I want to **use generated SDKs**,
So that **I can integrate faster**.

**Acceptance Criteria:**

**Given** the API spec is published
**When** SDKs are generated
**Then** developers can download:
  - TypeScript/JavaScript SDK
  - Python SDK
  - Go SDK
**And** SDKs include types and documentation

**Technical Notes:**
- OpenAPI Generator for SDK creation
- Automated SDK publishing to npm/pypi
- SDK versioning aligned with API

---

#### Epic 70: Competitive Feature Enhancements
**Goal:** Add features that differentiate from competitors - virtual tours, dynamic pricing, neighborhood data.

**Target Apps:** api-server, reality-web, reality-server
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 15 (Listings)

---

##### Story 70.1: Virtual Tour Integration

As a **listing agent**,
I want to **add 360° virtual tours to listings**,
So that **buyers can explore properties remotely**.

**Acceptance Criteria:**

**Given** an agent edits a listing
**When** they add virtual tour
**Then** they can:
  - Upload 360° photos
  - Embed Matterport/similar tours
  - Set tour order and hotspots
**And** tour is viewable on listing page

**Technical Notes:**
- 360° viewer component
- Matterport embed support
- Mobile VR compatibility

---

##### Story 70.2: Dynamic Pricing Suggestions

As a **landlord**,
I want to **receive pricing suggestions based on market data**,
So that **I can price my property competitively**.

**Acceptance Criteria:**

**Given** a landlord views pricing tools
**When** they analyze their listing
**Then** system suggests price based on:
  - Comparable listings in area
  - Historical price trends
  - Seasonal adjustments
  - Property attributes
**And** shows confidence level and reasoning

**Technical Notes:**
- Comparable property algorithm
- Price history analysis
- ML model for suggestions (future)
- Explanation generation

---

##### Story 70.3: Neighborhood Insights

As a **prospective buyer/tenant**,
I want to **see neighborhood information**,
So that **I can evaluate the location**.

**Acceptance Criteria:**

**Given** a user views a listing
**When** they open neighborhood tab
**Then** they see:
  - Walk/transit/bike scores
  - Nearby amenities (schools, shops, transit)
  - Crime statistics (if available)
  - Demographics overview
**And** data is sourced from public APIs

**Technical Notes:**
- Integration with Walk Score API
- OpenStreetMap for amenities
- Caching for API efficiency
- Attribution for data sources

---

##### Story 70.4: Comparable Sales/Rentals

As a **user**,
I want to **see comparable properties**,
So that **I can evaluate if pricing is fair**.

**Acceptance Criteria:**

**Given** a user views a listing
**When** they request comparables
**Then** system shows:
  - Similar properties recently sold/rented
  - Price per sqm comparison
  - Feature comparison table
**And** comparables are within reasonable distance

**Technical Notes:**
- Similarity scoring algorithm
- Historical transaction data
- Distance-based filtering
- Anonymized if needed for privacy

---

### Phase 20: Regional & Operational Excellence

#### Epic 71: Cross-Cutting Infrastructure
**Goal:** Implement infrastructure improvements for scalability, observability, and developer experience.

**Target Apps:** All servers
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** None (infrastructure)

---

##### Story 71.1: Distributed Tracing

As a **developer**,
I want to **trace requests across services**,
So that **I can debug distributed issues**.

**Acceptance Criteria:**

**Given** a request enters the system
**When** it's processed across components
**Then** a trace ID follows the request
**And** spans are recorded for:
  - HTTP handlers
  - Database queries
  - External API calls
**And** traces are viewable in Jaeger/similar

**Technical Notes:**
- OpenTelemetry integration
- Trace context propagation
- Span attributes for debugging
- Sampling for high-volume

---

##### Story 71.2: Feature Flags

As a **developer**,
I want to **control feature rollout with flags**,
So that **I can release safely**.

**Acceptance Criteria:**

**Given** a new feature is ready
**When** it's deployed with a flag
**Then** the flag controls:
  - On/off toggle
  - Percentage rollout
  - User/org targeting
**And** changes take effect without deploy

**Technical Notes:**
- Feature flag service/table
- SDK for flag evaluation
- Admin UI for flag management
- Audit log for changes

---

##### Story 71.3: Background Job Dashboard

As a **administrator**,
I want to **monitor background jobs**,
So that **I can ensure they complete successfully**.

**Acceptance Criteria:**

**Given** background jobs are running
**When** admin views job dashboard
**Then** they see:
  - Active/pending/failed jobs
  - Job execution history
  - Retry controls
  - Performance metrics
**And** can manually retry failed jobs

**Technical Notes:**
- Job queue visibility
- Failure alerting
- Manual retry capability
- Job prioritization

---

##### Story 71.4: Health Monitoring

As a **operator**,
I want to **monitor system health**,
So that **I can respond to issues quickly**.

**Acceptance Criteria:**

**Given** the system is running
**When** health checks run
**Then** they verify:
  - Database connectivity
  - Redis connectivity
  - External service status
  - Disk/memory usage
**And** unhealthy status triggers alerts

**Technical Notes:**
- Health check endpoints
- Dependency health checks
- Prometheus metrics
- Alert rules

---

#### Epic 72: Regional Legal Compliance (SK/CZ)
**Goal:** Implement Slovakia and Czech Republic specific legal requirements for property management.

**Target Apps:** api-server, ppt-web
**Estimate:** 5 stories, ~2.5 weeks
**Dependencies:** Epic 5 (Voting), Epic 11 (Financial)

---

##### Story 72.1: Slovak Voting Requirements

As a **manager in Slovakia**,
I want to **conduct votes per Slovak housing law**,
So that **decisions are legally binding**.

**Acceptance Criteria:**

**Given** a vote is created for Slovak organization
**When** it's configured
**Then** it enforces:
  - Quorum requirements per law
  - Voting by ownership share
  - Written ballot requirements
  - Notarization thresholds
**And** generates legally compliant minutes

**Technical Notes:**
- Slovak law voting rules
- Ownership share calculation
- Minutes template per law
- Notarization flag

---

##### Story 72.2: Czech SVJ Compliance

As a **manager in Czech Republic**,
I want to **manage SVJ (housing cooperative) per Czech law**,
So that **the SVJ operates legally**.

**Acceptance Criteria:**

**Given** a Czech SVJ organization
**When** configured
**Then** system enforces:
  - SVJ-specific voting rules
  - Contribution fund requirements
  - Member registry per law
  - Annual meeting requirements
**And** generates Czech-compliant documents

**Technical Notes:**
- Czech civil code requirements
- SVJ document templates
- Member registry fields
- Annual meeting workflow

---

##### Story 72.3: Slovak Accounting Export

As a **manager in Slovakia**,
I want to **export accounting data for Slovak systems**,
So that **I can integrate with local accounting software**.

**Acceptance Criteria:**

**Given** financial data exists
**When** Slovak export is requested
**Then** data exports in:
  - POHODA XML format
  - Money S3 format
  - Slovak tax report format
**And** includes required Slovak fields

**Technical Notes:**
- POHODA XML schema
- Slovak invoice requirements
- VAT handling per Slovak law

---

##### Story 72.4: GDPR Consent (SK/CZ Specific)

As a **resident in SK/CZ**,
I want to **manage data consent per local GDPR interpretation**,
So that **my privacy is protected**.

**Acceptance Criteria:**

**Given** a resident in SK/CZ
**When** they view consent settings
**Then** they see:
  - Consent categories per local DPA guidance
  - Processing purposes in local language
  - Data retention periods
  - DPO contact information
**And** can withdraw consent granularly

**Technical Notes:**
- SK/CZ specific consent categories
- Localized consent text
- DPA guidance alignment

---

##### Story 72.5: Regional Document Templates

As a **manager**,
I want to **use legally compliant document templates**,
So that **contracts and notices are valid**.

**Acceptance Criteria:**

**Given** a manager creates a document
**When** they select template
**Then** templates are available for:
  - Lease agreements (SK/CZ versions)
  - Meeting minutes (per local law)
  - Notice templates
  - Fee schedules
**And** templates include required legal clauses

**Technical Notes:**
- Template library per jurisdiction
- Required clause validation
- Language variants

---

#### Epic 73: Infrastructure & Operations
**Goal:** Enhance operational capabilities for production deployment.

**Target Apps:** All servers, deployment
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 71 (Infrastructure)

---

##### Story 73.1: Blue-Green Deployment

As a **DevOps engineer**,
I want to **deploy with zero downtime**,
So that **users aren't affected by releases**.

**Acceptance Criteria:**

**Given** a new version is ready
**When** deployment starts
**Then**:
  - New version deploys to green environment
  - Health checks pass
  - Traffic switches to green
  - Blue becomes standby
**And** rollback is instant if needed

**Technical Notes:**
- Kubernetes deployment strategy
- Health check gates
- Traffic switching
- Rollback automation

---

##### Story 73.2: Database Migration Safety

As a **developer**,
I want to **run migrations safely in production**,
So that **schema changes don't cause outages**.

**Acceptance Criteria:**

**Given** a migration is needed
**When** it runs in production
**Then**:
  - Migration is backward compatible
  - No table locks for extended periods
  - Progress is logged
  - Rollback is possible
**And** application handles old/new schema

**Technical Notes:**
- Expand-contract migrations
- Online DDL where possible
- Migration testing in staging
- Schema version tracking

---

##### Story 73.3: Disaster Recovery

As a **administrator**,
I want to **recover from disasters**,
So that **data isn't lost**.

**Acceptance Criteria:**

**Given** a disaster occurs
**When** recovery is initiated
**Then**:
  - Database restores from backup
  - Point-in-time recovery available
  - File storage recovers
  - RTO < 4 hours, RPO < 1 hour
**And** recovery is documented and tested

**Technical Notes:**
- Automated backup verification
- Cross-region backup storage
- Recovery runbooks
- Regular DR drills

---

##### Story 73.4: Cost Monitoring

As a **administrator**,
I want to **monitor infrastructure costs**,
So that **spending is optimized**.

**Acceptance Criteria:**

**Given** infrastructure is running
**When** costs are analyzed
**Then** dashboard shows:
  - Cost by service
  - Cost trends
  - Resource utilization
  - Optimization recommendations
**And** alerts on budget thresholds

**Technical Notes:**
- Cloud cost APIs
- Resource tagging
- Utilization metrics
- Budget alerts

---

#### Epic 74: Owner Investment Analytics
**Goal:** Provide property owners with investment analysis and portfolio management tools.

**Target Apps:** api-server, ppt-web
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 11 (Financial), Epic 3 (Buildings)

---

##### Story 74.1: Property Valuation Tracking

As a **property owner**,
I want to **track my property's estimated value over time**,
So that **I can monitor my investment**.

**Acceptance Criteria:**

**Given** an owner has properties
**When** they view valuation dashboard
**Then** they see:
  - Current estimated value
  - Value history chart
  - Value change (amount and %)
  - Comparison to market index
**And** can input manual valuations

**Technical Notes:**
- Valuation history table
- Manual valuation entry
- Chart visualization
- Market index comparison

---

##### Story 74.2: ROI Calculator

As a **property owner**,
I want to **calculate return on investment**,
So that **I can evaluate property performance**.

**Acceptance Criteria:**

**Given** an owner views ROI tools
**When** they analyze a property
**Then** calculation includes:
  - Rental income (actual and potential)
  - Operating expenses
  - Mortgage costs (if tracked)
  - Capital appreciation
  - Net yield percentage
**And** can compare multiple properties

**Technical Notes:**
- Income/expense aggregation
- Yield calculations
- Comparison view
- Export to PDF

---

##### Story 74.3: Cash Flow Analysis

As a **property owner**,
I want to **see cash flow projections**,
So that **I can plan financially**.

**Acceptance Criteria:**

**Given** income and expenses are tracked
**When** owner views cash flow
**Then** they see:
  - Monthly cash flow (actual)
  - 12-month projection
  - Expense breakdown
  - Vacancy impact modeling
**And** can adjust assumptions

**Technical Notes:**
- Cash flow calculation engine
- Projection algorithms
- What-if scenarios
- Visualization charts

---

##### Story 74.4: Portfolio Dashboard

As a **property owner with multiple properties**,
I want to **see portfolio overview**,
So that **I can manage investments holistically**.

**Acceptance Criteria:**

**Given** an owner has multiple properties
**When** they view portfolio dashboard
**Then** they see:
  - Total portfolio value
  - Aggregate performance metrics
  - Property-by-property breakdown
  - Diversification analysis
  - Alert for underperforming properties
**And** can drill down to each property

**Technical Notes:**
- Aggregation queries
- Performance scoring
- Alert thresholds
- Drill-down navigation

---

### Phase 21: Tenant & Vendor Experience

#### Epic 75: Tenant Safety & Wellbeing
**Goal:** Features focused on tenant safety, emergency preparedness, and community wellbeing.

**Target Apps:** api-server, ppt-web, mobile
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 62 (Emergency Contacts), Epic 2B (Notifications)

---

##### Story 75.1: Safety Alerts

As a **building manager**,
I want to **send safety alerts to all residents**,
So that **they're informed of emergencies**.

**Acceptance Criteria:**

**Given** an emergency occurs
**When** manager sends safety alert
**Then**:
  - All residents receive push notification
  - SMS sent to registered phones
  - Email sent immediately
  - Alert appears prominently in app
**And** read receipts are tracked

**Technical Notes:**
- Multi-channel delivery
- Priority bypass for DND
- Delivery tracking
- Alert history

---

##### Story 75.2: Evacuation Plans

As a **resident**,
I want to **access evacuation plans**,
So that **I know what to do in emergency**.

**Acceptance Criteria:**

**Given** a resident opens safety section
**When** they view evacuation info
**Then** they see:
  - Floor plans with exit routes
  - Assembly point locations
  - Emergency contact numbers
  - Building-specific instructions
**And** plans are available offline

**Technical Notes:**
- PDF/image upload for plans
- Offline caching
- Unit-specific routing (future)
- Accessibility considerations

---

##### Story 75.3: Wellbeing Check-ins

As a **building manager**,
I want to **check on vulnerable residents**,
So that **I can ensure their safety**.

**Acceptance Criteria:**

**Given** residents are flagged for check-ins
**When** check-in period arrives
**Then**:
  - Resident receives check-in prompt
  - They can confirm they're okay
  - Non-response triggers escalation
  - Manager sees check-in status
**And** respects privacy preferences

**Technical Notes:**
- Opt-in program
- Scheduled check-ins
- Escalation workflow
- Privacy controls

---

##### Story 75.4: Incident Reporting

As a **resident**,
I want to **report safety incidents anonymously**,
So that **issues are addressed without fear of retaliation**.

**Acceptance Criteria:**

**Given** a resident witnesses an incident
**When** they submit anonymous report
**Then**:
  - No identifying information required
  - Can attach photos/evidence
  - Manager receives report
  - Status updates available
**And** identity is protected

**Technical Notes:**
- Anonymous submission flow
- Evidence upload
- Status tracking without login
- No IP logging for reports

---

#### Epic 76: Move-in/Move-out Workflow
**Goal:** Streamline the tenant transition process with checklists, inspections, and handover tracking.

**Target Apps:** api-server, ppt-web, mobile
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 3 (Units), Epic 7A (Documents)

---

##### Story 76.1: Move-in Checklist

As a **manager**,
I want to **create move-in checklists**,
So that **new tenants complete all required steps**.

**Acceptance Criteria:**

**Given** a new tenant is moving in
**When** move-in workflow starts
**Then** checklist includes:
  - Key handover
  - Meter readings
  - Document signing
  - Welcome information
  - Inspection scheduling
**And** progress is tracked

**Technical Notes:**
- Configurable checklist templates
- Progress tracking
- Notification reminders
- Completion confirmation

---

##### Story 76.2: Property Inspection

As a **manager**,
I want to **conduct move-in/out inspections**,
So that **property condition is documented**.

**Acceptance Criteria:**

**Given** inspection is scheduled
**When** it's conducted
**Then** inspector records:
  - Room-by-room condition
  - Photos of each area
  - Existing damage notes
  - Meter readings
**And** both parties sign digitally

**Technical Notes:**
- Mobile-first inspection form
- Photo capture integration
- Digital signature
- PDF report generation

---

##### Story 76.3: Key Management

As a **manager**,
I want to **track key handover**,
So that **access control is maintained**.

**Acceptance Criteria:**

**Given** tenant transition occurs
**When** keys are exchanged
**Then** system records:
  - Keys given/returned
  - Key types (main, mailbox, garage)
  - Handover timestamp
  - Recipient signature
**And** alerts for unreturned keys

**Technical Notes:**
- Key inventory per unit
- Handover logging
- Overdue key alerts
- Access code rotation

---

##### Story 76.4: Deposit Handling

As a **tenant**,
I want to **track my deposit status**,
So that **I know what deductions are made**.

**Acceptance Criteria:**

**Given** tenant is moving out
**When** deposit is processed
**Then** they see:
  - Original deposit amount
  - Itemized deductions
  - Supporting photos/invoices
  - Final refund amount
**And** can dispute deductions

**Technical Notes:**
- Deposit ledger
- Deduction categories
- Evidence linking
- Dispute workflow

---

#### Epic 77: Dispute Resolution
**Goal:** Provide structured process for resolving disputes between parties (tenant-landlord, neighbor-neighbor).

**Target Apps:** api-server, ppt-web
**Estimate:** 4 stories, ~2 weeks
**Dependencies:** Epic 2A (Messaging)

---

##### Story 77.1: Dispute Filing

As a **resident**,
I want to **file a formal dispute**,
So that **issues are addressed through proper channels**.

**Acceptance Criteria:**

**Given** a resident has a dispute
**When** they file it
**Then** they provide:
  - Dispute category (noise, damage, payment, etc.)
  - Description of issue
  - Evidence/attachments
  - Desired resolution
**And** case is assigned a reference number

**Technical Notes:**
- Dispute form with categories
- Evidence upload
- Auto-assignment rules
- Notification to parties

---

##### Story 77.2: Mediation Process

As a **manager**,
I want to **mediate disputes between parties**,
So that **conflicts are resolved fairly**.

**Acceptance Criteria:**

**Given** a dispute is filed
**When** mediation begins
**Then** manager can:
  - View all submissions from parties
  - Schedule mediation session
  - Record session notes
  - Propose resolutions
**And** all communications are logged

**Technical Notes:**
- Case management view
- Timeline of events
- Session scheduling
- Resolution proposals

---

##### Story 77.3: Resolution Tracking

As a **dispute party**,
I want to **track dispute resolution progress**,
So that **I know the status**.

**Acceptance Criteria:**

**Given** a dispute is ongoing
**When** parties check status
**Then** they see:
  - Current stage (filed, under review, mediation, resolved)
  - Timeline of actions
  - Next steps required
  - Resolution outcome (when complete)
**And** receive updates on changes

**Technical Notes:**
- Status workflow
- Activity timeline
- Party notifications
- Outcome recording

---

##### Story 77.4: Resolution Enforcement

As a **manager**,
I want to **enforce dispute resolutions**,
So that **agreed actions are completed**.

**Acceptance Criteria:**

**Given** a resolution is agreed
**When** enforcement begins
**Then** system:
  - Creates action items for parties
  - Tracks completion
  - Sends reminders
  - Escalates non-compliance
**And** resolution is marked complete when done

**Technical Notes:**
- Action item creation
- Deadline tracking
- Escalation rules
- Completion confirmation

---

#### Epic 78: Vendor Operations Portal
**Goal:** Dedicated portal for vendors to manage jobs, access properties, submit completions, and track payments.

**Target Apps:** api-server, vendor-portal (new frontend or ppt-web section)
**Estimate:** 5 stories, ~2.5 weeks
**Dependencies:** Epic 21 (Vendors), Epic 20 (Work Orders)

---

##### Story 78.1: Vendor Job Dashboard

As a **vendor**,
I want to **see my assigned jobs**,
So that **I can plan my work**.

**Acceptance Criteria:**

**Given** a vendor logs in
**When** they view dashboard
**Then** they see:
  - Today's jobs with details
  - Upcoming jobs calendar
  - Pending action items
  - Completed jobs history
**And** can accept/decline new assignments

**Technical Notes:**
- Vendor-specific view
- Calendar integration
- Job status management
- Mobile-responsive

---

##### Story 78.2: Property Access Information

As a **vendor**,
I want to **access property entry information**,
So that **I can enter buildings for service**.

**Acceptance Criteria:**

**Given** a vendor has assigned job
**When** they view job details
**Then** they see:
  - Building address and directions
  - Access codes (time-limited)
  - Contact person and phone
  - Special instructions
**And** codes expire after job window

**Technical Notes:**
- Temporary access code generation
- Time-window enforcement
- Contact information
- Expiration handling

---

##### Story 78.3: Work Completion Submission

As a **vendor**,
I want to **submit work completion details**,
So that **I can get paid**.

**Acceptance Criteria:**

**Given** work is completed
**When** vendor submits completion
**Then** they provide:
  - Before/after photos
  - Time spent
  - Materials used
  - Notes
**And** submission goes for approval

**Technical Notes:**
- Photo upload
- Time tracking
- Materials logging
- Approval workflow

---

##### Story 78.4: Invoice Generation

As a **vendor**,
I want to **generate invoices for completed work**,
So that **billing is streamlined**.

**Acceptance Criteria:**

**Given** work is approved
**When** vendor generates invoice
**Then** invoice includes:
  - Job reference
  - Labor charges
  - Materials costs
  - Total amount
**And** is submitted to management

**Technical Notes:**
- Invoice from job data
- PDF generation
- Submission tracking
- Payment status

---

##### Story 78.5: Payment Tracking

As a **vendor**,
I want to **track invoice payments**,
So that **I know when I'll be paid**.

**Acceptance Criteria:**

**Given** invoices are submitted
**When** vendor checks payments
**Then** they see:
  - Invoice statuses (pending, approved, paid)
  - Payment dates
  - Payment history
  - Outstanding balance
**And** receive notification on payment

**Technical Notes:**
- Payment status sync
- Payment history
- Notification on payment
- Export statements

---

## Summary

### Phase 19-21 Totals

| Phase | Epics | Stories | Estimate |
|-------|-------|---------|----------|
| **Phase 19** | 64-70 | 27 | ~15 weeks |
| **Phase 20** | 71-74 | 17 | ~9 weeks |
| **Phase 21** | 75-78 | 17 | ~9 weeks |
| **Total** | 15 | 61 | ~33 weeks |

### Epic Implementation Priority

**High Priority (Core Differentiators):**
- Epic 72: Regional Compliance (SK/CZ) - Legal requirement
- Epic 74: Owner Analytics - Key differentiator
- Epic 78: Vendor Portal - Operational efficiency

**Medium Priority (Platform Enhancement):**
- Epic 66: Migration - Enables customer acquisition
- Epic 68: Marketplace - Revenue opportunity
- Epic 69: Public API - Partner ecosystem

**Lower Priority (Nice to Have):**
- Epic 67: AML/DSA - Future compliance
- Epic 70: Competitive Features - Market positioning
- Epic 71/73: Infrastructure - Operational improvement

### File Organization for Parallel Development

To avoid merge conflicts, each epic creates isolated files:

| Epic | Backend Model | Backend Route | Frontend Feature |
|------|--------------|---------------|------------------|
| 66 | `migration.rs` | `migration.rs` | `features/migration/` |
| 67 | `compliance.rs` | `compliance.rs` | `features/compliance/` |
| 68 | `marketplace.rs` | `marketplace.rs` | `features/marketplace/` |
| 69 | `public_api.rs` | `public_api.rs` | `features/developer/` |
| 70 | `competitive.rs` | `competitive.rs` | `features/competitive/` |
| 71 | `infrastructure.rs` | `infrastructure.rs` | N/A (backend only) |
| 72 | `regional_compliance.rs` | `regional_compliance.rs` | `features/regional/` |
| 73 | `operations.rs` | `operations.rs` | N/A (backend only) |
| 74 | `owner_analytics.rs` | `owner_analytics.rs` | `features/analytics/` |
| 75 | `tenant_safety.rs` | `tenant_safety.rs` | `features/safety/` |
| 76 | `move_workflow.rs` | `move_workflow.rs` | `features/move-workflow/` |
| 77 | `disputes.rs` | `disputes.rs` | `features/disputes/` |
| 78 | `vendor_portal.rs` | `vendor_portal.rs` | `features/vendor-portal/` |

**Conflict-prone files (update at merge time):**
- `backend/crates/db/src/models/mod.rs`
- `backend/crates/db/src/repositories/mod.rs`
- `backend/servers/api-server/src/routes/mod.rs`
- `backend/servers/api-server/src/main.rs`
- `backend/servers/api-server/src/state.rs`
