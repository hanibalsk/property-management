/**
 * Property comparison context for Reality Portal.
 *
 * Epic 51 - Story 51.1: Add to Comparison
 */

'use client';

import type { ListingSummary } from '@ppt/reality-api-client';
import { createContext, useCallback, useContext, useEffect, useState } from 'react';

const MAX_COMPARISON_ITEMS = 4;
const STORAGE_KEY = 'ppt-comparison';

interface ComparisonContextValue {
  listings: ListingSummary[];
  isInComparison: (listingId: string) => boolean;
  addToComparison: (listing: ListingSummary) => boolean;
  removeFromComparison: (listingId: string) => void;
  clearComparison: () => void;
  canAddMore: boolean;
  shareUrl: string | null;
  generateShareUrl: () => string;
}

const ComparisonContext = createContext<ComparisonContextValue | null>(null);

export function ComparisonProvider({ children }: { children: React.ReactNode }) {
  const [listings, setListings] = useState<ListingSummary[]>([]);
  const [shareUrl, setShareUrl] = useState<string | null>(null);

  // Load from localStorage on mount (SSR safety: check window exists)
  useEffect(() => {
    if (typeof window === 'undefined') return;
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      try {
        const parsed = JSON.parse(stored);
        setListings(parsed);
      } catch {
        // Invalid data, clear it
        localStorage.removeItem(STORAGE_KEY);
      }
    }
  }, []);

  // Save to localStorage when listings change (SSR safety: check window exists)
  useEffect(() => {
    if (typeof window === 'undefined') return;
    if (listings.length > 0) {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(listings));
    } else {
      localStorage.removeItem(STORAGE_KEY);
    }
    // Clear share URL when comparison changes
    setShareUrl(null);
  }, [listings]);

  const isInComparison = useCallback(
    (listingId: string) => {
      return listings.some((l) => l.id === listingId);
    },
    [listings]
  );

  const addToComparison = useCallback((listing: ListingSummary) => {
    let added = false;
    setListings((prev) => {
      if (prev.length >= MAX_COMPARISON_ITEMS) {
        return prev;
      }
      if (prev.some((l) => l.id === listing.id)) {
        return prev;
      }
      added = true;
      return [...prev, listing];
    });
    return added;
  }, []);

  const removeFromComparison = useCallback((listingId: string) => {
    setListings((prev) => prev.filter((l) => l.id !== listingId));
  }, []);

  const clearComparison = useCallback(() => {
    setListings([]);
  }, []);

  const generateShareUrl = useCallback(() => {
    const ids = listings.map((l) => l.id).join(',');
    const baseUrl =
      typeof window !== 'undefined'
        ? window.location.origin
        : process.env.NEXT_PUBLIC_APP_URL ?? '';
    const url = baseUrl ? `${baseUrl}/compare?ids=${ids}` : `/compare?ids=${ids}`;
    setShareUrl(url);
    return url;
  }, [listings]);

  const canAddMore = listings.length < MAX_COMPARISON_ITEMS;

  return (
    <ComparisonContext.Provider
      value={{
        listings,
        isInComparison,
        addToComparison,
        removeFromComparison,
        clearComparison,
        canAddMore,
        shareUrl,
        generateShareUrl,
      }}
    >
      {children}
    </ComparisonContext.Provider>
  );
}

export function useComparison() {
  const context = useContext(ComparisonContext);
  if (!context) {
    throw new Error('useComparison must be used within a ComparisonProvider');
  }
  return context;
}
