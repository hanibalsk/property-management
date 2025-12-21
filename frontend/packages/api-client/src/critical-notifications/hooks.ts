/**
 * Critical Notifications Hooks (Epic 8A, Story 8A.2)
 */

import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import {
  acknowledgeCriticalNotification,
  createCriticalNotification,
  getCriticalNotificationStats,
  getCriticalNotifications,
  getUnacknowledgedNotifications,
} from './api';
import type { CreateCriticalNotificationRequest } from './types';

interface UseNotificationsOptions {
  organizationId: string;
  accessToken: string;
  tenantContext?: string;
  enabled?: boolean;
}

/**
 * Hook to fetch all critical notifications for an organization
 */
export function useCriticalNotifications(options: UseNotificationsOptions) {
  const { organizationId, accessToken, tenantContext, enabled = true } = options;

  return useQuery({
    queryKey: ['critical-notifications', organizationId],
    queryFn: () => getCriticalNotifications(organizationId, { accessToken, tenantContext }),
    enabled: enabled && !!accessToken && !!organizationId,
    staleTime: 30000, // 30 seconds
  });
}

/**
 * Hook to fetch unacknowledged critical notifications
 */
export function useUnacknowledgedNotifications(options: UseNotificationsOptions) {
  const { organizationId, accessToken, tenantContext, enabled = true } = options;

  return useQuery({
    queryKey: ['critical-notifications', organizationId, 'unacknowledged'],
    queryFn: () => getUnacknowledgedNotifications(organizationId, { accessToken, tenantContext }),
    enabled: enabled && !!accessToken && !!organizationId,
    staleTime: 10000, // 10 seconds - check frequently for new critical notifications
    refetchInterval: 60000, // Refetch every minute
  });
}

/**
 * Hook to create a new critical notification
 */
export function useCreateCriticalNotification(options: UseNotificationsOptions) {
  const { organizationId, accessToken, tenantContext } = options;
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (request: CreateCriticalNotificationRequest) =>
      createCriticalNotification(organizationId, request, {
        accessToken,
        tenantContext,
      }),
    onSuccess: () => {
      // Invalidate all related queries
      queryClient.invalidateQueries({
        queryKey: ['critical-notifications', organizationId],
      });
    },
  });
}

/**
 * Hook to acknowledge a critical notification
 */
export function useAcknowledgeCriticalNotification(options: UseNotificationsOptions) {
  const { organizationId, accessToken, tenantContext } = options;
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (notificationId: string) =>
      acknowledgeCriticalNotification(organizationId, notificationId, {
        accessToken,
        tenantContext,
      }),
    onSuccess: () => {
      // Invalidate all related queries
      queryClient.invalidateQueries({
        queryKey: ['critical-notifications', organizationId],
      });
    },
  });
}

/**
 * Hook to fetch notification statistics (admin only)
 */
export function useCriticalNotificationStats(
  options: UseNotificationsOptions & { notificationId: string }
) {
  const { organizationId, notificationId, accessToken, tenantContext, enabled = true } = options;

  return useQuery({
    queryKey: ['critical-notifications', organizationId, notificationId, 'stats'],
    queryFn: () =>
      getCriticalNotificationStats(organizationId, notificationId, {
        accessToken,
        tenantContext,
      }),
    enabled: enabled && !!accessToken && !!organizationId && !!notificationId,
    staleTime: 30000, // 30 seconds
  });
}
