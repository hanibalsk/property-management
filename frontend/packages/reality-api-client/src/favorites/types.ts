/**
 * Reality Portal Favorites & Saved Searches Types
 *
 * TypeScript types for favorites and saved searches API (Epic 44).
 */

import type { ListingFilters, ListingSummary } from '../listings/types';

// Favorite Listing
export interface FavoriteListing {
  id: string;
  listingId: string;
  listing: ListingSummary;
  addedAt: string;
  notes?: string;
}

// Saved Search
export interface SavedSearch {
  id: string;
  name: string;
  filters: ListingFilters;
  alertsEnabled: boolean;
  alertFrequency?: 'daily' | 'weekly' | 'instant';
  newListingsCount?: number;
  lastAlertAt?: string;
  createdAt: string;
  updatedAt: string;
}

// Create Saved Search Request
export interface CreateSavedSearchRequest {
  name: string;
  filters: ListingFilters;
  alertsEnabled?: boolean;
  alertFrequency?: 'daily' | 'weekly' | 'instant';
}

// Update Saved Search Request
export interface UpdateSavedSearchRequest {
  name?: string;
  filters?: ListingFilters;
  alertsEnabled?: boolean;
  alertFrequency?: 'daily' | 'weekly' | 'instant';
}

// Paginated Response
export interface PaginatedFavorites {
  data: FavoriteListing[];
  total: number;
  page: number;
  pageSize: number;
  totalPages: number;
}
