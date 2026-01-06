/**
 * ChangePlanPage - Upgrade or downgrade subscription plan.
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { DiscountCodeInput, PlanCard } from '../components';
import type { AppliedDiscount, Plan, Subscription } from '../types';

interface ChangePlanPageProps {
  currentSubscription: Subscription;
  availablePlans: Plan[];
  isLoading?: boolean;
  isProcessing?: boolean;
  onChangePlan?: (planId: string, discountCode?: string) => void;
  onApplyDiscount?: (code: string, planId: string) => Promise<AppliedDiscount | null>;
  onBack?: () => void;
}

function formatPrice(price: number, currency: string): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: currency,
    minimumFractionDigits: 0,
    maximumFractionDigits: 2,
  }).format(price);
}

export function ChangePlanPage({
  currentSubscription,
  availablePlans,
  isLoading,
  isProcessing,
  onChangePlan,
  onApplyDiscount,
  onBack,
}: ChangePlanPageProps) {
  const { t } = useTranslation();
  const [selectedPlanId, setSelectedPlanId] = useState<string | null>(null);
  const [appliedDiscount, setAppliedDiscount] = useState<AppliedDiscount | null>(null);

  const currentPlan = currentSubscription.plan;
  const selectedPlan = availablePlans.find((p) => p.id === selectedPlanId);

  const isUpgrade = selectedPlan && currentPlan && selectedPlan.price > currentPlan.price;
  const isDowngrade = selectedPlan && currentPlan && selectedPlan.price < currentPlan.price;

  const handleSelectPlan = (planId: string) => {
    if (planId !== currentSubscription.planId) {
      setSelectedPlanId(planId);
      // Clear discount when changing plans
      setAppliedDiscount(null);
    }
  };

  const handleApplyDiscount = async (code: string): Promise<AppliedDiscount | null> => {
    if (!selectedPlanId || !onApplyDiscount) return null;

    const result = await onApplyDiscount(code, selectedPlanId);
    if (result) {
      setAppliedDiscount(result);
    }
    return result;
  };

  const handleRemoveDiscount = () => {
    setAppliedDiscount(null);
  };

  const handleConfirmChange = () => {
    if (selectedPlanId && onChangePlan) {
      onChangePlan(selectedPlanId, appliedDiscount?.code);
    }
  };

  // Filter plans to same interval as current subscription
  const sameIntervalPlans = availablePlans.filter(
    (p) => p.status === 'active' && p.interval === currentPlan?.interval
  );

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
          <div>
            <h1 className="text-2xl font-bold text-gray-900">
              {t('subscription.changePlan.title')}
            </h1>
            <p className="mt-1 text-sm text-gray-500">{t('subscription.changePlan.subtitle')}</p>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Current Plan Info */}
        {currentPlan && (
          <div className="mb-8 p-4 bg-blue-50 rounded-lg">
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
                  d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                />
              </svg>
              <span className="text-sm text-blue-700">
                {t('subscription.changePlan.currentPlanInfo', {
                  plan: currentPlan.name,
                  price: formatPrice(currentPlan.price, currentPlan.currency),
                })}
              </span>
            </div>
          </div>
        )}

        {/* Plans Grid */}
        <div
          className={`grid gap-8 mb-8 ${
            sameIntervalPlans.length === 1
              ? 'grid-cols-1 max-w-md mx-auto'
              : sameIntervalPlans.length === 2
                ? 'grid-cols-1 md:grid-cols-2 max-w-3xl mx-auto'
                : 'grid-cols-1 md:grid-cols-2 lg:grid-cols-3'
          }`}
        >
          {sameIntervalPlans.map((plan) => (
            <div
              key={plan.id}
              className={`cursor-pointer transition-transform ${
                selectedPlanId === plan.id ? 'scale-105' : ''
              }`}
              onClick={() => handleSelectPlan(plan.id)}
            >
              <PlanCard
                plan={plan}
                isCurrentPlan={plan.id === currentSubscription.planId}
                onSelect={handleSelectPlan}
                disabled={plan.id === currentSubscription.planId}
              />
            </div>
          ))}
        </div>

        {/* Selected Plan Summary */}
        {selectedPlan && (
          <div className="max-w-2xl mx-auto">
            <div className="bg-white rounded-lg shadow p-6">
              <h3 className="text-lg font-semibold text-gray-900 mb-4">
                {t('subscription.changePlan.summary')}
              </h3>

              <div className="space-y-4">
                {/* Change Type Badge */}
                <div className="flex items-center gap-2">
                  {isUpgrade && (
                    <span className="px-3 py-1 text-sm font-medium bg-green-100 text-green-800 rounded-full">
                      {t('subscription.changePlan.upgrade')}
                    </span>
                  )}
                  {isDowngrade && (
                    <span className="px-3 py-1 text-sm font-medium bg-yellow-100 text-yellow-800 rounded-full">
                      {t('subscription.changePlan.downgrade')}
                    </span>
                  )}
                </div>

                {/* Plan Details */}
                <div className="flex justify-between items-center py-2 border-b">
                  <span className="text-gray-600">{t('subscription.changePlan.newPlan')}</span>
                  <span className="font-medium text-gray-900">{selectedPlan.name}</span>
                </div>
                {/* Price Display */}
                <div className="flex justify-between items-center py-2 border-b">
                  <span className="text-gray-600">{t('subscription.changePlan.newPrice')}</span>
                  <div className="text-right">
                    {appliedDiscount ? (
                      <div className="flex items-center gap-2">
                        <span className="text-gray-400 line-through text-sm">
                          {formatPrice(selectedPlan.price, selectedPlan.currency)}
                        </span>
                        <span className="font-medium text-green-600">
                          {formatPrice(appliedDiscount.discountedPrice, appliedDiscount.currency)}/
                          {selectedPlan.interval}
                        </span>
                        <span className="px-1.5 py-0.5 bg-green-100 text-green-800 rounded text-xs font-medium">
                          {appliedDiscount.discountType === 'percentage'
                            ? `-${appliedDiscount.discountValue}%`
                            : `-${formatPrice(appliedDiscount.savings, appliedDiscount.currency)}`}
                        </span>
                      </div>
                    ) : (
                      <span className="font-medium text-gray-900">
                        {formatPrice(selectedPlan.price, selectedPlan.currency)}/
                        {selectedPlan.interval}
                      </span>
                    )}
                  </div>
                </div>

                {/* Discount Code Input */}
                {onApplyDiscount && (
                  <div className="pt-4">
                    <DiscountCodeInput
                      onApply={handleApplyDiscount}
                      appliedDiscount={appliedDiscount}
                      onRemove={handleRemoveDiscount}
                      disabled={isProcessing}
                    />
                  </div>
                )}

                {/* Proration Notice */}
                <div className="pt-4 p-3 bg-gray-50 rounded-lg">
                  <p className="text-sm text-gray-600">
                    {isUpgrade
                      ? t('subscription.changePlan.upgradeNotice')
                      : t('subscription.changePlan.downgradeNotice')}
                  </p>
                </div>

                {/* Confirm Button */}
                <div className="pt-4">
                  <button
                    type="button"
                    onClick={handleConfirmChange}
                    disabled={isProcessing}
                    className="w-full py-3 px-4 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    {isProcessing
                      ? t('common.loading')
                      : isUpgrade
                        ? t('subscription.changePlan.confirmUpgrade')
                        : t('subscription.changePlan.confirmDowngrade')}
                  </button>
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
