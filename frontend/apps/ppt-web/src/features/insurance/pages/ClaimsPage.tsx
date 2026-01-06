/**
 * ClaimsPage - List all insurance claims with filters.
 * Insurance Management Feature
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { ClaimCard } from '../components/ClaimCard';
import type { ClaimStatus, ClaimType, InsuranceClaim } from '../types';

interface ClaimsPageProps {
  claims: InsuranceClaim[];
  total: number;
  isLoading?: boolean;
  onNavigateToFileClaim: () => void;
  onNavigateToDashboard: () => void;
  onViewClaim: (id: string) => void;
  onFilterChange: (filters: ClaimFilterParams) => void;
}

interface ClaimFilterParams {
  status?: ClaimStatus;
  type?: ClaimType;
  search?: string;
  page: number;
  pageSize: number;
}

export function ClaimsPage({
  claims,
  total,
  isLoading,
  onNavigateToFileClaim,
  onNavigateToDashboard,
  onViewClaim,
  onFilterChange,
}: ClaimsPageProps) {
  const { t } = useTranslation();
  const [page, setPage] = useState(1);
  const [pageSize] = useState(12);
  const [filters, setFilters] = useState<Omit<ClaimFilterParams, 'page' | 'pageSize'>>({});

  const statusOptions: { value: ClaimStatus | ''; label: string }[] = [
    { value: '', label: t('common.all') },
    { value: 'submitted', label: t('insurance.claimStatusSubmitted') },
    { value: 'under_review', label: t('insurance.claimStatusUnderReview') },
    { value: 'approved', label: t('insurance.claimStatusApproved') },
    { value: 'denied', label: t('insurance.claimStatusDenied') },
    { value: 'settled', label: t('insurance.claimStatusSettled') },
    { value: 'closed', label: t('insurance.claimStatusClosed') },
  ];

  const typeOptions: { value: ClaimType | ''; label: string }[] = [
    { value: '', label: t('common.all') },
    { value: 'property_damage', label: t('insurance.claimTypePropertyDamage') },
    { value: 'liability', label: t('insurance.claimTypeLiability') },
    { value: 'theft', label: t('insurance.claimTypeTheft') },
    { value: 'water_damage', label: t('insurance.claimTypeWaterDamage') },
    { value: 'fire_damage', label: t('insurance.claimTypeFireDamage') },
    { value: 'natural_disaster', label: t('insurance.claimTypeNaturalDisaster') },
    { value: 'personal_injury', label: t('insurance.claimTypePersonalInjury') },
    { value: 'other', label: t('insurance.claimTypeOther') },
  ];

  const handleStatusChange = (status: ClaimStatus | '') => {
    const newFilters = { ...filters, status: status || undefined };
    setFilters(newFilters);
    setPage(1);
    onFilterChange({ ...newFilters, page: 1, pageSize });
  };

  const handleTypeChange = (type: ClaimType | '') => {
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
              <h1 className="text-2xl font-bold text-gray-900">{t('insurance.claims.title')}</h1>
            </div>
            <button
              type="button"
              onClick={onNavigateToFileClaim}
              className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
            >
              {t('insurance.fileClaim')}
            </button>
          </div>

          {/* Filters */}
          <div className="mt-4 flex flex-col sm:flex-row gap-4">
            <div className="flex-1">
              <input
                type="text"
                placeholder={t('insurance.searchClaims')}
                onChange={(e) => handleSearch(e.target.value)}
                className="w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <select
              value={filters.status || ''}
              onChange={(e) => handleStatusChange(e.target.value as ClaimStatus | '')}
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
              onChange={(e) => handleTypeChange(e.target.value as ClaimType | '')}
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
        ) : claims.length === 0 ? (
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
                d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
              />
            </svg>
            <h3 className="mt-4 text-lg font-medium text-gray-900">{t('insurance.noClaims')}</h3>
            <p className="mt-2 text-gray-500">{t('insurance.noClaimsDescription')}</p>
            <button
              type="button"
              onClick={onNavigateToFileClaim}
              className="mt-4 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
            >
              {t('insurance.fileClaim')}
            </button>
          </div>
        ) : (
          <>
            <div className="mb-4 text-sm text-gray-500">
              {t('common.showing')} {(page - 1) * pageSize + 1} {t('common.to')}{' '}
              {Math.min(page * pageSize, total)} {t('common.of')} {total}
            </div>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {claims.map((claim) => (
                <ClaimCard key={claim.id} claim={claim} onView={onViewClaim} />
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
