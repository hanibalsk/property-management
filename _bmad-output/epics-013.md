---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - _bmad-output/prd.md
  - _bmad-output/architecture.md
  - _bmad-output/epics-012.md
workflowType: 'epics-and-stories'
lastStep: 4
status: 'ready'
project_name: 'Property Management System (PPT) & Reality Portal'
user_name: 'Martin Janci'
date: '2026-01-02'
continues_from: 'epics-012.md'
phase_range: '34-35'
epic_range: '107-110'
---

# Property Management System (PPT) & Reality Portal - Epic Breakdown (Part 13)

## Overview

This document continues from `epics-012.md` and addresses **Phase 34: Feature Management & Monetization** and **Phase 35: Package Management & User Type Differentiation** - implementing comprehensive feature descriptors, monetization packages, and user-type based feature access.

**Continuation from:** `epics-012.md` (Epics 102-106, Phases 31-33)

**Source:** User requirement analysis (2026-01-02)

**Key Requirements:**
- Feature flags with rich descriptors (name, description, category, icon)
- Feature packages for bundling features together
- User-type based feature differentiation
- UI forms displaying features as checkboxes with descriptors
- Integration with existing subscription/billing system

---

## Epic List

### Phase 34: Feature Management & Monetization

#### Epic 107: Feature Descriptors & Catalog
**Goal:** Enhance feature flags with rich descriptors, categories, and UI metadata for display in forms and dashboards.

**Target Apps:** api-server, ppt-web
**Estimate:** 5 stories, ~1 week
**Dependencies:** Epic 89 (existing Feature Flags)
**Priority:** P1 - HIGH

**PRD Reference:** Monetization platform, Feature configuration

---

##### Story 107.1: Feature Descriptor Model

As a **platform administrator**,
I want to **define rich descriptors for each feature**,
So that **users understand what each feature provides**.

**Acceptance Criteria:**

**Given** a feature flag exists
**When** a descriptor is associated with it
**Then**:
  - Descriptor includes: name, description, short_description, icon, category
  - Descriptor supports localization (i18n keys or JSON)
  - Descriptor has marketing copy (benefits, use_cases)
  - Descriptor has technical metadata (api_scope, dependencies)
  - Features can be grouped by category
**And** descriptors are queryable via API

**Technical Notes:**
- Extend `feature_flags` table or create `feature_descriptors` table
- Add JSON fields for i18n support
- Include `display_order` for UI ordering
- Add `preview_image_url` for feature screenshots

**Files to Modify:**
- `backend/crates/db/src/models/platform_admin.rs` (add FeatureDescriptor model)
- `backend/crates/db/src/repositories/feature_flag.rs` (add descriptor methods)
- `backend/crates/db/src/migrations/` (add descriptor migration)

**Database Schema:**
```sql
CREATE TABLE feature_descriptors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    feature_flag_id UUID NOT NULL REFERENCES feature_flags(id) ON DELETE CASCADE,

    -- Display info
    display_name VARCHAR(100) NOT NULL,
    short_description VARCHAR(255),
    full_description TEXT,
    icon VARCHAR(50),  -- Icon identifier (e.g., 'home', 'chart', 'lock')
    preview_image_url VARCHAR(500),

    -- Categorization
    category VARCHAR(50) NOT NULL,  -- e.g., 'core', 'advanced', 'premium', 'ai'
    subcategory VARCHAR(50),
    tags JSONB DEFAULT '[]',

    -- Localization
    translations JSONB DEFAULT '{}',  -- {"sk": {"name": "...", "description": "..."}}

    -- Marketing
    benefits JSONB DEFAULT '[]',  -- ["Benefit 1", "Benefit 2"]
    use_cases JSONB DEFAULT '[]',

    -- Technical
    api_scopes JSONB DEFAULT '[]',  -- Required OAuth scopes
    depends_on JSONB DEFAULT '[]',  -- Feature flag keys this depends on
    conflicts_with JSONB DEFAULT '[]',  -- Feature flag keys this conflicts with

    -- UI
    display_order INT DEFAULT 0,
    is_highlighted BOOLEAN DEFAULT false,
    badge_text VARCHAR(50),  -- e.g., 'NEW', 'BETA', 'POPULAR'

    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_feature_descriptors_category ON feature_descriptors(category);
CREATE INDEX idx_feature_descriptors_flag_id ON feature_descriptors(feature_flag_id);
```

---

##### Story 107.2: Feature Category Management

As a **platform administrator**,
I want to **organize features into categories**,
So that **users can browse features by domain**.

**Acceptance Criteria:**

**Given** features have descriptors
**When** categories are managed
**Then**:
  - Categories have: name, description, icon, color
  - Categories can be hierarchical (parent/child)
  - Categories have display order
  - Features can be filtered by category
  - Category metadata supports i18n
**And** categories appear in feature browsing UI

**Technical Notes:**
- Create `feature_categories` table
- Support nested categories for advanced UI
- Include color/theme for visual grouping

**Files to Modify:**
- `backend/crates/db/src/models/platform_admin.rs` (add FeatureCategory model)
- `backend/crates/db/src/repositories/feature_flag.rs` (add category methods)
- `backend/servers/api-server/src/routes/platform_admin.rs` (add category endpoints)

**Database Schema:**
```sql
CREATE TABLE feature_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key VARCHAR(50) UNIQUE NOT NULL,  -- e.g., 'property_management', 'ai_features'
    name VARCHAR(100) NOT NULL,
    description TEXT,
    icon VARCHAR(50),
    color VARCHAR(7),  -- Hex color e.g., '#3B82F6'

    parent_id UUID REFERENCES feature_categories(id),
    display_order INT DEFAULT 0,

    translations JSONB DEFAULT '{}',
    metadata JSONB DEFAULT '{}',

    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

---

##### Story 107.3: User Type Feature Matrix

As a **platform administrator**,
I want to **configure which features are available per user type**,
So that **different user roles see relevant features**.

**Acceptance Criteria:**

**Given** features and user types exist
**When** the feature matrix is configured
**Then**:
  - Each feature can be enabled/disabled per user type
  - User types include: Owner, Tenant, Manager, Organization Admin, etc.
  - Matrix supports "included", "optional", "excluded" states
  - Optional features can be toggled by the user
  - API returns features filtered by user type
**And** users only see applicable features

**Technical Notes:**
- Create `feature_user_type_access` table
- Access states: 'included' (always on), 'optional' (user choice), 'excluded' (not available)
- Query integration with existing role system

**Files to Modify:**
- `backend/crates/db/src/models/platform_admin.rs` (add FeatureUserTypeAccess)
- `backend/crates/db/src/repositories/feature_flag.rs` (add matrix methods)
- `backend/servers/api-server/src/routes/platform_admin.rs` (add matrix endpoints)

**Database Schema:**
```sql
CREATE TYPE feature_access_state AS ENUM ('included', 'optional', 'excluded');

CREATE TABLE feature_user_type_access (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    feature_flag_id UUID NOT NULL REFERENCES feature_flags(id) ON DELETE CASCADE,
    user_type VARCHAR(50) NOT NULL,  -- e.g., 'owner', 'tenant', 'manager', 'org_admin'

    access_state feature_access_state NOT NULL DEFAULT 'excluded',

    -- Override settings
    can_override BOOLEAN DEFAULT false,  -- Can user change their preference
    default_enabled BOOLEAN DEFAULT true,  -- Default state for 'optional'

    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(feature_flag_id, user_type)
);

CREATE INDEX idx_feature_user_type_access_user_type ON feature_user_type_access(user_type);
```

---

##### Story 107.4: Feature Catalog API

As a **frontend developer**,
I want to **query the feature catalog with filtering**,
So that **I can display features in forms and dashboards**.

**Acceptance Criteria:**

**Given** features with descriptors exist
**When** the catalog API is called
**Then**:
  - Features can be filtered by category, user_type, package
  - Response includes full descriptor with i18n
  - Response indicates current enabled state for user
  - Response includes dependencies and conflicts
  - Pagination and sorting are supported
**And** frontend can render feature selection forms

**Technical Notes:**
- New endpoint: `GET /api/v1/features/catalog`
- Query params: category, user_type, package_id, search, locale
- Return FeatureCatalogItem with descriptor + state

**Files to Create/Modify:**
- `backend/servers/api-server/src/routes/features.rs` (create new routes file)
- `backend/servers/api-server/src/routes/mod.rs` (add features module)

**API Response:**
```json
{
  "features": [
    {
      "key": "ai_document_analysis",
      "descriptor": {
        "display_name": "AI Document Analysis",
        "short_description": "Automatically extract information from documents",
        "icon": "document-search",
        "category": "ai_features",
        "benefits": ["Save time", "Reduce errors"],
        "badge_text": "NEW"
      },
      "state": {
        "is_enabled": true,
        "access_state": "included",
        "can_toggle": false
      },
      "dependencies": ["ocr_engine"],
      "conflicts_with": []
    }
  ],
  "categories": [
    {"key": "ai_features", "name": "AI Features", "icon": "sparkles", "count": 5}
  ],
  "pagination": {"total": 45, "page": 1, "per_page": 20}
}
```

---

##### Story 107.5: Feature Selection Form Component

As a **user configuring their account**,
I want to **select features via checkboxes in a form**,
So that **I can customize my experience**.

**Acceptance Criteria:**

**Given** the feature catalog is available
**When** the selection form is displayed
**Then**:
  - Features grouped by category with collapsible sections
  - Each feature shows: checkbox, icon, name, short description
  - Clicking feature expands to show full description and benefits
  - Dependencies are automatically selected when required
  - Conflicts show warning and prevent selection
  - Form tracks changes and submits preferences
**And** user preferences are persisted

**Technical Notes:**
- Frontend component: FeatureSelectionForm
- API endpoint for saving preferences: `PUT /api/v1/users/me/features`
- Store preferences in `user_feature_preferences` table

**Files to Create/Modify:**
- `frontend/packages/ppt-web/src/components/features/FeatureSelectionForm.tsx`
- `frontend/packages/ppt-web/src/components/features/FeatureCard.tsx`
- `frontend/packages/ppt-web/src/components/features/FeatureCategoryGroup.tsx`
- `backend/servers/api-server/src/routes/users.rs` (add feature preferences endpoint)

**Database Schema:**
```sql
CREATE TABLE user_feature_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    feature_flag_id UUID NOT NULL REFERENCES feature_flags(id) ON DELETE CASCADE,

    is_enabled BOOLEAN NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(user_id, feature_flag_id)
);
```

---

#### Epic 108: Feature Packages & Bundles
**Goal:** Create a package system to bundle features together for monetization and simplified selection.

**Target Apps:** api-server, ppt-web
**Estimate:** 5 stories, ~1 week
**Dependencies:** Epic 107 (Feature Descriptors)
**Priority:** P1 - HIGH

**PRD Reference:** Monetization platform, Package management

---

##### Story 108.1: Feature Package Model

As a **platform administrator**,
I want to **create feature packages that bundle multiple features**,
So that **users can subscribe to feature bundles**.

**Acceptance Criteria:**

**Given** features exist with descriptors
**When** a package is created
**Then**:
  - Package has: name, description, icon, pricing info
  - Package contains multiple feature flags
  - Package can be standalone or add-on
  - Package supports versioning for changes
  - Package has validity period (optional)
**And** packages appear in pricing/selection UI

**Technical Notes:**
- Create `feature_packages` and `feature_package_items` tables
- Package types: 'base' (standalone subscription), 'addon' (requires base)
- Support package versioning for grandfathering

**Files to Modify:**
- `backend/crates/db/src/models/` (add feature_package.rs)
- `backend/crates/db/src/repositories/` (add feature_package.rs)
- `backend/crates/db/src/migrations/` (add package migration)

**Database Schema:**
```sql
CREATE TYPE package_type AS ENUM ('base', 'addon', 'trial');

CREATE TABLE feature_packages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key VARCHAR(50) UNIQUE NOT NULL,  -- e.g., 'starter', 'professional', 'enterprise'
    name VARCHAR(100) NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    description TEXT,
    short_description VARCHAR(255),
    icon VARCHAR(50),

    -- Type and hierarchy
    package_type package_type NOT NULL DEFAULT 'base',
    parent_package_id UUID REFERENCES feature_packages(id),  -- For addons

    -- Pricing (can link to subscription plans)
    linked_plan_id UUID REFERENCES subscription_plans(id),
    standalone_monthly_price DECIMAL(10,2),
    standalone_annual_price DECIMAL(10,2),
    currency VARCHAR(3) DEFAULT 'EUR',

    -- Limits
    max_users INT,
    max_buildings INT,
    max_units INT,

    -- Display
    display_order INT DEFAULT 0,
    is_highlighted BOOLEAN DEFAULT false,
    highlight_text VARCHAR(50),  -- e.g., 'Most Popular', 'Best Value'
    color VARCHAR(7),

    -- Status
    is_active BOOLEAN DEFAULT true,
    is_public BOOLEAN DEFAULT true,

    -- Versioning
    version INT DEFAULT 1,

    -- Validity
    valid_from TIMESTAMPTZ,
    valid_until TIMESTAMPTZ,

    -- Localization
    translations JSONB DEFAULT '{}',
    metadata JSONB DEFAULT '{}',

    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE feature_package_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    package_id UUID NOT NULL REFERENCES feature_packages(id) ON DELETE CASCADE,
    feature_flag_id UUID NOT NULL REFERENCES feature_flags(id) ON DELETE CASCADE,

    -- Override descriptor for this package context
    custom_description TEXT,

    -- Limits within package
    usage_limit INT,  -- e.g., "10 AI analyses per month"
    usage_unit VARCHAR(50),

    display_order INT DEFAULT 0,

    created_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(package_id, feature_flag_id)
);

CREATE INDEX idx_feature_packages_type ON feature_packages(package_type);
CREATE INDEX idx_feature_package_items_package ON feature_package_items(package_id);
```

---

##### Story 108.2: Package Management API

As a **platform administrator**,
I want to **manage feature packages via API**,
So that **I can create, update, and configure packages**.

**Acceptance Criteria:**

**Given** the package model exists
**When** package management endpoints are called
**Then**:
  - CRUD operations for packages work correctly
  - Features can be added/removed from packages
  - Package-feature relationships are validated (no duplicates)
  - Package versioning is handled on significant changes
  - Audit log tracks package modifications
**And** admin UI can manage packages

**Technical Notes:**
- Endpoints: `/api/v1/admin/feature-packages/*`
- Include batch operations for feature assignment
- Validate feature dependencies when modifying packages

**Files to Create/Modify:**
- `backend/servers/api-server/src/routes/feature_packages.rs` (create)
- `backend/servers/api-server/src/routes/mod.rs` (add module)
- `backend/crates/db/src/repositories/feature_package.rs` (repository)

**API Endpoints:**
```
GET    /api/v1/admin/feature-packages           # List packages
POST   /api/v1/admin/feature-packages           # Create package
GET    /api/v1/admin/feature-packages/:id       # Get package with features
PUT    /api/v1/admin/feature-packages/:id       # Update package
DELETE /api/v1/admin/feature-packages/:id       # Delete package (soft)
POST   /api/v1/admin/feature-packages/:id/features    # Add features
DELETE /api/v1/admin/feature-packages/:id/features/:fid  # Remove feature
POST   /api/v1/admin/feature-packages/:id/clone  # Clone package
```

---

##### Story 108.3: Package-Subscription Integration

As a **organization administrator**,
I want to **packages linked to subscription plans**,
So that **subscribing to a plan grants package features**.

**Acceptance Criteria:**

**Given** packages and subscription plans exist
**When** an organization subscribes to a plan
**Then**:
  - Linked packages are automatically activated
  - Feature flags from packages are enabled for organization
  - Plan changes update package access automatically
  - Downgrade warns about losing package features
  - Addon packages can be purchased separately
**And** subscription determines available features

**Technical Notes:**
- Link `feature_packages.linked_plan_id` to `subscription_plans.id`
- Create `organization_packages` for explicit package assignments
- Auto-sync packages when subscription changes

**Files to Modify:**
- `backend/crates/db/src/repositories/subscription.rs` (add package sync)
- `backend/servers/api-server/src/routes/subscriptions.rs` (trigger sync)
- `backend/crates/db/src/models/subscription.rs` (add OrganizationPackage)

**Database Schema:**
```sql
CREATE TABLE organization_packages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    package_id UUID NOT NULL REFERENCES feature_packages(id),

    -- Source
    source VARCHAR(50) NOT NULL,  -- 'subscription', 'addon_purchase', 'trial', 'manual'
    subscription_id UUID REFERENCES organization_subscriptions(id),

    -- Status
    is_active BOOLEAN DEFAULT true,
    activated_at TIMESTAMPTZ DEFAULT NOW(),
    deactivated_at TIMESTAMPTZ,

    -- Validity
    valid_from TIMESTAMPTZ DEFAULT NOW(),
    valid_until TIMESTAMPTZ,

    metadata JSONB DEFAULT '{}',

    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(organization_id, package_id)
);
```

---

##### Story 108.4: Package Comparison UI

As a **potential subscriber**,
I want to **compare feature packages side by side**,
So that **I can choose the right package for my needs**.

**Acceptance Criteria:**

**Given** multiple packages exist
**When** the comparison view is loaded
**Then**:
  - Packages shown in columns with features in rows
  - Feature availability shown with checkmark/cross/partial
  - Pricing clearly displayed per package
  - "Current package" highlighted if applicable
  - "Upgrade" and "Downgrade" buttons shown appropriately
  - Feature tooltips show descriptions on hover
**And** user can select and purchase packages

**Technical Notes:**
- Frontend component: PackageComparisonTable
- API: `GET /api/v1/features/packages/compare?ids=...`
- Mobile: Swipeable comparison cards

**Files to Create/Modify:**
- `frontend/packages/ppt-web/src/components/packages/PackageComparisonTable.tsx`
- `frontend/packages/ppt-web/src/components/packages/PackageCard.tsx`
- `frontend/packages/ppt-web/src/pages/pricing/index.tsx`

---

##### Story 108.5: Addon Package Purchase Flow

As an **organization administrator**,
I want to **purchase addon packages for my organization**,
So that **we can access additional features**.

**Acceptance Criteria:**

**Given** addon packages are available
**When** an addon is purchased
**Then**:
  - User can browse available addons
  - Addon price is shown (one-time or recurring)
  - Payment is processed via existing billing system
  - Package is immediately activated on success
  - Invoice is generated for the addon
  - Addon appears in organization's active packages
**And** addon features are accessible immediately

**Technical Notes:**
- Integrate with existing subscription/payment system
- Create invoice line item type 'addon'
- Handle both one-time and recurring addon billing

**Files to Modify:**
- `backend/servers/api-server/src/routes/subscriptions.rs` (add addon purchase)
- `backend/crates/db/src/repositories/subscription.rs` (addon methods)
- `frontend/packages/ppt-web/src/pages/settings/addons.tsx`

---

### Phase 35: User Type Differentiation & Advanced Features

#### Epic 109: User Type Feature Experience
**Goal:** Deliver differentiated feature experiences based on user type with appropriate UI/UX.

**Target Apps:** api-server, ppt-web, mobile
**Estimate:** 4 stories, ~1 week
**Dependencies:** Epic 107 (Feature Matrix), Epic 108 (Packages)
**Priority:** P1 - HIGH

**PRD Reference:** User type differentiation

---

##### Story 109.1: Role-Based Feature Resolution

As a **user with a specific role**,
I want to **features resolved based on my role and organization**,
So that **I only see features relevant to me**.

**Acceptance Criteria:**

**Given** user type feature matrix exists
**When** features are resolved for a user
**Then**:
  - Resolution order: user preference → role → organization → package → global
  - Excluded features never shown regardless of other settings
  - Included features always enabled
  - Optional features respect user preference
  - API response is performant (< 100ms)
**And** feature resolution is consistent across platforms

**Technical Notes:**
- Enhance `resolve_all_for_context` in feature_flag repository
- Add caching layer for resolved features (Redis)
- Include user type in resolution context

**Files to Modify:**
- `backend/crates/db/src/repositories/feature_flag.rs` (enhance resolution)
- `backend/servers/api-server/src/services/` (add feature_service.rs)
- `backend/servers/api-server/src/state.rs` (add feature service)

**Resolution Logic:**
```rust
async fn resolve_feature_for_user(
    &self,
    flag_key: &str,
    user_id: Uuid,
    org_id: Uuid,
    role: &str,
) -> FeatureState {
    // 1. Check user type access matrix
    let type_access = self.get_user_type_access(flag_key, role).await?;
    if type_access == Excluded { return FeatureState::Disabled; }

    // 2. Check organization packages
    let org_packages = self.get_org_packages(org_id).await?;
    let in_package = org_packages.iter().any(|p| p.contains_feature(flag_key));

    // 3. Check user preference (for optional features)
    if type_access == Optional {
        if let Some(pref) = self.get_user_preference(user_id, flag_key).await? {
            return if pref.is_enabled { Enabled } else { Disabled };
        }
    }

    // 4. Check org/role overrides
    // 5. Return default based on access state and package
}
```

---

##### Story 109.2: Feature-Aware Navigation

As a **user browsing the application**,
I want to **navigation reflecting my available features**,
So that **I don't see options I can't use**.

**Acceptance Criteria:**

**Given** features are resolved for user
**When** navigation is rendered
**Then**:
  - Menu items for disabled features are hidden
  - Disabled features with "teaser" flag show locked icon
  - Clicking locked features shows upgrade prompt
  - Navigation is consistent across web and mobile
  - Navigation updates in real-time on feature changes
**And** navigation is clean and relevant

**Technical Notes:**
- Create navigation config mapping menu items to feature flags
- Frontend hook: `useFeatureNavigation()`
- Add `show_teaser_when_disabled` to feature descriptors

**Files to Create/Modify:**
- `frontend/packages/ppt-web/src/hooks/useFeatureNavigation.ts`
- `frontend/packages/ppt-web/src/components/navigation/FeatureAwareNav.tsx`
- `frontend/packages/ppt-web/src/config/navigation-features.ts`

---

##### Story 109.3: Feature Upgrade Prompts

As a **user encountering a disabled feature**,
I want to **see an upgrade option**,
So that **I can access the feature if I want it**.

**Acceptance Criteria:**

**Given** a feature is disabled but visible as teaser
**When** user attempts to access it
**Then**:
  - Modal shows feature description and benefits
  - Shows which package(s) include this feature
  - Shows pricing for upgrade path
  - One-click upgrade for org admins
  - Non-admins see "Request from admin" option
**And** upgrade path is clear and frictionless

**Technical Notes:**
- Create FeatureUpgradeModal component
- API: `GET /api/v1/features/:key/upgrade-options`
- Track upgrade prompt impressions for analytics

**Files to Create/Modify:**
- `frontend/packages/ppt-web/src/components/features/FeatureUpgradeModal.tsx`
- `frontend/packages/ppt-web/src/components/features/FeatureGate.tsx`
- `backend/servers/api-server/src/routes/features.rs` (add upgrade-options endpoint)

---

##### Story 109.4: Feature Usage Analytics

As a **platform administrator**,
I want to **track feature usage by user type**,
So that **I can optimize packages and pricing**.

**Acceptance Criteria:**

**Given** features are being used
**When** analytics are collected
**Then**:
  - Feature access events are logged
  - Usage is aggregated by feature, user type, organization
  - Upgrade prompt conversions are tracked
  - Reports show feature adoption rates
  - Data supports A/B testing feature placement
**And** business decisions are data-driven

**Technical Notes:**
- Create `feature_usage_events` table
- Background aggregation job for reporting
- Admin dashboard widget for feature analytics

**Files to Create/Modify:**
- `backend/crates/db/src/models/analytics.rs` (add feature analytics)
- `backend/crates/db/src/repositories/analytics.rs` (add feature methods)
- `backend/servers/api-server/src/routes/analytics.rs` (add endpoints)

**Database Schema:**
```sql
CREATE TABLE feature_usage_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    feature_flag_id UUID NOT NULL REFERENCES feature_flags(id),
    user_id UUID REFERENCES users(id),
    organization_id UUID REFERENCES organizations(id),

    event_type VARCHAR(50) NOT NULL,  -- 'access', 'blocked', 'upgrade_prompt', 'upgrade_clicked'
    user_type VARCHAR(50),

    metadata JSONB DEFAULT '{}',

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_feature_usage_events_feature ON feature_usage_events(feature_flag_id);
CREATE INDEX idx_feature_usage_events_created ON feature_usage_events(created_at);
```

---

#### Epic 110: Feature Toggle Guards & SDK
**Goal:** Implement robust feature guards in code and provide SDK for consistent feature checking.

**Target Apps:** api-server, ppt-web, mobile
**Estimate:** 4 stories, ~1 week
**Dependencies:** Epic 109 (Feature Resolution)
**Priority:** P2 - MEDIUM

**PRD Reference:** Feature flags integration

---

##### Story 110.1: Backend Feature Guard Middleware

As a **backend developer**,
I want to **guard API endpoints with feature flags**,
So that **disabled features return appropriate errors**.

**Acceptance Criteria:**

**Given** an endpoint is guarded by a feature flag
**When** request is made with feature disabled
**Then**:
  - Returns 403 Forbidden with clear message
  - Response includes feature key and upgrade path
  - Audit log records blocked access
  - Whitelisted users can bypass (for testing)
  - Performance impact < 5ms per request
**And** feature gates are declarative and maintainable

**Technical Notes:**
- Create `require_feature` macro/attribute for Axum handlers
- Cache resolved features per request context
- Include feature check in existing auth middleware

**Files to Create/Modify:**
- `backend/crates/api-core/src/middleware/feature_guard.rs` (create)
- `backend/crates/api-core/src/extractors/` (add FeatureContext extractor)
- `backend/servers/api-server/src/routes/*.rs` (apply guards)

**Usage Example:**
```rust
#[require_feature("ai_document_analysis")]
async fn analyze_document(
    State(state): State<AppState>,
    feature_ctx: FeatureContext,  // Extracted resolved features
    Json(payload): Json<AnalyzeRequest>,
) -> Result<Json<AnalyzeResponse>, ApiError> {
    // Feature is guaranteed enabled here
}
```

---

##### Story 110.2: Frontend Feature SDK

As a **frontend developer**,
I want to **a simple SDK for feature checking**,
So that **UI components respect feature flags consistently**.

**Acceptance Criteria:**

**Given** features are loaded from API
**When** feature checks are performed
**Then**:
  - `useFeature(key)` hook returns { enabled, loading, descriptor }
  - `<FeatureGate feature="key">` component conditionally renders
  - `<FeatureGate feature="key" fallback={...}>` shows fallback
  - Features are cached and refreshed appropriately
  - TypeScript types for feature keys (codegen from API)
**And** feature checks are consistent and ergonomic

**Technical Notes:**
- Create @ppt/features package
- Generate TypeScript types from OpenAPI
- Use TanStack Query for caching/refresh

**Files to Create/Modify:**
- `frontend/packages/features/` (create new package)
- `frontend/packages/features/src/hooks/useFeature.ts`
- `frontend/packages/features/src/components/FeatureGate.tsx`
- `frontend/packages/features/src/context/FeatureProvider.tsx`
- `frontend/packages/features/src/generated/feature-keys.ts` (codegen)

**Usage Example:**
```tsx
// Hook usage
const { enabled, descriptor } = useFeature('ai_document_analysis');

// Component usage
<FeatureGate feature="ai_document_analysis" fallback={<UpgradePrompt />}>
  <AIAnalysisButton />
</FeatureGate>

// Multiple features
<FeatureGate features={['ai_ocr', 'ai_analysis']} requireAll>
  <AdvancedAIPanel />
</FeatureGate>
```

---

##### Story 110.3: Feature Toggle Dashboard

As a **organization administrator**,
I want to **view and toggle optional features for my organization**,
So that **I can customize our experience**.

**Acceptance Criteria:**

**Given** organization has optional features
**When** the dashboard is accessed
**Then**:
  - Shows all features grouped by category
  - Indicates included/optional/excluded status
  - Allows toggling optional features on/off
  - Shows which package provides each feature
  - Warns about dependencies when disabling
  - Changes take effect immediately
**And** org admin has full control over optional features

**Technical Notes:**
- Page: `/settings/features`
- API: `PUT /api/v1/organizations/:id/features`
- WebSocket for real-time toggle feedback

**Files to Create/Modify:**
- `frontend/packages/ppt-web/src/pages/settings/features.tsx`
- `backend/servers/api-server/src/routes/organizations.rs` (add feature toggle)
- `backend/crates/db/src/repositories/organization.rs` (feature preferences)

---

##### Story 110.4: Feature Flag CLI Tools

As a **developer or DevOps engineer**,
I want to **manage feature flags via CLI**,
So that **I can script flag changes and deployments**.

**Acceptance Criteria:**

**Given** the CLI tool is available
**When** feature commands are run
**Then**:
  - `ppt features list` shows all flags with status
  - `ppt features enable <key> --org <id>` enables for org
  - `ppt features disable <key> --user <id>` disables for user
  - `ppt features export` exports flag configuration
  - `ppt features import <file>` imports configuration
  - `ppt features sync` syncs flags from config file
**And** feature management is automatable

**Technical Notes:**
- Extend existing CLI or create feature management subcommand
- Support YAML/JSON configuration files
- Integrate with CI/CD pipelines

**Files to Create/Modify:**
- `backend/cli/src/commands/features.rs` (create)
- `backend/cli/src/main.rs` (add subcommand)
- `docs/feature-flags-cli.md` (documentation)

---

## Summary

| Phase | Epics | Stories | Priority |
|-------|-------|---------|----------|
| 34: Feature Management & Monetization | 107-108 | 10 | P1 |
| 35: User Type Differentiation & Advanced | 109-110 | 8 | P1/P2 |

**Total:** 4 Epics, 18 Stories

### Implementation Order

1. **Epic 107** - Feature Descriptors & Catalog (P1) ~1 week
   - Stories 107.1-107.5: Rich feature metadata and UI

2. **Epic 108** - Feature Packages & Bundles (P1) ~1 week
   - Stories 108.1-108.5: Package system and monetization

3. **Epic 109** - User Type Feature Experience (P1) ~1 week
   - Stories 109.1-109.4: Role-based features and UX

4. **Epic 110** - Feature Toggle Guards & SDK (P2) ~1 week
   - Stories 110.1-110.4: Developer tooling and dashboards

### Parallel Implementation

- Epic 107 and early Epic 108 can be worked in parallel (different focus)
- Epic 109 requires Epic 107 completion
- Epic 110 can start once Epic 109.1 is complete

### Dependencies

```
Epic 107 (Descriptors) → Independent, builds on existing feature flags
Epic 108 (Packages) → Requires Epic 107.1 (descriptors)
Epic 109 (User Types) → Requires Epic 107.3 (user type matrix)
Epic 110 (SDK/Guards) → Requires Epic 109.1 (resolution logic)
```

### Feature Flags

| Flag | Stories | Default |
|------|---------|---------|
| `feature_management.descriptors_enabled` | 107.1-107.5 | false |
| `feature_management.packages_enabled` | 108.1-108.5 | false |
| `feature_management.user_type_matrix_enabled` | 109.1-109.4 | false |
| `feature_management.sdk_guards_enabled` | 110.1-110.4 | false |

### Integration Points

| System | Integration |
|--------|-------------|
| Existing Feature Flags (Epic 89) | Extended with descriptors and user type matrix |
| Subscription System (Epic 26) | Packages linked to plans, addon purchases |
| Authentication | User type from JWT claims for resolution |
| Role System (Epic 2A) | Role-based feature access |
| Analytics | Feature usage tracking and reporting |
