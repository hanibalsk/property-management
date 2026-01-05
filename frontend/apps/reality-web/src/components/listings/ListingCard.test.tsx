/**
 * ListingCard Component Tests
 *
 * Tests for property listing card component (Epic 44).
 */

import type { ListingSummary } from '@ppt/reality-api-client';
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { ListingCard } from './ListingCard';

// Mock listing data
const mockListing: ListingSummary = {
  id: 'listing-1',
  slug: 'beautiful-apartment-bratislava',
  title: 'Beautiful Apartment in Bratislava',
  price: 150000,
  currency: 'EUR',
  transactionType: 'sale',
  propertyType: 'apartment',
  status: 'active',
  area: 75,
  rooms: 3,
  floor: 2,
  address: {
    city: 'Bratislava',
    district: 'Old Town',
    country: 'Slovakia',
  },
  primaryPhoto: {
    id: 'photo-1',
    url: 'https://example.com/photo-original.jpg',
    thumbnailUrl: 'https://example.com/photo.jpg',
    isPrimary: true,
    order: 0,
  },
  isFavorite: false,
  isFeatured: false,
  createdAt: '2024-01-01T00:00:00Z',
  updatedAt: '2024-01-01T00:00:00Z',
};

describe('ListingCard', () => {
  it('renders listing title', () => {
    render(<ListingCard listing={mockListing} />);
    expect(screen.getByText('Beautiful Apartment in Bratislava')).toBeInTheDocument();
  });

  it('renders formatted price', () => {
    render(<ListingCard listing={mockListing} />);
    // Price should be formatted as currency
    expect(screen.getByText(/€150,000/)).toBeInTheDocument();
  });

  it('renders location with city and district', () => {
    render(<ListingCard listing={mockListing} />);
    expect(screen.getByText('Bratislava, Old Town')).toBeInTheDocument();
  });

  it('renders area in square meters', () => {
    render(<ListingCard listing={mockListing} />);
    expect(screen.getByText(/75/)).toBeInTheDocument();
  });

  it('shows favorite button by default', () => {
    render(<ListingCard listing={mockListing} />);
    expect(screen.getByRole('button', { name: /add to favorites/i })).toBeInTheDocument();
  });

  it('hides favorite button when showFavoriteButton is false', () => {
    render(<ListingCard listing={mockListing} showFavoriteButton={false} />);
    expect(screen.queryByRole('button', { name: /favorites/i })).not.toBeInTheDocument();
  });

  it('calls onToggleFavorite when favorite button is clicked', () => {
    const onToggleFavorite = vi.fn();
    render(<ListingCard listing={mockListing} onToggleFavorite={onToggleFavorite} />);

    const favoriteButton = screen.getByRole('button', { name: /add to favorites/i });
    fireEvent.click(favoriteButton);

    expect(onToggleFavorite).toHaveBeenCalledWith('listing-1', false);
  });

  it('shows remove from favorites when listing is favorite', () => {
    const favoriteListing = { ...mockListing, isFavorite: true };
    render(<ListingCard listing={favoriteListing} />);
    expect(screen.getByRole('button', { name: /remove from favorites/i })).toBeInTheDocument();
  });

  it('renders featured badge when listing is featured', () => {
    const featuredListing = { ...mockListing, isFeatured: true };
    render(<ListingCard listing={featuredListing} />);
    // Mock translation returns just the key name without namespace
    expect(screen.getByText('featured')).toBeInTheDocument();
  });

  it('shows forSale badge for sale listings', () => {
    render(<ListingCard listing={mockListing} />);
    // Mock translation returns just the key name without namespace
    expect(screen.getByText('forSale')).toBeInTheDocument();
  });

  it('shows forRent badge for rent listings', () => {
    const rentListing = { ...mockListing, transactionType: 'rent' as const };
    render(<ListingCard listing={rentListing} />);
    // Mock translation returns just the key name without namespace
    expect(screen.getByText('forRent')).toBeInTheDocument();
  });

  it('shows per month suffix for rent listings', () => {
    const rentListing = { ...mockListing, transactionType: 'rent' as const };
    render(<ListingCard listing={rentListing} />);
    // Mock translation returns just the key name without namespace
    expect(screen.getByText('perMonth')).toBeInTheDocument();
  });

  it('renders link to listing detail page', () => {
    render(<ListingCard listing={mockListing} />);
    const link = screen.getByRole('link');
    expect(link).toHaveAttribute('href', '/listings/beautiful-apartment-bratislava');
  });

  it('renders placeholder when no photo is provided', () => {
    const listingWithoutPhoto = { ...mockListing, primaryPhoto: undefined };
    render(<ListingCard listing={listingWithoutPhoto} />);
    // Should not throw and should render the card
    expect(screen.getByText('Beautiful Apartment in Bratislava')).toBeInTheDocument();
  });

  it('renders floor number when provided', () => {
    render(<ListingCard listing={mockListing} />);
    // Mock translation returns just the key name without namespace
    expect(screen.getByText('floorNumber')).toBeInTheDocument();
  });

  it('does not render floor when not provided', () => {
    const listingWithoutFloor = { ...mockListing, floor: undefined };
    render(<ListingCard listing={listingWithoutFloor} />);
    expect(screen.queryByText('floorNumber')).not.toBeInTheDocument();
  });
});
