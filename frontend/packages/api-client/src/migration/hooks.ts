/**
 * Migration Import React Hooks (Epic 90 - Frontend API Integration).
 * Originally from Epic 66, integrated as part of Epic 90.
 *
 * TanStack Query hooks for bulk data import functionality.
 */

import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import * as api from './api';
import type { RetryImportRequest, StartImportRequest } from './types';

// Query keys
export const migrationKeys = {
  all: ['migration'] as const,
  templates: () => [...migrationKeys.all, 'templates'] as const,
  template: (id: string) => [...migrationKeys.templates(), id] as const,
  jobs: () => [...migrationKeys.all, 'jobs'] as const,
  jobsList: (params?: { status?: string }) => [...migrationKeys.jobs(), 'list', params] as const,
  job: (id: string) => [...migrationKeys.jobs(), id] as const,
  jobProgress: (id: string) => [...migrationKeys.job(id), 'progress'] as const,
  jobErrors: (id: string) => [...migrationKeys.job(id), 'errors'] as const,
};

// Template hooks
export function useImportTemplates() {
  return useQuery({
    queryKey: migrationKeys.templates(),
    queryFn: () => api.listTemplates(),
  });
}

export function useImportTemplate(id: string) {
  return useQuery({
    queryKey: migrationKeys.template(id),
    queryFn: () => api.getTemplate(id),
    enabled: !!id,
  });
}

export function useDownloadTemplate() {
  return useMutation({
    mutationFn: ({ id, format }: { id: string; format: 'csv' | 'xlsx' }) =>
      api.downloadTemplate(id, format),
    onSuccess: (data) => {
      // Trigger download
      window.open(data.url, '_blank');
    },
  });
}

// Job hooks
export function useImportJobs(params?: { status?: string; limit?: number; offset?: number }) {
  return useQuery({
    queryKey: migrationKeys.jobsList(params),
    queryFn: () => api.listJobs(params),
  });
}

export function useImportJob(id: string) {
  return useQuery({
    queryKey: migrationKeys.job(id),
    queryFn: () => api.getJob(id),
    enabled: !!id,
  });
}

export function useImportJobProgress(id: string, enabled = true) {
  return useQuery({
    queryKey: migrationKeys.jobProgress(id),
    queryFn: () => api.getJobProgress(id),
    enabled: !!id && enabled,
    refetchInterval: (query) => {
      // Poll while job is in progress
      const status = query.state.data?.status;
      if (status === 'importing' || status === 'validating') {
        return 2000; // Poll every 2 seconds
      }
      return false;
    },
  });
}

export function useImportJobErrors(id: string) {
  return useQuery({
    queryKey: migrationKeys.jobErrors(id),
    queryFn: () => api.getJobErrors(id),
    enabled: !!id,
  });
}

// Upload hook
export function useUploadImportFile() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      templateId,
      file,
      onProgress,
    }: {
      templateId: string;
      file: File;
      onProgress?: (progress: number) => void;
    }) => api.uploadFile(templateId, file, onProgress),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: migrationKeys.jobs() });
    },
  });
}

// Import control hooks
export function useStartImport() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (request: StartImportRequest) => api.startImport(request),
    onSuccess: (_, { jobId }) => {
      queryClient.invalidateQueries({ queryKey: migrationKeys.job(jobId) });
      queryClient.invalidateQueries({ queryKey: migrationKeys.jobs() });
    },
  });
}

export function useRetryImport() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (request: RetryImportRequest) => api.retryImport(request),
    onSuccess: (_, { jobId }) => {
      queryClient.invalidateQueries({ queryKey: migrationKeys.job(jobId) });
      queryClient.invalidateQueries({ queryKey: migrationKeys.jobs() });
    },
  });
}

export function useCancelImport() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (jobId: string) => api.cancelImport(jobId),
    onSuccess: (_, jobId) => {
      queryClient.invalidateQueries({ queryKey: migrationKeys.job(jobId) });
      queryClient.invalidateQueries({ queryKey: migrationKeys.jobs() });
    },
  });
}
