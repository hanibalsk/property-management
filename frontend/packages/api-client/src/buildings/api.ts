/**
 * Buildings API Client
 *
 * API functions for managing buildings (UC-15).
 */

import type { ApiConfig } from '../index';
import type {
  Building,
  BuildingDocument,
  BuildingsPaginatedResponse,
  CommonArea,
  CreateBuildingRequest,
  CreateCommonAreaRequest,
  CreateFloorRequest,
  Floor,
  ListBuildingDocumentsParams,
  ListBuildingsParams,
  UpdateBuildingRequest,
  UploadDocumentRequest,
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

/**
 * Creates a buildings API client.
 *
 * @param config - API configuration including base URL and auth token
 * @returns Buildings API methods
 */
export const createBuildingsApi = (config: ApiConfig) => {
  const baseUrl = `${config.baseUrl}/api/v1/buildings`;
  const headers = buildHeaders(config);

  return {
    /**
     * List buildings with optional filters.
     */
    list: async (params?: ListBuildingsParams): Promise<BuildingsPaginatedResponse<Building>> => {
      const searchParams = new URLSearchParams();
      if (params?.page) searchParams.set('page', params.page.toString());
      if (params?.pageSize) searchParams.set('pageSize', params.pageSize.toString());
      if (params?.status) searchParams.set('status', params.status);
      if (params?.type) searchParams.set('type', params.type);

      const url = searchParams.toString() ? `${baseUrl}?${searchParams}` : baseUrl;
      const response = await fetch(url, { headers });
      return handleResponse(response);
    },

    /**
     * Create a new building.
     */
    create: async (data: CreateBuildingRequest): Promise<Building> => {
      const response = await fetch(baseUrl, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Get building by ID.
     */
    get: async (id: string): Promise<Building> => {
      const response = await fetch(`${baseUrl}/${id}`, { headers });
      return handleResponse(response);
    },

    /**
     * Update a building.
     */
    update: async (id: string, data: UpdateBuildingRequest): Promise<Building> => {
      const response = await fetch(`${baseUrl}/${id}`, {
        method: 'PATCH',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * Delete a building.
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
     * List floors in a building.
     */
    listFloors: async (buildingId: string): Promise<Floor[]> => {
      const response = await fetch(`${baseUrl}/${buildingId}/floors`, { headers });
      return handleResponse(response);
    },

    /**
     * Create a floor in a building.
     */
    createFloor: async (buildingId: string, data: CreateFloorRequest): Promise<Floor> => {
      const response = await fetch(`${baseUrl}/${buildingId}/floors`, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * List common areas in a building.
     */
    listCommonAreas: async (buildingId: string): Promise<CommonArea[]> => {
      const response = await fetch(`${baseUrl}/${buildingId}/common-areas`, { headers });
      return handleResponse(response);
    },

    /**
     * Create a common area in a building.
     */
    createCommonArea: async (
      buildingId: string,
      data: CreateCommonAreaRequest
    ): Promise<CommonArea> => {
      const response = await fetch(`${baseUrl}/${buildingId}/common-areas`, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    /**
     * List documents for a building.
     */
    listDocuments: async (
      buildingId: string,
      params?: ListBuildingDocumentsParams
    ): Promise<BuildingsPaginatedResponse<BuildingDocument>> => {
      const searchParams = new URLSearchParams();
      if (params?.page) searchParams.set('page', params.page.toString());
      if (params?.pageSize) searchParams.set('pageSize', params.pageSize.toString());
      if (params?.category) searchParams.set('category', params.category);

      const url = searchParams.toString()
        ? `${baseUrl}/${buildingId}/documents?${searchParams}`
        : `${baseUrl}/${buildingId}/documents`;
      const response = await fetch(url, { headers });
      return handleResponse(response);
    },

    /**
     * Upload a document to a building.
     */
    uploadDocument: async (
      buildingId: string,
      data: UploadDocumentRequest
    ): Promise<BuildingDocument> => {
      const response = await fetch(`${baseUrl}/${buildingId}/documents`, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },
  };
};

export type BuildingsApi = ReturnType<typeof createBuildingsApi>;
