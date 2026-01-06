/**
 * PaymentMethodCard component - displays payment method details.
 */

import { useTranslation } from 'react-i18next';
import type { PaymentMethod, PaymentMethodType } from '../types';

interface PaymentMethodCardProps {
  paymentMethod: PaymentMethod;
  onSetDefault?: (id: string) => void;
  onDelete?: (id: string) => void;
  onEdit?: (id: string) => void;
}

const cardBrandLogos: Record<string, string> = {
  visa: 'Visa',
  mastercard: 'Mastercard',
  amex: 'American Express',
  discover: 'Discover',
  diners: 'Diners Club',
  jcb: 'JCB',
  unionpay: 'UnionPay',
};

function CardIcon({ brand }: { brand: string }) {
  const brandLower = brand.toLowerCase();

  // Simple colored card icons based on brand
  const brandColors: Record<string, string> = {
    visa: 'text-blue-600',
    mastercard: 'text-orange-500',
    amex: 'text-blue-400',
    discover: 'text-orange-600',
    default: 'text-gray-600',
  };

  const color = brandColors[brandLower] || brandColors.default;

  return (
    <div
      className={`flex items-center justify-center w-12 h-8 rounded border ${color} border-current`}
    >
      <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={2}
          d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"
        />
      </svg>
    </div>
  );
}

function BankIcon() {
  return (
    <div className="flex items-center justify-center w-12 h-8 rounded border text-gray-600 border-current">
      <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={2}
          d="M8 14v3m4-3v3m4-3v3M3 21h18M3 10h18M3 7l9-4 9 4M4 10h16v11H4V10z"
        />
      </svg>
    </div>
  );
}

function getPaymentMethodIcon(type: PaymentMethodType, brand?: string) {
  if (type === 'card' && brand) {
    return <CardIcon brand={brand} />;
  }
  if (type === 'bank_account' || type === 'sepa_debit') {
    return <BankIcon />;
  }
  return <CardIcon brand="default" />;
}

function getPaymentMethodTitle(method: PaymentMethod, t: (key: string) => string): string {
  if (method.type === 'card' && method.card) {
    const brandName = cardBrandLogos[method.card.brand.toLowerCase()] || method.card.brand;
    return `${brandName} ${t('subscription.paymentMethods.endingIn')} ${method.card.last4}`;
  }
  if (method.type === 'bank_account' && method.bankAccount) {
    return `${method.bankAccount.bankName} ${t('subscription.paymentMethods.endingIn')} ${method.bankAccount.last4}`;
  }
  if (method.type === 'sepa_debit') {
    return t('subscription.paymentMethods.sepaDebit');
  }
  return t('subscription.paymentMethods.unknown');
}

function getPaymentMethodSubtitle(method: PaymentMethod, t: (key: string) => string): string {
  if (method.type === 'card' && method.card) {
    return `${t('subscription.paymentMethods.expires')} ${method.card.expMonth.toString().padStart(2, '0')}/${method.card.expYear}`;
  }
  if (method.type === 'bank_account' && method.bankAccount) {
    return t(`subscription.paymentMethods.accountType.${method.bankAccount.accountType}`);
  }
  return '';
}

export function PaymentMethodCard({
  paymentMethod,
  onSetDefault,
  onDelete,
  onEdit,
}: PaymentMethodCardProps) {
  const { t } = useTranslation();
  const title = getPaymentMethodTitle(paymentMethod, t);
  const subtitle = getPaymentMethodSubtitle(paymentMethod, t);

  const isExpiringSoon =
    paymentMethod.type === 'card' &&
    paymentMethod.card &&
    (() => {
      const now = new Date();
      const expDate = new Date(paymentMethod.card.expYear, paymentMethod.card.expMonth - 1);
      const threeMonthsFromNow = new Date(now.getFullYear(), now.getMonth() + 3);
      return expDate <= threeMonthsFromNow;
    })();

  return (
    <div
      className={`bg-white rounded-lg border p-4 ${
        paymentMethod.isDefault ? 'border-blue-300 ring-1 ring-blue-100' : 'border-gray-200'
      }`}
    >
      <div className="flex items-start justify-between">
        <div className="flex items-start gap-4">
          {getPaymentMethodIcon(paymentMethod.type, paymentMethod.card?.brand)}
          <div>
            <div className="flex items-center gap-2">
              <p className="font-medium text-gray-900">{title}</p>
              {paymentMethod.isDefault && (
                <span className="px-2 py-0.5 text-xs font-medium bg-blue-100 text-blue-800 rounded">
                  {t('subscription.paymentMethods.default')}
                </span>
              )}
            </div>
            {subtitle && <p className="text-sm text-gray-500 mt-0.5">{subtitle}</p>}
            {paymentMethod.billingName && (
              <p className="text-sm text-gray-500 mt-1">{paymentMethod.billingName}</p>
            )}
          </div>
        </div>

        <div className="flex items-center gap-2">
          {onEdit && (
            <button
              type="button"
              onClick={() => onEdit(paymentMethod.id)}
              className="p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100"
              title={t('common.edit')}
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"
                />
              </svg>
            </button>
          )}
          {onDelete && !paymentMethod.isDefault && (
            <button
              type="button"
              onClick={() => onDelete(paymentMethod.id)}
              className="p-2 text-gray-400 hover:text-red-600 rounded-full hover:bg-red-50"
              title={t('common.delete')}
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                />
              </svg>
            </button>
          )}
        </div>
      </div>

      {isExpiringSoon && (
        <div className="mt-3 p-2 bg-yellow-50 rounded flex items-center gap-2">
          <svg
            className="w-4 h-4 text-yellow-500"
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
          <span className="text-sm text-yellow-700">
            {t('subscription.paymentMethods.expiringSoon')}
          </span>
        </div>
      )}

      {!paymentMethod.isDefault && onSetDefault && (
        <div className="mt-3 pt-3 border-t">
          <button
            type="button"
            onClick={() => onSetDefault(paymentMethod.id)}
            className="text-sm text-blue-600 hover:text-blue-800"
          >
            {t('subscription.paymentMethods.setAsDefault')}
          </button>
        </div>
      )}
    </div>
  );
}
