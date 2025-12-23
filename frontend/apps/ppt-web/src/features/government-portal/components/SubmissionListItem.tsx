/**
 * SubmissionListItem - List item for regulatory submissions.
 * Epic 41: Government Portal UI (Story 41.2)
 */

import type { RegulatorySubmission } from '@ppt/api-client';
import { SubmissionStatusBadge } from './SubmissionStatusBadge';

interface SubmissionListItemProps {
  submission: RegulatorySubmission;
  onView: (id: string) => void;
  onRetry?: (id: string) => void;
}

export function SubmissionListItem({ submission, onView, onRetry }: SubmissionListItemProps) {
  const formatDate = (dateString: string | undefined) => {
    if (!dateString) return '-';
    return new Date(dateString).toLocaleDateString('en-GB', {
      day: '2-digit',
      month: 'short',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  const canRetry = submission.status === 'rejected' || submission.status === 'requires_correction';
  const hasError = submission.lastError && submission.lastError.length > 0;

  return (
    <div className="flex items-center justify-between rounded-lg border border-gray-200 bg-white p-4 hover:shadow-md transition-shadow">
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-3">
          <h3 className="text-sm font-medium text-gray-900 truncate">
            {submission.submissionReference}
          </h3>
          <SubmissionStatusBadge status={submission.status} />
        </div>
        <div className="mt-1 flex items-center gap-4 text-sm text-gray-500">
          <span>Type: {submission.reportType}</span>
          <span>
            Period: {formatDate(submission.reportPeriodStart)} -{' '}
            {formatDate(submission.reportPeriodEnd)}
          </span>
        </div>
        {hasError && (
          <div className="mt-2 flex items-start gap-2 rounded-md bg-red-50 p-2">
            <svg
              className="h-4 w-4 flex-shrink-0 text-red-500 mt-0.5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              aria-hidden="true"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            <div className="flex-1 min-w-0">
              <p className="text-xs text-red-700 truncate" title={submission.lastError}>
                {submission.lastError}
              </p>
              {submission.nextRetryAt && (
                <p className="text-xs text-red-500">
                  Next retry: {formatDate(submission.nextRetryAt)}
                </p>
              )}
            </div>
          </div>
        )}
        <div className="mt-2 text-xs text-gray-400">
          Created: {formatDate(submission.createdAt)}
          {submission.submittedAt && ` | Submitted: ${formatDate(submission.submittedAt)}`}
        </div>
      </div>
      <div className="flex items-center gap-2 ml-4">
        {canRetry && onRetry && (
          <button
            type="button"
            onClick={() => onRetry(submission.id)}
            className="inline-flex items-center gap-1.5 rounded-md bg-orange-50 px-3 py-1.5 text-sm font-medium text-orange-700 hover:bg-orange-100"
          >
            <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
              />
            </svg>
            Retry
          </button>
        )}
        <button
          type="button"
          onClick={() => onView(submission.id)}
          className="inline-flex items-center gap-1.5 rounded-md bg-gray-50 px-3 py-1.5 text-sm font-medium text-gray-700 hover:bg-gray-100"
        >
          <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
            />
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
            />
          </svg>
          View
        </button>
      </div>
    </div>
  );
}
