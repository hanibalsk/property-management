/**
 * Neighbor TanStack Query Hooks
 *
 * React hooks for neighbor information with server state caching (Epic 6, Story 6.6).
 */

import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import type { NeighborsApi } from './api';
import type { UpdatePrivacySettingsRequest } from './types';

// Query keys factory for cache management
export const neighborKeys = {
  all: ['neighbors'] as const,
  list: (buildingId: string) => [...neighborKeys.all, 'list', buildingId] as const,
  privacy: () => [...neighborKeys.all, 'privacy'] as const,
};

export const createNeighborHooks = (api: NeighborsApi) => ({
  /**
   * List neighbors in a building
   */
  useNeighbors: (buildingId: string, enabled = true) =>
    useQuery({
      queryKey: neighborKeys.list(buildingId),
      queryFn: () => api.listNeighbors(buildingId),
      enabled: enabled && !!buildingId,
    }),

  /**
   * Get current user's privacy settings
   */
  usePrivacySettings: () =>
    useQuery({
      queryKey: neighborKeys.privacy(),
      queryFn: () => api.getPrivacySettings(),
    }),

  /**
   * Update privacy settings mutation
   */
  useUpdatePrivacySettings: () => {
    const queryClient = useQueryClient();
    return useMutation({
      mutationFn: (data: UpdatePrivacySettingsRequest) => api.updatePrivacySettings(data),
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: neighborKeys.privacy() });
        // Also invalidate neighbor lists since visibility may have changed
        queryClient.invalidateQueries({ queryKey: neighborKeys.all });
      },
    });
  },
});

export type NeighborHooks = ReturnType<typeof createNeighborHooks>;
