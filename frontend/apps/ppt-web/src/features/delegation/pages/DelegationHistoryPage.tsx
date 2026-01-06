/**
 * DelegationHistoryPage - View historical delegation activity.
 * Epic 3: Ownership Management (Story 3.4) - UC-28 Delegation History
 */

import { useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { DelegationActivityItem } from '../components/DelegationActivityItem';
import type { DelegationActivity, DelegationActivityType, DelegationHistoryFilter } from '../types';

interface DelegationHistoryPageProps {
  activities: DelegationActivity[];
  isLoading?: boolean;
  totalCount: number;
  currentPage: number;
  pageSize: number;
  onPageChange: (page: number) => void;
  onFilterChange: (filter: DelegationHistoryFilter) => void;
  onExportCsv: () => void;
  onBack: () => void;
}

const ACTIVITY_TYPES: DelegationActivityType[] = [
  'created',
  'accepted',
  'declined',
  'revoked',
  'expired',
  'modified',
];

export function DelegationHistoryPage({
  activities,
  isLoading,
  totalCount,
  currentPage,
  pageSize,
  onPageChange,
  onFilterChange,
  onExportCsv,
  onBack,
}: DelegationHistoryPageProps) {
  const { t } = useTranslation();
  const [startDate, setStartDate] = useState('');
  const [endDate, setEndDate] = useState('');
  const [selectedTypes, setSelectedTypes] = useState<DelegationActivityType[]>([]);

  const totalPages = Math.ceil(totalCount / pageSize);

  const activityTypeLabels: Record<DelegationActivityType, string> = useMemo(
    () => ({
      created: t('delegation.history.activityCreated'),
      accepted: t('delegation.history.activityAccepted'),
      declined: t('delegation.history.activityDeclined'),
      revoked: t('delegation.history.activityRevoked'),
      expired: t('delegation.history.activityExpired'),
      modified: t('delegation.history.activityModified'),
    }),
    [t]
  );

  const handleApplyFilters = () => {
    const filter: DelegationHistoryFilter = {};
    if (startDate) filter.startDate = startDate;
    if (endDate) filter.endDate = endDate;
    if (selectedTypes.length > 0) filter.activityTypes = selectedTypes;
    onFilterChange(filter);
  };

  const handleClearFilters = () => {
    setStartDate('');
    setEndDate('');
    setSelectedTypes([]);
    onFilterChange({});
  };

  const handleTypeToggle = (type: DelegationActivityType) => {
    setSelectedTypes((prev) =>
      prev.includes(type) ? prev.filter((t) => t !== type) : [...prev, type]
    );
  };

  if (isLoading) {
    return (
      <div className="flex justify-center py-12">
        <div className="animate-spin rounded-full h-10 w-10 border-b-2 border-blue-600" />
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-6">
        <button
          type="button"
          onClick={onBack}
          className="text-sm text-blue-600 hover:text-blue-800 flex items-center gap-1 mb-4"
        >
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M15 19l-7-7 7-7"
            />
          </svg>
          {t('delegation.backToDelegations')}
        </button>
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold text-gray-900">{t('delegation.history.title')}</h1>
            <p className="text-gray-600 mt-1">{t('delegation.history.subtitle')}</p>
          </div>
          <button
            type="button"
            onClick={onExportCsv}
            className="flex items-center gap-2 px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
              />
            </svg>
            {t('delegation.history.exportCsv')}
          </button>
        </div>
      </div>

      {/* Filters */}
      <div className="bg-white rounded-lg shadow p-4 mb-6">
        <h2 className="text-sm font-medium text-gray-700 mb-4">
          {t('delegation.history.filters')}
        </h2>

        {/* Date Range */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
          <div>
            <label htmlFor="startDate" className="block text-sm text-gray-600 mb-1">
              {t('delegation.history.startDate')}
            </label>
            <input
              type="date"
              id="startDate"
              value={startDate}
              onChange={(e) => setStartDate(e.target.value)}
              className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
          <div>
            <label htmlFor="endDate" className="block text-sm text-gray-600 mb-1">
              {t('delegation.history.endDate')}
            </label>
            <input
              type="date"
              id="endDate"
              value={endDate}
              onChange={(e) => setEndDate(e.target.value)}
              className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
        </div>

        {/* Activity Type Filter */}
        <div className="mb-4">
          <label className="block text-sm text-gray-600 mb-2">
            {t('delegation.history.activityType')}
          </label>
          <div className="flex flex-wrap gap-2">
            {ACTIVITY_TYPES.map((type) => (
              <button
                key={type}
                type="button"
                onClick={() => handleTypeToggle(type)}
                className={`px-3 py-1.5 text-xs rounded-full transition-colors ${
                  selectedTypes.includes(type)
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                }`}
              >
                {activityTypeLabels[type]}
              </button>
            ))}
          </div>
        </div>

        {/* Filter Actions */}
        <div className="flex gap-2">
          <button
            type="button"
            onClick={handleApplyFilters}
            className="px-4 py-2 bg-blue-600 text-white text-sm rounded-lg hover:bg-blue-700"
          >
            {t('delegation.history.applyFilters')}
          </button>
          <button
            type="button"
            onClick={handleClearFilters}
            className="px-4 py-2 border border-gray-300 text-sm rounded-lg hover:bg-gray-50"
          >
            {t('delegation.history.clearFilters')}
          </button>
        </div>
      </div>

      {/* Results Summary */}
      <div className="flex items-center justify-between mb-4">
        <p className="text-sm text-gray-600">
          {t('delegation.history.showing', {
            from: (currentPage - 1) * pageSize + 1,
            to: Math.min(currentPage * pageSize, totalCount),
            total: totalCount,
          })}
        </p>
      </div>

      {/* Activity Timeline */}
      {activities.length === 0 ? (
        <div className="text-center py-12 bg-gray-50 rounded-lg">
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
              d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <p className="mt-4 text-gray-500">{t('delegation.history.noActivities')}</p>
        </div>
      ) : (
        <div className="bg-white rounded-lg shadow p-6">
          <div className="space-y-0">
            {activities.map((activity, index) => (
              <DelegationActivityItem
                key={activity.id}
                activity={activity}
                isFirst={index === 0}
                isLast={index === activities.length - 1}
              />
            ))}
          </div>
        </div>
      )}

      {/* Pagination */}
      {totalPages > 1 && (
        <div className="mt-6 flex items-center justify-between">
          <button
            type="button"
            onClick={() => onPageChange(currentPage - 1)}
            disabled={currentPage === 1}
            className="px-4 py-2 border border-gray-300 rounded-lg text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
          >
            {t('common.previous')}
          </button>
          <div className="flex items-center gap-2">
            {Array.from({ length: totalPages }, (_, i) => i + 1)
              .filter(
                (page) =>
                  page === 1 ||
                  page === totalPages ||
                  (page >= currentPage - 1 && page <= currentPage + 1)
              )
              .map((page, index, array) => (
                <span key={page}>
                  {index > 0 && array[index - 1] !== page - 1 && (
                    <span className="px-2 text-gray-400">...</span>
                  )}
                  <button
                    type="button"
                    onClick={() => onPageChange(page)}
                    className={`w-10 h-10 rounded-lg text-sm ${
                      page === currentPage
                        ? 'bg-blue-600 text-white'
                        : 'border border-gray-300 hover:bg-gray-50'
                    }`}
                  >
                    {page}
                  </button>
                </span>
              ))}
          </div>
          <button
            type="button"
            onClick={() => onPageChange(currentPage + 1)}
            disabled={currentPage === totalPages}
            className="px-4 py-2 border border-gray-300 rounded-lg text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
          >
            {t('common.next')}
          </button>
        </div>
      )}
    </div>
  );
}
