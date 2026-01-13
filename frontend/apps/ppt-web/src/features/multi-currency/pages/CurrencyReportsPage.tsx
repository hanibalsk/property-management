/**
 * CurrencyReportsPage - Story 145.5
 *
 * Page for generating and viewing consolidated multi-currency reports.
 */

import { useState } from 'react';
import { type ReportConfig, ReportConfigForm } from '../components/ReportConfigForm';

interface ReportSnapshot {
  id: string;
  periodStart: string;
  periodEnd: string;
  reportCurrency: string;
  totalRevenue: number;
  totalExpenses: number;
  netIncome: number;
  exchangeRateImpact?: number;
  unrealizedFxGainLoss?: number;
  realizedFxGainLoss?: number;
  generatedAt: string;
}

interface SavedReportConfig {
  id: string;
  name: string;
  description?: string;
  reportCurrency: string;
  isDefault: boolean;
}

interface CurrencyReportsPageProps {
  snapshots: ReportSnapshot[];
  savedConfigs: SavedReportConfig[];
  isLoading?: boolean;
  onGenerateReport?: (config: {
    periodStart: string;
    periodEnd: string;
    reportCurrency: string;
    configId?: string;
  }) => void;
  onSaveConfig?: (config: ReportConfig) => void;
  onDeleteConfig?: (configId: string) => void;
  onViewSnapshot?: (snapshotId: string) => void;
}

export function CurrencyReportsPage({
  snapshots,
  savedConfigs,
  isLoading,
  onGenerateReport,
  onSaveConfig,
  onDeleteConfig,
  onViewSnapshot,
}: CurrencyReportsPageProps) {
  const [showConfigForm, setShowConfigForm] = useState(false);
  const [periodStart, setPeriodStart] = useState<string>('');
  const [periodEnd, setPeriodEnd] = useState<string>('');
  const [reportCurrency, setReportCurrency] = useState<string>('EUR');
  const [selectedConfigId, setSelectedConfigId] = useState<string>('');

  const formatCurrency = (amount: number, currency: string) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: currency,
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(amount);
  };

  const handleGenerateReport = () => {
    if (periodStart && periodEnd) {
      onGenerateReport?.({
        periodStart,
        periodEnd,
        reportCurrency,
        configId: selectedConfigId || undefined,
      });
    }
  };

  // Set default dates (last month)
  const setLastMonth = () => {
    const now = new Date();
    const firstDay = new Date(now.getFullYear(), now.getMonth() - 1, 1);
    const lastDay = new Date(now.getFullYear(), now.getMonth(), 0);
    setPeriodStart(firstDay.toISOString().split('T')[0]);
    setPeriodEnd(lastDay.toISOString().split('T')[0]);
  };

  const setLastQuarter = () => {
    const now = new Date();
    const currentQuarter = Math.floor(now.getMonth() / 3);
    const firstMonth = (currentQuarter - 1) * 3;
    const firstDay = new Date(now.getFullYear(), firstMonth, 1);
    const lastDay = new Date(now.getFullYear(), firstMonth + 3, 0);
    setPeriodStart(firstDay.toISOString().split('T')[0]);
    setPeriodEnd(lastDay.toISOString().split('T')[0]);
  };

  const setLastYear = () => {
    const now = new Date();
    const firstDay = new Date(now.getFullYear() - 1, 0, 1);
    const lastDay = new Date(now.getFullYear() - 1, 11, 31);
    setPeriodStart(firstDay.toISOString().split('T')[0]);
    setPeriodEnd(lastDay.toISOString().split('T')[0]);
  };

  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-2xl font-bold text-gray-900">Currency Reports</h1>
              <p className="text-sm text-gray-500">
                Generate consolidated multi-currency reports with exchange rate analysis
              </p>
            </div>
            <button
              type="button"
              onClick={() => setShowConfigForm(!showConfigForm)}
              className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
            >
              {showConfigForm ? 'Hide Config Form' : 'New Report Config'}
            </button>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6 space-y-6">
        {/* Report Configuration Form */}
        {showConfigForm && (
          <div className="bg-white rounded-lg shadow p-6">
            <h2 className="text-lg font-medium text-gray-900 mb-4">Create Report Configuration</h2>
            <ReportConfigForm
              isLoading={isLoading}
              onSave={(config) => {
                onSaveConfig?.(config);
                setShowConfigForm(false);
              }}
              onCancel={() => setShowConfigForm(false)}
            />
          </div>
        )}

        {/* Report Generator */}
        <div className="bg-white rounded-lg shadow p-6">
          <h2 className="text-lg font-medium text-gray-900 mb-4">Generate Report</h2>
          <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Period Start</label>
              <input
                type="date"
                value={periodStart}
                onChange={(e) => setPeriodStart(e.target.value)}
                className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Period End</label>
              <input
                type="date"
                value={periodEnd}
                onChange={(e) => setPeriodEnd(e.target.value)}
                className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Report Currency
              </label>
              <select
                value={reportCurrency}
                onChange={(e) => setReportCurrency(e.target.value)}
                className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
              >
                <option value="EUR">EUR - Euro</option>
                <option value="CZK">CZK - Czech Koruna</option>
                <option value="GBP">GBP - British Pound</option>
                <option value="CHF">CHF - Swiss Franc</option>
                <option value="USD">USD - US Dollar</option>
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Use Configuration
              </label>
              <select
                value={selectedConfigId}
                onChange={(e) => setSelectedConfigId(e.target.value)}
                className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
              >
                <option value="">Default Settings</option>
                {savedConfigs.map((config) => (
                  <option key={config.id} value={config.id}>
                    {config.name} {config.isDefault && '(Default)'}
                  </option>
                ))}
              </select>
            </div>
          </div>

          {/* Quick Period Selection */}
          <div className="flex flex-wrap gap-2 mb-4">
            <button
              type="button"
              onClick={setLastMonth}
              className="px-3 py-1 text-sm text-gray-600 bg-gray-100 rounded hover:bg-gray-200"
            >
              Last Month
            </button>
            <button
              type="button"
              onClick={setLastQuarter}
              className="px-3 py-1 text-sm text-gray-600 bg-gray-100 rounded hover:bg-gray-200"
            >
              Last Quarter
            </button>
            <button
              type="button"
              onClick={setLastYear}
              className="px-3 py-1 text-sm text-gray-600 bg-gray-100 rounded hover:bg-gray-200"
            >
              Last Year
            </button>
          </div>

          <button
            type="button"
            onClick={handleGenerateReport}
            disabled={isLoading || !periodStart || !periodEnd}
            className="w-full px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50"
          >
            {isLoading ? 'Generating...' : 'Generate Report'}
          </button>
        </div>

        {/* Saved Configurations */}
        {savedConfigs.length > 0 && (
          <div className="bg-white rounded-lg shadow">
            <div className="px-4 py-3 border-b border-gray-200">
              <h2 className="text-lg font-medium text-gray-900">Saved Configurations</h2>
            </div>
            <div className="divide-y divide-gray-200">
              {savedConfigs.map((config) => (
                <div
                  key={config.id}
                  className="p-4 flex items-center justify-between hover:bg-gray-50"
                >
                  <div>
                    <div className="flex items-center space-x-2">
                      <span className="font-medium text-gray-900">{config.name}</span>
                      {config.isDefault && (
                        <span className="px-2 py-0.5 text-xs bg-blue-100 text-blue-800 rounded">
                          Default
                        </span>
                      )}
                    </div>
                    {config.description && (
                      <p className="text-sm text-gray-500">{config.description}</p>
                    )}
                    <span className="text-xs text-gray-400">Currency: {config.reportCurrency}</span>
                  </div>
                  <div className="flex space-x-2">
                    <button
                      type="button"
                      onClick={() => setSelectedConfigId(config.id)}
                      className="text-sm text-blue-600 hover:text-blue-800"
                    >
                      Use
                    </button>
                    {onDeleteConfig && (
                      <button
                        type="button"
                        onClick={() => onDeleteConfig(config.id)}
                        className="text-sm text-red-600 hover:text-red-800"
                      >
                        Delete
                      </button>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Report History */}
        <div className="bg-white rounded-lg shadow">
          <div className="px-4 py-3 border-b border-gray-200">
            <h2 className="text-lg font-medium text-gray-900">Report History</h2>
          </div>
          {snapshots.length === 0 ? (
            <div className="p-8 text-center text-gray-500">No reports generated yet.</div>
          ) : (
            <div className="overflow-x-auto">
              <table className="min-w-full divide-y divide-gray-200">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                      Period
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                      Currency
                    </th>
                    <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase">
                      Revenue
                    </th>
                    <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase">
                      Expenses
                    </th>
                    <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase">
                      Net Income
                    </th>
                    <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase">
                      FX Impact
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                      Generated
                    </th>
                    <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase">
                      Actions
                    </th>
                  </tr>
                </thead>
                <tbody className="bg-white divide-y divide-gray-200">
                  {snapshots.map((snapshot) => (
                    <tr key={snapshot.id} className="hover:bg-gray-50">
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                        {new Date(snapshot.periodStart).toLocaleDateString()} -{' '}
                        {new Date(snapshot.periodEnd).toLocaleDateString()}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                        {snapshot.reportCurrency}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-right text-green-600">
                        {formatCurrency(snapshot.totalRevenue, snapshot.reportCurrency)}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-right text-red-600">
                        {formatCurrency(snapshot.totalExpenses, snapshot.reportCurrency)}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-right font-medium text-gray-900">
                        {formatCurrency(snapshot.netIncome, snapshot.reportCurrency)}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-right">
                        {snapshot.exchangeRateImpact !== undefined ? (
                          <span
                            className={
                              snapshot.exchangeRateImpact >= 0 ? 'text-green-600' : 'text-red-600'
                            }
                          >
                            {formatCurrency(snapshot.exchangeRateImpact, snapshot.reportCurrency)}
                          </span>
                        ) : (
                          <span className="text-gray-400">-</span>
                        )}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        {new Date(snapshot.generatedAt).toLocaleString()}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-right">
                        {onViewSnapshot && (
                          <button
                            type="button"
                            onClick={() => onViewSnapshot(snapshot.id)}
                            className="text-sm text-blue-600 hover:text-blue-800"
                          >
                            View Details
                          </button>
                        )}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
