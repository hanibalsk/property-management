/**
 * Forms Management API Client
 *
 * API functions for managing forms (Epic 54).
 */

import type { ApiConfig } from '../index';
import type {
  CreateFormField,
  CreateFormRequest,
  CreateFormResponse,
  FieldOrderRequest,
  FormExportFormat,
  FormField,
  FormPaginatedResponse,
  FormResponse,
  FormStatistics,
  FormSubmission,
  FormSubmissionSummary,
  FormSubmissionWithDetails,
  FormSummary,
  FormWithDetails,
  ListFormSubmissionsParams,
  ListFormsParams,
  MessageResponse,
  ReviewSubmissionRequest,
  SubmitFormRequest,
  UpdateFormField,
  UpdateFormRequest,
} from './types';

const buildHeaders = (config: ApiConfig): HeadersInit => ({
  'Content-Type': 'application/json',
  ...(config.accessToken && { Authorization: `Bearer ${config.accessToken}` }),
  ...(config.tenantId && { 'X-Tenant-ID': config.tenantId }),
});

const handleResponse = async <T>(response: Response): Promise<T> => {
  if (!response.ok) {
    const error = await response.json().catch(() => ({ message: 'Unknown error' }));
    throw new Error(error.message || `HTTP ${response.status}`);
  }
  return response.json();
};

export const createFormsApi = (config: ApiConfig) => {
  const baseUrl = `${config.baseUrl}/api/v1/forms`;
  const headers = buildHeaders(config);

  return {
    // ========================================================================
    // Form CRUD Operations (Story 54.1)
    // ========================================================================

    /**
     * Create a new form template (managers only)
     */
    create: async (data: CreateFormRequest): Promise<CreateFormResponse> => {
      const response = await fetch(baseUrl, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * List forms with filters (managers)
     */
    list: async (params?: ListFormsParams): Promise<FormPaginatedResponse<FormSummary>> => {
      const searchParams = new URLSearchParams();
      if (params?.page) searchParams.set('page', params.page.toString());
      if (params?.pageSize) searchParams.set('pageSize', params.pageSize.toString());
      if (params?.status) searchParams.set('status', params.status);
      if (params?.category) searchParams.set('category', params.category);
      if (params?.search) searchParams.set('search', params.search);

      const url = searchParams.toString() ? `${baseUrl}?${searchParams}` : baseUrl;
      const response = await fetch(url, { headers });
      return handleResponse(response);
    },

    /**
     * List available forms for current user (Story 54.2)
     */
    listAvailable: async (params?: {
      page?: number;
      pageSize?: number;
      category?: string;
      search?: string;
    }): Promise<FormPaginatedResponse<FormSummary>> => {
      const searchParams = new URLSearchParams();
      if (params?.page) searchParams.set('page', params.page.toString());
      if (params?.pageSize) searchParams.set('pageSize', params.pageSize.toString());
      if (params?.category) searchParams.set('category', params.category);
      if (params?.search) searchParams.set('search', params.search);

      const url = searchParams.toString()
        ? `${baseUrl}/available?${searchParams}`
        : `${baseUrl}/available`;
      const response = await fetch(url, { headers });
      return handleResponse(response);
    },

    /**
     * Get form details
     */
    get: async (id: string): Promise<FormWithDetails> => {
      const response = await fetch(`${baseUrl}/${id}`, { headers });
      return handleResponse(response);
    },

    /**
     * Update a form (draft only)
     */
    update: async (id: string, data: UpdateFormRequest): Promise<FormResponse> => {
      const response = await fetch(`${baseUrl}/${id}`, {
        method: 'PUT',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Delete a form (draft only)
     */
    delete: async (id: string): Promise<void> => {
      const response = await fetch(`${baseUrl}/${id}`, {
        method: 'DELETE',
        headers,
      });
      if (!response.ok) {
        const error = await response.json().catch(() => ({ message: 'Unknown error' }));
        throw new Error(error.message || `HTTP ${response.status}`);
      }
    },

    /**
     * Publish a form (makes it available to users)
     */
    publish: async (id: string): Promise<FormResponse> => {
      const response = await fetch(`${baseUrl}/${id}/publish`, {
        method: 'POST',
        headers,
      });
      return handleResponse(response);
    },

    /**
     * Archive a form (no new submissions)
     */
    archive: async (id: string): Promise<FormResponse> => {
      const response = await fetch(`${baseUrl}/${id}/archive`, {
        method: 'POST',
        headers,
      });
      return handleResponse(response);
    },

    /**
     * Duplicate a form as a new draft
     */
    duplicate: async (id: string): Promise<CreateFormResponse> => {
      const response = await fetch(`${baseUrl}/${id}/duplicate`, {
        method: 'POST',
        headers,
      });
      return handleResponse(response);
    },

    // ========================================================================
    // Form Fields Operations (Story 54.1)
    // ========================================================================

    /**
     * List fields for a form
     */
    listFields: async (formId: string): Promise<FormField[]> => {
      const response = await fetch(`${baseUrl}/${formId}/fields`, { headers });
      return handleResponse(response);
    },

    /**
     * Add a field to a form
     */
    addField: async (formId: string, data: CreateFormField): Promise<FormField> => {
      const response = await fetch(`${baseUrl}/${formId}/fields`, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Update a field
     */
    updateField: async (
      formId: string,
      fieldId: string,
      data: UpdateFormField
    ): Promise<FormField> => {
      const response = await fetch(`${baseUrl}/${formId}/fields/${fieldId}`, {
        method: 'PUT',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Delete a field
     */
    deleteField: async (formId: string, fieldId: string): Promise<void> => {
      const response = await fetch(`${baseUrl}/${formId}/fields/${fieldId}`, {
        method: 'DELETE',
        headers,
      });
      if (!response.ok) {
        const error = await response.json().catch(() => ({ message: 'Unknown error' }));
        throw new Error(error.message || `HTTP ${response.status}`);
      }
    },

    /**
     * Reorder fields (drag-and-drop support)
     */
    reorderFields: async (formId: string, data: FieldOrderRequest): Promise<MessageResponse> => {
      const response = await fetch(`${baseUrl}/${formId}/fields/reorder`, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    // ========================================================================
    // Form Download/Export (Story 54.2)
    // ========================================================================

    /**
     * Download form as PDF (blank template)
     */
    downloadPdf: async (id: string): Promise<Blob> => {
      const response = await fetch(`${baseUrl}/${id}/download/pdf`, { headers });
      if (!response.ok) {
        const error = await response.json().catch(() => ({ message: 'Unknown error' }));
        throw new Error(error.message || `HTTP ${response.status}`);
      }
      return response.blob();
    },

    /**
     * Export form submissions data
     */
    exportSubmissions: async (id: string, format: FormExportFormat): Promise<Blob> => {
      const response = await fetch(`${baseUrl}/${id}/export?format=${format}`, { headers });
      if (!response.ok) {
        const error = await response.json().catch(() => ({ message: 'Unknown error' }));
        throw new Error(error.message || `HTTP ${response.status}`);
      }
      return response.blob();
    },

    // ========================================================================
    // Form Submissions (Story 54.3, 54.4, 54.5)
    // ========================================================================

    /**
     * Submit a filled form (Story 54.3)
     */
    submit: async (
      id: string,
      data: SubmitFormRequest
    ): Promise<{ id: string; message: string }> => {
      const response = await fetch(`${baseUrl}/${id}/submit`, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * List submissions for a form (managers - Story 54.4)
     */
    listSubmissions: async (
      formId: string,
      params?: ListFormSubmissionsParams
    ): Promise<FormPaginatedResponse<FormSubmissionSummary>> => {
      const searchParams = new URLSearchParams();
      if (params?.page) searchParams.set('page', params.page.toString());
      if (params?.pageSize) searchParams.set('pageSize', params.pageSize.toString());
      if (params?.status) searchParams.set('status', params.status);
      if (params?.userId) searchParams.set('userId', params.userId);
      if (params?.fromDate) searchParams.set('fromDate', params.fromDate);
      if (params?.toDate) searchParams.set('toDate', params.toDate);

      const url = searchParams.toString()
        ? `${baseUrl}/${formId}/submissions?${searchParams}`
        : `${baseUrl}/${formId}/submissions`;
      const response = await fetch(url, { headers });
      return handleResponse(response);
    },

    /**
     * Get submission details
     */
    getSubmission: async (
      formId: string,
      submissionId: string
    ): Promise<FormSubmissionWithDetails> => {
      const response = await fetch(`${baseUrl}/${formId}/submissions/${submissionId}`, { headers });
      return handleResponse(response);
    },

    /**
     * Review a submission (approve/reject - Story 54.4)
     */
    reviewSubmission: async (
      formId: string,
      submissionId: string,
      data: ReviewSubmissionRequest
    ): Promise<{ message: string; submission: FormSubmission }> => {
      const response = await fetch(`${baseUrl}/${formId}/submissions/${submissionId}/review`, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Get my submissions (user view)
     */
    listMySubmissions: async (params?: {
      page?: number;
      pageSize?: number;
      status?: string;
    }): Promise<FormPaginatedResponse<FormSubmissionSummary>> => {
      const searchParams = new URLSearchParams();
      if (params?.page) searchParams.set('page', params.page.toString());
      if (params?.pageSize) searchParams.set('pageSize', params.pageSize.toString());
      if (params?.status) searchParams.set('status', params.status);

      const url = searchParams.toString()
        ? `${baseUrl}/my-submissions?${searchParams}`
        : `${baseUrl}/my-submissions`;
      const response = await fetch(url, { headers });
      return handleResponse(response);
    },

    // ========================================================================
    // Statistics
    // ========================================================================

    /**
     * Get form statistics (dashboard)
     */
    getStatistics: async (): Promise<FormStatistics> => {
      const response = await fetch(`${baseUrl}/statistics`, { headers });
      return handleResponse(response);
    },

    /**
     * Get categories (for filtering)
     */
    getCategories: async (): Promise<string[]> => {
      const response = await fetch(`${baseUrl}/categories`, { headers });
      return handleResponse(response);
    },
  };
};

export type FormsApi = ReturnType<typeof createFormsApi>;
