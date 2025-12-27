/**
 * Developer Dashboard Component (Epic 69)
 *
 * Main dashboard for the developer portal showing account info,
 * usage statistics, and quick access to API keys and webhooks.
 */

import type { DeveloperAccount, DeveloperUsageSummary, RateLimitStatus } from '../types';

interface DeveloperDashboardProps {
  account: DeveloperAccount;
  usage: DeveloperUsageSummary;
  rateLimitStatus: RateLimitStatus;
}

export function DeveloperDashboard({ account, usage, rateLimitStatus }: DeveloperDashboardProps) {
  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Developer Portal</h1>
          <p className="text-muted-foreground">Manage your API keys, webhooks, and monitor usage</p>
        </div>
        <div className="flex items-center gap-2">
          {account.isVerified ? (
            <span className="inline-flex items-center gap-1 px-2 py-1 text-xs font-medium bg-green-100 text-green-800 rounded-full">
              <svg className="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                <path
                  fillRule="evenodd"
                  d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                  clipRule="evenodd"
                />
              </svg>
              Verified
            </span>
          ) : (
            <span className="inline-flex items-center px-2 py-1 text-xs font-medium bg-yellow-100 text-yellow-800 rounded-full">
              Pending Verification
            </span>
          )}
          <span className="inline-flex items-center px-2 py-1 text-xs font-medium bg-blue-100 text-blue-800 rounded-full capitalize">
            {account.tier} Tier
          </span>
        </div>
      </div>

      {/* Stats Grid */}
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <StatCard
          title="API Keys"
          value={usage.apiKeysCount.toString()}
          description="Active keys"
          icon={
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"
              />
            </svg>
          }
        />
        <StatCard
          title="Webhooks"
          value={usage.webhooksCount.toString()}
          description="Active subscriptions"
          icon={
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M13 10V3L4 14h7v7l9-11h-7z"
              />
            </svg>
          }
        />
        <StatCard
          title="Requests Today"
          value={formatNumber(usage.totalRequestsToday)}
          description="API calls"
          icon={
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a1 1 0 01-1 1H5a1 1 0 01-1-1V4z"
              />
            </svg>
          }
        />
        <StatCard
          title="Rate Limit Hits"
          value={usage.rateLimitHits.toString()}
          description="This month"
          icon={
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
          }
          isWarning={usage.rateLimitHits > 0}
        />
      </div>

      {/* Rate Limit Status */}
      <div className="p-6 bg-white rounded-lg border shadow-sm">
        <h2 className="text-lg font-semibold mb-4">Rate Limit Status</h2>
        <div className="grid gap-4 md:grid-cols-3">
          <RateLimitBar
            label="Per Minute"
            used={
              rateLimitStatus.requestsPerMinute.limit - rateLimitStatus.requestsPerMinute.remaining
            }
            limit={rateLimitStatus.requestsPerMinute.limit}
            resetAt={rateLimitStatus.requestsPerMinute.resetAt}
          />
          <RateLimitBar
            label="Per Hour"
            used={rateLimitStatus.requestsPerHour.limit - rateLimitStatus.requestsPerHour.remaining}
            limit={rateLimitStatus.requestsPerHour.limit}
            resetAt={rateLimitStatus.requestsPerHour.resetAt}
          />
          <RateLimitBar
            label="Per Day"
            used={rateLimitStatus.requestsPerDay.limit - rateLimitStatus.requestsPerDay.remaining}
            limit={rateLimitStatus.requestsPerDay.limit}
            resetAt={rateLimitStatus.requestsPerDay.resetAt}
          />
        </div>
      </div>

      {/* Quick Actions */}
      <div className="grid gap-4 md:grid-cols-3">
        <QuickActionCard
          title="API Keys"
          description="Create and manage your API keys for authentication"
          href="/developer/keys"
          icon={
            <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"
              />
            </svg>
          }
        />
        <QuickActionCard
          title="Webhooks"
          description="Subscribe to events and receive real-time notifications"
          href="/developer/webhooks"
          icon={
            <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M13 10V3L4 14h7v7l9-11h-7z"
              />
            </svg>
          }
        />
        <QuickActionCard
          title="API Documentation"
          description="Explore endpoints and test API calls in the sandbox"
          href="/developer/docs"
          icon={
            <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
              />
            </svg>
          }
        />
      </div>

      {/* Account Info */}
      {account.companyName && (
        <div className="p-6 bg-white rounded-lg border shadow-sm">
          <h2 className="text-lg font-semibold mb-4">Account Information</h2>
          <dl className="grid gap-4 md:grid-cols-2">
            <div>
              <dt className="text-sm text-muted-foreground">Company</dt>
              <dd className="font-medium">{account.companyName}</dd>
            </div>
            {account.website && (
              <div>
                <dt className="text-sm text-muted-foreground">Website</dt>
                <dd>
                  <a
                    href={account.website}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="font-medium text-blue-600 hover:underline"
                  >
                    {account.website}
                  </a>
                </dd>
              </div>
            )}
            <div>
              <dt className="text-sm text-muted-foreground">Contact Email</dt>
              <dd className="font-medium">{account.contactEmail}</dd>
            </div>
            {account.contactName && (
              <div>
                <dt className="text-sm text-muted-foreground">Contact Name</dt>
                <dd className="font-medium">{account.contactName}</dd>
              </div>
            )}
          </dl>
        </div>
      )}
    </div>
  );
}

// ==================== Helper Components ====================

interface StatCardProps {
  title: string;
  value: string;
  description: string;
  icon: React.ReactNode;
  isWarning?: boolean;
}

function StatCard({ title, value, description, icon, isWarning }: StatCardProps) {
  return (
    <div className="p-6 bg-white rounded-lg border shadow-sm">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-sm text-muted-foreground">{title}</p>
          <p className={`text-2xl font-bold ${isWarning ? 'text-yellow-600' : ''}`}>{value}</p>
          <p className="text-xs text-muted-foreground">{description}</p>
        </div>
        <div
          className={`p-3 rounded-full ${isWarning ? 'bg-yellow-100 text-yellow-600' : 'bg-blue-100 text-blue-600'}`}
        >
          {icon}
        </div>
      </div>
    </div>
  );
}

interface RateLimitBarProps {
  label: string;
  used: number;
  limit: number;
  resetAt: string;
}

function RateLimitBar({ label, used, limit, resetAt }: RateLimitBarProps) {
  const percentage = Math.min((used / limit) * 100, 100);
  const isHigh = percentage > 80;
  const isMedium = percentage > 50;

  const getBarColor = () => {
    if (isHigh) return 'bg-red-500';
    if (isMedium) return 'bg-yellow-500';
    return 'bg-green-500';
  };

  const resetTime = new Date(resetAt);
  const now = new Date();
  const diffMs = resetTime.getTime() - now.getTime();
  const diffMins = Math.max(0, Math.floor(diffMs / 60000));

  return (
    <div>
      <div className="flex justify-between text-sm mb-1">
        <span className="font-medium">{label}</span>
        <span className="text-muted-foreground">
          {used.toLocaleString()} / {limit.toLocaleString()}
        </span>
      </div>
      <div className="h-2 bg-gray-200 rounded-full overflow-hidden">
        <div
          className={`h-full ${getBarColor()} transition-all duration-300`}
          style={{ width: `${percentage}%` }}
        />
      </div>
      <p className="text-xs text-muted-foreground mt-1">
        Resets in {diffMins} minute{diffMins !== 1 ? 's' : ''}
      </p>
    </div>
  );
}

interface QuickActionCardProps {
  title: string;
  description: string;
  href: string;
  icon: React.ReactNode;
}

function QuickActionCard({ title, description, href, icon }: QuickActionCardProps) {
  return (
    <a
      href={href}
      className="block p-6 bg-white rounded-lg border shadow-sm hover:shadow-md transition-shadow"
    >
      <div className="flex items-start gap-4">
        <div className="p-3 bg-blue-100 text-blue-600 rounded-lg">{icon}</div>
        <div>
          <h3 className="font-semibold">{title}</h3>
          <p className="text-sm text-muted-foreground">{description}</p>
        </div>
      </div>
    </a>
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
  return num.toString();
}
