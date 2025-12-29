# Story 81.2: Report Execution History

Status: pending

## Story

As a **property manager**,
I want to **view the history of report executions**,
So that **I can access past reports and monitor execution status**.

## Acceptance Criteria

1. **AC-1: Execution History List**
   - Given I have scheduled reports
   - When I view a schedule's history
   - Then I see a list of all past executions
   - And each entry shows date, status, and download link

2. **AC-2: Report Download**
   - Given I am viewing execution history
   - When I click download on a past report
   - Then the report file is downloaded
   - And the file is in the original format (PDF, Excel, CSV)

3. **AC-3: Execution Status Tracking**
   - Given a scheduled report is running
   - When I view the execution status
   - Then I see current progress (pending, running, completed, failed)
   - And failed executions show error details

4. **AC-4: History Filtering**
   - Given I have many executions
   - When I filter by date range or status
   - Then only matching executions are shown
   - And filters persist across page navigation

5. **AC-5: Retry Failed Executions**
   - Given a report execution failed
   - When I click retry
   - Then a new execution is triggered
   - And I am notified when complete

## Tasks / Subtasks

- [ ] Task 1: Create Execution History Component (AC: 1, 3)
  - [ ] 1.1 Create `/frontend/apps/ppt-web/src/features/reports/components/ExecutionHistory.tsx`
  - [ ] 1.2 Fetch history from `/api/v1/reports/schedules/{id}/executions`
  - [ ] 1.3 Display status badges (pending, running, completed, failed)
  - [ ] 1.4 Show execution timestamp and duration
  - [ ] 1.5 Add pagination for long histories

- [ ] Task 2: Implement Report Download (AC: 2)
  - [ ] 2.1 Add `useDownloadReport` hook
  - [ ] 2.2 Fetch presigned URL from `/api/v1/reports/executions/{id}/download`
  - [ ] 2.3 Trigger browser download with correct filename
  - [ ] 2.4 Handle expired URLs with refresh

- [ ] Task 3: Create History Filters (AC: 4)
  - [ ] 3.1 Create `/frontend/apps/ppt-web/src/features/reports/components/HistoryFilters.tsx`
  - [ ] 3.2 Add date range picker
  - [ ] 3.3 Add status filter dropdown
  - [ ] 3.4 Persist filters in URL query params
  - [ ] 3.5 Apply filters to API query

- [ ] Task 4: Implement Retry Functionality (AC: 5)
  - [ ] 4.1 Add `useRetryExecution` mutation
  - [ ] 4.2 Add retry button for failed executions
  - [ ] 4.3 Show loading state during retry
  - [ ] 4.4 Refresh history list after retry

- [ ] Task 5: Wire to ReportsPage (AC: 1, 2, 3, 4, 5)
  - [ ] 5.1 Update `/frontend/apps/ppt-web/src/features/reports/pages/ReportsPage.tsx`
  - [ ] 5.2 Add "View History" button to schedules (lines 321-328)
  - [ ] 5.3 Create history panel/modal
  - [ ] 5.4 Add real-time status updates via polling or WebSocket

## Dev Notes

### Architecture Requirements
- Paginated history list with efficient loading
- Background polling for running executions
- Presigned URLs for secure downloads
- Error details for troubleshooting

### Technical Specifications
- API Endpoints:
  - GET `/api/v1/reports/schedules/{id}/executions` - List executions
  - GET `/api/v1/reports/executions/{id}` - Execution details
  - GET `/api/v1/reports/executions/{id}/download` - Download URL
  - POST `/api/v1/reports/executions/{id}/retry` - Retry execution

### Execution Model
```typescript
interface ReportExecution {
  id: string;
  scheduleId: string;
  status: ExecutionStatus;
  startedAt: string;
  completedAt?: string;
  durationMs?: number;
  fileKey?: string;       // S3 key for completed reports
  fileName?: string;
  fileSize?: number;
  error?: ExecutionError;
  createdAt: string;
}

type ExecutionStatus = 'pending' | 'running' | 'completed' | 'failed';

interface ExecutionError {
  code: string;
  message: string;
  details?: string;
}

interface ExecutionFilters {
  status?: ExecutionStatus;
  dateFrom?: string;
  dateTo?: string;
}
```

### Status Badge Styling
```typescript
const statusColors: Record<ExecutionStatus, string> = {
  pending: 'gray',
  running: 'blue',
  completed: 'green',
  failed: 'red',
};
```

### Download URL Response
```typescript
interface DownloadUrlResponse {
  url: string;           // Presigned S3 URL
  expiresAt: string;     // URL expiry timestamp
  fileName: string;
  contentType: string;
}
```

### File List (to create/modify)

**Create:**
- `/frontend/apps/ppt-web/src/features/reports/components/ExecutionHistory.tsx`
- `/frontend/apps/ppt-web/src/features/reports/components/HistoryFilters.tsx`
- `/frontend/apps/ppt-web/src/features/reports/components/ExecutionStatusBadge.tsx`
- `/frontend/apps/ppt-web/src/features/reports/hooks/useExecutionHistory.ts`
- `/frontend/apps/ppt-web/src/features/reports/hooks/useDownloadReport.ts`

**Modify:**
- `/frontend/apps/ppt-web/src/features/reports/pages/ReportsPage.tsx` - Add history view
- `/frontend/packages/api-client/src/reports/hooks.ts` - Add history hooks
- `/frontend/packages/api-client/src/reports/api.ts` - Add endpoints
- `/frontend/packages/api-client/src/reports/types.ts` - Add types

### Existing TODO Reference
```typescript
// From ReportsPage.tsx:321-328
// TODO: Implement report execution history view
// - Show list of past executions with status
// - Allow downloading completed reports
// - Show error details for failed executions
```

### Polling Strategy
```typescript
// Poll running executions every 5 seconds
const { data } = useExecutionHistory(scheduleId, {
  refetchInterval: (data) =>
    data?.some(e => e.status === 'running') ? 5000 : false,
});
```

### Dependencies
- Story 81.1 (Report Schedule Editing) - Schedule management
- Story 79.1 (API Client Integration) - Base API configuration

### References
- [Source: frontend/apps/ppt-web/src/features/reports/pages/ReportsPage.tsx:321-328]
- [UC-29: Reports]
