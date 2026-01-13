/**
 * ReportConfigForm - Story 145.5
 *
 * Form component for configuring multi-currency reports.
 */

import { useState } from 'react';
import { CurrencySelector } from './CurrencySelector';

export interface ReportConfig {
  name: string;
  description?: string;
  reportCurrency: string;
  showOriginalCurrencies: boolean;
  showConversionDetails: boolean;
  rateDateType: string;
  specificRateDate?: string;
  groupByCurrency: boolean;
  groupByCountry: boolean;
  groupByProperty: boolean;
  isSaved: boolean;
  isDefault: boolean;
}

interface ReportConfigFormProps {
  config?: Partial<ReportConfig>;
  isLoading?: boolean;
  onSave: (config: ReportConfig) => void;
  onCancel?: () => void;
}

const RATE_DATE_TYPES = [
  { value: 'end_of_period', label: 'End of Period' },
  { value: 'average', label: 'Period Average' },
  { value: 'specific_date', label: 'Specific Date' },
];

export function ReportConfigForm({ config, isLoading, onSave, onCancel }: ReportConfigFormProps) {
  const [formData, setFormData] = useState<ReportConfig>({
    name: config?.name || '',
    description: config?.description || '',
    reportCurrency: config?.reportCurrency || 'EUR',
    showOriginalCurrencies: config?.showOriginalCurrencies ?? true,
    showConversionDetails: config?.showConversionDetails ?? true,
    rateDateType: config?.rateDateType || 'end_of_period',
    specificRateDate: config?.specificRateDate,
    groupByCurrency: config?.groupByCurrency ?? true,
    groupByCountry: config?.groupByCountry ?? false,
    groupByProperty: config?.groupByProperty ?? true,
    isSaved: config?.isSaved ?? false,
    isDefault: config?.isDefault ?? false,
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSave(formData);
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Basic Info */}
      <div className="space-y-4">
        <div>
          <label htmlFor="name" className="block text-sm font-medium text-gray-700">
            Report Name
          </label>
          <input
            type="text"
            id="name"
            value={formData.name}
            onChange={(e) => setFormData({ ...formData, name: e.target.value })}
            required
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
            placeholder="Monthly Currency Report"
          />
        </div>

        <div>
          <label htmlFor="description" className="block text-sm font-medium text-gray-700">
            Description
          </label>
          <textarea
            id="description"
            value={formData.description || ''}
            onChange={(e) => setFormData({ ...formData, description: e.target.value })}
            rows={2}
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
            placeholder="Optional description..."
          />
        </div>
      </div>

      {/* Currency Settings */}
      <div className="border-t pt-4">
        <h3 className="text-sm font-medium text-gray-900 mb-4">Currency Settings</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <CurrencySelector
            label="Report Currency"
            value={formData.reportCurrency}
            onChange={(currency) => setFormData({ ...formData, reportCurrency: currency })}
          />

          <div>
            <label htmlFor="rateDateType" className="block text-sm font-medium text-gray-700">
              Exchange Rate Date
            </label>
            <select
              id="rateDateType"
              value={formData.rateDateType}
              onChange={(e) => setFormData({ ...formData, rateDateType: e.target.value })}
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
            >
              {RATE_DATE_TYPES.map((type) => (
                <option key={type.value} value={type.value}>
                  {type.label}
                </option>
              ))}
            </select>
          </div>
        </div>

        {formData.rateDateType === 'specific_date' && (
          <div className="mt-4">
            <label htmlFor="specificRateDate" className="block text-sm font-medium text-gray-700">
              Specific Date
            </label>
            <input
              type="date"
              id="specificRateDate"
              value={formData.specificRateDate || ''}
              onChange={(e) => setFormData({ ...formData, specificRateDate: e.target.value })}
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
            />
          </div>
        )}
      </div>

      {/* Display Options */}
      <div className="border-t pt-4">
        <h3 className="text-sm font-medium text-gray-900 mb-4">Display Options</h3>
        <div className="space-y-3">
          <label className="flex items-center space-x-2">
            <input
              type="checkbox"
              checked={formData.showOriginalCurrencies}
              onChange={(e) =>
                setFormData({ ...formData, showOriginalCurrencies: e.target.checked })
              }
              className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
            />
            <span className="text-sm text-gray-700">Show original currencies</span>
          </label>

          <label className="flex items-center space-x-2">
            <input
              type="checkbox"
              checked={formData.showConversionDetails}
              onChange={(e) =>
                setFormData({ ...formData, showConversionDetails: e.target.checked })
              }
              className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
            />
            <span className="text-sm text-gray-700">Show conversion details</span>
          </label>
        </div>
      </div>

      {/* Grouping Options */}
      <div className="border-t pt-4">
        <h3 className="text-sm font-medium text-gray-900 mb-4">Group By</h3>
        <div className="space-y-3">
          <label className="flex items-center space-x-2">
            <input
              type="checkbox"
              checked={formData.groupByCurrency}
              onChange={(e) => setFormData({ ...formData, groupByCurrency: e.target.checked })}
              className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
            />
            <span className="text-sm text-gray-700">Currency</span>
          </label>

          <label className="flex items-center space-x-2">
            <input
              type="checkbox"
              checked={formData.groupByCountry}
              onChange={(e) => setFormData({ ...formData, groupByCountry: e.target.checked })}
              className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
            />
            <span className="text-sm text-gray-700">Country</span>
          </label>

          <label className="flex items-center space-x-2">
            <input
              type="checkbox"
              checked={formData.groupByProperty}
              onChange={(e) => setFormData({ ...formData, groupByProperty: e.target.checked })}
              className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
            />
            <span className="text-sm text-gray-700">Property</span>
          </label>
        </div>
      </div>

      {/* Save Options */}
      <div className="border-t pt-4">
        <div className="space-y-3">
          <label className="flex items-center space-x-2">
            <input
              type="checkbox"
              checked={formData.isSaved}
              onChange={(e) => setFormData({ ...formData, isSaved: e.target.checked })}
              className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
            />
            <span className="text-sm text-gray-700">Save this configuration</span>
          </label>

          {formData.isSaved && (
            <label className="flex items-center space-x-2 ml-6">
              <input
                type="checkbox"
                checked={formData.isDefault}
                onChange={(e) => setFormData({ ...formData, isDefault: e.target.checked })}
                className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
              <span className="text-sm text-gray-700">Set as default</span>
            </label>
          )}
        </div>
      </div>

      {/* Actions */}
      <div className="border-t pt-4 flex justify-end space-x-3">
        {onCancel && (
          <button
            type="button"
            onClick={onCancel}
            className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200"
          >
            Cancel
          </button>
        )}
        <button
          type="submit"
          disabled={isLoading || !formData.name}
          className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50"
        >
          {isLoading ? 'Saving...' : 'Save Configuration'}
        </button>
      </div>
    </form>
  );
}
