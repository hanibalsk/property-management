/**
 * Disputes types (Epic 77: Dispute Resolution).
 */

export type DisputeCategory =
  | 'deposit'
  | 'maintenance'
  | 'noise'
  | 'fees'
  | 'contract'
  | 'neighbor'
  | 'other';

export type DisputeStatus =
  | 'filed'
  | 'pending_response'
  | 'under_review'
  | 'mediation'
  | 'resolution_proposed'
  | 'agreement_pending'
  | 'resolved'
  | 'escalated'
  | 'closed'
  | 'cancelled';

export type DisputePriority = 'low' | 'medium' | 'high' | 'urgent';

export type MessageType =
  | 'text'
  | 'proposal'
  | 'counter_proposal'
  | 'acceptance'
  | 'rejection'
  | 'session_summary'
  | 'system';

export interface DisputeSummary {
  id: string;
  title: string;
  category: DisputeCategory;
  status: DisputeStatus;
  priority: DisputePriority;
  filedByName: string;
  respondentName: string;
  responseDeadline?: string;
  createdAt: string;
}

export interface CreateDisputeRequest {
  respondentId: string;
  title: string;
  description: string;
  category: DisputeCategory;
  desiredResolution: string;
  priority?: DisputePriority;
}

export const CATEGORY_LABELS: Record<DisputeCategory, string> = {
  deposit: 'Security Deposit',
  maintenance: 'Maintenance Issue',
  noise: 'Noise Complaint',
  fees: 'Fees & Charges',
  contract: 'Contract Terms',
  neighbor: 'Neighbor Dispute',
  other: 'Other',
};

export const STATUS_LABELS: Record<DisputeStatus, string> = {
  filed: 'Filed',
  pending_response: 'Pending Response',
  under_review: 'Under Review',
  mediation: 'In Mediation',
  resolution_proposed: 'Resolution Proposed',
  agreement_pending: 'Agreement Pending',
  resolved: 'Resolved',
  escalated: 'Escalated',
  closed: 'Closed',
  cancelled: 'Cancelled',
};

export const PRIORITY_LABELS: Record<DisputePriority, string> = {
  low: 'Low',
  medium: 'Medium',
  high: 'High',
  urgent: 'Urgent',
};
