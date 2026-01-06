/**
 * PlansPage - Compare and select subscription plans.
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { PlanCard } from '../components';
import type { Plan, PlanInterval, TrialStatus } from '../types';

interface PlansPageProps {
  plans: Plan[];
  currentPlanId?: string;
  trialStatus?: TrialStatus;
  trialEligiblePlanIds?: string[];
  isLoading?: boolean;
  onSelectPlan?: (planId: string) => void;
  onStartTrial?: (planId: string) => void;
  onBack?: () => void;
}

export function PlansPage({
  plans,
  currentPlanId,
  trialStatus,
  trialEligiblePlanIds = [],
  isLoading,
  onSelectPlan,
  onStartTrial,
  onBack,
}: PlansPageProps) {
  const { t } = useTranslation();
  const [selectedInterval, setSelectedInterval] = useState<PlanInterval>('monthly');

  const isInTrial = trialStatus?.status === 'active';
  const trialDaysRemaining = trialStatus?.daysRemaining ?? 0;

  const filteredPlans = plans.filter(
    (plan) => plan.interval === selectedInterval && plan.status === 'active'
  );

  const hasYearlyPlans = plans.some(
    (plan) => plan.interval === 'yearly' && plan.status === 'active'
  );
  const hasMonthlyPlans = plans.some(
    (plan) => plan.interval === 'monthly' && plan.status === 'active'
  );

  // Find the "popular" plan (middle tier or explicitly marked)
  const popularPlanIndex = Math.floor(filteredPlans.length / 2);

  if (isLoading) {
    return (
      <div className="min-h-screen bg-gray-100">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
          <div className="animate-pulse space-y-6">
            <div className="h-8 w-64 bg-gray-200 rounded" />
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
              {[1, 2, 3].map((i) => (
                <div key={i} className="h-96 bg-gray-200 rounded-lg" />
              ))}
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          {onBack && (
            <button
              type="button"
              onClick={onBack}
              className="mb-4 text-blue-600 hover:text-blue-800 flex items-center gap-1"
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M15 19l-7-7 7-7"
                />
              </svg>
              {t('common.back')}
            </button>
          )}
          <div className="text-center">
            <h1 className="text-3xl font-bold text-gray-900">{t('subscription.plans.title')}</h1>
            <p className="mt-2 text-lg text-gray-500">{t('subscription.plans.subtitle')}</p>
          </div>

          {/* Interval Toggle */}
          {hasMonthlyPlans && hasYearlyPlans && (
            <div className="mt-6 flex justify-center">
              <div className="inline-flex rounded-lg border border-gray-200 bg-gray-50 p-1">
                <button
                  type="button"
                  onClick={() => setSelectedInterval('monthly')}
                  className={`px-4 py-2 text-sm font-medium rounded-md transition-colors ${
                    selectedInterval === 'monthly'
                      ? 'bg-white text-gray-900 shadow'
                      : 'text-gray-500 hover:text-gray-700'
                  }`}
                >
                  {t('subscription.plans.monthly')}
                </button>
                <button
                  type="button"
                  onClick={() => setSelectedInterval('yearly')}
                  className={`px-4 py-2 text-sm font-medium rounded-md transition-colors ${
                    selectedInterval === 'yearly'
                      ? 'bg-white text-gray-900 shadow'
                      : 'text-gray-500 hover:text-gray-700'
                  }`}
                >
                  {t('subscription.plans.yearly')}
                  <span className="ml-1.5 text-xs text-green-600">
                    {t('subscription.plans.savePercent', { percent: 20 })}
                  </span>
                </button>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Trial Status Banner */}
      {isInTrial && trialDaysRemaining > 0 && (
        <div className="bg-blue-600 text-white">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-3">
            <div className="flex items-center justify-center gap-2">
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                />
              </svg>
              <span className="text-sm font-medium">
                {t('subscription.trial.banner', { days: trialDaysRemaining })}
              </span>
              <span className="text-sm">-</span>
              <span className="text-sm">{t('subscription.trial.upgradeNow')}</span>
            </div>
          </div>
        </div>
      )}

      {/* Plans Grid */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        {filteredPlans.length === 0 ? (
          <div className="text-center py-12">
            <svg
              className="mx-auto h-16 w-16 text-gray-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={1.5}
                d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
              />
            </svg>
            <h2 className="mt-4 text-xl font-semibold text-gray-900">
              {t('subscription.plans.noPlans')}
            </h2>
            <p className="mt-2 text-gray-500">{t('subscription.plans.noPlansDesc')}</p>
          </div>
        ) : (
          <div
            className={`grid gap-8 ${
              filteredPlans.length === 1
                ? 'grid-cols-1 max-w-md mx-auto'
                : filteredPlans.length === 2
                  ? 'grid-cols-1 md:grid-cols-2 max-w-3xl mx-auto'
                  : 'grid-cols-1 md:grid-cols-2 lg:grid-cols-3'
            }`}
          >
            {filteredPlans.map((plan, index) => {
              const isTrialEligible = trialEligiblePlanIds.includes(plan.id);
              const showTrialButton = isTrialEligible && onStartTrial && !isInTrial;

              return (
                <div key={plan.id} className="relative">
                  {/* Trial Badge */}
                  {isTrialEligible && plan.trialDays && plan.trialDays > 0 && (
                    <div className="absolute -top-3 left-1/2 -translate-x-1/2 z-10">
                      <span className="inline-flex items-center gap-1 px-3 py-1 bg-blue-600 text-white text-xs font-medium rounded-full shadow-sm">
                        <svg
                          className="w-3 h-3"
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
                        {t('subscription.plans.trialAvailable', { days: plan.trialDays })}
                      </span>
                    </div>
                  )}

                  <PlanCard
                    plan={plan}
                    isCurrentPlan={plan.id === currentPlanId}
                    isPopular={index === popularPlanIndex && filteredPlans.length > 2}
                    onSelect={onSelectPlan}
                  />

                  {/* Start Trial Button */}
                  {showTrialButton && (
                    <div className="mt-3">
                      <button
                        type="button"
                        onClick={() => onStartTrial(plan.id)}
                        className="w-full py-2 px-4 text-sm font-medium text-blue-600 bg-blue-50 border border-blue-200 rounded-lg hover:bg-blue-100 transition-colors flex items-center justify-center gap-2"
                      >
                        <svg
                          className="w-4 h-4"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={2}
                            d="M13 10V3L4 14h7v7l9-11h-7z"
                          />
                        </svg>
                        {t('subscription.plans.startFreeTrial')}
                      </button>
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        )}

        {/* FAQ or Additional Info */}
        <div className="mt-16 border-t pt-12">
          <h2 className="text-xl font-semibold text-gray-900 text-center">
            {t('subscription.plans.faqTitle')}
          </h2>
          <div className="mt-8 max-w-3xl mx-auto space-y-6">
            <div className="bg-white rounded-lg p-6 shadow-sm">
              <h3 className="font-medium text-gray-900">
                {t('subscription.plans.faq.changePlan.question')}
              </h3>
              <p className="mt-2 text-gray-500">{t('subscription.plans.faq.changePlan.answer')}</p>
            </div>
            <div className="bg-white rounded-lg p-6 shadow-sm">
              <h3 className="font-medium text-gray-900">
                {t('subscription.plans.faq.cancel.question')}
              </h3>
              <p className="mt-2 text-gray-500">{t('subscription.plans.faq.cancel.answer')}</p>
            </div>
            <div className="bg-white rounded-lg p-6 shadow-sm">
              <h3 className="font-medium text-gray-900">
                {t('subscription.plans.faq.trial.question')}
              </h3>
              <p className="mt-2 text-gray-500">{t('subscription.plans.faq.trial.answer')}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
