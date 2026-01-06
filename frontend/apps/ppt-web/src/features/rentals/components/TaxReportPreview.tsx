/**
 * TaxReportPreview Component
 *
 * Displays a preview of the generated tax report with export options.
 * Epic 18: Short-Term Rental Integration - UC-29 Tax Export
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { TaxExportFormat, TaxReportData } from '../types';

interface TaxReportPreviewProps {
  report: TaxReportData;
  onExport: (format: TaxExportFormat) => void;
  onClose?: () => void;
  isExporting?: boolean;
}

function formatCurrency(amount: number, currency: string): string {
  return new Intl.NumberFormat(undefined, {
    style: 'currency',
    currency,
  }).format(amount);
}

function formatPercentage(value: number): string {
  return `${(value * 100).toFixed(1)}%`;
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString(undefined, {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  });
}

export function TaxReportPreview({
  report,
  onExport,
  onClose,
  isExporting,
}: TaxReportPreviewProps) {
  const { t } = useTranslation();
  const [selectedFormat, setSelectedFormat] = useState<TaxExportFormat>('pdf');
  const [activeTab, setActiveTab] = useState<'summary' | 'monthly' | 'bookings'>('summary');

  const exportFormats: { value: TaxExportFormat; label: string; icon: string }[] = [
    {
      value: 'pdf',
      label: 'PDF',
      icon: 'M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z',
    },
    {
      value: 'csv',
      label: 'CSV',
      icon: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z',
    },
    {
      value: 'excel',
      label: 'Excel',
      icon: 'M4 5a1 1 0 011-1h14a1 1 0 011 1v14a1 1 0 01-1 1H5a1 1 0 01-1-1V5zm3 3a1 1 0 011-1h8a1 1 0 110 2H8a1 1 0 01-1-1zm0 4a1 1 0 011-1h8a1 1 0 110 2H8a1 1 0 01-1-1zm0 4a1 1 0 011-1h4a1 1 0 110 2H8a1 1 0 01-1-1z',
    },
  ];

  const showMonthlyTab = report.reportType !== 'per_booking' && report.monthlyBreakdown;
  const showBookingsTab = report.reportType === 'per_booking' && report.bookingDetails;

  return (
    <div className="bg-white rounded-lg shadow-lg overflow-hidden">
      {/* Header */}
      <div className="px-6 py-4 bg-gray-50 border-b flex items-center justify-between">
        <div>
          <h2 className="text-xl font-bold text-gray-900">{t('rentals.tax.reportPreviewTitle')}</h2>
          <p className="text-sm text-gray-500 mt-1">
            {t('rentals.tax.generatedOn', { date: formatDate(report.generatedAt) })}
          </p>
        </div>
        {onClose && (
          <button
            type="button"
            onClick={onClose}
            className="p-2 text-gray-400 hover:text-gray-600 rounded-full"
          >
            <svg
              className="w-6 h-6"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              aria-hidden="true"
            >
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

      {/* Report Info */}
      <div className="px-6 py-4 border-b bg-indigo-50">
        <div className="flex flex-wrap gap-6">
          <div>
            <span className="text-xs font-medium text-indigo-600 uppercase tracking-wide">
              {t('rentals.tax.reportYear')}
            </span>
            <p className="text-lg font-semibold text-gray-900">{report.year}</p>
          </div>
          <div>
            <span className="text-xs font-medium text-indigo-600 uppercase tracking-wide">
              {t('rentals.tax.jurisdiction')}
            </span>
            <p className="text-lg font-semibold text-gray-900">{report.jurisdiction.name}</p>
          </div>
          <div>
            <span className="text-xs font-medium text-indigo-600 uppercase tracking-wide">
              {t('rentals.tax.reportType')}
            </span>
            <p className="text-lg font-semibold text-gray-900">
              {t(`rentals.tax.reportTypes.${report.reportType}`)}
            </p>
          </div>
          <div>
            <span className="text-xs font-medium text-indigo-600 uppercase tracking-wide">
              {t('rentals.tax.propertiesCovered')}
            </span>
            <p className="text-lg font-semibold text-gray-900">{report.propertiesCovered.length}</p>
          </div>
        </div>
      </div>

      {/* Tabs */}
      <div className="border-b">
        <nav className="flex -mb-px">
          <button
            type="button"
            onClick={() => setActiveTab('summary')}
            className={`px-6 py-3 text-sm font-medium border-b-2 ${
              activeTab === 'summary'
                ? 'border-indigo-500 text-indigo-600'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            }`}
          >
            {t('rentals.tax.tabs.summary')}
          </button>
          {showMonthlyTab && (
            <button
              type="button"
              onClick={() => setActiveTab('monthly')}
              className={`px-6 py-3 text-sm font-medium border-b-2 ${
                activeTab === 'monthly'
                  ? 'border-indigo-500 text-indigo-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              {t('rentals.tax.tabs.monthly')}
            </button>
          )}
          {showBookingsTab && (
            <button
              type="button"
              onClick={() => setActiveTab('bookings')}
              className={`px-6 py-3 text-sm font-medium border-b-2 ${
                activeTab === 'bookings'
                  ? 'border-indigo-500 text-indigo-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              {t('rentals.tax.tabs.bookings')}
            </button>
          )}
        </nav>
      </div>

      {/* Tab Content */}
      <div className="p-6 max-h-96 overflow-y-auto">
        {activeTab === 'summary' && (
          <div className="space-y-6">
            {/* Summary Cards */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div className="p-4 bg-green-50 rounded-lg">
                <p className="text-xs font-medium text-green-600 uppercase tracking-wide">
                  {t('rentals.tax.totalIncome')}
                </p>
                <p className="mt-2 text-2xl font-bold text-green-700">
                  {formatCurrency(report.summary.totalIncome, report.currency)}
                </p>
              </div>
              <div className="p-4 bg-blue-50 rounded-lg">
                <p className="text-xs font-medium text-blue-600 uppercase tracking-wide">
                  {t('rentals.tax.totalBookings')}
                </p>
                <p className="mt-2 text-2xl font-bold text-blue-700">
                  {report.summary.totalBookings}
                </p>
              </div>
              <div className="p-4 bg-purple-50 rounded-lg">
                <p className="text-xs font-medium text-purple-600 uppercase tracking-wide">
                  {t('rentals.tax.occupancyRate')}
                </p>
                <p className="mt-2 text-2xl font-bold text-purple-700">
                  {formatPercentage(report.summary.averageOccupancyRate)}
                </p>
              </div>
              <div className="p-4 bg-orange-50 rounded-lg">
                <p className="text-xs font-medium text-orange-600 uppercase tracking-wide">
                  {t('rentals.tax.nightsOccupied')}
                </p>
                <p className="mt-2 text-2xl font-bold text-orange-700">
                  {report.summary.totalNightsOccupied}
                </p>
              </div>
            </div>

            {/* Financial Summary */}
            <div className="bg-gray-50 rounded-lg p-4">
              <h3 className="text-sm font-semibold text-gray-900 mb-4">
                {t('rentals.tax.financialSummary')}
              </h3>
              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <span className="text-gray-600">{t('rentals.tax.totalIncome')}</span>
                  <span className="font-medium text-gray-900">
                    {formatCurrency(report.summary.totalIncome, report.currency)}
                  </span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-gray-600">{t('rentals.tax.totalExpenses')}</span>
                  <span className="font-medium text-red-600">
                    -{formatCurrency(report.summary.totalExpenses, report.currency)}
                  </span>
                </div>
                <div className="border-t pt-3 flex items-center justify-between">
                  <span className="font-medium text-gray-900">{t('rentals.tax.netProfit')}</span>
                  <span className="font-bold text-green-600">
                    {formatCurrency(report.summary.netProfit, report.currency)}
                  </span>
                </div>
                <div className="border-t pt-3 flex items-center justify-between">
                  <span className="text-gray-600">
                    {t('rentals.tax.estimatedTax')} (
                    {formatPercentage(report.summary.effectiveTaxRate)})
                  </span>
                  <span className="font-semibold text-indigo-600">
                    {formatCurrency(report.summary.estimatedTax, report.currency)}
                  </span>
                </div>
              </div>
            </div>

            {/* Expense Categories */}
            {report.expensesByCategory.length > 0 && (
              <div>
                <h3 className="text-sm font-semibold text-gray-900 mb-4">
                  {t('rentals.tax.expenseBreakdown')}
                </h3>
                <div className="space-y-2">
                  {report.expensesByCategory.map((expense) => (
                    <div key={expense.category} className="flex items-center gap-3">
                      <div className="flex-1">
                        <div className="flex items-center justify-between text-sm mb-1">
                          <span className="text-gray-600">
                            {t(
                              `rentals.tax.expenseCategories.${expense.category}`,
                              expense.category
                            )}
                          </span>
                          <span className="font-medium text-gray-900">
                            {formatCurrency(expense.amount, report.currency)}
                          </span>
                        </div>
                        <div className="w-full bg-gray-200 rounded-full h-2">
                          <div
                            className="bg-indigo-500 h-2 rounded-full"
                            style={{ width: `${expense.percentage}%` }}
                          />
                        </div>
                      </div>
                      <span className="text-xs text-gray-500 w-12 text-right">
                        {expense.percentage.toFixed(0)}%
                      </span>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        )}

        {activeTab === 'monthly' && report.monthlyBreakdown && (
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead>
                <tr>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    {t('rentals.tax.month')}
                  </th>
                  <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                    {t('rentals.tax.income')}
                  </th>
                  <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                    {t('rentals.tax.bookings')}
                  </th>
                  <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                    {t('rentals.tax.nights')}
                  </th>
                  <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                    {t('rentals.tax.occupancy')}
                  </th>
                  <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                    {t('rentals.tax.expenses')}
                  </th>
                  <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                    {t('rentals.tax.netProfit')}
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {report.monthlyBreakdown.map((month) => (
                  <tr key={month.month} className="hover:bg-gray-50">
                    <td className="px-4 py-3 text-sm font-medium text-gray-900">
                      {month.monthName}
                    </td>
                    <td className="px-4 py-3 text-sm text-gray-900 text-right">
                      {formatCurrency(month.income, report.currency)}
                    </td>
                    <td className="px-4 py-3 text-sm text-gray-600 text-right">
                      {month.bookingsCount}
                    </td>
                    <td className="px-4 py-3 text-sm text-gray-600 text-right">
                      {month.nightsOccupied}
                    </td>
                    <td className="px-4 py-3 text-sm text-gray-600 text-right">
                      {formatPercentage(month.occupancyRate)}
                    </td>
                    <td className="px-4 py-3 text-sm text-red-600 text-right">
                      {formatCurrency(month.expenses, report.currency)}
                    </td>
                    <td className="px-4 py-3 text-sm font-medium text-green-600 text-right">
                      {formatCurrency(month.netProfit, report.currency)}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}

        {activeTab === 'bookings' && report.bookingDetails && (
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead>
                <tr>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    {t('rentals.tax.guest')}
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    {t('rentals.tax.unit')}
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    {t('rentals.tax.dates')}
                  </th>
                  <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                    {t('rentals.tax.nights')}
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    {t('rentals.tax.source')}
                  </th>
                  <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                    {t('rentals.tax.income')}
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {report.bookingDetails.map((booking) => (
                  <tr key={booking.bookingId} className="hover:bg-gray-50">
                    <td className="px-4 py-3 text-sm font-medium text-gray-900">
                      {booking.guestName}
                    </td>
                    <td className="px-4 py-3 text-sm text-gray-600">{booking.unitName}</td>
                    <td className="px-4 py-3 text-sm text-gray-600">
                      {new Date(booking.checkIn).toLocaleDateString()} -{' '}
                      {new Date(booking.checkOut).toLocaleDateString()}
                    </td>
                    <td className="px-4 py-3 text-sm text-gray-600 text-right">{booking.nights}</td>
                    <td className="px-4 py-3 text-sm text-gray-600">
                      {t(`rentals.source.${booking.source}`)}
                    </td>
                    <td className="px-4 py-3 text-sm font-medium text-gray-900 text-right">
                      {formatCurrency(booking.income, report.currency)}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* Export Options */}
      <div className="px-6 py-4 bg-gray-50 border-t">
        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
          <div>
            <label className="text-sm font-medium text-gray-700 mr-3">
              {t('rentals.tax.exportFormat')}
            </label>
            <div className="inline-flex rounded-md shadow-sm mt-2 sm:mt-0">
              {exportFormats.map((format) => (
                <button
                  key={format.value}
                  type="button"
                  onClick={() => setSelectedFormat(format.value)}
                  className={`relative inline-flex items-center px-4 py-2 text-sm font-medium ${
                    selectedFormat === format.value
                      ? 'bg-indigo-600 text-white'
                      : 'bg-white text-gray-700 hover:bg-gray-50'
                  } ${
                    format.value === 'pdf'
                      ? 'rounded-l-md'
                      : format.value === 'excel'
                        ? 'rounded-r-md'
                        : ''
                  } border border-gray-300 -ml-px first:ml-0`}
                >
                  {format.label}
                </button>
              ))}
            </div>
          </div>
          <button
            type="button"
            onClick={() => onExport(selectedFormat)}
            disabled={isExporting}
            className="inline-flex items-center px-6 py-2 text-sm font-medium text-white bg-indigo-600 rounded-md hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isExporting ? (
              <>
                <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2" />
                {t('rentals.tax.exporting')}
              </>
            ) : (
              <>
                <svg
                  className="w-4 h-4 mr-2"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                  />
                </svg>
                {t('rentals.tax.downloadReport')}
              </>
            )}
          </button>
        </div>
      </div>
    </div>
  );
}
