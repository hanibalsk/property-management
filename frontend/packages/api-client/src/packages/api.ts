/**
 * Package & Visitor Management API Client
 *
 * API functions for managing packages and visitors (Epic 58).
 */

import type { ApiConfig } from '../index';
import type {
  AccessCodeVerification,
  CheckInVisitorRequest,
  CheckOutVisitorRequest,
  CreatePackageRequest,
  CreateVisitorRequest,
  ListPackagesParams,
  ListVisitorsParams,
  Package,
  PackageDetailResponse,
  PackageListResponse,
  PackageSettingsResponse,
  PackageStatisticsResponse,
  PackageWithDetails,
  PickupPackageRequest,
  ReceivePackageRequest,
  UpdatePackageRequest,
  UpdatePackageSettingsRequest,
  UpdateVisitorRequest,
  UpdateVisitorSettingsRequest,
  VerifyAccessCodeRequest,
  Visitor,
  VisitorDetailResponse,
  VisitorListResponse,
  VisitorSettingsResponse,
  VisitorStatisticsResponse,
  VisitorWithDetails,
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

// ============================================================================
// Packages API
// ============================================================================

export const createPackagesApi = (config: ApiConfig) => {
  const baseUrl = `${config.baseUrl}/api/v1/packages`;
  const headers = buildHeaders(config);

  return {
    // ========================================================================
    // Package CRUD Operations (Story 58.1, 58.2, 58.3)
    // ========================================================================

    /**
     * Register an expected package (Story 58.1)
     */
    create: async (data: CreatePackageRequest): Promise<{ message: string; package: Package }> => {
      const response = await fetch(baseUrl, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * List packages with filters
     */
    list: async (params?: ListPackagesParams): Promise<PackageListResponse> => {
      const searchParams = new URLSearchParams();
      if (params?.buildingId) searchParams.set('building_id', params.buildingId);
      if (params?.unitId) searchParams.set('unit_id', params.unitId);
      if (params?.residentId) searchParams.set('resident_id', params.residentId);
      if (params?.status) searchParams.set('status', params.status);
      if (params?.carrier) searchParams.set('carrier', params.carrier);
      if (params?.fromDate) searchParams.set('from_date', params.fromDate);
      if (params?.toDate) searchParams.set('to_date', params.toDate);
      if (params?.limit) searchParams.set('limit', params.limit.toString());
      if (params?.offset) searchParams.set('offset', params.offset.toString());

      const url = searchParams.toString() ? `${baseUrl}?${searchParams}` : baseUrl;
      const response = await fetch(url, { headers });
      return handleResponse(response);
    },

    /**
     * Get package details
     */
    get: async (id: string): Promise<PackageWithDetails> => {
      const response = await fetch(`${baseUrl}/${id}`, { headers });
      const data: PackageDetailResponse = await handleResponse(response);
      return data.package;
    },

    /**
     * Update a package
     */
    update: async (
      id: string,
      data: UpdatePackageRequest
    ): Promise<{ message: string; package: Package }> => {
      const response = await fetch(`${baseUrl}/${id}`, {
        method: 'PUT',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Delete a package
     */
    delete: async (id: string): Promise<void> => {
      const response = await fetch(`${baseUrl}/${id}`, {
        method: 'DELETE',
        headers,
      });
      if (!response.ok) {
        const error = await response.json().catch(() => ({ message: 'Unknown error' }));
        throw new Error(error.message || `HTTP ${response.status}`);
      }
    },

    /**
     * Log package arrival (Story 58.2)
     */
    receive: async (
      id: string,
      data: ReceivePackageRequest
    ): Promise<{ message: string; package: Package }> => {
      const response = await fetch(`${baseUrl}/${id}/receive`, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Log package pickup (Story 58.3)
     */
    pickup: async (
      id: string,
      data: PickupPackageRequest
    ): Promise<{ message: string; package: Package }> => {
      const response = await fetch(`${baseUrl}/${id}/pickup`, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    // ========================================================================
    // Settings & Statistics
    // ========================================================================

    /**
     * Get building package settings
     */
    getSettings: async (buildingId: string): Promise<PackageSettingsResponse> => {
      const response = await fetch(`${baseUrl}/buildings/${buildingId}/settings`, { headers });
      return handleResponse(response);
    },

    /**
     * Update building package settings
     */
    updateSettings: async (
      buildingId: string,
      data: UpdatePackageSettingsRequest
    ): Promise<PackageSettingsResponse> => {
      const response = await fetch(`${baseUrl}/buildings/${buildingId}/settings`, {
        method: 'PUT',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Get package statistics for a building
     */
    getStatistics: async (buildingId: string): Promise<PackageStatisticsResponse> => {
      const response = await fetch(`${baseUrl}/buildings/${buildingId}/statistics`, { headers });
      return handleResponse(response);
    },
  };
};

// ============================================================================
// Visitors API
// ============================================================================

export const createVisitorsApi = (config: ApiConfig) => {
  const baseUrl = `${config.baseUrl}/api/v1/visitors`;
  const headers = buildHeaders(config);

  return {
    // ========================================================================
    // Visitor CRUD Operations (Story 58.4, 58.5)
    // ========================================================================

    /**
     * Pre-register a visitor (Story 58.4)
     */
    create: async (data: CreateVisitorRequest): Promise<{ message: string; visitor: Visitor }> => {
      const response = await fetch(baseUrl, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * List visitors with filters (Story 58.5)
     */
    list: async (params?: ListVisitorsParams): Promise<VisitorListResponse> => {
      const searchParams = new URLSearchParams();
      if (params?.buildingId) searchParams.set('building_id', params.buildingId);
      if (params?.unitId) searchParams.set('unit_id', params.unitId);
      if (params?.hostId) searchParams.set('host_id', params.hostId);
      if (params?.status) searchParams.set('status', params.status);
      if (params?.purpose) searchParams.set('purpose', params.purpose);
      if (params?.fromDate) searchParams.set('from_date', params.fromDate);
      if (params?.toDate) searchParams.set('to_date', params.toDate);
      if (params?.todayOnly) searchParams.set('today_only', 'true');
      if (params?.limit) searchParams.set('limit', params.limit.toString());
      if (params?.offset) searchParams.set('offset', params.offset.toString());

      const url = searchParams.toString() ? `${baseUrl}?${searchParams}` : baseUrl;
      const response = await fetch(url, { headers });
      return handleResponse(response);
    },

    /**
     * Get visitor details
     */
    get: async (id: string): Promise<VisitorWithDetails> => {
      const response = await fetch(`${baseUrl}/${id}`, { headers });
      const data: VisitorDetailResponse = await handleResponse(response);
      return data.visitor;
    },

    /**
     * Update a visitor registration
     */
    update: async (
      id: string,
      data: UpdateVisitorRequest
    ): Promise<{ message: string; visitor: Visitor }> => {
      const response = await fetch(`${baseUrl}/${id}`, {
        method: 'PUT',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Delete/cancel a visitor registration
     */
    delete: async (id: string): Promise<void> => {
      const response = await fetch(`${baseUrl}/${id}`, {
        method: 'DELETE',
        headers,
      });
      if (!response.ok) {
        const error = await response.json().catch(() => ({ message: 'Unknown error' }));
        throw new Error(error.message || `HTTP ${response.status}`);
      }
    },

    /**
     * Check in a visitor
     */
    checkIn: async (
      id: string,
      data?: CheckInVisitorRequest
    ): Promise<{ message: string; visitor: Visitor }> => {
      const response = await fetch(`${baseUrl}/${id}/check-in`, {
        method: 'POST',
        headers,
        body: JSON.stringify(data || {}),
      });
      return handleResponse(response);
    },

    /**
     * Check out a visitor
     */
    checkOut: async (
      id: string,
      data?: CheckOutVisitorRequest
    ): Promise<{ message: string; visitor: Visitor }> => {
      const response = await fetch(`${baseUrl}/${id}/check-out`, {
        method: 'POST',
        headers,
        body: JSON.stringify(data || {}),
      });
      return handleResponse(response);
    },

    /**
     * Cancel a visitor registration
     */
    cancel: async (id: string): Promise<{ message: string; visitor: Visitor }> => {
      const response = await fetch(`${baseUrl}/${id}/cancel`, {
        method: 'POST',
        headers,
      });
      return handleResponse(response);
    },

    /**
     * Verify an access code
     */
    verifyCode: async (data: VerifyAccessCodeRequest): Promise<AccessCodeVerification> => {
      const response = await fetch(`${baseUrl}/verify-code`, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    // ========================================================================
    // Settings & Statistics
    // ========================================================================

    /**
     * Get building visitor settings
     */
    getSettings: async (buildingId: string): Promise<VisitorSettingsResponse> => {
      const response = await fetch(`${baseUrl}/buildings/${buildingId}/settings`, { headers });
      return handleResponse(response);
    },

    /**
     * Update building visitor settings
     */
    updateSettings: async (
      buildingId: string,
      data: UpdateVisitorSettingsRequest
    ): Promise<VisitorSettingsResponse> => {
      const response = await fetch(`${baseUrl}/buildings/${buildingId}/settings`, {
        method: 'PUT',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Get visitor statistics for a building
     */
    getStatistics: async (buildingId: string): Promise<VisitorStatisticsResponse> => {
      const response = await fetch(`${baseUrl}/buildings/${buildingId}/statistics`, { headers });
      return handleResponse(response);
    },
  };
};

export type PackagesApi = ReturnType<typeof createPackagesApi>;
export type VisitorsApi = ReturnType<typeof createVisitorsApi>;
