/**
 * SubscriptionDashboardPage - Overview with current plan, usage, and recent invoices.
 */

import { useTranslation } from 'react-i18next';
import { InvoiceCard, SubscriptionStatusCard, UsageChart } from '../components';
import type { Invoice, Subscription, TrialStatus, UsageSummary } from '../types';

interface SubscriptionDashboardPageProps {
  subscription?: Subscription;
  trialStatus?: TrialStatus;
  usageSummaries: UsageSummary[];
  recentInvoices: Invoice[];
  isLoading?: boolean;
  onManageSubscription?: () => void;
  onChangePlan?: () => void;
  onCancelSubscription?: () => void;
  onViewInvoice?: (invoiceId: string) => void;
  onDownloadInvoice?: (invoiceId: string) => void;
  onViewAllInvoices?: () => void;
  onViewPaymentMethods?: () => void;
  onUpgrade?: () => void;
  onExtendTrial?: () => void;
}

export function SubscriptionDashboardPage({
  subscription,
  trialStatus,
  usageSummaries,
  recentInvoices,
  isLoading,
  onManageSubscription,
  onChangePlan,
  onCancelSubscription,
  onViewInvoice,
  onDownloadInvoice,
  onViewAllInvoices,
  onViewPaymentMethods,
  onUpgrade,
  onExtendTrial,
}: SubscriptionDashboardPageProps) {
  const { t } = useTranslation();

  const isInTrial = trialStatus?.status === 'active';
  const trialDaysRemaining = trialStatus?.daysRemaining ?? 0;
  const canExtendTrial = trialStatus?.canExtend ?? false;

  if (isLoading) {
    return (
      <div className="min-h-screen bg-gray-100">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
          <div className="animate-pulse space-y-6">
            <div className="h-8 w-64 bg-gray-200 rounded" />
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
              <div className="lg:col-span-2 h-64 bg-gray-200 rounded-lg" />
              <div className="h-64 bg-gray-200 rounded-lg" />
            </div>
            <div className="h-48 bg-gray-200 rounded-lg" />
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
          <div className="flex flex-col lg:flex-row lg:items-center lg:justify-between gap-4">
            <div>
              <h1 className="text-2xl font-bold text-gray-900">
                {t('subscription.dashboard.title')}
              </h1>
              <p className="mt-1 text-sm text-gray-500">{t('subscription.dashboard.subtitle')}</p>
            </div>
            <div className="flex items-center gap-3">
              {onViewPaymentMethods && (
                <button
                  type="button"
                  onClick={onViewPaymentMethods}
                  className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
                >
                  {t('subscription.actions.paymentMethods')}
                </button>
              )}
              {onUpgrade && subscription?.status === 'active' && (
                <button
                  type="button"
                  onClick={onUpgrade}
                  className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
                >
                  {t('subscription.actions.upgrade')}
                </button>
              )}
            </div>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {!subscription ? (
          <div className="bg-white rounded-lg shadow p-8 text-center">
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
                d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"
              />
            </svg>
            <h2 className="mt-4 text-xl font-semibold text-gray-900">
              {t('subscription.dashboard.noSubscription')}
            </h2>
            <p className="mt-2 text-gray-500">{t('subscription.dashboard.noSubscriptionDesc')}</p>
            {onUpgrade && (
              <button
                type="button"
                onClick={onUpgrade}
                className="mt-6 px-6 py-3 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
              >
                {t('subscription.actions.choosePlan')}
              </button>
            )}
          </div>
        ) : (
          <div className="space-y-8">
            {/* Trial Status Banner */}
            {isInTrial && trialDaysRemaining > 0 && (
              <div className="bg-gradient-to-r from-blue-500 to-blue-600 rounded-lg shadow-lg p-6 text-white">
                <div className="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
                  <div className="flex items-start gap-4">
                    <div className="flex-shrink-0 w-12 h-12 bg-white/20 rounded-full flex items-center justify-center">
                      <svg
                        className="w-6 h-6"
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
                      <h3 className="text-lg font-semibold">
                        {t('subscription.trial.statusTitle')}
                      </h3>
                      <p className="text-blue-100 mt-1">
                        {t('subscription.trial.daysRemainingMessage', { days: trialDaysRemaining })}
                      </p>
                      {trialStatus?.trialEndDate && (
                        <p className="text-blue-200 text-sm mt-1">
                          {t('subscription.trial.endsOn', {
                            date: new Date(trialStatus.trialEndDate).toLocaleDateString('en-US', {
                              year: 'numeric',
                              month: 'long',
                              day: 'numeric',
                            }),
                          })}
                        </p>
                      )}
                    </div>
                  </div>
                  <div className="flex flex-col sm:flex-row gap-3">
                    {canExtendTrial && onExtendTrial && (
                      <button
                        type="button"
                        onClick={onExtendTrial}
                        className="px-4 py-2 text-sm font-medium text-blue-600 bg-white rounded-md hover:bg-blue-50 transition-colors"
                      >
                        {t('subscription.trial.extend', { days: trialStatus?.extensionDays ?? 7 })}
                      </button>
                    )}
                    {onUpgrade && (
                      <button
                        type="button"
                        onClick={onUpgrade}
                        className="px-4 py-2 text-sm font-medium text-white bg-white/20 border border-white/30 rounded-md hover:bg-white/30 transition-colors"
                      >
                        {t('subscription.trial.upgradeBeforeExpiry')}
                      </button>
                    )}
                  </div>
                </div>

                {/* Trial Countdown Progress */}
                <div className="mt-4">
                  <div className="flex justify-between text-sm text-blue-100 mb-1">
                    <span>{t('subscription.trial.progressLabel')}</span>
                    <span>
                      {trialDaysRemaining} {t('subscription.trial.daysLeft')}
                    </span>
                  </div>
                  <div className="h-2 bg-white/20 rounded-full overflow-hidden">
                    <div
                      className="h-full bg-white rounded-full transition-all duration-300"
                      style={{
                        width: `${Math.max(0, Math.min(100, (trialDaysRemaining / (trialStatus?.trialDays ?? 14)) * 100))}%`,
                      }}
                    />
                  </div>
                </div>
              </div>
            )}

            {/* Subscription Status */}
            <SubscriptionStatusCard
              subscription={subscription}
              onManage={onManageSubscription}
              onChangePlan={onChangePlan}
              onCancel={onCancelSubscription}
            />

            {/* Usage Section */}
            <UsageChart usageSummaries={usageSummaries} />

            {/* Recent Invoices */}
            <div className="bg-white rounded-lg shadow">
              <div className="p-6 border-b flex items-center justify-between">
                <h3 className="text-lg font-semibold text-gray-900">
                  {t('subscription.dashboard.recentInvoices')}
                </h3>
                {onViewAllInvoices && recentInvoices.length > 0 && (
                  <button
                    type="button"
                    onClick={onViewAllInvoices}
                    className="text-sm text-blue-600 hover:text-blue-800"
                  >
                    {t('subscription.actions.viewAll')}
                  </button>
                )}
              </div>
              <div className="p-6">
                {recentInvoices.length === 0 ? (
                  <p className="text-gray-500 text-center py-4">
                    {t('subscription.invoices.noInvoices')}
                  </p>
                ) : (
                  <div className="space-y-3">
                    {recentInvoices.map((invoice) => (
                      <InvoiceCard
                        key={invoice.id}
                        invoice={invoice}
                        onView={onViewInvoice}
                        onDownload={onDownloadInvoice}
                        compact
                      />
                    ))}
                  </div>
                )}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
