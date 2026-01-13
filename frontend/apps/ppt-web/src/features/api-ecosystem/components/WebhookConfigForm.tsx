/**
 * Webhook Configuration Form (Epic 150, Story 150.3)
 *
 * Form for creating and editing webhook subscriptions.
 */

import type React from 'react';
import { useState } from 'react';

export interface WebhookConfig {
  name: string;
  description?: string;
  url: string;
  authType: 'hmac_sha256' | 'hmac_sha512' | 'bearer_token' | 'basic_auth' | 'none';
  events: string[];
  retryPolicy: {
    maxRetries: number;
    initialDelayMs: number;
    maxDelayMs: number;
    exponentialBackoff: boolean;
  };
  rateLimitRequests?: number;
  rateLimitWindowSeconds?: number;
  timeoutMs: number;
  verifySsl: boolean;
}

interface WebhookConfigFormProps {
  initialValues?: Partial<WebhookConfig>;
  availableEvents: { type: string; description: string }[];
  onSubmit: (config: WebhookConfig) => void;
  onCancel: () => void;
  isSubmitting?: boolean;
}

const defaultRetryPolicy = {
  maxRetries: 3,
  initialDelayMs: 1000,
  maxDelayMs: 30000,
  exponentialBackoff: true,
};

export function WebhookConfigForm({
  initialValues,
  availableEvents,
  onSubmit,
  onCancel,
  isSubmitting = false,
}: WebhookConfigFormProps) {
  const [config, setConfig] = useState<WebhookConfig>({
    name: initialValues?.name || '',
    description: initialValues?.description || '',
    url: initialValues?.url || '',
    authType: initialValues?.authType || 'hmac_sha256',
    events: initialValues?.events || [],
    retryPolicy: initialValues?.retryPolicy || defaultRetryPolicy,
    rateLimitRequests: initialValues?.rateLimitRequests,
    rateLimitWindowSeconds: initialValues?.rateLimitWindowSeconds,
    timeoutMs: initialValues?.timeoutMs || 30000,
    verifySsl: initialValues?.verifySsl ?? true,
  });

  const [secret, setSecret] = useState('');
  const [bearerToken, setBearerToken] = useState('');
  const [basicUsername, setBasicUsername] = useState('');
  const [basicPassword, setBasicPassword] = useState('');
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [errors, setErrors] = useState<Record<string, string>>({});

  const validateUrl = (url: string): boolean => {
    try {
      const parsed = new URL(url);
      return (
        parsed.protocol === 'https:' ||
        (parsed.protocol === 'http:' && parsed.hostname === 'localhost')
      );
    } catch {
      return false;
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    const newErrors: Record<string, string> = {};

    if (!config.name.trim()) {
      newErrors.name = 'Name is required';
    }

    if (!config.url.trim()) {
      newErrors.url = 'URL is required';
    } else if (!validateUrl(config.url)) {
      newErrors.url = 'URL must be a valid HTTPS URL (HTTP allowed only for localhost)';
    }

    if (config.events.length === 0) {
      newErrors.events = 'At least one event must be selected';
    }

    if (Object.keys(newErrors).length > 0) {
      setErrors(newErrors);
      return;
    }

    onSubmit(config);
  };

  const toggleEvent = (eventType: string) => {
    setConfig((prev) => ({
      ...prev,
      events: prev.events.includes(eventType)
        ? prev.events.filter((e) => e !== eventType)
        : [...prev.events, eventType],
    }));
  };

  const selectAllEvents = () => {
    setConfig((prev) => ({
      ...prev,
      events: availableEvents.map((e) => e.type),
    }));
  };

  const clearAllEvents = () => {
    setConfig((prev) => ({
      ...prev,
      events: [],
    }));
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Basic Information */}
      <div className="space-y-4">
        <h3 className="text-lg font-medium text-gray-900">Basic Information</h3>

        <div>
          <label htmlFor="name" className="block text-sm font-medium text-gray-700">
            Name *
          </label>
          <input
            type="text"
            id="name"
            value={config.name}
            onChange={(e) => setConfig({ ...config, name: e.target.value })}
            className={`mt-1 block w-full rounded-md shadow-sm sm:text-sm ${
              errors.name ? 'border-red-300' : 'border-gray-300'
            }`}
            placeholder="My Webhook"
          />
          {errors.name && <p className="mt-1 text-sm text-red-600">{errors.name}</p>}
        </div>

        <div>
          <label htmlFor="description" className="block text-sm font-medium text-gray-700">
            Description
          </label>
          <textarea
            id="description"
            value={config.description}
            onChange={(e) => setConfig({ ...config, description: e.target.value })}
            rows={2}
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm sm:text-sm"
            placeholder="Optional description of this webhook"
          />
        </div>

        <div>
          <label htmlFor="url" className="block text-sm font-medium text-gray-700">
            Endpoint URL *
          </label>
          <input
            type="url"
            id="url"
            value={config.url}
            onChange={(e) => setConfig({ ...config, url: e.target.value })}
            className={`mt-1 block w-full rounded-md shadow-sm sm:text-sm ${
              errors.url ? 'border-red-300' : 'border-gray-300'
            }`}
            placeholder="https://your-server.com/webhooks"
          />
          {errors.url && <p className="mt-1 text-sm text-red-600">{errors.url}</p>}
          <p className="mt-1 text-xs text-gray-500">
            Must be an HTTPS URL. HTTP is only allowed for localhost during development.
          </p>
        </div>
      </div>

      {/* Authentication */}
      <div className="space-y-4">
        <h3 className="text-lg font-medium text-gray-900">Authentication</h3>

        <div>
          <label htmlFor="authType" className="block text-sm font-medium text-gray-700">
            Authentication Type
          </label>
          <select
            id="authType"
            value={config.authType}
            onChange={(e) =>
              setConfig({ ...config, authType: e.target.value as WebhookConfig['authType'] })
            }
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm sm:text-sm"
          >
            <option value="hmac_sha256">HMAC SHA-256</option>
            <option value="hmac_sha512">HMAC SHA-512</option>
            <option value="bearer_token">Bearer Token</option>
            <option value="basic_auth">Basic Authentication</option>
            <option value="none">None</option>
          </select>
        </div>

        {(config.authType === 'hmac_sha256' || config.authType === 'hmac_sha512') && (
          <div>
            <label htmlFor="secret" className="block text-sm font-medium text-gray-700">
              Secret Key
            </label>
            <input
              type="password"
              id="secret"
              value={secret}
              onChange={(e) => setSecret(e.target.value)}
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm sm:text-sm"
              placeholder="Your webhook secret"
            />
            <p className="mt-1 text-xs text-gray-500">
              We will include an X-Signature header with each request for verification.
            </p>
          </div>
        )}

        {config.authType === 'bearer_token' && (
          <div>
            <label htmlFor="bearerToken" className="block text-sm font-medium text-gray-700">
              Bearer Token
            </label>
            <input
              type="password"
              id="bearerToken"
              value={bearerToken}
              onChange={(e) => setBearerToken(e.target.value)}
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm sm:text-sm"
              placeholder="Your bearer token"
            />
          </div>
        )}

        {config.authType === 'basic_auth' && (
          <>
            <div>
              <label htmlFor="basicUsername" className="block text-sm font-medium text-gray-700">
                Username
              </label>
              <input
                type="text"
                id="basicUsername"
                value={basicUsername}
                onChange={(e) => setBasicUsername(e.target.value)}
                className="mt-1 block w-full rounded-md border-gray-300 shadow-sm sm:text-sm"
              />
            </div>
            <div>
              <label htmlFor="basicPassword" className="block text-sm font-medium text-gray-700">
                Password
              </label>
              <input
                type="password"
                id="basicPassword"
                value={basicPassword}
                onChange={(e) => setBasicPassword(e.target.value)}
                className="mt-1 block w-full rounded-md border-gray-300 shadow-sm sm:text-sm"
              />
            </div>
          </>
        )}
      </div>

      {/* Events */}
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <h3 className="text-lg font-medium text-gray-900">Events *</h3>
          <div className="space-x-2">
            <button
              type="button"
              onClick={selectAllEvents}
              className="text-sm text-primary-600 hover:text-primary-700"
            >
              Select All
            </button>
            <span className="text-gray-400">|</span>
            <button
              type="button"
              onClick={clearAllEvents}
              className="text-sm text-primary-600 hover:text-primary-700"
            >
              Clear All
            </button>
          </div>
        </div>

        {errors.events && <p className="text-sm text-red-600">{errors.events}</p>}

        <div className="grid grid-cols-1 gap-2 sm:grid-cols-2">
          {availableEvents.map((event) => (
            <label
              key={event.type}
              className="flex items-start space-x-3 rounded-lg border p-3 hover:bg-gray-50"
            >
              <input
                type="checkbox"
                checked={config.events.includes(event.type)}
                onChange={() => toggleEvent(event.type)}
                className="mt-1 h-4 w-4 rounded border-gray-300 text-primary-600"
              />
              <div>
                <p className="text-sm font-medium text-gray-900">{event.type}</p>
                <p className="text-xs text-gray-500">{event.description}</p>
              </div>
            </label>
          ))}
        </div>
      </div>

      {/* Advanced Settings */}
      <div>
        <button
          type="button"
          onClick={() => setShowAdvanced(!showAdvanced)}
          className="flex items-center text-sm text-gray-600 hover:text-gray-900"
        >
          <svg
            className={`mr-2 h-4 w-4 transition-transform ${showAdvanced ? 'rotate-90' : ''}`}
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
          </svg>
          Advanced Settings
        </button>

        {showAdvanced && (
          <div className="mt-4 space-y-4 rounded-lg border border-gray-200 bg-gray-50 p-4">
            <h4 className="text-sm font-medium text-gray-900">Retry Policy</h4>

            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm text-gray-700">Max Retries</label>
                <input
                  type="number"
                  min={0}
                  max={10}
                  value={config.retryPolicy.maxRetries}
                  onChange={(e) =>
                    setConfig({
                      ...config,
                      retryPolicy: {
                        ...config.retryPolicy,
                        maxRetries: Number.parseInt(e.target.value, 10),
                      },
                    })
                  }
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm sm:text-sm"
                />
              </div>

              <div>
                <label className="block text-sm text-gray-700">Initial Delay (ms)</label>
                <input
                  type="number"
                  min={100}
                  value={config.retryPolicy.initialDelayMs}
                  onChange={(e) =>
                    setConfig({
                      ...config,
                      retryPolicy: {
                        ...config.retryPolicy,
                        initialDelayMs: Number.parseInt(e.target.value, 10),
                      },
                    })
                  }
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm sm:text-sm"
                />
              </div>
            </div>

            <label className="flex items-center space-x-2">
              <input
                type="checkbox"
                checked={config.retryPolicy.exponentialBackoff}
                onChange={(e) =>
                  setConfig({
                    ...config,
                    retryPolicy: {
                      ...config.retryPolicy,
                      exponentialBackoff: e.target.checked,
                    },
                  })
                }
                className="h-4 w-4 rounded border-gray-300 text-primary-600"
              />
              <span className="text-sm text-gray-700">Use exponential backoff</span>
            </label>

            <h4 className="pt-4 text-sm font-medium text-gray-900">Other Settings</h4>

            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm text-gray-700">Timeout (ms)</label>
                <input
                  type="number"
                  min={1000}
                  max={60000}
                  value={config.timeoutMs}
                  onChange={(e) =>
                    setConfig({ ...config, timeoutMs: Number.parseInt(e.target.value, 10) })
                  }
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm sm:text-sm"
                />
              </div>

              <div>
                <label className="block text-sm text-gray-700">Rate Limit (requests/window)</label>
                <input
                  type="number"
                  min={1}
                  value={config.rateLimitRequests || ''}
                  onChange={(e) =>
                    setConfig({
                      ...config,
                      rateLimitRequests: e.target.value
                        ? Number.parseInt(e.target.value, 10)
                        : undefined,
                    })
                  }
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm sm:text-sm"
                  placeholder="No limit"
                />
              </div>
            </div>

            <label className="flex items-center space-x-2">
              <input
                type="checkbox"
                checked={config.verifySsl}
                onChange={(e) => setConfig({ ...config, verifySsl: e.target.checked })}
                className="h-4 w-4 rounded border-gray-300 text-primary-600"
              />
              <span className="text-sm text-gray-700">Verify SSL certificate</span>
            </label>
          </div>
        )}
      </div>

      {/* Actions */}
      <div className="flex justify-end space-x-3 border-t pt-4">
        <button
          type="button"
          onClick={onCancel}
          className="rounded-lg border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50"
        >
          Cancel
        </button>
        <button
          type="submit"
          disabled={isSubmitting}
          className="rounded-lg bg-primary-600 px-4 py-2 text-sm font-medium text-white hover:bg-primary-700 disabled:opacity-50"
        >
          {isSubmitting ? 'Saving...' : 'Save Webhook'}
        </button>
      </div>
    </form>
  );
}

export default WebhookConfigForm;
