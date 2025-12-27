/**
 * Disputes feature barrel export (Epic 77: Dispute Resolution).
 */
export { DisputesPage, CreateDisputePage, DisputeDetailPage } from './pages';
export type {
  DisputeCategory,
  DisputeStatus,
  DisputePriority,
  MessageType,
  DisputeSummary,
  CreateDisputeRequest,
} from './types';
export { CATEGORY_LABELS, STATUS_LABELS, PRIORITY_LABELS } from './types';
