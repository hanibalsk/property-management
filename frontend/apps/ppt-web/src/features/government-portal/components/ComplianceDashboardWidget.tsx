/**
 * ComplianceDashboardWidget - Statistics overview widget for compliance dashboard.
 * Epic 41: Government Portal UI (Story 41.3)
 */

import type { GovernmentPortalStats } from '@ppt/api-client';

interface ComplianceDashboardWidgetProps {
  stats: GovernmentPortalStats;
}

interface StatCardProps {
  label: string;
  value: number;
  variant?: 'default' | 'warning' | 'danger' | 'success';
  icon: React.ReactNode;
}

function StatCard({ label, value, variant = 'default', icon }: StatCardProps) {
  const variantStyles = {
    default: 'bg-gray-50 border-gray-200 text-gray-900',
    warning: 'bg-amber-50 border-amber-200 text-amber-900',
    danger: 'bg-red-50 border-red-200 text-red-900',
    success: 'bg-green-50 border-green-200 text-green-900',
  };

  const iconStyles = {
    default: 'text-gray-500',
    warning: 'text-amber-500',
    danger: 'text-red-500',
    success: 'text-green-500',
  };

  return (
    <div className={`rounded-lg border p-4 ${variantStyles[variant]}`}>
      <div className="flex items-center gap-3">
        <div className={`${iconStyles[variant]}`}>{icon}</div>
        <div>
          <p className="text-2xl font-bold">{value}</p>
          <p className="text-sm opacity-75">{label}</p>
        </div>
      </div>
    </div>
  );
}

export function ComplianceDashboardWidget({ stats }: ComplianceDashboardWidgetProps) {
  return (
    <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
      <StatCard
        label="Total Connections"
        value={stats.totalConnections}
        icon={
          <svg className="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"
            />
          </svg>
        }
      />

      <StatCard
        label="Active Connections"
        value={stats.activeConnections}
        variant="success"
        icon={
          <svg className="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
        }
      />

      <StatCard
        label="Total Submissions"
        value={stats.totalSubmissions}
        icon={
          <svg className="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
            />
          </svg>
        }
      />

      <StatCard
        label="This Month"
        value={stats.submissionsThisMonth}
        icon={
          <svg className="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
            />
          </svg>
        }
      />

      <StatCard
        label="Pending Submissions"
        value={stats.pendingSubmissions}
        variant={stats.pendingSubmissions > 0 ? 'warning' : 'default'}
        icon={
          <svg className="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
        }
      />

      <StatCard
        label="Rejected"
        value={stats.rejectedSubmissions}
        variant={stats.rejectedSubmissions > 0 ? 'danger' : 'default'}
        icon={
          <svg className="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
        }
      />
    </div>
  );
}
