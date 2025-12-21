# Story 6.6: Neighbor Information (Privacy-Aware)

Status: ready-for-dev

## Story

As a **resident**,
I want to **see information about my neighbors**,
so that **I can connect with my community**.

## Acceptance Criteria

1. **AC-1: View Neighbors with Public Profiles**
   - Given a resident views neighbor list
   - When neighbors have public profiles
   - Then name and unit number are displayed
   - And contact options if enabled

2. **AC-2: Hidden Profile Display**
   - Given a neighbor has hidden their profile
   - When their entry would be displayed
   - Then they appear as "Resident of Unit X"
   - And no contact information shown

3. **AC-3: Update Visibility**
   - Given a resident updates their visibility
   - When they toggle profile visibility
   - Then their display to neighbors updates immediately

## Tasks / Subtasks

- [ ] Task 1: Database Schema & Migrations (AC: 1, 2, 3)
  - [ ] 1.1 Add privacy columns to users table: profile_visibility (ENUM: visible, hidden, contacts_only), show_contact_info (BOOLEAN DEFAULT false)
  - [ ] 1.2 Create index for neighbor queries: idx_users_building_unit

- [ ] Task 2: Backend Domain Models & Repository (AC: 1, 2, 3)
  - [ ] 2.1 Add ProfileVisibility enum to User model
  - [ ] 2.2 Implement get_neighbors(user_id, building_id) method respecting privacy
  - [ ] 2.3 Implement update_profile_visibility(user_id, visibility) method
  - [ ] 2.4 Create NeighborView model with privacy-aware fields

- [ ] Task 3: Backend API Handlers (AC: 1, 2, 3)
  - [ ] 3.1 Create GET `/api/v1/neighbors` handler - list neighbors in same building
  - [ ] 3.2 Create GET `/api/v1/users/me/privacy` handler - get current privacy settings
  - [ ] 3.3 Create PUT `/api/v1/users/me/privacy` handler - update privacy settings
  - [ ] 3.4 Ensure neighbor list respects privacy settings

- [ ] Task 4: TypeSpec API Specification (AC: 1, 2, 3)
  - [ ] 4.1 Add NeighborResponse model with privacy-aware fields
  - [ ] 4.2 Add ProfileVisibility enum to TypeSpec
  - [ ] 4.3 Add PrivacySettingsRequest/Response models
  - [ ] 4.4 Document all endpoints with OpenAPI annotations

- [ ] Task 5: Frontend Components - ppt-web (AC: 1, 2, 3)
  - [ ] 5.1 Create NeighborList component
  - [ ] 5.2 Create NeighborCard component with conditional display
  - [ ] 5.3 Create PrivacySettings component for user preferences
  - [ ] 5.4 Add "Contact" button when contact info is enabled
  - [ ] 5.5 Show "Resident of Unit X" for hidden profiles

- [ ] Task 6: Frontend State & API Integration (AC: 1, 2, 3)
  - [ ] 6.1 Create useNeighbors hook with TanStack Query
  - [ ] 6.2 Create usePrivacySettings hook
  - [ ] 6.3 Create useUpdatePrivacySettings mutation hook

- [ ] Task 7: Frontend Pages (AC: 1, 2, 3)
  - [ ] 7.1 Create NeighborsPage showing building residents
  - [ ] 7.2 Add privacy settings section to user profile/settings page
  - [ ] 7.3 Add neighbors link to navigation

- [ ] Task 8: Integration Testing (AC: 1, 2, 3)
  - [ ] 8.1 Write backend tests for neighbor list with public profiles
  - [ ] 8.2 Write backend tests for hidden profile display
  - [ ] 8.3 Write backend tests for privacy settings update
  - [ ] 8.4 Write backend tests for contacts_only visibility mode

## Dev Notes

### Architecture Requirements
- Follow multi-tenancy pattern: all queries MUST include TenantContext
- Neighbor = same building, different unit
- Privacy must be enforced at database/API level, not just frontend

### Technical Specifications
- Database: PostgreSQL, add columns to existing users table
- Backend: Rust + Axum handlers in new neighbors module
- Privacy modes:
  - `visible`: Full name and unit shown
  - `hidden`: Shows as "Resident of Unit X"
  - `contacts_only`: Name shown, but no contact info unless already connected

### Privacy Considerations
- Default visibility should be `visible` for community building
- User can change at any time
- Contact info (email, phone) only shown if explicitly enabled
- Hidden users still appear in neighbor count but anonymized

### Dependencies
- Epic 1: User authentication and profile
- Epic 2A/2B: Building/unit associations

### References
- [Source: _bmad-output/epics.md#Epic-6-Story-6.6]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

N/A

### Completion Notes List

(To be filled during development)

### File List

(To be filled during development)
