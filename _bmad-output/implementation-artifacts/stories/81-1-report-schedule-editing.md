# Story 81.1: Report Schedule Editing

Status: pending

## Story

As a **property manager**,
I want to **edit existing report schedules**,
So that **I can modify report parameters without recreating schedules**.

## Acceptance Criteria

1. **AC-1: Edit Schedule Access**
   - Given I have existing report schedules
   - When I view the schedules list
   - Then I see an edit button for each schedule
   - And clicking edit opens the schedule editor

2. **AC-2: Schedule Parameter Modification**
   - Given I am editing a schedule
   - When I modify parameters (frequency, recipients, time)
   - Then changes are saved via API
   - And the schedule list updates to reflect changes

3. **AC-3: Report Type Configuration**
   - Given I am editing a schedule
   - When I change report parameters (date range, filters)
   - Then the preview updates to show expected output
   - And changes are validated before saving

4. **AC-4: Recipient Management**
   - Given I am editing schedule recipients
   - When I add or remove recipients
   - Then the recipient list is updated
   - And removed recipients no longer receive reports

5. **AC-5: Schedule Pause/Resume**
   - Given I want to temporarily disable a schedule
   - When I pause the schedule
   - Then reports are not generated until resumed
   - And the schedule shows paused status

## Tasks / Subtasks

- [ ] Task 1: Create Edit Schedule Modal (AC: 1, 2, 3)
  - [ ] 1.1 Create `/frontend/apps/ppt-web/src/features/reports/components/EditScheduleModal.tsx`
  - [ ] 1.2 Pre-populate form with existing schedule data
  - [ ] 1.3 Add frequency selector (daily, weekly, monthly)
  - [ ] 1.4 Add time/day picker for schedule timing
  - [ ] 1.5 Add validation for schedule parameters

- [ ] Task 2: Implement useUpdateSchedule Mutation (AC: 2, 4)
  - [ ] 2.1 Add `useUpdateSchedule` mutation hook
  - [ ] 2.2 Handle partial updates (PATCH semantics)
  - [ ] 2.3 Implement optimistic updates
  - [ ] 2.4 Invalidate schedules query on success

- [ ] Task 3: Create Recipient Manager Component (AC: 4)
  - [ ] 3.1 Create `/frontend/apps/ppt-web/src/features/reports/components/RecipientManager.tsx`
  - [ ] 3.2 Add recipient search and selection
  - [ ] 3.3 Show current recipients with remove option
  - [ ] 3.4 Support email addresses for external recipients
  - [ ] 3.5 Validate email format

- [ ] Task 4: Implement Pause/Resume Functionality (AC: 5)
  - [ ] 4.1 Add `usePauseSchedule` mutation
  - [ ] 4.2 Add `useResumeSchedule` mutation
  - [ ] 4.3 Add pause/resume toggle in schedule list
  - [ ] 4.4 Show paused indicator on schedule row

- [ ] Task 5: Wire to ReportsPage (AC: 1, 2, 3, 4, 5)
  - [ ] 5.1 Update `/frontend/apps/ppt-web/src/features/reports/pages/ReportsPage.tsx`
  - [ ] 5.2 Add edit button to schedule list items (lines 305-311)
  - [ ] 5.3 Integrate EditScheduleModal
  - [ ] 5.4 Add pause/resume buttons

## Dev Notes

### Architecture Requirements
- Modal-based editing for better UX
- Optimistic updates for responsive feel
- Validation before API call
- Support partial updates (only changed fields)

### Technical Specifications
- API Endpoints:
  - PATCH `/api/v1/reports/schedules/{id}` - Update schedule
  - PUT `/api/v1/reports/schedules/{id}/pause` - Pause schedule
  - PUT `/api/v1/reports/schedules/{id}/resume` - Resume schedule

### Update Schedule Request
```typescript
interface UpdateScheduleRequest {
  name?: string;
  reportType?: ReportType;
  frequency?: ScheduleFrequency;
  dayOfWeek?: number;      // 0-6 for weekly
  dayOfMonth?: number;     // 1-31 for monthly
  hour?: number;           // 0-23
  minute?: number;         // 0-59
  timezone?: string;
  recipients?: string[];
  parameters?: ReportParameters;
  enabled?: boolean;
}

type ScheduleFrequency = 'daily' | 'weekly' | 'monthly';

interface ReportParameters {
  dateRange?: 'last_week' | 'last_month' | 'last_quarter' | 'custom';
  buildingIds?: string[];
  includeCharts?: boolean;
  format?: 'pdf' | 'excel' | 'csv';
}
```

### File List (to create/modify)

**Create:**
- `/frontend/apps/ppt-web/src/features/reports/components/EditScheduleModal.tsx`
- `/frontend/apps/ppt-web/src/features/reports/components/RecipientManager.tsx`
- `/frontend/apps/ppt-web/src/features/reports/components/ScheduleFrequencyPicker.tsx`

**Modify:**
- `/frontend/apps/ppt-web/src/features/reports/pages/ReportsPage.tsx` - Add edit functionality
- `/frontend/packages/api-client/src/reports/hooks.ts` - Add mutations
- `/frontend/packages/api-client/src/reports/api.ts` - Add endpoints

### Existing TODO Reference
```typescript
// From ReportsPage.tsx:305-311
// TODO: Implement schedule editing
// - Load existing schedule data into form
// - Support modifying all schedule parameters
// - Handle recipient list changes
```

### Dependencies
- Story 79.1 (API Client Integration) - Base API configuration
- Story 79.3 (Error Handling) - Toast notifications

### References
- [Source: frontend/apps/ppt-web/src/features/reports/pages/ReportsPage.tsx:305-311]
- [UC-29: Reports]
