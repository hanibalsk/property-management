/**
 * API Key Usage Chart Component (Epic 69 - Story 69.1)
 *
 * Displays usage statistics and charts for an API key.
 */

import type { ApiKeyUsageStats } from '../types';

interface ApiKeyUsageChartProps {
  stats: ApiKeyUsageStats[];
  keyName: string;
  isLoading?: boolean;
}

export function ApiKeyUsageChart({ stats, keyName, isLoading }: ApiKeyUsageChartProps) {
  if (isLoading) {
    return (
      <div className="p-8 text-center">
        <div className="animate-spin w-8 h-8 border-4 border-blue-600 border-t-transparent rounded-full mx-auto" />
        <p className="mt-4 text-muted-foreground">Loading usage data...</p>
      </div>
    );
  }

  if (stats.length === 0) {
    return (
      <div className="p-8 text-center bg-gray-50 rounded-lg">
        <svg
          className="w-12 h-12 mx-auto text-gray-400 mb-4"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
          />
        </svg>
        <h3 className="text-lg font-medium mb-1">No usage data yet</h3>
        <p className="text-muted-foreground">
          Usage statistics will appear here once the API key is used.
        </p>
      </div>
    );
  }

  // Calculate summary stats
  const totalRequests = stats.reduce((sum, s) => sum + s.totalRequests, 0);
  const totalSuccessful = stats.reduce((sum, s) => sum + s.successfulRequests, 0);
  const totalFailed = stats.reduce((sum, s) => sum + s.failedRequests, 0);
  const totalRateLimited = stats.reduce((sum, s) => sum + s.rateLimitedRequests, 0);
  const avgResponseTime =
    stats.filter((s) => s.avgResponseTimeMs).reduce((sum, s) => sum + (s.avgResponseTimeMs || 0), 0) /
    stats.filter((s) => s.avgResponseTimeMs).length;
  const successRate = totalRequests > 0 ? (totalSuccessful / totalRequests) * 100 : 0;

  // Get max for chart scaling
  const maxRequests = Math.max(...stats.map((s) => s.totalRequests));

  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-semibold mb-1">Usage for {keyName}</h3>
        <p className="text-sm text-muted-foreground">Last 30 days of API activity</p>
      </div>

      {/* Summary Stats */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        <StatCard
          label="Total Requests"
          value={formatNumber(totalRequests)}
          color="blue"
        />
        <StatCard
          label="Success Rate"
          value={`${successRate.toFixed(1)}%`}
          color={successRate >= 99 ? 'green' : successRate >= 95 ? 'yellow' : 'red'}
        />
        <StatCard
          label="Rate Limited"
          value={formatNumber(totalRateLimited)}
          color={totalRateLimited === 0 ? 'green' : 'yellow'}
        />
        <StatCard
          label="Avg Response"
          value={avgResponseTime ? `${avgResponseTime.toFixed(0)}ms` : 'N/A'}
          color="blue"
        />
      </div>

      {/* Bar Chart */}
      <div className="p-4 bg-white rounded-lg border">
        <h4 className="text-sm font-medium mb-4">Daily Requests</h4>
        <div className="h-48 flex items-end gap-1">
          {stats.map((stat, index) => {
            const height = maxRequests > 0 ? (stat.totalRequests / maxRequests) * 100 : 0;
            const successHeight = stat.totalRequests > 0
              ? (stat.successfulRequests / stat.totalRequests) * height
              : 0;
            const failedHeight = stat.totalRequests > 0
              ? (stat.failedRequests / stat.totalRequests) * height
              : 0;

            return (
              <div
                key={stat.date}
                className="flex-1 flex flex-col-reverse gap-0.5 group relative"
                title={`${stat.date}: ${stat.totalRequests} requests`}
              >
                {/* Success portion */}
                <div
                  className="bg-green-500 rounded-t transition-all group-hover:bg-green-600"
                  style={{ height: `${successHeight}%` }}
                />
                {/* Failed portion */}
                {failedHeight > 0 && (
                  <div
                    className="bg-red-500 transition-all group-hover:bg-red-600"
                    style={{ height: `${failedHeight}%` }}
                  />
                )}
                {/* Tooltip */}
                <div className="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 hidden group-hover:block z-10">
                  <div className="bg-gray-900 text-white text-xs rounded px-2 py-1 whitespace-nowrap">
                    {formatShortDate(stat.date)}: {stat.totalRequests} requests
                    <br />
                    {stat.successfulRequests} success, {stat.failedRequests} failed
                  </div>
                </div>
              </div>
            );
          })}
        </div>
        <div className="flex justify-between mt-2 text-xs text-muted-foreground">
          <span>{formatShortDate(stats[0]?.date || '')}</span>
          <span>{formatShortDate(stats[stats.length - 1]?.date || '')}</span>
        </div>
      </div>

      {/* Legend */}
      <div className="flex items-center gap-6 text-sm">
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 bg-green-500 rounded" />
          <span>Successful</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 bg-red-500 rounded" />
          <span>Failed</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 bg-yellow-500 rounded" />
          <span>Rate Limited</span>
        </div>
      </div>

      {/* Detailed Table */}
      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead className="bg-gray-50">
            <tr>
              <th className="px-4 py-2 text-left font-medium">Date</th>
              <th className="px-4 py-2 text-right font-medium">Total</th>
              <th className="px-4 py-2 text-right font-medium">Success</th>
              <th className="px-4 py-2 text-right font-medium">Failed</th>
              <th className="px-4 py-2 text-right font-medium">Rate Limited</th>
              <th className="px-4 py-2 text-right font-medium">Avg Response</th>
            </tr>
          </thead>
          <tbody className="divide-y">
            {stats.slice().reverse().slice(0, 7).map((stat) => (
              <tr key={stat.date} className="hover:bg-gray-50">
                <td className="px-4 py-2">{formatShortDate(stat.date)}</td>
                <td className="px-4 py-2 text-right font-mono">
                  {stat.totalRequests.toLocaleString()}
                </td>
                <td className="px-4 py-2 text-right font-mono text-green-600">
                  {stat.successfulRequests.toLocaleString()}
                </td>
                <td className="px-4 py-2 text-right font-mono text-red-600">
                  {stat.failedRequests.toLocaleString()}
                </td>
                <td className="px-4 py-2 text-right font-mono text-yellow-600">
                  {stat.rateLimitedRequests.toLocaleString()}
                </td>
                <td className="px-4 py-2 text-right font-mono">
                  {stat.avgResponseTimeMs ? `${stat.avgResponseTimeMs.toFixed(0)}ms` : '-'}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

// ==================== Helper Components ====================

interface StatCardProps {
  label: string;
  value: string;
  color: 'blue' | 'green' | 'yellow' | 'red';
}

function StatCard({ label, value, color }: StatCardProps) {
  const colors = {
    blue: 'bg-blue-50 text-blue-700',
    green: 'bg-green-50 text-green-700',
    yellow: 'bg-yellow-50 text-yellow-700',
    red: 'bg-red-50 text-red-700',
  };

  return (
    <div className={`p-4 rounded-lg ${colors[color]}`}>
      <p className="text-xs font-medium opacity-75">{label}</p>
      <p className="text-2xl font-bold">{value}</p>
    </div>
  );
}

// ==================== Utility Functions ====================

function formatNumber(num: number): string {
  if (num >= 1000000) {
    return `${(num / 1000000).toFixed(1)}M`;
  }
  if (num >= 1000) {
    return `${(num / 1000).toFixed(1)}K`;
  }
  return num.toLocaleString();
}

function formatShortDate(dateString: string): string {
  if (!dateString) return '';
  const date = new Date(dateString);
  return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
}
