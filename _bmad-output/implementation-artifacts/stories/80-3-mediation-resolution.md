# Story 80.3: Mediation and Resolution

Status: pending

## Story

As a **property manager or mediator**,
I want to **manage the dispute resolution lifecycle**,
So that **disputes can be properly mediated and resolved**.

## Acceptance Criteria

1. **AC-1: Dispute Assignment**
   - Given a dispute is filed and under review
   - When I assign a mediator
   - Then the mediator is notified
   - And the dispute status updates to "mediation"
   - And the assigned mediator appears on the dispute

2. **AC-2: Mediation Communication**
   - Given I am mediating a dispute
   - When I need to communicate with parties
   - Then I can add mediation notes visible to all parties
   - And I can schedule mediation meetings
   - And parties are notified of updates

3. **AC-3: Resolution Recording**
   - Given a dispute has been resolved
   - When I record the resolution
   - Then I can select resolution type (agreement, favor filer, favor respondent, withdrawn)
   - And I can add resolution details and terms
   - And both parties are notified of the resolution

4. **AC-4: Escalation Path**
   - Given mediation is unsuccessful
   - When I escalate the dispute
   - Then the dispute status changes to "escalated"
   - And escalation reason is recorded
   - And appropriate parties are notified

5. **AC-5: Dispute Timeline**
   - Given I am viewing a dispute
   - When I check the timeline
   - Then all events are shown chronologically
   - And each event shows actor, action, and timestamp
   - And I can filter timeline by event type

## Tasks / Subtasks

- [ ] Task 1: Create Dispute Management Panel (AC: 1, 3, 4)
  - [ ] 1.1 Create `/frontend/apps/ppt-web/src/features/disputes/components/DisputeManagementPanel.tsx`
  - [ ] 1.2 Add mediator assignment dropdown
  - [ ] 1.3 Add status change buttons (based on current status)
  - [ ] 1.4 Add resolution recording form
  - [ ] 1.5 Add escalation dialog with reason

- [ ] Task 2: Implement Mutation Hooks (AC: 1, 3, 4)
  - [ ] 2.1 Add `useAssignMediator` mutation
  - [ ] 2.2 Add `useResolveDispute` mutation
  - [ ] 2.3 Add `useEscalateDispute` mutation
  - [ ] 2.4 Add `useUpdateDisputeStatus` mutation
  - [ ] 2.5 Implement optimistic updates for all mutations

- [ ] Task 3: Create Mediation Notes Component (AC: 2)
  - [ ] 3.1 Create `/frontend/apps/ppt-web/src/features/disputes/components/MediationNotes.tsx`
  - [ ] 3.2 Add note creation form
  - [ ] 3.3 Display notes chronologically
  - [ ] 3.4 Add `useAddMediationNote` mutation
  - [ ] 3.5 Support rich text formatting

- [ ] Task 4: Create Dispute Timeline Component (AC: 5)
  - [ ] 4.1 Create `/frontend/apps/ppt-web/src/features/disputes/components/DisputeTimeline.tsx`
  - [ ] 4.2 Fetch timeline from `/api/v1/disputes/{id}/timeline`
  - [ ] 4.3 Display events with icons by type
  - [ ] 4.4 Add event type filter
  - [ ] 4.5 Support infinite scroll for long timelines

- [ ] Task 5: Create Resolution Dialog (AC: 3)
  - [ ] 5.1 Create `/frontend/apps/ppt-web/src/features/disputes/components/ResolutionDialog.tsx`
  - [ ] 5.2 Resolution type selector
  - [ ] 5.3 Resolution details textarea
  - [ ] 5.4 Terms and conditions fields
  - [ ] 5.5 Confirmation before submission

- [ ] Task 6: Update Dispute Detail Page (AC: 1, 2, 3, 4, 5)
  - [ ] 6.1 Add management panel for managers/mediators
  - [ ] 6.2 Add timeline section
  - [ ] 6.3 Add mediation notes section
  - [ ] 6.4 Show resolution details when resolved
  - [ ] 6.5 Implement role-based visibility

## Dev Notes

### Architecture Requirements
- Role-based access control for management actions
- Real-time updates via WebSocket for dispute changes
- Audit trail for all dispute actions
- Notification integration for all status changes

### Technical Specifications
- Only managers and assigned mediators can take management actions
- Timeline events are immutable and append-only
- Resolution requires confirmation from both parties (optional)

### Resolution Types
```typescript
type ResolutionType =
  | 'mutual_agreement'  // Both parties agreed to terms
  | 'favor_filer'       // Decided in favor of filer
  | 'favor_respondent'  // Decided in favor of respondent
  | 'withdrawn'         // Filer withdrew the dispute
  | 'dismissed';        // Dispute dismissed by mediator

interface ResolveDisputeRequest {
  resolutionType: ResolutionType;
  resolutionDetails: string;
  terms?: string;
  requiresConfirmation: boolean;
}
```

### Timeline Event Types
```typescript
type TimelineEventType =
  | 'dispute_filed'
  | 'status_changed'
  | 'mediator_assigned'
  | 'evidence_added'
  | 'note_added'
  | 'meeting_scheduled'
  | 'resolution_proposed'
  | 'resolution_accepted'
  | 'escalated'
  | 'closed';

interface TimelineEvent {
  id: string;
  disputeId: string;
  eventType: TimelineEventType;
  actorId: string;
  actorName: string;
  description: string;
  metadata?: Record<string, unknown>;
  createdAt: string;
}
```

### API Endpoints
- PUT `/api/v1/disputes/{id}/assign` - Assign mediator
- PUT `/api/v1/disputes/{id}/status` - Update status
- POST `/api/v1/disputes/{id}/resolve` - Record resolution
- POST `/api/v1/disputes/{id}/escalate` - Escalate dispute
- POST `/api/v1/disputes/{id}/notes` - Add mediation note
- GET `/api/v1/disputes/{id}/timeline` - Get timeline

### File List (to create/modify)

**Create:**
- `/frontend/apps/ppt-web/src/features/disputes/components/DisputeManagementPanel.tsx`
- `/frontend/apps/ppt-web/src/features/disputes/components/MediationNotes.tsx`
- `/frontend/apps/ppt-web/src/features/disputes/components/DisputeTimeline.tsx`
- `/frontend/apps/ppt-web/src/features/disputes/components/ResolutionDialog.tsx`
- `/frontend/apps/ppt-web/src/features/disputes/components/EscalationDialog.tsx`
- `/frontend/apps/ppt-web/src/features/disputes/pages/DisputeDetailPage.tsx`

**Modify:**
- `/frontend/packages/api-client/src/disputes/hooks.ts` - Add mutations
- `/frontend/packages/api-client/src/disputes/api.ts` - Add endpoints
- `/frontend/packages/api-client/src/disputes/types.ts` - Add types
- `/frontend/apps/ppt-web/src/App.tsx` - Add dispute detail route

### Role-Based Access
```typescript
const canManageDispute = (dispute: Dispute, user: User) => {
  return (
    user.role === 'manager' ||
    user.role === 'admin' ||
    dispute.assignedMediator === user.id
  );
};
```

### Dependencies
- Story 80.1 (Disputes API Integration) - API client base
- Story 80.2 (Dispute Filing Flow) - Filing functionality
- Story 79.4 (WebSocket Real-time) - Real-time updates

### References
- [Backend: backend/servers/api-server/src/routes/disputes.rs]
- [UC-33.2: Mediate Dispute]
- [UC-33.3: Resolve Dispute]
