/**
 * GroupList Component
 *
 * Displays a list/grid of community groups with filtering.
 * Part of Story 42.1: Community Groups.
 */

import type { CommunityGroupSummary, GroupVisibility, ListGroupsParams } from '@ppt/api-client';
import { useState } from 'react';
import { GroupCard } from './GroupCard';

interface GroupListProps {
  groups: CommunityGroupSummary[];
  total: number;
  page: number;
  pageSize: number;
  isLoading?: boolean;
  joiningGroupId?: string;
  leavingGroupId?: string;
  onPageChange: (page: number) => void;
  onFilterChange: (params: ListGroupsParams) => void;
  onView: (id: string) => void;
  onJoin: (id: string) => void;
  onLeave: (id: string) => void;
  onCreate: () => void;
}

const categories = [
  'All Categories',
  'Social',
  'Sports',
  'Hobbies',
  'Parenting',
  'Pets',
  'Business',
  'Other',
];

const visibilityOptions: { value: GroupVisibility | ''; label: string }[] = [
  { value: '', label: 'All Groups' },
  { value: 'public', label: 'Public' },
  { value: 'private', label: 'Private' },
];

export function GroupList({
  groups,
  total,
  page,
  pageSize,
  isLoading,
  joiningGroupId,
  leavingGroupId,
  onPageChange,
  onFilterChange,
  onView,
  onJoin,
  onLeave,
  onCreate,
}: GroupListProps) {
  const [search, setSearch] = useState('');
  const [category, setCategory] = useState('All Categories');
  const [visibility, setVisibility] = useState<GroupVisibility | ''>('');
  const [memberOnly, setMemberOnly] = useState(false);

  const totalPages = Math.ceil(total / pageSize);

  const handleSearchSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onFilterChange({
      search: search || undefined,
      category: category !== 'All Categories' ? category : undefined,
      visibility: visibility || undefined,
      memberOnly: memberOnly || undefined,
      page: 1,
    });
  };

  const handleCategoryChange = (newCategory: string) => {
    setCategory(newCategory);
    onFilterChange({
      search: search || undefined,
      category: newCategory !== 'All Categories' ? newCategory : undefined,
      visibility: visibility || undefined,
      memberOnly: memberOnly || undefined,
      page: 1,
    });
  };

  const handleVisibilityChange = (newVisibility: GroupVisibility | '') => {
    setVisibility(newVisibility);
    onFilterChange({
      search: search || undefined,
      category: category !== 'All Categories' ? category : undefined,
      visibility: newVisibility || undefined,
      memberOnly: memberOnly || undefined,
      page: 1,
    });
  };

  const handleMemberOnlyChange = (checked: boolean) => {
    setMemberOnly(checked);
    onFilterChange({
      search: search || undefined,
      category: category !== 'All Categories' ? category : undefined,
      visibility: visibility || undefined,
      memberOnly: checked || undefined,
      page: 1,
    });
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Community Groups</h1>
          <p className="mt-1 text-sm text-gray-500">
            Connect with your neighbors through shared interests
          </p>
        </div>
        <button
          type="button"
          onClick={onCreate}
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
          Create Group
        </button>
      </div>

      {/* Filters */}
      <div className="bg-white rounded-lg shadow p-4">
        <form onSubmit={handleSearchSubmit} className="flex flex-wrap gap-4 items-end">
          {/* Search */}
          <div className="flex-1 min-w-[200px]">
            <label htmlFor="search" className="block text-sm font-medium text-gray-700 mb-1">
              Search
            </label>
            <div className="relative">
              <input
                type="text"
                id="search"
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                placeholder="Search groups..."
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

          {/* Category */}
          <div className="min-w-[150px]">
            <label htmlFor="category" className="block text-sm font-medium text-gray-700 mb-1">
              Category
            </label>
            <select
              id="category"
              value={category}
              onChange={(e) => handleCategoryChange(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            >
              {categories.map((cat) => (
                <option key={cat} value={cat}>
                  {cat}
                </option>
              ))}
            </select>
          </div>

          {/* Visibility */}
          <div className="min-w-[120px]">
            <label htmlFor="visibility" className="block text-sm font-medium text-gray-700 mb-1">
              Visibility
            </label>
            <select
              id="visibility"
              value={visibility}
              onChange={(e) => handleVisibilityChange(e.target.value as GroupVisibility | '')}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            >
              {visibilityOptions.map((opt) => (
                <option key={opt.value} value={opt.value}>
                  {opt.label}
                </option>
              ))}
            </select>
          </div>

          {/* Member Only Toggle */}
          <div className="flex items-center">
            <label className="flex items-center cursor-pointer">
              <input
                type="checkbox"
                checked={memberOnly}
                onChange={(e) => handleMemberOnlyChange(e.target.checked)}
                className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
              />
              <span className="ml-2 text-sm text-gray-700">My Groups Only</span>
            </label>
          </div>

          <button
            type="submit"
            className="px-4 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200"
          >
            Search
          </button>
        </form>
      </div>

      {/* Results */}
      {isLoading ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {['skeleton-1', 'skeleton-2', 'skeleton-3', 'skeleton-4', 'skeleton-5', 'skeleton-6'].map(
            (key) => (
              <div key={key} className="bg-white rounded-lg shadow animate-pulse">
                <div className="h-32 bg-gray-200" />
                <div className="p-4 space-y-3">
                  <div className="h-5 bg-gray-200 rounded w-3/4" />
                  <div className="h-4 bg-gray-200 rounded w-1/2" />
                  <div className="h-4 bg-gray-200 rounded w-full" />
                </div>
              </div>
            )
          )}
        </div>
      ) : groups.length === 0 ? (
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
              d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
            />
          </svg>
          <h3 className="mt-4 text-lg font-medium text-gray-900">No groups found</h3>
          <p className="mt-2 text-sm text-gray-500">
            {memberOnly
              ? "You haven't joined any groups yet."
              : 'Try adjusting your filters or create a new group.'}
          </p>
          <button
            type="button"
            onClick={onCreate}
            className="mt-4 inline-flex items-center px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
          >
            Create Group
          </button>
        </div>
      ) : (
        <>
          <div className="flex items-center justify-between text-sm text-gray-500">
            <span>
              Showing {groups.length} of {total} groups
            </span>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {groups.map((group) => (
              <GroupCard
                key={group.id}
                group={group}
                onView={onView}
                onJoin={onJoin}
                onLeave={onLeave}
                isJoining={joiningGroupId === group.id}
                isLeaving={leavingGroupId === group.id}
              />
            ))}
          </div>
        </>
      )}

      {/* Pagination */}
      {totalPages > 1 && (
        <div className="flex items-center justify-center gap-2">
          <button
            type="button"
            onClick={() => onPageChange(page - 1)}
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
            onClick={() => onPageChange(page + 1)}
            disabled={page === totalPages}
            className="px-3 py-1 text-sm border rounded-md disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
          >
            Next
          </button>
        </div>
      )}
    </div>
  );
}
