/**
 * FormsPage Component
 *
 * Main forms listing page for managers (Epic 54, Story 54.1).
 */

import type { FormStatus, FormSummary, ListFormsParams } from '@ppt/api-client';
import { useState } from 'react';
import { FormList } from '../components/FormList';

interface FormsPageProps {
  forms: FormSummary[];
  total: number;
  categories: string[];
  isLoading?: boolean;
  onNavigateToCreate: () => void;
  onNavigateToView: (id: string) => void;
  onNavigateToEdit: (id: string) => void;
  onDelete: (id: string) => void;
  onPublish: (id: string) => void;
  onArchive: (id: string) => void;
  onDuplicate: (id: string) => void;
  onFilterChange: (params: ListFormsParams) => void;
}

export function FormsPage({
  forms,
  total,
  categories,
  isLoading,
  onNavigateToCreate,
  onNavigateToView,
  onNavigateToEdit,
  onDelete,
  onPublish,
  onArchive,
  onDuplicate,
  onFilterChange,
}: FormsPageProps) {
  const [page, setPage] = useState(1);
  const [pageSize] = useState(10);
  const [filters, setFilters] = useState<ListFormsParams>({});

  const handlePageChange = (newPage: number) => {
    setPage(newPage);
    onFilterChange({ ...filters, page: newPage, pageSize });
  };

  const handleStatusFilter = (status?: FormStatus) => {
    const newFilters = { ...filters, status, page: 1 };
    setFilters(newFilters);
    setPage(1);
    onFilterChange(newFilters);
  };

  const handleCategoryFilter = (category?: string) => {
    const newFilters = { ...filters, category, page: 1 };
    setFilters(newFilters);
    setPage(1);
    onFilterChange(newFilters);
  };

  const handleSearchChange = (search: string) => {
    const newFilters = { ...filters, search, page: 1 };
    setFilters(newFilters);
    setPage(1);
    onFilterChange(newFilters);
  };

  return (
    <div className="max-w-6xl mx-auto px-4 py-8">
      <FormList
        forms={forms}
        total={total}
        page={page}
        pageSize={pageSize}
        categories={categories}
        isLoading={isLoading}
        onPageChange={handlePageChange}
        onStatusFilter={handleStatusFilter}
        onCategoryFilter={handleCategoryFilter}
        onSearchChange={handleSearchChange}
        onView={onNavigateToView}
        onEdit={onNavigateToEdit}
        onDelete={onDelete}
        onPublish={onPublish}
        onArchive={onArchive}
        onDuplicate={onDuplicate}
        onCreate={onNavigateToCreate}
      />
    </div>
  );
}
