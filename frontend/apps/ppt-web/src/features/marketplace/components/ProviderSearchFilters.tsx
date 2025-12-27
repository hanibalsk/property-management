/**
 * ProviderSearchFilters component - search and filter controls for marketplace.
 * Epic 68: Service Provider Marketplace (Story 68.2)
 */

import { useState } from 'react';
import type { BadgeType, ServiceCategory } from './ProviderCard';

export interface SearchFilters {
  query?: string;
  category?: ServiceCategory;
  location?: string;
  region?: string;
  minRating?: number;
  minHourlyRate?: number;
  maxHourlyRate?: number;
  verifiedOnly?: boolean;
  emergencyOnly?: boolean;
  badges?: BadgeType[];
  sortBy?: 'rating' | 'reviews' | 'jobs_completed' | 'hourly_rate';
  sortOrder?: 'asc' | 'desc';
}

interface ProviderSearchFiltersProps {
  filters: SearchFilters;
  onFiltersChange: (filters: SearchFilters) => void;
  onSearch: () => void;
  onReset: () => void;
  isLoading?: boolean;
}

const categories: { value: ServiceCategory; label: string }[] = [
  { value: 'plumbing', label: 'Plumbing' },
  { value: 'electrical', label: 'Electrical' },
  { value: 'hvac', label: 'HVAC' },
  { value: 'cleaning', label: 'Cleaning' },
  { value: 'landscaping', label: 'Landscaping' },
  { value: 'security', label: 'Security' },
  { value: 'painting', label: 'Painting' },
  { value: 'roofing', label: 'Roofing' },
  { value: 'carpentry', label: 'Carpentry' },
  { value: 'locksmith', label: 'Locksmith' },
  { value: 'pest_control', label: 'Pest Control' },
  { value: 'general_maintenance', label: 'General Maintenance' },
  { value: 'elevator_maintenance', label: 'Elevator Maintenance' },
  { value: 'fire_safety', label: 'Fire Safety' },
  { value: 'waste_management', label: 'Waste Management' },
  { value: 'other', label: 'Other' },
];

const ratingOptions = [
  { value: 4.5, label: '4.5+ stars' },
  { value: 4, label: '4+ stars' },
  { value: 3.5, label: '3.5+ stars' },
  { value: 3, label: '3+ stars' },
];

const sortOptions = [
  { value: 'rating', label: 'Highest Rated' },
  { value: 'reviews', label: 'Most Reviews' },
  { value: 'jobs_completed', label: 'Most Jobs' },
  { value: 'hourly_rate', label: 'Hourly Rate' },
];

export function ProviderSearchFilters({
  filters,
  onFiltersChange,
  onSearch,
  onReset,
  isLoading,
}: ProviderSearchFiltersProps) {
  const [showAdvanced, setShowAdvanced] = useState(false);

  const updateFilter = <K extends keyof SearchFilters>(key: K, value: SearchFilters[K]) => {
    onFiltersChange({ ...filters, [key]: value });
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSearch();
  };

  return (
    <form onSubmit={handleSubmit} className="bg-white rounded-lg shadow p-4">
      {/* Primary Search */}
      <div className="flex flex-col md:flex-row gap-4">
        {/* Search Query */}
        <div className="flex-1">
          <label htmlFor="search-query" className="sr-only">
            Search providers
          </label>
          <div className="relative">
            <svg
              className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <title>Search icon</title>
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
              />
            </svg>
            <input
              id="search-query"
              type="text"
              placeholder="Search providers, services..."
              value={filters.query || ''}
              onChange={(e) => updateFilter('query', e.target.value || undefined)}
              className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
        </div>

        {/* Category */}
        <div className="md:w-48">
          <label htmlFor="category" className="sr-only">
            Service Category
          </label>
          <select
            id="category"
            value={filters.category || ''}
            onChange={(e) =>
              updateFilter('category', (e.target.value as ServiceCategory) || undefined)
            }
            className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
          >
            <option value="">All Categories</option>
            {categories.map((cat) => (
              <option key={cat.value} value={cat.value}>
                {cat.label}
              </option>
            ))}
          </select>
        </div>

        {/* Location */}
        <div className="md:w-48">
          <label htmlFor="location" className="sr-only">
            Location
          </label>
          <input
            id="location"
            type="text"
            placeholder="City or Postal Code"
            value={filters.location || ''}
            onChange={(e) => updateFilter('location', e.target.value || undefined)}
            className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
          />
        </div>

        {/* Search Button */}
        <button
          type="submit"
          disabled={isLoading}
          className="px-6 py-2 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
        >
          {isLoading ? 'Searching...' : 'Search'}
        </button>
      </div>

      {/* Toggle Advanced Filters */}
      <button
        type="button"
        onClick={() => setShowAdvanced(!showAdvanced)}
        className="mt-4 text-sm text-blue-600 hover:text-blue-800 flex items-center gap-1"
      >
        <svg
          className={`w-4 h-4 transition-transform ${showAdvanced ? 'rotate-180' : ''}`}
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <title>Toggle filters</title>
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
        {showAdvanced ? 'Hide' : 'Show'} Advanced Filters
      </button>

      {/* Advanced Filters */}
      {showAdvanced && (
        <div className="mt-4 pt-4 border-t grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {/* Minimum Rating */}
          <div>
            <label htmlFor="min-rating" className="block text-sm font-medium text-gray-700 mb-1">
              Minimum Rating
            </label>
            <select
              id="min-rating"
              value={filters.minRating || ''}
              onChange={(e) =>
                updateFilter('minRating', e.target.value ? Number(e.target.value) : undefined)
              }
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="">Any Rating</option>
              {ratingOptions.map((opt) => (
                <option key={opt.value} value={opt.value}>
                  {opt.label}
                </option>
              ))}
            </select>
          </div>

          {/* Price Range */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Hourly Rate (EUR)
            </label>
            <div className="flex items-center gap-2">
              <input
                type="number"
                placeholder="Min"
                value={filters.minHourlyRate || ''}
                onChange={(e) =>
                  updateFilter('minHourlyRate', e.target.value ? Number(e.target.value) : undefined)
                }
                className="w-20 px-2 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              />
              <span className="text-gray-500">-</span>
              <input
                type="number"
                placeholder="Max"
                value={filters.maxHourlyRate || ''}
                onChange={(e) =>
                  updateFilter('maxHourlyRate', e.target.value ? Number(e.target.value) : undefined)
                }
                className="w-20 px-2 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              />
            </div>
          </div>

          {/* Sort By */}
          <div>
            <label htmlFor="sort-by" className="block text-sm font-medium text-gray-700 mb-1">
              Sort By
            </label>
            <select
              id="sort-by"
              value={filters.sortBy || ''}
              onChange={(e) =>
                updateFilter('sortBy', (e.target.value as SearchFilters['sortBy']) || undefined)
              }
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="">Relevance</option>
              {sortOptions.map((opt) => (
                <option key={opt.value} value={opt.value}>
                  {opt.label}
                </option>
              ))}
            </select>
          </div>

          {/* Checkboxes */}
          <div className="space-y-2">
            <label className="block text-sm font-medium text-gray-700 mb-1">Options</label>
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={filters.verifiedOnly || false}
                onChange={(e) => updateFilter('verifiedOnly', e.target.checked || undefined)}
                className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
              />
              <span className="text-sm text-gray-700">Verified Only</span>
            </label>
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={filters.emergencyOnly || false}
                onChange={(e) => updateFilter('emergencyOnly', e.target.checked || undefined)}
                className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
              />
              <span className="text-sm text-gray-700">Emergency Available</span>
            </label>
          </div>
        </div>
      )}

      {/* Active Filters Summary */}
      {(filters.category ||
        filters.location ||
        filters.minRating ||
        filters.verifiedOnly ||
        filters.emergencyOnly) && (
        <div className="mt-4 pt-4 border-t flex items-center gap-2 flex-wrap">
          <span className="text-sm text-gray-500">Active filters:</span>
          {filters.category && (
            <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
              {categories.find((c) => c.value === filters.category)?.label}
              <button
                type="button"
                onClick={() => updateFilter('category', undefined)}
                className="ml-1 hover:text-blue-600"
              >
                x
              </button>
            </span>
          )}
          {filters.location && (
            <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
              {filters.location}
              <button
                type="button"
                onClick={() => updateFilter('location', undefined)}
                className="ml-1 hover:text-blue-600"
              >
                x
              </button>
            </span>
          )}
          {filters.minRating && (
            <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
              {filters.minRating}+ stars
              <button
                type="button"
                onClick={() => updateFilter('minRating', undefined)}
                className="ml-1 hover:text-yellow-600"
              >
                x
              </button>
            </span>
          )}
          {filters.verifiedOnly && (
            <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-green-100 text-green-800">
              Verified
              <button
                type="button"
                onClick={() => updateFilter('verifiedOnly', undefined)}
                className="ml-1 hover:text-green-600"
              >
                x
              </button>
            </span>
          )}
          {filters.emergencyOnly && (
            <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-red-100 text-red-800">
              Emergency
              <button
                type="button"
                onClick={() => updateFilter('emergencyOnly', undefined)}
                className="ml-1 hover:text-red-600"
              >
                x
              </button>
            </span>
          )}
          <button
            type="button"
            onClick={onReset}
            className="text-sm text-gray-500 hover:text-gray-700 underline"
          >
            Clear all
          </button>
        </div>
      )}
    </form>
  );
}
