/**
 * Notification Preferences Hooks (Epic 8A, Story 8A.1)
 */

import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import {
  ConfirmationRequiredError,
  getNotificationPreferences,
  updateNotificationPreference,
} from './api';
import type {
  NotificationChannel,
  NotificationPreferencesResponse,
  UpdateNotificationPreferenceRequest,
} from './types';

/** Query key for notification preferences */
export const NOTIFICATION_PREFERENCES_KEY = ['notification-preferences'] as const;

interface UseNotificationPreferencesOptions {
  baseUrl: string;
  accessToken: string;
}

/**
 * Hook to fetch notification preferences.
 */
export function useNotificationPreferences({
  baseUrl,
  accessToken,
}: UseNotificationPreferencesOptions) {
  return useQuery({
    queryKey: NOTIFICATION_PREFERENCES_KEY,
    queryFn: () => getNotificationPreferences(baseUrl, accessToken),
    enabled: !!accessToken,
  });
}

/**
 * Hook to update a notification preference with optimistic updates.
 */
export function useUpdateNotificationPreference({
  baseUrl,
  accessToken,
}: UseNotificationPreferencesOptions) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      channel,
      request,
    }: {
      channel: NotificationChannel;
      request: UpdateNotificationPreferenceRequest;
    }) => updateNotificationPreference(baseUrl, accessToken, channel, request),

    // Optimistic update
    onMutate: async ({ channel, request }) => {
      // Cancel any outgoing refetches
      await queryClient.cancelQueries({ queryKey: NOTIFICATION_PREFERENCES_KEY });

      // Snapshot the previous value
      const previousPreferences = queryClient.getQueryData<NotificationPreferencesResponse>(
        NOTIFICATION_PREFERENCES_KEY
      );

      // Optimistically update
      if (previousPreferences) {
        queryClient.setQueryData<NotificationPreferencesResponse>(NOTIFICATION_PREFERENCES_KEY, {
          ...previousPreferences,
          preferences: previousPreferences.preferences.map((pref) =>
            pref.channel === channel ? { ...pref, enabled: request.enabled } : pref
          ),
        });
      }

      return { previousPreferences };
    },

    // Rollback on error
    onError: (_err, _variables, context) => {
      if (context?.previousPreferences) {
        queryClient.setQueryData(NOTIFICATION_PREFERENCES_KEY, context.previousPreferences);
      }
    },

    // Refetch after success
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: NOTIFICATION_PREFERENCES_KEY });
    },
  });
}

export { ConfirmationRequiredError };
