/**
 * Outages React Query Hooks
 *
 * React Query hooks for Outages API (UC-12).
 */

import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import * as api from './api';
import type {
  CancelOutageRequest,
  CreateOutageRequest,
  OutageListQuery,
  ResolveOutageRequest,
  StartOutageRequest,
  UpdateOutageRequest,
} from './types';

// ============================================
// Query Key Factory
// ============================================

export const outageKeys = {
  all: ['outages'] as const,
  lists: () => [...outageKeys.all, 'list'] as const,
  list: (query?: OutageListQuery) => [...outageKeys.lists(), query] as const,
  active: () => [...outageKeys.lists(), 'active'] as const,
  details: () => [...outageKeys.all, 'detail'] as const,
  detail: (id: string) => [...outageKeys.details(), id] as const,
  statistics: () => [...outageKeys.all, 'statistics'] as const,
  dashboard: () => [...outageKeys.all, 'dashboard'] as const,
  unreadCount: () => [...outageKeys.all, 'unread-count'] as const,
};

// ============================================
// List & Detail Queries
// ============================================

export function useOutages(query?: OutageListQuery) {
  return useQuery({
    queryKey: outageKeys.list(query),
    queryFn: () => api.listOutages(query),
    staleTime: 30_000,
  });
}

export function useActiveOutages() {
  return useQuery({
    queryKey: outageKeys.active(),
    queryFn: () => api.listActiveOutages(),
    staleTime: 30_000,
  });
}

export function useOutage(id: string) {
  return useQuery({
    queryKey: outageKeys.detail(id),
    queryFn: () => api.getOutage(id),
    enabled: !!id,
    staleTime: 60_000,
  });
}

export function useOutageStatistics() {
  return useQuery({
    queryKey: outageKeys.statistics(),
    queryFn: () => api.getOutageStatistics(),
    staleTime: 60_000,
  });
}

export function useOutageDashboard() {
  return useQuery({
    queryKey: outageKeys.dashboard(),
    queryFn: () => api.getOutageDashboard(),
    staleTime: 30_000,
  });
}

export function useUnreadOutagesCount() {
  return useQuery({
    queryKey: outageKeys.unreadCount(),
    queryFn: () => api.getUnreadCount(),
    staleTime: 30_000,
  });
}

// ============================================
// Create & Update Mutations
// ============================================

export function useCreateOutage() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateOutageRequest) => api.createOutage(data),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: outageKeys.lists(),
      });
      queryClient.invalidateQueries({
        queryKey: outageKeys.statistics(),
      });
      queryClient.invalidateQueries({
        queryKey: outageKeys.dashboard(),
      });
    },
  });
}

export function useUpdateOutage() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateOutageRequest }) =>
      api.updateOutage(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({
        queryKey: outageKeys.detail(id),
      });
      queryClient.invalidateQueries({
        queryKey: outageKeys.lists(),
      });
      queryClient.invalidateQueries({
        queryKey: outageKeys.dashboard(),
      });
    },
  });
}

export function useDeleteOutage() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.deleteOutage(id),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: outageKeys.lists(),
      });
      queryClient.invalidateQueries({
        queryKey: outageKeys.statistics(),
      });
      queryClient.invalidateQueries({
        queryKey: outageKeys.dashboard(),
      });
    },
  });
}

// ============================================
// Status Change Mutations
// ============================================

export function useStartOutage() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data?: StartOutageRequest }) =>
      api.startOutage(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({
        queryKey: outageKeys.detail(id),
      });
      queryClient.invalidateQueries({
        queryKey: outageKeys.lists(),
      });
      queryClient.invalidateQueries({
        queryKey: outageKeys.statistics(),
      });
      queryClient.invalidateQueries({
        queryKey: outageKeys.dashboard(),
      });
    },
  });
}

export function useResolveOutage() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data?: ResolveOutageRequest }) =>
      api.resolveOutage(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({
        queryKey: outageKeys.detail(id),
      });
      queryClient.invalidateQueries({
        queryKey: outageKeys.lists(),
      });
      queryClient.invalidateQueries({
        queryKey: outageKeys.statistics(),
      });
      queryClient.invalidateQueries({
        queryKey: outageKeys.dashboard(),
      });
    },
  });
}

export function useCancelOutage() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data?: CancelOutageRequest }) =>
      api.cancelOutage(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({
        queryKey: outageKeys.detail(id),
      });
      queryClient.invalidateQueries({
        queryKey: outageKeys.lists(),
      });
      queryClient.invalidateQueries({
        queryKey: outageKeys.statistics(),
      });
      queryClient.invalidateQueries({
        queryKey: outageKeys.dashboard(),
      });
    },
  });
}

// ============================================
// Read Tracking Mutation
// ============================================

export function useMarkOutageRead() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.markOutageRead(id),
    onSuccess: (_, id) => {
      queryClient.invalidateQueries({
        queryKey: outageKeys.detail(id),
      });
      queryClient.invalidateQueries({
        queryKey: outageKeys.unreadCount(),
      });
    },
  });
}
