/**
 * Import Hooks
 *
 * React Query hooks for property import functionality (Epic 46).
 */

import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import type {
  ColumnMapping,
  CreateCrmConnectionRequest,
  CreateFeedSourceRequest,
  CrmConnection,
  CrmConnectionTestResult,
  CrmCredentials,
  CsvImportPreview,
  CsvImportResult,
  FeedPreview,
  FeedSource,
  SyncHistoryItem,
  SyncSchedule,
  UpdateCrmConnectionRequest,
  UpdateFeedSourceRequest,
  UpdateSyncScheduleRequest,
} from './types';

const API_BASE = process.env.NEXT_PUBLIC_REALITY_API_URL || '/api/v1';

async function fetchApi<T>(path: string, options?: RequestInit): Promise<T> {
  const response = await fetch(`${API_BASE}${path}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
    credentials: 'include',
  });

  if (!response.ok) {
    let errorMessage = `API error: ${response.status}`;
    try {
      const errorBody = await response.json();
      if (errorBody?.message) {
        errorMessage += ` - ${errorBody.message}`;
      }
    } catch {
      // Ignore parsing errors
    }
    throw new Error(errorMessage);
  }

  return response.json();
}

// CSV Import Hooks
export function useCsvPreview(agencyId: string, file: File | null) {
  return useQuery({
    queryKey: ['csv-preview', agencyId, file?.name],
    queryFn: async () => {
      if (!file) throw new Error('No file provided');
      const formData = new FormData();
      formData.append('file', file);

      const response = await fetch(`${API_BASE}/agencies/${agencyId}/import/csv/preview`, {
        method: 'POST',
        body: formData,
        credentials: 'include',
      });

      if (!response.ok) {
        throw new Error(`Failed to preview CSV: ${response.status}`);
      }

      return response.json() as Promise<CsvImportPreview>;
    },
    enabled: !!agencyId && !!file,
    staleTime: 0,
  });
}

export function useCsvImport(agencyId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      file,
      mapping,
      skipInvalid,
    }: {
      file: File;
      mapping: ColumnMapping;
      skipInvalid: boolean;
    }) => {
      const formData = new FormData();
      formData.append('file', file);
      formData.append('mapping', JSON.stringify(mapping));
      formData.append('skipInvalid', String(skipInvalid));

      const response = await fetch(`${API_BASE}/agencies/${agencyId}/import/csv`, {
        method: 'POST',
        body: formData,
        credentials: 'include',
      });

      if (!response.ok) {
        throw new Error(`Failed to import CSV: ${response.status}`);
      }

      return response.json() as Promise<CsvImportResult>;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['agency-listings', agencyId] });
    },
  });
}

// CRM Connection Hooks
export function useCrmConnections(agencyId: string) {
  return useQuery({
    queryKey: ['crm-connections', agencyId],
    queryFn: () => fetchApi<CrmConnection[]>(`/agencies/${agencyId}/crm-connections`),
    enabled: !!agencyId,
  });
}

export function useCrmConnection(agencyId: string, connectionId: string) {
  return useQuery({
    queryKey: ['crm-connection', agencyId, connectionId],
    queryFn: () => fetchApi<CrmConnection>(`/agencies/${agencyId}/crm-connections/${connectionId}`),
    enabled: !!agencyId && !!connectionId,
  });
}

export function useTestCrmConnection(agencyId: string) {
  return useMutation({
    mutationFn: async (credentials: CrmCredentials & { provider: string }) => {
      return fetchApi<CrmConnectionTestResult>(`/agencies/${agencyId}/crm-connections/test`, {
        method: 'POST',
        body: JSON.stringify(credentials),
      });
    },
  });
}

export function useCreateCrmConnection(agencyId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (request: CreateCrmConnectionRequest) => {
      return fetchApi<CrmConnection>(`/agencies/${agencyId}/crm-connections`, {
        method: 'POST',
        body: JSON.stringify(request),
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['crm-connections', agencyId] });
    },
  });
}

export function useUpdateCrmConnection(agencyId: string, connectionId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (request: UpdateCrmConnectionRequest) => {
      return fetchApi<CrmConnection>(`/agencies/${agencyId}/crm-connections/${connectionId}`, {
        method: 'PATCH',
        body: JSON.stringify(request),
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['crm-connections', agencyId] });
      queryClient.invalidateQueries({ queryKey: ['crm-connection', agencyId, connectionId] });
    },
  });
}

export function useDeleteCrmConnection(agencyId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (connectionId: string) => {
      await fetchApi(`/agencies/${agencyId}/crm-connections/${connectionId}`, {
        method: 'DELETE',
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['crm-connections', agencyId] });
    },
  });
}

export function useSyncCrmConnection(agencyId: string, connectionId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async () => {
      return fetchApi<SyncHistoryItem>(
        `/agencies/${agencyId}/crm-connections/${connectionId}/sync`,
        { method: 'POST' }
      );
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['crm-connection', agencyId, connectionId] });
      queryClient.invalidateQueries({ queryKey: ['sync-history', agencyId, connectionId] });
      queryClient.invalidateQueries({ queryKey: ['agency-listings', agencyId] });
    },
  });
}

// Sync Schedule Hooks
export function useSyncSchedule(agencyId: string, connectionId: string) {
  return useQuery({
    queryKey: ['sync-schedule', agencyId, connectionId],
    queryFn: () =>
      fetchApi<SyncSchedule>(`/agencies/${agencyId}/crm-connections/${connectionId}/schedule`),
    enabled: !!agencyId && !!connectionId,
  });
}

export function useUpdateSyncSchedule(agencyId: string, connectionId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (request: UpdateSyncScheduleRequest) => {
      return fetchApi<SyncSchedule>(
        `/agencies/${agencyId}/crm-connections/${connectionId}/schedule`,
        {
          method: 'PUT',
          body: JSON.stringify(request),
        }
      );
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['sync-schedule', agencyId, connectionId],
      });
    },
  });
}

export function useSyncHistory(agencyId: string, connectionId: string, limit = 10) {
  return useQuery({
    queryKey: ['sync-history', agencyId, connectionId, limit],
    queryFn: () =>
      fetchApi<SyncHistoryItem[]>(
        `/agencies/${agencyId}/crm-connections/${connectionId}/sync-history?limit=${limit}`
      ),
    enabled: !!agencyId && !!connectionId,
  });
}

// Feed Source Hooks
export function useFeedSources(agencyId: string) {
  return useQuery({
    queryKey: ['feed-sources', agencyId],
    queryFn: () => fetchApi<FeedSource[]>(`/agencies/${agencyId}/feeds`),
    enabled: !!agencyId,
  });
}

export function useFeedSource(agencyId: string, feedId: string) {
  return useQuery({
    queryKey: ['feed-source', agencyId, feedId],
    queryFn: () => fetchApi<FeedSource>(`/agencies/${agencyId}/feeds/${feedId}`),
    enabled: !!agencyId && !!feedId,
  });
}

export function useFeedPreview(agencyId: string) {
  return useMutation({
    mutationFn: async (url: string) => {
      return fetchApi<FeedPreview>(`/agencies/${agencyId}/feeds/preview`, {
        method: 'POST',
        body: JSON.stringify({ url }),
      });
    },
  });
}

export function useCreateFeedSource(agencyId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (request: CreateFeedSourceRequest) => {
      return fetchApi<FeedSource>(`/agencies/${agencyId}/feeds`, {
        method: 'POST',
        body: JSON.stringify(request),
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['feed-sources', agencyId] });
    },
  });
}

export function useUpdateFeedSource(agencyId: string, feedId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (request: UpdateFeedSourceRequest) => {
      return fetchApi<FeedSource>(`/agencies/${agencyId}/feeds/${feedId}`, {
        method: 'PATCH',
        body: JSON.stringify(request),
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['feed-sources', agencyId] });
      queryClient.invalidateQueries({ queryKey: ['feed-source', agencyId, feedId] });
    },
  });
}

export function useDeleteFeedSource(agencyId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (feedId: string) => {
      await fetchApi(`/agencies/${agencyId}/feeds/${feedId}`, {
        method: 'DELETE',
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['feed-sources', agencyId] });
    },
  });
}

export function useSyncFeedSource(agencyId: string, feedId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async () => {
      return fetchApi<SyncHistoryItem>(`/agencies/${agencyId}/feeds/${feedId}/sync`, {
        method: 'POST',
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['feed-source', agencyId, feedId] });
      queryClient.invalidateQueries({ queryKey: ['feed-sync-history', agencyId, feedId] });
      queryClient.invalidateQueries({ queryKey: ['agency-listings', agencyId] });
    },
  });
}

export function useFeedSyncHistory(agencyId: string, feedId: string, limit = 10) {
  return useQuery({
    queryKey: ['feed-sync-history', agencyId, feedId, limit],
    queryFn: () =>
      fetchApi<SyncHistoryItem[]>(
        `/agencies/${agencyId}/feeds/${feedId}/sync-history?limit=${limit}`
      ),
    enabled: !!agencyId && !!feedId,
  });
}
