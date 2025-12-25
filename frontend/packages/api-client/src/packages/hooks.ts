/**
 * Package & Visitor Management React Hooks
 *
 * TanStack Query hooks for packages and visitors (Epic 58).
 */

import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import type { ApiConfig } from '../index';
import { createPackagesApi, createVisitorsApi } from './api';
import type {
  CheckInVisitorRequest,
  CheckOutVisitorRequest,
  CreatePackageRequest,
  CreateVisitorRequest,
  ListPackagesParams,
  ListVisitorsParams,
  PickupPackageRequest,
  ReceivePackageRequest,
  UpdatePackageRequest,
  UpdatePackageSettingsRequest,
  UpdateVisitorRequest,
  UpdateVisitorSettingsRequest,
  VerifyAccessCodeRequest,
} from './types';

// ============================================================================
// Query Keys
// ============================================================================

export const packageKeys = {
  all: ['packages'] as const,
  lists: () => [...packageKeys.all, 'list'] as const,
  list: (params?: ListPackagesParams) => [...packageKeys.lists(), params] as const,
  details: () => [...packageKeys.all, 'detail'] as const,
  detail: (id: string) => [...packageKeys.details(), id] as const,
  settings: (buildingId: string) => [...packageKeys.all, 'settings', buildingId] as const,
  statistics: (buildingId: string) => [...packageKeys.all, 'statistics', buildingId] as const,
};

export const visitorKeys = {
  all: ['visitors'] as const,
  lists: () => [...visitorKeys.all, 'list'] as const,
  list: (params?: ListVisitorsParams) => [...visitorKeys.lists(), params] as const,
  details: () => [...visitorKeys.all, 'detail'] as const,
  detail: (id: string) => [...visitorKeys.details(), id] as const,
  settings: (buildingId: string) => [...visitorKeys.all, 'settings', buildingId] as const,
  statistics: (buildingId: string) => [...visitorKeys.all, 'statistics', buildingId] as const,
};

// ============================================================================
// Package Hooks
// ============================================================================

export const usePackages = (config: ApiConfig, params?: ListPackagesParams) => {
  const api = createPackagesApi(config);
  return useQuery({
    queryKey: packageKeys.list(params),
    queryFn: () => api.list(params),
  });
};

export const usePackage = (config: ApiConfig, id: string) => {
  const api = createPackagesApi(config);
  return useQuery({
    queryKey: packageKeys.detail(id),
    queryFn: () => api.get(id),
    enabled: !!id,
  });
};

export const usePackageSettings = (config: ApiConfig, buildingId: string) => {
  const api = createPackagesApi(config);
  return useQuery({
    queryKey: packageKeys.settings(buildingId),
    queryFn: () => api.getSettings(buildingId),
    enabled: !!buildingId,
  });
};

export const usePackageStatistics = (config: ApiConfig, buildingId: string) => {
  const api = createPackagesApi(config);
  return useQuery({
    queryKey: packageKeys.statistics(buildingId),
    queryFn: () => api.getStatistics(buildingId),
    enabled: !!buildingId,
  });
};

export const useCreatePackage = (config: ApiConfig) => {
  const api = createPackagesApi(config);
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: CreatePackageRequest) => api.create(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: packageKeys.lists() });
    },
  });
};

export const useUpdatePackage = (config: ApiConfig) => {
  const api = createPackagesApi(config);
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdatePackageRequest }) => api.update(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: packageKeys.detail(id) });
      queryClient.invalidateQueries({ queryKey: packageKeys.lists() });
    },
  });
};

export const useDeletePackage = (config: ApiConfig) => {
  const api = createPackagesApi(config);
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => api.delete(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: packageKeys.lists() });
    },
  });
};

export const useReceivePackage = (config: ApiConfig) => {
  const api = createPackagesApi(config);
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: ReceivePackageRequest }) =>
      api.receive(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: packageKeys.detail(id) });
      queryClient.invalidateQueries({ queryKey: packageKeys.lists() });
    },
  });
};

export const usePickupPackage = (config: ApiConfig) => {
  const api = createPackagesApi(config);
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: PickupPackageRequest }) => api.pickup(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: packageKeys.detail(id) });
      queryClient.invalidateQueries({ queryKey: packageKeys.lists() });
    },
  });
};

export const useUpdatePackageSettings = (config: ApiConfig) => {
  const api = createPackagesApi(config);
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      buildingId,
      data,
    }: {
      buildingId: string;
      data: UpdatePackageSettingsRequest;
    }) => api.updateSettings(buildingId, data),
    onSuccess: (_, { buildingId }) => {
      queryClient.invalidateQueries({ queryKey: packageKeys.settings(buildingId) });
    },
  });
};

// ============================================================================
// Visitor Hooks
// ============================================================================

export const useVisitors = (config: ApiConfig, params?: ListVisitorsParams) => {
  const api = createVisitorsApi(config);
  return useQuery({
    queryKey: visitorKeys.list(params),
    queryFn: () => api.list(params),
  });
};

export const useVisitor = (config: ApiConfig, id: string) => {
  const api = createVisitorsApi(config);
  return useQuery({
    queryKey: visitorKeys.detail(id),
    queryFn: () => api.get(id),
    enabled: !!id,
  });
};

export const useVisitorSettings = (config: ApiConfig, buildingId: string) => {
  const api = createVisitorsApi(config);
  return useQuery({
    queryKey: visitorKeys.settings(buildingId),
    queryFn: () => api.getSettings(buildingId),
    enabled: !!buildingId,
  });
};

export const useVisitorStatistics = (config: ApiConfig, buildingId: string) => {
  const api = createVisitorsApi(config);
  return useQuery({
    queryKey: visitorKeys.statistics(buildingId),
    queryFn: () => api.getStatistics(buildingId),
    enabled: !!buildingId,
  });
};

export const useCreateVisitor = (config: ApiConfig) => {
  const api = createVisitorsApi(config);
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: CreateVisitorRequest) => api.create(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: visitorKeys.lists() });
    },
  });
};

export const useUpdateVisitor = (config: ApiConfig) => {
  const api = createVisitorsApi(config);
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateVisitorRequest }) => api.update(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: visitorKeys.detail(id) });
      queryClient.invalidateQueries({ queryKey: visitorKeys.lists() });
    },
  });
};

export const useDeleteVisitor = (config: ApiConfig) => {
  const api = createVisitorsApi(config);
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => api.delete(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: visitorKeys.lists() });
    },
  });
};

export const useCheckInVisitor = (config: ApiConfig) => {
  const api = createVisitorsApi(config);
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: string; data?: CheckInVisitorRequest }) =>
      api.checkIn(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: visitorKeys.detail(id) });
      queryClient.invalidateQueries({ queryKey: visitorKeys.lists() });
    },
  });
};

export const useCheckOutVisitor = (config: ApiConfig) => {
  const api = createVisitorsApi(config);
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: string; data?: CheckOutVisitorRequest }) =>
      api.checkOut(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: visitorKeys.detail(id) });
      queryClient.invalidateQueries({ queryKey: visitorKeys.lists() });
    },
  });
};

export const useCancelVisitor = (config: ApiConfig) => {
  const api = createVisitorsApi(config);
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => api.cancel(id),
    onSuccess: (_, id) => {
      queryClient.invalidateQueries({ queryKey: visitorKeys.detail(id) });
      queryClient.invalidateQueries({ queryKey: visitorKeys.lists() });
    },
  });
};

export const useVerifyAccessCode = (config: ApiConfig) => {
  const api = createVisitorsApi(config);

  return useMutation({
    mutationFn: (data: VerifyAccessCodeRequest) => api.verifyCode(data),
  });
};

export const useUpdateVisitorSettings = (config: ApiConfig) => {
  const api = createVisitorsApi(config);
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      buildingId,
      data,
    }: {
      buildingId: string;
      data: UpdateVisitorSettingsRequest;
    }) => api.updateSettings(buildingId, data),
    onSuccess: (_, { buildingId }) => {
      queryClient.invalidateQueries({ queryKey: visitorKeys.settings(buildingId) });
    },
  });
};
