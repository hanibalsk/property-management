/**
 * Webhook Delivery Logs Component
 *
 * Displays detailed delivery logs for webhook subscriptions (Story 61.5).
 */

import type { WebhookDeliveryLog } from '@ppt/api-client';
import { useWebhookLogs } from '@ppt/api-client';

interface WebhookDeliveryLogsProps {
  webhookId: string;
  onClose?: () => void;
}

const statusColors: Record<string, string> = {
  pending: 'bg-yellow-100 text-yellow-800',
  delivered: 'bg-green-100 text-green-800',
  failed: 'bg-red-100 text-red-800',
  retrying: 'bg-blue-100 text-blue-800',
};

export function WebhookDeliveryLogs({ webhookId, onClose }: WebhookDeliveryLogsProps) {
  const { data: logs, isLoading } = useWebhookLogs(webhookId);

  const formatDateTime = (dateStr: string) => {
    const date = new Date(dateStr);
    return {
      date: date.toLocaleDateString(),
      time: date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' }),
    };
  };

  if (isLoading) {
    return (
      <div className="rounded-lg border bg-card p-6">
        <h3 className="text-lg font-semibold">Delivery Logs</h3>
        <p className="text-muted-foreground">Loading...</p>
      </div>
    );
  }

  return (
    <div className="rounded-lg border bg-card p-6">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h3 className="text-lg font-semibold">Delivery Logs</h3>
          <p className="text-sm text-muted-foreground">Recent webhook delivery attempts</p>
        </div>
        {onClose && (
          <button
            type="button"
            onClick={onClose}
            className="px-3 py-1 text-sm border rounded-md hover:bg-muted"
          >
            Back
          </button>
        )}
      </div>

      {logs?.length === 0 ? (
        <div className="flex flex-col items-center justify-center py-8 text-center">
          <div className="text-4xl mb-4">log</div>
          <p className="text-muted-foreground">No delivery logs yet</p>
          <p className="text-sm text-muted-foreground">
            Logs will appear here when webhook events are triggered
          </p>
        </div>
      ) : (
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="py-3 px-4 text-left text-sm font-medium">Event</th>
                <th className="py-3 px-4 text-left text-sm font-medium">Status</th>
                <th className="py-3 px-4 text-left text-sm font-medium">Response</th>
                <th className="py-3 px-4 text-left text-sm font-medium">Duration</th>
                <th className="py-3 px-4 text-left text-sm font-medium">Attempts</th>
                <th className="py-3 px-4 text-left text-sm font-medium">Timestamp</th>
              </tr>
            </thead>
            <tbody>
              {logs?.map((log: WebhookDeliveryLog) => {
                const { date, time } = formatDateTime(log.createdAt);
                return (
                  <tr key={log.id} className="border-b">
                    <td className="py-3 px-4">
                      <span className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-muted">
                        {log.eventType}
                      </span>
                    </td>
                    <td className="py-3 px-4">
                      <span
                        className={`inline-flex items-center px-2 py-0.5 rounded text-xs font-medium ${statusColors[log.status]}`}
                      >
                        {log.status}
                      </span>
                    </td>
                    <td className="py-3 px-4 text-sm">
                      {log.responseStatus ? (
                        <span
                          className={
                            log.responseStatus >= 200 && log.responseStatus < 300
                              ? 'text-green-600'
                              : 'text-red-600'
                          }
                        >
                          HTTP {log.responseStatus}
                        </span>
                      ) : (
                        <span className="text-muted-foreground">-</span>
                      )}
                    </td>
                    <td className="py-3 px-4 text-sm text-muted-foreground">
                      {log.durationMs ? `${log.durationMs}ms` : '-'}
                    </td>
                    <td className="py-3 px-4 text-sm">
                      <span className={log.attempts > 1 ? 'text-orange-600' : ''}>
                        {log.attempts}
                      </span>
                    </td>
                    <td className="py-3 px-4 text-sm text-muted-foreground">
                      <div>{date}</div>
                      <div className="text-xs">{time}</div>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      )}

      {logs && logs.length > 0 && (
        <div className="mt-4 pt-4 border-t">
          <h4 className="text-sm font-medium mb-2">Retry Policy</h4>
          <p className="text-sm text-muted-foreground">
            Failed deliveries are automatically retried with exponential backoff. Maximum 5 attempts
            with delays of 30s, 2m, 10m, 30m, and 2h.
          </p>
        </div>
      )}
    </div>
  );
}
