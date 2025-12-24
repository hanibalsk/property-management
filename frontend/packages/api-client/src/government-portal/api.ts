/**
 * Government Portal API Client
 *
 * API functions for managing government portal integrations (Epic 30, UC-22.3).
 */

import type { ApiConfig } from '../index';
import type {
  AddSubmissionAttachmentRequest,
  CreatePortalConnectionRequest,
  CreateRegulatorySubmissionRequest,
  CreateSubmissionScheduleRequest,
  GovernmentPortalConnection,
  GovernmentPortalStats,
  ListSubmissionsParams,
  ListTemplatesParams,
  RegulatoryReportTemplate,
  RegulatorySubmission,
  RegulatorySubmissionAttachment,
  RegulatorySubmissionAudit,
  RegulatorySubmissionSchedule,
  TestConnectionResponse,
  UpdatePortalConnectionRequest,
  UpdateRegulatorySubmissionRequest,
  UpdateSubmissionScheduleRequest,
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

export const createGovernmentPortalApi = (config: ApiConfig) => {
  const baseUrl = `${config.baseUrl}/api/v1/government-portal`;
  const headers = buildHeaders(config);

  return {
    // ========================================================================
    // Portal Connections
    // ========================================================================

    /**
     * List portal connections
     */
    listConnections: async (): Promise<GovernmentPortalConnection[]> => {
      const response = await fetch(`${baseUrl}/connections`, { headers });
      return handleResponse(response);
    },

    /**
     * Get a portal connection
     */
    getConnection: async (id: string): Promise<GovernmentPortalConnection> => {
      const response = await fetch(`${baseUrl}/connections/${id}`, { headers });
      return handleResponse(response);
    },

    /**
     * Create a portal connection
     */
    createConnection: async (
      data: CreatePortalConnectionRequest
    ): Promise<GovernmentPortalConnection> => {
      const response = await fetch(`${baseUrl}/connections`, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Update a portal connection
     */
    updateConnection: async (
      id: string,
      data: UpdatePortalConnectionRequest
    ): Promise<GovernmentPortalConnection> => {
      const response = await fetch(`${baseUrl}/connections/${id}`, {
        method: 'PUT',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Delete a portal connection
     */
    deleteConnection: async (id: string): Promise<void> => {
      const response = await fetch(`${baseUrl}/connections/${id}`, {
        method: 'DELETE',
        headers,
      });
      if (!response.ok) {
        const error = await response.json().catch(() => ({ message: 'Unknown error' }));
        throw new Error(error.message || `HTTP ${response.status}`);
      }
    },

    /**
     * Test a portal connection
     */
    testConnection: async (id: string): Promise<TestConnectionResponse> => {
      const response = await fetch(`${baseUrl}/connections/${id}/test`, {
        method: 'POST',
        headers,
      });
      return handleResponse(response);
    },

    // ========================================================================
    // Report Templates
    // ========================================================================

    /**
     * List report templates
     */
    listTemplates: async (params?: ListTemplatesParams): Promise<RegulatoryReportTemplate[]> => {
      const searchParams = new URLSearchParams();
      if (params?.portalType) searchParams.set('portalType', params.portalType);
      if (params?.countryCode) searchParams.set('countryCode', params.countryCode);

      const url = searchParams.toString()
        ? `${baseUrl}/templates?${searchParams}`
        : `${baseUrl}/templates`;
      const response = await fetch(url, { headers });
      return handleResponse(response);
    },

    /**
     * Get a report template
     */
    getTemplate: async (id: string): Promise<RegulatoryReportTemplate> => {
      const response = await fetch(`${baseUrl}/templates/${id}`, { headers });
      return handleResponse(response);
    },

    // ========================================================================
    // Regulatory Submissions
    // ========================================================================

    /**
     * List submissions
     */
    listSubmissions: async (params?: ListSubmissionsParams): Promise<RegulatorySubmission[]> => {
      const searchParams = new URLSearchParams();
      if (params?.status) searchParams.set('status', params.status);
      if (params?.reportType) searchParams.set('reportType', params.reportType);
      if (params?.fromDate) searchParams.set('fromDate', params.fromDate);
      if (params?.toDate) searchParams.set('toDate', params.toDate);
      if (params?.limit) searchParams.set('limit', params.limit.toString());
      if (params?.offset) searchParams.set('offset', params.offset.toString());

      const url = searchParams.toString()
        ? `${baseUrl}/submissions?${searchParams}`
        : `${baseUrl}/submissions`;
      const response = await fetch(url, { headers });
      return handleResponse(response);
    },

    /**
     * Get a submission
     */
    getSubmission: async (id: string): Promise<RegulatorySubmission> => {
      const response = await fetch(`${baseUrl}/submissions/${id}`, { headers });
      return handleResponse(response);
    },

    /**
     * Create a submission
     */
    createSubmission: async (
      data: CreateRegulatorySubmissionRequest
    ): Promise<RegulatorySubmission> => {
      const response = await fetch(`${baseUrl}/submissions`, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Update a submission
     */
    updateSubmission: async (
      id: string,
      data: UpdateRegulatorySubmissionRequest
    ): Promise<RegulatorySubmission> => {
      const response = await fetch(`${baseUrl}/submissions/${id}`, {
        method: 'PUT',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Validate a submission
     */
    validateSubmission: async (id: string): Promise<RegulatorySubmission> => {
      const response = await fetch(`${baseUrl}/submissions/${id}/validate`, {
        method: 'POST',
        headers,
      });
      return handleResponse(response);
    },

    /**
     * Submit a submission to the portal
     */
    submitSubmission: async (id: string): Promise<RegulatorySubmission> => {
      const response = await fetch(`${baseUrl}/submissions/${id}/submit`, {
        method: 'POST',
        headers,
      });
      return handleResponse(response);
    },

    /**
     * Cancel a submission
     */
    cancelSubmission: async (id: string): Promise<RegulatorySubmission> => {
      const response = await fetch(`${baseUrl}/submissions/${id}/cancel`, {
        method: 'POST',
        headers,
      });
      return handleResponse(response);
    },

    /**
     * Get submission audit trail
     */
    getSubmissionAudit: async (id: string): Promise<RegulatorySubmissionAudit[]> => {
      const response = await fetch(`${baseUrl}/submissions/${id}/audit`, { headers });
      return handleResponse(response);
    },

    // ========================================================================
    // Submission Attachments
    // ========================================================================

    /**
     * List attachments for a submission
     */
    listAttachments: async (submissionId: string): Promise<RegulatorySubmissionAttachment[]> => {
      const response = await fetch(`${baseUrl}/submissions/${submissionId}/attachments`, {
        headers,
      });
      return handleResponse(response);
    },

    /**
     * Add an attachment to a submission
     */
    addAttachment: async (
      submissionId: string,
      data: AddSubmissionAttachmentRequest
    ): Promise<RegulatorySubmissionAttachment> => {
      const response = await fetch(`${baseUrl}/submissions/${submissionId}/attachments`, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Delete an attachment
     */
    deleteAttachment: async (submissionId: string, attachmentId: string): Promise<void> => {
      const response = await fetch(
        `${baseUrl}/submissions/${submissionId}/attachments/${attachmentId}`,
        {
          method: 'DELETE',
          headers,
        }
      );
      if (!response.ok) {
        const error = await response.json().catch(() => ({ message: 'Unknown error' }));
        throw new Error(error.message || `HTTP ${response.status}`);
      }
    },

    // ========================================================================
    // Submission Schedules
    // ========================================================================

    /**
     * List schedules
     */
    listSchedules: async (): Promise<RegulatorySubmissionSchedule[]> => {
      const response = await fetch(`${baseUrl}/schedules`, { headers });
      return handleResponse(response);
    },

    /**
     * Get a schedule
     */
    getSchedule: async (id: string): Promise<RegulatorySubmissionSchedule> => {
      const response = await fetch(`${baseUrl}/schedules/${id}`, { headers });
      return handleResponse(response);
    },

    /**
     * Create a schedule
     */
    createSchedule: async (
      data: CreateSubmissionScheduleRequest
    ): Promise<RegulatorySubmissionSchedule> => {
      const response = await fetch(`${baseUrl}/schedules`, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Update a schedule
     */
    updateSchedule: async (
      id: string,
      data: UpdateSubmissionScheduleRequest
    ): Promise<RegulatorySubmissionSchedule> => {
      const response = await fetch(`${baseUrl}/schedules/${id}`, {
        method: 'PUT',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Delete a schedule
     */
    deleteSchedule: async (id: string): Promise<void> => {
      const response = await fetch(`${baseUrl}/schedules/${id}`, {
        method: 'DELETE',
        headers,
      });
      if (!response.ok) {
        const error = await response.json().catch(() => ({ message: 'Unknown error' }));
        throw new Error(error.message || `HTTP ${response.status}`);
      }
    },

    // ========================================================================
    // Statistics
    // ========================================================================

    /**
     * Get government portal statistics
     */
    getStats: async (): Promise<GovernmentPortalStats> => {
      const response = await fetch(`${baseUrl}/stats`, { headers });
      return handleResponse(response);
    },
  };
};

export type GovernmentPortalApi = ReturnType<typeof createGovernmentPortalApi>;
