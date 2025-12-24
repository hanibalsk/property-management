/**
 * Government Portal TanStack Query Hooks
 *
 * React hooks for managing government portal integrations with server state caching.
 */

import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import type { GovernmentPortalApi } from './api';
import type {
  AddSubmissionAttachmentRequest,
  CreatePortalConnectionRequest,
  CreateRegulatorySubmissionRequest,
  CreateSubmissionScheduleRequest,
  ListSubmissionsParams,
  ListTemplatesParams,
  UpdatePortalConnectionRequest,
  UpdateRegulatorySubmissionRequest,
  UpdateSubmissionScheduleRequest,
} from './types';

// Query keys factory for cache management
export const governmentPortalKeys = {
  all: ['government-portal'] as const,

  // Connections
  connections: () => [...governmentPortalKeys.all, 'connections'] as const,
  connection: (id: string) => [...governmentPortalKeys.connections(), id] as const,

  // Templates
  templates: () => [...governmentPortalKeys.all, 'templates'] as const,
  templatesList: (params?: ListTemplatesParams) =>
    [...governmentPortalKeys.templates(), params] as const,
  template: (id: string) => [...governmentPortalKeys.templates(), id] as const,

  // Submissions
  submissions: () => [...governmentPortalKeys.all, 'submissions'] as const,
  submissionsList: (params?: ListSubmissionsParams) =>
    [...governmentPortalKeys.submissions(), 'list', params] as const,
  submission: (id: string) => [...governmentPortalKeys.submissions(), id] as const,
  submissionAudit: (id: string) => [...governmentPortalKeys.submission(id), 'audit'] as const,
  submissionAttachments: (id: string) =>
    [...governmentPortalKeys.submission(id), 'attachments'] as const,

  // Schedules
  schedules: () => [...governmentPortalKeys.all, 'schedules'] as const,
  schedule: (id: string) => [...governmentPortalKeys.schedules(), id] as const,

  // Statistics
  stats: () => [...governmentPortalKeys.all, 'stats'] as const,
};

export const createGovernmentPortalHooks = (api: GovernmentPortalApi) => ({
  // ========================================================================
  // Connections
  // ========================================================================

  /**
   * List portal connections
   */
  useConnections: () =>
    useQuery({
      queryKey: governmentPortalKeys.connections(),
      queryFn: () => api.listConnections(),
    }),

  /**
   * Get a portal connection
   */
  useConnection: (id: string, enabled = true) =>
    useQuery({
      queryKey: governmentPortalKeys.connection(id),
      queryFn: () => api.getConnection(id),
      enabled: enabled && !!id,
    }),

  /**
   * Create connection mutation
   */
  useCreateConnection: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: (data: CreatePortalConnectionRequest) => api.createConnection(data),
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.connections() });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.stats() });
      },
    });
  },

  /**
   * Update connection mutation
   */
  useUpdateConnection: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: ({ id, data }: { id: string; data: UpdatePortalConnectionRequest }) =>
        api.updateConnection(id, data),
      onSuccess: (_, { id }) => {
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.connection(id) });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.connections() });
      },
    });
  },

  /**
   * Delete connection mutation
   */
  useDeleteConnection: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: (id: string) => api.deleteConnection(id),
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.connections() });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.stats() });
      },
    });
  },

  /**
   * Test connection mutation
   */
  useTestConnection: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: (id: string) => api.testConnection(id),
      onSuccess: (_data, id) => {
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.connection(id) });
      },
    });
  },

  // ========================================================================
  // Templates
  // ========================================================================

  /**
   * List report templates
   */
  useTemplates: (params?: ListTemplatesParams) =>
    useQuery({
      queryKey: governmentPortalKeys.templatesList(params),
      queryFn: () => api.listTemplates(params),
    }),

  /**
   * Get a report template
   */
  useTemplate: (id: string, enabled = true) =>
    useQuery({
      queryKey: governmentPortalKeys.template(id),
      queryFn: () => api.getTemplate(id),
      enabled: enabled && !!id,
    }),

  // ========================================================================
  // Submissions
  // ========================================================================

  /**
   * List submissions
   */
  useSubmissions: (params?: ListSubmissionsParams) =>
    useQuery({
      queryKey: governmentPortalKeys.submissionsList(params),
      queryFn: () => api.listSubmissions(params),
    }),

  /**
   * Get a submission
   */
  useSubmission: (id: string, enabled = true) =>
    useQuery({
      queryKey: governmentPortalKeys.submission(id),
      queryFn: () => api.getSubmission(id),
      enabled: enabled && !!id,
    }),

  /**
   * Get submission audit trail
   */
  useSubmissionAudit: (id: string, enabled = true) =>
    useQuery({
      queryKey: governmentPortalKeys.submissionAudit(id),
      queryFn: () => api.getSubmissionAudit(id),
      enabled: enabled && !!id,
    }),

  /**
   * Create submission mutation
   */
  useCreateSubmission: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: (data: CreateRegulatorySubmissionRequest) => api.createSubmission(data),
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.submissions() });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.stats() });
      },
    });
  },

  /**
   * Update submission mutation
   */
  useUpdateSubmission: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: ({ id, data }: { id: string; data: UpdateRegulatorySubmissionRequest }) =>
        api.updateSubmission(id, data),
      onSuccess: (_, { id }) => {
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.submission(id) });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.submissions() });
      },
    });
  },

  /**
   * Validate submission mutation
   */
  useValidateSubmission: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: (id: string) => api.validateSubmission(id),
      onSuccess: (_data, id) => {
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.submission(id) });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.submissionAudit(id) });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.submissions() });
      },
    });
  },

  /**
   * Submit submission mutation
   */
  useSubmitSubmission: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: (id: string) => api.submitSubmission(id),
      onSuccess: (_data, id) => {
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.submission(id) });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.submissionAudit(id) });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.submissions() });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.stats() });
      },
    });
  },

  /**
   * Cancel submission mutation
   */
  useCancelSubmission: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: (id: string) => api.cancelSubmission(id),
      onSuccess: (_data, id) => {
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.submission(id) });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.submissionAudit(id) });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.submissions() });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.stats() });
      },
    });
  },

  // ========================================================================
  // Submission Attachments
  // ========================================================================

  /**
   * List attachments for a submission
   */
  useSubmissionAttachments: (submissionId: string, enabled = true) =>
    useQuery({
      queryKey: governmentPortalKeys.submissionAttachments(submissionId),
      queryFn: () => api.listAttachments(submissionId),
      enabled: enabled && !!submissionId,
    }),

  /**
   * Add attachment mutation
   */
  useAddAttachment: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: ({
        submissionId,
        data,
      }: {
        submissionId: string;
        data: AddSubmissionAttachmentRequest;
      }) => api.addAttachment(submissionId, data),
      onSuccess: (_, { submissionId }) => {
        queryClient.invalidateQueries({
          queryKey: governmentPortalKeys.submissionAttachments(submissionId),
        });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.submission(submissionId) });
      },
    });
  },

  /**
   * Delete attachment mutation
   */
  useDeleteAttachment: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: ({
        submissionId,
        attachmentId,
      }: {
        submissionId: string;
        attachmentId: string;
      }) => api.deleteAttachment(submissionId, attachmentId),
      onSuccess: (_, { submissionId }) => {
        queryClient.invalidateQueries({
          queryKey: governmentPortalKeys.submissionAttachments(submissionId),
        });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.submission(submissionId) });
      },
    });
  },

  // ========================================================================
  // Schedules
  // ========================================================================

  /**
   * List schedules
   */
  useSchedules: () =>
    useQuery({
      queryKey: governmentPortalKeys.schedules(),
      queryFn: () => api.listSchedules(),
    }),

  /**
   * Get a schedule
   */
  useSchedule: (id: string, enabled = true) =>
    useQuery({
      queryKey: governmentPortalKeys.schedule(id),
      queryFn: () => api.getSchedule(id),
      enabled: enabled && !!id,
    }),

  /**
   * Create schedule mutation
   */
  useCreateSchedule: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: (data: CreateSubmissionScheduleRequest) => api.createSchedule(data),
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.schedules() });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.stats() });
      },
    });
  },

  /**
   * Update schedule mutation
   */
  useUpdateSchedule: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: ({ id, data }: { id: string; data: UpdateSubmissionScheduleRequest }) =>
        api.updateSchedule(id, data),
      onSuccess: (_, { id }) => {
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.schedule(id) });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.schedules() });
      },
    });
  },

  /**
   * Delete schedule mutation
   */
  useDeleteSchedule: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: (id: string) => api.deleteSchedule(id),
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.schedules() });
        queryClient.invalidateQueries({ queryKey: governmentPortalKeys.stats() });
      },
    });
  },

  // ========================================================================
  // Statistics
  // ========================================================================

  /**
   * Get government portal statistics
   */
  useStats: () =>
    useQuery({
      queryKey: governmentPortalKeys.stats(),
      queryFn: () => api.getStats(),
      refetchInterval: 60000, // Refetch every minute
    }),
});

export type GovernmentPortalHooks = ReturnType<typeof createGovernmentPortalHooks>;
