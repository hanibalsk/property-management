/**
 * InsuranceDashboardPage - Overview of insurance policies and claims.
 * Displays statistics, expiring policies, and recent claims.
 * Insurance Management Feature
 */

import { useTranslation } from 'react-i18next';
import { ClaimCard } from '../components/ClaimCard';
import { PolicyCard } from '../components/PolicyCard';
import type { InsuranceClaim, InsurancePolicy, InsuranceStatistics } from '../types';

interface InsuranceDashboardPageProps {
  statistics: InsuranceStatistics;
  expiringPolicies: InsurancePolicy[];
  recentClaims: InsuranceClaim[];
  isLoading?: boolean;
  onNavigateToPolicies: () => void;
  onNavigateToClaims: () => void;
  onNavigateToCreatePolicy: () => void;
  onNavigateToFileClaim: () => void;
  onViewPolicy: (id: string) => void;
  onEditPolicy: (id: string) => void;
  onFileClaimForPolicy: (policyId: string) => void;
  onViewClaim: (id: string) => void;
}

export function InsuranceDashboardPage({
  statistics,
  expiringPolicies,
  recentClaims,
  isLoading,
  onNavigateToPolicies,
  onNavigateToClaims,
  onNavigateToCreatePolicy,
  onNavigateToFileClaim,
  onViewPolicy,
  onEditPolicy,
  onFileClaimForPolicy,
  onViewClaim,
}: InsuranceDashboardPageProps) {
  const { t } = useTranslation();

  const formatCurrency = (amount: number, currency: string) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: currency,
      maximumFractionDigits: 0,
    }).format(amount);
  };

  if (isLoading) {
    return (
      <div className="flex justify-center py-12">
        <div className="animate-spin rounded-full h-10 w-10 border-b-2 border-blue-600" />
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
            <div>
              <h1 className="text-2xl font-bold text-gray-900">{t('insurance.dashboard.title')}</h1>
              <p className="mt-1 text-gray-600">{t('insurance.dashboard.subtitle')}</p>
            </div>
            <div className="flex gap-3">
              <button
                type="button"
                onClick={onNavigateToCreatePolicy}
                className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
              >
                {t('insurance.addPolicy')}
              </button>
              <button
                type="button"
                onClick={onNavigateToFileClaim}
                className="px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50"
              >
                {t('insurance.fileClaim')}
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Statistics Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-3 rounded-full bg-blue-100 text-blue-600">
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
                  />
                </svg>
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">
                  {t('insurance.stats.activePolicies')}
                </p>
                <p className="text-2xl font-semibold text-gray-900">{statistics.activePolicies}</p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-3 rounded-full bg-green-100 text-green-600">
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                  />
                </svg>
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">
                  {t('insurance.stats.totalCoverage')}
                </p>
                <p className="text-2xl font-semibold text-gray-900">
                  {formatCurrency(statistics.totalCoverage, statistics.currency)}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-3 rounded-full bg-yellow-100 text-yellow-600">
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                  />
                </svg>
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">
                  {t('insurance.stats.expiringPolicies')}
                </p>
                <p className="text-2xl font-semibold text-gray-900">
                  {statistics.expiringPolicies}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-3 rounded-full bg-purple-100 text-purple-600">
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
                  />
                </svg>
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">
                  {t('insurance.stats.pendingClaims')}
                </p>
                <p className="text-2xl font-semibold text-gray-900">{statistics.pendingClaims}</p>
              </div>
            </div>
          </div>
        </div>

        {/* Additional Stats Row */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
          <div className="bg-white rounded-lg shadow p-6">
            <p className="text-sm font-medium text-gray-500">
              {t('insurance.stats.totalPremiums')}
            </p>
            <p className="text-xl font-semibold text-gray-900 mt-1">
              {formatCurrency(statistics.totalPremiums, statistics.currency)}
              <span className="text-sm text-gray-500 font-normal">
                {' '}
                / {t('insurance.frequency.annually')}
              </span>
            </p>
          </div>
          <div className="bg-white rounded-lg shadow p-6">
            <p className="text-sm font-medium text-gray-500">{t('insurance.stats.totalClaims')}</p>
            <p className="text-xl font-semibold text-gray-900 mt-1">{statistics.totalClaims}</p>
          </div>
          <div className="bg-white rounded-lg shadow p-6">
            <p className="text-sm font-medium text-gray-500">
              {t('insurance.stats.approvedClaimsAmount')}
            </p>
            <p className="text-xl font-semibold text-green-600 mt-1">
              {formatCurrency(statistics.approvedClaimsAmount, statistics.currency)}
            </p>
          </div>
        </div>

        {/* Expiring Policies */}
        <div className="mb-8">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold text-gray-900">
              {t('insurance.expiringPolicies')}
            </h2>
            <button
              type="button"
              onClick={onNavigateToPolicies}
              className="text-sm text-blue-600 hover:text-blue-800"
            >
              {t('insurance.viewAllPolicies')}
            </button>
          </div>
          {expiringPolicies.length === 0 ? (
            <div className="bg-white rounded-lg shadow p-6 text-center text-gray-500">
              {t('insurance.noExpiringPolicies')}
            </div>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {expiringPolicies.map((policy) => (
                <PolicyCard
                  key={policy.id}
                  policy={policy}
                  onView={onViewPolicy}
                  onEdit={onEditPolicy}
                  onFileClaim={onFileClaimForPolicy}
                />
              ))}
            </div>
          )}
        </div>

        {/* Recent Claims */}
        <div>
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold text-gray-900">{t('insurance.recentClaims')}</h2>
            <button
              type="button"
              onClick={onNavigateToClaims}
              className="text-sm text-blue-600 hover:text-blue-800"
            >
              {t('insurance.viewAllClaims')}
            </button>
          </div>
          {recentClaims.length === 0 ? (
            <div className="bg-white rounded-lg shadow p-6 text-center text-gray-500">
              {t('insurance.noClaims')}
            </div>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {recentClaims.map((claim) => (
                <ClaimCard key={claim.id} claim={claim} onView={onViewClaim} />
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
