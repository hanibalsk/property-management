# Domain Model

This document defines the domain model for the Property Management System (PPT) and Reality Portal, following Domain-Driven Design (DDD) principles.

## Table of Contents

1. [Bounded Contexts](#bounded-contexts)
2. [Core Domain](#core-domain)
3. [Aggregates](#aggregates)
4. [Entities](#entities)
5. [Value Objects](#value-objects)
6. [Relationships](#relationships)
7. [Domain Events](#domain-events)

---

## Bounded Contexts

The system is divided into the following bounded contexts:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           PLATFORM CONTEXT                                   │
│  (Super Admin, Platform Billing, Organizations)                             │
└─────────────────────────────────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                        IDENTITY & ACCESS CONTEXT                             │
│  (Users, Roles, Permissions, Delegations, Sessions)                         │
└─────────────────────────────────────────────────────────────────────────────┘
         │
         ├──────────────────────────────────────────┬─────────────────────────┐
         ▼                                          ▼                         ▼
┌─────────────────────────────┐  ┌─────────────────────────────┐  ┌──────────────────────┐
│  PROPERTY MANAGEMENT        │  │     REALITY PORTAL          │  │    INTEGRATION       │
│  CONTEXT                    │  │     CONTEXT                 │  │    CONTEXT           │
│                             │  │                             │  │                      │
│  - Buildings                │  │  - Listings                 │  │  - IoT Devices       │
│  - Units                    │  │  - Agencies                 │  │  - Calendar Sync     │
│  - Faults                   │  │  - Realtors                 │  │  - Payment Gateway   │
│  - Announcements            │  │  - Portal Users             │  │  - External Portals  │
│  - Voting                   │  │  - Inquiries                │  │  - SMS/Email         │
│  - Documents                │  │  - Favorites                │  │  - Accounting        │
│  - Messages                 │  │  - Saved Searches           │  │                      │
│  - Finances                 │  │                             │  │                      │
│  - Short-term Rentals       │  │                             │  │                      │
└─────────────────────────────┘  └─────────────────────────────┘  └──────────────────────┘
```

### Context Map

| Context | Relationship | Context | Type |
|---------|--------------|---------|------|
| Platform | Upstream | All Contexts | Customer-Supplier |
| Identity & Access | Upstream | Property Management | Conformist |
| Identity & Access | Upstream | Reality Portal | Conformist |
| Property Management | Partnership | Reality Portal | Shared Kernel |
| Integration | Downstream | Property Management | Anti-Corruption Layer |
| Integration | Downstream | Reality Portal | Anti-Corruption Layer |

---

## Core Domain

### Platform Context

Manages multi-tenancy, subscriptions, and super-admin operations.

### Identity & Access Context

Manages users, authentication, authorization, roles, and delegations.

### Property Management Context

**Subdomains:**
- **Building Management** - Buildings, units, entrances
- **Communication** - Announcements, messages, news
- **Issue Tracking** - Faults, maintenance, outages
- **Governance** - Voting, meetings, delegations
- **Financial** - Payments, invoices, budgets
- **Documents** - Files, folders, forms
- **Community** - Neighbors, events, facilities
- **Short-term Rentals** - Airbnb/Booking integration, guests

### Reality Portal Context

**Subdomains:**
- **Listings** - Property listings, photos, status
- **Agencies** - Reality agencies, realtors
- **User Engagement** - Favorites, saved searches, inquiries
- **Import/Export** - CRM integration, data feeds

---

## Aggregates

### Platform Context Aggregates

#### Organization (Aggregate Root)

```
Organization
├── OrganizationId (Identity)
├── name: String
├── type: OrganizationType (HousingCooperative | PropertyManagement)
├── contactInfo: ContactInfo
├── branding: OrganizationBranding
├── settings: OrganizationSettings
├── subscription: Subscription
├── status: OrganizationStatus
├── createdAt: Timestamp
└── updatedAt: Timestamp

Responsibilities:
- Manages organization-level settings
- Controls branding and customization
- Maintains subscription state
- Aggregates buildings under management
```

#### Subscription (Aggregate Root)

```
Subscription
├── SubscriptionId (Identity)
├── organizationId: OrganizationId
├── plan: SubscriptionPlan
├── status: SubscriptionStatus
├── billingCycle: BillingCycle
├── currentPeriod: DateRange
├── paymentMethod: PaymentMethod
├── discountCode: DiscountCode?
├── usageMetrics: UsageMetrics
├── trialEndsAt: Timestamp?
└── createdAt: Timestamp

Responsibilities:
- Tracks subscription lifecycle
- Calculates usage-based billing
- Manages plan upgrades/downgrades
- Handles trial periods
```

---

### Identity & Access Context Aggregates

#### User (Aggregate Root)

```
User
├── UserId (Identity)
├── email: EmailAddress
├── passwordHash: PasswordHash
├── profile: UserProfile
│   ├── firstName: String
│   ├── lastName: String
│   ├── phoneNumber: PhoneNumber?
│   ├── avatar: ImageUrl?
│   └── language: LanguageCode
├── authSettings: AuthSettings
│   ├── mfaEnabled: Boolean
│   ├── mfaMethod: MfaMethod?
│   └── trustedDevices: List<DeviceId>
├── privacySettings: PrivacySettings
├── notificationPreferences: NotificationPreferences
├── status: UserStatus
├── sessions: List<Session>
├── createdAt: Timestamp
└── lastLoginAt: Timestamp?

Responsibilities:
- Manages user identity and credentials
- Controls authentication settings
- Maintains privacy preferences
- Tracks active sessions
```

#### Role (Aggregate Root)

```
Role
├── RoleId (Identity)
├── name: RoleName
├── scope: RoleScope (Platform | Organization | Building | Unit)
├── permissions: Set<Permission>
├── isSystem: Boolean
└── createdAt: Timestamp

Standard Roles:
- SuperAdministrator (Platform)
- OrganizationAdmin (Organization)
- Manager (Organization)
- TechnicalManager (Organization)
- Owner (Unit)
- Tenant (Unit)
- Resident (Unit)
- PropertyManager (Unit)
- Guest (Unit)
```

#### UserRole (Entity)

```
UserRole
├── userId: UserId
├── roleId: RoleId
├── scopeId: ScopeId (OrganizationId | BuildingId | UnitId)
├── assignedBy: UserId
├── assignedAt: Timestamp
└── expiresAt: Timestamp?
```

#### Delegation (Aggregate Root)

```
Delegation
├── DelegationId (Identity)
├── delegatorId: UserId (Owner)
├── delegateId: UserId
├── unitId: UnitId
├── rights: Set<DelegationRight>
│   ├── Voting
│   ├── Payment
│   ├── Documents
│   └── Communication
├── status: DelegationStatus
├── validFrom: Timestamp
├── expiresAt: Timestamp?
├── acceptedAt: Timestamp?
└── createdAt: Timestamp

Responsibilities:
- Tracks delegated rights
- Manages delegation lifecycle
- Validates delegation scope
```

---

### Property Management Context Aggregates

#### Building (Aggregate Root)

```
Building
├── BuildingId (Identity)
├── organizationId: OrganizationId
├── address: Address
├── name: String?
├── details: BuildingDetails
│   ├── yearBuilt: Year?
│   ├── totalUnits: Integer
│   ├── totalFloors: Integer
│   ├── totalEntrances: Integer
│   └── amenities: List<Amenity>
├── entrances: List<Entrance>
├── contacts: List<BuildingContact>
├── statistics: BuildingStatistics (computed)
├── status: BuildingStatus
├── createdAt: Timestamp
└── updatedAt: Timestamp

Responsibilities:
- Aggregates units within building
- Manages building-level information
- Maintains entrance structure
- Tracks building contacts
```

#### Unit (Aggregate Root)

```
Unit
├── UnitId (Identity)
├── buildingId: BuildingId
├── entranceId: EntranceId?
├── designation: String (e.g., "3B")
├── floor: Integer
├── details: UnitDetails
│   ├── size: SquareMeters?
│   ├── rooms: Integer?
│   ├── type: UnitType
│   └── ownershipShare: Percentage
├── ownership: List<UnitOwnership>
├── occupancy: List<UnitOccupancy>
├── meters: List<Meter>
├── status: UnitStatus
├── createdAt: Timestamp
└── updatedAt: Timestamp

Responsibilities:
- Tracks ownership history
- Manages occupancy records
- Aggregates utility meters
- Calculates ownership share
```

#### Fault (Aggregate Root)

```
Fault
├── FaultId (Identity)
├── buildingId: BuildingId
├── unitId: UnitId?
├── reporterId: UserId
├── title: String
├── description: String
├── category: FaultCategory
├── location: FaultLocation
├── priority: Priority (Low | Medium | High | Critical)
├── status: FaultStatus (New | InProgress | Resolved | Closed | Reopened)
├── assigneeId: UserId?
├── photos: List<Photo>
├── communications: List<FaultCommunication>
├── statusHistory: List<FaultStatusChange>
├── estimatedResolution: Timestamp?
├── resolvedAt: Timestamp?
├── createdAt: Timestamp
└── updatedAt: Timestamp

Responsibilities:
- Tracks fault lifecycle
- Manages assignments
- Records communication history
- Monitors resolution SLA
```

#### Announcement (Aggregate Root)

```
Announcement
├── AnnouncementId (Identity)
├── buildingId: BuildingId
├── authorId: UserId
├── title: String
├── content: RichText
├── type: AnnouncementType (General | Meeting | Urgent)
├── visibility: Visibility
├── attachments: List<Attachment>
├── comments: List<Comment>
├── status: AnnouncementStatus (Draft | Published | Archived)
├── isPinned: Boolean
├── scheduledAt: Timestamp?
├── publishedAt: Timestamp?
├── meetingDate: Timestamp? (for meetings)
├── createdAt: Timestamp
└── updatedAt: Timestamp

Responsibilities:
- Manages announcement lifecycle
- Controls visibility and targeting
- Handles scheduled publication
- Aggregates comments
```

#### Vote (Aggregate Root)

```
Vote
├── VoteId (Identity)
├── buildingId: BuildingId
├── creatorId: UserId
├── title: String
├── description: String
├── options: List<VoteOption>
│   ├── optionId: VoteOptionId
│   ├── text: String
│   └── voteCount: Integer (computed)
├── settings: VoteSettings
│   ├── allowChangeVote: Boolean
│   ├── requiresQuorum: Boolean
│   ├── quorumPercentage: Percentage?
│   └── weightByOwnership: Boolean
├── status: VoteStatus (Draft | Active | Completed | Cancelled)
├── ballots: List<Ballot>
├── comments: List<Comment>
├── startDate: Timestamp
├── endDate: Timestamp
├── results: VoteResults? (computed)
├── createdAt: Timestamp
└── updatedAt: Timestamp

Ballot (Entity within Vote)
├── BallotId (Identity)
├── voterId: UserId
├── unitId: UnitId
├── selectedOptionId: VoteOptionId
├── delegateId: UserId? (if proxy voting)
├── weight: Decimal (ownership share)
└── castAt: Timestamp

Responsibilities:
- Manages voting lifecycle
- Validates voter eligibility
- Calculates weighted results
- Handles proxy voting
```

#### Conversation (Aggregate Root)

```
Conversation
├── ConversationId (Identity)
├── organizationId: OrganizationId
├── type: ConversationType (Direct | Group)
├── participants: List<Participant>
│   ├── userId: UserId
│   ├── joinedAt: Timestamp
│   └── leftAt: Timestamp?
├── messages: List<Message>
│   ├── MessageId (Identity)
│   ├── senderId: UserId
│   ├── content: String
│   ├── attachments: List<Attachment>
│   ├── readBy: List<ReadReceipt>
│   ├── deletedAt: Timestamp?
│   └── sentAt: Timestamp
├── lastMessageAt: Timestamp?
├── status: ConversationStatus
├── createdAt: Timestamp
└── updatedAt: Timestamp

Responsibilities:
- Manages message history
- Tracks read receipts
- Handles group conversations
```

#### Document (Aggregate Root)

```
Document
├── DocumentId (Identity)
├── buildingId: BuildingId
├── folderId: FolderId?
├── uploaderId: UserId
├── name: String
├── description: String?
├── file: FileInfo
│   ├── url: Url
│   ├── mimeType: MimeType
│   ├── size: Bytes
│   └── checksum: Checksum
├── versions: List<DocumentVersion>
├── tags: List<Tag>
├── accessControl: AccessControl
├── isEncrypted: Boolean
├── ocrText: String? (extracted text)
├── createdAt: Timestamp
└── updatedAt: Timestamp

DocumentFolder (Aggregate Root)
├── FolderId (Identity)
├── buildingId: BuildingId
├── parentId: FolderId?
├── name: String
├── accessControl: AccessControl
└── createdAt: Timestamp

Responsibilities:
- Manages document storage
- Controls access permissions
- Tracks version history
- Supports OCR indexing
```

#### MeterReading (Aggregate Root)

```
MeterReading
├── MeterReadingId (Identity)
├── meterId: MeterId
├── unitId: UnitId
├── submitterId: UserId
├── value: Decimal
├── photo: Photo?
├── submissionPeriod: DateRange
├── status: ReadingStatus (Submitted | Verified | Rejected)
├── verifiedBy: UserId?
├── verifiedAt: Timestamp?
├── rejectionReason: String?
├── ocrValue: Decimal? (AI extracted)
├── submittedAt: Timestamp
└── updatedAt: Timestamp

Meter (Entity)
├── MeterId (Identity)
├── unitId: UnitId
├── type: MeterType (Electricity | Water | Gas | Heat)
├── serialNumber: String
├── installationDate: Date
└── lastReadingValue: Decimal?

Responsibilities:
- Tracks consumption readings
- Validates submitted values
- Supports OCR verification
- Maintains reading history
```

#### FinancialAccount (Aggregate Root)

```
FinancialAccount
├── AccountId (Identity)
├── unitId: UnitId
├── ownerId: UserId
├── balance: Money
├── transactions: List<Transaction>
│   ├── TransactionId (Identity)
│   ├── type: TransactionType
│   ├── amount: Money
│   ├── description: String
│   ├── invoiceId: InvoiceId?
│   ├── paymentId: PaymentId?
│   └── createdAt: Timestamp
├── invoices: List<Invoice>
├── payments: List<Payment>
├── createdAt: Timestamp
└── updatedAt: Timestamp

Invoice (Entity)
├── InvoiceId (Identity)
├── number: InvoiceNumber
├── items: List<InvoiceItem>
├── totalAmount: Money
├── dueDate: Date
├── status: InvoiceStatus
├── pdfUrl: Url?
└── issuedAt: Timestamp

Responsibilities:
- Tracks financial transactions
- Manages invoices and payments
- Calculates outstanding balance
- Generates statements
```

#### Reservation (Aggregate Root) - Short-term Rentals

```
Reservation
├── ReservationId (Identity)
├── unitId: UnitId
├── propertyManagerId: UserId
├── platform: ReservationPlatform (Airbnb | Booking | Direct)
├── externalId: String?
├── guest: GuestInfo
│   ├── firstName: String
│   ├── lastName: String
│   ├── email: EmailAddress
│   ├── phone: PhoneNumber?
│   ├── nationality: CountryCode
│   └── idDocument: IdDocument?
├── dates: DateRange
├── checkIn: CheckInInfo?
├── checkOut: CheckOutInfo?
├── accessCode: AccessCode?
├── policeRegistration: PoliceRegistration?
├── rating: GuestRating?
├── status: ReservationStatus
├── totalAmount: Money?
├── createdAt: Timestamp
└── updatedAt: Timestamp

Responsibilities:
- Manages reservation lifecycle
- Tracks guest check-in/out
- Generates police registration
- Syncs with external platforms
```

#### Budget (Aggregate Root)

```
Budget
├── BudgetId (Identity)
├── buildingId: BuildingId
├── year: Year
├── items: List<BudgetItem>
│   ├── category: BudgetCategory
│   ├── plannedAmount: Money
│   ├── actualAmount: Money (computed)
│   └── variance: Money (computed)
├── totalPlanned: Money
├── totalActual: Money (computed)
├── reserveFund: Money
├── status: BudgetStatus (Draft | Proposed | Approved | Active)
├── approvalVoteId: VoteId?
├── createdAt: Timestamp
└── updatedAt: Timestamp

Responsibilities:
- Tracks planned vs actual expenses
- Manages reserve fund
- Links to approval voting
```

---

### Reality Portal Context Aggregates

#### Listing (Aggregate Root)

```
Listing
├── ListingId (Identity)
├── realtorId: RealtorId
├── agencyId: AgencyId?
├── property: PropertyDetails
│   ├── type: PropertyType (Apartment | House | Commercial | Land)
│   ├── transactionType: TransactionType (Sale | Rent)
│   ├── address: Address
│   ├── coordinates: GeoCoordinates
│   ├── size: SquareMeters
│   ├── rooms: Integer?
│   ├── floor: Integer?
│   ├── totalFloors: Integer?
│   └── yearBuilt: Year?
├── features: List<Feature>
├── description: LocalizedText
├── pricing: Pricing
│   ├── price: Money
│   ├── pricePerSqm: Money (computed)
│   ├── previousPrice: Money?
│   └── priceHistory: List<PriceChange>
├── photos: List<ListingPhoto>
├── virtualTour: VirtualTourUrl?
├── status: ListingStatus (Active | Pending | Sold | Rented | Withdrawn)
├── isFeatured: Boolean
├── analytics: ListingAnalytics
│   ├── views: Integer
│   ├── favorites: Integer
│   └── inquiries: Integer
├── publishedAt: Timestamp?
├── expiresAt: Timestamp?
├── createdAt: Timestamp
└── updatedAt: Timestamp

Responsibilities:
- Manages listing lifecycle
- Tracks pricing history
- Aggregates analytics
- Handles multi-language content
```

#### Agency (Aggregate Root)

```
Agency
├── AgencyId (Identity)
├── name: String
├── profile: AgencyProfile
│   ├── description: LocalizedText
│   ├── logo: ImageUrl
│   ├── website: Url?
│   ├── contactEmail: EmailAddress
│   └── contactPhone: PhoneNumber
├── branding: AgencyBranding
│   ├── primaryColor: Color
│   └── logoWatermark: ImageUrl?
├── ownerId: UserId
├── managers: List<UserId>
├── realtors: List<AgencyRealtor>
│   ├── realtorId: RealtorId
│   ├── status: RealtorStatus
│   ├── joinedAt: Timestamp
│   └── suspendedAt: Timestamp?
├── settings: AgencySettings
├── statistics: AgencyStatistics (computed)
├── status: AgencyStatus
├── createdAt: Timestamp
└── updatedAt: Timestamp

Responsibilities:
- Manages agency membership
- Controls branding
- Tracks agency performance
```

#### Realtor (Aggregate Root)

```
Realtor
├── RealtorId (Identity)
├── userId: UserId
├── profile: RealtorProfile
│   ├── bio: LocalizedText
│   ├── photo: ImageUrl
│   ├── specializations: List<Specialization>
│   └── languages: List<LanguageCode>
├── license: RealtorLicense
│   ├── number: String
│   ├── issuingAuthority: String
│   ├── validUntil: Date
│   └── verified: Boolean
├── agencyId: AgencyId?
├── contactInfo: ContactInfo
├── statistics: RealtorStatistics (computed)
├── status: RealtorStatus
├── createdAt: Timestamp
└── updatedAt: Timestamp

Responsibilities:
- Manages realtor profile
- Tracks license verification
- Aggregates performance metrics
```

#### PortalUser (Aggregate Root)

```
PortalUser
├── PortalUserId (Identity)
├── userId: UserId (linked to Identity context)
├── linkedPropertyAccountId: UserId? (PPT account)
├── socialAccounts: List<SocialAccount>
│   ├── provider: SocialProvider (Google | Apple | Facebook)
│   ├── externalId: String
│   └── linkedAt: Timestamp
├── favorites: List<Favorite>
│   ├── listingId: ListingId
│   └── addedAt: Timestamp
├── savedSearches: List<SavedSearch>
│   ├── SavedSearchId (Identity)
│   ├── name: String
│   ├── criteria: SearchCriteria
│   ├── alertSettings: AlertSettings
│   └── createdAt: Timestamp
├── inquiries: List<InquiryId>
├── comparison: List<ListingId>
├── createdAt: Timestamp
└── updatedAt: Timestamp

Responsibilities:
- Manages portal user preferences
- Tracks favorites and searches
- Handles social logins
- Links to PPT account
```

#### Inquiry (Aggregate Root)

```
Inquiry
├── InquiryId (Identity)
├── listingId: ListingId
├── portalUserId: PortalUserId
├── realtorId: RealtorId
├── type: InquiryType (Message | ViewingRequest)
├── message: String
├── preferredViewingTimes: List<DateRange>?
├── status: InquiryStatus (Pending | Responded | Scheduled | Completed)
├── responses: List<InquiryResponse>
│   ├── responderId: UserId
│   ├── message: String
│   └── respondedAt: Timestamp
├── scheduledViewing: Viewing?
│   ├── dateTime: Timestamp
│   └── notes: String?
├── createdAt: Timestamp
└── updatedAt: Timestamp

Responsibilities:
- Tracks inquiry lifecycle
- Manages viewing scheduling
- Records response history
```

#### PropertyImport (Aggregate Root)

```
PropertyImport
├── ImportId (Identity)
├── agencyId: AgencyId
├── realtorId: RealtorId
├── source: ImportSource
│   ├── type: ImportSourceType (CRM | XML | CSV | IDX)
│   ├── connectionInfo: ConnectionInfo
│   └── fieldMapping: FieldMapping
├── schedule: ImportSchedule?
├── history: List<ImportRun>
│   ├── runId: ImportRunId
│   ├── startedAt: Timestamp
│   ├── completedAt: Timestamp?
│   ├── status: ImportStatus
│   ├── itemsProcessed: Integer
│   ├── itemsImported: Integer
│   ├── itemsFailed: Integer
│   └── errors: List<ImportError>
├── createdAt: Timestamp
└── updatedAt: Timestamp

Responsibilities:
- Manages import configuration
- Tracks import history
- Handles field mapping
- Resolves conflicts
```

---

## Entities

### Platform Context Entities

| Entity | Description |
|--------|-------------|
| Organization | Multi-tenant organization (housing cooperative or property management company) |
| Subscription | Platform subscription with plan, billing, and usage |
| SubscriptionPlan | Available subscription tiers with features |
| DiscountCode | Promotional discount codes |

### Identity & Access Context Entities

| Entity | Description |
|--------|-------------|
| User | System user with credentials and profile |
| Role | Named set of permissions with scope |
| UserRole | User-role assignment with scope |
| Session | Active user session with device info |
| Delegation | Delegated rights from owner to delegate |
| AuditLogEntry | Record of security-relevant actions |

### Property Management Context Entities

| Entity | Description |
|--------|-------------|
| Building | Residential building with address and details |
| Entrance | Building entrance/staircase |
| Unit | Apartment/unit within building |
| UnitOwnership | Ownership record for a unit |
| UnitOccupancy | Occupancy record (tenant, resident) |
| Meter | Utility meter (water, electricity, gas, heat) |
| Fault | Reported problem or issue |
| FaultCommunication | Message thread on fault |
| Announcement | Building announcement |
| Vote | Poll or voting session |
| Ballot | Individual vote cast |
| Conversation | Message thread |
| Message | Individual message |
| Document | Uploaded file with metadata |
| DocumentFolder | Folder for organizing documents |
| DocumentVersion | Version of a document |
| Form | Form template |
| FormSubmission | Submitted form data |
| MeterReading | Submitted utility reading |
| PersonMonth | Monthly occupancy record |
| Outage | Utility outage notice |
| NewsArticle | News item for building |
| Comment | Comment on content |
| Invoice | Financial invoice |
| Payment | Payment transaction |
| FinancialAccount | Unit's financial account |
| Reservation | Short-term rental reservation |
| Guest | Registered guest |
| PoliceRegistration | Guest police registration record |
| InsurancePolicy | Building/unit insurance |
| MaintenanceTask | Scheduled maintenance |
| Supplier | Vendor/supplier |
| SupplierContract | Contract with supplier |
| LegalCase | Legal dispute record |
| EmergencyAlert | Emergency notification |
| Budget | Annual budget |
| BudgetItem | Budget line item |
| FacilityBooking | Common area reservation |
| Event | Community event |
| Vehicle | Registered vehicle |
| Pet | Registered pet |
| Package | Tracked package |

### Reality Portal Context Entities

| Entity | Description |
|--------|-------------|
| Listing | Property listing |
| ListingPhoto | Photo attached to listing |
| Agency | Real estate agency |
| Realtor | Licensed real estate agent |
| RealtorLicense | License verification |
| PortalUser | Portal user account |
| Favorite | Favorited listing |
| SavedSearch | Saved search criteria |
| Inquiry | Contact inquiry |
| Viewing | Scheduled property viewing |
| PropertyImport | Import configuration |
| ImportRun | Import execution record |

---

## Value Objects

### Common Value Objects

```
EmailAddress
├── value: String
└── Validation: RFC 5322 compliant

PhoneNumber
├── countryCode: String
├── number: String
└── Validation: E.164 format

Address
├── street: String
├── streetNumber: String
├── city: String
├── postalCode: String
├── country: CountryCode
├── coordinates: GeoCoordinates?
└── Validation: Country-specific formats

GeoCoordinates
├── latitude: Decimal
├── longitude: Decimal
└── Validation: Valid lat/long ranges

Money
├── amount: Decimal
├── currency: CurrencyCode
└── Operations: add, subtract, multiply

DateRange
├── start: Date
├── end: Date
└── Validation: start <= end

Timestamp
├── value: DateTime (UTC)
└── Operations: comparison, formatting

ImageUrl
├── url: Url
├── width: Integer?
├── height: Integer?
└── Validation: Valid URL, image mime type

LocalizedText
├── translations: Map<LanguageCode, String>
└── default: LanguageCode

Percentage
├── value: Decimal (0-100)
└── Validation: 0 <= value <= 100

SquareMeters
├── value: Decimal
└── Validation: value > 0

Color
├── hex: String
└── Validation: Valid hex color (#RRGGBB)
```

### Identity Value Objects

```
PasswordHash
├── hash: String
├── algorithm: HashAlgorithm
└── Operations: verify

DeviceId
├── value: String
├── platform: Platform
└── lastSeenAt: Timestamp

Permission
├── resource: String
├── action: Action (Create | Read | Update | Delete | Manage)
└── scope: PermissionScope

MfaMethod
├── type: MfaType (TOTP | SMS | Biometric)
└── configuredAt: Timestamp
```

### Property Management Value Objects

```
Priority
├── level: PriorityLevel (Low | Medium | High | Critical)
└── escalationHours: Integer

FaultCategory
├── category: String
├── subcategory: String?
└── tags: List<String>

FaultLocation
├── area: String (Common | Unit | Exterior)
├── floor: Integer?
├── description: String?
└── coordinates: GeoCoordinates?

Visibility
├── scope: VisibilityScope (All | Owners | Tenants | Selected)
├── selectedUnits: List<UnitId>?
└── selectedEntrances: List<EntranceId>?

AccessControl
├── type: AccessType (Public | Restricted | Private)
├── allowedRoles: List<RoleName>?
├── allowedUsers: List<UserId>?
└── allowedUnits: List<UnitId>?

FileInfo
├── url: Url
├── mimeType: MimeType
├── size: Bytes
├── checksum: Checksum
└── uploadedAt: Timestamp

AccessCode
├── code: String
├── validFrom: Timestamp
├── validUntil: Timestamp
└── usageCount: Integer

VoteSettings
├── allowChangeVote: Boolean
├── requiresQuorum: Boolean
├── quorumPercentage: Percentage?
├── weightByOwnership: Boolean
└── anonymousResults: Boolean

VoteResults
├── totalEligible: Integer
├── totalVoted: Integer
├── quorumReached: Boolean
├── optionResults: Map<VoteOptionId, VoteOptionResult>
└── calculatedAt: Timestamp

VoteOptionResult
├── voteCount: Integer
├── weightedCount: Decimal
├── percentage: Percentage
└── isWinner: Boolean
```

### Reality Portal Value Objects

```
SearchCriteria
├── propertyTypes: List<PropertyType>?
├── transactionType: TransactionType?
├── location: LocationFilter?
├── priceRange: PriceRange?
├── sizeRange: SizeRange?
├── rooms: RoomRange?
├── features: List<Feature>?
└── keywords: String?

LocationFilter
├── type: LocationFilterType (City | District | Radius)
├── cities: List<String>?
├── districts: List<String>?
├── center: GeoCoordinates?
├── radiusKm: Decimal?

PriceRange
├── min: Money?
├── max: Money?

AlertSettings
├── enabled: Boolean
├── frequency: AlertFrequency (Instant | Daily | Weekly)
├── channels: List<AlertChannel>

Pricing
├── price: Money
├── pricePerSqm: Money (computed)
├── negotiable: Boolean
├── hidePrice: Boolean

PropertyDetails
├── type: PropertyType
├── transactionType: TransactionType
├── address: Address
├── coordinates: GeoCoordinates
├── size: SquareMeters
├── usableSize: SquareMeters?
├── rooms: Integer?
├── bathrooms: Integer?
├── floor: Integer?
├── totalFloors: Integer?
├── yearBuilt: Year?
├── condition: PropertyCondition
├── energyRating: EnergyRating?

Feature
├── category: FeatureCategory
├── name: String
├── value: String?
```

---

## Relationships

### Entity Relationship Diagram (Simplified)

```
┌──────────────────┐
│   Organization   │
└────────┬─────────┘
         │ 1:N
         ▼
┌──────────────────┐       ┌──────────────────┐
│    Building      │───────│     Manager      │
└────────┬─────────┘ N:M   └──────────────────┘
         │ 1:N
         ▼
┌──────────────────┐
│    Entrance      │
└────────┬─────────┘
         │ 1:N
         ▼
┌──────────────────┐       ┌──────────────────┐
│      Unit        │───────│      Owner       │
└────────┬─────────┘ N:M   └────────┬─────────┘
         │                          │
         ├─────────────────┐        │ 1:N
         │                 │        ▼
         ▼                 │   ┌──────────────────┐
┌──────────────────┐       │   │   Delegation     │
│     Tenant       │       │   └──────────────────┘
└──────────────────┘       │
                           ▼
                    ┌──────────────────┐
                    │      Meter       │
                    └────────┬─────────┘
                             │ 1:N
                             ▼
                    ┌──────────────────┐
                    │  MeterReading    │
                    └──────────────────┘
```

### Key Relationships

| Relationship | Cardinality | Description |
|--------------|-------------|-------------|
| Organization → Building | 1:N | Organization manages multiple buildings |
| Building → Entrance | 1:N | Building has multiple entrances |
| Building → Unit | 1:N | Building contains multiple units |
| Entrance → Unit | 1:N | Entrance groups units |
| Unit → Owner | N:M | Unit can have multiple co-owners |
| Unit → Tenant | 1:N | Unit can have multiple tenants |
| Owner → Delegation | 1:N | Owner can delegate rights |
| Building → Fault | 1:N | Faults reported per building |
| Building → Announcement | 1:N | Announcements per building |
| Building → Vote | 1:N | Votes per building |
| Building → Document | 1:N | Documents per building |
| Unit → Meter | 1:N | Multiple meters per unit |
| Meter → MeterReading | 1:N | Multiple readings per meter |
| Unit → FinancialAccount | 1:1 | One account per unit |
| Unit → Reservation | 1:N | Short-term rental reservations |
| Agency → Realtor | 1:N | Agency employs realtors |
| Realtor → Listing | 1:N | Realtor manages listings |
| PortalUser → Favorite | 1:N | User's favorite listings |
| PortalUser → SavedSearch | 1:N | User's saved searches |
| Listing → Inquiry | 1:N | Inquiries per listing |
| User → Role | N:M | Users have multiple roles |
| User → Session | 1:N | User has multiple sessions |

---

## Domain Events

### Platform Context Events

| Event | Description | Triggered By |
|-------|-------------|--------------|
| OrganizationCreated | New organization registered | Super Admin |
| OrganizationDeleted | Organization removed | Super Admin |
| SubscriptionUpgraded | Plan upgraded | Org Admin |
| SubscriptionDowngraded | Plan downgraded | Org Admin |
| SubscriptionCancelled | Subscription cancelled | Org Admin |
| TrialStarted | Free trial started | Org Admin |
| TrialExpiring | Trial expiration warning | System |

### Identity & Access Events

| Event | Description | Triggered By |
|-------|-------------|--------------|
| UserRegistered | New user registered | User |
| UserLoggedIn | User authenticated | User |
| UserLoggedOut | User logged out | User |
| PasswordChanged | Password updated | User |
| MfaEnabled | MFA activated | User |
| MfaDisabled | MFA deactivated | User |
| SessionTerminated | Session ended | User/System |
| DelegationCreated | Rights delegated | Owner |
| DelegationAccepted | Delegation accepted | Delegate |
| DelegationRevoked | Delegation revoked | Owner |
| DelegationExpired | Delegation expired | System |
| RoleAssigned | Role assigned to user | Admin |
| RoleRevoked | Role removed from user | Admin |
| AccountLocked | Account locked after failures | System |
| AccountUnlocked | Account unlocked | Admin/System |

### Property Management Events

| Event | Description | Triggered By |
|-------|-------------|--------------|
| BuildingCreated | Building added | Admin |
| BuildingArchived | Building archived | Admin |
| UnitCreated | Unit added | Admin |
| OwnerAssigned | Owner assigned to unit | Admin |
| TenantMoved | Tenant moved in/out | Admin |
| FaultReported | Fault created | User |
| FaultAssigned | Fault assigned to technician | Manager |
| FaultStatusChanged | Fault status updated | Manager |
| FaultResolved | Fault resolved | Manager |
| FaultReopened | Fault reopened | User |
| AnnouncementPublished | Announcement published | Manager |
| AnnouncementArchived | Announcement archived | Manager |
| VoteCreated | Vote created | Manager |
| VoteStarted | Voting period started | System |
| BallotCast | Vote submitted | Owner |
| VoteEnded | Voting period ended | System |
| VoteResultsPublished | Results published | Manager |
| MessageSent | Message sent | User |
| MessageRead | Message read | User |
| DocumentUploaded | Document uploaded | Manager |
| DocumentShared | Document shared | Manager |
| MeterReadingSubmitted | Reading submitted | Owner |
| MeterReadingVerified | Reading verified | Manager |
| MeterReadingRejected | Reading rejected | Manager |
| InvoiceGenerated | Invoice created | Manager |
| PaymentReceived | Payment received | System |
| PaymentOverdue | Payment past due | System |
| ReservationCreated | Booking received | System |
| GuestCheckedIn | Guest checked in | Guest |
| GuestCheckedOut | Guest checked out | Guest |
| EmergencyAlertSent | Emergency broadcast | Manager |
| OutageReported | Outage reported | User/System |
| MaintenanceScheduled | Maintenance planned | Manager |
| BudgetApproved | Budget approved by vote | System |

### Reality Portal Events

| Event | Description | Triggered By |
|-------|-------------|--------------|
| ListingCreated | New listing added | Realtor |
| ListingPublished | Listing made public | Realtor |
| ListingUpdated | Listing details changed | Realtor |
| ListingPriceChanged | Price updated | Realtor |
| ListingSold | Property sold | Realtor |
| ListingRented | Property rented | Realtor |
| ListingWithdrawn | Listing removed | Realtor |
| ListingViewed | Listing page viewed | User |
| AgencyCreated | Agency registered | Owner |
| RealtorJoinedAgency | Realtor joined agency | Realtor |
| RealtorLeftAgency | Realtor left agency | Admin/Realtor |
| RealtorSuspended | Realtor suspended | Owner |
| FavoriteAdded | Listing favorited | User |
| FavoriteRemoved | Favorite removed | User |
| SavedSearchCreated | Search saved | User |
| SearchAlertTriggered | Alert for new match | System |
| InquirySent | Inquiry submitted | User |
| InquiryResponded | Inquiry answered | Realtor |
| ViewingScheduled | Viewing booked | Realtor |
| ViewingCompleted | Viewing done | Realtor |
| ImportCompleted | Property import finished | System |
| ImportFailed | Import failed | System |

---

## Summary

| Context | Aggregates | Entities | Value Objects |
|---------|------------|----------|---------------|
| Platform | 2 | 4 | 5 |
| Identity & Access | 3 | 6 | 8 |
| Property Management | 12 | 48 | 18 |
| Reality Portal | 6 | 14 | 12 |
| **Total** | **23** | **72** | **43** |

### Aggregate Design Principles

1. **Transactional Consistency** - Each aggregate ensures consistency within its boundary
2. **Eventual Consistency** - Cross-aggregate operations use domain events
3. **Identity** - Each aggregate root has a globally unique identifier
4. **Encapsulation** - Internal entities accessed only through aggregate root
5. **Small Aggregates** - Prefer smaller aggregates with references over large nested structures
