/**
 * API Key Create Dialog Component (Epic 69 - Story 69.1)
 *
 * Dialog for creating new API keys with scope selection and rate limit configuration.
 */

import { useState } from 'react';
import type { ApiKeyScope, CreateApiKey } from '../types';

interface ApiKeyCreateDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSubmit: (data: CreateApiKey) => Promise<void>;
  isLoading?: boolean;
}

const AVAILABLE_SCOPES: { value: ApiKeyScope; label: string; description: string }[] = [
  { value: 'read', label: 'Read', description: 'Read access to all resources' },
  { value: 'write', label: 'Write', description: 'Write access to all resources' },
  { value: 'admin', label: 'Admin', description: 'Full administrative access' },
  { value: 'buildings:read', label: 'Buildings (Read)', description: 'Read building data' },
  { value: 'buildings:write', label: 'Buildings (Write)', description: 'Modify building data' },
  { value: 'faults:read', label: 'Faults (Read)', description: 'Read fault reports' },
  { value: 'faults:write', label: 'Faults (Write)', description: 'Create and update faults' },
  { value: 'financial:read', label: 'Financial (Read)', description: 'Read financial data' },
  { value: 'financial:write', label: 'Financial (Write)', description: 'Modify financial data' },
  { value: 'residents:read', label: 'Residents (Read)', description: 'Read resident data' },
  { value: 'residents:write', label: 'Residents (Write)', description: 'Modify resident data' },
  { value: 'webhooks:manage', label: 'Webhooks', description: 'Manage webhook subscriptions' },
];

export function ApiKeyCreateDialog({
  isOpen,
  onClose,
  onSubmit,
  isLoading,
}: ApiKeyCreateDialogProps) {
  const [name, setName] = useState('');
  const [selectedScopes, setSelectedScopes] = useState<ApiKeyScope[]>([]);
  const [expiresIn, setExpiresIn] = useState<string>('never');
  const [rateLimitPerMinute, setRateLimitPerMinute] = useState<string>('');
  const [allowedIps, setAllowedIps] = useState<string>('');
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleScopeToggle = (scope: ApiKeyScope) => {
    setSelectedScopes((prev) =>
      prev.includes(scope) ? prev.filter((s) => s !== scope) : [...prev, scope]
    );
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    if (!name.trim()) {
      setError('Please enter a name for the API key');
      return;
    }

    if (selectedScopes.length === 0) {
      setError('Please select at least one scope');
      return;
    }

    const expiresAt = calculateExpiresAt(expiresIn);
    const ips = allowedIps
      .split(',')
      .map((ip) => ip.trim())
      .filter((ip) => ip);

    const data: CreateApiKey = {
      name: name.trim(),
      scopes: selectedScopes,
      expiresAt: expiresAt ?? undefined,
      rateLimitPerMinute: rateLimitPerMinute ? parseInt(rateLimitPerMinute, 10) : undefined,
      allowedIps: ips.length > 0 ? ips : undefined,
    };

    try {
      await onSubmit(data);
      resetForm();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create API key');
    }
  };

  const resetForm = () => {
    setName('');
    setSelectedScopes([]);
    setExpiresIn('never');
    setRateLimitPerMinute('');
    setAllowedIps('');
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
      <div className="bg-white rounded-lg shadow-xl max-w-lg w-full mx-4 max-h-[90vh] overflow-y-auto">
        <div className="p-6 border-b">
          <h2 className="text-xl font-semibold">Create API Key</h2>
          <p className="text-sm text-muted-foreground">
            Generate a new API key for authenticating your application
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
              placeholder="e.g., Production API Key"
              className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <p className="mt-1 text-xs text-muted-foreground">
              A descriptive name to identify this key
            </p>
          </div>

          {/* Scopes */}
          <div>
            <label className="block text-sm font-medium mb-2">
              Scopes <span className="text-red-500">*</span>
            </label>
            <div className="grid grid-cols-2 gap-2 max-h-48 overflow-y-auto p-1">
              {AVAILABLE_SCOPES.map((scope) => (
                <label
                  key={scope.value}
                  className={`flex items-start gap-2 p-2 border rounded-md cursor-pointer transition-colors ${
                    selectedScopes.includes(scope.value)
                      ? 'border-blue-500 bg-blue-50'
                      : 'border-gray-200 hover:border-gray-300'
                  }`}
                >
                  <input
                    type="checkbox"
                    checked={selectedScopes.includes(scope.value)}
                    onChange={() => handleScopeToggle(scope.value)}
                    className="mt-0.5"
                  />
                  <div>
                    <div className="text-sm font-medium">{scope.label}</div>
                    <div className="text-xs text-muted-foreground">{scope.description}</div>
                  </div>
                </label>
              ))}
            </div>
          </div>

          {/* Expiration */}
          <div>
            <label htmlFor="expiresIn" className="block text-sm font-medium mb-1">
              Expiration
            </label>
            <select
              id="expiresIn"
              value={expiresIn}
              onChange={(e) => setExpiresIn(e.target.value)}
              className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="never">Never expires</option>
              <option value="30days">30 days</option>
              <option value="90days">90 days</option>
              <option value="1year">1 year</option>
            </select>
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
            <div className="space-y-4 p-4 bg-gray-50 rounded-md">
              {/* Rate Limit */}
              <div>
                <label htmlFor="rateLimit" className="block text-sm font-medium mb-1">
                  Rate Limit (requests/minute)
                </label>
                <input
                  type="number"
                  id="rateLimit"
                  value={rateLimitPerMinute}
                  onChange={(e) => setRateLimitPerMinute(e.target.value)}
                  placeholder="Leave empty for default"
                  min="1"
                  className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              {/* Allowed IPs */}
              <div>
                <label htmlFor="allowedIps" className="block text-sm font-medium mb-1">
                  Allowed IP Addresses
                </label>
                <input
                  type="text"
                  id="allowedIps"
                  value={allowedIps}
                  onChange={(e) => setAllowedIps(e.target.value)}
                  placeholder="e.g., 192.168.1.1, 10.0.0.0/24"
                  className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
                <p className="mt-1 text-xs text-muted-foreground">
                  Comma-separated list. Leave empty to allow all IPs.
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
              {isLoading ? 'Creating...' : 'Create API Key'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

function calculateExpiresAt(expiresIn: string): string | null {
  const now = new Date();
  switch (expiresIn) {
    case '30days':
      now.setDate(now.getDate() + 30);
      return now.toISOString();
    case '90days':
      now.setDate(now.getDate() + 90);
      return now.toISOString();
    case '1year':
      now.setFullYear(now.getFullYear() + 1);
      return now.toISOString();
    default:
      return null;
  }
}
