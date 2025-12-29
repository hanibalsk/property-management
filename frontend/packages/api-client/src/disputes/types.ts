/**
 * Disputes Types
 *
 * TypeScript types for Disputes API (Epic 80).
 */

// ============================================
// Core Types (Story 80.1)
// ============================================

export type DisputeType = 'noise' | 'damage' | 'payment' | 'lease' | 'maintenance' | 'other';

export type DisputeStatus =
  | 'filed'
  | 'under_review'
  | 'mediation'
  | 'escalated'
  | 'resolved'
  | 'closed';

export interface Dispute {
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
  respondentId?: string;
  assignedMediator?: string;
  assignedMediatorId?: string;
  resolutionDeadline?: string;
  resolution?: string;
  resolvedAt?: string;
  createdAt: string;
  updatedAt: string;
}

export interface DisputeFilters {
  status?: DisputeStatus;
  type?: DisputeType;
  dateFrom?: string;
  dateTo?: string;
  unitId?: string;
  filedBy?: string;
  assignedMediator?: string;
}

export interface DisputeListQuery extends DisputeFilters {
  page?: number;
  limit?: number;
  sortBy?: string;
  sortOrder?: 'asc' | 'desc';
}

export interface PaginatedDisputes {
  data: Dispute[];
  total: number;
  page: number;
  limit: number;
  totalPages: number;
}

// ============================================
// Dispute Filing Types (Story 80.2)
// ============================================

export interface CreateDisputeRequest {
  type: DisputeType;
  subject: string;
  description: string;
  unitId: string;
  respondentId?: string;
}

export interface CreateDisputeResponse {
  id: string;
  createdAt: string;
}

export interface UploadEvidenceRequest {
  file: File;
  description: string;
}

export interface DisputeEvidence {
  id: string;
  disputeId: string;
  fileName: string;
  fileType: string;
  fileSize: number;
  fileUrl: string;
  description?: string;
  uploadedBy: string;
  uploadedAt: string;
  createdAt: string;
}

export interface EvidenceUploadProgress {
  fileId: string;
  fileName: string;
  progress: number;
  status: 'pending' | 'uploading' | 'completed' | 'error';
  error?: string;
}

// ============================================
// Mediation & Resolution Types (Story 80.3)
// ============================================

export type ResolutionType =
  | 'mutual_agreement'
  | 'favor_filer'
  | 'favor_respondent'
  | 'withdrawn'
  | 'dismissed';

export interface ResolveDisputeRequest {
  resolutionType: ResolutionType;
  resolutionDetails: string;
  terms?: string;
  requiresConfirmation: boolean;
}

export interface AssignMediatorRequest {
  mediatorId: string;
}

export interface EscalateDisputeRequest {
  reason: string;
  escalateTo?: string;
}

export interface AddMediationNoteRequest {
  content: string;
  isPrivate?: boolean;
}

export interface MediationNote {
  id: string;
  disputeId: string;
  content: string;
  isPrivate: boolean;
  authorId: string;
  authorName: string;
  createdAt: string;
  updatedAt: string;
}

export interface UpdateDisputeStatusRequest {
  status: DisputeStatus;
  reason?: string;
}

// ============================================
// Timeline Types (Story 80.3)
// ============================================

export type TimelineEventType =
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

export interface TimelineEvent {
  id: string;
  disputeId: string;
  eventType: TimelineEventType;
  actorId: string;
  actorName: string;
  description: string;
  metadata?: Record<string, unknown>;
  createdAt: string;
}

export interface TimelineQuery {
  eventType?: TimelineEventType;
  limit?: number;
  offset?: number;
}

// ============================================
// Dispute Detail Types
// ============================================

export interface DisputeWithDetails extends Dispute {
  evidence: DisputeEvidence[];
  timeline: TimelineEvent[];
  mediationNotes: MediationNote[];
  filerDetails?: PartyDetails;
  respondentDetails?: PartyDetails;
  mediatorDetails?: PartyDetails;
}

export interface PartyDetails {
  id: string;
  name: string;
  email?: string;
  phone?: string;
  unitNumber?: string;
}

// ============================================
// Statistics Types
// ============================================

export interface DisputeStatistics {
  totalDisputes: number;
  disputesByStatus: Record<DisputeStatus, number>;
  disputesByType: Record<DisputeType, number>;
  averageResolutionDays: number;
  resolutionRate: number;
}
