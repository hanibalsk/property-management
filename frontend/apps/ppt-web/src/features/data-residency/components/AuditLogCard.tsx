/**
 * Audit Log Card (Epic 146, Story 146.4).
 *
 * Displays data residency audit trail with tamper-evident verification.
 */

import type React from 'react';

export interface AuditChange {
  field: string;
  old_value?: string;
  new_value?: string;
}

export interface AuditLogEntry {
  id: string;
  event_type: string;
  description: string;
  user_id?: string;
  user_name?: string;
  changes?: AuditChange[];
  details?: Record<string, unknown>;
  ip_address?: string;
  created_at: string;
  chain_valid: boolean;
}

interface AuditLogCardProps {
  entries: AuditLogEntry[];
  totalCount: number;
  chainValid: boolean;
  onVerifyChain?: () => void;
  onLoadMore?: () => void;
  hasMore?: boolean;
}

const getEventTypeStyles = (eventType: string) => {
  if (eventType.includes('created') || eventType.includes('completed')) {
    return 'bg-green-100 text-green-800';
  }
  if (eventType.includes('updated') || eventType.includes('changed')) {
    return 'bg-blue-100 text-blue-800';
  }
  if (eventType.includes('started') || eventType.includes('access')) {
    return 'bg-yellow-100 text-yellow-800';
  }
  if (eventType.includes('removed') || eventType.includes('failed')) {
    return 'bg-red-100 text-red-800';
  }
  return 'bg-gray-100 text-gray-800';
};

const formatEventType = (eventType: string): string => {
  return eventType
    .split('_')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
};

export const AuditLogCard: React.FC<AuditLogCardProps> = ({
  entries,
  totalCount,
  chainValid,
  onVerifyChain,
  onLoadMore,
  hasMore,
}) => {
  return (
    <div className="bg-white rounded-lg shadow-md border border-gray-200 overflow-hidden">
      {/* Header */}
      <div className="px-6 py-4 border-b border-gray-200">
        <div className="flex items-center justify-between">
          <div>
            <h3 className="text-lg font-semibold text-gray-900">Audit Trail</h3>
            <p className="text-sm text-gray-500 mt-1">
              {totalCount} events recorded (tamper-evident)
            </p>
          </div>
          <div className="flex items-center space-x-3">
            <span
              className={`px-3 py-1 text-xs font-medium rounded-full ${
                chainValid ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'
              }`}
            >
              {chainValid ? 'Chain Valid' : 'Chain Invalid'}
            </span>
            {onVerifyChain && (
              <button
                type="button"
                onClick={onVerifyChain}
                className="px-3 py-1 text-xs font-medium text-indigo-600 bg-indigo-50 rounded hover:bg-indigo-100 transition-colors"
              >
                Verify Chain
              </button>
            )}
          </div>
        </div>
      </div>

      {/* Audit Log List */}
      <div className="divide-y divide-gray-100">
        {entries.map((entry) => (
          <div
            key={entry.id}
            className={`px-6 py-4 ${!entry.chain_valid ? 'bg-red-50' : ''}`}
          >
            <div className="flex items-start justify-between">
              <div className="flex items-start space-x-4">
                {/* Event Type Icon */}
                <div className="flex-shrink-0 mt-1">
                  <span
                    className={`inline-block w-2 h-2 rounded-full ${
                      entry.chain_valid ? 'bg-green-400' : 'bg-red-400'
                    }`}
                  />
                </div>

                {/* Event Details */}
                <div className="min-w-0 flex-1">
                  <div className="flex items-center space-x-2">
                    <span
                      className={`px-2 py-0.5 text-xs font-medium rounded ${getEventTypeStyles(
                        entry.event_type
                      )}`}
                    >
                      {formatEventType(entry.event_type)}
                    </span>
                    {!entry.chain_valid && (
                      <span className="px-2 py-0.5 text-xs font-medium rounded bg-red-100 text-red-800">
                        Chain Broken
                      </span>
                    )}
                  </div>

                  <p className="mt-1 text-sm text-gray-900">{entry.description}</p>

                  {/* Changes */}
                  {entry.changes && entry.changes.length > 0 && (
                    <div className="mt-2 space-y-1">
                      {entry.changes.map((change, idx) => (
                        <div key={idx} className="text-xs text-gray-600 flex items-center space-x-2">
                          <span className="font-medium">{change.field}:</span>
                          {change.old_value && (
                            <span className="line-through text-red-500">{change.old_value}</span>
                          )}
                          {change.old_value && change.new_value && <span>-&gt;</span>}
                          {change.new_value && (
                            <span className="text-green-600">{change.new_value}</span>
                          )}
                        </div>
                      ))}
                    </div>
                  )}

                  {/* User Info */}
                  <div className="mt-2 flex items-center space-x-4 text-xs text-gray-500">
                    {entry.user_name && (
                      <span className="flex items-center">
                        <svg
                          className="w-3 h-3 mr-1"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={2}
                            d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                          />
                        </svg>
                        {entry.user_name}
                      </span>
                    )}
                    {entry.ip_address && (
                      <span className="flex items-center">
                        <svg
                          className="w-3 h-3 mr-1"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={2}
                            d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"
                          />
                        </svg>
                        {entry.ip_address}
                      </span>
                    )}
                  </div>
                </div>
              </div>

              {/* Timestamp */}
              <div className="flex-shrink-0 text-right">
                <p className="text-sm text-gray-500">
                  {new Date(entry.created_at).toLocaleDateString()}
                </p>
                <p className="text-xs text-gray-400">
                  {new Date(entry.created_at).toLocaleTimeString()}
                </p>
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Load More */}
      {hasMore && onLoadMore && (
        <div className="px-6 py-4 border-t border-gray-200 text-center">
          <button
            type="button"
            onClick={onLoadMore}
            className="text-sm font-medium text-indigo-600 hover:text-indigo-700 transition-colors"
          >
            Load More Events
          </button>
        </div>
      )}

      {/* Empty State */}
      {entries.length === 0 && (
        <div className="px-6 py-12 text-center">
          <svg
            className="mx-auto h-12 w-12 text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
            />
          </svg>
          <h3 className="mt-2 text-sm font-medium text-gray-900">No audit events</h3>
          <p className="mt-1 text-sm text-gray-500">
            Audit events will appear here once configuration changes are made.
          </p>
        </div>
      )}
    </div>
  );
};
