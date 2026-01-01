/**
 * Migration Import Types (Epic 90 - Frontend API Integration).
 * Originally from Epic 66, integrated as part of Epic 90.
 *
 * Type definitions for bulk data import functionality.
 */

// Import job status
export type ImportJobStatus =
  | 'pending'
  | 'validating'
  | 'validated'
  | 'validation_failed'
  | 'importing'
  | 'completed'
  | 'partially_completed'
  | 'failed'
  | 'cancelled';

// Data types that can be imported
export type ImportDataType =
  | 'buildings'
  | 'units'
  | 'residents'
  | 'financials'
  | 'faults'
  | 'documents'
  | 'meters'
  | 'votes'
  | 'custom';

// Issue severity levels
export type IssueSeverity = 'error' | 'warning' | 'info';

// Import template summary
export interface ImportTemplateSummary {
  id: string;
  name: string;
  dataType: ImportDataType;
  description?: string;
  isSystemTemplate: boolean;
  fieldCount: number;
  updatedAt: string;
}

// Import template with field mappings
export interface ImportTemplate {
  id: string;
  name: string;
  dataType: ImportDataType;
  description?: string;
  isSystemTemplate: boolean;
  fields: ImportFieldMapping[];
  createdAt: string;
  updatedAt: string;
}

// Field mapping in a template
export interface ImportFieldMapping {
  sourceColumn: string;
  targetField: string;
  isRequired: boolean;
  defaultValue?: string;
  transformRule?: string;
}

// Import job
export interface ImportJob {
  id: string;
  templateId: string;
  templateName: string;
  dataType: ImportDataType;
  status: ImportJobStatus;
  filename: string;
  totalRows: number;
  processedRows: number;
  importedRows: number;
  failedRows: number;
  createdById: string;
  createdByName: string;
  createdAt: string;
  startedAt?: string;
  completedAt?: string;
  errorMessage?: string;
}

// Import job history item (for list display)
export interface ImportJobHistoryItem {
  id: string;
  status: ImportJobStatus;
  filename: string;
  dataType: ImportDataType;
  recordsImported: number;
  recordsFailed: number;
  createdByName: string;
  createdAt: string;
  completedAt?: string;
}

// Import preview data (validation result)
export interface ImportPreviewData {
  jobId: string;
  isValid: boolean;
  totalRows: number;
  importableRows: number;
  errorRows: number;
  warningRows: number;
  recordCounts: {
    newRecords: number;
    updates: number;
    skipped: number;
  };
  issues: ImportIssue[];
  totalIssueCount: number;
  sampleRecords: Record<string, unknown>[];
  columnMapping: ColumnMappingPreview[];
}

// Import issue (validation error/warning)
export interface ImportIssue {
  rowNumber: number;
  column: string;
  severity: IssueSeverity;
  code: string;
  message: string;
  originalValue?: string;
  suggestedValue?: string;
}

// Column mapping preview
export interface ColumnMappingPreview {
  sourceColumn: string;
  targetField: string;
  isMapped: boolean;
  isRequired: boolean;
  sampleValues: string[];
}

// Import job progress
export interface ImportJobProgress {
  jobId: string;
  status: ImportJobStatus;
  progress: number;
  processedRows: number;
  totalRows: number;
  importedRows: number;
  failedRows: number;
  currentStep?: string;
  estimatedTimeRemaining?: number;
}

// Request types
export interface UploadImportFileRequest {
  templateId: string;
  file: File;
}

export interface StartImportRequest {
  jobId: string;
  acknowledgeWarnings?: boolean;
}

export interface RetryImportRequest {
  jobId: string;
  skipFailedRows?: boolean;
}

// Response types
export interface ImportTemplateListResponse {
  templates: ImportTemplateSummary[];
  total: number;
}

export interface ImportTemplateResponse {
  template: ImportTemplate;
}

export interface ImportJobListResponse {
  jobs: ImportJobHistoryItem[];
  total: number;
}

export interface ImportJobResponse {
  job: ImportJob;
}

export interface ImportPreviewResponse {
  preview: ImportPreviewData;
}

export interface ImportProgressResponse {
  progress: ImportJobProgress;
}

export interface TemplateDownloadResponse {
  url: string;
  filename: string;
  expiresAt: string;
}
