/**
 * ExchangeRatesPage - Story 145.2
 *
 * Page for viewing and managing exchange rates.
 */

import { useState } from 'react';
import { CurrencySelector } from '../components/CurrencySelector';
import { ExchangeRateTable } from '../components/ExchangeRateTable';

interface ExchangeRate {
  id: string;
  fromCurrency: string;
  toCurrency: string;
  rate: number;
  inverseRate: number;
  rateDate: string;
  source: string;
  isOverride: boolean;
  overrideReason?: string;
}

interface ExchangeRatesPageProps {
  rates: ExchangeRate[];
  isLoading?: boolean;
  onFetchRates?: () => void;
  onOverrideRate?: (rate: ExchangeRate, newRate: number, reason: string) => void;
  onFilter?: (
    fromCurrency?: string,
    toCurrency?: string,
    dateFrom?: string,
    dateTo?: string
  ) => void;
}

export function ExchangeRatesPage({
  rates,
  isLoading,
  onFetchRates,
  onOverrideRate,
  onFilter,
}: ExchangeRatesPageProps) {
  const [fromCurrency, setFromCurrency] = useState<string>('');
  const [toCurrency, setToCurrency] = useState<string>('');
  const [dateFrom, setDateFrom] = useState<string>('');
  const [dateTo, setDateTo] = useState<string>('');
  const [showOverrideModal, setShowOverrideModal] = useState(false);
  const [selectedRate, setSelectedRate] = useState<ExchangeRate | null>(null);
  const [overrideValue, setOverrideValue] = useState<string>('');
  const [overrideReason, setOverrideReason] = useState<string>('');

  const handleFilter = () => {
    onFilter?.(
      fromCurrency || undefined,
      toCurrency || undefined,
      dateFrom || undefined,
      dateTo || undefined
    );
  };

  const handleClearFilters = () => {
    setFromCurrency('');
    setToCurrency('');
    setDateFrom('');
    setDateTo('');
    onFilter?.();
  };

  const handleOverride = (rate: ExchangeRate) => {
    setSelectedRate(rate);
    setOverrideValue(rate.rate.toString());
    setOverrideReason('');
    setShowOverrideModal(true);
  };

  const handleSubmitOverride = () => {
    if (selectedRate && overrideValue && overrideReason) {
      onOverrideRate?.(selectedRate, Number.parseFloat(overrideValue), overrideReason);
      setShowOverrideModal(false);
      setSelectedRate(null);
      setOverrideValue('');
      setOverrideReason('');
    }
  };

  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-2xl font-bold text-gray-900">Exchange Rates</h1>
              <p className="text-sm text-gray-500">Manage and view historical exchange rates</p>
            </div>
            <button
              type="button"
              onClick={onFetchRates}
              disabled={isLoading}
              className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50"
            >
              {isLoading ? 'Fetching...' : 'Fetch Latest Rates'}
            </button>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6 space-y-6">
        {/* Filters */}
        <div className="bg-white rounded-lg shadow p-4">
          <div className="grid grid-cols-1 md:grid-cols-5 gap-4">
            <CurrencySelector
              label="From Currency"
              value={fromCurrency}
              onChange={setFromCurrency}
            />
            <CurrencySelector label="To Currency" value={toCurrency} onChange={setToCurrency} />
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Date From</label>
              <input
                type="date"
                value={dateFrom}
                onChange={(e) => setDateFrom(e.target.value)}
                className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Date To</label>
              <input
                type="date"
                value={dateTo}
                onChange={(e) => setDateTo(e.target.value)}
                className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
              />
            </div>
            <div className="flex items-end space-x-2">
              <button
                type="button"
                onClick={handleFilter}
                className="flex-1 px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
              >
                Filter
              </button>
              <button
                type="button"
                onClick={handleClearFilters}
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200"
              >
                Clear
              </button>
            </div>
          </div>
        </div>

        {/* Exchange Rates Table */}
        <div className="bg-white rounded-lg shadow">
          <ExchangeRateTable
            rates={rates}
            isLoading={isLoading}
            onOverride={onOverrideRate ? handleOverride : undefined}
          />
        </div>
      </div>

      {/* Override Modal */}
      {showOverrideModal && selectedRate && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg shadow-xl p-6 w-full max-w-md">
            <h3 className="text-lg font-medium text-gray-900 mb-4">Override Exchange Rate</h3>
            <p className="text-sm text-gray-500 mb-4">
              {selectedRate.fromCurrency} to {selectedRate.toCurrency} (
              {new Date(selectedRate.rateDate).toLocaleDateString()})
            </p>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Current Rate</label>
                <input
                  type="text"
                  value={selectedRate.rate.toFixed(6)}
                  disabled
                  className="block w-full rounded-md border-gray-300 bg-gray-50 shadow-sm"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">New Rate</label>
                <input
                  type="number"
                  step="0.000001"
                  value={overrideValue}
                  onChange={(e) => setOverrideValue(e.target.value)}
                  className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Reason for Override
                </label>
                <textarea
                  value={overrideReason}
                  onChange={(e) => setOverrideReason(e.target.value)}
                  required
                  rows={3}
                  className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
                  placeholder="Please provide a reason for this manual override..."
                />
              </div>
            </div>
            <div className="mt-6 flex justify-end space-x-3">
              <button
                type="button"
                onClick={() => setShowOverrideModal(false)}
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200"
              >
                Cancel
              </button>
              <button
                type="button"
                onClick={handleSubmitOverride}
                disabled={!overrideValue || !overrideReason}
                className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50"
              >
                Override Rate
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
