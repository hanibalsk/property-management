/**
<<<<<<< HEAD
 * Document API client (Epic 39).
=======
 * Document Intelligence API (Epic 39).
 *
 * API functions for document search, OCR, classification, and summarization.
>>>>>>> 09dd25d (feat(api-client): add documents module with types and hooks for Epic 39)
 */

import type {
  ClassificationFeedback,
<<<<<<< HEAD
  ClassificationHistoryEntry,
  ClassificationResponse,
  CreateDocumentRequest,
  Document,
  DocumentIntelligenceStats,
  DocumentListQuery,
  DocumentListResponse,
  DocumentSearchRequest,
  DocumentSearchResponse,
  FolderTreeNode,
  FolderWithCount,
  GenerateSummaryRequest,
  OcrReprocessResponse,
  SummarizationResponse,
  UpdateDocumentRequest,
} from './types';

const API_BASE = '/api/v1/documents';

async function fetchApi<T>(url: string, options: RequestInit = {}): Promise<T> {
  const response = await fetch(url, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options.headers,
    },
  });

  if (!response.ok) {
    const error = await response.json().catch(() => ({ message: 'Request failed' }));
    throw new Error(error.message || `HTTP ${response.status}`);
  }

  return response.json();
}

// Document CRUD
export async function createDocument(
  data: CreateDocumentRequest
): Promise<{ id: string; message: string }> {
  return fetchApi(`${API_BASE}`, {
    method: 'POST',
    body: JSON.stringify(data),
  });
}

export async function listDocuments(query?: DocumentListQuery): Promise<DocumentListResponse> {
  const params = new URLSearchParams();
  if (query?.folder_id) params.set('folder_id', query.folder_id);
  if (query?.category) params.set('category', query.category);
  if (query?.search) params.set('search', query.search);
  if (query?.limit) params.set('limit', query.limit.toString());
  if (query?.offset) params.set('offset', query.offset.toString());

  const queryString = params.toString();
  return fetchApi(`${API_BASE}${queryString ? `?${queryString}` : ''}`);
}

export async function getDocument(id: string): Promise<{ document: Document }> {
  return fetchApi(`${API_BASE}/${id}`);
}

export async function updateDocument(
  id: string,
  data: UpdateDocumentRequest
): Promise<{ message: string; document: Document }> {
  return fetchApi(`${API_BASE}/${id}`, {
    method: 'PUT',
    body: JSON.stringify(data),
  });
}

export async function deleteDocument(id: string): Promise<void> {
  await fetchApi(`${API_BASE}/${id}`, { method: 'DELETE' });
}

// Download/Preview
export async function getDownloadUrl(id: string): Promise<{ url: string; expires_at: string }> {
  return fetchApi(`${API_BASE}/${id}/download`);
}

export async function getPreviewUrl(id: string): Promise<{ url: string; expires_at: string }> {
  return fetchApi(`${API_BASE}/${id}/preview`);
}

// Folders
export async function listFolders(buildingId?: string): Promise<{ folders: FolderWithCount[] }> {
  const params = buildingId ? `?building_id=${buildingId}` : '';
  return fetchApi(`${API_BASE}/folders${params}`);
}

export async function getFolderTree(buildingId?: string): Promise<{ tree: FolderTreeNode[] }> {
  const params = buildingId ? `?building_id=${buildingId}` : '';
  return fetchApi(`${API_BASE}/folders/tree${params}`);
}

// Document Intelligence (Epic 28)

// Story 28.1: OCR
export async function reprocessOcr(id: string): Promise<OcrReprocessResponse> {
  return fetchApi(`${API_BASE}/${id}/ocr/reprocess`, { method: 'POST' });
}

// Story 28.2: Full-text search
export async function searchDocuments(
  request: DocumentSearchRequest
): Promise<DocumentSearchResponse> {
  return fetchApi(`${API_BASE}/search`, {
    method: 'POST',
    body: JSON.stringify(request),
  });
}

// Story 28.3: Classification
export async function getClassification(id: string): Promise<ClassificationResponse> {
  return fetchApi(`${API_BASE}/${id}/classification`);
}

export async function submitClassificationFeedback(
  id: string,
  feedback: ClassificationFeedback
): Promise<{ message: string }> {
  return fetchApi(`${API_BASE}/${id}/classification/feedback`, {
    method: 'POST',
    body: JSON.stringify(feedback),
  });
}

export async function getClassificationHistory(
  id: string
): Promise<{ history: ClassificationHistoryEntry[] }> {
  return fetchApi(`${API_BASE}/${id}/classification/history`);
}

// Story 28.4: Summarization
export async function requestSummarization(
  id: string,
  options?: GenerateSummaryRequest
): Promise<SummarizationResponse> {
  return fetchApi(`${API_BASE}/${id}/summarize`, {
    method: 'POST',
    body: JSON.stringify(options || {}),
  });
}

// Intelligence stats
export async function getIntelligenceStats(): Promise<{
  stats: DocumentIntelligenceStats[];
}> {
  return fetchApi(`${API_BASE}/intelligence/stats`);
=======
  ClassificationResponse,
  DocumentResponse,
  DocumentSearchRequest,
  DocumentSearchResponse,
  SummarizationOptions,
} from './types';

const API_BASE = '/api/v1';

/**
 * Fetch a single document by ID.
 */
export async function fetchDocument(id: string): Promise<DocumentResponse> {
  const response = await fetch(`${API_BASE}/documents/${id}`, {
    credentials: 'include',
  });
  if (!response.ok) {
    throw new Error(`Failed to fetch document: ${response.statusText}`);
  }
  return response.json();
}

/**
 * Search documents with full-text search.
 */
export async function searchDocuments(
  request: DocumentSearchRequest
): Promise<DocumentSearchResponse> {
  const response = await fetch(`${API_BASE}/documents/search`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    body: JSON.stringify(request),
  });
  if (!response.ok) {
    throw new Error(`Search failed: ${response.statusText}`);
  }
  return response.json();
}

/**
 * Get document classification.
 */
export async function getDocumentClassification(
  documentId: string
): Promise<ClassificationResponse> {
  const response = await fetch(`${API_BASE}/documents/${documentId}/classification`, {
    credentials: 'include',
  });
  if (!response.ok) {
    throw new Error(`Failed to get classification: ${response.statusText}`);
  }
  return response.json();
}

/**
 * Submit classification feedback.
 */
export async function submitClassificationFeedback(
  documentId: string,
  feedback: ClassificationFeedback
): Promise<ClassificationResponse> {
  const response = await fetch(`${API_BASE}/documents/${documentId}/classification/feedback`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    body: JSON.stringify(feedback),
  });
  if (!response.ok) {
    throw new Error(`Failed to submit feedback: ${response.statusText}`);
  }
  return response.json();
}

/**
 * Request document summarization.
 */
export async function requestSummarization(
  documentId: string,
  options: SummarizationOptions
): Promise<void> {
  const response = await fetch(`${API_BASE}/documents/${documentId}/summarize`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    body: JSON.stringify(options),
  });
  if (!response.ok) {
    throw new Error(`Failed to request summarization: ${response.statusText}`);
  }
}

/**
 * Reprocess OCR for a document.
 */
export async function reprocessOcr(documentId: string): Promise<void> {
  const response = await fetch(`${API_BASE}/documents/${documentId}/ocr/reprocess`, {
    method: 'POST',
    credentials: 'include',
  });
  if (!response.ok) {
    throw new Error(`Failed to reprocess OCR: ${response.statusText}`);
  }
>>>>>>> 09dd25d (feat(api-client): add documents module with types and hooks for Epic 39)
}
