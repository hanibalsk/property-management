/**
 * DisputesPage - main page listing all disputes.
 * Epic 77: Dispute Resolution (Story 77.1)
 */

import { useState } from 'react';
import type {
  DisputeCategory,
  DisputePriority,
  DisputeStatus,
  DisputeSummary,
} from '../components/DisputeCard';
import { DisputeList } from '../components/DisputeList';

interface DisputeListParams {
  status?: DisputeStatus;
  priority?: DisputePriority;
  category?: DisputeCategory;
  search?: string;
  page: number;
  pageSize: number;
}

interface DisputesPageProps {
  disputes: DisputeSummary[];
  total: number;
  isLoading?: boolean;
  onNavigateToCreate: () => void;
  onNavigateToView: (id: string) => void;
  onNavigateToManage: (id: string) => void;
  onFilterChange: (params: DisputeListParams) => void;
}

export function DisputesPage({
  disputes,
  total,
  isLoading,
  onNavigateToCreate,
  onNavigateToView,
  onNavigateToManage,
  onFilterChange,
}: DisputesPageProps) {
  const [page, setPage] = useState(1);
  const [pageSize] = useState(10);
  const [filters, setFilters] = useState<Omit<DisputeListParams, 'page' | 'pageSize'>>({});

  const handlePageChange = (newPage: number) => {
    setPage(newPage);
    onFilterChange({ ...filters, page: newPage, pageSize });
  };

  const handleStatusFilter = (status?: DisputeStatus) => {
    const newFilters = { ...filters, status };
    setFilters(newFilters);
    setPage(1);
    onFilterChange({ ...newFilters, page: 1, pageSize });
  };

  const handlePriorityFilter = (priority?: DisputePriority) => {
    const newFilters = { ...filters, priority };
    setFilters(newFilters);
    setPage(1);
    onFilterChange({ ...newFilters, page: 1, pageSize });
  };

  const handleCategoryFilter = (category?: DisputeCategory) => {
    const newFilters = { ...filters, category };
    setFilters(newFilters);
    setPage(1);
    onFilterChange({ ...newFilters, page: 1, pageSize });
  };

  const handleSearch = (search: string) => {
    const newFilters = { ...filters, search: search || undefined };
    setFilters(newFilters);
    setPage(1);
    onFilterChange({ ...newFilters, page: 1, pageSize });
  };

  return (
    <div className="max-w-4xl mx-auto px-4 py-8">
      <DisputeList
        disputes={disputes}
        total={total}
        page={page}
        pageSize={pageSize}
        isLoading={isLoading}
        onPageChange={handlePageChange}
        onStatusFilter={handleStatusFilter}
        onPriorityFilter={handlePriorityFilter}
        onCategoryFilter={handleCategoryFilter}
        onSearch={handleSearch}
        onView={onNavigateToView}
        onManage={onNavigateToManage}
        onCreate={onNavigateToCreate}
      />
    </div>
  );
}
