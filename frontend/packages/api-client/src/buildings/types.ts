/**
 * Buildings Types
 *
 * Type definitions for the Buildings API (UC-15).
 */

/** Building type */
export type BuildingType = 'residential' | 'commercial' | 'mixed' | 'industrial';

/** Building status */
export type BuildingStatus = 'active' | 'under_construction' | 'renovation' | 'inactive';

/** Common area type */
export type CommonAreaType =
  | 'staircase'
  | 'elevator'
  | 'lobby'
  | 'hallway'
  | 'basement'
  | 'attic'
  | 'parking'
  | 'garage'
  | 'garden'
  | 'playground'
  | 'pool'
  | 'gym'
  | 'laundry_room'
  | 'storage_room'
  | 'other';

/** Building document category */
export type BuildingDocumentCategory =
  | 'legal_document'
  | 'insurance'
  | 'maintenance'
  | 'financial_report'
  | 'meeting_minutes'
  | 'contract'
  | 'technical_document'
  | 'other';

/** Document visibility */
export type DocumentVisibility = 'public' | 'owners_only' | 'managers_only';

/** GPS location */
export interface GeoLocation {
  latitude: number;
  longitude: number;
}

/** Address */
export interface Address {
  street: string;
  city: string;
  state?: string;
  postalCode: string;
  country: string;
}

/** Building */
export interface Building {
  id: string;
  organizationId: string;
  name: string;
  address: Address;
  location?: GeoLocation;
  type: BuildingType;
  floorCount: number;
  unitCount: number;
  yearBuilt?: number;
  totalAreaM2?: number;
  photoUrl?: string;
  description?: string;
  status: BuildingStatus;
  managerId?: string;
  technicalManagerId?: string;
  createdAt: string;
  updatedAt: string;
}

/** Building summary for list views */
export interface BuildingSummary {
  id: string;
  name: string;
  address: Address;
  type: BuildingType;
  status: BuildingStatus;
  unitCount: number;
  floorCount: number;
  photoUrl?: string;
}

/** Floor in a building */
export interface Floor {
  id: string;
  buildingId: string;
  number: number;
  name?: string;
  floorPlanUrl?: string;
  unitCount: number;
}

/** Common area */
export interface CommonArea {
  id: string;
  buildingId: string;
  name: string;
  type: CommonAreaType;
  description?: string;
  areaM2?: number;
  floorId?: string;
  photoUrl?: string;
}

/** File attachment */
export interface Attachment {
  id: string;
  fileKey: string;
  fileName: string;
  fileType: string;
  fileSize: number;
  url: string;
}

/** Building document */
export interface BuildingDocument {
  id: string;
  buildingId: string;
  title: string;
  category: BuildingDocumentCategory;
  file: Attachment;
  description?: string;
  visibility: DocumentVisibility;
  createdAt: string;
  updatedAt: string;
}

/** Create building request */
export interface CreateBuildingRequest {
  name: string;
  address: Address;
  location?: GeoLocation;
  type: BuildingType;
  floorCount: number;
  yearBuilt?: number;
  totalAreaM2?: number;
  description?: string;
  managerId?: string;
  technicalManagerId?: string;
}

/** Update building request */
export interface UpdateBuildingRequest {
  name?: string;
  address?: Address;
  location?: GeoLocation;
  type?: BuildingType;
  floorCount?: number;
  yearBuilt?: number;
  totalAreaM2?: number;
  description?: string;
  status?: BuildingStatus;
  managerId?: string;
  technicalManagerId?: string;
}

/** Create floor request */
export interface CreateFloorRequest {
  number: number;
  name?: string;
}

/** Create common area request */
export interface CreateCommonAreaRequest {
  name: string;
  type: CommonAreaType;
  description?: string;
  areaM2?: number;
  floorId?: string;
}

/** Upload building document request */
export interface UploadDocumentRequest {
  title: string;
  category: BuildingDocumentCategory;
  description?: string;
  visibility: DocumentVisibility;
  fileContent: string;
  fileName: string;
  mimeType: string;
}

/** List buildings query parameters */
export interface ListBuildingsParams {
  page?: number;
  pageSize?: number;
  status?: BuildingStatus;
  type?: BuildingType;
}

/** List building documents query parameters */
export interface ListBuildingDocumentsParams {
  page?: number;
  pageSize?: number;
  category?: BuildingDocumentCategory;
}

/** Paginated response for buildings */
export interface BuildingsPaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  pageSize: number;
  totalPages: number;
}
