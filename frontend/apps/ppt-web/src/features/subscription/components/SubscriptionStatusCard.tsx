/**
 * SubscriptionStatusCard component - displays current subscription status.
 */

import { useTranslation } from 'react-i18next';
import type { Subscription, SubscriptionStatus } from '../types';

interface SubscriptionStatusCardProps {
  subscription: Subscription;
  onManage?: () => void;
  onChangePlan?: () => void;
  onCancel?: () => void;
}

const statusColors: Record<SubscriptionStatus, string> = {
  active: 'bg-green-100 text-green-800',
  trialing: 'bg-blue-100 text-blue-800',
  past_due: 'bg-red-100 text-red-800',
  cancelled: 'bg-gray-100 text-gray-800',
  expired: 'bg-gray-100 text-gray-800',
  suspended: 'bg-orange-100 text-orange-800',
};

const statusIcons: Record<SubscriptionStatus, string> = {
  active: 'M5 13l4 4L19 7',
  trialing: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z',
  past_due: 'M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z',
  cancelled: 'M6 18L18 6M6 6l12 12',
  expired: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z',
  suspended: 'M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z',
};

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  });
}

function getDaysRemaining(endDate: string): number {
  const end = new Date(endDate);
  const now = new Date();
  const diff = end.getTime() - now.getTime();
  return Math.ceil(diff / (1000 * 60 * 60 * 24));
}

export function SubscriptionStatusCard({
  subscription,
  onManage,
  onChangePlan,
  onCancel,
}: SubscriptionStatusCardProps) {
  const { t } = useTranslation();
  const daysRemaining = getDaysRemaining(subscription.currentPeriodEnd);
  const isTrialing = subscription.status === 'trialing' && subscription.trialEnd;
  const trialDaysRemaining = isTrialing ? getDaysRemaining(subscription.trialEnd!) : 0;

  return (
    <div className="bg-white rounded-lg shadow overflow-hidden">
      <div className="p-6">
        <div className="flex items-start justify-between">
          <div>
            <h3 className="text-lg font-semibold text-gray-900">
              {t('subscription.status.title')}
            </h3>
            {subscription.plan && (
              <p className="mt-1 text-2xl font-bold text-gray-900">{subscription.plan.name}</p>
            )}
          </div>
          <span
            className={`inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-sm font-medium ${statusColors[subscription.status]}`}
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d={statusIcons[subscription.status]}
              />
            </svg>
            {t(`subscription.status.${subscription.status}`)}
          </span>
        </div>

        {isTrialing && (
          <div className="mt-4 p-3 bg-blue-50 rounded-lg">
            <div className="flex items-center gap-2">
              <svg
                className="w-5 h-5 text-blue-500"
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
              <span className="text-sm font-medium text-blue-700">
                {t('subscription.status.trialEndsIn', { days: trialDaysRemaining })}
              </span>
            </div>
            <p className="mt-1 text-sm text-blue-600">
              {t('subscription.status.trialEndsOn', { date: formatDate(subscription.trialEnd!) })}
            </p>
          </div>
        )}

        <div className="mt-6 grid grid-cols-2 gap-4">
          <div>
            <p className="text-sm text-gray-500">{t('subscription.status.currentPeriod')}</p>
            <p className="mt-1 text-sm font-medium text-gray-900">
              {formatDate(subscription.currentPeriodStart)} -{' '}
              {formatDate(subscription.currentPeriodEnd)}
            </p>
          </div>
          <div>
            <p className="text-sm text-gray-500">{t('subscription.status.renewsIn')}</p>
            <p className="mt-1 text-sm font-medium text-gray-900">
              {subscription.cancelAtPeriodEnd
                ? t('subscription.status.willNotRenew')
                : t('subscription.status.daysRemaining', { days: daysRemaining })}
            </p>
          </div>
        </div>

        {subscription.cancelAtPeriodEnd && (
          <div className="mt-4 p-3 bg-yellow-50 rounded-lg">
            <div className="flex items-center gap-2">
              <svg
                className="w-5 h-5 text-yellow-500"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                />
              </svg>
              <span className="text-sm font-medium text-yellow-700">
                {t('subscription.status.cancelledNotice')}
              </span>
            </div>
            <p className="mt-1 text-sm text-yellow-600">
              {t('subscription.status.accessUntil', {
                date: formatDate(subscription.currentPeriodEnd),
              })}
            </p>
          </div>
        )}
      </div>

      <div className="px-6 py-4 bg-gray-50 border-t flex items-center gap-3">
        {onManage && (
          <button
            type="button"
            onClick={onManage}
            className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
          >
            {t('subscription.actions.manage')}
          </button>
        )}
        {onChangePlan &&
          subscription.status !== 'cancelled' &&
          subscription.status !== 'expired' && (
            <button
              type="button"
              onClick={onChangePlan}
              className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
            >
              {t('subscription.actions.changePlan')}
            </button>
          )}
        {onCancel && subscription.status === 'active' && !subscription.cancelAtPeriodEnd && (
          <button
            type="button"
            onClick={onCancel}
            className="px-4 py-2 text-sm font-medium text-red-600 hover:text-red-700"
          >
            {t('subscription.actions.cancel')}
          </button>
        )}
      </div>
    </div>
  );
}
