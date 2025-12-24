/**
 * BuildingMetricsCard component - Story 53.3
 *
 * Displays metrics for a single building with drill-down capability.
 */

import type { BuildingAnalytics } from '@ppt/api-client';

interface BuildingMetricsCardProps {
  analytics: BuildingAnalytics;
  onDrillDown?: (section: 'occupancy' | 'maintenance' | 'financial' | 'utility') => void;
}

function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'EUR',
    maximumFractionDigits: 0,
  }).format(amount);
}

export function BuildingMetricsCard({ analytics, onDrillDown }: BuildingMetricsCardProps) {
  const { occupancy, maintenance, financial, utility } = analytics;

  return (
    <div className="bg-white rounded-lg shadow overflow-hidden">
      <div className="px-6 py-4 border-b border-gray-200">
        <h3 className="text-lg font-medium text-gray-900">{analytics.building_name}</h3>
      </div>

      <div className="grid grid-cols-2 divide-x divide-y divide-gray-100">
        {/* Occupancy */}
        <div
          className={`p-4 ${onDrillDown ? 'cursor-pointer hover:bg-gray-50' : ''}`}
          onClick={() => onDrillDown?.('occupancy')}
          onKeyDown={onDrillDown ? (e) => e.key === 'Enter' && onDrillDown('occupancy') : undefined}
          tabIndex={onDrillDown ? 0 : undefined}
          role={onDrillDown ? 'button' : undefined}
          aria-label={onDrillDown ? 'View occupancy metrics details' : undefined}
        >
          <h4 className="text-sm font-medium text-gray-500 mb-3">Occupancy</h4>
          <div className="flex items-end justify-between">
            <div>
              <p className="text-2xl font-bold text-gray-900">
                {occupancy.occupancy_rate.toFixed(1)}%
              </p>
              <p className="text-xs text-gray-500 mt-1">
                {occupancy.occupied_units}/{occupancy.total_units} units
              </p>
            </div>
            <div className="text-right">
              <p className="text-sm text-gray-600">{formatCurrency(occupancy.average_rent)}</p>
              <p className="text-xs text-gray-400">avg rent</p>
            </div>
          </div>
          {/* Mini progress bar */}
          <div className="mt-3 h-2 bg-gray-200 rounded-full overflow-hidden">
            <div
              className={`h-full ${
                occupancy.occupancy_rate >= 90
                  ? 'bg-green-500'
                  : occupancy.occupancy_rate >= 75
                    ? 'bg-yellow-500'
                    : 'bg-red-500'
              }`}
              style={{ width: `${occupancy.occupancy_rate}%` }}
            />
          </div>
        </div>

        {/* Maintenance */}
        <div
          className={`p-4 ${onDrillDown ? 'cursor-pointer hover:bg-gray-50' : ''}`}
          onClick={() => onDrillDown?.('maintenance')}
          onKeyDown={
            onDrillDown ? (e) => e.key === 'Enter' && onDrillDown('maintenance') : undefined
          }
          tabIndex={onDrillDown ? 0 : undefined}
          role={onDrillDown ? 'button' : undefined}
          aria-label={onDrillDown ? 'View maintenance metrics details' : undefined}
        >
          <h4 className="text-sm font-medium text-gray-500 mb-3">Maintenance</h4>
          <div className="flex items-end justify-between">
            <div>
              <p className="text-2xl font-bold text-gray-900">{maintenance.open_faults}</p>
              <p className="text-xs text-gray-500 mt-1">open faults</p>
            </div>
            <div className="text-right">
              <p className="text-sm text-gray-600">
                {maintenance.average_resolution_days.toFixed(1)}d
              </p>
              <p className="text-xs text-gray-400">avg resolution</p>
            </div>
          </div>
          {/* Priority breakdown */}
          <div className="mt-3 flex gap-2 text-xs">
            {maintenance.by_priority.high > 0 && (
              <span className="px-2 py-0.5 bg-red-100 text-red-700 rounded-full">
                {maintenance.by_priority.high} high
              </span>
            )}
            {maintenance.by_priority.medium > 0 && (
              <span className="px-2 py-0.5 bg-yellow-100 text-yellow-700 rounded-full">
                {maintenance.by_priority.medium} med
              </span>
            )}
            {maintenance.by_priority.low > 0 && (
              <span className="px-2 py-0.5 bg-gray-100 text-gray-700 rounded-full">
                {maintenance.by_priority.low} low
              </span>
            )}
          </div>
        </div>

        {/* Financial */}
        <div
          className={`p-4 ${onDrillDown ? 'cursor-pointer hover:bg-gray-50' : ''}`}
          onClick={() => onDrillDown?.('financial')}
          onKeyDown={onDrillDown ? (e) => e.key === 'Enter' && onDrillDown('financial') : undefined}
          tabIndex={onDrillDown ? 0 : undefined}
          role={onDrillDown ? 'button' : undefined}
          aria-label={onDrillDown ? 'View financial metrics details' : undefined}
        >
          <h4 className="text-sm font-medium text-gray-500 mb-3">Financial</h4>
          <div className="flex items-end justify-between">
            <div>
              <p className="text-2xl font-bold text-gray-900">
                {formatCurrency(financial.net_income)}
              </p>
              <p className="text-xs text-gray-500 mt-1">net income</p>
            </div>
            <div className="text-right">
              <p className="text-sm text-gray-600">{financial.collection_rate.toFixed(1)}%</p>
              <p className="text-xs text-gray-400">collected</p>
            </div>
          </div>
          {/* Budget variance */}
          <div className="mt-3">
            <div className="flex items-center justify-between text-xs">
              <span className="text-gray-500">Budget variance</span>
              <span className={financial.budget_variance >= 0 ? 'text-green-600' : 'text-red-600'}>
                {financial.budget_variance >= 0 ? '+' : ''}
                {formatCurrency(financial.budget_variance)}
              </span>
            </div>
          </div>
        </div>

        {/* Utilities */}
        <div
          className={`p-4 ${onDrillDown ? 'cursor-pointer hover:bg-gray-50' : ''}`}
          onClick={() => onDrillDown?.('utility')}
          onKeyDown={onDrillDown ? (e) => e.key === 'Enter' && onDrillDown('utility') : undefined}
          tabIndex={onDrillDown ? 0 : undefined}
          role={onDrillDown ? 'button' : undefined}
          aria-label={onDrillDown ? 'View utility metrics details' : undefined}
        >
          <h4 className="text-sm font-medium text-gray-500 mb-3">Utilities</h4>
          <div className="flex items-end justify-between">
            <div>
              <p className="text-2xl font-bold text-gray-900">
                {formatCurrency(utility.total_cost)}
              </p>
              <p className="text-xs text-gray-500 mt-1">total cost</p>
            </div>
          </div>
          {/* YoY changes */}
          <div className="mt-3 flex flex-wrap gap-2 text-xs">
            {Object.entries(utility.year_over_year_change)
              .slice(0, 3)
              .map(([type, change]) => (
                <span
                  key={type}
                  className={`px-2 py-0.5 rounded-full ${
                    change <= 0 ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'
                  }`}
                >
                  {type}: {change > 0 ? '+' : ''}
                  {change.toFixed(0)}%
                </span>
              ))}
          </div>
        </div>
      </div>
    </div>
  );
}
