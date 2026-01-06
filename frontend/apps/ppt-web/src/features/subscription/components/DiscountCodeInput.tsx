/**
 * DiscountCodeInput component - Input field with validation for discount codes.
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { AppliedDiscount } from '../types';

interface DiscountCodeInputProps {
  onApply: (code: string) => Promise<AppliedDiscount | null>;
  appliedDiscount?: AppliedDiscount | null;
  onRemove?: () => void;
  disabled?: boolean;
  className?: string;
}

function formatPrice(price: number, currency: string): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: currency,
    minimumFractionDigits: 0,
    maximumFractionDigits: 2,
  }).format(price);
}

export function DiscountCodeInput({
  onApply,
  appliedDiscount,
  onRemove,
  disabled = false,
  className = '',
}: DiscountCodeInputProps) {
  const { t } = useTranslation();
  const [code, setCode] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleApply = async () => {
    if (!code.trim()) return;

    setIsLoading(true);
    setError(null);

    try {
      const result = await onApply(code.trim().toUpperCase());
      if (!result) {
        setError(t('subscription.discount.invalidCode'));
      } else {
        setCode('');
      }
    } catch {
      setError(t('subscription.discount.applyError'));
    } finally {
      setIsLoading(false);
    }
  };

  const handleRemove = () => {
    setCode('');
    setError(null);
    onRemove?.();
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      handleApply();
    }
  };

  // Show applied discount summary
  if (appliedDiscount) {
    return (
      <div className={`bg-green-50 border border-green-200 rounded-lg p-4 ${className}`}>
        <div className="flex items-start justify-between">
          <div className="flex items-center gap-2">
            <svg
              className="w-5 h-5 text-green-500 flex-shrink-0"
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
            <div>
              <p className="text-sm font-medium text-green-800">
                {t('subscription.discount.applied')}
              </p>
              <p className="text-sm text-green-700">
                {t('subscription.discount.code')}:{' '}
                <span className="font-mono">{appliedDiscount.code}</span>
              </p>
            </div>
          </div>
          {onRemove && (
            <button
              type="button"
              onClick={handleRemove}
              disabled={disabled}
              className="text-green-600 hover:text-green-800 text-sm font-medium disabled:opacity-50"
            >
              {t('subscription.discount.remove')}
            </button>
          )}
        </div>

        {/* Discount Summary */}
        <div className="mt-3 pt-3 border-t border-green-200 space-y-1">
          <div className="flex justify-between text-sm">
            <span className="text-green-700">{t('subscription.discount.originalPrice')}</span>
            <span className="text-green-700 line-through">
              {formatPrice(appliedDiscount.originalPrice, appliedDiscount.currency)}
            </span>
          </div>
          <div className="flex justify-between text-sm">
            <span className="text-green-700">
              {t('subscription.discount.discountLabel')}
              {appliedDiscount.discountType === 'percentage' && (
                <span className="ml-1 px-1.5 py-0.5 bg-green-100 text-green-800 rounded text-xs font-medium">
                  -{appliedDiscount.discountValue}%
                </span>
              )}
            </span>
            <span className="text-green-700 font-medium">
              -{formatPrice(appliedDiscount.savings, appliedDiscount.currency)}
            </span>
          </div>
          <div className="flex justify-between text-sm font-semibold">
            <span className="text-green-800">{t('subscription.discount.newPrice')}</span>
            <span className="text-green-800">
              {formatPrice(appliedDiscount.discountedPrice, appliedDiscount.currency)}
            </span>
          </div>
        </div>
      </div>
    );
  }

  // Show input form
  return (
    <div className={className}>
      <label className="block text-sm font-medium text-gray-700 mb-2">
        {t('subscription.discount.title')}
      </label>
      <div className="flex gap-2">
        <div className="flex-1 relative">
          <input
            type="text"
            value={code}
            onChange={(e) => {
              setCode(e.target.value.toUpperCase());
              setError(null);
            }}
            onKeyDown={handleKeyDown}
            placeholder={t('subscription.discount.placeholder')}
            disabled={disabled || isLoading}
            className={`w-full px-3 py-2 border rounded-md text-sm font-mono uppercase focus:ring-blue-500 focus:border-blue-500 ${
              error ? 'border-red-300 focus:ring-red-500 focus:border-red-500' : 'border-gray-300'
            } disabled:bg-gray-100 disabled:cursor-not-allowed`}
          />
          {code && !isLoading && (
            <button
              type="button"
              onClick={() => {
                setCode('');
                setError(null);
              }}
              className="absolute right-2 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600"
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg>
            </button>
          )}
        </div>
        <button
          type="button"
          onClick={handleApply}
          disabled={disabled || isLoading || !code.trim()}
          className="px-4 py-2 text-sm font-medium text-blue-600 border border-blue-600 rounded-md hover:bg-blue-50 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
        >
          {isLoading ? (
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
              {t('common.loading')}
            </>
          ) : (
            t('subscription.discount.apply')
          )}
        </button>
      </div>
      {error && (
        <div className="mt-2 flex items-center gap-1.5 text-sm text-red-600">
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          {error}
        </div>
      )}
      <p className="mt-1.5 text-xs text-gray-500">{t('subscription.discount.hint')}</p>
    </div>
  );
}
