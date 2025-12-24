/**
 * Forms Management Types
 *
 * Type definitions for the Forms Management API (Epic 54).
 */

// Form status lifecycle: draft -> published -> archived
export type FormStatus = 'draft' | 'published' | 'archived';

// Submission status: pending -> reviewed -> approved/rejected
export type FormSubmissionStatus = 'pending' | 'reviewed' | 'approved' | 'rejected';

// Form target scope
export type FormTargetType = 'all' | 'building' | 'unit' | 'role';

// Supported field types for form builder
export type FormFieldType =
  | 'text'
  | 'textarea'
  | 'number'
  | 'email'
  | 'phone'
  | 'date'
  | 'datetime'
  | 'checkbox'
  | 'radio'
  | 'select'
  | 'multiselect'
  | 'file'
  | 'signature';

// Export format for form data
export type FormExportFormat = 'csv' | 'xlsx' | 'pdf';

// ============================================================================
// Core Form Types
// ============================================================================

export interface Form {
  id: string;
  organizationId: string;
  title: string;
  description?: string;
  category?: string;
  status: FormStatus;
  targetType: FormTargetType;
  targetIds: string[];
  requireSignatures: boolean;
  allowMultipleSubmissions: boolean;
  submissionDeadline?: string;
  createdBy: string;
  createdAt: string;
  updatedAt: string;
  publishedAt?: string;
  publishedBy?: string;
  archivedAt?: string;
  archivedBy?: string;
}

export interface FormSummary {
  id: string;
  title: string;
  description?: string;
  category?: string;
  status: FormStatus;
  targetType: FormTargetType;
  requireSignatures: boolean;
  allowMultipleSubmissions: boolean;
  submissionDeadline?: string;
  publishedAt?: string;
  createdAt: string;
  fieldCount: number;
  submissionCount: number;
}

export interface FormWithDetails extends Form {
  fields: FormField[];
  creatorName: string;
  submissionCount: number;
  pendingSubmissions: number;
  downloadCount: number;
}

// ============================================================================
// Form Field Types
// ============================================================================

export interface ConditionalDisplay {
  field: string;
  operator: string; // "equals", "not_equals", "contains", "not_empty"
  value: string;
}

export interface FieldOption {
  value: string;
  label: string;
}

export interface ValidationRules {
  min?: number;
  max?: number;
  minLength?: number;
  maxLength?: number;
  pattern?: string;
  patternMessage?: string;
}

export interface FormField {
  id: string;
  formId: string;
  label: string;
  fieldType: FormFieldType;
  required: boolean;
  placeholder?: string;
  helpText?: string;
  defaultValue?: string;
  options?: FieldOption[]; // For radio, select, multiselect
  validation?: ValidationRules;
  conditionalDisplay?: ConditionalDisplay;
  sortOrder: number;
  createdAt: string;
  updatedAt: string;
}

export interface CreateFormField {
  label: string;
  fieldType: FormFieldType;
  required?: boolean;
  placeholder?: string;
  helpText?: string;
  defaultValue?: string;
  options?: FieldOption[];
  validation?: ValidationRules;
  conditionalDisplay?: ConditionalDisplay;
  sortOrder?: number;
}

export interface UpdateFormField {
  label?: string;
  fieldType?: FormFieldType;
  required?: boolean;
  placeholder?: string;
  helpText?: string;
  defaultValue?: string;
  options?: FieldOption[];
  validation?: ValidationRules;
  conditionalDisplay?: ConditionalDisplay;
  sortOrder?: number;
}

// ============================================================================
// Form Submission Types
// ============================================================================

export interface FormSubmission {
  id: string;
  formId: string;
  userId: string;
  status: FormSubmissionStatus;
  data: Record<string, unknown>; // Field ID -> value mapping
  signatureData?: string; // Base64 signature image if required
  submittedAt: string;
  reviewedAt?: string;
  reviewedBy?: string;
  reviewNotes?: string;
}

export interface FormSubmissionSummary {
  id: string;
  formId: string;
  formTitle: string;
  userId: string;
  userName: string;
  status: FormSubmissionStatus;
  submittedAt: string;
  reviewedAt?: string;
}

export interface FormSubmissionWithDetails extends FormSubmission {
  userName: string;
  userEmail: string;
  formTitle: string;
  fields: FormField[];
  reviewerName?: string;
}

export interface SubmitFormRequest {
  data: Record<string, unknown>;
  signatureData?: string;
}

export interface ReviewSubmissionRequest {
  status: 'approved' | 'rejected';
  notes?: string;
}

// ============================================================================
// Form Download Types
// ============================================================================

export interface FormDownload {
  id: string;
  formId: string;
  userId: string;
  downloadedAt: string;
  format: FormExportFormat;
}

// ============================================================================
// Form Statistics
// ============================================================================

export interface FormStatistics {
  total: number;
  draft: number;
  published: number;
  archived: number;
  totalSubmissions: number;
  pendingSubmissions: number;
  totalDownloads: number;
}

// ============================================================================
// Request/Response Types
// ============================================================================

export interface CreateFormRequest {
  title: string;
  description?: string;
  category?: string;
  targetType?: FormTargetType;
  targetIds?: string[];
  requireSignatures?: boolean;
  allowMultipleSubmissions?: boolean;
  submissionDeadline?: string;
}

export interface UpdateFormRequest {
  title?: string;
  description?: string;
  category?: string;
  targetType?: FormTargetType;
  targetIds?: string[];
  requireSignatures?: boolean;
  allowMultipleSubmissions?: boolean;
  submissionDeadline?: string;
}

export interface ListFormsParams {
  page?: number;
  pageSize?: number;
  status?: FormStatus;
  category?: string;
  search?: string;
}

export interface ListFormSubmissionsParams {
  page?: number;
  pageSize?: number;
  status?: FormSubmissionStatus;
  userId?: string;
  fromDate?: string;
  toDate?: string;
}

export interface FormPaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  pageSize: number;
  totalPages: number;
}

// API Response wrappers
export interface CreateFormResponse {
  id: string;
  message: string;
}

export interface MessageResponse {
  message: string;
}

export interface FormResponse {
  message: string;
  form: Form;
}

export interface FieldOrderRequest {
  fieldIds: string[];
}
