/**
 * Rate Limit Status Component (Epic 69 - Story 69.4)
 *
 * Displays current rate limit status and tier information.
 */

import type { RateLimitConfig, RateLimitStatus as RateLimitStatusType } from '../types';

interface RateLimitStatusProps {
  status: RateLimitStatusType;
  onUpgradeTier?: () => void;
}

export function RateLimitStatus({ status, onUpgradeTier }: RateLimitStatusProps) {
  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold tracking-tight">Rate Limits</h2>
          <p className="text-muted-foreground">Monitor your API usage and rate limit status</p>
        </div>
        <div className="flex items-center gap-2">
          <span className="text-sm text-muted-foreground">Current tier:</span>
          <span className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full font-medium capitalize">
            {status.tier}
          </span>
        </div>
      </div>

      {/* Current Usage */}
      <div className="grid md:grid-cols-3 gap-4">
        <UsageCard
          title="Per Minute"
          limit={status.requestsPerMinute.limit}
          remaining={status.requestsPerMinute.remaining}
          resetAt={status.requestsPerMinute.resetAt}
        />
        <UsageCard
          title="Per Hour"
          limit={status.requestsPerHour.limit}
          remaining={status.requestsPerHour.remaining}
          resetAt={status.requestsPerHour.resetAt}
        />
        <UsageCard
          title="Per Day"
          limit={status.requestsPerDay.limit}
          remaining={status.requestsPerDay.remaining}
          resetAt={status.requestsPerDay.resetAt}
        />
      </div>

      {/* Upgrade CTA */}
      {status.tier !== 'enterprise' && onUpgradeTier && (
        <div className="p-6 bg-gradient-to-r from-blue-500 to-purple-600 rounded-lg text-white">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-lg font-semibold">Need higher limits?</h3>
              <p className="text-blue-100">
                Upgrade your tier to get more requests and priority support.
              </p>
            </div>
            <button
              type="button"
              onClick={onUpgradeTier}
              className="px-4 py-2 bg-white text-blue-600 rounded-md font-medium hover:bg-blue-50 transition-colors"
            >
              Upgrade Tier
            </button>
          </div>
        </div>
      )}

      {/* Rate Limit Tips */}
      <div className="p-6 bg-gray-50 rounded-lg">
        <h3 className="font-semibold mb-3">Rate Limiting Tips</h3>
        <ul className="space-y-2 text-sm text-muted-foreground">
          <li className="flex items-start gap-2">
            <svg
              className="w-5 h-5 text-green-500 flex-shrink-0"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M5 13l4 4L19 7"
              />
            </svg>
            Implement exponential backoff when receiving 429 responses
          </li>
          <li className="flex items-start gap-2">
            <svg
              className="w-5 h-5 text-green-500 flex-shrink-0"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M5 13l4 4L19 7"
              />
            </svg>
            Cache responses when possible to reduce API calls
          </li>
          <li className="flex items-start gap-2">
            <svg
              className="w-5 h-5 text-green-500 flex-shrink-0"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M5 13l4 4L19 7"
              />
            </svg>
            Use webhooks instead of polling for real-time updates
          </li>
          <li className="flex items-start gap-2">
            <svg
              className="w-5 h-5 text-green-500 flex-shrink-0"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M5 13l4 4L19 7"
              />
            </svg>
            Check the X-RateLimit-* headers in API responses
          </li>
        </ul>
      </div>
    </div>
  );
}

interface UsageCardProps {
  title: string;
  limit: number;
  remaining: number;
  resetAt: string;
}

function UsageCard({ title, limit, remaining, resetAt }: UsageCardProps) {
  const used = limit - remaining;
  const percentage = (used / limit) * 100;
  const isHigh = percentage > 80;
  const isMedium = percentage > 50;

  const getBarColor = () => {
    if (isHigh) return 'bg-red-500';
    if (isMedium) return 'bg-yellow-500';
    return 'bg-green-500';
  };

  const getTextColor = () => {
    if (isHigh) return 'text-red-600';
    if (isMedium) return 'text-yellow-600';
    return 'text-green-600';
  };

  const resetTime = new Date(resetAt);
  const now = new Date();
  const diffMs = resetTime.getTime() - now.getTime();
  const diffSecs = Math.max(0, Math.floor(diffMs / 1000));
  const diffMins = Math.floor(diffSecs / 60);

  const formatResetTime = () => {
    if (diffMins > 60) {
      const hours = Math.floor(diffMins / 60);
      return `${hours}h ${diffMins % 60}m`;
    }
    if (diffMins > 0) {
      return `${diffMins}m ${diffSecs % 60}s`;
    }
    return `${diffSecs}s`;
  };

  return (
    <div className="p-4 bg-white rounded-lg border shadow-sm">
      <div className="flex items-center justify-between mb-2">
        <h3 className="font-medium">{title}</h3>
        <span className={`text-sm font-medium ${getTextColor()}`}>
          {remaining.toLocaleString()} left
        </span>
      </div>

      <div className="h-3 bg-gray-200 rounded-full overflow-hidden mb-2">
        <div
          className={`h-full ${getBarColor()} transition-all duration-300`}
          style={{ width: `${percentage}%` }}
        />
      </div>

      <div className="flex items-center justify-between text-xs text-muted-foreground">
        <span>
          {used.toLocaleString()} / {limit.toLocaleString()} used
        </span>
        <span>Resets in {formatResetTime()}</span>
      </div>
    </div>
  );
}

// Additional component for tier comparison
interface RateLimitTierComparisonProps {
  tiers: RateLimitConfig[];
  currentTier: string;
  onSelectTier?: (tier: string) => void;
}

export function RateLimitTierComparison({
  tiers,
  currentTier,
  onSelectTier,
}: RateLimitTierComparisonProps) {
  return (
    <div className="space-y-6">
      <h3 className="text-lg font-semibold">Compare Rate Limit Tiers</h3>

      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead className="bg-gray-50">
            <tr>
              <th className="px-4 py-3 text-left font-medium">Tier</th>
              <th className="px-4 py-3 text-right font-medium">Per Minute</th>
              <th className="px-4 py-3 text-right font-medium">Per Hour</th>
              <th className="px-4 py-3 text-right font-medium">Per Day</th>
              <th className="px-4 py-3 text-right font-medium">Burst</th>
              <th className="px-4 py-3 text-center font-medium">Action</th>
            </tr>
          </thead>
          <tbody className="divide-y">
            {tiers.map((tier) => (
              <tr
                key={tier.tier}
                className={tier.tier === currentTier ? 'bg-blue-50' : 'hover:bg-gray-50'}
              >
                <td className="px-4 py-3">
                  <div className="flex items-center gap-2">
                    <span className="font-medium capitalize">{tier.tier}</span>
                    {tier.tier === currentTier && (
                      <span className="px-2 py-0.5 text-xs bg-blue-100 text-blue-800 rounded-full">
                        Current
                      </span>
                    )}
                  </div>
                  {tier.description && (
                    <p className="text-xs text-muted-foreground mt-1">{tier.description}</p>
                  )}
                </td>
                <td className="px-4 py-3 text-right font-mono">
                  {tier.requestsPerMinute.toLocaleString()}
                </td>
                <td className="px-4 py-3 text-right font-mono">
                  {tier.requestsPerHour.toLocaleString()}
                </td>
                <td className="px-4 py-3 text-right font-mono">
                  {tier.requestsPerDay.toLocaleString()}
                </td>
                <td className="px-4 py-3 text-right font-mono">
                  {tier.burstLimit?.toLocaleString() || '-'}
                </td>
                <td className="px-4 py-3 text-center">
                  {tier.tier !== currentTier && onSelectTier && (
                    <button
                      type="button"
                      onClick={() => onSelectTier(tier.tier)}
                      className="px-3 py-1 text-xs bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
                    >
                      Upgrade
                    </button>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
