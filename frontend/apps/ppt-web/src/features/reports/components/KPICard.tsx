/**
 * KPICard component - Story 53.3
 *
 * Displays a single KPI metric with trend indicator.
 */

import type { KPIMetric } from '@ppt/api-client';

interface KPICardProps {
  metric: KPIMetric;
  onClick?: () => void;
}

function formatValue(value: number, unit?: string): string {
  if (unit === 'currency') {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'EUR',
      maximumFractionDigits: 0,
    }).format(value);
  }
  if (unit === 'percent') {
    return `${value.toFixed(1)}%`;
  }
  return new Intl.NumberFormat('en-US').format(value);
}

export function KPICard({ metric, onClick }: KPICardProps) {
  const getTrendColor = () => {
    switch (metric.trend) {
      case 'up':
        return 'text-green-600';
      case 'down':
        return 'text-red-600';
      default:
        return 'text-gray-500';
    }
  };

  const getTrendIcon = () => {
    switch (metric.trend) {
      case 'up':
        return (
          <svg
            className="w-4 h-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M5 10l7-7m0 0l7 7m-7-7v18"
            />
          </svg>
        );
      case 'down':
        return (
          <svg
            className="w-4 h-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M19 14l-7 7m0 0l-7-7m7 7V3"
            />
          </svg>
        );
      default:
        return (
          <svg
            className="w-4 h-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            aria-hidden="true"
          >
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 12h14" />
          </svg>
        );
    }
  };

  const progressToTarget = metric.target
    ? Math.min((metric.value / metric.target) * 100, 100)
    : null;

  return (
    <div
      className={`bg-white rounded-lg shadow p-6 ${onClick ? 'cursor-pointer hover:shadow-md transition-shadow' : ''}`}
      onClick={onClick}
      onKeyDown={onClick ? (e) => e.key === 'Enter' && onClick() : undefined}
      tabIndex={onClick ? 0 : undefined}
    >
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <p className="text-sm font-medium text-gray-500">{metric.name}</p>
          <p className="mt-2 text-3xl font-bold text-gray-900">
            {formatValue(metric.value, metric.unit)}
          </p>
        </div>

        {metric.change_percentage !== undefined && (
          <div className={`flex items-center gap-1 ${getTrendColor()}`}>
            {getTrendIcon()}
            <span className="text-sm font-medium">
              {metric.change_percentage > 0 ? '+' : ''}
              {metric.change_percentage.toFixed(1)}%
            </span>
          </div>
        )}
      </div>

      {/* Progress to Target */}
      {progressToTarget !== null && metric.target && (
        <div className="mt-4">
          <div className="flex items-center justify-between text-xs text-gray-500 mb-1">
            <span>Progress to target</span>
            <span>{formatValue(metric.target, metric.unit)}</span>
          </div>
          <div className="h-2 bg-gray-200 rounded-full overflow-hidden">
            <div
              className={`h-full transition-all duration-500 ${
                progressToTarget >= 100
                  ? 'bg-green-500'
                  : progressToTarget >= 75
                    ? 'bg-blue-500'
                    : 'bg-yellow-500'
              }`}
              style={{ width: `${progressToTarget}%` }}
            />
          </div>
        </div>
      )}

      {/* Previous Value Comparison */}
      {metric.previous_value !== undefined && (
        <p className="mt-3 text-xs text-gray-400">
          Previous: {formatValue(metric.previous_value, metric.unit)}
        </p>
      )}
    </div>
  );
}
