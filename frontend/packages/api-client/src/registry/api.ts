/**
 * Building Registries API Client
 *
 * API functions for managing building registries - pets and vehicles (Epic 57).
 */

import type { ApiConfig } from '../index';
import type {
  BuildingRegistryRules,
  CreateParkingSpotRequest,
  CreatePetRegistrationRequest,
  CreateRegistrationResponse,
  CreateVehicleRegistrationRequest,
  ListParkingSpotsParams,
  ListRegistrationsParams,
  ParkingSpot,
  ParkingSpotListResponse,
  PetRegistrationListResponse,
  PetRegistrationWithDetails,
  RegistryMessageResponse,
  ReviewRegistrationRequest,
  UpdatePetRegistrationRequest,
  UpdateRegistryRulesRequest,
  UpdateVehicleRegistrationRequest,
  VehicleRegistrationListResponse,
  VehicleRegistrationWithDetails,
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

export const createRegistryApi = (config: ApiConfig) => {
  const baseUrl = `${config.baseUrl}/api/v1/registries`;
  const headers = buildHeaders(config);

  return {
    // ========================================================================
    // Pet Registration Operations (Story 57.1, 57.2, 57.3)
    // ========================================================================

    pets: {
      /**
       * List all pet registrations (managers)
       */
      list: async (params?: ListRegistrationsParams): Promise<PetRegistrationListResponse> => {
        const searchParams = new URLSearchParams();
        if (params?.page) searchParams.set('page', params.page.toString());
        if (params?.pageSize) searchParams.set('pageSize', params.pageSize.toString());
        if (params?.status) searchParams.set('status', params.status);
        if (params?.unitId) searchParams.set('unitId', params.unitId);
        if (params?.ownerId) searchParams.set('ownerId', params.ownerId);
        if (params?.search) searchParams.set('search', params.search);

        const url = searchParams.toString() ? `${baseUrl}/pets?${searchParams}` : `${baseUrl}/pets`;
        const response = await fetch(url, { headers });
        return handleResponse(response);
      },

      /**
       * Get pet registration details
       */
      get: async (id: string): Promise<PetRegistrationWithDetails> => {
        const response = await fetch(`${baseUrl}/pets/${id}`, { headers });
        return handleResponse(response);
      },

      /**
       * Create pet registration
       */
      create: async (data: CreatePetRegistrationRequest): Promise<CreateRegistrationResponse> => {
        const response = await fetch(`${baseUrl}/pets`, {
          method: 'POST',
          headers,
          body: JSON.stringify(data),
        });
        return handleResponse(response);
      },

      /**
       * Update pet registration (pending only)
       */
      update: async (
        id: string,
        data: UpdatePetRegistrationRequest
      ): Promise<RegistryMessageResponse> => {
        const response = await fetch(`${baseUrl}/pets/${id}`, {
          method: 'PUT',
          headers,
          body: JSON.stringify(data),
        });
        return handleResponse(response);
      },

      /**
       * Delete pet registration (pending only)
       */
      delete: async (id: string): Promise<void> => {
        const response = await fetch(`${baseUrl}/pets/${id}`, {
          method: 'DELETE',
          headers,
        });
        if (!response.ok) {
          const error = await response.json().catch(() => ({ message: 'Unknown error' }));
          throw new Error(error.message || `HTTP ${response.status}`);
        }
      },

      /**
       * Review pet registration (approve/reject - managers only)
       */
      review: async (
        id: string,
        data: ReviewRegistrationRequest
      ): Promise<RegistryMessageResponse> => {
        const response = await fetch(`${baseUrl}/pets/${id}/review`, {
          method: 'POST',
          headers,
          body: JSON.stringify(data),
        });
        return handleResponse(response);
      },

      /**
       * List my pet registrations (resident view)
       */
      listMy: async (params?: {
        page?: number;
        pageSize?: number;
        status?: string;
      }): Promise<PetRegistrationListResponse> => {
        const searchParams = new URLSearchParams();
        if (params?.page) searchParams.set('page', params.page.toString());
        if (params?.pageSize) searchParams.set('pageSize', params.pageSize.toString());
        if (params?.status) searchParams.set('status', params.status);

        const url = searchParams.toString()
          ? `${baseUrl}/pets/my?${searchParams}`
          : `${baseUrl}/pets/my`;
        const response = await fetch(url, { headers });
        return handleResponse(response);
      },
    },

    // ========================================================================
    // Vehicle Registration Operations (Story 57.4, 57.5)
    // ========================================================================

    vehicles: {
      /**
       * List all vehicle registrations (managers)
       */
      list: async (params?: ListRegistrationsParams): Promise<VehicleRegistrationListResponse> => {
        const searchParams = new URLSearchParams();
        if (params?.page) searchParams.set('page', params.page.toString());
        if (params?.pageSize) searchParams.set('pageSize', params.pageSize.toString());
        if (params?.status) searchParams.set('status', params.status);
        if (params?.unitId) searchParams.set('unitId', params.unitId);
        if (params?.ownerId) searchParams.set('ownerId', params.ownerId);
        if (params?.search) searchParams.set('search', params.search);

        const url = searchParams.toString()
          ? `${baseUrl}/vehicles?${searchParams}`
          : `${baseUrl}/vehicles`;
        const response = await fetch(url, { headers });
        return handleResponse(response);
      },

      /**
       * Get vehicle registration details
       */
      get: async (id: string): Promise<VehicleRegistrationWithDetails> => {
        const response = await fetch(`${baseUrl}/vehicles/${id}`, { headers });
        return handleResponse(response);
      },

      /**
       * Create vehicle registration
       */
      create: async (
        data: CreateVehicleRegistrationRequest
      ): Promise<CreateRegistrationResponse> => {
        const response = await fetch(`${baseUrl}/vehicles`, {
          method: 'POST',
          headers,
          body: JSON.stringify(data),
        });
        return handleResponse(response);
      },

      /**
       * Update vehicle registration (pending only)
       */
      update: async (
        id: string,
        data: UpdateVehicleRegistrationRequest
      ): Promise<RegistryMessageResponse> => {
        const response = await fetch(`${baseUrl}/vehicles/${id}`, {
          method: 'PUT',
          headers,
          body: JSON.stringify(data),
        });
        return handleResponse(response);
      },

      /**
       * Delete vehicle registration (pending only)
       */
      delete: async (id: string): Promise<void> => {
        const response = await fetch(`${baseUrl}/vehicles/${id}`, {
          method: 'DELETE',
          headers,
        });
        if (!response.ok) {
          const error = await response.json().catch(() => ({ message: 'Unknown error' }));
          throw new Error(error.message || `HTTP ${response.status}`);
        }
      },

      /**
       * Review vehicle registration (approve/reject - managers only)
       */
      review: async (
        id: string,
        data: ReviewRegistrationRequest
      ): Promise<RegistryMessageResponse> => {
        const response = await fetch(`${baseUrl}/vehicles/${id}/review`, {
          method: 'POST',
          headers,
          body: JSON.stringify(data),
        });
        return handleResponse(response);
      },

      /**
       * List my vehicle registrations (resident view)
       */
      listMy: async (params?: {
        page?: number;
        pageSize?: number;
        status?: string;
      }): Promise<VehicleRegistrationListResponse> => {
        const searchParams = new URLSearchParams();
        if (params?.page) searchParams.set('page', params.page.toString());
        if (params?.pageSize) searchParams.set('pageSize', params.pageSize.toString());
        if (params?.status) searchParams.set('status', params.status);

        const url = searchParams.toString()
          ? `${baseUrl}/vehicles/my?${searchParams}`
          : `${baseUrl}/vehicles/my`;
        const response = await fetch(url, { headers });
        return handleResponse(response);
      },
    },

    // ========================================================================
    // Parking Spot Operations (Story 57.6)
    // ========================================================================

    parkingSpots: {
      /**
       * List parking spots in building
       */
      list: async (
        buildingId: string,
        params?: ListParkingSpotsParams
      ): Promise<ParkingSpotListResponse> => {
        const searchParams = new URLSearchParams();
        if (params?.page) searchParams.set('page', params.page.toString());
        if (params?.pageSize) searchParams.set('pageSize', params.pageSize.toString());
        if (params?.available !== undefined)
          searchParams.set('available', params.available.toString());
        if (params?.level) searchParams.set('level', params.level);
        if (params?.zone) searchParams.set('zone', params.zone);

        const url = searchParams.toString()
          ? `${baseUrl}/buildings/${buildingId}/parking-spots?${searchParams}`
          : `${baseUrl}/buildings/${buildingId}/parking-spots`;
        const response = await fetch(url, { headers });
        return handleResponse(response);
      },

      /**
       * Create parking spot
       */
      create: async (buildingId: string, data: CreateParkingSpotRequest): Promise<ParkingSpot> => {
        const response = await fetch(`${baseUrl}/buildings/${buildingId}/parking-spots`, {
          method: 'POST',
          headers,
          body: JSON.stringify(data),
        });
        return handleResponse(response);
      },

      /**
       * Delete parking spot (unassigned only)
       */
      delete: async (buildingId: string, spotId: string): Promise<void> => {
        const response = await fetch(`${baseUrl}/buildings/${buildingId}/parking-spots/${spotId}`, {
          method: 'DELETE',
          headers,
        });
        if (!response.ok) {
          const error = await response.json().catch(() => ({ message: 'Unknown error' }));
          throw new Error(error.message || `HTTP ${response.status}`);
        }
      },
    },

    // ========================================================================
    // Building Registry Rules (Story 57.7)
    // ========================================================================

    rules: {
      /**
       * Get registry rules for building
       */
      get: async (buildingId: string): Promise<BuildingRegistryRules> => {
        const response = await fetch(`${baseUrl}/buildings/${buildingId}/rules`, { headers });
        return handleResponse(response);
      },

      /**
       * Update registry rules (managers only)
       */
      update: async (
        buildingId: string,
        data: UpdateRegistryRulesRequest
      ): Promise<BuildingRegistryRules> => {
        const response = await fetch(`${baseUrl}/buildings/${buildingId}/rules`, {
          method: 'PUT',
          headers,
          body: JSON.stringify(data),
        });
        return handleResponse(response);
      },
    },
  };
};

export type RegistryApi = ReturnType<typeof createRegistryApi>;
