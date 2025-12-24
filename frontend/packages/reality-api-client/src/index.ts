/**
 * Reality Portal API Client
 *
 * Generated from reality-server OpenAPI spec.
 * Used by: reality-web (Next.js), mobile-native (KMP via shared types)
 */

// Export generated types and client
// export * from './generated';

// Export domain-specific modules
export * from './listings';
export * from './favorites';
export * from './inquiries';

// Agency module - export with renamed ListingStatus to avoid conflict
export type {
  Agency,
  AgencyAddress,
  AgencyStats,
  AgencyPerformance,
  Realtor,
  RealtorStatus,
  RealtorStats,
  RealtorInvitation,
  AgencyBranding,
  AgencyListing,
  ListingStatus as AgencyListingStatus,
  CreateAgencyRequest,
  UpdateAgencyRequest,
  UpdateBrandingRequest,
  InviteRealtorRequest,
  UpdateRealtorRequest,
} from './agency/types';
export * from './agency/hooks';

// Import module - property import functionality (Epic 46)
export * from './import';

// API version
export const REALITY_API_VERSION = '1.0.0';
