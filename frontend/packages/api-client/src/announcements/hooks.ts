/**
 * Announcement TanStack Query Hooks
 *
 * React hooks for managing announcements with server state caching.
 */

import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import type { AnnouncementsApi } from './api';
import type {
  AddAttachmentRequest,
  CreateAnnouncementRequest,
  CreateCommentRequest,
  ListAnnouncementsParams,
  ListCommentsParams,
  PinAnnouncementRequest,
  ScheduleAnnouncementRequest,
  UpdateAnnouncementRequest,
} from './types';

// Query keys factory for cache management
export const announcementKeys = {
  all: ['announcements'] as const,
  lists: () => [...announcementKeys.all, 'list'] as const,
  list: (params?: ListAnnouncementsParams) => [...announcementKeys.lists(), params] as const,
  published: () => [...announcementKeys.all, 'published'] as const,
  details: () => [...announcementKeys.all, 'detail'] as const,
  detail: (id: string) => [...announcementKeys.details(), id] as const,
  attachments: (id: string) => [...announcementKeys.detail(id), 'attachments'] as const,
  acknowledgments: (id: string) => [...announcementKeys.detail(id), 'acknowledgments'] as const,
  comments: (id: string, params?: ListCommentsParams) =>
    [...announcementKeys.detail(id), 'comments', params] as const,
  statistics: () => [...announcementKeys.all, 'statistics'] as const,
  unreadCount: () => [...announcementKeys.all, 'unread-count'] as const,
};

export const createAnnouncementHooks = (api: AnnouncementsApi) => ({
  /**
   * List announcements with filters
   */
  useList: (params?: ListAnnouncementsParams) =>
    useQuery({
      queryKey: announcementKeys.list(params),
      queryFn: () => api.list(params),
    }),

  /**
   * List published announcements
   */
  useListPublished: (params?: { page?: number; pageSize?: number }) =>
    useQuery({
      queryKey: announcementKeys.published(),
      queryFn: () => api.listPublished(params),
    }),

  /**
   * Get announcement details
   */
  useGet: (id: string, enabled = true) =>
    useQuery({
      queryKey: announcementKeys.detail(id),
      queryFn: () => api.get(id),
      enabled: enabled && !!id,
    }),

  /**
   * Get announcement statistics
   */
  useStatistics: () =>
    useQuery({
      queryKey: announcementKeys.statistics(),
      queryFn: () => api.getStatistics(),
    }),

  /**
   * Get unread announcement count
   */
  useUnreadCount: () =>
    useQuery({
      queryKey: announcementKeys.unreadCount(),
      queryFn: () => api.getUnreadCount(),
      refetchInterval: 60000, // Refetch every minute
    }),

  /**
   * Create announcement mutation
   */
  useCreate: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: (data: CreateAnnouncementRequest) => api.create(data),
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: announcementKeys.lists() });
        queryClient.invalidateQueries({ queryKey: announcementKeys.statistics() });
      },
    });
  },

  /**
   * Update announcement mutation
   */
  useUpdate: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: ({ id, data }: { id: string; data: UpdateAnnouncementRequest }) =>
        api.update(id, data),
      onSuccess: (_, { id }) => {
        queryClient.invalidateQueries({ queryKey: announcementKeys.detail(id) });
        queryClient.invalidateQueries({ queryKey: announcementKeys.lists() });
      },
    });
  },

  /**
   * Delete announcement mutation
   */
  useDelete: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: (id: string) => api.delete(id),
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: announcementKeys.lists() });
        queryClient.invalidateQueries({ queryKey: announcementKeys.statistics() });
      },
    });
  },

  /**
   * Publish announcement mutation
   */
  usePublish: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: (id: string) => api.publish(id),
      onSuccess: (_, id) => {
        queryClient.invalidateQueries({ queryKey: announcementKeys.detail(id) });
        queryClient.invalidateQueries({ queryKey: announcementKeys.lists() });
        queryClient.invalidateQueries({ queryKey: announcementKeys.published() });
        queryClient.invalidateQueries({ queryKey: announcementKeys.statistics() });
      },
    });
  },

  /**
   * Schedule announcement mutation
   */
  useSchedule: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: ({ id, data }: { id: string; data: ScheduleAnnouncementRequest }) =>
        api.schedule(id, data),
      onSuccess: (_, { id }) => {
        queryClient.invalidateQueries({ queryKey: announcementKeys.detail(id) });
        queryClient.invalidateQueries({ queryKey: announcementKeys.lists() });
        queryClient.invalidateQueries({ queryKey: announcementKeys.statistics() });
      },
    });
  },

  /**
   * Archive announcement mutation
   */
  useArchive: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: (id: string) => api.archive(id),
      onSuccess: (_, id) => {
        queryClient.invalidateQueries({ queryKey: announcementKeys.detail(id) });
        queryClient.invalidateQueries({ queryKey: announcementKeys.lists() });
        queryClient.invalidateQueries({ queryKey: announcementKeys.published() });
        queryClient.invalidateQueries({ queryKey: announcementKeys.statistics() });
      },
    });
  },

  /**
   * Pin/unpin announcement mutation
   */
  usePin: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: ({ id, data }: { id: string; data: PinAnnouncementRequest }) => api.pin(id, data),
      onSuccess: (_, { id }) => {
        queryClient.invalidateQueries({ queryKey: announcementKeys.detail(id) });
        queryClient.invalidateQueries({ queryKey: announcementKeys.lists() });
        queryClient.invalidateQueries({ queryKey: announcementKeys.published() });
      },
    });
  },

  /**
   * Add attachment mutation
   */
  useAddAttachment: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: ({ id, data }: { id: string; data: AddAttachmentRequest }) =>
        api.addAttachment(id, data),
      onSuccess: (_, { id }) => {
        queryClient.invalidateQueries({ queryKey: announcementKeys.detail(id) });
        queryClient.invalidateQueries({ queryKey: announcementKeys.attachments(id) });
      },
    });
  },

  /**
   * Delete attachment mutation
   */
  useDeleteAttachment: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: ({ id, attachmentId }: { id: string; attachmentId: string }) =>
        api.deleteAttachment(id, attachmentId),
      onSuccess: (_, { id }) => {
        queryClient.invalidateQueries({ queryKey: announcementKeys.detail(id) });
        queryClient.invalidateQueries({ queryKey: announcementKeys.attachments(id) });
      },
    });
  },

  /**
   * Mark announcement as read mutation
   */
  useMarkRead: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: (id: string) => api.markRead(id),
      onSuccess: (_, id) => {
        queryClient.invalidateQueries({ queryKey: announcementKeys.detail(id) });
        queryClient.invalidateQueries({ queryKey: announcementKeys.unreadCount() });
      },
    });
  },

  /**
   * Acknowledge announcement mutation
   */
  useAcknowledge: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: (id: string) => api.acknowledge(id),
      onSuccess: (_, id) => {
        queryClient.invalidateQueries({ queryKey: announcementKeys.detail(id) });
        queryClient.invalidateQueries({ queryKey: announcementKeys.acknowledgments(id) });
      },
    });
  },

  /**
   * Get acknowledgment statistics for an announcement (Story 6.2)
   */
  useAcknowledgmentStats: (id: string, enabled = true) =>
    useQuery({
      queryKey: announcementKeys.acknowledgments(id),
      queryFn: () => api.getAcknowledgmentStats(id),
      enabled: enabled && !!id,
    }),

  // ========================================================================
  // Comments (Story 6.3)
  // ========================================================================

  /**
   * List comments for an announcement
   */
  useComments: (id: string, params?: ListCommentsParams, enabled = true) =>
    useQuery({
      queryKey: announcementKeys.comments(id, params),
      queryFn: () => api.listComments(id, params),
      enabled: enabled && !!id,
    }),

  /**
   * Create comment mutation
   */
  useCreateComment: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: ({ id, data }: { id: string; data: CreateCommentRequest }) =>
        api.createComment(id, data),
      onSuccess: (_, { id }) => {
        queryClient.invalidateQueries({ queryKey: announcementKeys.comments(id) });
        queryClient.invalidateQueries({ queryKey: announcementKeys.detail(id) });
      },
    });
  },

  /**
   * Delete comment mutation
   */
  useDeleteComment: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: ({
        announcementId,
        commentId,
        reason,
      }: {
        announcementId: string;
        commentId: string;
        reason?: string;
      }) => api.deleteComment(announcementId, commentId, reason ? { reason } : undefined),
      onSuccess: (_, { announcementId }) => {
        queryClient.invalidateQueries({ queryKey: announcementKeys.comments(announcementId) });
        queryClient.invalidateQueries({ queryKey: announcementKeys.detail(announcementId) });
      },
    });
  },
});

export type AnnouncementHooks = ReturnType<typeof createAnnouncementHooks>;
