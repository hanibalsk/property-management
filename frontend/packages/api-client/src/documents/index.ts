/**
<<<<<<< HEAD
 * Documents API module (Epic 39).
 */

export * from './types';
export * from './api';
export * from './hooks';
=======
 * Document Intelligence Module (Epic 39).
 *
 * Exports for document search, OCR, classification, and summarization.
 */

// Types
export type {
  ClassificationFeedback,
  ClassificationResponse,
  Document,
  DocumentCategory,
  DocumentResponse,
  DocumentSearchRequest,
  DocumentSearchResponse,
  DocumentSearchResult,
  OcrStatus,
  SearchHighlight,
  SummarizationOptions,
} from './types';

// Constants
export { DOCUMENT_CATEGORIES } from './types';

// API functions
export {
  fetchDocument,
  getDocumentClassification,
  reprocessOcr,
  requestSummarization,
  searchDocuments,
  submitClassificationFeedback,
} from './api';

// Hooks
export {
  documentKeys,
  useDocument,
  useDocumentClassification,
  useDocumentSearch,
  useReprocessOcr,
  useRequestSummarization,
  useSubmitClassificationFeedback,
} from './hooks';
>>>>>>> 09dd25d (feat(api-client): add documents module with types and hooks for Epic 39)
