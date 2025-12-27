/**
 * Webhook Delivery Logs Component (Epic 69 - Story 69.3)
 *
 * Displays webhook delivery history with status and details.
 */

import { useState } from 'react';
import type { WebhookDelivery, WebhookDeliveryStatus } from '../types';

interface WebhookDeliveryLogsProps {
  deliveries: WebhookDelivery[];
  webhookName: string;
  onRetry?: (deliveryId: string) => void;
  isLoading?: boolean;
}

export function WebhookDeliveryLogs({
  deliveries,
  webhookName,
  onRetry,
  isLoading,
}: WebhookDeliveryLogsProps) {
  const [expandedDelivery, setExpandedDelivery] = useState<string | null>(null);
  const [filter, setFilter] = useState<WebhookDeliveryStatus | 'all'>('all');

  const filteredDeliveries = deliveries.filter((d) => {
    if (filter === 'all') return true;
    return d.status === filter;
  });

  if (isLoading) {
    return (
      <div className="p-8 text-center">
        <div className="animate-spin w-8 h-8 border-4 border-blue-600 border-t-transparent rounded-full mx-auto" />
        <p className="mt-4 text-muted-foreground">Loading deliveries...</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-semibold">Delivery Logs</h3>
        <p className="text-sm text-muted-foreground">Delivery history for {webhookName}</p>
      </div>

      {/* Filter */}
      <div className="flex gap-2">
        <FilterButton active={filter === 'all'} onClick={() => setFilter('all')}>
          All
        </FilterButton>
        <FilterButton active={filter === 'delivered'} onClick={() => setFilter('delivered')}>
          Delivered
        </FilterButton>
        <FilterButton active={filter === 'failed'} onClick={() => setFilter('failed')}>
          Failed
        </FilterButton>
        <FilterButton active={filter === 'retrying'} onClick={() => setFilter('retrying')}>
          Retrying
        </FilterButton>
      </div>

      {/* Deliveries List */}
      {filteredDeliveries.length === 0 ? (
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
              d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
            />
          </svg>
          <h3 className="text-lg font-medium mb-1">No deliveries found</h3>
          <p className="text-muted-foreground">
            Webhook deliveries will appear here when events are triggered.
          </p>
        </div>
      ) : (
        <div className="space-y-2">
          {filteredDeliveries.map((delivery) => (
            <DeliveryItem
              key={delivery.id}
              delivery={delivery}
              isExpanded={expandedDelivery === delivery.id}
              onToggle={() =>
                setExpandedDelivery(expandedDelivery === delivery.id ? null : delivery.id)
              }
              onRetry={onRetry ? () => onRetry(delivery.id) : undefined}
            />
          ))}
        </div>
      )}
    </div>
  );
}

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

interface DeliveryItemProps {
  delivery: WebhookDelivery;
  isExpanded: boolean;
  onToggle: () => void;
  onRetry?: () => void;
}

function DeliveryItem({ delivery, isExpanded, onToggle, onRetry }: DeliveryItemProps) {
  const getStatusColor = (status: WebhookDeliveryStatus) => {
    switch (status) {
      case 'delivered':
        return 'bg-green-100 text-green-800';
      case 'failed':
        return 'bg-red-100 text-red-800';
      case 'retrying':
        return 'bg-yellow-100 text-yellow-800';
      case 'pending':
        return 'bg-blue-100 text-blue-800';
      case 'exhausted':
        return 'bg-gray-100 text-gray-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const getStatusIcon = (status: WebhookDeliveryStatus) => {
    switch (status) {
      case 'delivered':
        return (
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
          </svg>
        );
      case 'failed':
      case 'exhausted':
        return (
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        );
      case 'retrying':
        return (
          <svg
            className="w-4 h-4 animate-spin"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
            />
          </svg>
        );
      default:
        return (
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
        );
    }
  };

  return (
    <div className="border rounded-lg overflow-hidden">
      <button
        type="button"
        onClick={onToggle}
        className="w-full px-4 py-3 flex items-center gap-4 text-left hover:bg-gray-50 transition-colors"
      >
        <span
          className={`inline-flex items-center gap-1 px-2 py-1 text-xs font-medium rounded ${getStatusColor(delivery.status)}`}
        >
          {getStatusIcon(delivery.status)}
          {delivery.status}
        </span>
        <span className="text-sm font-mono">{delivery.eventType}</span>
        <span className="text-sm text-muted-foreground flex-1">
          Attempt {delivery.attemptNumber}
        </span>
        {delivery.responseTimeMs && (
          <span className="text-sm text-muted-foreground">{delivery.responseTimeMs}ms</span>
        )}
        {delivery.responseStatusCode && (
          <span
            className={`text-sm font-mono ${
              delivery.responseStatusCode >= 200 && delivery.responseStatusCode < 300
                ? 'text-green-600'
                : 'text-red-600'
            }`}
          >
            {delivery.responseStatusCode}
          </span>
        )}
        <span className="text-xs text-muted-foreground">
          {formatRelativeTime(delivery.createdAt)}
        </span>
        <svg
          className={`w-5 h-5 text-gray-400 transition-transform ${isExpanded ? 'rotate-180' : ''}`}
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>

      {isExpanded && (
        <div className="px-4 pb-4 space-y-4 bg-gray-50">
          {/* Details */}
          <div className="grid grid-cols-2 gap-4 text-sm pt-3">
            <div>
              <span className="text-muted-foreground">Event ID:</span>
              <span className="ml-2 font-mono text-xs">{delivery.eventId}</span>
            </div>
            <div>
              <span className="text-muted-foreground">Delivery ID:</span>
              <span className="ml-2 font-mono text-xs">{delivery.id}</span>
            </div>
            {delivery.deliveredAt && (
              <div>
                <span className="text-muted-foreground">Delivered at:</span>
                <span className="ml-2">{formatDateTime(delivery.deliveredAt)}</span>
              </div>
            )}
            {delivery.nextRetryAt && (
              <div>
                <span className="text-muted-foreground">Next retry:</span>
                <span className="ml-2">{formatDateTime(delivery.nextRetryAt)}</span>
              </div>
            )}
          </div>

          {/* Error Message */}
          {delivery.errorMessage && (
            <div className="p-3 bg-red-50 rounded-md">
              <h4 className="text-sm font-medium text-red-800 mb-1">Error</h4>
              <p className="text-sm text-red-700">{delivery.errorMessage}</p>
            </div>
          )}

          {/* Payload */}
          <div>
            <h4 className="text-sm font-medium mb-2">Payload</h4>
            <pre className="p-3 bg-gray-900 text-gray-100 rounded-md text-xs overflow-x-auto max-h-48">
              <code>{JSON.stringify(delivery.payload, null, 2)}</code>
            </pre>
          </div>

          {/* Response Body */}
          {delivery.responseBody && (
            <div>
              <h4 className="text-sm font-medium mb-2">Response</h4>
              <pre className="p-3 bg-gray-900 text-gray-100 rounded-md text-xs overflow-x-auto max-h-32">
                <code>{delivery.responseBody}</code>
              </pre>
            </div>
          )}

          {/* Actions */}
          {(delivery.status === 'failed' || delivery.status === 'exhausted') && onRetry && (
            <div className="pt-2">
              <button
                type="button"
                onClick={onRetry}
                className="px-4 py-2 bg-blue-600 text-white text-sm rounded-md hover:bg-blue-700 transition-colors"
              >
                Retry Delivery
              </button>
            </div>
          )}
        </div>
      )}
    </div>
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

function formatDateTime(dateString: string): string {
  return new Date(dateString).toLocaleString('en-US', {
    month: 'short',
    day: 'numeric',
    hour: 'numeric',
    minute: '2-digit',
  });
}

// Stub for test dialog
export function WebhookTestDialog() {
  return <div>Webhook Test Dialog</div>;
}
