/**
 * Import Job List Component (Story 66.2).
 *
 * Displays history of import jobs with status and actions.
 */

import type { ImportJobStatus } from './ImportJobProgress';
import type { ImportDataType } from './ImportTemplateBuilder';

export interface ImportJobHistoryItem {
  id: string;
  status: ImportJobStatus;
  filename: string;
  dataType: ImportDataType;
  recordsImported: number;
  recordsFailed: number;
  createdByName: string;
  createdAt: string;
  completedAt?: string;
}

interface ImportJobListProps {
  jobs: ImportJobHistoryItem[];
  isLoading?: boolean;
  onViewJob: (job: ImportJobHistoryItem) => void;
  onRetryJob: (job: ImportJobHistoryItem) => void;
  onViewErrors: (job: ImportJobHistoryItem) => void;
  statusFilter?: ImportJobStatus;
  onStatusFilterChange?: (status: ImportJobStatus | undefined) => void;
}

const STATUS_LABELS: Record<ImportJobStatus, string> = {
  pending: 'Pending',
  validating: 'Validating',
  validated: 'Validated',
  validation_failed: 'Validation Failed',
  importing: 'Importing',
  completed: 'Completed',
  partially_completed: 'Partial',
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

const DATA_TYPE_LABELS: Record<ImportDataType, string> = {
  buildings: 'Buildings',
  units: 'Units',
  residents: 'Residents',
  financials: 'Financials',
  faults: 'Faults',
  documents: 'Documents',
  meters: 'Meters',
  votes: 'Votes',
  custom: 'Custom',
};

export function ImportJobList({
  jobs,
  isLoading = false,
  onViewJob,
  onRetryJob,
  onViewErrors,
  statusFilter,
  onStatusFilterChange,
}: ImportJobListProps) {
  const formatDate = (dateString: string): string => {
    const date = new Date(dateString);
    return new Intl.DateTimeFormat('en-US', {
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    }).format(date);
  };

  const formatRelativeTime = (dateString: string): string => {
    const date = new Date(dateString);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMins / 60);
    const diffDays = Math.floor(diffHours / 24);

    if (diffMins < 1) return 'Just now';
    if (diffMins < 60) return `${diffMins}m ago`;
    if (diffHours < 24) return `${diffHours}h ago`;
    if (diffDays < 7) return `${diffDays}d ago`;
    return formatDate(dateString);
  };

  return (
    <div className="space-y-4">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-medium text-gray-900">Import History</h2>
        <div className="flex items-center gap-2">
          <label className="text-sm text-gray-500">Filter:</label>
          <select
            value={statusFilter ?? ''}
            onChange={(e) =>
              onStatusFilterChange?.((e.target.value as ImportJobStatus) || undefined)
            }
            className="rounded-md border-gray-300 text-sm shadow-sm focus:border-blue-500 focus:ring-blue-500"
          >
            <option value="">All statuses</option>
            {Object.entries(STATUS_LABELS).map(([value, label]) => (
              <option key={value} value={value}>
                {label}
              </option>
            ))}
          </select>
        </div>
      </div>

      {/* Job List */}
      {isLoading ? (
        <div className="flex items-center justify-center py-12">
          <div className="h-8 w-8 animate-spin rounded-full border-2 border-blue-600 border-t-transparent" />
        </div>
      ) : jobs.length === 0 ? (
        <div className="rounded-lg border-2 border-dashed border-gray-300 p-12 text-center">
          <svg
            className="mx-auto h-12 w-12 text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={1.5}
              d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
            />
          </svg>
          <h3 className="mt-4 text-sm font-medium text-gray-900">No import jobs</h3>
          <p className="mt-1 text-sm text-gray-500">
            Import jobs will appear here once you start importing data.
          </p>
        </div>
      ) : (
        <div className="overflow-hidden rounded-lg border border-gray-200 bg-white">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-4 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                  File
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                  Type
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                  Status
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                  Records
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                  Date
                </th>
                <th className="px-4 py-3 text-right text-xs font-medium uppercase tracking-wider text-gray-500">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-200">
              {jobs.map((job) => (
                <tr key={job.id} className="hover:bg-gray-50">
                  <td className="whitespace-nowrap px-4 py-3">
                    <div>
                      <p className="font-medium text-gray-900">{job.filename}</p>
                      <p className="text-xs text-gray-500">by {job.createdByName}</p>
                    </div>
                  </td>
                  <td className="whitespace-nowrap px-4 py-3">
                    <span className="rounded bg-gray-100 px-2 py-0.5 text-xs text-gray-600">
                      {DATA_TYPE_LABELS[job.dataType]}
                    </span>
                  </td>
                  <td className="whitespace-nowrap px-4 py-3">
                    <span
                      className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${STATUS_COLORS[job.status]}`}
                    >
                      {STATUS_LABELS[job.status]}
                    </span>
                  </td>
                  <td className="whitespace-nowrap px-4 py-3 text-sm">
                    <span className="text-green-600">{job.recordsImported}</span>
                    {job.recordsFailed > 0 && (
                      <>
                        {' / '}
                        <span className="text-red-600">{job.recordsFailed} failed</span>
                      </>
                    )}
                  </td>
                  <td className="whitespace-nowrap px-4 py-3 text-sm text-gray-500">
                    <span title={formatDate(job.createdAt)}>
                      {formatRelativeTime(job.createdAt)}
                    </span>
                  </td>
                  <td className="whitespace-nowrap px-4 py-3 text-right">
                    <div className="flex items-center justify-end gap-2">
                      <button
                        type="button"
                        onClick={() => onViewJob(job)}
                        className="text-sm font-medium text-blue-600 hover:text-blue-700"
                      >
                        View
                      </button>
                      {job.recordsFailed > 0 && (
                        <button
                          type="button"
                          onClick={() => onViewErrors(job)}
                          className="text-sm font-medium text-red-600 hover:text-red-700"
                        >
                          Errors
                        </button>
                      )}
                      {['failed', 'cancelled'].includes(job.status) && (
                        <button
                          type="button"
                          onClick={() => onRetryJob(job)}
                          className="text-sm font-medium text-gray-600 hover:text-gray-700"
                        >
                          Retry
                        </button>
                      )}
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
