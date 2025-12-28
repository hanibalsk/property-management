/**
 * API Keys List Component (Epic 69 - Story 69.1)
 *
 * Displays a list of API keys with actions to create, view, rotate, and revoke.
 */

import { useState } from 'react';
import type { ApiKey, ApiKeyScope, ApiKeyStatus } from '../types';

interface ApiKeysListProps {
  apiKeys: ApiKey[];
  onCreateKey: () => void;
  onRotateKey: (keyId: string) => void;
  onRevokeKey: (keyId: string) => void;
  onViewUsage: (keyId: string) => void;
}

export function ApiKeysList({
  apiKeys,
  onCreateKey,
  onRotateKey,
  onRevokeKey,
  onViewUsage,
}: ApiKeysListProps) {
  const [filter, setFilter] = useState<ApiKeyStatus | 'all'>('all');
  const [showRevokeConfirm, setShowRevokeConfirm] = useState<string | null>(null);

  const filteredKeys = apiKeys.filter((key) => {
    if (filter === 'all') return true;
    return key.status === filter;
  });

  const activeCount = apiKeys.filter((k) => k.status === 'active').length;
  const revokedCount = apiKeys.filter((k) => k.status === 'revoked').length;

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold tracking-tight">API Keys</h2>
          <p className="text-muted-foreground">Manage your API keys for authenticating requests</p>
        </div>
        <button
          type="button"
          onClick={onCreateKey}
          className="inline-flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
        >
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
          </svg>
          Create API Key
        </button>
      </div>

      {/* Stats */}
      <div className="flex gap-4">
        <div className="px-4 py-2 bg-green-100 text-green-800 rounded-lg">
          <span className="font-semibold">{activeCount}</span> active
        </div>
        <div className="px-4 py-2 bg-gray-100 text-gray-800 rounded-lg">
          <span className="font-semibold">{revokedCount}</span> revoked
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
        <FilterButton active={filter === 'revoked'} onClick={() => setFilter('revoked')}>
          Revoked
        </FilterButton>
        <FilterButton active={filter === 'expired'} onClick={() => setFilter('expired')}>
          Expired
        </FilterButton>
      </div>

      {/* Keys List */}
      {filteredKeys.length === 0 ? (
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
              d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"
            />
          </svg>
          <h3 className="text-lg font-medium mb-1">No API keys found</h3>
          <p className="text-muted-foreground mb-4">
            Create your first API key to start making authenticated requests.
          </p>
          <button
            type="button"
            onClick={onCreateKey}
            className="inline-flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
          >
            Create API Key
          </button>
        </div>
      ) : (
        <div className="space-y-4">
          {filteredKeys.map((key) => (
            <ApiKeyCard
              key={key.id}
              apiKey={key}
              onRotate={() => onRotateKey(key.id)}
              onRevoke={() => setShowRevokeConfirm(key.id)}
              onViewUsage={() => onViewUsage(key.id)}
            />
          ))}
        </div>
      )}

      {/* Revoke Confirmation Modal */}
      {showRevokeConfirm && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
          <div className="bg-white rounded-lg p-6 max-w-md w-full mx-4">
            <h3 className="text-lg font-semibold mb-2">Revoke API Key?</h3>
            <p className="text-muted-foreground mb-4">
              This action cannot be undone. Any applications using this key will immediately lose
              access.
            </p>
            <div className="flex justify-end gap-2">
              <button
                type="button"
                onClick={() => setShowRevokeConfirm(null)}
                className="px-4 py-2 border rounded-md hover:bg-gray-50 transition-colors"
              >
                Cancel
              </button>
              <button
                type="button"
                onClick={() => {
                  onRevokeKey(showRevokeConfirm);
                  setShowRevokeConfirm(null);
                }}
                className="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 transition-colors"
              >
                Revoke Key
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

interface ApiKeyCardProps {
  apiKey: ApiKey;
  onRotate: () => void;
  onRevoke: () => void;
  onViewUsage: () => void;
}

function ApiKeyCard({ apiKey, onRotate, onRevoke, onViewUsage }: ApiKeyCardProps) {
  const isActive = apiKey.status === 'active';
  const isExpired = apiKey.expiresAt && new Date(apiKey.expiresAt) < new Date();

  return (
    <div className="p-4 bg-white rounded-lg border shadow-sm">
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <div className="flex items-center gap-2 mb-1">
            <h3 className="font-semibold">{apiKey.name}</h3>
            <StatusBadge status={apiKey.status} />
            {isExpired && <span className="text-xs text-red-600">(Expired)</span>}
          </div>
          <p className="text-sm font-mono text-muted-foreground mb-2">{apiKey.keyPrefix}...</p>
          <div className="flex flex-wrap gap-1 mb-2">
            {apiKey.scopes.map((scope) => (
              <ScopeBadge key={scope} scope={scope} />
            ))}
          </div>
          <div className="flex items-center gap-4 text-xs text-muted-foreground">
            <span>Created: {formatDate(apiKey.createdAt)}</span>
            {apiKey.lastUsedAt && <span>Last used: {formatDate(apiKey.lastUsedAt)}</span>}
            <span>Requests: {apiKey.totalRequests.toLocaleString()}</span>
          </div>
        </div>
        <div className="flex gap-2">
          <button
            type="button"
            onClick={onViewUsage}
            className="p-2 text-gray-600 hover:bg-gray-100 rounded-md transition-colors"
            title="View Usage"
          >
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
              />
            </svg>
          </button>
          {isActive && (
            <>
              <button
                type="button"
                onClick={onRotate}
                className="p-2 text-blue-600 hover:bg-blue-50 rounded-md transition-colors"
                title="Rotate Key"
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
              <button
                type="button"
                onClick={onRevoke}
                className="p-2 text-red-600 hover:bg-red-50 rounded-md transition-colors"
                title="Revoke Key"
              >
                <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636"
                  />
                </svg>
              </button>
            </>
          )}
        </div>
      </div>
    </div>
  );
}

function StatusBadge({ status }: { status: ApiKeyStatus }) {
  const colors = {
    active: 'bg-green-100 text-green-800',
    revoked: 'bg-red-100 text-red-800',
    expired: 'bg-gray-100 text-gray-800',
    suspended: 'bg-yellow-100 text-yellow-800',
  };

  return (
    <span className={`px-2 py-0.5 text-xs font-medium rounded-full ${colors[status]}`}>
      {status}
    </span>
  );
}

function ScopeBadge({ scope }: { scope: ApiKeyScope }) {
  return <span className="px-2 py-0.5 text-xs bg-gray-100 text-gray-700 rounded">{scope}</span>;
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  });
}
