# Story 80.2: Dispute Filing Flow

Status: pending

## Story

As a **tenant or property owner**,
I want to **file a new dispute with supporting evidence**,
So that **I can formally report issues and seek resolution**.

## Acceptance Criteria

1. **AC-1: Dispute Filing Form**
   - Given I want to file a dispute
   - When I access the file dispute page
   - Then I see a form with all required fields (type, subject, description, respondent)
   - And I can select the dispute type from available options
   - And I can identify the other party (respondent)

2. **AC-2: Evidence Upload**
   - Given I am filing a dispute
   - When I want to add supporting evidence
   - Then I can upload files (photos, documents, videos)
   - And I can add descriptions to each piece of evidence
   - And upload progress is shown

3. **AC-3: Dispute Submission**
   - Given I have filled out the dispute form
   - When I submit the dispute
   - Then it is created via `/api/v1/disputes` using useMutation
   - And I am redirected to the dispute detail page
   - And a confirmation notification is shown

4. **AC-4: Draft Saving**
   - Given I am in the middle of filing a dispute
   - When I navigate away or close the browser
   - Then my draft is saved locally
   - And I can resume filing when I return

5. **AC-5: Validation and Error Handling**
   - Given I submit incomplete or invalid data
   - When the form is validated
   - Then inline validation errors are shown
   - And the form prevents submission until fixed

## Tasks / Subtasks

- [ ] Task 1: Create Dispute Filing Page (AC: 1, 5)
  - [ ] 1.1 Create `/frontend/apps/ppt-web/src/features/disputes/pages/FileDisputePage.tsx`
  - [ ] 1.2 Create dispute form with all required fields
  - [ ] 1.3 Add type selector with all dispute types
  - [ ] 1.4 Add respondent selection (unit/resident picker)
  - [ ] 1.5 Implement form validation with error messages

- [ ] Task 2: Create Evidence Upload Component (AC: 2)
  - [ ] 2.1 Create `/frontend/apps/ppt-web/src/features/disputes/components/EvidenceUploader.tsx`
  - [ ] 2.2 Support multiple file types (images, PDFs, videos)
  - [ ] 2.3 Add drag-and-drop upload functionality
  - [ ] 2.4 Show upload progress for each file
  - [ ] 2.5 Allow adding description to each evidence item
  - [ ] 2.6 Preview uploaded evidence

- [ ] Task 3: Implement useCreateDispute Mutation (AC: 3)
  - [ ] 3.1 Add `useCreateDispute` mutation hook
  - [ ] 3.2 Handle file uploads to `/api/v1/disputes/{id}/evidence`
  - [ ] 3.3 Implement two-step: create dispute, then upload evidence
  - [ ] 3.4 Add error handling and rollback

- [ ] Task 4: Implement Draft Saving (AC: 4)
  - [ ] 4.1 Create `/frontend/apps/ppt-web/src/hooks/useDraftStorage.ts`
  - [ ] 4.2 Save form state to localStorage on change (debounced)
  - [ ] 4.3 Restore draft on page load
  - [ ] 4.4 Clear draft on successful submission
  - [ ] 4.5 Add "Resume draft" prompt if draft exists

- [ ] Task 5: Add Route and Navigation (AC: 1, 3)
  - [ ] 5.1 Add `/disputes/file` route to router
  - [ ] 5.2 Add "File Dispute" button to DisputesPage
  - [ ] 5.3 Add navigation to dispute detail after creation
  - [ ] 5.4 Add breadcrumb navigation

## Dev Notes

### Architecture Requirements
- Multi-step form for better UX on mobile
- Evidence upload with progress tracking
- Draft auto-save to prevent data loss
- Optimistic UI updates on submission

### Technical Specifications
- Evidence file size limit: 10MB per file
- Supported types: image/*, application/pdf, video/* (max 100MB)
- Max evidence items: 10 per dispute
- Draft storage key: `dispute_draft_{userId}`

### Create Dispute Request
```typescript
interface CreateDisputeRequest {
  type: DisputeType;
  subject: string;
  description: string;
  unitId: string;
  respondentId?: string;
}

interface CreateDisputeResponse {
  id: string;
  createdAt: string;
}

interface UploadEvidenceRequest {
  file: File;
  description: string;
}
```

### Evidence Upload Flow
```typescript
// 1. Create dispute
const dispute = await createDispute(disputeData);

// 2. Upload each evidence file
for (const evidence of evidenceFiles) {
  const formData = new FormData();
  formData.append('file', evidence.file);
  formData.append('description', evidence.description);
  await uploadEvidence(dispute.id, formData);
}

// 3. Navigate to dispute detail
navigate(`/disputes/${dispute.id}`);
```

### File List (to create/modify)

**Create:**
- `/frontend/apps/ppt-web/src/features/disputes/pages/FileDisputePage.tsx`
- `/frontend/apps/ppt-web/src/features/disputes/components/EvidenceUploader.tsx`
- `/frontend/apps/ppt-web/src/features/disputes/components/DisputeForm.tsx`
- `/frontend/apps/ppt-web/src/hooks/useDraftStorage.ts`

**Modify:**
- `/frontend/packages/api-client/src/disputes/hooks.ts` - Add mutations
- `/frontend/packages/api-client/src/disputes/api.ts` - Add create/upload endpoints
- `/frontend/apps/ppt-web/src/App.tsx` - Add route
- `/frontend/apps/ppt-web/src/features/disputes/pages/DisputesPage.tsx` - Add file button

### Form Validation Rules
```typescript
const disputeSchema = z.object({
  type: z.enum(['noise', 'damage', 'payment', 'lease', 'maintenance', 'other']),
  subject: z.string().min(10, 'Subject must be at least 10 characters').max(100),
  description: z.string().min(50, 'Description must be at least 50 characters').max(5000),
  unitId: z.string().uuid('Invalid unit'),
  respondentId: z.string().uuid().optional(),
});
```

### Dependencies
- Story 80.1 (Disputes API Integration) - API client module
- Story 79.3 (Error Handling and Toasts) - Toast notifications

### References
- [Backend: backend/servers/api-server/src/routes/disputes.rs]
- [UC-33.1: File Dispute]
