/**
 * MarketplacePage
 *
 * Page for browsing marketplace items.
 * Part of Story 42.4: Item Marketplace.
 */

import type {
  ItemCondition,
  ItemListingType,
  ListItemsParams,
  MarketplaceItem,
} from '@ppt/api-client';
import { useState } from 'react';
import { ItemCard } from '../components/ItemCard';

interface MarketplacePageProps {
  items: MarketplaceItem[];
  total: number;
  isLoading?: boolean;
  currentUserId?: string;
  onNavigateToCreate: () => void;
  onNavigateToItem: (id: string) => void;
  onContactSeller: (id: string) => void;
  onEditItem: (id: string) => void;
  onDeleteItem: (id: string) => void;
  onMarkSold: (id: string) => void;
  onFilterChange: (params: ListItemsParams) => void;
}

const categories = [
  'All Categories',
  'Electronics',
  'Furniture',
  'Clothing',
  'Books',
  'Sports',
  'Home & Garden',
  'Kids & Baby',
  'Vehicles',
  'Other',
];

const listingTypeFilters: { value: ItemListingType | ''; label: string }[] = [
  { value: '', label: 'All Types' },
  { value: 'sale', label: 'For Sale' },
  { value: 'free', label: 'Free' },
  { value: 'wanted', label: 'Wanted' },
  { value: 'trade', label: 'For Trade' },
];

const conditionFilters: { value: ItemCondition | ''; label: string }[] = [
  { value: '', label: 'Any Condition' },
  { value: 'new', label: 'New' },
  { value: 'like_new', label: 'Like New' },
  { value: 'good', label: 'Good' },
  { value: 'fair', label: 'Fair' },
  { value: 'poor', label: 'Poor' },
];

export function MarketplacePage({
  items,
  total,
  isLoading,
  currentUserId,
  onNavigateToCreate,
  onNavigateToItem,
  onContactSeller,
  onEditItem,
  onDeleteItem,
  onMarkSold,
  onFilterChange,
}: MarketplacePageProps) {
  const [page, setPage] = useState(1);
  const [pageSize] = useState(12);
  const [search, setSearch] = useState('');
  const [category, setCategory] = useState('All Categories');
  const [listingType, setListingType] = useState<ItemListingType | ''>('');
  const [condition, setCondition] = useState<ItemCondition | ''>('');
  const [minPrice, setMinPrice] = useState('');
  const [maxPrice, setMaxPrice] = useState('');

  const totalPages = Math.ceil(total / pageSize);

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    applyFilters(1);
  };

  const applyFilters = (newPage: number) => {
    setPage(newPage);
    onFilterChange({
      page: newPage,
      pageSize,
      search: search || undefined,
      category: category !== 'All Categories' ? category : undefined,
      listingType: listingType || undefined,
      condition: condition || undefined,
      status: 'active',
      minPrice: minPrice ? Number(minPrice) : undefined,
      maxPrice: maxPrice ? Number(maxPrice) : undefined,
    });
  };

  const handleCategoryChange = (newCategory: string) => {
    setCategory(newCategory);
    setPage(1);
    onFilterChange({
      page: 1,
      pageSize,
      search: search || undefined,
      category: newCategory !== 'All Categories' ? newCategory : undefined,
      listingType: listingType || undefined,
      condition: condition || undefined,
      status: 'active',
      minPrice: minPrice ? Number(minPrice) : undefined,
      maxPrice: maxPrice ? Number(maxPrice) : undefined,
    });
  };

  const handleListingTypeChange = (type: ItemListingType | '') => {
    setListingType(type);
    setPage(1);
    onFilterChange({
      page: 1,
      pageSize,
      search: search || undefined,
      category: category !== 'All Categories' ? category : undefined,
      listingType: type || undefined,
      condition: condition || undefined,
      status: 'active',
      minPrice: minPrice ? Number(minPrice) : undefined,
      maxPrice: maxPrice ? Number(maxPrice) : undefined,
    });
  };

  const handleConditionChange = (cond: ItemCondition | '') => {
    setCondition(cond);
    setPage(1);
    onFilterChange({
      page: 1,
      pageSize,
      search: search || undefined,
      category: category !== 'All Categories' ? category : undefined,
      listingType: listingType || undefined,
      condition: cond || undefined,
      status: 'active',
      minPrice: minPrice ? Number(minPrice) : undefined,
      maxPrice: maxPrice ? Number(maxPrice) : undefined,
    });
  };

  const handlePageChange = (newPage: number) => {
    applyFilters(newPage);
  };

  return (
    <div className="max-w-7xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Marketplace</h1>
          <p className="mt-1 text-sm text-gray-500">Buy, sell, and trade with your neighbors</p>
        </div>
        <button
          type="button"
          onClick={onNavigateToCreate}
          className="inline-flex items-center px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
        >
          <svg
            className="w-5 h-5 mr-2"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            aria-hidden="true"
          >
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
          </svg>
          Create Listing
        </button>
      </div>

      {/* Filters */}
      <div className="bg-white rounded-lg shadow p-4 mb-6">
        <form onSubmit={handleSearch} className="space-y-4">
          {/* Search Row */}
          <div className="flex flex-wrap gap-4">
            <div className="flex-1 min-w-[200px]">
              <div className="relative">
                <input
                  type="text"
                  value={search}
                  onChange={(e) => setSearch(e.target.value)}
                  placeholder="Search items..."
                  className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                />
                <svg
                  className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                  />
                </svg>
              </div>
            </div>
            <button
              type="submit"
              className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
            >
              Search
            </button>
          </div>

          {/* Filter Row */}
          <div className="flex flex-wrap gap-4 items-center">
            <select
              value={category}
              onChange={(e) => handleCategoryChange(e.target.value)}
              className="px-3 py-2 text-sm border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            >
              {categories.map((cat) => (
                <option key={cat} value={cat}>
                  {cat}
                </option>
              ))}
            </select>

            <select
              value={listingType}
              onChange={(e) => handleListingTypeChange(e.target.value as ItemListingType | '')}
              className="px-3 py-2 text-sm border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            >
              {listingTypeFilters.map((type) => (
                <option key={type.value} value={type.value}>
                  {type.label}
                </option>
              ))}
            </select>

            <select
              value={condition}
              onChange={(e) => handleConditionChange(e.target.value as ItemCondition | '')}
              className="px-3 py-2 text-sm border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            >
              {conditionFilters.map((cond) => (
                <option key={cond.value} value={cond.value}>
                  {cond.label}
                </option>
              ))}
            </select>

            {/* Price Range */}
            <div className="flex items-center gap-2">
              <input
                type="number"
                value={minPrice}
                onChange={(e) => setMinPrice(e.target.value)}
                placeholder="Min €"
                min="0"
                className="w-24 px-3 py-2 text-sm border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
              />
              <span className="text-gray-400">-</span>
              <input
                type="number"
                value={maxPrice}
                onChange={(e) => setMaxPrice(e.target.value)}
                placeholder="Max €"
                min="0"
                className="w-24 px-3 py-2 text-sm border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
              />
            </div>

            <span className="ml-auto text-sm text-gray-500">
              {total} {total === 1 ? 'item' : 'items'}
            </span>
          </div>
        </form>
      </div>

      {/* Items Grid */}
      {isLoading ? (
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
          {[
            'skeleton-1',
            'skeleton-2',
            'skeleton-3',
            'skeleton-4',
            'skeleton-5',
            'skeleton-6',
            'skeleton-7',
            'skeleton-8',
          ].map((key) => (
            <div key={key} className="bg-white rounded-lg shadow animate-pulse">
              <div className="aspect-square bg-gray-200" />
              <div className="p-4 space-y-2">
                <div className="h-5 bg-gray-200 rounded w-1/2" />
                <div className="h-4 bg-gray-200 rounded w-3/4" />
                <div className="h-3 bg-gray-200 rounded w-1/3" />
              </div>
            </div>
          ))}
        </div>
      ) : items.length === 0 ? (
        <div className="bg-white rounded-lg shadow p-8 text-center">
          <svg
            className="mx-auto h-12 w-12 text-gray-400"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"
            />
          </svg>
          <h3 className="mt-4 text-lg font-medium text-gray-900">No items found</h3>
          <p className="mt-2 text-sm text-gray-500">
            Try adjusting your filters or create a new listing.
          </p>
          <button
            type="button"
            onClick={onNavigateToCreate}
            className="mt-4 inline-flex items-center px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
          >
            Create Listing
          </button>
        </div>
      ) : (
        <>
          <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
            {items.map((item) => (
              <ItemCard
                key={item.id}
                item={item}
                isCurrentUser={item.sellerId === currentUserId}
                onView={onNavigateToItem}
                onContact={onContactSeller}
                onEdit={onEditItem}
                onDelete={onDeleteItem}
                onMarkSold={onMarkSold}
              />
            ))}
          </div>

          {/* Pagination */}
          {totalPages > 1 && (
            <div className="flex items-center justify-center gap-2 mt-8">
              <button
                type="button"
                onClick={() => handlePageChange(page - 1)}
                disabled={page === 1}
                className="px-3 py-1 text-sm border rounded-md disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
              >
                Previous
              </button>
              <span className="text-sm text-gray-600">
                Page {page} of {totalPages}
              </span>
              <button
                type="button"
                onClick={() => handlePageChange(page + 1)}
                disabled={page === totalPages}
                className="px-3 py-1 text-sm border rounded-md disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
              >
                Next
              </button>
            </div>
          )}
        </>
      )}
    </div>
  );
}
