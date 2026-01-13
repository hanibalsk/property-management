/**
 * MultiCurrencyDashboardPage - Epic 145
 *
 * Main dashboard for multi-currency management showing currency config,
 * recent exchange rates, transactions, and exposure analysis.
 */

import { useState } from 'react';
import { type CurrencyConfig, CurrencyConfigForm } from '../components/CurrencyConfigForm';
import { CurrencyExposureChart } from '../components/CurrencyExposureChart';
import { ExchangeRateCard } from '../components/ExchangeRateCard';
import { TransactionsList } from '../components/TransactionsList';

interface CurrencyStatistics {
  totalCurrenciesUsed: number;
  totalTransactions: number;
  totalCrossBorderLeases: number;
  totalFxGainLoss: number;
  currencyDistribution: Array<{
    currency: string;
    transactionCount: number;
    totalAmount: number;
    percentage: number;
  }>;
}

interface MultiCurrencyDashboardPageProps {
  organizationId: string;
  config: CurrencyConfig;
  statistics: CurrencyStatistics;
  recentTransactions: Array<{
    id: string;
    sourceType: string;
    sourceId: string;
    originalCurrency: string;
    originalAmount: number;
    baseCurrency: string;
    convertedAmount: number;
    exchangeRate: number;
    rateDate: string;
    conversionStatus: string;
    isRateOverride: boolean;
    realizedGainLoss: number;
    createdAt: string;
  }>;
  exchangeRates: Array<{
    fromCurrency: string;
    toCurrency: string;
    rate: number;
    rateDate: string;
    source: string;
    change24h?: number;
    change7d?: number;
  }>;
  exposures: Array<{
    currency: string;
    receivablesAmount: number;
    payablesAmount: number;
    netExposure: number;
    assetValue: number;
    projectedRevenue: number;
    projectedExpenses: number;
  }>;
  isLoading?: boolean;
  onSaveConfig?: (config: CurrencyConfig) => void;
  onFetchRates?: () => void;
  onViewAllTransactions?: () => void;
  onViewAllRates?: () => void;
}

export function MultiCurrencyDashboardPage({
  config,
  statistics,
  recentTransactions,
  exchangeRates,
  exposures,
  isLoading,
  onSaveConfig,
  onFetchRates,
  onViewAllTransactions,
  onViewAllRates,
}: MultiCurrencyDashboardPageProps) {
  const [showConfigForm, setShowConfigForm] = useState(false);

  const formatCurrency = (amount: number, currency: string) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: currency,
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(amount);
  };

  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-2xl font-bold text-gray-900">Multi-Currency Dashboard</h1>
              <p className="text-sm text-gray-500">
                Base Currency: {config.baseCurrency} | {config.enabledCurrencies.length} currencies
                enabled
              </p>
            </div>
            <div className="flex space-x-3">
              <button
                type="button"
                onClick={onFetchRates}
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200"
              >
                Update Rates
              </button>
              <button
                type="button"
                onClick={() => setShowConfigForm(!showConfigForm)}
                className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
              >
                {showConfigForm ? 'Hide Settings' : 'Currency Settings'}
              </button>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6 space-y-6">
        {/* Configuration Form */}
        {showConfigForm && (
          <div className="bg-white rounded-lg shadow p-6">
            <h2 className="text-lg font-medium text-gray-900 mb-4">Currency Configuration</h2>
            <CurrencyConfigForm
              config={config}
              isLoading={isLoading}
              onSave={(newConfig) => {
                onSaveConfig?.(newConfig);
                setShowConfigForm(false);
              }}
            />
          </div>
        )}

        {/* Statistics Cards */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div className="bg-white rounded-lg shadow p-4">
            <div className="text-sm text-gray-500">Currencies Used</div>
            <div className="text-2xl font-bold text-gray-900">{statistics.totalCurrenciesUsed}</div>
          </div>
          <div className="bg-white rounded-lg shadow p-4">
            <div className="text-sm text-gray-500">Total Transactions</div>
            <div className="text-2xl font-bold text-gray-900">
              {statistics.totalTransactions.toLocaleString()}
            </div>
          </div>
          <div className="bg-white rounded-lg shadow p-4">
            <div className="text-sm text-gray-500">Cross-Border Leases</div>
            <div className="text-2xl font-bold text-gray-900">
              {statistics.totalCrossBorderLeases}
            </div>
          </div>
          <div className="bg-white rounded-lg shadow p-4">
            <div className="text-sm text-gray-500">FX Gain/Loss</div>
            <div
              className={`text-2xl font-bold ${
                statistics.totalFxGainLoss >= 0 ? 'text-green-600' : 'text-red-600'
              }`}
            >
              {formatCurrency(statistics.totalFxGainLoss, config.baseCurrency)}
            </div>
          </div>
        </div>

        {/* Exchange Rates */}
        <div className="bg-white rounded-lg shadow">
          <div className="px-4 py-3 border-b border-gray-200 flex items-center justify-between">
            <h2 className="text-lg font-medium text-gray-900">Exchange Rates</h2>
            {onViewAllRates && (
              <button
                type="button"
                onClick={onViewAllRates}
                className="text-sm text-blue-600 hover:text-blue-800"
              >
                View All
              </button>
            )}
          </div>
          <div className="p-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {exchangeRates.slice(0, 6).map((rate) => (
              <ExchangeRateCard
                key={`${rate.fromCurrency}-${rate.toCurrency}`}
                fromCurrency={rate.fromCurrency}
                toCurrency={rate.toCurrency}
                rate={rate.rate}
                rateDate={rate.rateDate}
                source={rate.source}
                change24h={rate.change24h}
                change7d={rate.change7d}
              />
            ))}
          </div>
        </div>

        {/* Currency Exposure */}
        <div className="bg-white rounded-lg shadow">
          <div className="px-4 py-3 border-b border-gray-200">
            <h2 className="text-lg font-medium text-gray-900">Currency Exposure</h2>
          </div>
          <div className="p-4">
            <CurrencyExposureChart
              exposures={exposures}
              baseCurrency={config.baseCurrency}
              isLoading={isLoading}
            />
          </div>
        </div>

        {/* Recent Transactions */}
        <div className="bg-white rounded-lg shadow">
          <div className="px-4 py-3 border-b border-gray-200 flex items-center justify-between">
            <h2 className="text-lg font-medium text-gray-900">Recent Transactions</h2>
            {onViewAllTransactions && (
              <button
                type="button"
                onClick={onViewAllTransactions}
                className="text-sm text-blue-600 hover:text-blue-800"
              >
                View All
              </button>
            )}
          </div>
          <div className="p-4">
            <TransactionsList transactions={recentTransactions.slice(0, 5)} isLoading={isLoading} />
          </div>
        </div>

        {/* Currency Distribution */}
        {statistics.currencyDistribution.length > 0 && (
          <div className="bg-white rounded-lg shadow">
            <div className="px-4 py-3 border-b border-gray-200">
              <h2 className="text-lg font-medium text-gray-900">Currency Distribution</h2>
            </div>
            <div className="p-4">
              <div className="space-y-3">
                {statistics.currencyDistribution.map((dist) => (
                  <div key={dist.currency} className="flex items-center space-x-4">
                    <span className="w-12 font-medium text-gray-900">{dist.currency}</span>
                    <div className="flex-1 bg-gray-100 rounded h-4 overflow-hidden">
                      <div
                        className="bg-blue-500 h-full transition-all duration-300"
                        style={{ width: `${dist.percentage}%` }}
                      />
                    </div>
                    <span className="w-24 text-right text-sm text-gray-600">
                      {dist.percentage.toFixed(1)}%
                    </span>
                    <span className="w-32 text-right text-sm text-gray-500">
                      {dist.transactionCount} txns
                    </span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
