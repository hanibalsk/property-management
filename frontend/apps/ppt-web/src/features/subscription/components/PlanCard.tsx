/**
 * PlanCard component - displays a subscription plan with features and pricing.
 */

import { useTranslation } from 'react-i18next';
import type { Plan, PlanFeature } from '../types';

interface PlanCardProps {
  plan: Plan;
  isCurrentPlan?: boolean;
  isPopular?: boolean;
  onSelect?: (planId: string) => void;
  disabled?: boolean;
}

function formatPrice(price: number, currency: string, interval: string): string {
  const formatter = new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: currency,
    minimumFractionDigits: 0,
    maximumFractionDigits: 2,
  });
  return `${formatter.format(price)}/${interval === 'yearly' ? 'year' : 'month'}`;
}

function FeatureItem({ feature }: { feature: PlanFeature }) {
  const { t } = useTranslation();

  return (
    <li className="flex items-start gap-2">
      {feature.included ? (
        <svg
          className="w-5 h-5 text-green-500 flex-shrink-0 mt-0.5"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
          aria-hidden="true"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
        </svg>
      ) : (
        <svg
          className="w-5 h-5 text-gray-300 flex-shrink-0 mt-0.5"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
          aria-hidden="true"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M6 18L18 6M6 6l12 12"
          />
        </svg>
      )}
      <span className={feature.included ? 'text-gray-700' : 'text-gray-400'}>
        {feature.name}
        {feature.limit !== undefined && feature.included && (
          <span className="text-gray-500 ml-1">
            ({t('subscription.plans.upTo', { count: feature.limit })})
          </span>
        )}
      </span>
    </li>
  );
}

export function PlanCard({
  plan,
  isCurrentPlan = false,
  isPopular = false,
  onSelect,
  disabled = false,
}: PlanCardProps) {
  const { t } = useTranslation();

  return (
    <div
      className={`relative bg-white rounded-lg shadow-lg overflow-hidden ${
        isPopular ? 'ring-2 ring-blue-500' : 'border border-gray-200'
      } ${isCurrentPlan ? 'ring-2 ring-green-500' : ''}`}
    >
      {isPopular && (
        <div className="absolute top-0 left-0 right-0 bg-blue-500 text-white text-center text-sm font-medium py-1">
          {t('subscription.plans.mostPopular')}
        </div>
      )}
      {isCurrentPlan && (
        <div className="absolute top-0 left-0 right-0 bg-green-500 text-white text-center text-sm font-medium py-1">
          {t('subscription.plans.currentPlan')}
        </div>
      )}

      <div className={`p-6 ${isPopular || isCurrentPlan ? 'pt-10' : ''}`}>
        <h3 className="text-xl font-bold text-gray-900">{plan.name}</h3>
        <p className="mt-2 text-sm text-gray-500">{plan.description}</p>

        <div className="mt-4">
          <span className="text-4xl font-bold text-gray-900">
            {formatPrice(plan.price, plan.currency, plan.interval)}
          </span>
        </div>

        {plan.trialDays && plan.trialDays > 0 && (
          <p className="mt-2 text-sm text-blue-600">
            {t('subscription.plans.trialDays', { count: plan.trialDays })}
          </p>
        )}

        <ul className="mt-6 space-y-3">
          {plan.maxUsers !== undefined && (
            <li className="flex items-center gap-2 text-gray-700">
              <svg
                className="w-5 h-5 text-blue-500"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                aria-hidden="true"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"
                />
              </svg>
              {t('subscription.plans.maxUsers', { count: plan.maxUsers })}
            </li>
          )}
          {plan.maxBuildings !== undefined && (
            <li className="flex items-center gap-2 text-gray-700">
              <svg
                className="w-5 h-5 text-blue-500"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                aria-hidden="true"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"
                />
              </svg>
              {t('subscription.plans.maxBuildings', { count: plan.maxBuildings })}
            </li>
          )}
          {plan.maxUnits !== undefined && (
            <li className="flex items-center gap-2 text-gray-700">
              <svg
                className="w-5 h-5 text-blue-500"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                aria-hidden="true"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
                />
              </svg>
              {t('subscription.plans.maxUnits', { count: plan.maxUnits })}
            </li>
          )}
        </ul>

        <div className="mt-6 border-t pt-6">
          <h4 className="text-sm font-semibold text-gray-900 mb-3">
            {t('subscription.plans.features')}
          </h4>
          <ul className="space-y-2">
            {plan.features.map((feature) => (
              <FeatureItem key={feature.key} feature={feature} />
            ))}
          </ul>
        </div>

        <button
          type="button"
          onClick={() => onSelect?.(plan.id)}
          disabled={disabled || isCurrentPlan}
          className={`mt-6 w-full py-3 px-4 rounded-lg font-medium transition-colors ${
            isCurrentPlan
              ? 'bg-gray-100 text-gray-500 cursor-not-allowed'
              : isPopular
                ? 'bg-blue-600 text-white hover:bg-blue-700'
                : 'bg-gray-900 text-white hover:bg-gray-800'
          } ${disabled ? 'opacity-50 cursor-not-allowed' : ''}`}
        >
          {isCurrentPlan ? t('subscription.plans.currentPlan') : t('subscription.plans.selectPlan')}
        </button>
      </div>
    </div>
  );
}
