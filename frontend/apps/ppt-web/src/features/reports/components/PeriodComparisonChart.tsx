/**
 * PeriodComparisonChart component - Story 53.4
 *
 * Compare metrics across different time periods.
 */

import type { PeriodComparison } from '@ppt/api-client';

interface PeriodComparisonChartProps {
  comparison: PeriodComparison;
  isLoading?: boolean;
}

function formatValue(value: number): string {
  if (value >= 1000000) {
    return `${(value / 1000000).toFixed(1)}M`;
  }
  if (value >= 1000) {
    return `${(value / 1000).toFixed(1)}K`;
  }
  return value.toFixed(0);
}

const COLORS = ['#3b82f6', '#10b981', '#f59e0b', '#8b5cf6'];

export function PeriodComparisonChart({ comparison, isLoading }: PeriodComparisonChartProps) {
  if (isLoading) {
    return (
      <div className="bg-white rounded-lg shadow p-6">
        <div className="animate-pulse">
          <div className="h-6 bg-gray-200 rounded w-1/3 mb-4" />
          <div className="h-48 bg-gray-200 rounded" />
        </div>
      </div>
    );
  }

  const maxValue = Math.max(...comparison.periods.map((p) => p.value), 1);

  return (
    <div className="bg-white rounded-lg shadow overflow-hidden">
      {/* Header */}
      <div className="px-6 py-4 border-b border-gray-200">
        <div className="flex items-center justify-between">
          <div>
            <h3 className="text-lg font-medium text-gray-900">Period Comparison</h3>
            <p className="text-sm text-gray-500 mt-1">{comparison.metric}</p>
          </div>
          <div
            className={`text-right ${
              comparison.difference >= 0 ? 'text-green-600' : 'text-red-600'
            }`}
          >
            <p className="text-2xl font-bold">
              {comparison.difference >= 0 ? '+' : ''}
              {comparison.difference_percentage.toFixed(1)}%
            </p>
            <p className="text-sm">
              {comparison.difference >= 0 ? '+' : ''}
              {formatValue(comparison.difference)}
            </p>
          </div>
        </div>
      </div>

      {/* Bar Chart */}
      <div className="p-6">
        <div className="space-y-4">
          {comparison.periods.map((period, index) => {
            const percentage = (period.value / maxValue) * 100;
            const color = COLORS[index % COLORS.length];

            return (
              <div key={period.label}>
                <div className="flex items-center justify-between mb-1">
                  <div>
                    <span className="text-sm font-medium text-gray-700">{period.label}</span>
                    <span className="text-xs text-gray-500 ml-2">
                      (
                      {new Date(period.start_date).toLocaleDateString('en-US', {
                        month: 'short',
                        day: 'numeric',
                      })}{' '}
                      -{' '}
                      {new Date(period.end_date).toLocaleDateString('en-US', {
                        month: 'short',
                        day: 'numeric',
                      })}
                      )
                    </span>
                  </div>
                  <span className="text-sm font-bold text-gray-900">
                    {formatValue(period.value)}
                  </span>
                </div>
                <div className="h-8 bg-gray-100 rounded-lg overflow-hidden relative">
                  <div
                    className="h-full rounded-lg transition-all duration-500 flex items-center"
                    style={{
                      width: `${percentage}%`,
                      backgroundColor: color,
                    }}
                  >
                    {percentage > 15 && (
                      <span className="ml-2 text-xs text-white font-medium">
                        {percentage.toFixed(0)}%
                      </span>
                    )}
                  </div>
                </div>
              </div>
            );
          })}
        </div>

        {/* Comparison Summary */}
        {comparison.periods.length >= 2 && (
          <div className="mt-6 pt-4 border-t border-gray-100">
            <div className="grid grid-cols-2 gap-4 text-center">
              <div>
                <p className="text-sm text-gray-500">Highest Period</p>
                <p className="text-lg font-bold text-gray-900">
                  {comparison.periods.reduce((max, p) => (p.value > max.value ? p : max)).label}
                </p>
              </div>
              <div>
                <p className="text-sm text-gray-500">Lowest Period</p>
                <p className="text-lg font-bold text-gray-900">
                  {comparison.periods.reduce((min, p) => (p.value < min.value ? p : min)).label}
                </p>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
