/**
<<<<<<< HEAD
 * Document Intelligence types (Epic 39).
 */

// Base types
=======
 * Document Intelligence Types (Epic 39).
 *
 * Types for document search, OCR, classification, and summarization.
 */

/**
 * OCR processing status values.
 */
export type OcrStatus = 'pending' | 'processing' | 'completed' | 'failed' | 'not_applicable';

/**
 * Document categories for classification.
 */
export const DOCUMENT_CATEGORIES = [
  'Contract',
  'Invoice',
  'Meeting Minutes',
  'Maintenance Report',
  'Insurance',
  'Legal',
  'Financial',
  'Technical',
  'Correspondence',
  'Other',
] as const;

export type DocumentCategory = (typeof DOCUMENT_CATEGORIES)[number];

/**
 * Document entity with intelligence fields.
 */
>>>>>>> 09dd25d (feat(api-client): add documents module with types and hooks for Epic 39)
export interface Document {
  id: string;
  title: string;
  description?: string;
<<<<<<< HEAD
  category: string;
  folder_id?: string;
  file_key: string;
  file_name: string;
  mime_type: string;
  size_bytes: number;
  access_scope: AccessScope;
  created_at: string;
  updated_at: string;
  created_by: string;
  organization_id: string;
  building_id?: string;
  // Intelligence fields (Epic 28)
  ocr_status?: OcrStatus;
  ocr_text?: string;
  ocr_processed_at?: string;
  predicted_category?: string;
  classification_confidence?: number;
  classified_at?: string;
  classification_accepted?: boolean;
=======
  file_name: string;
  category: string;
  size_bytes: number;
  created_at: string;
  updated_at: string;
  // OCR fields
  ocr_status?: OcrStatus;
  ocr_text?: string;
  ocr_processed_at?: string;
  // Classification fields
  predicted_category?: string;
  classification_confidence?: number;
  // Summary fields
>>>>>>> 09dd25d (feat(api-client): add documents module with types and hooks for Epic 39)
  summary?: string;
  summary_generated_at?: string;
}

<<<<<<< HEAD
export type AccessScope = 'organization' | 'building' | 'unit' | 'user' | 'public';
export type OcrStatus = 'pending' | 'processing' | 'completed' | 'failed' | 'not_applicable';

export interface DocumentSummary {
  id: string;
  title: string;
  description?: string;
  category: string;
  file_name: string;
  mime_type: string;
  size_bytes: number;
  created_at: string;
  updated_at: string;
  ocr_status?: OcrStatus;
  predicted_category?: string;
  classification_confidence?: number;
  summary?: string;
}

export interface DocumentFolder {
  id: string;
  name: string;
  description?: string;
  parent_id?: string;
  organization_id: string;
  building_id?: string;
  created_at: string;
  updated_at: string;
}

export interface FolderWithCount extends DocumentFolder {
  document_count: number;
  subfolder_count: number;
}

export interface FolderTreeNode {
  folder: DocumentFolder;
  children: FolderTreeNode[];
  document_count: number;
}

// Search types (Story 28.2)
export interface DocumentSearchRequest {
  query: string;
  organization_id?: string;
  building_id?: string;
  categories?: string[];
  date_from?: string;
  date_to?: string;
  ocr_status?: OcrStatus[];
  has_summary?: boolean;
=======
/**
 * AI classification response.
 */
export interface ClassificationResponse {
  document_id: string;
  predicted_category?: string;
  confidence?: number;
  classified_at?: string;
  accepted?: boolean | null;
  user_feedback_category?: string;
}

/**
 * Classification feedback payload.
 */
export interface ClassificationFeedback {
  accepted: boolean;
  correct_category?: string;
}

/**
 * Document search request.
 */
export interface DocumentSearchRequest {
  query: string;
  organization_id: string;
  building_id?: string;
  categories?: string[];
  ocr_status?: OcrStatus[];
  has_summary?: boolean;
  date_from?: string;
  date_to?: string;
>>>>>>> 09dd25d (feat(api-client): add documents module with types and hooks for Epic 39)
  limit?: number;
  offset?: number;
}

<<<<<<< HEAD
export interface DocumentSearchResult {
  document: DocumentSummary;
  score: number;
  highlights: SearchHighlight[];
}

=======
/**
 * Search highlight for matched text.
 */
>>>>>>> 09dd25d (feat(api-client): add documents module with types and hooks for Epic 39)
export interface SearchHighlight {
  field: 'title' | 'description' | 'ocr_text' | 'summary';
  snippet: string;
}

<<<<<<< HEAD
=======
/**
 * Individual search result.
 */
export interface DocumentSearchResult {
  document: Document;
  score: number;
  highlights: SearchHighlight[];
}

/**
 * Search response with results and metadata.
 */
>>>>>>> 09dd25d (feat(api-client): add documents module with types and hooks for Epic 39)
export interface DocumentSearchResponse {
  results: DocumentSearchResult[];
  total: number;
  took_ms: number;
}

<<<<<<< HEAD
// Classification types (Story 28.3)
export interface ClassificationResponse {
  document_id: string;
  predicted_category?: string;
  confidence?: number;
  classified_at?: string;
  accepted?: boolean;
}

export interface ClassificationFeedback {
  accepted: boolean;
  correct_category?: string;
}

export interface ClassificationHistoryEntry {
  id: string;
  predicted_category: string;
  confidence: number;
  classified_at: string;
  feedback_accepted?: boolean;
  feedback_correct_category?: string;
  feedback_at?: string;
}

// Summarization types (Story 28.4)
export interface GenerateSummaryRequest {
  max_length?: number;
  style?: 'brief' | 'detailed' | 'bullets';
}

export interface SummarizationResponse {
  message: string;
  queue_id: string;
}

export interface DocumentSummaryView {
  document_id: string;
  summary: string;
  key_points?: string[];
  generated_at: string;
}

// OCR types (Story 28.1)
export interface OcrReprocessResponse {
  message: string;
  queue_id?: string;
}

// Intelligence stats
export interface DocumentIntelligenceStats {
  organization_id: string;
  total_documents: number;
  ocr_completed: number;
  ocr_pending: number;
  ocr_failed: number;
  classified_documents: number;
  summarized_documents: number;
  avg_classification_confidence: number;
}

// List query params
export interface DocumentListQuery {
  folder_id?: string;
  category?: string;
  search?: string;
  limit?: number;
  offset?: number;
}

// Responses
export interface DocumentListResponse {
  documents: DocumentSummary[];
  count: number;
  total: number;
}

export interface CreateDocumentRequest {
  title: string;
  description?: string;
  category: string;
  folder_id?: string;
  file_key: string;
  file_name: string;
  mime_type: string;
  size_bytes: number;
  access_scope?: AccessScope;
  access_target_ids?: string[];
  access_roles?: string[];
}

export interface UpdateDocumentRequest {
  title?: string;
  description?: string;
  category?: string;
  folder_id?: string;
  access_scope?: AccessScope;
  access_target_ids?: string[];
  access_roles?: string[];
}

// Document categories
export const DOCUMENT_CATEGORIES = [
  'contract',
  'invoice',
  'receipt',
  'report',
  'minutes',
  'policy',
  'notice',
  'maintenance',
  'insurance',
  'legal',
  'financial',
  'correspondence',
  'other',
] as const;

export type DocumentCategory = (typeof DOCUMENT_CATEGORIES)[number];
=======
/**
 * Summarization options.
 */
export interface SummarizationOptions {
  style: 'brief' | 'detailed' | 'bullets';
}

/**
 * Document response wrapper.
 */
export interface DocumentResponse {
  document: Document;
}
>>>>>>> 09dd25d (feat(api-client): add documents module with types and hooks for Epic 39)
