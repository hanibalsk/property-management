/**
 * DelegationsPage - main page listing delegations given and received.
 * Epic 3: Ownership Management (Story 3.4) - UC-28 Delegation History
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import {
  DelegationCard,
  type DelegationStatus,
  type DelegationSummary,
} from '../components/DelegationCard';
import type { DelegationActivity, DelegationHistorySummary } from '../types';

type TabType = 'given' | 'received' | 'history';

interface DelegationsPageProps {
  givenDelegations: DelegationSummary[];
  receivedDelegations: DelegationSummary[];
  historySummary?: DelegationHistorySummary;
  recentActivities?: DelegationActivity[];
  isLoading?: boolean;
  onNavigateToCreate: () => void;
  onNavigateToView: (id: string) => void;
  onNavigateToHistory: () => void;
  onAccept: (id: string) => void;
  onDecline: (id: string) => void;
  onRevoke: (id: string) => void;
}

export function DelegationsPage({
  givenDelegations,
  receivedDelegations,
  historySummary,
  recentActivities = [],
  isLoading,
  onNavigateToCreate,
  onNavigateToView,
  onNavigateToHistory,
  onAccept,
  onDecline,
  onRevoke,
}: DelegationsPageProps) {
  const { t } = useTranslation();
  const [activeTab, setActiveTab] = useState<TabType>('given');
  const [statusFilter, setStatusFilter] = useState<DelegationStatus | ''>('');

  const delegations = activeTab === 'given' ? givenDelegations : receivedDelegations;

  const filteredDelegations = statusFilter
    ? delegations.filter((d) => d.status === statusFilter)
    : delegations;

  const pendingReceivedCount = receivedDelegations.filter((d) => d.status === 'pending').length;

  const activityTypeLabels: Record<string, string> = {
    created: t('delegation.history.activityCreated'),
    accepted: t('delegation.history.activityAccepted'),
    declined: t('delegation.history.activityDeclined'),
    revoked: t('delegation.history.activityRevoked'),
    expired: t('delegation.history.activityExpired'),
    modified: t('delegation.history.activityModified'),
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
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">{t('delegation.title')}</h1>
          <p className="text-gray-600 mt-1">{t('delegation.subtitle')}</p>
        </div>
        <div className="flex gap-2">
          <button
            type="button"
            onClick={onNavigateToHistory}
            className="flex items-center gap-2 px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            {t('delegation.history.viewHistory')}
          </button>
          <button
            type="button"
            onClick={onNavigateToCreate}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
          >
            {t('delegation.createNew')}
          </button>
        </div>
      </div>

      {/* Tabs */}
      <div className="border-b border-gray-200 mb-6">
        <nav className="-mb-px flex space-x-8">
          <button
            type="button"
            onClick={() => setActiveTab('given')}
            className={`py-4 px-1 border-b-2 font-medium text-sm ${
              activeTab === 'given'
                ? 'border-blue-500 text-blue-600'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            }`}
          >
            {t('delegation.tabGiven')} ({givenDelegations.length})
          </button>
          <button
            type="button"
            onClick={() => setActiveTab('received')}
            className={`py-4 px-1 border-b-2 font-medium text-sm flex items-center gap-2 ${
              activeTab === 'received'
                ? 'border-blue-500 text-blue-600'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            }`}
          >
            {t('delegation.tabReceived')} ({receivedDelegations.length})
            {pendingReceivedCount > 0 && (
              <span className="px-2 py-0.5 text-xs bg-yellow-100 text-yellow-800 rounded-full">
                {pendingReceivedCount} {t('delegation.pending')}
              </span>
            )}
          </button>
          <button
            type="button"
            onClick={() => setActiveTab('history')}
            className={`py-4 px-1 border-b-2 font-medium text-sm flex items-center gap-2 ${
              activeTab === 'history'
                ? 'border-blue-500 text-blue-600'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            }`}
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            {t('delegation.tabHistory')}
          </button>
        </nav>
      </div>

      {/* History Tab Content */}
      {activeTab === 'history' && (
        <div className="space-y-6">
          {/* Quick Summary Widget */}
          {historySummary && (
            <div className="bg-white rounded-lg shadow p-6">
              <h2 className="text-lg font-semibold text-gray-900 mb-4">
                {t('delegation.history.summaryTitle')}
              </h2>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div className="text-center p-3 bg-gray-50 rounded-lg">
                  <p className="text-2xl font-bold text-gray-900">
                    {historySummary.totalDelegations}
                  </p>
                  <p className="text-xs text-gray-500">
                    {t('delegation.history.totalDelegations')}
                  </p>
                </div>
                <div className="text-center p-3 bg-green-50 rounded-lg">
                  <p className="text-2xl font-bold text-green-600">
                    {historySummary.activeDelegations}
                  </p>
                  <p className="text-xs text-gray-500">
                    {t('delegation.history.activeDelegations')}
                  </p>
                </div>
                <div className="text-center p-3 bg-orange-50 rounded-lg">
                  <p className="text-2xl font-bold text-orange-600">
                    {historySummary.revokedDelegations}
                  </p>
                  <p className="text-xs text-gray-500">
                    {t('delegation.history.revokedDelegations')}
                  </p>
                </div>
                <div className="text-center p-3 bg-gray-50 rounded-lg">
                  <p className="text-2xl font-bold text-gray-600">
                    {historySummary.expiredDelegations}
                  </p>
                  <p className="text-xs text-gray-500">
                    {t('delegation.history.expiredDelegations')}
                  </p>
                </div>
              </div>
            </div>
          )}

          {/* Recent Activities */}
          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-lg font-semibold text-gray-900">
                {t('delegation.history.recentActivity')}
              </h2>
              <button
                type="button"
                onClick={onNavigateToHistory}
                className="text-sm text-blue-600 hover:text-blue-800"
              >
                {t('delegation.history.viewAll')}
              </button>
            </div>

            {recentActivities.length === 0 ? (
              <div className="text-center py-8 text-gray-500">
                <svg
                  className="mx-auto h-10 w-10 text-gray-400 mb-2"
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
                <p>{t('delegation.history.noRecentActivity')}</p>
              </div>
            ) : (
              <div className="space-y-3">
                {recentActivities.slice(0, 5).map((activity) => (
                  <div
                    key={activity.id}
                    className="flex items-center justify-between py-2 border-b border-gray-100 last:border-0"
                  >
                    <div className="flex items-center gap-3">
                      <div className="w-8 h-8 rounded-full bg-gray-100 flex items-center justify-center">
                        <svg
                          className="w-4 h-4 text-gray-600"
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
                      </div>
                      <div>
                        <p className="text-sm font-medium text-gray-900">
                          {activityTypeLabels[activity.activityType]}
                        </p>
                        <p className="text-xs text-gray-500">
                          {t('delegation.history.byUser', { name: activity.performedByName })}
                        </p>
                      </div>
                    </div>
                    <time className="text-xs text-gray-400">
                      {new Date(activity.performedAt).toLocaleDateString()}
                    </time>
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* View Full History Button */}
          <div className="text-center">
            <button
              type="button"
              onClick={onNavigateToHistory}
              className="inline-flex items-center gap-2 px-6 py-3 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                />
              </svg>
              {t('delegation.history.viewFullHistory')}
            </button>
          </div>
        </div>
      )}

      {/* Given/Received Tab Content */}
      {activeTab !== 'history' && (
        <>
          {/* Filters */}
          <div className="mb-6 flex items-center gap-4">
            <label htmlFor="statusFilter" className="text-sm font-medium text-gray-700">
              {t('delegation.filterByStatus')}:
            </label>
            <select
              id="statusFilter"
              value={statusFilter}
              onChange={(e) => setStatusFilter(e.target.value as DelegationStatus | '')}
              className="rounded-md border border-gray-300 px-3 py-1.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="">{t('common.all')}</option>
              <option value="pending">{t('delegation.statusPending')}</option>
              <option value="active">{t('delegation.statusActive')}</option>
              <option value="revoked">{t('delegation.statusRevoked')}</option>
              <option value="expired">{t('delegation.statusExpired')}</option>
              <option value="declined">{t('delegation.statusDeclined')}</option>
            </select>
          </div>

          {/* Delegation List */}
          {filteredDelegations.length === 0 ? (
            <div className="text-center py-12 bg-gray-50 rounded-lg">
              <p className="text-gray-500">
                {activeTab === 'given'
                  ? t('delegation.noDelegationsGiven')
                  : t('delegation.noDelegationsReceived')}
              </p>
              {activeTab === 'given' && (
                <button
                  type="button"
                  onClick={onNavigateToCreate}
                  className="mt-4 text-blue-600 hover:text-blue-800"
                >
                  {t('delegation.createFirst')}
                </button>
              )}
            </div>
          ) : (
            <div className="space-y-4">
              {filteredDelegations.map((delegation) => (
                <DelegationCard
                  key={delegation.id}
                  delegation={delegation}
                  viewType={activeTab}
                  onView={onNavigateToView}
                  onAccept={activeTab === 'received' ? onAccept : undefined}
                  onDecline={activeTab === 'received' ? onDecline : undefined}
                  onRevoke={activeTab === 'given' ? onRevoke : undefined}
                />
              ))}
            </div>
          )}
        </>
      )}
    </div>
  );
}
