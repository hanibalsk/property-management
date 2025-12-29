# Story 80.1: Disputes API Integration

Status: pending

## Story

As a **ppt-web user**,
I want to **view and manage disputes through the connected backend API**,
So that **I can track and resolve disputes with real data persistence**.

## Acceptance Criteria

1. **AC-1: Disputes List Fetching**
   - Given I am on the disputes page
   - When the page loads
   - Then disputes are fetched from `/api/v1/disputes` using TanStack Query
   - And loading state is displayed during fetch
   - And disputes are paginated with proper controls

2. **AC-2: Dispute Detail View**
   - Given I click on a dispute in the list
   - When the detail page loads
   - Then dispute details are fetched from `/api/v1/disputes/{id}`
   - And all dispute information is displayed (parties, evidence, timeline)

3. **AC-3: Dispute Filtering and Search**
   - Given I am viewing the disputes list
   - When I apply filters (status, type, date range)
   - Then the API is called with filter parameters
   - And results update accordingly

4. **AC-4: Real-time Status Updates**
   - Given I am viewing a dispute
   - When the dispute status changes on the backend
   - Then the UI updates to reflect the new status
   - And a notification is shown

5. **AC-5: Error Handling**
   - Given an API call fails
   - When viewing or managing disputes
   - Then appropriate error messages are displayed
   - And retry options are available

## Tasks / Subtasks

- [ ] Task 1: Create Disputes API Client Module (AC: 1, 2, 3, 5)
  - [ ] 1.1 Create `/frontend/packages/api-client/src/disputes/types.ts`
  - [ ] 1.2 Create `/frontend/packages/api-client/src/disputes/api.ts`
  - [ ] 1.3 Create `/frontend/packages/api-client/src/disputes/hooks.ts`
  - [ ] 1.4 Create `/frontend/packages/api-client/src/disputes/index.ts`
  - [ ] 1.5 Export from main api-client index

- [ ] Task 2: Implement useDisputes Hook (AC: 1, 3)
  - [ ] 2.1 Create `useDisputes` query hook with pagination
  - [ ] 2.2 Add filter parameters (status, type, dateFrom, dateTo)
  - [ ] 2.3 Implement query key factory for cache management
  - [ ] 2.4 Add prefetching for next page

- [ ] Task 3: Implement useDispute Detail Hook (AC: 2)
  - [ ] 3.1 Create `useDispute(id)` query hook
  - [ ] 3.2 Include related data (evidence, timeline, parties)
  - [ ] 3.3 Add stale time configuration

- [ ] Task 4: Wire DisputesPage to API (AC: 1, 2, 3, 5)
  - [ ] 4.1 Update `/frontend/apps/ppt-web/src/features/disputes/pages/DisputesPage.tsx`
  - [ ] 4.2 Replace mock data with useDisputes hook
  - [ ] 4.3 Add loading and error states
  - [ ] 4.4 Wire filter controls to query params

- [ ] Task 5: Wire DisputeDetailPage to API (AC: 2, 4)
  - [ ] 5.1 Update or create dispute detail page
  - [ ] 5.2 Wire to useDispute hook
  - [ ] 5.3 Display timeline, evidence, and parties
  - [ ] 5.4 Add WebSocket subscription for real-time updates

## Dev Notes

### Architecture Requirements
- Follow existing api-client patterns from announcements, faults modules
- Use TanStack Query v5 for server state management
- Implement query invalidation on mutations
- Support optimistic updates where appropriate

### Technical Specifications
- API Base: `/api/v1/disputes`
- Endpoints:
  - GET `/api/v1/disputes` - List with pagination and filters
  - GET `/api/v1/disputes/{id}` - Single dispute with details
  - GET `/api/v1/disputes/{id}/timeline` - Dispute timeline
  - GET `/api/v1/disputes/{id}/evidence` - Dispute evidence

### Type Definitions
```typescript
interface Dispute {
  id: string;
  organizationId: string;
  unitId: string;
  type: DisputeType;
  status: DisputeStatus;
  subject: string;
  description: string;
  filedBy: string;
  filedAt: string;
  respondent?: string;
  assignedMediator?: string;
  resolutionDeadline?: string;
  resolution?: string;
  resolvedAt?: string;
  createdAt: string;
  updatedAt: string;
}

type DisputeType = 'noise' | 'damage' | 'payment' | 'lease' | 'maintenance' | 'other';
type DisputeStatus = 'filed' | 'under_review' | 'mediation' | 'escalated' | 'resolved' | 'closed';

interface DisputeFilters {
  status?: DisputeStatus;
  type?: DisputeType;
  dateFrom?: string;
  dateTo?: string;
  unitId?: string;
}
```

### Query Key Factory
```typescript
const disputeKeys = {
  all: ['disputes'] as const,
  lists: () => [...disputeKeys.all, 'list'] as const,
  list: (filters: DisputeFilters) => [...disputeKeys.lists(), filters] as const,
  details: () => [...disputeKeys.all, 'detail'] as const,
  detail: (id: string) => [...disputeKeys.details(), id] as const,
  timeline: (id: string) => [...disputeKeys.detail(id), 'timeline'] as const,
  evidence: (id: string) => [...disputeKeys.detail(id), 'evidence'] as const,
};
```

### File List (to create/modify)

**Create:**
- `/frontend/packages/api-client/src/disputes/types.ts`
- `/frontend/packages/api-client/src/disputes/api.ts`
- `/frontend/packages/api-client/src/disputes/hooks.ts`
- `/frontend/packages/api-client/src/disputes/index.ts`

**Modify:**
- `/frontend/packages/api-client/src/index.ts` - Add disputes export
- `/frontend/apps/ppt-web/src/features/disputes/pages/DisputesPage.tsx`

### Dependencies
- Story 79.1 (API Client Integration) - Base API client configuration
- Backend disputes endpoints (Epic 77 - already implemented)

### References
- [Backend: backend/servers/api-server/src/routes/disputes.rs]
- [Pattern: frontend/packages/api-client/src/faults/]
- [UC-33: Dispute Resolution]
