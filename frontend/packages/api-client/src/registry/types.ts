/**
 * Building Registries Types
 *
 * Type definitions for the Building Registries API (Epic 57).
 */

// Pet types
export type PetType = 'dog' | 'cat' | 'bird' | 'fish' | 'reptile' | 'rodent' | 'other';
export type PetSize = 'small' | 'medium' | 'large' | 'extra_large';

// Vehicle types
export type VehicleType = 'car' | 'motorcycle' | 'bicycle' | 'scooter' | 'other';

// Registry status
export type RegistryStatus = 'pending' | 'approved' | 'rejected' | 'expired';

// ============================================================================
// Pet Registration Types
// ============================================================================

export interface PetRegistration {
  id: string;
  unitId: string;
  ownerId: string;
  petType: PetType;
  petSize: PetSize;
  name: string;
  breed?: string;
  age?: number;
  weight?: number;
  description?: string;
  veterinarianName?: string;
  veterinarianContact?: string;
  vaccinationRecordUrl?: string;
  registrationDocumentUrl?: string;
  photoUrl?: string;
  status: RegistryStatus;
  submittedAt: string;
  reviewedAt?: string;
  reviewedBy?: string;
  rejectionReason?: string;
  createdAt: string;
  updatedAt: string;
}

export interface PetRegistrationWithDetails extends PetRegistration {
  ownerName: string;
  ownerEmail: string;
  unitNumber: string;
  buildingName: string;
  reviewerName?: string;
}

export interface CreatePetRegistrationRequest {
  petType: PetType;
  petSize: PetSize;
  name: string;
  breed?: string;
  age?: number;
  weight?: number;
  description?: string;
  veterinarianName?: string;
  veterinarianContact?: string;
  vaccinationRecordUrl?: string;
  registrationDocumentUrl?: string;
  photoUrl?: string;
}

export interface UpdatePetRegistrationRequest {
  petType?: PetType;
  petSize?: PetSize;
  name?: string;
  breed?: string;
  age?: number;
  weight?: number;
  description?: string;
  veterinarianName?: string;
  veterinarianContact?: string;
  vaccinationRecordUrl?: string;
  registrationDocumentUrl?: string;
  photoUrl?: string;
}

// ============================================================================
// Vehicle Registration Types
// ============================================================================

export interface VehicleRegistration {
  id: string;
  unitId: string;
  ownerId: string;
  vehicleType: VehicleType;
  make: string;
  model: string;
  year?: number;
  color?: string;
  licensePlate: string;
  parkingSpotId?: string;
  registrationDocumentUrl?: string;
  photoUrl?: string;
  status: RegistryStatus;
  submittedAt: string;
  reviewedAt?: string;
  reviewedBy?: string;
  rejectionReason?: string;
  createdAt: string;
  updatedAt: string;
}

export interface VehicleRegistrationWithDetails extends VehicleRegistration {
  ownerName: string;
  ownerEmail: string;
  unitNumber: string;
  buildingName: string;
  parkingSpotNumber?: string;
  reviewerName?: string;
}

export interface CreateVehicleRegistrationRequest {
  vehicleType: VehicleType;
  make: string;
  model: string;
  year?: number;
  color?: string;
  licensePlate: string;
  parkingSpotId?: string;
  registrationDocumentUrl?: string;
  photoUrl?: string;
}

export interface UpdateVehicleRegistrationRequest {
  vehicleType?: VehicleType;
  make?: string;
  model?: string;
  year?: number;
  color?: string;
  licensePlate?: string;
  parkingSpotId?: string;
  registrationDocumentUrl?: string;
  photoUrl?: string;
}

// ============================================================================
// Parking Spot Types
// ============================================================================

export interface ParkingSpot {
  id: string;
  buildingId: string;
  spotNumber: string;
  level?: string;
  zone?: string;
  isAssigned: boolean;
  assignedVehicleId?: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateParkingSpotRequest {
  spotNumber: string;
  level?: string;
  zone?: string;
}

// ============================================================================
// Building Registry Rules
// ============================================================================

export interface BuildingRegistryRules {
  id: string;
  buildingId: string;
  petsAllowed: boolean;
  petsRequireApproval: boolean;
  maxPetsPerUnit?: number;
  allowedPetTypes?: PetType[];
  bannedPetBreeds?: string[];
  maxPetWeight?: number;
  vehiclesRequireApproval: boolean;
  maxVehiclesPerUnit?: number;
  notes?: string;
  createdAt: string;
  updatedAt: string;
}

export interface UpdateRegistryRulesRequest {
  petsAllowed?: boolean;
  petsRequireApproval?: boolean;
  maxPetsPerUnit?: number;
  allowedPetTypes?: PetType[];
  bannedPetBreeds?: string[];
  maxPetWeight?: number;
  vehiclesRequireApproval?: boolean;
  maxVehiclesPerUnit?: number;
  notes?: string;
}

// ============================================================================
// Review Request
// ============================================================================

export interface ReviewRegistrationRequest {
  approve: boolean;
  rejectionReason?: string;
}

// ============================================================================
// List/Filter Parameters
// ============================================================================

export interface ListRegistrationsParams {
  page?: number;
  pageSize?: number;
  status?: RegistryStatus;
  unitId?: string;
  ownerId?: string;
  search?: string;
}

export interface ListParkingSpotsParams {
  page?: number;
  pageSize?: number;
  available?: boolean;
  level?: string;
  zone?: string;
}

// ============================================================================
// Paginated Responses
// ============================================================================

export interface RegistryPaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  pageSize: number;
  totalPages: number;
}

export type PetRegistrationListResponse = RegistryPaginatedResponse<PetRegistrationWithDetails>;
export type VehicleRegistrationListResponse =
  RegistryPaginatedResponse<VehicleRegistrationWithDetails>;
export type ParkingSpotListResponse = RegistryPaginatedResponse<ParkingSpot>;

// ============================================================================
// Response Messages
// ============================================================================

export interface RegistryMessageResponse {
  message: string;
}

export interface CreateRegistrationResponse {
  id: string;
  message: string;
}
