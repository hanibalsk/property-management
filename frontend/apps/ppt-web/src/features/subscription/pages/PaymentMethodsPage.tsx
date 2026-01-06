/**
 * PaymentMethodsPage - Manage payment methods.
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { PaymentMethodCard } from '../components';
import type { PaymentMethod } from '../types';

interface PaymentMethodsPageProps {
  paymentMethods: PaymentMethod[];
  isLoading?: boolean;
  onAddPaymentMethod?: () => void;
  onSetDefault?: (id: string) => void;
  onDelete?: (id: string) => void;
  onEdit?: (id: string) => void;
  onBack?: () => void;
}

export function PaymentMethodsPage({
  paymentMethods,
  isLoading,
  onAddPaymentMethod,
  onSetDefault,
  onDelete,
  onEdit,
  onBack,
}: PaymentMethodsPageProps) {
  const { t } = useTranslation();
  const [deleteConfirmId, setDeleteConfirmId] = useState<string | null>(null);

  const handleDeleteClick = (id: string) => {
    setDeleteConfirmId(id);
  };

  const handleDeleteConfirm = () => {
    if (deleteConfirmId) {
      onDelete?.(deleteConfirmId);
      setDeleteConfirmId(null);
    }
  };

  const handleDeleteCancel = () => {
    setDeleteConfirmId(null);
  };

  const defaultMethod = paymentMethods.find((m) => m.isDefault);
  const otherMethods = paymentMethods.filter((m) => !m.isDefault);

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
          <div className="flex flex-col lg:flex-row lg:items-center lg:justify-between gap-4">
            <div>
              <h1 className="text-2xl font-bold text-gray-900">
                {t('subscription.paymentMethods.title')}
              </h1>
              <p className="mt-1 text-sm text-gray-500">
                {t('subscription.paymentMethods.subtitle')}
              </p>
            </div>
            {onAddPaymentMethod && (
              <button
                type="button"
                onClick={onAddPaymentMethod}
                className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 flex items-center gap-2"
              >
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M12 4v16m8-8H4"
                  />
                </svg>
                {t('subscription.paymentMethods.add')}
              </button>
            )}
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {isLoading ? (
          <div className="space-y-4">
            {[1, 2].map((i) => (
              <div key={i} className="h-24 bg-gray-200 rounded-lg animate-pulse" />
            ))}
          </div>
        ) : paymentMethods.length === 0 ? (
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
              {t('subscription.paymentMethods.noMethods')}
            </h2>
            <p className="mt-2 text-gray-500">{t('subscription.paymentMethods.noMethodsDesc')}</p>
            {onAddPaymentMethod && (
              <button
                type="button"
                onClick={onAddPaymentMethod}
                className="mt-6 px-6 py-3 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
              >
                {t('subscription.paymentMethods.addFirst')}
              </button>
            )}
          </div>
        ) : (
          <div className="space-y-6">
            {/* Default Payment Method */}
            {defaultMethod && (
              <div>
                <h2 className="text-sm font-semibold text-gray-700 uppercase tracking-wide mb-3">
                  {t('subscription.paymentMethods.defaultMethod')}
                </h2>
                <PaymentMethodCard
                  paymentMethod={defaultMethod}
                  onEdit={onEdit}
                  onDelete={handleDeleteClick}
                />
              </div>
            )}

            {/* Other Payment Methods */}
            {otherMethods.length > 0 && (
              <div>
                <h2 className="text-sm font-semibold text-gray-700 uppercase tracking-wide mb-3">
                  {t('subscription.paymentMethods.otherMethods')}
                </h2>
                <div className="space-y-4">
                  {otherMethods.map((method) => (
                    <PaymentMethodCard
                      key={method.id}
                      paymentMethod={method}
                      onSetDefault={onSetDefault}
                      onEdit={onEdit}
                      onDelete={handleDeleteClick}
                    />
                  ))}
                </div>
              </div>
            )}

            {/* Security Notice */}
            <div className="bg-gray-50 rounded-lg p-4 flex items-start gap-3">
              <svg
                className="w-5 h-5 text-gray-400 flex-shrink-0 mt-0.5"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                />
              </svg>
              <div>
                <p className="text-sm font-medium text-gray-700">
                  {t('subscription.paymentMethods.securityTitle')}
                </p>
                <p className="mt-1 text-sm text-gray-500">
                  {t('subscription.paymentMethods.securityDesc')}
                </p>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Delete Confirmation Modal */}
      {deleteConfirmId && (
        <div className="fixed inset-0 z-50 overflow-y-auto">
          <div className="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
            <div
              className="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"
              onClick={handleDeleteCancel}
            />
            <div className="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg">
              <div className="bg-white px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
                <div className="sm:flex sm:items-start">
                  <div className="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-red-100 sm:mx-0 sm:h-10 sm:w-10">
                    <svg
                      className="h-6 w-6 text-red-600"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                      />
                    </svg>
                  </div>
                  <div className="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
                    <h3 className="text-lg font-semibold leading-6 text-gray-900">
                      {t('subscription.paymentMethods.deleteTitle')}
                    </h3>
                    <div className="mt-2">
                      <p className="text-sm text-gray-500">
                        {t('subscription.paymentMethods.deleteConfirm')}
                      </p>
                    </div>
                  </div>
                </div>
              </div>
              <div className="bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6">
                <button
                  type="button"
                  onClick={handleDeleteConfirm}
                  className="inline-flex w-full justify-center rounded-md bg-red-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-red-500 sm:ml-3 sm:w-auto"
                >
                  {t('common.delete')}
                </button>
                <button
                  type="button"
                  onClick={handleDeleteCancel}
                  className="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:mt-0 sm:w-auto"
                >
                  {t('common.cancel')}
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
