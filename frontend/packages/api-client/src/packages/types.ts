/**
 * Package & Visitor Management Types
 *
 * Type definitions for the Package & Visitor Management API (Epic 58).
 */

// Package status lifecycle: expected -> received -> notified -> picked_up
export type PackageStatus =
  | 'expected'
  | 'received'
  | 'notified'
  | 'picked_up'
  | 'returned'
  | 'unclaimed';

// Package carriers
export type PackageCarrier = 'usps' | 'ups' | 'fedex' | 'dhl' | 'amazon' | 'other';

// Visitor status lifecycle: pending -> checked_in -> checked_out
export type VisitorStatus = 'pending' | 'checked_in' | 'checked_out' | 'expired' | 'cancelled';

// Visitor purpose categories
export type VisitorPurpose =
  | 'guest'
  | 'delivery'
  | 'service'
  | 'contractor'
  | 'real_estate'
  | 'other';

// ============================================================================
// Package Types
// ============================================================================

export interface Package {
  id: string;
  tenantId: string;
  buildingId: string;
  unitId: string;
  residentId: string;
  trackingNumber?: string;
  carrier: PackageCarrier;
  carrierName?: string;
  description?: string;
  status: PackageStatus;
  expectedDate?: string;
  receivedAt?: string;
  receivedBy?: string;
  notifiedAt?: string;
  pickedUpAt?: string;
  pickedUpBy?: string;
  storageLocation?: string;
  photoUrl?: string;
  notes?: string;
  createdAt: string;
  updatedAt: string;
}

export interface PackageSummary {
  id: string;
  unitId: string;
  unitNumber?: string;
  residentId: string;
  residentName?: string;
  trackingNumber?: string;
  carrier: PackageCarrier;
  status: PackageStatus;
  expectedDate?: string;
  receivedAt?: string;
  createdAt: string;
}

export interface PackageWithDetails extends Package {
  unitNumber?: string;
  buildingName?: string;
  residentName?: string;
  receivedByName?: string;
}

// ============================================================================
// Visitor Types
// ============================================================================

export interface Visitor {
  id: string;
  tenantId: string;
  buildingId: string;
  unitId: string;
  hostId: string;
  visitorName: string;
  visitorEmail?: string;
  visitorPhone?: string;
  companyName?: string;
  purpose: VisitorPurpose;
  purposeNotes?: string;
  accessCode: string;
  accessCodeExpiresAt: string;
  expectedArrival: string;
  expectedDeparture?: string;
  status: VisitorStatus;
  checkedInAt?: string;
  checkedInBy?: string;
  checkedOutAt?: string;
  checkedOutBy?: string;
  notificationSentAt?: string;
  notificationMethod?: string;
  vehicleLicensePlate?: string;
  notes?: string;
  createdAt: string;
  updatedAt: string;
}

export interface VisitorSummary {
  id: string;
  unitId: string;
  unitNumber?: string;
  hostId: string;
  hostName?: string;
  visitorName: string;
  purpose: VisitorPurpose;
  expectedArrival: string;
  status: VisitorStatus;
  accessCode: string;
  createdAt: string;
}

export interface VisitorWithDetails extends Visitor {
  unitNumber?: string;
  buildingName?: string;
  hostName?: string;
  checkedInByName?: string;
}

// ============================================================================
// Settings Types
// ============================================================================

export interface BuildingPackageSettings {
  id: string;
  tenantId: string;
  buildingId: string;
  maxStorageDays: number;
  sendReminderAfterDays: number;
  requirePhotoOnReceipt: boolean;
  allowResidentSelfPickup: boolean;
  notifyOnArrival: boolean;
  sendDailySummary: boolean;
  storageInstructions?: string;
  createdAt: string;
  updatedAt: string;
}

export interface BuildingVisitorSettings {
  id: string;
  tenantId: string;
  buildingId: string;
  defaultCodeValidityHours: number;
  codeLength: number;
  requirePurpose: boolean;
  maxVisitorsPerDayPerUnit?: number;
  maxAdvanceRegistrationDays: number;
  notifyHostOnCheckin: boolean;
  sendVisitorInstructions: boolean;
  requireIdVerification: boolean;
  requirePhoto: boolean;
  visitorInstructions?: string;
  staffInstructions?: string;
  createdAt: string;
  updatedAt: string;
}

// ============================================================================
// Statistics Types
// ============================================================================

export interface PackageStatistics {
  totalPackages: number;
  expectedPackages: number;
  receivedPackages: number;
  pickedUpPackages: number;
  unclaimedPackages: number;
  avgPickupTimeHours?: number;
}

export interface VisitorStatistics {
  totalVisitorsToday: number;
  pendingArrivals: number;
  checkedInNow: number;
  totalThisWeek: number;
  totalThisMonth: number;
}

// ============================================================================
// Request Types
// ============================================================================

export interface CreatePackageRequest {
  unitId: string;
  trackingNumber?: string;
  carrier: PackageCarrier;
  carrierName?: string;
  description?: string;
  expectedDate?: string;
  notes?: string;
}

export interface UpdatePackageRequest {
  trackingNumber?: string;
  carrier?: PackageCarrier;
  carrierName?: string;
  description?: string;
  expectedDate?: string;
  storageLocation?: string;
  notes?: string;
}

export interface ReceivePackageRequest {
  storageLocation?: string;
  photoUrl?: string;
  notes?: string;
}

export interface PickupPackageRequest {
  pickedUpBy?: string;
  notes?: string;
}

export interface CreateVisitorRequest {
  unitId: string;
  visitorName: string;
  visitorEmail?: string;
  visitorPhone?: string;
  companyName?: string;
  purpose: VisitorPurpose;
  purposeNotes?: string;
  expectedArrival: string;
  expectedDeparture?: string;
  vehicleLicensePlate?: string;
  notes?: string;
  sendNotification?: boolean;
}

export interface UpdateVisitorRequest {
  visitorName?: string;
  visitorEmail?: string;
  visitorPhone?: string;
  companyName?: string;
  purpose?: VisitorPurpose;
  purposeNotes?: string;
  expectedArrival?: string;
  expectedDeparture?: string;
  vehicleLicensePlate?: string;
  notes?: string;
}

export interface CheckInVisitorRequest {
  notes?: string;
}

export interface CheckOutVisitorRequest {
  notes?: string;
}

export interface VerifyAccessCodeRequest {
  accessCode: string;
  buildingId: string;
}

export interface UpdatePackageSettingsRequest {
  maxStorageDays?: number;
  sendReminderAfterDays?: number;
  requirePhotoOnReceipt?: boolean;
  allowResidentSelfPickup?: boolean;
  notifyOnArrival?: boolean;
  sendDailySummary?: boolean;
  storageInstructions?: string;
}

export interface UpdateVisitorSettingsRequest {
  defaultCodeValidityHours?: number;
  codeLength?: number;
  requirePurpose?: boolean;
  maxVisitorsPerDayPerUnit?: number;
  maxAdvanceRegistrationDays?: number;
  notifyHostOnCheckin?: boolean;
  sendVisitorInstructions?: boolean;
  requireIdVerification?: boolean;
  requirePhoto?: boolean;
  visitorInstructions?: string;
  staffInstructions?: string;
}

// ============================================================================
// Query Parameters
// ============================================================================

export interface ListPackagesParams {
  buildingId?: string;
  unitId?: string;
  residentId?: string;
  status?: PackageStatus;
  carrier?: PackageCarrier;
  fromDate?: string;
  toDate?: string;
  limit?: number;
  offset?: number;
}

export interface ListVisitorsParams {
  buildingId?: string;
  unitId?: string;
  hostId?: string;
  status?: VisitorStatus;
  purpose?: VisitorPurpose;
  fromDate?: string;
  toDate?: string;
  todayOnly?: boolean;
  limit?: number;
  offset?: number;
}

// ============================================================================
// Response Types
// ============================================================================

export interface PackageListResponse {
  packages: PackageSummary[];
  total: number;
}

export interface PackageDetailResponse {
  package: PackageWithDetails;
}

export interface PackageActionResponse {
  message: string;
  package: Package;
}

export interface VisitorListResponse {
  visitors: VisitorSummary[];
  total: number;
}

export interface VisitorDetailResponse {
  visitor: VisitorWithDetails;
}

export interface VisitorActionResponse {
  message: string;
  visitor: Visitor;
}

export interface AccessCodeVerification {
  valid: boolean;
  visitor?: VisitorSummary;
  message: string;
}

export interface PackageSettingsResponse {
  settings: BuildingPackageSettings;
}

export interface VisitorSettingsResponse {
  settings: BuildingVisitorSettings;
}

export interface PackageStatisticsResponse {
  statistics: PackageStatistics;
}

export interface VisitorStatisticsResponse {
  statistics: VisitorStatistics;
}
