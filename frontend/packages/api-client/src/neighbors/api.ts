/**
 * Neighbor API Client
 *
 * API functions for neighbor information (Epic 6, Story 6.6).
 */

import type { ApiConfig } from '../index';
import type {
  NeighborsResponse,
  PrivacySettingsResponse,
  UpdatePrivacySettingsRequest,
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

export const createNeighborsApi = (config: ApiConfig) => {
  const baseUrl = config.baseUrl;
  const headers = buildHeaders(config);

  return {
    // ========================================================================
    // Neighbor Operations
    // ========================================================================

    /**
     * List neighbors in the same building
     */
    listNeighbors: async (buildingId: string): Promise<NeighborsResponse> => {
      const response = await fetch(`${baseUrl}/api/v1/buildings/${buildingId}/neighbors`, {
        headers,
      });
      return handleResponse(response);
    },

    // ========================================================================
    // Privacy Settings
    // ========================================================================

    /**
     * Get current user's privacy settings
     */
    getPrivacySettings: async (): Promise<PrivacySettingsResponse> => {
      const response = await fetch(`${baseUrl}/api/v1/users/me/privacy`, { headers });
      return handleResponse(response);
    },

    /**
     * Update current user's privacy settings
     */
    updatePrivacySettings: async (
      data: UpdatePrivacySettingsRequest
    ): Promise<PrivacySettingsResponse> => {
      const response = await fetch(`${baseUrl}/api/v1/users/me/privacy`, {
        method: 'PUT',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },
  };
};

export type NeighborsApi = ReturnType<typeof createNeighborsApi>;
