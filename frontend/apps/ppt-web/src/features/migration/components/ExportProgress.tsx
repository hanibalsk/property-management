/**
 * Export Progress Component (Story 66.3).
 *
 * Displays progress of a migration export with download when ready.
 */

import { useEffect, useState } from 'react';
import type { ExportDataCategory } from './ExportCategorySelector';

export type ExportStatus = 'pending' | 'processing' | 'ready' | 'downloaded' | 'expired' | 'failed';

export interface ExportStatusData {
  exportId: string;
  status: ExportStatus;
  categories: ExportDataCategory[];
  downloadUrl?: string;
  fileSizeBytes?: number;
  expiresAt: string;
  errorMessage?: string;
  recordCounts?: Record<string, number>;
}

interface ExportProgressProps {
  exportId: string;
  initialStatus?: ExportStatusData;
  onComplete?: () => void;
  onDownload?: () => void;
  pollInterval?: number;
}

const STATUS_LABELS: Record<ExportStatus, string> = {
  pending: 'Preparing export...',
  processing: 'Generating export files...',
  ready: 'Export ready',
  downloaded: 'Downloaded',
  expired: 'Expired',
  failed: 'Export failed',
};

export function ExportProgress({
  exportId,
  initialStatus,
  onComplete,
  onDownload,
  pollInterval = 3000,
}: ExportProgressProps) {
  const [status, setStatus] = useState<ExportStatusData | null>(initialStatus ?? null);
  const [isPolling, setIsPolling] = useState(true);

  useEffect(() => {
    if (!isPolling) return;

    const isTerminalStatus = (s: ExportStatus) =>
      ['ready', 'downloaded', 'expired', 'failed'].includes(s);

    const fetchStatus = async () => {
      try {
        // In a real implementation:
        // const response = await fetch(`/api/v1/migration/export/${exportId}`);
        // const data = await response.json();
        // setStatus(data);

        // Simulate status updates
        setStatus((prev) => {
          if (!prev) {
            return {
              exportId,
              status: 'processing',
              categories: ['buildings', 'units', 'residents'] as ExportDataCategory[],
              expiresAt: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000).toISOString(),
            };
          }

          if (prev.status === 'pending') {
            return { ...prev, status: 'processing' };
          }

          if (prev.status === 'processing') {
            // Randomly complete after a few polls
            if (Math.random() > 0.7) {
              return {
                ...prev,
                status: 'ready',
                downloadUrl: `/api/v1/migration/export/${exportId}/download`,
                fileSizeBytes: 15234567,
                recordCounts: {
                  buildings: 45,
                  units: 320,
                  residents: 580,
                },
              };
            }
          }

          return prev;
        });
      } catch (error) {
        console.error('Failed to fetch export status:', error);
      }
    };

    fetchStatus();

    const interval = setInterval(() => {
      if (status && isTerminalStatus(status.status)) {
        setIsPolling(false);
        if (status.status === 'ready' && onComplete) {
          onComplete();
        }
        return;
      }
      fetchStatus();
    }, pollInterval);

    return () => clearInterval(interval);
  }, [exportId, pollInterval, isPolling, status, onComplete]);

  const handleDownload = () => {
    if (status?.downloadUrl) {
      // In a real implementation, trigger download
      window.open(status.downloadUrl, '_blank');
      setStatus((prev) => (prev ? { ...prev, status: 'downloaded' } : prev));
      onDownload?.();
    }
  };

  const formatFileSize = (bytes: number): string => {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
  };

  const formatExpirationTime = (expiresAt: string): string => {
    const expires = new Date(expiresAt);
    const now = new Date();
    const diffMs = expires.getTime() - now.getTime();
    const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));
    const diffHours = Math.floor((diffMs % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));

    if (diffDays > 0) return `${diffDays} days`;
    if (diffHours > 0) return `${diffHours} hours`;
    return 'Less than an hour';
  };

  if (!status) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="h-8 w-8 animate-spin rounded-full border-2 border-blue-600 border-t-transparent" />
      </div>
    );
  }

  const isProcessing = ['pending', 'processing'].includes(status.status);
  const isReady = status.status === 'ready' || status.status === 'downloaded';
  const isFailed = status.status === 'failed';
  const isExpired = status.status === 'expired';

  return (
    <div className="space-y-6">
      {/* Status Header */}
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-medium text-gray-900">Migration Export</h2>
        <span
          className={`inline-flex items-center rounded-full px-3 py-1 text-sm font-medium ${
            isProcessing
              ? 'bg-blue-100 text-blue-800'
              : isReady
                ? 'bg-green-100 text-green-800'
                : isFailed
                  ? 'bg-red-100 text-red-800'
                  : 'bg-gray-100 text-gray-800'
          }`}
        >
          {isProcessing && (
            <svg className="-ml-0.5 mr-1.5 h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
              <circle
                className="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                strokeWidth="4"
              />
              <path
                className="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
              />
            </svg>
          )}
          {STATUS_LABELS[status.status]}
        </span>
      </div>

      {/* Processing Animation */}
      {isProcessing && (
        <div className="rounded-lg bg-blue-50 p-6 text-center">
          <div className="mx-auto h-16 w-16 animate-pulse rounded-full bg-blue-100 p-4">
            <svg
              className="h-8 w-8 text-blue-600"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={1.5}
                d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
              />
            </svg>
          </div>
          <p className="mt-4 text-sm text-blue-700">
            Generating export files for {status.categories.length} categories...
          </p>
          <p className="mt-1 text-xs text-blue-600">
            This may take a few minutes for large datasets.
          </p>
        </div>
      )}

      {/* Ready State */}
      {isReady && (
        <div className="rounded-lg bg-green-50 p-6">
          <div className="flex items-start gap-4">
            <div className="flex h-12 w-12 items-center justify-center rounded-full bg-green-100">
              <svg
                className="h-6 w-6 text-green-600"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M5 13l4 4L19 7"
                />
              </svg>
            </div>
            <div className="flex-1">
              <h3 className="text-sm font-medium text-green-800">Export Ready for Download</h3>
              <div className="mt-2 flex flex-wrap gap-4 text-sm text-green-700">
                {status.fileSizeBytes && <span>Size: {formatFileSize(status.fileSizeBytes)}</span>}
                <span>Expires in: {formatExpirationTime(status.expiresAt)}</span>
              </div>
              <button
                type="button"
                onClick={handleDownload}
                className="mt-4 inline-flex items-center rounded-md bg-green-600 px-4 py-2 text-sm font-medium text-white hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2"
              >
                <svg
                  className="-ml-0.5 mr-2 h-4 w-4"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                  />
                </svg>
                Download Export
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Record Counts */}
      {status.recordCounts && (
        <div className="rounded-lg border border-gray-200 bg-white p-4">
          <h3 className="text-sm font-medium text-gray-900">Exported Records</h3>
          <div className="mt-3 grid gap-2 sm:grid-cols-2 lg:grid-cols-3">
            {Object.entries(status.recordCounts).map(([category, count]) => (
              <div
                key={category}
                className="flex items-center justify-between rounded bg-gray-50 px-3 py-2"
              >
                <span className="text-sm capitalize text-gray-600">
                  {category.replace('_', ' ')}
                </span>
                <span className="font-medium text-gray-900">{count.toLocaleString()}</span>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Failed State */}
      {isFailed && (
        <div className="rounded-lg border border-red-200 bg-red-50 p-4">
          <div className="flex">
            <svg
              className="h-5 w-5 text-red-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            <div className="ml-3">
              <h3 className="text-sm font-medium text-red-800">Export Failed</h3>
              <p className="mt-1 text-sm text-red-700">
                {status.errorMessage || 'An error occurred while generating the export.'}
              </p>
            </div>
          </div>
        </div>
      )}

      {/* Expired State */}
      {isExpired && (
        <div className="rounded-lg border border-gray-200 bg-gray-50 p-4">
          <div className="flex">
            <svg
              className="h-5 w-5 text-gray-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            <div className="ml-3">
              <h3 className="text-sm font-medium text-gray-800">Export Expired</h3>
              <p className="mt-1 text-sm text-gray-600">
                This export has expired. Please create a new export request.
              </p>
            </div>
          </div>
        </div>
      )}

      {/* Categories List */}
      <div className="rounded-lg border border-gray-200 bg-white p-4">
        <h3 className="text-sm font-medium text-gray-900">Included Categories</h3>
        <div className="mt-2 flex flex-wrap gap-2">
          {status.categories.map((category) => (
            <span
              key={category}
              className="rounded bg-gray-100 px-2 py-1 text-sm capitalize text-gray-600"
            >
              {category.replace('_', ' ')}
            </span>
          ))}
        </div>
      </div>
    </div>
  );
}
