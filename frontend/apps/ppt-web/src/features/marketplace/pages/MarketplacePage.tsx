/**
 * MarketplacePage - main marketplace search and discovery page.
 * Epic 68: Service Provider Marketplace (Story 68.2)
 */

import { useState } from 'react';
import { ProviderCard, type ProviderSummary } from '../components/ProviderCard';
import { ProviderSearchFilters, type SearchFilters } from '../components/ProviderSearchFilters';

interface MarketplacePageProps {
  providers: ProviderSummary[];
  totalProviders: number;
  isLoading?: boolean;
  onSearch: (filters: SearchFilters) => void;
  onViewProvider: (id: string) => void;
  onRequestQuote: (id: string) => void;
  onContactProvider: (id: string) => void;
  onCreateRfq: () => void;
  onLoadMore?: () => void;
  hasMore?: boolean;
}

export function MarketplacePage({
  providers,
  totalProviders,
  isLoading,
  onSearch,
  onViewProvider,
  onRequestQuote,
  onContactProvider,
  onCreateRfq,
  onLoadMore,
  hasMore,
}: MarketplacePageProps) {
  const [filters, setFilters] = useState<SearchFilters>({});

  const handleFiltersChange = (newFilters: SearchFilters) => {
    setFilters(newFilters);
  };

  const handleSearch = () => {
    onSearch(filters);
  };

  const handleReset = () => {
    setFilters({});
    onSearch({});
  };

  return (
    <div className="max-w-7xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Service Provider Marketplace</h1>
          <p className="mt-1 text-gray-600">
            Find and hire verified service providers for your properties
          </p>
        </div>
        <button
          type="button"
          onClick={onCreateRfq}
          className="px-4 py-2 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
        >
          Create RFQ
        </button>
      </div>

      {/* Search and Filters */}
      <ProviderSearchFilters
        filters={filters}
        onFiltersChange={handleFiltersChange}
        onSearch={handleSearch}
        onReset={handleReset}
        isLoading={isLoading}
      />

      {/* Results Summary */}
      <div className="mt-6 flex items-center justify-between">
        <p className="text-sm text-gray-600">
          {totalProviders > 0 ? (
            <>
              Showing <span className="font-medium">{providers.length}</span> of{' '}
              <span className="font-medium">{totalProviders}</span> providers
            </>
          ) : (
            'No providers found'
          )}
        </p>
        {filters.sortBy && (
          <p className="text-sm text-gray-500">
            Sorted by: {filters.sortBy.replace('_', ' ')}
            {filters.sortOrder === 'desc' ? ' (descending)' : ' (ascending)'}
          </p>
        )}
      </div>

      {/* Results Grid */}
      {isLoading && providers.length === 0 ? (
        <div className="mt-8 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {[...Array(6)].map((_, i) => (
            <div key={`skeleton-${i}`} className="bg-white rounded-lg shadow p-6 animate-pulse">
              <div className="flex items-start gap-4">
                <div className="w-16 h-16 bg-gray-200 rounded-lg" />
                <div className="flex-1">
                  <div className="h-5 bg-gray-200 rounded w-3/4" />
                  <div className="mt-2 h-4 bg-gray-200 rounded w-1/2" />
                </div>
              </div>
              <div className="mt-4 space-y-2">
                <div className="h-4 bg-gray-200 rounded" />
                <div className="h-4 bg-gray-200 rounded w-2/3" />
              </div>
            </div>
          ))}
        </div>
      ) : providers.length === 0 ? (
        <div className="mt-8 text-center py-12 bg-gray-50 rounded-lg">
          <svg
            className="mx-auto w-12 h-12 text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <title>No results</title>
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
          <h3 className="mt-4 text-lg font-medium text-gray-900">No providers found</h3>
          <p className="mt-2 text-gray-500">
            Try adjusting your search filters or browse all providers
          </p>
          <button
            type="button"
            onClick={handleReset}
            className="mt-4 text-blue-600 font-medium hover:text-blue-800"
          >
            Clear all filters
          </button>
        </div>
      ) : (
        <>
          <div className="mt-6 grid grid-cols-1 md:grid-cols-2 gap-6">
            {providers.map((provider) => (
              <ProviderCard
                key={provider.id}
                provider={provider}
                onView={onViewProvider}
                onRequestQuote={onRequestQuote}
                onContact={onContactProvider}
              />
            ))}
          </div>

          {/* Load More */}
          {hasMore && (
            <div className="mt-8 text-center">
              <button
                type="button"
                onClick={onLoadMore}
                disabled={isLoading}
                className="px-6 py-2 border border-gray-300 text-gray-700 font-medium rounded-lg hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
              >
                {isLoading ? 'Loading...' : 'Load More Providers'}
              </button>
            </div>
          )}
        </>
      )}

      {/* Quick Categories */}
      {!filters.category && providers.length > 0 && (
        <div className="mt-12">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">Browse by Category</h2>
          <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
            {[
              { value: 'plumbing', label: 'Plumbing', icon: 'M12 2a10 10 0 100 20 10 10 0 000-20z' },
              { value: 'electrical', label: 'Electrical', icon: 'M13 10V3L4 14h7v7l9-11h-7z' },
              { value: 'hvac', label: 'HVAC', icon: 'M12 2a10 10 0 100 20 10 10 0 000-20z' },
              { value: 'cleaning', label: 'Cleaning', icon: 'M12 2a10 10 0 100 20 10 10 0 000-20z' },
              { value: 'security', label: 'Security', icon: 'M12 2a10 10 0 100 20 10 10 0 000-20z' },
              { value: 'general_maintenance', label: 'Maintenance', icon: 'M12 2a10 10 0 100 20 10 10 0 000-20z' },
            ].map((cat) => (
              <button
                key={cat.value}
                type="button"
                onClick={() => {
                  setFilters((prev) => ({ ...prev, category: cat.value as SearchFilters['category'] }));
                  onSearch({ ...filters, category: cat.value as SearchFilters['category'] });
                }}
                className="p-4 bg-white rounded-lg shadow hover:shadow-md transition-shadow text-center"
              >
                <div className="w-10 h-10 mx-auto bg-blue-100 rounded-full flex items-center justify-center">
                  <svg className="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <title>{cat.label}</title>
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d={cat.icon} />
                  </svg>
                </div>
                <p className="mt-2 text-sm font-medium text-gray-900">{cat.label}</p>
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
