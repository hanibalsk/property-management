/**
 * FormList Component
 *
 * Displays a list of forms with filtering and pagination (Epic 54, Story 54.1).
 */

import type { FormStatus, FormSummary } from '@ppt/api-client';
import { useState } from 'react';
import { FormCard } from './FormCard';

interface FormListProps {
  forms: FormSummary[];
  total: number;
  page: number;
  pageSize: number;
  categories: string[];
  isLoading?: boolean;
  onPageChange: (page: number) => void;
  onStatusFilter: (status?: FormStatus) => void;
  onCategoryFilter: (category?: string) => void;
  onSearchChange: (search: string) => void;
  onView: (id: string) => void;
  onEdit: (id: string) => void;
  onDelete: (id: string) => void;
  onPublish: (id: string) => void;
  onArchive: (id: string) => void;
  onDuplicate: (id: string) => void;
  onCreate: () => void;
}

export function FormList({
  forms,
  total,
  page,
  pageSize,
  categories,
  isLoading,
  onPageChange,
  onStatusFilter,
  onCategoryFilter,
  onSearchChange,
  onView,
  onEdit,
  onDelete,
  onPublish,
  onArchive,
  onDuplicate,
  onCreate,
}: FormListProps) {
  const [statusFilter, setStatusFilter] = useState<FormStatus | ''>('');
  const [categoryFilter, setCategoryFilter] = useState<string>('');
  const [searchQuery, setSearchQuery] = useState<string>('');

  const totalPages = Math.ceil(total / pageSize);

  const handleStatusChange = (value: string) => {
    setStatusFilter(value as FormStatus | '');
    onStatusFilter(value ? (value as FormStatus) : undefined);
  };

  const handleCategoryChange = (value: string) => {
    setCategoryFilter(value);
    onCategoryFilter(value || undefined);
  };

  const handleSearchSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSearchChange(searchQuery);
  };

  return (
    <div>
      {/* Header with create button */}
      <div className="mb-6 flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <h1 className="text-2xl font-bold text-gray-900">Forms Management</h1>
        <button
          type="button"
          onClick={onCreate}
          className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors flex items-center gap-2"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <title>Create</title>
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M12 6v6m0 0v6m0-6h6m-6 0H6"
            />
          </svg>
          Create Form
        </button>
      </div>

      {/* Filters and Search */}
      <div className="mb-6 flex flex-wrap gap-4">
        <select
          value={statusFilter}
          onChange={(e) => handleStatusChange(e.target.value)}
          className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="">All Statuses</option>
          <option value="draft">Draft</option>
          <option value="published">Published</option>
          <option value="archived">Archived</option>
        </select>

        <select
          value={categoryFilter}
          onChange={(e) => handleCategoryChange(e.target.value)}
          className="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="">All Categories</option>
          {categories.map((cat) => (
            <option key={cat} value={cat}>
              {cat}
            </option>
          ))}
        </select>

        <form onSubmit={handleSearchSubmit} className="flex-1 min-w-[200px]">
          <div className="relative">
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search forms..."
              className="w-full px-3 py-2 pl-10 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <svg
              className="absolute left-3 top-2.5 w-5 h-5 text-gray-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <title>Search</title>
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
              />
            </svg>
          </div>
        </form>
      </div>

      {/* List */}
      {isLoading ? (
        <div className="flex items-center justify-center py-12">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
        </div>
      ) : forms.length === 0 ? (
        <div className="text-center py-12 text-gray-500">
          <svg
            className="mx-auto h-12 w-12 text-gray-400 mb-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <title>No forms</title>
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
            />
          </svg>
          <p className="text-lg font-medium mb-2">No forms found</p>
          <p className="mb-4">Get started by creating your first form template.</p>
          <button
            type="button"
            onClick={onCreate}
            className="text-blue-600 hover:text-blue-800 font-medium"
          >
            Create your first form
          </button>
        </div>
      ) : (
        <div className="grid gap-4">
          {forms.map((form) => (
            <FormCard
              key={form.id}
              form={form}
              onView={onView}
              onEdit={onEdit}
              onDelete={onDelete}
              onPublish={onPublish}
              onArchive={onArchive}
              onDuplicate={onDuplicate}
            />
          ))}
        </div>
      )}

      {/* Pagination */}
      {totalPages > 1 && (
        <div className="mt-6 flex items-center justify-between">
          <p className="text-sm text-gray-500">
            Showing {(page - 1) * pageSize + 1} to {Math.min(page * pageSize, total)} of {total}
          </p>
          <div className="flex gap-2">
            <button
              type="button"
              onClick={() => onPageChange(page - 1)}
              disabled={page === 1}
              className="px-3 py-1 border rounded-md disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
            >
              Previous
            </button>
            <span className="px-3 py-1">
              Page {page} of {totalPages}
            </span>
            <button
              type="button"
              onClick={() => onPageChange(page + 1)}
              disabled={page === totalPages}
              className="px-3 py-1 border rounded-md disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
            >
              Next
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
