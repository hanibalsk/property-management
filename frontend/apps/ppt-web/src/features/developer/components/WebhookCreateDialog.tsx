/**
 * Webhook Create Dialog Component (Epic 69 - Story 69.3)
 *
 * Dialog for creating new webhook subscriptions with event type selection.
 */

import { useState } from 'react';
import type { CreateWebhookSubscription, WebhookEventType } from '../types';

interface WebhookCreateDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSubmit: (data: CreateWebhookSubscription) => Promise<void>;
  isLoading?: boolean;
}

const AVAILABLE_EVENTS: { value: WebhookEventType; label: string; category: string }[] = [
  { value: 'fault.created', label: 'Fault Created', category: 'Faults' },
  { value: 'fault.updated', label: 'Fault Updated', category: 'Faults' },
  { value: 'fault.resolved', label: 'Fault Resolved', category: 'Faults' },
  { value: 'payment.received', label: 'Payment Received', category: 'Payments' },
  { value: 'payment.overdue', label: 'Payment Overdue', category: 'Payments' },
  { value: 'resident.moved_in', label: 'Resident Moved In', category: 'Residents' },
  { value: 'resident.moved_out', label: 'Resident Moved Out', category: 'Residents' },
  { value: 'vote.started', label: 'Vote Started', category: 'Voting' },
  { value: 'vote.ended', label: 'Vote Ended', category: 'Voting' },
  { value: 'announcement.published', label: 'Announcement Published', category: 'Announcements' },
  { value: 'document.uploaded', label: 'Document Uploaded', category: 'Documents' },
  { value: 'work_order.created', label: 'Work Order Created', category: 'Work Orders' },
  { value: 'work_order.completed', label: 'Work Order Completed', category: 'Work Orders' },
];

const CATEGORIES = [...new Set(AVAILABLE_EVENTS.map((e) => e.category))];

export function WebhookCreateDialog({
  isOpen,
  onClose,
  onSubmit,
  isLoading,
}: WebhookCreateDialogProps) {
  const [name, setName] = useState('');
  const [endpointUrl, setEndpointUrl] = useState('');
  const [selectedEvents, setSelectedEvents] = useState<WebhookEventType[]>([]);
  const [retryCount, setRetryCount] = useState('3');
  const [timeoutSeconds, setTimeoutSeconds] = useState('30');
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleEventToggle = (event: WebhookEventType) => {
    setSelectedEvents((prev) =>
      prev.includes(event) ? prev.filter((e) => e !== event) : [...prev, event]
    );
  };

  const handleCategoryToggle = (category: string) => {
    const categoryEvents = AVAILABLE_EVENTS.filter((e) => e.category === category).map(
      (e) => e.value
    );
    const allSelected = categoryEvents.every((e) => selectedEvents.includes(e));

    if (allSelected) {
      setSelectedEvents((prev) => prev.filter((e) => !categoryEvents.includes(e)));
    } else {
      setSelectedEvents((prev) => [...new Set([...prev, ...categoryEvents])]);
    }
  };

  const handleSelectAll = () => {
    if (selectedEvents.length === AVAILABLE_EVENTS.length) {
      setSelectedEvents([]);
    } else {
      setSelectedEvents(AVAILABLE_EVENTS.map((e) => e.value));
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    if (!name.trim()) {
      setError('Please enter a name for the webhook');
      return;
    }

    if (!endpointUrl.trim()) {
      setError('Please enter an endpoint URL');
      return;
    }

    try {
      new URL(endpointUrl);
    } catch {
      setError('Please enter a valid URL');
      return;
    }

    if (!endpointUrl.startsWith('https://')) {
      setError('Endpoint URL must use HTTPS');
      return;
    }

    if (selectedEvents.length === 0) {
      setError('Please select at least one event type');
      return;
    }

    const data: CreateWebhookSubscription = {
      name: name.trim(),
      endpointUrl: endpointUrl.trim(),
      eventTypes: selectedEvents,
      retryCount: parseInt(retryCount, 10),
      timeoutSeconds: parseInt(timeoutSeconds, 10),
    };

    try {
      await onSubmit(data);
      resetForm();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create webhook');
    }
  };

  const resetForm = () => {
    setName('');
    setEndpointUrl('');
    setSelectedEvents([]);
    setRetryCount('3');
    setTimeoutSeconds('30');
    setShowAdvanced(false);
    setError(null);
  };

  const handleClose = () => {
    resetForm();
    onClose();
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="bg-white rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto">
        <div className="p-6 border-b">
          <h2 className="text-xl font-semibold">Create Webhook</h2>
          <p className="text-sm text-muted-foreground">
            Subscribe to events and receive notifications at your endpoint
          </p>
        </div>

        <form onSubmit={handleSubmit} className="p-6 space-y-6">
          {error && (
            <div className="p-3 text-sm text-red-600 bg-red-50 rounded-md">{error}</div>
          )}

          {/* Name */}
          <div>
            <label htmlFor="name" className="block text-sm font-medium mb-1">
              Name <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              id="name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g., Production Webhook"
              className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          {/* Endpoint URL */}
          <div>
            <label htmlFor="endpointUrl" className="block text-sm font-medium mb-1">
              Endpoint URL <span className="text-red-500">*</span>
            </label>
            <input
              type="url"
              id="endpointUrl"
              value={endpointUrl}
              onChange={(e) => setEndpointUrl(e.target.value)}
              placeholder="https://your-domain.com/webhooks/ppt"
              className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <p className="mt-1 text-xs text-muted-foreground">
              Must be a valid HTTPS URL that can receive POST requests
            </p>
          </div>

          {/* Event Types */}
          <div>
            <div className="flex items-center justify-between mb-2">
              <label className="block text-sm font-medium">
                Event Types <span className="text-red-500">*</span>
              </label>
              <button
                type="button"
                onClick={handleSelectAll}
                className="text-xs text-blue-600 hover:underline"
              >
                {selectedEvents.length === AVAILABLE_EVENTS.length ? 'Deselect All' : 'Select All'}
              </button>
            </div>

            <div className="space-y-4 max-h-64 overflow-y-auto p-1">
              {CATEGORIES.map((category) => {
                const categoryEvents = AVAILABLE_EVENTS.filter((e) => e.category === category);
                const allSelected = categoryEvents.every((e) =>
                  selectedEvents.includes(e.value)
                );
                const someSelected = categoryEvents.some((e) =>
                  selectedEvents.includes(e.value)
                );

                return (
                  <div key={category} className="border rounded-md p-3">
                    <label className="flex items-center gap-2 mb-2 cursor-pointer">
                      <input
                        type="checkbox"
                        checked={allSelected}
                        ref={(el) => {
                          if (el) {
                            el.indeterminate = someSelected && !allSelected;
                          }
                        }}
                        onChange={() => handleCategoryToggle(category)}
                        className="rounded"
                      />
                      <span className="font-medium text-sm">{category}</span>
                      <span className="text-xs text-muted-foreground">
                        ({categoryEvents.length} events)
                      </span>
                    </label>
                    <div className="grid grid-cols-2 gap-2 pl-6">
                      {categoryEvents.map((event) => (
                        <label
                          key={event.value}
                          className="flex items-center gap-2 text-sm cursor-pointer"
                        >
                          <input
                            type="checkbox"
                            checked={selectedEvents.includes(event.value)}
                            onChange={() => handleEventToggle(event.value)}
                            className="rounded"
                          />
                          <span>{event.label}</span>
                        </label>
                      ))}
                    </div>
                  </div>
                );
              })}
            </div>
            <p className="mt-2 text-xs text-muted-foreground">
              {selectedEvents.length} event{selectedEvents.length !== 1 ? 's' : ''} selected
            </p>
          </div>

          {/* Advanced Options Toggle */}
          <button
            type="button"
            onClick={() => setShowAdvanced(!showAdvanced)}
            className="flex items-center gap-1 text-sm text-blue-600 hover:underline"
          >
            <svg
              className={`w-4 h-4 transition-transform ${showAdvanced ? 'rotate-90' : ''}`}
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9 5l7 7-7 7"
              />
            </svg>
            Advanced Options
          </button>

          {showAdvanced && (
            <div className="grid grid-cols-2 gap-4 p-4 bg-gray-50 rounded-md">
              <div>
                <label htmlFor="retryCount" className="block text-sm font-medium mb-1">
                  Retry Count
                </label>
                <select
                  id="retryCount"
                  value={retryCount}
                  onChange={(e) => setRetryCount(e.target.value)}
                  className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="0">No retries</option>
                  <option value="1">1 retry</option>
                  <option value="3">3 retries</option>
                  <option value="5">5 retries</option>
                </select>
                <p className="mt-1 text-xs text-muted-foreground">
                  Number of retry attempts for failed deliveries
                </p>
              </div>

              <div>
                <label htmlFor="timeoutSeconds" className="block text-sm font-medium mb-1">
                  Timeout (seconds)
                </label>
                <select
                  id="timeoutSeconds"
                  value={timeoutSeconds}
                  onChange={(e) => setTimeoutSeconds(e.target.value)}
                  className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="5">5 seconds</option>
                  <option value="10">10 seconds</option>
                  <option value="30">30 seconds</option>
                  <option value="60">60 seconds</option>
                </select>
                <p className="mt-1 text-xs text-muted-foreground">
                  Maximum time to wait for a response
                </p>
              </div>
            </div>
          )}

          {/* Actions */}
          <div className="flex justify-end gap-3 pt-4 border-t">
            <button
              type="button"
              onClick={handleClose}
              className="px-4 py-2 border rounded-md hover:bg-gray-50 transition-colors"
              disabled={isLoading}
            >
              Cancel
            </button>
            <button
              type="submit"
              className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors disabled:opacity-50"
              disabled={isLoading}
            >
              {isLoading ? 'Creating...' : 'Create Webhook'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
