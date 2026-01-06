/**
 * TrialPage - Free trial signup page.
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { Plan, TrialEligibility } from '../types';

interface TrialPageProps {
  plan: Plan;
  trialEligibility: TrialEligibility;
  isLoading?: boolean;
  isStarting?: boolean;
  onStartTrial?: () => void;
  onBack?: () => void;
}

const trialFeatures = [
  'trial.features.fullAccess',
  'trial.features.allFeatures',
  'trial.features.noCommitment',
  'trial.features.easyUpgrade',
];

export function TrialPage({
  plan,
  trialEligibility,
  isLoading,
  isStarting,
  onStartTrial,
  onBack,
}: TrialPageProps) {
  const { t } = useTranslation();
  const [acceptedTerms, setAcceptedTerms] = useState(false);

  const handleStartTrial = () => {
    if (acceptedTerms && onStartTrial) {
      onStartTrial();
    }
  };

  if (isLoading) {
    return (
      <div className="min-h-screen bg-gray-100">
        <div className="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
          <div className="animate-pulse space-y-6">
            <div className="h-8 w-64 bg-gray-200 rounded" />
            <div className="h-96 bg-gray-200 rounded-lg" />
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
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
            <div className="inline-flex items-center justify-center w-16 h-16 bg-blue-100 rounded-full mb-4">
              <svg
                className="w-8 h-8 text-blue-600"
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
            <h1 className="text-3xl font-bold text-gray-900">{t('subscription.trial.title')}</h1>
            <p className="mt-2 text-lg text-gray-500">
              {t('subscription.trial.subtitle', { plan: plan.name })}
            </p>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Trial Duration Banner */}
        <div className="bg-gradient-to-r from-blue-600 to-blue-700 rounded-lg p-6 text-white text-center mb-8">
          <div className="text-5xl font-bold mb-2">{trialEligibility.trialDays}</div>
          <div className="text-xl">{t('subscription.trial.daysFreeTrial')}</div>
          {!trialEligibility.requiresCreditCard && (
            <div className="mt-3 inline-flex items-center gap-1.5 px-3 py-1 bg-white/20 rounded-full text-sm">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M5 13l4 4L19 7"
                />
              </svg>
              {t('subscription.trial.noCreditCard')}
            </div>
          )}
        </div>

        {/* Plan Info Card */}
        <div className="bg-white rounded-lg shadow p-6 mb-8">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">
            {t('subscription.trial.planIncluded')}
          </h2>
          <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
            <div>
              <p className="font-medium text-gray-900">{plan.name}</p>
              <p className="text-sm text-gray-500">{plan.description}</p>
            </div>
            <div className="text-right">
              <p className="text-lg font-bold text-gray-900">
                {new Intl.NumberFormat('en-US', {
                  style: 'currency',
                  currency: plan.currency,
                  minimumFractionDigits: 0,
                }).format(plan.price)}
                <span className="text-sm font-normal text-gray-500">
                  /
                  {plan.interval === 'yearly'
                    ? t('subscription.plans.yearly').toLowerCase()
                    : t('subscription.plans.monthly').toLowerCase()}
                </span>
              </p>
              <p className="text-sm text-gray-500">{t('subscription.trial.afterTrial')}</p>
            </div>
          </div>
        </div>

        {/* Trial Features */}
        <div className="bg-white rounded-lg shadow p-6 mb-8">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">
            {t('subscription.trial.whatsIncluded')}
          </h2>
          <ul className="space-y-3">
            {trialFeatures.map((featureKey) => (
              <li key={featureKey} className="flex items-start gap-3">
                <svg
                  className="w-5 h-5 text-green-500 flex-shrink-0 mt-0.5"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span className="text-gray-700">{t(`subscription.${featureKey}`)}</span>
              </li>
            ))}
          </ul>
        </div>

        {/* What Happens After Trial */}
        <div className="bg-white rounded-lg shadow p-6 mb-8">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">
            {t('subscription.trial.whatHappensAfter')}
          </h2>
          <div className="space-y-4">
            <div className="flex items-start gap-3">
              <div className="flex-shrink-0 w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center">
                <span className="text-blue-600 font-semibold">1</span>
              </div>
              <div>
                <p className="font-medium text-gray-900">{t('subscription.trial.step1Title')}</p>
                <p className="text-sm text-gray-500">{t('subscription.trial.step1Desc')}</p>
              </div>
            </div>
            <div className="flex items-start gap-3">
              <div className="flex-shrink-0 w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center">
                <span className="text-blue-600 font-semibold">2</span>
              </div>
              <div>
                <p className="font-medium text-gray-900">{t('subscription.trial.step2Title')}</p>
                <p className="text-sm text-gray-500">{t('subscription.trial.step2Desc')}</p>
              </div>
            </div>
            <div className="flex items-start gap-3">
              <div className="flex-shrink-0 w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center">
                <span className="text-blue-600 font-semibold">3</span>
              </div>
              <div>
                <p className="font-medium text-gray-900">{t('subscription.trial.step3Title')}</p>
                <p className="text-sm text-gray-500">{t('subscription.trial.step3Desc')}</p>
              </div>
            </div>
          </div>
        </div>

        {/* Terms and Start Button */}
        <div className="bg-white rounded-lg shadow p-6">
          {!trialEligibility.eligible ? (
            <div className="text-center py-4">
              <svg
                className="mx-auto h-12 w-12 text-gray-400"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={1.5}
                  d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                />
              </svg>
              <h3 className="mt-4 text-lg font-semibold text-gray-900">
                {t('subscription.trial.notEligible')}
              </h3>
              <p className="mt-2 text-gray-500">
                {trialEligibility.reason || t('subscription.trial.notEligibleDefault')}
              </p>
            </div>
          ) : (
            <>
              <label className="flex items-start gap-3 cursor-pointer">
                <input
                  type="checkbox"
                  checked={acceptedTerms}
                  onChange={(e) => setAcceptedTerms(e.target.checked)}
                  className="mt-1 h-4 w-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                />
                <span className="text-sm text-gray-600">
                  {t('subscription.trial.termsAccept')}{' '}
                  <a href="#terms" className="text-blue-600 hover:text-blue-800">
                    {t('subscription.trial.termsLink')}
                  </a>{' '}
                  {t('subscription.trial.termsAnd')}{' '}
                  <a href="#privacy" className="text-blue-600 hover:text-blue-800">
                    {t('subscription.trial.privacyLink')}
                  </a>
                </span>
              </label>

              <button
                type="button"
                onClick={handleStartTrial}
                disabled={!acceptedTerms || isStarting}
                className="mt-6 w-full py-3 px-4 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
              >
                {isStarting ? (
                  <>
                    <svg className="animate-spin h-4 w-4" fill="none" viewBox="0 0 24 24">
                      <circle
                        className="opacity-25"
                        cx="12"
                        cy="12"
                        r="10"
                        stroke="currentColor"
                        strokeWidth="4"
                      />
                      <path
                        className="opacity-75"
                        fill="currentColor"
                        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                      />
                    </svg>
                    {t('subscription.trial.starting')}
                  </>
                ) : (
                  <>
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M13 10V3L4 14h7v7l9-11h-7z"
                      />
                    </svg>
                    {t('subscription.trial.startButton', { days: trialEligibility.trialDays })}
                  </>
                )}
              </button>

              <p className="mt-4 text-center text-xs text-gray-500">
                {t('subscription.trial.cancelAnytime')}
              </p>
            </>
          )}
        </div>
      </div>
    </div>
  );
}
