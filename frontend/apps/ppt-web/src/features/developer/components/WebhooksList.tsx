/**
 * Webhooks List Component (Epic 69 - Story 69.3)
 *
 * Displays a list of webhook subscriptions with actions to manage them.
 */

import { useState } from 'react';
import type { WebhookEventType, WebhookSubscription } from '../types';

interface WebhooksListProps {
  webhooks: WebhookSubscription[];
  onCreateWebhook: () => void;
  onEditWebhook: (webhookId: string) => void;
  onDeleteWebhook: (webhookId: string) => void;
  onTestWebhook: (webhookId: string) => void;
  onViewDeliveries: (webhookId: string) => void;
  onRotateSecret: (webhookId: string) => void;
  onToggleActive: (webhookId: string, isActive: boolean) => void;
}

export function WebhooksList({
  webhooks,
  onCreateWebhook,
  onEditWebhook,
  onDeleteWebhook,
  onTestWebhook,
  onViewDeliveries,
  onRotateSecret,
  onToggleActive,
}: WebhooksListProps) {
  const [showDeleteConfirm, setShowDeleteConfirm] = useState<string | null>(null);
  const [filter, setFilter] = useState<'all' | 'active' | 'inactive'>('all');

  const filteredWebhooks = webhooks.filter((wh) => {
    if (filter === 'all') return true;
    return filter === 'active' ? wh.isActive : !wh.isActive;
  });

  const activeCount = webhooks.filter((wh) => wh.isActive).length;
  const inactiveCount = webhooks.filter((wh) => !wh.isActive).length;

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold tracking-tight">Webhooks</h2>
          <p className="text-muted-foreground">
            Subscribe to events and receive real-time notifications
          </p>
        </div>
        <button
          type="button"
          onClick={onCreateWebhook}
          className="inline-flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
        >
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
          </svg>
          Add Webhook
        </button>
      </div>

      {/* Stats */}
      <div className="flex gap-4">
        <div className="px-4 py-2 bg-green-100 text-green-800 rounded-lg">
          <span className="font-semibold">{activeCount}</span> active
        </div>
        <div className="px-4 py-2 bg-gray-100 text-gray-800 rounded-lg">
          <span className="font-semibold">{inactiveCount}</span> inactive
        </div>
      </div>

      {/* Filter */}
      <div className="flex gap-2">
        <FilterButton active={filter === 'all'} onClick={() => setFilter('all')}>
          All
        </FilterButton>
        <FilterButton active={filter === 'active'} onClick={() => setFilter('active')}>
          Active
        </FilterButton>
        <FilterButton active={filter === 'inactive'} onClick={() => setFilter('inactive')}>
          Inactive
        </FilterButton>
      </div>

      {/* Webhooks List */}
      {filteredWebhooks.length === 0 ? (
        <div className="p-8 text-center bg-gray-50 rounded-lg border-2 border-dashed">
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
              d="M13 10V3L4 14h7v7l9-11h-7z"
            />
          </svg>
          <h3 className="text-lg font-medium mb-1">No webhooks found</h3>
          <p className="text-muted-foreground mb-4">
            Create your first webhook to start receiving event notifications.
          </p>
          <button
            type="button"
            onClick={onCreateWebhook}
            className="inline-flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
          >
            Add Webhook
          </button>
        </div>
      ) : (
        <div className="space-y-4">
          {filteredWebhooks.map((webhook) => (
            <WebhookCard
              key={webhook.id}
              webhook={webhook}
              onEdit={() => onEditWebhook(webhook.id)}
              onDelete={() => setShowDeleteConfirm(webhook.id)}
              onTest={() => onTestWebhook(webhook.id)}
              onViewDeliveries={() => onViewDeliveries(webhook.id)}
              onRotateSecret={() => onRotateSecret(webhook.id)}
              onToggleActive={(active) => onToggleActive(webhook.id, active)}
            />
          ))}
        </div>
      )}

      {/* Delete Confirmation Modal */}
      {showDeleteConfirm && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
          <div className="bg-white rounded-lg p-6 max-w-md w-full mx-4">
            <h3 className="text-lg font-semibold mb-2">Delete Webhook?</h3>
            <p className="text-muted-foreground mb-4">
              This will permanently delete the webhook subscription. You will no longer receive
              event notifications at this endpoint.
            </p>
            <div className="flex justify-end gap-2">
              <button
                type="button"
                onClick={() => setShowDeleteConfirm(null)}
                className="px-4 py-2 border rounded-md hover:bg-gray-50 transition-colors"
              >
                Cancel
              </button>
              <button
                type="button"
                onClick={() => {
                  onDeleteWebhook(showDeleteConfirm);
                  setShowDeleteConfirm(null);
                }}
                className="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 transition-colors"
              >
                Delete
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

// ==================== Helper Components ====================

interface FilterButtonProps {
  active: boolean;
  onClick: () => void;
  children: React.ReactNode;
}

function FilterButton({ active, onClick, children }: FilterButtonProps) {
  return (
    <button
      type="button"
      onClick={onClick}
      className={`px-3 py-1 text-sm rounded-md transition-colors ${
        active ? 'bg-blue-600 text-white' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
      }`}
    >
      {children}
    </button>
  );
}

interface WebhookCardProps {
  webhook: WebhookSubscription;
  onEdit: () => void;
  onDelete: () => void;
  onTest: () => void;
  onViewDeliveries: () => void;
  onRotateSecret: () => void;
  onToggleActive: (active: boolean) => void;
}

function WebhookCard({
  webhook,
  onEdit,
  onDelete,
  onTest,
  onViewDeliveries,
  onRotateSecret,
  onToggleActive,
}: WebhookCardProps) {
  const successRate =
    webhook.totalDeliveries > 0
      ? (webhook.successfulDeliveries / webhook.totalDeliveries) * 100
      : 100;

  return (
    <div className="p-4 bg-white rounded-lg border shadow-sm">
      <div className="flex items-start justify-between gap-4">
        <div className="flex-1 min-w-0">
          {/* Header */}
          <div className="flex items-center gap-2 mb-2">
            <h3 className="font-semibold truncate">{webhook.name}</h3>
            <span
              className={`px-2 py-0.5 text-xs font-medium rounded-full ${
                webhook.isActive ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'
              }`}
            >
              {webhook.isActive ? 'Active' : 'Inactive'}
            </span>
          </div>

          {/* Endpoint URL */}
          <p className="text-sm font-mono text-muted-foreground truncate mb-2">
            {webhook.endpointUrl}
          </p>

          {/* Event Types */}
          <div className="flex flex-wrap gap-1 mb-3">
            {webhook.eventTypes.slice(0, 5).map((event) => (
              <EventBadge key={event} event={event} />
            ))}
            {webhook.eventTypes.length > 5 && (
              <span className="px-2 py-0.5 text-xs bg-gray-100 text-gray-600 rounded">
                +{webhook.eventTypes.length - 5} more
              </span>
            )}
          </div>

          {/* Stats */}
          <div className="flex items-center gap-4 text-xs text-muted-foreground">
            <span>Deliveries: {webhook.totalDeliveries.toLocaleString()}</span>
            <span className={successRate >= 95 ? 'text-green-600' : 'text-yellow-600'}>
              Success: {successRate.toFixed(1)}%
            </span>
            {webhook.lastTriggeredAt && (
              <span>Last triggered: {formatRelativeTime(webhook.lastTriggeredAt)}</span>
            )}
          </div>
        </div>

        {/* Actions */}
        <div className="flex items-center gap-1">
          {/* Toggle Active */}
          <button
            type="button"
            onClick={() => onToggleActive(!webhook.isActive)}
            className={`p-2 rounded-md transition-colors ${
              webhook.isActive
                ? 'text-green-600 hover:bg-green-50'
                : 'text-gray-400 hover:bg-gray-100'
            }`}
            title={webhook.isActive ? 'Disable' : 'Enable'}
          >
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d={
                  webhook.isActive
                    ? 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z'
                    : 'M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z'
                }
              />
            </svg>
          </button>

          {/* Test */}
          <button
            type="button"
            onClick={onTest}
            className="p-2 text-blue-600 hover:bg-blue-50 rounded-md transition-colors"
            title="Test Webhook"
          >
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"
              />
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
          </button>

          {/* View Deliveries */}
          <button
            type="button"
            onClick={onViewDeliveries}
            className="p-2 text-gray-600 hover:bg-gray-100 rounded-md transition-colors"
            title="View Deliveries"
          >
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
              />
            </svg>
          </button>

          {/* Edit */}
          <button
            type="button"
            onClick={onEdit}
            className="p-2 text-gray-600 hover:bg-gray-100 rounded-md transition-colors"
            title="Edit"
          >
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
              />
            </svg>
          </button>

          {/* Rotate Secret */}
          <button
            type="button"
            onClick={onRotateSecret}
            className="p-2 text-yellow-600 hover:bg-yellow-50 rounded-md transition-colors"
            title="Rotate Secret"
          >
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
              />
            </svg>
          </button>

          {/* Delete */}
          <button
            type="button"
            onClick={onDelete}
            className="p-2 text-red-600 hover:bg-red-50 rounded-md transition-colors"
            title="Delete"
          >
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
              />
            </svg>
          </button>
        </div>
      </div>
    </div>
  );
}

function EventBadge({ event }: { event: WebhookEventType }) {
  const getEventColor = (eventType: string): string => {
    if (eventType.startsWith('fault')) return 'bg-red-100 text-red-800';
    if (eventType.startsWith('payment')) return 'bg-green-100 text-green-800';
    if (eventType.startsWith('resident')) return 'bg-blue-100 text-blue-800';
    if (eventType.startsWith('vote')) return 'bg-purple-100 text-purple-800';
    if (eventType.startsWith('document')) return 'bg-yellow-100 text-yellow-800';
    if (eventType.startsWith('work_order')) return 'bg-orange-100 text-orange-800';
    return 'bg-gray-100 text-gray-800';
  };

  return (
    <span className={`px-2 py-0.5 text-xs font-medium rounded ${getEventColor(event)}`}>
      {event}
    </span>
  );
}

function formatRelativeTime(dateString: string): string {
  const date = new Date(dateString);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMs / 3600000);
  const diffDays = Math.floor(diffMs / 86400000);

  if (diffMins < 1) return 'just now';
  if (diffMins < 60) return `${diffMins}m ago`;
  if (diffHours < 24) return `${diffHours}h ago`;
  if (diffDays < 7) return `${diffDays}d ago`;

  return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
}
