/**
 * Integrations Dashboard Component
 *
 * Displays overview of all external integrations (Epic 61).
 */

import { useIntegrationStatistics } from '@ppt/api-client';

interface IntegrationsDashboardProps {
  organizationId: string;
}

export function IntegrationsDashboard({ organizationId }: IntegrationsDashboardProps) {
  const { data: stats, isLoading } = useIntegrationStatistics(organizationId);

  if (isLoading) {
    return (
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-5">
        {['calendar', 'accounting', 'esignature', 'video', 'webhooks'].map((key) => (
          <div key={key} className="rounded-lg border bg-card p-4">
            <div className="text-sm font-medium text-muted-foreground">Loading...</div>
            <div className="text-2xl font-bold">--</div>
          </div>
        ))}
      </div>
    );
  }

  const integrationCards = [
    {
      title: 'Calendar Sync',
      value: stats?.calendarConnections ?? 0,
      subtitle: `${stats?.activeCalendarSyncs ?? 0} active`,
      icon: 'cal',
      color: 'text-blue-500',
    },
    {
      title: 'Accounting Exports',
      value: stats?.accountingExportsThisMonth ?? 0,
      subtitle: 'this month',
      icon: 'sheet',
      color: 'text-green-500',
    },
    {
      title: 'E-Signatures',
      value: stats?.esignatureWorkflowsPending ?? 0,
      subtitle: `${stats?.esignatureWorkflowsCompleted ?? 0} completed`,
      icon: 'pen',
      color: 'text-purple-500',
    },
    {
      title: 'Video Meetings',
      value: stats?.videoMeetingsScheduled ?? 0,
      subtitle: 'scheduled',
      icon: 'video',
      color: 'text-orange-500',
    },
    {
      title: 'Webhooks',
      value: stats?.webhookSubscriptions ?? 0,
      subtitle: `${stats?.webhookSuccessRate?.toFixed(1) ?? 0}% success rate`,
      icon: 'hook',
      color: 'text-cyan-500',
    },
  ];

  return (
    <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-5">
      {integrationCards.map((card) => (
        <div key={card.title} className="rounded-lg border bg-card p-4">
          <div className="flex items-center justify-between pb-2">
            <span className="text-sm font-medium">{card.title}</span>
            <span className={card.color}>{card.icon}</span>
          </div>
          <div className="text-2xl font-bold">{card.value}</div>
          <p className="text-xs text-muted-foreground">{card.subtitle}</p>
        </div>
      ))}
    </div>
  );
}
