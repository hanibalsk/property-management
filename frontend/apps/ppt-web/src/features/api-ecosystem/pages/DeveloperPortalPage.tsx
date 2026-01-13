/**
 * Developer Portal Page (Epic 150, Story 150.5)
 *
 * Main page for the API developer portal with documentation and API key management.
 */

import { useState } from 'react';
import { DeveloperPortalNav } from '../components/DeveloperPortalNav';

interface ApiKey {
  id: string;
  name: string;
  keyPrefix: string;
  scopes: string[];
  rateLimitTier: string;
  isSandbox: boolean;
  status: 'active' | 'revoked';
  lastUsedAt?: string;
  expiresAt?: string;
  createdAt: string;
}

// Sample API keys
const sampleApiKeys: ApiKey[] = [
  {
    id: '1',
    name: 'Production API Key',
    keyPrefix: 'ppt_live_1234',
    scopes: ['read:buildings', 'read:units', 'write:faults'],
    rateLimitTier: 'standard',
    isSandbox: false,
    status: 'active',
    lastUsedAt: '2024-01-13T10:30:00Z',
    createdAt: '2024-01-01T00:00:00Z',
  },
  {
    id: '2',
    name: 'Development API Key',
    keyPrefix: 'ppt_test_5678',
    scopes: ['read:*', 'write:*'],
    rateLimitTier: 'sandbox',
    isSandbox: true,
    status: 'active',
    lastUsedAt: '2024-01-12T14:20:00Z',
    createdAt: '2024-01-05T00:00:00Z',
  },
];

export function DeveloperPortalPage() {
  const [apiKeys] = useState<ApiKey[]>(sampleApiKeys);
  const [showCreateKeyModal, setShowCreateKeyModal] = useState(false);
  const [newKeyName, setNewKeyName] = useState('');
  const [newKeyScopes, setNewKeyScopes] = useState<string[]>([]);
  const [newKeyIsSandbox, setNewKeyIsSandbox] = useState(true);
  const [createdKey, setCreatedKey] = useState<string | null>(null);

  const availableScopes = [
    { value: 'read:buildings', label: 'Read Buildings' },
    { value: 'write:buildings', label: 'Write Buildings' },
    { value: 'read:units', label: 'Read Units' },
    { value: 'write:units', label: 'Write Units' },
    { value: 'read:faults', label: 'Read Faults' },
    { value: 'write:faults', label: 'Write Faults' },
    { value: 'read:documents', label: 'Read Documents' },
    { value: 'write:documents', label: 'Write Documents' },
    { value: 'read:*', label: 'Read All (Sandbox only)' },
    { value: 'write:*', label: 'Write All (Sandbox only)' },
  ];

  const handleCreateKey = () => {
    // Simulate API key creation
    const newKey = `ppt_${newKeyIsSandbox ? 'test' : 'live'}_${Math.random().toString(36).slice(2, 18)}`;
    setCreatedKey(newKey);
  };

  const formatDate = (dateString?: string) => {
    if (!dateString) return 'Never';
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  return (
    <div className="flex min-h-screen bg-gray-50">
      {/* Sidebar */}
      <div className="hidden w-64 flex-shrink-0 border-r border-gray-200 bg-white lg:block">
        <div className="sticky top-0 p-6">
          <div className="mb-6">
            <h2 className="text-lg font-semibold text-gray-900">Developer Portal</h2>
            <p className="text-sm text-gray-500">PPT API Documentation</p>
          </div>
          <DeveloperPortalNav />
        </div>
      </div>

      {/* Main content */}
      <div className="flex-1">
        <div className="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
          {/* Welcome section */}
          <div className="mb-8 rounded-lg bg-gradient-to-r from-primary-600 to-primary-800 p-8 text-white">
            <h1 className="text-2xl font-bold">Welcome to the PPT API</h1>
            <p className="mt-2 text-primary-100">
              Build powerful integrations with our comprehensive REST API. Access property data,
              manage faults, sync documents, and more.
            </p>
            <div className="mt-4 flex space-x-4">
              <button
                type="button"
                className="rounded-lg bg-white px-4 py-2 text-sm font-medium text-primary-700 hover:bg-primary-50"
              >
                View Documentation
              </button>
              <button
                type="button"
                className="rounded-lg border border-white/30 px-4 py-2 text-sm font-medium text-white hover:bg-white/10"
              >
                API Reference
              </button>
            </div>
          </div>

          {/* Quick start */}
          <div className="mb-8">
            <h2 className="mb-4 text-lg font-medium text-gray-900">Quick Start</h2>
            <div className="rounded-lg border border-gray-200 bg-white p-6">
              <h3 className="text-sm font-medium text-gray-900">Make your first API call</h3>
              <div className="mt-4 overflow-x-auto rounded-lg bg-gray-900 p-4">
                <pre className="text-sm text-gray-100">
                  <code>{`curl -X GET "https://api.ppt.com/api/v1/buildings" \\
  -H "Authorization: Bearer YOUR_API_KEY" \\
  -H "Content-Type: application/json"`}</code>
                </pre>
              </div>
              <p className="mt-4 text-sm text-gray-600">
                Replace <code className="rounded bg-gray-100 px-1 py-0.5">YOUR_API_KEY</code> with
                your actual API key. Get one below.
              </p>
            </div>
          </div>

          {/* API Keys section */}
          <div className="mb-8">
            <div className="mb-4 flex items-center justify-between">
              <h2 className="text-lg font-medium text-gray-900">Your API Keys</h2>
              <button
                type="button"
                onClick={() => setShowCreateKeyModal(true)}
                className="rounded-lg bg-primary-600 px-4 py-2 text-sm font-medium text-white hover:bg-primary-700"
              >
                Create New Key
              </button>
            </div>

            <div className="overflow-hidden rounded-lg border border-gray-200 bg-white">
              <table className="min-w-full divide-y divide-gray-200">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                      Name
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                      Key
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                      Type
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                      Last Used
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                      Status
                    </th>
                    <th className="px-6 py-3 text-right text-xs font-medium uppercase tracking-wider text-gray-500">
                      Actions
                    </th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-200 bg-white">
                  {apiKeys.map((key) => (
                    <tr key={key.id}>
                      <td className="whitespace-nowrap px-6 py-4">
                        <div className="text-sm font-medium text-gray-900">{key.name}</div>
                        <div className="text-xs text-gray-500">
                          {key.scopes.slice(0, 2).join(', ')}
                          {key.scopes.length > 2 && ` +${key.scopes.length - 2} more`}
                        </div>
                      </td>
                      <td className="whitespace-nowrap px-6 py-4">
                        <code className="rounded bg-gray-100 px-2 py-1 text-sm text-gray-800">
                          {key.keyPrefix}...
                        </code>
                      </td>
                      <td className="whitespace-nowrap px-6 py-4">
                        <span
                          className={`inline-flex rounded-full px-2 py-1 text-xs font-medium ${
                            key.isSandbox
                              ? 'bg-yellow-100 text-yellow-800'
                              : 'bg-green-100 text-green-800'
                          }`}
                        >
                          {key.isSandbox ? 'Sandbox' : 'Production'}
                        </span>
                      </td>
                      <td className="whitespace-nowrap px-6 py-4 text-sm text-gray-500">
                        {formatDate(key.lastUsedAt)}
                      </td>
                      <td className="whitespace-nowrap px-6 py-4">
                        <span
                          className={`inline-flex rounded-full px-2 py-1 text-xs font-medium ${
                            key.status === 'active'
                              ? 'bg-green-100 text-green-800'
                              : 'bg-red-100 text-red-800'
                          }`}
                        >
                          {key.status}
                        </span>
                      </td>
                      <td className="whitespace-nowrap px-6 py-4 text-right text-sm">
                        <button type="button" className="text-primary-600 hover:text-primary-700">
                          Rotate
                        </button>
                        <span className="mx-2 text-gray-300">|</span>
                        <button type="button" className="text-red-600 hover:text-red-700">
                          Revoke
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>

          {/* Usage stats */}
          <div className="mb-8">
            <h2 className="mb-4 text-lg font-medium text-gray-900">Usage Statistics</h2>
            <div className="grid grid-cols-1 gap-4 sm:grid-cols-3">
              <div className="rounded-lg border border-gray-200 bg-white p-6">
                <p className="text-sm font-medium text-gray-500">API Calls Today</p>
                <p className="mt-2 text-3xl font-semibold text-gray-900">1,247</p>
                <p className="mt-1 text-sm text-green-600">+12% from yesterday</p>
              </div>
              <div className="rounded-lg border border-gray-200 bg-white p-6">
                <p className="text-sm font-medium text-gray-500">API Calls This Month</p>
                <p className="mt-2 text-3xl font-semibold text-gray-900">42,891</p>
                <p className="mt-1 text-sm text-gray-500">of 100,000 limit</p>
              </div>
              <div className="rounded-lg border border-gray-200 bg-white p-6">
                <p className="text-sm font-medium text-gray-500">Avg Response Time</p>
                <p className="mt-2 text-3xl font-semibold text-gray-900">127ms</p>
                <p className="mt-1 text-sm text-green-600">Excellent</p>
              </div>
            </div>
          </div>

          {/* Resources */}
          <div>
            <h2 className="mb-4 text-lg font-medium text-gray-900">Resources</h2>
            <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
              <a
                href="/api-ecosystem/docs"
                className="flex items-center rounded-lg border border-gray-200 bg-white p-4 hover:border-primary-300 hover:shadow-sm"
              >
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-primary-100">
                  <svg
                    className="h-6 w-6 text-primary-600"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
                    />
                  </svg>
                </div>
                <div className="ml-4">
                  <h3 className="text-sm font-medium text-gray-900">API Documentation</h3>
                  <p className="text-sm text-gray-500">Complete API reference and guides</p>
                </div>
              </a>
              <a
                href="/api-ecosystem/code-samples"
                className="flex items-center rounded-lg border border-gray-200 bg-white p-4 hover:border-primary-300 hover:shadow-sm"
              >
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-purple-100">
                  <svg
                    className="h-6 w-6 text-purple-600"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"
                    />
                  </svg>
                </div>
                <div className="ml-4">
                  <h3 className="text-sm font-medium text-gray-900">Code Samples</h3>
                  <p className="text-sm text-gray-500">Examples in multiple languages</p>
                </div>
              </a>
              <a
                href="/api-ecosystem/webhooks"
                className="flex items-center rounded-lg border border-gray-200 bg-white p-4 hover:border-primary-300 hover:shadow-sm"
              >
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-yellow-100">
                  <svg
                    className="h-6 w-6 text-yellow-600"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"
                    />
                  </svg>
                </div>
                <div className="ml-4">
                  <h3 className="text-sm font-medium text-gray-900">Webhooks Guide</h3>
                  <p className="text-sm text-gray-500">Real-time event notifications</p>
                </div>
              </a>
              <a
                href="/api-ecosystem/support"
                className="flex items-center rounded-lg border border-gray-200 bg-white p-4 hover:border-primary-300 hover:shadow-sm"
              >
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-green-100">
                  <svg
                    className="h-6 w-6 text-green-600"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                  </svg>
                </div>
                <div className="ml-4">
                  <h3 className="text-sm font-medium text-gray-900">Support</h3>
                  <p className="text-sm text-gray-500">Get help from our team</p>
                </div>
              </a>
            </div>
          </div>
        </div>
      </div>

      {/* Create Key Modal */}
      {showCreateKeyModal && (
        <div className="fixed inset-0 z-50 overflow-y-auto">
          <div className="flex min-h-screen items-center justify-center p-4">
            <button
              type="button"
              aria-label="Close modal"
              className="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity cursor-default"
              onClick={() => {
                setShowCreateKeyModal(false);
                setCreatedKey(null);
              }}
            />
            <div className="relative z-50 w-full max-w-md rounded-lg bg-white p-6 shadow-xl">
              {createdKey ? (
                <>
                  <h3 className="text-lg font-medium text-gray-900">API Key Created</h3>
                  <p className="mt-2 text-sm text-gray-500">
                    Copy your API key now. You will not be able to see it again.
                  </p>
                  <div className="mt-4 rounded-lg bg-gray-100 p-4">
                    <code className="break-all text-sm text-gray-800">{createdKey}</code>
                  </div>
                  <div className="mt-4 flex justify-end space-x-3">
                    <button
                      type="button"
                      onClick={() => {
                        navigator.clipboard.writeText(createdKey);
                      }}
                      className="rounded-lg border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50"
                    >
                      Copy to Clipboard
                    </button>
                    <button
                      type="button"
                      onClick={() => {
                        setShowCreateKeyModal(false);
                        setCreatedKey(null);
                      }}
                      className="rounded-lg bg-primary-600 px-4 py-2 text-sm font-medium text-white hover:bg-primary-700"
                    >
                      Done
                    </button>
                  </div>
                </>
              ) : (
                <>
                  <h3 className="text-lg font-medium text-gray-900">Create New API Key</h3>
                  <div className="mt-4 space-y-4">
                    <div>
                      <label className="block text-sm font-medium text-gray-700">Key Name</label>
                      <input
                        type="text"
                        value={newKeyName}
                        onChange={(e) => setNewKeyName(e.target.value)}
                        className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
                        placeholder="My API Key"
                      />
                    </div>
                    <div>
                      <label className="block text-sm font-medium text-gray-700">Environment</label>
                      <div className="mt-2 flex space-x-4">
                        <label className="flex items-center">
                          <input
                            type="radio"
                            checked={newKeyIsSandbox}
                            onChange={() => setNewKeyIsSandbox(true)}
                            className="h-4 w-4 border-gray-300 text-primary-600"
                          />
                          <span className="ml-2 text-sm text-gray-700">Sandbox</span>
                        </label>
                        <label className="flex items-center">
                          <input
                            type="radio"
                            checked={!newKeyIsSandbox}
                            onChange={() => setNewKeyIsSandbox(false)}
                            className="h-4 w-4 border-gray-300 text-primary-600"
                          />
                          <span className="ml-2 text-sm text-gray-700">Production</span>
                        </label>
                      </div>
                    </div>
                    <div>
                      <label className="block text-sm font-medium text-gray-700">Scopes</label>
                      <div className="mt-2 max-h-48 overflow-y-auto rounded-md border border-gray-200 p-2">
                        {availableScopes.map((scope) => (
                          <label key={scope.value} className="flex items-center p-1">
                            <input
                              type="checkbox"
                              checked={newKeyScopes.includes(scope.value)}
                              onChange={(e) => {
                                if (e.target.checked) {
                                  setNewKeyScopes([...newKeyScopes, scope.value]);
                                } else {
                                  setNewKeyScopes(newKeyScopes.filter((s) => s !== scope.value));
                                }
                              }}
                              className="h-4 w-4 rounded border-gray-300 text-primary-600"
                            />
                            <span className="ml-2 text-sm text-gray-700">{scope.label}</span>
                          </label>
                        ))}
                      </div>
                    </div>
                  </div>
                  <div className="mt-6 flex justify-end space-x-3">
                    <button
                      type="button"
                      onClick={() => setShowCreateKeyModal(false)}
                      className="rounded-lg border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50"
                    >
                      Cancel
                    </button>
                    <button
                      type="button"
                      onClick={handleCreateKey}
                      className="rounded-lg bg-primary-600 px-4 py-2 text-sm font-medium text-white hover:bg-primary-700"
                    >
                      Create Key
                    </button>
                  </div>
                </>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default DeveloperPortalPage;
