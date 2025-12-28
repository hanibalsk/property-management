/**
 * Webhook Test Dialog Component (Epic 69 - Story 69.3)
 *
 * Dialog for testing webhook endpoints by sending a test payload.
 */

import { useState } from 'react';
import type {
  TestWebhookRequest,
  TestWebhookResponse,
  WebhookEventType,
  WebhookSubscription,
} from '../types';

interface WebhookTestDialogProps {
  isOpen: boolean;
  onClose: () => void;
  webhook: WebhookSubscription;
  onTest: (request: TestWebhookRequest) => Promise<TestWebhookResponse>;
}

const EVENT_TYPE_LABELS: Record<WebhookEventType, string> = {
  'fault.created': 'Fault Created',
  'fault.updated': 'Fault Updated',
  'fault.resolved': 'Fault Resolved',
  'payment.received': 'Payment Received',
  'payment.overdue': 'Payment Overdue',
  'resident.moved_in': 'Resident Moved In',
  'resident.moved_out': 'Resident Moved Out',
  'vote.started': 'Vote Started',
  'vote.ended': 'Vote Ended',
  'announcement.published': 'Announcement Published',
  'document.uploaded': 'Document Uploaded',
  'work_order.created': 'Work Order Created',
  'work_order.completed': 'Work Order Completed',
};

const SAMPLE_PAYLOADS: Record<WebhookEventType, Record<string, unknown>> = {
  'fault.created': {
    id: 'fault_test_123',
    title: 'Test Fault Report',
    description: 'This is a test fault report',
    priority: 'medium',
    status: 'open',
    building_id: 'building_456',
    unit_id: 'unit_789',
    created_at: new Date().toISOString(),
  },
  'fault.updated': {
    id: 'fault_test_123',
    title: 'Test Fault Report',
    status: 'in_progress',
    updated_at: new Date().toISOString(),
  },
  'fault.resolved': {
    id: 'fault_test_123',
    title: 'Test Fault Report',
    status: 'resolved',
    resolved_at: new Date().toISOString(),
    resolution_notes: 'Issue has been fixed',
  },
  'payment.received': {
    id: 'payment_test_123',
    amount: 500.0,
    currency: 'EUR',
    unit_id: 'unit_789',
    resident_id: 'resident_456',
    received_at: new Date().toISOString(),
  },
  'payment.overdue': {
    id: 'payment_test_123',
    amount: 500.0,
    currency: 'EUR',
    unit_id: 'unit_789',
    due_date: new Date().toISOString(),
    days_overdue: 15,
  },
  'resident.moved_in': {
    id: 'resident_test_123',
    name: 'Test Resident',
    unit_id: 'unit_789',
    move_in_date: new Date().toISOString(),
  },
  'resident.moved_out': {
    id: 'resident_test_123',
    name: 'Test Resident',
    unit_id: 'unit_789',
    move_out_date: new Date().toISOString(),
  },
  'vote.started': {
    id: 'vote_test_123',
    title: 'Test Vote',
    description: 'This is a test vote',
    start_date: new Date().toISOString(),
    end_date: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000).toISOString(),
  },
  'vote.ended': {
    id: 'vote_test_123',
    title: 'Test Vote',
    result: 'approved',
    yes_votes: 15,
    no_votes: 5,
    abstain_votes: 2,
    ended_at: new Date().toISOString(),
  },
  'announcement.published': {
    id: 'announcement_test_123',
    title: 'Test Announcement',
    content: 'This is a test announcement',
    author_id: 'user_456',
    published_at: new Date().toISOString(),
  },
  'document.uploaded': {
    id: 'document_test_123',
    filename: 'test_document.pdf',
    file_type: 'application/pdf',
    file_size: 1024000,
    uploaded_by: 'user_456',
    uploaded_at: new Date().toISOString(),
  },
  'work_order.created': {
    id: 'work_order_test_123',
    title: 'Test Work Order',
    description: 'This is a test work order',
    priority: 'high',
    fault_id: 'fault_test_123',
    created_at: new Date().toISOString(),
  },
  'work_order.completed': {
    id: 'work_order_test_123',
    title: 'Test Work Order',
    status: 'completed',
    completed_at: new Date().toISOString(),
    completion_notes: 'Work has been completed',
  },
};

export function WebhookTestDialog({ isOpen, onClose, webhook, onTest }: WebhookTestDialogProps) {
  const [selectedEvent, setSelectedEvent] = useState<WebhookEventType>(webhook.eventTypes[0]);
  const [customPayload, setCustomPayload] = useState<string>('');
  const [useCustomPayload, setUseCustomPayload] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [result, setResult] = useState<TestWebhookResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleTest = async () => {
    setIsLoading(true);
    setResult(null);
    setError(null);

    try {
      let payload: Record<string, unknown> | undefined;

      if (useCustomPayload && customPayload.trim()) {
        try {
          payload = JSON.parse(customPayload);
        } catch {
          setError('Invalid JSON payload. Please check your syntax.');
          setIsLoading(false);
          return;
        }
      } else {
        payload = SAMPLE_PAYLOADS[selectedEvent];
      }

      const response = await onTest({
        eventType: selectedEvent,
        payload,
      });

      setResult(response);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to send test webhook');
    } finally {
      setIsLoading(false);
    }
  };

  const handleClose = () => {
    setResult(null);
    setError(null);
    setCustomPayload('');
    setUseCustomPayload(false);
    onClose();
  };

  const handleLoadSamplePayload = () => {
    setCustomPayload(JSON.stringify(SAMPLE_PAYLOADS[selectedEvent], null, 2));
    setUseCustomPayload(true);
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="bg-white rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[90vh] overflow-hidden flex flex-col">
        {/* Header */}
        <div className="p-6 border-b">
          <div className="flex items-center justify-between">
            <div>
              <h2 className="text-xl font-semibold">Test Webhook</h2>
              <p className="text-sm text-muted-foreground mt-1">{webhook.name}</p>
            </div>
            <button
              type="button"
              onClick={handleClose}
              className="p-2 text-gray-500 hover:bg-gray-100 rounded-md"
            >
              <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg>
            </button>
          </div>
        </div>

        {/* Content */}
        <div className="p-6 space-y-6 overflow-y-auto flex-1">
          {/* Endpoint Info */}
          <div className="p-4 bg-gray-50 rounded-md">
            <label className="block text-xs font-medium text-muted-foreground mb-1">
              Endpoint URL
            </label>
            <code className="text-sm text-gray-900 break-all">{webhook.endpointUrl}</code>
          </div>

          {/* Event Type Selection */}
          <div>
            <label htmlFor="event-type" className="block text-sm font-medium mb-2">
              Event Type
            </label>
            <select
              id="event-type"
              value={selectedEvent}
              onChange={(e) => setSelectedEvent(e.target.value as WebhookEventType)}
              className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              {webhook.eventTypes.map((event) => (
                <option key={event} value={event}>
                  {EVENT_TYPE_LABELS[event]}
                </option>
              ))}
            </select>
          </div>

          {/* Custom Payload Toggle */}
          <div className="flex items-center justify-between">
            <label className="flex items-center gap-2">
              <input
                type="checkbox"
                checked={useCustomPayload}
                onChange={(e) => setUseCustomPayload(e.target.checked)}
                className="rounded border-gray-300"
              />
              <span className="text-sm">Use custom payload</span>
            </label>
            <button
              type="button"
              onClick={handleLoadSamplePayload}
              className="text-sm text-blue-600 hover:underline"
            >
              Load sample payload
            </button>
          </div>

          {/* Custom Payload Editor */}
          {useCustomPayload && (
            <div>
              <label htmlFor="custom-payload" className="block text-sm font-medium mb-2">
                Custom Payload (JSON)
              </label>
              <textarea
                id="custom-payload"
                value={customPayload}
                onChange={(e) => setCustomPayload(e.target.value)}
                rows={8}
                className="w-full px-3 py-2 border rounded-md font-mono text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder='{"key": "value"}'
              />
            </div>
          )}

          {/* Sample Payload Preview */}
          {!useCustomPayload && (
            <div>
              <label className="block text-sm font-medium mb-2">Sample Payload</label>
              <pre className="p-4 bg-gray-900 text-gray-100 rounded-md text-sm overflow-x-auto max-h-48">
                <code>{JSON.stringify(SAMPLE_PAYLOADS[selectedEvent], null, 2)}</code>
              </pre>
            </div>
          )}

          {/* Error Message */}
          {error && (
            <div className="p-4 bg-red-50 border border-red-200 rounded-md">
              <div className="flex gap-3">
                <svg
                  className="w-5 h-5 text-red-600 flex-shrink-0"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                  />
                </svg>
                <p className="text-sm text-red-700">{error}</p>
              </div>
            </div>
          )}

          {/* Test Result */}
          {result && (
            <div
              className={`p-4 rounded-md border ${
                result.success ? 'bg-green-50 border-green-200' : 'bg-yellow-50 border-yellow-200'
              }`}
            >
              <div className="flex items-start gap-3">
                {result.success ? (
                  <svg
                    className="w-5 h-5 text-green-600 flex-shrink-0"
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
                ) : (
                  <svg
                    className="w-5 h-5 text-yellow-600 flex-shrink-0"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                    />
                  </svg>
                )}
                <div className="flex-1">
                  <h4
                    className={`font-medium ${result.success ? 'text-green-800' : 'text-yellow-800'}`}
                  >
                    {result.success ? 'Webhook delivered successfully' : 'Webhook delivery failed'}
                  </h4>
                  <dl className="mt-2 text-sm space-y-1">
                    {result.responseStatusCode && (
                      <div className="flex gap-2">
                        <dt className="text-muted-foreground">Status Code:</dt>
                        <dd className="font-mono">{result.responseStatusCode}</dd>
                      </div>
                    )}
                    {result.responseTimeMs !== undefined && (
                      <div className="flex gap-2">
                        <dt className="text-muted-foreground">Response Time:</dt>
                        <dd className="font-mono">{result.responseTimeMs}ms</dd>
                      </div>
                    )}
                    {result.errorMessage && (
                      <div className="flex gap-2">
                        <dt className="text-muted-foreground">Error:</dt>
                        <dd className="text-red-600">{result.errorMessage}</dd>
                      </div>
                    )}
                  </dl>
                  {result.responseBody && (
                    <div className="mt-3">
                      <p className="text-xs font-medium text-muted-foreground mb-1">
                        Response Body:
                      </p>
                      <pre className="p-2 bg-gray-900 text-gray-100 rounded text-xs overflow-x-auto max-h-32">
                        <code>{result.responseBody}</code>
                      </pre>
                    </div>
                  )}
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="p-6 border-t bg-gray-50 flex justify-end gap-3">
          <button
            type="button"
            onClick={handleClose}
            className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border rounded-md hover:bg-gray-50"
          >
            Close
          </button>
          <button
            type="button"
            onClick={handleTest}
            disabled={isLoading}
            className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
          >
            {isLoading ? (
              <>
                <svg className="animate-spin h-4 w-4" viewBox="0 0 24 24">
                  <circle
                    className="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    strokeWidth="4"
                    fill="none"
                  />
                  <path
                    className="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                  />
                </svg>
                Sending...
              </>
            ) : (
              'Send Test Webhook'
            )}
          </button>
        </div>
      </div>
    </div>
  );
}
