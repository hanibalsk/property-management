/**
 * CurrencyConfigForm - Story 145.1
 *
 * Form for configuring organization-level currency settings.
 */

import { useState } from 'react';

export interface CurrencyConfig {
  baseCurrency: string;
  enabledCurrencies: string[];
  displayCurrency?: string;
  showOriginalAmount: boolean;
  decimalPlaces: number;
  exchangeRateSource: string;
  autoUpdateRates: boolean;
  updateFrequencyHours: number;
  roundingMode: string;
}

interface CurrencyConfigFormProps {
  config: CurrencyConfig;
  isLoading?: boolean;
  onSave: (config: CurrencyConfig) => void;
}

const SUPPORTED_CURRENCIES = [
  { code: 'EUR', name: 'Euro' },
  { code: 'CZK', name: 'Czech Koruna' },
  { code: 'CHF', name: 'Swiss Franc' },
  { code: 'GBP', name: 'British Pound' },
  { code: 'PLN', name: 'Polish Zloty' },
  { code: 'USD', name: 'US Dollar' },
  { code: 'HUF', name: 'Hungarian Forint' },
  { code: 'RON', name: 'Romanian Leu' },
  { code: 'SEK', name: 'Swedish Krona' },
  { code: 'DKK', name: 'Danish Krone' },
  { code: 'NOK', name: 'Norwegian Krone' },
];

const EXCHANGE_RATE_SOURCES = [
  { value: 'ecb', label: 'European Central Bank (ECB)' },
  { value: 'xe', label: 'XE.com' },
  { value: 'manual', label: 'Manual Entry' },
];

const ROUNDING_MODES = [
  { value: 'half_up', label: 'Round Half Up' },
  { value: 'half_down', label: 'Round Half Down' },
  { value: 'ceil', label: 'Always Round Up' },
  { value: 'floor', label: 'Always Round Down' },
];

export function CurrencyConfigForm({ config, isLoading, onSave }: CurrencyConfigFormProps) {
  const [formData, setFormData] = useState<CurrencyConfig>(config);

  const handleCurrencyToggle = (currency: string) => {
    const enabled = formData.enabledCurrencies.includes(currency);
    if (enabled) {
      setFormData({
        ...formData,
        enabledCurrencies: formData.enabledCurrencies.filter((c) => c !== currency),
      });
    } else {
      setFormData({
        ...formData,
        enabledCurrencies: [...formData.enabledCurrencies, currency],
      });
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSave(formData);
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Base Currency */}
      <div>
        <label htmlFor="baseCurrency" className="block text-sm font-medium text-gray-700">
          Base Currency
        </label>
        <select
          id="baseCurrency"
          value={formData.baseCurrency}
          onChange={(e) => setFormData({ ...formData, baseCurrency: e.target.value })}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
        >
          {SUPPORTED_CURRENCIES.map((currency) => (
            <option key={currency.code} value={currency.code}>
              {currency.code} - {currency.name}
            </option>
          ))}
        </select>
        <p className="mt-1 text-sm text-gray-500">
          All amounts will be converted to this currency for reporting.
        </p>
      </div>

      {/* Enabled Currencies */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">Enabled Currencies</label>
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-2">
          {SUPPORTED_CURRENCIES.map((currency) => (
            <label
              key={currency.code}
              className="flex items-center space-x-2 p-2 border rounded hover:bg-gray-50 cursor-pointer"
            >
              <input
                type="checkbox"
                checked={formData.enabledCurrencies.includes(currency.code)}
                onChange={() => handleCurrencyToggle(currency.code)}
                className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
              <span className="text-sm">
                {currency.code} - {currency.name}
              </span>
            </label>
          ))}
        </div>
      </div>

      {/* Exchange Rate Source */}
      <div>
        <label htmlFor="exchangeRateSource" className="block text-sm font-medium text-gray-700">
          Exchange Rate Source
        </label>
        <select
          id="exchangeRateSource"
          value={formData.exchangeRateSource}
          onChange={(e) => setFormData({ ...formData, exchangeRateSource: e.target.value })}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
        >
          {EXCHANGE_RATE_SOURCES.map((source) => (
            <option key={source.value} value={source.value}>
              {source.label}
            </option>
          ))}
        </select>
      </div>

      {/* Auto Update Settings */}
      <div className="flex items-center space-x-4">
        <label className="flex items-center space-x-2">
          <input
            type="checkbox"
            checked={formData.autoUpdateRates}
            onChange={(e) => setFormData({ ...formData, autoUpdateRates: e.target.checked })}
            className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
          <span className="text-sm font-medium text-gray-700">Auto-update exchange rates</span>
        </label>
        {formData.autoUpdateRates && (
          <div className="flex items-center space-x-2">
            <span className="text-sm text-gray-500">every</span>
            <input
              type="number"
              value={formData.updateFrequencyHours}
              onChange={(e) =>
                setFormData({
                  ...formData,
                  updateFrequencyHours: Number.parseInt(e.target.value, 10),
                })
              }
              min={1}
              max={168}
              className="w-20 rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
            />
            <span className="text-sm text-gray-500">hours</span>
          </div>
        )}
      </div>

      {/* Display Settings */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label htmlFor="decimalPlaces" className="block text-sm font-medium text-gray-700">
            Decimal Places
          </label>
          <input
            type="number"
            id="decimalPlaces"
            value={formData.decimalPlaces}
            onChange={(e) =>
              setFormData({ ...formData, decimalPlaces: Number.parseInt(e.target.value, 10) })
            }
            min={0}
            max={4}
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          />
        </div>
        <div>
          <label htmlFor="roundingMode" className="block text-sm font-medium text-gray-700">
            Rounding Mode
          </label>
          <select
            id="roundingMode"
            value={formData.roundingMode}
            onChange={(e) => setFormData({ ...formData, roundingMode: e.target.value })}
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          >
            {ROUNDING_MODES.map((mode) => (
              <option key={mode.value} value={mode.value}>
                {mode.label}
              </option>
            ))}
          </select>
        </div>
      </div>

      {/* Show Original Amount */}
      <div>
        <label className="flex items-center space-x-2">
          <input
            type="checkbox"
            checked={formData.showOriginalAmount}
            onChange={(e) => setFormData({ ...formData, showOriginalAmount: e.target.checked })}
            className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
          <span className="text-sm font-medium text-gray-700">
            Show original amount alongside converted amount
          </span>
        </label>
      </div>

      {/* Submit Button */}
      <div className="flex justify-end">
        <button
          type="submit"
          disabled={isLoading}
          className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
        >
          {isLoading ? 'Saving...' : 'Save Configuration'}
        </button>
      </div>
    </form>
  );
}
