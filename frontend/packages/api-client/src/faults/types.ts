/**
 * Fault API types (Epic 4: Fault Reporting & Resolution).
 */

/** Fault categories matching backend enum */
export type FaultCategory =
  | 'plumbing'
  | 'electrical'
  | 'heating'
  | 'structural'
  | 'exterior'
  | 'elevator'
  | 'common_area'
  | 'security'
  | 'cleaning'
  | 'other';

/** Fault priority levels */
export type FaultPriority = 'low' | 'medium' | 'high' | 'urgent';

/** Fault status values */
export type FaultStatus =
  | 'reported'
  | 'triaged'
  | 'in_progress'
  | 'scheduled'
  | 'on_hold'
  | 'resolved'
  | 'closed'
  | 'reopened';

/** Fault summary for list views */
export interface FaultSummary {
  id: string;
  title: string;
  description: string;
  category: FaultCategory;
  priority?: FaultPriority;
  status: FaultStatus;
  building_id: string;
  building_name?: string;
  unit_id?: string;
  unit_designation?: string;
  reporter_id: string;
  reporter_name?: string;
  assigned_to?: string;
  assigned_to_name?: string;
  created_at: string;
  updated_at: string;
  ai_suggested_category?: FaultCategory;
  ai_confidence?: number;
}

/** Full fault details */
export interface FaultWithDetails extends FaultSummary {
  location_description?: string;
  resolution_notes?: string;
  resolved_at?: string;
  resolved_by?: string;
  scheduled_date?: string;
  work_notes?: WorkNote[];
  comments?: FaultComment[];
}

/** Work note on a fault */
export interface WorkNote {
  id: string;
  fault_id: string;
  author_id: string;
  author_name?: string;
  content: string;
  created_at: string;
}

/** Comment on a fault */
export interface FaultComment {
  id: string;
  fault_id: string;
  author_id: string;
  author_name?: string;
  content: string;
  is_internal: boolean;
  created_at: string;
}

/** Fault attachment (photo, document) */
export interface FaultAttachment {
  id: string;
  fault_id: string;
  file_url: string;
  file_name: string;
  file_type: string;
  file_size: number;
  uploaded_by: string;
  uploaded_by_name?: string;
  created_at: string;
}

/** Timeline entry for fault history */
export interface FaultTimelineEntry {
  id: string;
  fault_id: string;
  event_type: string;
  description: string;
  actor_id: string;
  actor_name?: string;
  created_at: string;
  metadata?: Record<string, unknown>;
}

/** AI suggestion response */
export interface AiSuggestion {
  category: FaultCategory;
  confidence: number;
  priority?: FaultPriority;
}

/** Fault statistics */
export interface FaultStatistics {
  total: number;
  by_status: Record<FaultStatus, number>;
  by_category: Record<FaultCategory, number>;
  by_priority: Record<FaultPriority, number>;
  avg_resolution_time_hours?: number;
  ai_accuracy?: number;
}

/** Request to create a fault */
export interface CreateFaultRequest {
  building_id: string;
  unit_id?: string;
  title: string;
  description: string;
  location_description?: string;
  category: FaultCategory;
  priority?: FaultPriority;
  /** Photos to attach (base64 encoded or URLs) */
  photos?: string[];
  /** Idempotency key for duplicate prevention */
  idempotency_key?: string;
}

/** Request to update a fault */
export interface UpdateFaultRequest {
  title?: string;
  description?: string;
  location_description?: string;
  category?: FaultCategory;
}

/** Request to triage a fault */
export interface TriageFaultRequest {
  priority: FaultPriority;
  category?: FaultCategory;
  assigned_to?: string;
}

/** Request to resolve a fault */
export interface ResolveFaultRequest {
  resolution_notes: string;
}

/** Request to add a comment */
export interface AddCommentRequest {
  content: string;
  is_internal?: boolean;
}

/** Request to add a work note */
export interface AddWorkNoteRequest {
  content: string;
}

/** Query parameters for listing faults */
export interface FaultListQuery {
  building_id?: string;
  status?: FaultStatus;
  category?: FaultCategory;
  priority?: FaultPriority;
  assigned_to?: string;
  search?: string;
  page?: number;
  limit?: number;
}

/** Fault list response */
export interface FaultListResponse {
  faults: FaultSummary[];
  count: number;
}

/** Fault detail response */
export interface FaultDetailResponse {
  fault: FaultWithDetails;
  timeline: FaultTimelineEntry[];
  attachments: FaultAttachment[];
}

/** Response for creating a fault */
export interface CreateFaultResponse {
  id: string;
  message: string;
}

/** Response for AI suggestion */
export interface AiSuggestionResponse {
  suggestion: AiSuggestion;
}
