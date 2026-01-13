// Epic 144: Portfolio Performance Analytics - Financial Metrics Table Component
import type React from 'react';

interface FinancialMetric {
  propertyId?: string;
  propertyName?: string;
  noi: number;
  capRate?: number;
  cashOnCash?: number;
  irr?: number;
  dscr?: number;
  equityMultiple?: number;
  period: string;
  currency: string;
}

interface FinancialMetricsTableProps {
  metrics: FinancialMetric[];
  showPropertyColumn?: boolean;
  title?: string;
}

export const FinancialMetricsTable: React.FC<FinancialMetricsTableProps> = ({
  metrics,
  showPropertyColumn = true,
  title = 'Financial Metrics',
}) => {
  const formatCurrency = (value: number, currency: string) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: currency,
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(value);
  };

  const formatPercent = (value?: number) => {
    if (value === undefined || value === null) return '-';
    return `${value.toFixed(2)}%`;
  };

  const formatRatio = (value?: number) => {
    if (value === undefined || value === null) return '-';
    return `${value.toFixed(2)}x`;
  };

  const getCapRateColor = (capRate?: number) => {
    if (capRate === undefined) return 'text-gray-500';
    if (capRate >= 8) return 'text-green-600';
    if (capRate >= 6) return 'text-blue-600';
    if (capRate >= 4) return 'text-yellow-600';
    return 'text-red-600';
  };

  const getDscrColor = (dscr?: number) => {
    if (dscr === undefined) return 'text-gray-500';
    if (dscr >= 1.25) return 'text-green-600';
    if (dscr >= 1.0) return 'text-yellow-600';
    return 'text-red-600';
  };

  if (metrics.length === 0) {
    return (
      <div className="bg-white rounded-lg shadow-md p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">{title}</h3>
        <p className="text-center text-gray-500 py-4">No metrics available</p>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-md overflow-hidden">
      <div className="px-6 py-4 border-b border-gray-200">
        <h3 className="text-lg font-semibold text-gray-900">{title}</h3>
      </div>

      <div className="overflow-x-auto">
        <table className="min-w-full divide-y divide-gray-200">
          <thead className="bg-gray-50">
            <tr>
              {showPropertyColumn && (
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Property
                </th>
              )}
              <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                NOI
              </th>
              <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                Cap Rate
              </th>
              <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                Cash-on-Cash
              </th>
              <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                IRR
              </th>
              <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                DSCR
              </th>
              <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                Equity Multiple
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Period
              </th>
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {metrics.map((metric, index) => (
              <tr key={metric.propertyId || index} className="hover:bg-gray-50">
                {showPropertyColumn && (
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                    {metric.propertyName || 'Portfolio'}
                  </td>
                )}
                <td className="px-6 py-4 whitespace-nowrap text-sm text-right text-gray-900 font-semibold">
                  {formatCurrency(metric.noi, metric.currency)}
                </td>
                <td
                  className={`px-6 py-4 whitespace-nowrap text-sm text-right font-semibold ${getCapRateColor(metric.capRate)}`}
                >
                  {formatPercent(metric.capRate)}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-right text-gray-900">
                  {formatPercent(metric.cashOnCash)}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-right text-gray-900">
                  {formatPercent(metric.irr)}
                </td>
                <td
                  className={`px-6 py-4 whitespace-nowrap text-sm text-right font-semibold ${getDscrColor(metric.dscr)}`}
                >
                  {formatRatio(metric.dscr)}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-right text-gray-900">
                  {formatRatio(metric.equityMultiple)}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  {metric.period}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Legend */}
      <div className="px-6 py-3 bg-gray-50 border-t border-gray-200">
        <div className="flex items-center space-x-6 text-xs text-gray-500">
          <div className="flex items-center">
            <span className="font-semibold text-green-600 mr-1">Good</span>
            <span>Cap Rate: 8%+ | DSCR: 1.25x+</span>
          </div>
          <div className="flex items-center">
            <span className="font-semibold text-yellow-600 mr-1">Fair</span>
            <span>Cap Rate: 4-8% | DSCR: 1.0-1.25x</span>
          </div>
          <div className="flex items-center">
            <span className="font-semibold text-red-600 mr-1">Needs Attention</span>
            <span>Cap Rate: &lt;4% | DSCR: &lt;1.0x</span>
          </div>
        </div>
      </div>
    </div>
  );
};
