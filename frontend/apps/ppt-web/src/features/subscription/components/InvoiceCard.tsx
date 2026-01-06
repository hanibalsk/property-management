/**
 * InvoiceCard component - displays invoice details.
 */

import { useTranslation } from 'react-i18next';
import type { Invoice, InvoiceStatus } from '../types';

interface InvoiceCardProps {
  invoice: Invoice;
  onView?: (invoiceId: string) => void;
  onDownload?: (invoiceId: string) => void;
  onPay?: (invoiceId: string) => void;
  compact?: boolean;
}

const statusColors: Record<InvoiceStatus, string> = {
  draft: 'bg-gray-100 text-gray-800',
  open: 'bg-blue-100 text-blue-800',
  paid: 'bg-green-100 text-green-800',
  void: 'bg-gray-100 text-gray-800',
  uncollectible: 'bg-red-100 text-red-800',
};

function formatCurrency(amount: number, currency: string): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: currency,
  }).format(amount);
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  });
}

export function InvoiceCard({
  invoice,
  onView,
  onDownload,
  onPay,
  compact = false,
}: InvoiceCardProps) {
  const { t } = useTranslation();
  const isPayable = invoice.status === 'open' && invoice.amountDue > 0;
  const isOverdue = invoice.status === 'open' && new Date(invoice.dueDate) < new Date();

  if (compact) {
    return (
      <div className="flex items-center justify-between p-4 bg-white rounded-lg border border-gray-200 hover:shadow-sm transition-shadow">
        <div className="flex items-center gap-4">
          <div className="flex-shrink-0">
            <svg
              className="w-8 h-8 text-gray-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
              />
            </svg>
          </div>
          <div>
            <p className="font-medium text-gray-900">{invoice.invoiceNumber}</p>
            <p className="text-sm text-gray-500">{formatDate(invoice.createdAt)}</p>
          </div>
        </div>
        <div className="flex items-center gap-4">
          <div className="text-right">
            <p className="font-semibold text-gray-900">
              {formatCurrency(invoice.total, invoice.currency)}
            </p>
            <span
              className={`inline-block px-2 py-0.5 text-xs rounded ${statusColors[invoice.status]}`}
            >
              {t(`subscription.invoices.status.${invoice.status}`)}
            </span>
          </div>
          <div className="flex items-center gap-2">
            {onView && (
              <button
                type="button"
                onClick={() => onView(invoice.id)}
                className="p-2 text-gray-400 hover:text-gray-600"
                title={t('common.view')}
              >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                  />
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                  />
                </svg>
              </button>
            )}
            {onDownload && invoice.pdfUrl && (
              <button
                type="button"
                onClick={() => onDownload(invoice.id)}
                className="p-2 text-gray-400 hover:text-gray-600"
                title={t('subscription.invoices.download')}
              >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                  />
                </svg>
              </button>
            )}
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow overflow-hidden">
      <div className="p-6">
        <div className="flex items-start justify-between">
          <div>
            <h3 className="text-lg font-semibold text-gray-900">{invoice.invoiceNumber}</h3>
            <p className="text-sm text-gray-500">
              {t('subscription.invoices.period', {
                start: formatDate(invoice.periodStart),
                end: formatDate(invoice.periodEnd),
              })}
            </p>
          </div>
          <span className={`px-3 py-1 text-sm rounded-full ${statusColors[invoice.status]}`}>
            {t(`subscription.invoices.status.${invoice.status}`)}
          </span>
        </div>

        {isOverdue && (
          <div className="mt-4 p-3 bg-red-50 rounded-lg flex items-center gap-2">
            <svg
              className="w-5 h-5 text-red-500"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            <span className="text-sm font-medium text-red-700">
              {t('subscription.invoices.overdue')}
            </span>
          </div>
        )}

        <div className="mt-6 border-t pt-4">
          <div className="space-y-2">
            {invoice.lineItems.map((item) => (
              <div key={item.id} className="flex justify-between text-sm">
                <span className="text-gray-600">
                  {item.description}
                  {item.quantity > 1 && (
                    <span className="text-gray-400 ml-1">x{item.quantity}</span>
                  )}
                </span>
                <span className="text-gray-900">
                  {formatCurrency(item.amount, invoice.currency)}
                </span>
              </div>
            ))}
          </div>

          <div className="mt-4 pt-4 border-t space-y-2">
            <div className="flex justify-between text-sm">
              <span className="text-gray-500">{t('subscription.invoices.subtotal')}</span>
              <span className="text-gray-900">
                {formatCurrency(invoice.subtotal, invoice.currency)}
              </span>
            </div>
            {invoice.tax > 0 && (
              <div className="flex justify-between text-sm">
                <span className="text-gray-500">{t('subscription.invoices.tax')}</span>
                <span className="text-gray-900">
                  {formatCurrency(invoice.tax, invoice.currency)}
                </span>
              </div>
            )}
            <div className="flex justify-between text-base font-semibold">
              <span className="text-gray-900">{t('subscription.invoices.total')}</span>
              <span className="text-gray-900">
                {formatCurrency(invoice.total, invoice.currency)}
              </span>
            </div>
            {invoice.amountPaid > 0 && invoice.status !== 'paid' && (
              <>
                <div className="flex justify-between text-sm">
                  <span className="text-gray-500">{t('subscription.invoices.amountPaid')}</span>
                  <span className="text-green-600">
                    -{formatCurrency(invoice.amountPaid, invoice.currency)}
                  </span>
                </div>
                <div className="flex justify-between text-base font-semibold">
                  <span className="text-gray-900">{t('subscription.invoices.amountDue')}</span>
                  <span className="text-gray-900">
                    {formatCurrency(invoice.amountDue, invoice.currency)}
                  </span>
                </div>
              </>
            )}
          </div>
        </div>

        <div className="mt-4 text-sm text-gray-500">
          <span>{t('subscription.invoices.dueDate')}: </span>
          <span className={isOverdue ? 'text-red-600 font-medium' : ''}>
            {formatDate(invoice.dueDate)}
          </span>
        </div>
      </div>

      <div className="px-6 py-4 bg-gray-50 border-t flex items-center gap-3">
        {onView && (
          <button
            type="button"
            onClick={() => onView(invoice.id)}
            className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
          >
            {t('subscription.invoices.viewDetails')}
          </button>
        )}
        {onDownload && invoice.pdfUrl && (
          <button
            type="button"
            onClick={() => onDownload(invoice.id)}
            className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
          >
            {t('subscription.invoices.download')}
          </button>
        )}
        {isPayable && onPay && (
          <button
            type="button"
            onClick={() => onPay(invoice.id)}
            className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
          >
            {t('subscription.invoices.payNow')}
          </button>
        )}
      </div>
    </div>
  );
}
