/**
 * RecentPaymentsList component for displaying recent payments.
 */

import type { Payment } from '@ppt/api-client';

interface RecentPaymentsListProps {
  payments: Payment[];
  isLoading?: boolean;
  onViewAll?: () => void;
}

const PAYMENT_METHOD_LABELS: Record<string, string> = {
  bank_transfer: 'Bank Transfer',
  card: 'Card',
  cash: 'Cash',
  check: 'Check',
  online: 'Online',
  direct_debit: 'Direct Debit',
  other: 'Other',
};

const STATUS_STYLES: Record<string, string> = {
  pending: 'bg-yellow-100 text-yellow-800',
  completed: 'bg-green-100 text-green-800',
  failed: 'bg-red-100 text-red-800',
  refunded: 'bg-purple-100 text-purple-800',
  cancelled: 'bg-gray-100 text-gray-800',
};

function formatCurrency(amount: number, currency = 'EUR'): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency,
  }).format(amount);
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
    year: 'numeric',
  });
}

export function RecentPaymentsList({ payments, isLoading, onViewAll }: RecentPaymentsListProps) {
  if (isLoading) {
    return (
      <div className="bg-white rounded-lg shadow p-6">
        <h3 className="text-lg font-medium text-gray-900 mb-4">Recent Payments</h3>
        <div className="animate-pulse space-y-3">
          {[1, 2, 3].map((i) => (
            <div key={i} className="flex items-center space-x-3">
              <div className="h-10 w-10 bg-gray-200 rounded-full" />
              <div className="flex-1 space-y-2">
                <div className="h-4 bg-gray-200 rounded w-3/4" />
                <div className="h-3 bg-gray-200 rounded w-1/2" />
              </div>
            </div>
          ))}
        </div>
      </div>
    );
  }

  if (payments.length === 0) {
    return (
      <div className="bg-white rounded-lg shadow p-6">
        <h3 className="text-lg font-medium text-gray-900 mb-4">Recent Payments</h3>
        <p className="text-gray-500 text-center py-8">No recent payments</p>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow">
      <div className="px-6 py-4 border-b border-gray-200 flex items-center justify-between">
        <h3 className="text-lg font-medium text-gray-900">Recent Payments</h3>
        {onViewAll && (
          <button
            type="button"
            onClick={onViewAll}
            className="text-sm text-blue-600 hover:text-blue-800"
          >
            View all
          </button>
        )}
      </div>

      <ul className="divide-y divide-gray-200">
        {payments.map((payment) => (
          <li key={payment.id} className="px-6 py-4">
            <div className="flex items-center justify-between">
              <div className="flex-1 min-w-0">
                <p className="text-sm font-medium text-gray-900 truncate">
                  {formatCurrency(payment.amount, payment.currency)}
                </p>
                <p className="text-sm text-gray-500">
                  {PAYMENT_METHOD_LABELS[payment.payment_method] || payment.payment_method} •{' '}
                  {formatDate(payment.payment_date)}
                </p>
                {payment.reference && (
                  <p className="text-xs text-gray-400 truncate">Ref: {payment.reference}</p>
                )}
              </div>
              <span
                className={`ml-4 px-2.5 py-0.5 rounded-full text-xs font-medium ${
                  STATUS_STYLES[payment.status] || STATUS_STYLES.pending
                }`}
              >
                {payment.status}
              </span>
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
}
