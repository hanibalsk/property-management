/**
 * PoliciesPage - List all insurance policies with filters.
 * Insurance Management Feature
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { PolicyCard } from '../components/PolicyCard';
import type { InsurancePolicy, PolicyStatus, PolicyType } from '../types';

interface PoliciesPageProps {
  policies: InsurancePolicy[];
  total: number;
  isLoading?: boolean;
  onNavigateToCreate: () => void;
  onNavigateToDashboard: () => void;
  onViewPolicy: (id: string) => void;
  onEditPolicy: (id: string) => void;
  onFileClaim: (policyId: string) => void;
  onFilterChange: (filters: PolicyFilterParams) => void;
}

interface PolicyFilterParams {
  status?: PolicyStatus;
  type?: PolicyType;
  buildingId?: string;
  search?: string;
  page: number;
  pageSize: number;
}

export function PoliciesPage({
  policies,
  total,
  isLoading,
  onNavigateToCreate,
  onNavigateToDashboard,
  onViewPolicy,
  onEditPolicy,
  onFileClaim,
  onFilterChange,
}: PoliciesPageProps) {
  const { t } = useTranslation();
  const [page, setPage] = useState(1);
  const [pageSize] = useState(12);
  const [filters, setFilters] = useState<Omit<PolicyFilterParams, 'page' | 'pageSize'>>({});

  const statusOptions: { value: PolicyStatus | ''; label: string }[] = [
    { value: '', label: t('common.all') },
    { value: 'active', label: t('insurance.statusActive') },
    { value: 'expired', label: t('insurance.statusExpired') },
    { value: 'cancelled', label: t('insurance.statusCancelled') },
    { value: 'pending', label: t('insurance.statusPending') },
  ];

  const typeOptions: { value: PolicyType | ''; label: string }[] = [
    { value: '', label: t('common.all') },
    { value: 'building', label: t('insurance.typeBuilding') },
    { value: 'liability', label: t('insurance.typeLiability') },
    { value: 'property', label: t('insurance.typeProperty') },
    { value: 'flood', label: t('insurance.typeFlood') },
    { value: 'earthquake', label: t('insurance.typeEarthquake') },
    { value: 'umbrella', label: t('insurance.typeUmbrella') },
    { value: 'directors_officers', label: t('insurance.typeDirectorsOfficers') },
    { value: 'workers_comp', label: t('insurance.typeWorkersComp') },
    { value: 'other', label: t('insurance.typeOther') },
  ];

  const handleStatusChange = (status: PolicyStatus | '') => {
    const newFilters = { ...filters, status: status || undefined };
    setFilters(newFilters);
    setPage(1);
    onFilterChange({ ...newFilters, page: 1, pageSize });
  };

  const handleTypeChange = (type: PolicyType | '') => {
    const newFilters = { ...filters, type: type || undefined };
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

  const handlePageChange = (newPage: number) => {
    setPage(newPage);
    onFilterChange({ ...filters, page: newPage, pageSize });
  };

  const totalPages = Math.ceil(total / pageSize);

  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
            <div>
              <button
                type="button"
                onClick={onNavigateToDashboard}
                className="text-sm text-blue-600 hover:text-blue-800 mb-2"
              >
                {t('insurance.backToDashboard')}
              </button>
              <h1 className="text-2xl font-bold text-gray-900">{t('insurance.policies.title')}</h1>
            </div>
            <button
              type="button"
              onClick={onNavigateToCreate}
              className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
            >
              {t('insurance.addPolicy')}
            </button>
          </div>

          {/* Filters */}
          <div className="mt-4 flex flex-col sm:flex-row gap-4">
            <div className="flex-1">
              <input
                type="text"
                placeholder={t('insurance.searchPolicies')}
                onChange={(e) => handleSearch(e.target.value)}
                className="w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <select
              value={filters.status || ''}
              onChange={(e) => handleStatusChange(e.target.value as PolicyStatus | '')}
              className="rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              {statusOptions.map((opt) => (
                <option key={opt.value} value={opt.value}>
                  {opt.label}
                </option>
              ))}
            </select>
            <select
              value={filters.type || ''}
              onChange={(e) => handleTypeChange(e.target.value as PolicyType | '')}
              className="rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              {typeOptions.map((opt) => (
                <option key={opt.value} value={opt.value}>
                  {opt.label}
                </option>
              ))}
            </select>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {isLoading ? (
          <div className="flex justify-center py-12">
            <div className="animate-spin rounded-full h-10 w-10 border-b-2 border-blue-600" />
          </div>
        ) : policies.length === 0 ? (
          <div className="bg-white rounded-lg shadow p-12 text-center">
            <svg
              className="mx-auto h-12 w-12 text-gray-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
              />
            </svg>
            <h3 className="mt-4 text-lg font-medium text-gray-900">{t('insurance.noPolicies')}</h3>
            <p className="mt-2 text-gray-500">{t('insurance.noPoliciesDescription')}</p>
            <button
              type="button"
              onClick={onNavigateToCreate}
              className="mt-4 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
            >
              {t('insurance.addPolicy')}
            </button>
          </div>
        ) : (
          <>
            <div className="mb-4 text-sm text-gray-500">
              {t('common.showing')} {(page - 1) * pageSize + 1} {t('common.to')}{' '}
              {Math.min(page * pageSize, total)} {t('common.of')} {total}
            </div>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {policies.map((policy) => (
                <PolicyCard
                  key={policy.id}
                  policy={policy}
                  onView={onViewPolicy}
                  onEdit={onEditPolicy}
                  onFileClaim={onFileClaim}
                />
              ))}
            </div>

            {/* Pagination */}
            {totalPages > 1 && (
              <div className="mt-8 flex justify-center gap-2">
                <button
                  type="button"
                  onClick={() => handlePageChange(page - 1)}
                  disabled={page === 1}
                  className="px-3 py-1 border rounded disabled:opacity-50"
                >
                  {t('common.previous')}
                </button>
                <span className="px-3 py-1">
                  {page} / {totalPages}
                </span>
                <button
                  type="button"
                  onClick={() => handlePageChange(page + 1)}
                  disabled={page === totalPages}
                  className="px-3 py-1 border rounded disabled:opacity-50"
                >
                  {t('common.next')}
                </button>
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
}
