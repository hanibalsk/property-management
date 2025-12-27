/**
 * Import Job Progress Component (Story 66.2).
 *
 * Displays real-time progress of an import job with status updates.
 */

import { useEffect, useState } from 'react';

export type ImportJobStatus =
  | 'pending'
  | 'validating'
  | 'validated'
  | 'validation_failed'
  | 'importing'
  | 'completed'
  | 'partially_completed'
  | 'failed'
  | 'cancelled';

export interface ImportRowError {
  rowNumber: number;
  column?: string;
  message: string;
  errorCode: string;
  originalValue?: string;
}

export interface ImportJobStatusData {
  id: string;
  status: ImportJobStatus;
  filename: string;
  templateName: string;
  progressPercent: number;
  totalRows?: number;
  processedRows: number;
  successfulRows: number;
  failedRows: number;
  skippedRows: number;
  errorSummary?: ImportRowError[];
  startedAt?: string;
  completedAt?: string;
  estimatedRemainingSeconds?: number;
}

interface ImportJobProgressProps {
  jobId: string;
  initialStatus?: ImportJobStatusData;
  onComplete?: (status: ImportJobStatusData) => void;
  onCancel?: () => void;
  onViewErrors?: () => void;
  onRetry?: () => void;
  pollInterval?: number;
}

const STATUS_LABELS: Record<ImportJobStatus, string> = {
  pending: 'Pending',
  validating: 'Validating...',
  validated: 'Validated',
  validation_failed: 'Validation Failed',
  importing: 'Importing...',
  completed: 'Completed',
  partially_completed: 'Partially Completed',
  failed: 'Failed',
  cancelled: 'Cancelled',
};

const STATUS_COLORS: Record<ImportJobStatus, string> = {
  pending: 'bg-gray-100 text-gray-800',
  validating: 'bg-blue-100 text-blue-800',
  validated: 'bg-green-100 text-green-800',
  validation_failed: 'bg-red-100 text-red-800',
  importing: 'bg-blue-100 text-blue-800',
  completed: 'bg-green-100 text-green-800',
  partially_completed: 'bg-yellow-100 text-yellow-800',
  failed: 'bg-red-100 text-red-800',
  cancelled: 'bg-gray-100 text-gray-800',
};

export function ImportJobProgress({
  jobId,
  initialStatus,
  onComplete,
  onCancel,
  onViewErrors,
  onRetry,
  pollInterval = 2000,
}: ImportJobProgressProps) {
  const [status, setStatus] = useState<ImportJobStatusData | null>(initialStatus ?? null);
  const [isPolling, setIsPolling] = useState(true);

  // Poll for status updates
  useEffect(() => {
    if (!isPolling) return;

    const isTerminalStatus = (s: ImportJobStatus) =>
      ['completed', 'partially_completed', 'failed', 'cancelled', 'validated', 'validation_failed'].includes(s);

    const fetchStatus = async () => {
      try {
        // In a real implementation:
        // const response = await fetch(`/api/v1/migration/import/jobs/${jobId}`);
        // const data = await response.json();
        // setStatus(data);

        // Simulate status updates
        setStatus((prev) => {
          if (!prev) {
            return {
              id: jobId,
              status: 'validating',
              filename: 'import_data.csv',
              templateName: 'Buildings Import',
              progressPercent: 0,
              totalRows: 150,
              processedRows: 0,
              successfulRows: 0,
              failedRows: 0,
              skippedRows: 0,
              startedAt: new Date().toISOString(),
            };
          }

          // Simulate progress
          const newProgress = Math.min(prev.progressPercent + Math.random() * 15, 100);
          const newProcessed = Math.floor((newProgress / 100) * (prev.totalRows ?? 100));
          const newFailed = Math.floor(newProcessed * 0.02); // 2% failure rate
          const newSuccessful = newProcessed - newFailed;

          let newStatus = prev.status;
          if (newProgress >= 100) {
            newStatus = newFailed > 0 ? 'partially_completed' : 'completed';
          } else if (prev.status === 'validating' && newProgress > 30) {
            newStatus = 'importing';
          }

          return {
            ...prev,
            status: newStatus,
            progressPercent: Math.round(newProgress),
            processedRows: newProcessed,
            successfulRows: newSuccessful,
            failedRows: newFailed,
            completedAt: newProgress >= 100 ? new Date().toISOString() : undefined,
            estimatedRemainingSeconds:
              newProgress < 100 ? Math.round(((100 - newProgress) / 10) * 10) : undefined,
          };
        });
      } catch (error) {
        console.error('Failed to fetch job status:', error);
      }
    };

    fetchStatus();

    const interval = setInterval(() => {
      if (status && isTerminalStatus(status.status)) {
        setIsPolling(false);
        if (onComplete) {
          onComplete(status);
        }
        return;
      }
      fetchStatus();
    }, pollInterval);

    return () => clearInterval(interval);
  }, [jobId, pollInterval, isPolling, status, onComplete]);

  if (!status) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="h-8 w-8 animate-spin rounded-full border-2 border-blue-600 border-t-transparent" />
      </div>
    );
  }

  const isInProgress = ['pending', 'validating', 'importing'].includes(status.status);
  const isSuccess = ['completed', 'validated'].includes(status.status);
  const isPartial = status.status === 'partially_completed';
  const isFailed = ['failed', 'validation_failed', 'cancelled'].includes(status.status);

  const formatTime = (seconds: number): string => {
    if (seconds < 60) return `${seconds}s`;
    const minutes = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${minutes}m ${secs}s`;
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-start justify-between">
        <div>
          <h2 className="text-lg font-medium text-gray-900">{status.filename}</h2>
          <p className="mt-1 text-sm text-gray-500">
            Template: {status.templateName}
          </p>
        </div>
        <span
          className={`inline-flex items-center rounded-full px-3 py-1 text-sm font-medium ${STATUS_COLORS[status.status]}`}
        >
          {isInProgress && (
            <svg
              className="-ml-0.5 mr-1.5 h-4 w-4 animate-spin"
              fill="none"
              viewBox="0 0 24 24"
            >
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

      {/* Progress Bar */}
      <div>
        <div className="flex items-center justify-between text-sm">
          <span className="text-gray-600">Progress</span>
          <span className="font-medium text-gray-900">{status.progressPercent}%</span>
        </div>
        <div className="mt-2 h-3 overflow-hidden rounded-full bg-gray-200">
          <div
            className={`h-full transition-all duration-500 ${
              isFailed ? 'bg-red-500' : isPartial ? 'bg-yellow-500' : 'bg-blue-600'
            }`}
            style={{ width: `${status.progressPercent}%` }}
          />
        </div>
        {status.estimatedRemainingSeconds && (
          <p className="mt-1 text-xs text-gray-500">
            Estimated time remaining: {formatTime(status.estimatedRemainingSeconds)}
          </p>
        )}
      </div>

      {/* Statistics */}
      <div className="grid grid-cols-2 gap-4 sm:grid-cols-4">
        <div className="rounded-lg bg-gray-50 p-3">
          <p className="text-xs text-gray-500">Total Rows</p>
          <p className="mt-1 text-lg font-semibold text-gray-900">
            {status.totalRows?.toLocaleString() ?? '-'}
          </p>
        </div>
        <div className="rounded-lg bg-gray-50 p-3">
          <p className="text-xs text-gray-500">Processed</p>
          <p className="mt-1 text-lg font-semibold text-gray-900">
            {status.processedRows.toLocaleString()}
          </p>
        </div>
        <div className="rounded-lg bg-green-50 p-3">
          <p className="text-xs text-green-600">Successful</p>
          <p className="mt-1 text-lg font-semibold text-green-700">
            {status.successfulRows.toLocaleString()}
          </p>
        </div>
        <div className="rounded-lg bg-red-50 p-3">
          <p className="text-xs text-red-600">Failed</p>
          <p className="mt-1 text-lg font-semibold text-red-700">
            {status.failedRows.toLocaleString()}
          </p>
        </div>
      </div>

      {/* Error Summary */}
      {status.errorSummary && status.errorSummary.length > 0 && (
        <div className="rounded-lg border border-red-200 bg-red-50 p-4">
          <h3 className="text-sm font-medium text-red-800">Error Summary</h3>
          <ul className="mt-2 space-y-1">
            {status.errorSummary.slice(0, 3).map((error, index) => (
              <li key={index} className="text-sm text-red-700">
                Row {error.rowNumber}: {error.message}
                {error.column && ` (${error.column})`}
              </li>
            ))}
          </ul>
          {status.failedRows > 3 && onViewErrors && (
            <button
              type="button"
              onClick={onViewErrors}
              className="mt-2 text-sm font-medium text-red-800 hover:text-red-900"
            >
              View all {status.failedRows} errors
            </button>
          )}
        </div>
      )}

      {/* Success Message */}
      {isSuccess && (
        <div className="rounded-lg border border-green-200 bg-green-50 p-4">
          <div className="flex">
            <svg
              className="h-5 w-5 text-green-400"
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
            <div className="ml-3">
              <h3 className="text-sm font-medium text-green-800">Import Completed</h3>
              <p className="mt-1 text-sm text-green-700">
                Successfully imported {status.successfulRows.toLocaleString()} records.
              </p>
            </div>
          </div>
        </div>
      )}

      {/* Partial Completion Message */}
      {isPartial && (
        <div className="rounded-lg border border-yellow-200 bg-yellow-50 p-4">
          <div className="flex">
            <svg
              className="h-5 w-5 text-yellow-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
              />
            </svg>
            <div className="ml-3">
              <h3 className="text-sm font-medium text-yellow-800">Import Partially Completed</h3>
              <p className="mt-1 text-sm text-yellow-700">
                Imported {status.successfulRows.toLocaleString()} records.{' '}
                {status.failedRows} rows failed.
              </p>
            </div>
          </div>
        </div>
      )}

      {/* Actions */}
      <div className="flex justify-end gap-3 border-t border-gray-200 pt-4">
        {isInProgress && onCancel && (
          <button
            type="button"
            onClick={onCancel}
            className="rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
          >
            Cancel Import
          </button>
        )}
        {isFailed && onRetry && (
          <button
            type="button"
            onClick={onRetry}
            className="inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
          >
            Retry Import
          </button>
        )}
        {status.failedRows > 0 && onViewErrors && (
          <button
            type="button"
            onClick={onViewErrors}
            className="inline-flex items-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
          >
            View Errors
          </button>
        )}
      </div>
    </div>
  );
}
