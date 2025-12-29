# Story 79.1: API Client Integration for Core Features

Status: pending

## Story

As a **ppt-web developer**,
I want to **connect all feature components to actual API endpoints using TanStack Query hooks**,
So that **users see real data from the backend instead of mock/prop-injected data**.

## Acceptance Criteria

1. **AC-1: Announcements API Integration**
   - Given I am on the announcements page
   - When the page loads
   - Then announcements are fetched from `/api/v1/announcements` using useQuery
   - And loading state is displayed during fetch
   - And error state is shown if fetch fails

2. **AC-2: Faults API Integration**
   - Given I am on the faults page
   - When I submit a new fault report
   - Then it is created via `/api/v1/faults` using useMutation
   - And the faults list is invalidated and refetched
   - And a success notification is shown

3. **AC-3: Documents API Integration**
   - Given I am browsing documents
   - When I request a download
   - Then a presigned URL is fetched from `/api/v1/documents/{id}/download`
   - And the file download is triggered

4. **AC-4: Voting API Integration**
   - Given I am viewing a vote
   - When I cast my vote
   - Then it is submitted via `/api/v1/votes/{id}/cast` using useMutation
   - And the vote results are updated in real-time

5. **AC-5: Error Handling**
   - Given an API call fails
   - When the error is returned
   - Then appropriate error UI is displayed with retry option
   - And the error is logged for debugging

## Tasks / Subtasks

- [ ] Task 1: Create shared API configuration (AC: 1, 2, 3, 4, 5)
  - [ ] 1.1 Create `/frontend/apps/ppt-web/src/lib/api.ts` with axios instance
  - [ ] 1.2 Configure base URL from environment variable `VITE_API_URL`
  - [ ] 1.3 Add request interceptor for JWT token injection from auth context
  - [ ] 1.4 Add response interceptor for error transformation
  - [ ] 1.5 Export typed API client for use in hooks

- [ ] Task 2: Implement announcements API integration (AC: 1)
  - [ ] 2.1 Update `/frontend/packages/api-client/src/announcements/hooks.ts` to use real endpoints
  - [ ] 2.2 Wire `AnnouncementsPage` to use `useAnnouncements` hook
  - [ ] 2.3 Add optimistic updates for create/update/delete operations
  - [ ] 2.4 Implement pagination with `useInfiniteQuery`

- [ ] Task 3: Implement faults API integration (AC: 2)
  - [ ] 3.1 Update `/frontend/packages/api-client/src/faults/hooks.ts` to use real endpoints
  - [ ] 3.2 Wire `FaultsPage` to use `useFaults` hook
  - [ ] 3.3 Implement `useCreateFault` mutation with image upload
  - [ ] 3.4 Add real-time status update polling every 30 seconds

- [ ] Task 4: Implement documents API integration (AC: 3)
  - [ ] 4.1 Update `/frontend/packages/api-client/src/documents/hooks.ts`
  - [ ] 4.2 Implement download with presigned URL handling
  - [ ] 4.3 Add upload progress tracking using axios onUploadProgress
  - [ ] 4.4 Wire `DocumentsPage` to use document hooks

- [ ] Task 5: Implement voting API integration (AC: 4)
  - [ ] 5.1 Update `/frontend/packages/api-client/src/voting/hooks.ts`
  - [ ] 5.2 Wire `VotingPage` to use `useVotes` and `useCastVote` hooks
  - [ ] 5.3 Implement optimistic updates for vote casting
  - [ ] 5.4 Add real-time result polling during active voting

- [ ] Task 6: Wire remaining features (AC: 1, 2, 3, 4)
  - [ ] 6.1 Connect `NeighborsPage` to `/api/v1/neighbors`
  - [ ] 6.2 Connect `MessagesPage` to `/api/v1/messages`
  - [ ] 6.3 Connect `FormsPage` to `/api/v1/forms`
  - [ ] 6.4 Connect `PersonMonthsPage` to `/api/v1/person-months`
  - [ ] 6.5 Connect `SelfReadingsPage` to `/api/v1/self-readings`

## Dev Notes

### Architecture Requirements
- Use TanStack Query v5 for all server state management
- Implement query invalidation for related data updates
- Cache responses appropriately per data type (staleTime, gcTime)
- All mutations should show loading state and handle errors
- Use `queryClient.invalidateQueries` after successful mutations

### Technical Specifications
- Base URL: `import.meta.env.VITE_API_URL || 'http://localhost:8080/api/v1'`
- JWT token: Retrieved from `AuthContext` and injected via interceptor
- Error format: Match backend `ErrorResponse` structure with `requestId`, `error`, `message`
- Retry logic: 3 retries for transient failures (network errors, 5xx)

### Query Key Conventions
```typescript
// Use factory functions for consistent keys
const queryKeys = {
  announcements: {
    all: ['announcements'] as const,
    list: (filters: Filters) => ['announcements', 'list', filters] as const,
    detail: (id: string) => ['announcements', 'detail', id] as const,
  },
  // ... similar for other entities
};
```

### File List (to create/modify)

**Create:**
- `/frontend/apps/ppt-web/src/lib/api.ts` - Axios instance with interceptors
- `/frontend/apps/ppt-web/src/lib/queryKeys.ts` - Query key factory functions

**Modify:**
- `/frontend/packages/api-client/src/announcements/hooks.ts` - Real API calls
- `/frontend/packages/api-client/src/faults/hooks.ts` - Real API calls
- `/frontend/packages/api-client/src/documents/hooks.ts` - Real API calls
- `/frontend/packages/api-client/src/voting/hooks.ts` - Real API calls
- `/frontend/apps/ppt-web/src/features/announcements/pages/AnnouncementsPage.tsx`
- `/frontend/apps/ppt-web/src/features/faults/pages/FaultsPage.tsx`
- `/frontend/apps/ppt-web/src/features/documents/pages/DocumentsPage.tsx`
- `/frontend/apps/ppt-web/src/features/voting/pages/VotingPage.tsx`

### Dependencies
- Epic 79.2 (Authentication Flow) - For JWT token access
- Backend API endpoints (already implemented)

### References
- [Source: completeness analysis - ppt-web API integration gaps]
- [Pattern: frontend/packages/api-client/src/announcements/hooks.ts]
- [Backend: backend/servers/api-server/src/routes/]
