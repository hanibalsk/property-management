/**
 * Webhook Subscriptions List Component
 *
 * Displays and manages webhook subscriptions (Story 61.5).
 */

import type { WebhookSubscription } from '@ppt/api-client';
import {
  useDeleteWebhookSubscription,
  useTestWebhook,
  useWebhookStatistics,
  useWebhookSubscriptions,
} from '@ppt/api-client';
import { useState } from 'react';

interface WebhookSubscriptionsListProps {
  organizationId: string;
  onCreate?: () => void;
}

const statusColors: Record<string, string> = {
  active: 'bg-green-100 text-green-800',
  paused: 'bg-yellow-100 text-yellow-800',
  disabled: 'bg-gray-100 text-gray-800',
};

export function WebhookSubscriptionsList({
  organizationId,
  onCreate,
}: WebhookSubscriptionsListProps) {
  const { data: subscriptions, isLoading } = useWebhookSubscriptions(organizationId);
  const deleteSubscription = useDeleteWebhookSubscription(organizationId);
  const testWebhook = useTestWebhook();
  const [selectedWebhook, setSelectedWebhook] = useState<string | null>(null);

  const handleDelete = async (id: string) => {
    if (confirm('Are you sure you want to delete this webhook?')) {
      await deleteSubscription.mutateAsync(id);
    }
  };

  const handleTest = async (id: string) => {
    const result = await testWebhook.mutateAsync({
      id,
      data: {
        eventType: 'fault.created',
        payload: { test: true },
      },
    });
    alert(
      result.success
        ? `Test successful! Response time: ${result.responseTimeMs}ms`
        : `Test failed: ${result.error}`
    );
  };

  if (isLoading) {
    return (
      <div className="rounded-lg border bg-card p-6">
        <h3 className="text-lg font-semibold">Webhook Subscriptions</h3>
        <p className="text-muted-foreground">Loading...</p>
      </div>
    );
  }

  return (
    <div className="rounded-lg border bg-card p-6">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h3 className="text-lg font-semibold">Webhook Subscriptions</h3>
          <p className="text-sm text-muted-foreground">
            Receive real-time notifications when events occur
          </p>
        </div>
        <button
          type="button"
          onClick={onCreate}
          className="inline-flex items-center px-4 py-2 text-sm font-medium bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
        >
          + Create Webhook
        </button>
      </div>

      {subscriptions?.length === 0 ? (
        <div className="flex flex-col items-center justify-center py-8 text-center">
          <div className="text-4xl mb-4">webhook</div>
          <p className="text-muted-foreground">No webhooks configured</p>
          <p className="text-sm text-muted-foreground">
            Create a webhook to receive event notifications
          </p>
        </div>
      ) : (
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="py-3 px-4 text-left text-sm font-medium">Name</th>
                <th className="py-3 px-4 text-left text-sm font-medium">URL</th>
                <th className="py-3 px-4 text-left text-sm font-medium">Events</th>
                <th className="py-3 px-4 text-left text-sm font-medium">Status</th>
                <th className="py-3 px-4 text-left text-sm font-medium w-[150px]">Actions</th>
              </tr>
            </thead>
            <tbody>
              {subscriptions?.map((webhook: WebhookSubscription) => (
                <tr key={webhook.id} className="border-b">
                  <td className="py-3 px-4 font-medium">{webhook.name}</td>
                  <td className="py-3 px-4 max-w-[200px] truncate text-sm text-muted-foreground">
                    {webhook.url}
                  </td>
                  <td className="py-3 px-4">
                    <div className="flex flex-wrap gap-1">
                      {webhook.events.slice(0, 2).map((event) => (
                        <span
                          key={event}
                          className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-muted"
                        >
                          {event}
                        </span>
                      ))}
                      {webhook.events.length > 2 && (
                        <span className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-muted">
                          +{webhook.events.length - 2} more
                        </span>
                      )}
                    </div>
                  </td>
                  <td className="py-3 px-4">
                    <span
                      className={`inline-flex items-center px-2 py-0.5 rounded text-xs font-medium ${statusColors[webhook.status]}`}
                    >
                      {webhook.status}
                    </span>
                  </td>
                  <td className="py-3 px-4">
                    <div className="flex items-center gap-2">
                      <button
                        type="button"
                        onClick={() => handleTest(webhook.id)}
                        className="px-3 py-1 text-sm border rounded-md hover:bg-muted"
                      >
                        Test
                      </button>
                      <button
                        type="button"
                        onClick={() => setSelectedWebhook(webhook.id)}
                        className="px-3 py-1 text-sm border rounded-md hover:bg-muted"
                      >
                        Logs
                      </button>
                      <button
                        type="button"
                        onClick={() => handleDelete(webhook.id)}
                        className="px-3 py-1 text-sm text-red-600 border border-red-200 rounded-md hover:bg-red-50"
                      >
                        Delete
                      </button>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {selectedWebhook && (
        <WebhookStatsDialog webhookId={selectedWebhook} onClose={() => setSelectedWebhook(null)} />
      )}
    </div>
  );
}

function WebhookStatsDialog({
  webhookId,
  onClose,
}: {
  webhookId: string;
  onClose: () => void;
}) {
  const { data: stats, isLoading } = useWebhookStatistics(webhookId);

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="bg-background rounded-lg border shadow-lg w-full max-w-md p-6">
        <div className="mb-4">
          <h3 className="text-lg font-semibold">Webhook Statistics</h3>
          <p className="text-sm text-muted-foreground">Delivery statistics for this webhook</p>
        </div>
        {isLoading ? (
          <div className="py-8 text-center">Loading...</div>
        ) : (
          <div className="grid grid-cols-2 gap-4">
            <div className="rounded-lg border p-4">
              <div className="text-sm text-muted-foreground">Total Deliveries</div>
              <div className="text-2xl font-bold">{stats?.totalDeliveries}</div>
            </div>
            <div className="rounded-lg border p-4">
              <div className="text-sm text-muted-foreground">Success Rate</div>
              <div className="text-2xl font-bold">{stats?.successRate?.toFixed(1)}%</div>
            </div>
            <div className="rounded-lg border p-4">
              <div className="flex items-center gap-2">
                <span className="text-green-500">OK</span>
                <span className="text-sm text-muted-foreground">Successful</span>
              </div>
              <div className="text-xl font-bold">{stats?.successfulDeliveries}</div>
            </div>
            <div className="rounded-lg border p-4">
              <div className="flex items-center gap-2">
                <span className="text-red-500">ERR</span>
                <span className="text-sm text-muted-foreground">Failed</span>
              </div>
              <div className="text-xl font-bold">{stats?.failedDeliveries}</div>
            </div>
            <div className="col-span-2 rounded-lg border p-4">
              <div className="text-sm text-muted-foreground">Average Response Time</div>
              <div className="text-xl font-bold">
                {stats?.averageResponseTimeMs?.toFixed(0) ?? 'N/A'} ms
              </div>
            </div>
          </div>
        )}
        <div className="mt-4 flex justify-end">
          <button
            type="button"
            onClick={onClose}
            className="px-4 py-2 text-sm font-medium bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
}
