/**
 * Fault API module exports.
 */

// Types
export type {
  AddCommentRequest,
  AddWorkNoteRequest,
  AiSuggestion,
  AiSuggestionResponse,
  CreateFaultRequest,
  CreateFaultResponse,
  FaultAttachment,
  FaultCategory,
  FaultComment,
  FaultDetailResponse,
  FaultListQuery,
  FaultListResponse,
  FaultPriority,
  FaultStatistics,
  FaultStatus,
  FaultSummary,
  FaultTimelineEntry,
  FaultWithDetails,
  ResolveFaultRequest,
  TriageFaultRequest,
  UpdateFaultRequest,
  WorkNote,
} from './types';

// API functions
export {
  acceptAiSuggestion,
  addAttachment,
  addComment,
  addWorkNote,
  assignFault,
  confirmFault,
  createFault,
  deleteFault,
  deleteAttachment,
  getAiSuggestion,
  getFault,
  getFaultStatistics,
  getFaultTimeline,
  listAttachments,
  listFaultComments,
  listFaults,
  reopenFault,
  resolveFault,
  triageFault,
  updateFault,
} from './api';

// React Query hooks
export {
  faultKeys,
  useAcceptAiSuggestion,
  useAddAttachment,
  useAddComment,
  useAddWorkNote,
  useAiSuggestion,
  useAssignFault,
  useConfirmFault,
  useCreateFault,
  useDeleteAttachment,
  useDeleteFault,
  useFault,
  useFaultComments,
  useFaults,
  useFaultStatistics,
  useReopenFault,
  useRequestAiSuggestion,
  useResolveFault,
  useTriageFault,
  useUpdateFault,
} from './hooks';
