/**
 * SubmissionStatusPage - Government portal submission status tracking.
 * Epic 41: Government Portal UI (Story 41.2)
 *
 * Displays submission status with status badges, error details tooltip,
 * and retry button for failed submissions.
 */

import type {
  RegulatorySubmission,
  RegulatorySubmissionAttachment,
  RegulatorySubmissionAudit,
  SubmissionStatus,
} from '@ppt/api-client';
import { useState } from 'react';
import { SubmissionStatusBadge } from '../components/SubmissionStatusBadge';

interface SubmissionStatusPageProps {
  /** The submission to display */
  submission: RegulatorySubmission;
  /** Audit trail for the submission */
  auditTrail: RegulatorySubmissionAudit[];
  /** Attachments for the submission */
  attachments: RegulatorySubmissionAttachment[];
  /** Loading state */
  isLoading?: boolean;
  /** Is retry in progress */
  isRetrying?: boolean;
  /** Is validation in progress */
  isValidating?: boolean;
  /** Handler for retry submission */
  onRetry: (id: string) => Promise<void>;
  /** Handler for validate submission */
  onValidate: (id: string) => Promise<void>;
  /** Handler for cancel submission */
  onCancel: (id: string) => Promise<void>;
  /** Handler for navigating back */
  onBack: () => void;
}

export function SubmissionStatusPage({
  submission,
  auditTrail,
  attachments,
  isLoading = false,
  isRetrying = false,
  isValidating = false,
  onRetry,
  onValidate,
  onCancel,
  onBack,
}: SubmissionStatusPageProps) {
  const [showErrorDetails, setShowErrorDetails] = useState(false);
  const [showAuditTrail, setShowAuditTrail] = useState(false);
  const [cancelError, setCancelError] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);

  const formatDate = (dateString?: string) => {
    if (!dateString) return '—';
    return new Date(dateString).toLocaleDateString('en-GB', {
      day: '2-digit',
      month: 'short',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  const formatShortDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('en-GB', {
      day: '2-digit',
      month: 'short',
      year: 'numeric',
    });
  };

  const canRetry = submission.status === 'rejected' || submission.status === 'requires_correction';
  const canValidate = submission.status === 'draft' || submission.status === 'requires_correction';
  const canCancel = submission.status === 'draft' || submission.status === 'pending_validation';

  const handleRetry = async () => {
    setActionError(null);
    try {
      await onRetry(submission.id);
    } catch (err) {
      setActionError(err instanceof Error ? err.message : 'Retry failed');
    }
  };

  const handleValidate = async () => {
    setActionError(null);
    try {
      await onValidate(submission.id);
    } catch (err) {
      setActionError(err instanceof Error ? err.message : 'Validation failed');
    }
  };

  const handleCancel = async () => {
    setCancelError(null);
    try {
      await onCancel(submission.id);
    } catch (err) {
      setCancelError(err instanceof Error ? err.message : 'Cancel failed');
    }
  };

  const getStatusDescription = (status: SubmissionStatus): string => {
    const descriptions: Record<SubmissionStatus, string> = {
      draft: 'The submission is being prepared and has not been validated yet.',
      pending_validation: 'The submission is being validated against portal requirements.',
      validated: 'The submission passed validation and is ready to be sent.',
      submitted: 'The submission has been sent to the government portal.',
      acknowledged: 'The portal has acknowledged receipt of the submission.',
      processing: 'The portal is processing the submission.',
      accepted: 'The submission has been accepted by the portal.',
      rejected: 'The submission was rejected. Review the error details and retry.',
      requires_correction: 'The submission requires corrections before it can be accepted.',
      cancelled: 'The submission has been cancelled.',
    };
    return descriptions[status];
  };

  if (isLoading) {
    return (
      <div className="max-w-4xl mx-auto px-4 py-8">
        <div className="animate-pulse">
          <div className="h-4 w-32 bg-gray-200 rounded mb-4" />
          <div className="h-8 w-64 bg-gray-200 rounded mb-6" />
          <div className="space-y-4">
            <div className="h-24 bg-gray-100 rounded-lg" />
            <div className="h-24 bg-gray-100 rounded-lg" />
            <div className="h-24 bg-gray-100 rounded-lg" />
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-8">
        <button
          type="button"
          onClick={onBack}
          className="inline-flex items-center gap-2 text-gray-600 hover:text-gray-900 mb-4"
        >
          <svg
            className="h-4 w-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M15 19l-7-7 7-7"
            />
          </svg>
          Back to Dashboard
        </button>

        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold text-gray-900">Submission Status</h1>
            <p className="mt-1 text-gray-500">Reference: {submission.submissionReference}</p>
          </div>
          <SubmissionStatusBadge status={submission.status} size="lg" />
        </div>
      </div>

      {/* Error Alerts */}
      {(actionError || cancelError) && (
        <div className="mb-6 rounded-lg bg-red-50 border border-red-200 p-4">
          <div className="flex items-center gap-3">
            <svg
              className="h-5 w-5 text-red-500"
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
            <p className="text-red-700">{actionError || cancelError}</p>
          </div>
        </div>
      )}

      {/* Status Description */}
      <div className="mb-6 bg-gray-50 rounded-lg p-4">
        <p className="text-gray-700">{getStatusDescription(submission.status)}</p>
      </div>

      {/* Error Details Section (for rejected/requires_correction) */}
      {(submission.status === 'rejected' || submission.status === 'requires_correction') &&
        submission.lastError && (
          <div className="mb-6">
            <button
              type="button"
              onClick={() => setShowErrorDetails(!showErrorDetails)}
              className="w-full flex items-center justify-between p-4 bg-red-50 border border-red-200 rounded-lg text-left hover:bg-red-100 transition-colors"
            >
              <div className="flex items-center gap-3">
                <svg
                  className="h-5 w-5 text-red-500"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                  />
                </svg>
                <span className="font-medium text-red-900">Error Details</span>
              </div>
              <svg
                className={`h-5 w-5 text-red-500 transition-transform ${showErrorDetails ? 'rotate-180' : ''}`}
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                aria-hidden="true"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M19 9l-7 7-7-7"
                />
              </svg>
            </button>

            {showErrorDetails && (
              <div className="mt-2 p-4 bg-red-50 border border-red-200 border-t-0 rounded-b-lg">
                <pre className="text-sm text-red-800 whitespace-pre-wrap font-mono">
                  {submission.lastError}
                </pre>
                {submission.validationResult && !submission.validationResult.isValid && (
                  <div className="mt-4">
                    <h4 className="text-sm font-medium text-red-900 mb-2">Validation Errors:</h4>
                    <ul className="space-y-2">
                      {submission.validationResult.errors.map((error) => (
                        <li
                          key={`${error.field}-${error.code}`}
                          className="flex items-start gap-2 text-sm text-red-800"
                        >
                          <span className="text-red-500 mt-0.5">
                            <svg
                              className="h-4 w-4"
                              fill="none"
                              viewBox="0 0 24 24"
                              stroke="currentColor"
                              aria-hidden="true"
                            >
                              <path
                                strokeLinecap="round"
                                strokeLinejoin="round"
                                strokeWidth={2}
                                d="M6 18L18 6M6 6l12 12"
                              />
                            </svg>
                          </span>
                          <span>
                            <strong>{error.field}:</strong> {error.message} ({error.code})
                          </span>
                        </li>
                      ))}
                    </ul>
                  </div>
                )}
                {submission.validationResult?.warnings &&
                  submission.validationResult.warnings.length > 0 && (
                    <div className="mt-4">
                      <h4 className="text-sm font-medium text-yellow-900 mb-2">Warnings:</h4>
                      <ul className="space-y-2">
                        {submission.validationResult.warnings.map((warning) => (
                          <li
                            key={`${warning.field}-${warning.code}`}
                            className="flex items-start gap-2 text-sm text-yellow-800"
                          >
                            <span className="text-yellow-500 mt-0.5">
                              <svg
                                className="h-4 w-4"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                aria-hidden="true"
                              >
                                <path
                                  strokeLinecap="round"
                                  strokeLinejoin="round"
                                  strokeWidth={2}
                                  d="M12 9v2m0 4h.01"
                                />
                              </svg>
                            </span>
                            <span>
                              <strong>{warning.field}:</strong> {warning.message}
                            </span>
                          </li>
                        ))}
                      </ul>
                    </div>
                  )}
              </div>
            )}
          </div>
        )}

      {/* Submission Details */}
      <div className="bg-white rounded-lg border border-gray-200 p-6 mb-6">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">Submission Details</h2>
        <div className="grid grid-cols-2 md:grid-cols-3 gap-6">
          <div>
            <p className="text-sm font-medium text-gray-500">Report Type</p>
            <p className="text-gray-900">{submission.reportType}</p>
          </div>
          <div>
            <p className="text-sm font-medium text-gray-500">Report Period</p>
            <p className="text-gray-900">
              {formatShortDate(submission.reportPeriodStart)} -{' '}
              {formatShortDate(submission.reportPeriodEnd)}
            </p>
          </div>
          <div>
            <p className="text-sm font-medium text-gray-500">External Reference</p>
            <p className="text-gray-900">{submission.externalReference || '—'}</p>
          </div>
          <div>
            <p className="text-sm font-medium text-gray-500">Created</p>
            <p className="text-gray-900">{formatDate(submission.createdAt)}</p>
          </div>
          <div>
            <p className="text-sm font-medium text-gray-500">Validated</p>
            <p className="text-gray-900">{formatDate(submission.validatedAt)}</p>
          </div>
          <div>
            <p className="text-sm font-medium text-gray-500">Submitted</p>
            <p className="text-gray-900">{formatDate(submission.submittedAt)}</p>
          </div>
          <div>
            <p className="text-sm font-medium text-gray-500">Acknowledged</p>
            <p className="text-gray-900">{formatDate(submission.acknowledgedAt)}</p>
          </div>
          <div>
            <p className="text-sm font-medium text-gray-500">Processed</p>
            <p className="text-gray-900">{formatDate(submission.processedAt)}</p>
          </div>
          <div>
            <p className="text-sm font-medium text-gray-500">Submission Attempts</p>
            <p className="text-gray-900">{submission.submissionAttempts}</p>
          </div>
        </div>

        {/* Next Retry */}
        {submission.nextRetryAt && (
          <div className="mt-4 p-3 bg-yellow-50 rounded-lg">
            <p className="text-sm text-yellow-800">
              Next automatic retry scheduled: {formatDate(submission.nextRetryAt)}
            </p>
          </div>
        )}
      </div>

      {/* Report PDF */}
      {submission.reportPdfUrl && (
        <div className="bg-white rounded-lg border border-gray-200 p-6 mb-6">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">Report Document</h2>
          <a
            href={submission.reportPdfUrl}
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center gap-2 px-4 py-2 border border-gray-300 rounded-md text-gray-700 hover:bg-gray-50"
          >
            <svg
              className="h-5 w-5 text-red-500"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              aria-hidden="true"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"
              />
            </svg>
            View Report PDF
          </a>
        </div>
      )}

      {/* Attachments */}
      {attachments.length > 0 && (
        <div className="bg-white rounded-lg border border-gray-200 p-6 mb-6">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">
            Attachments ({attachments.length})
          </h2>
          <div className="space-y-2">
            {attachments.map((attachment) => (
              <a
                key={attachment.id}
                href={attachment.fileUrl}
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center justify-between p-3 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors"
              >
                <div className="flex items-center gap-3">
                  <svg
                    className="h-5 w-5 text-gray-400"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    aria-hidden="true"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13"
                    />
                  </svg>
                  <div>
                    <p className="text-sm font-medium text-gray-900">{attachment.fileName}</p>
                    <p className="text-xs text-gray-500">
                      {attachment.attachmentType} • {Math.round(attachment.fileSize / 1024)} KB
                    </p>
                  </div>
                </div>
                <svg
                  className="h-4 w-4 text-gray-400"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
                  />
                </svg>
              </a>
            ))}
          </div>
        </div>
      )}

      {/* Audit Trail */}
      <div className="bg-white rounded-lg border border-gray-200 overflow-hidden mb-6">
        <button
          type="button"
          onClick={() => setShowAuditTrail(!showAuditTrail)}
          className="w-full flex items-center justify-between p-4 text-left hover:bg-gray-50 transition-colors"
        >
          <h2 className="text-lg font-semibold text-gray-900">
            Audit Trail ({auditTrail.length} events)
          </h2>
          <svg
            className={`h-5 w-5 text-gray-400 transition-transform ${showAuditTrail ? 'rotate-180' : ''}`}
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            aria-hidden="true"
          >
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
          </svg>
        </button>

        {showAuditTrail && (
          <div className="border-t border-gray-200">
            {auditTrail.length === 0 ? (
              <div className="p-4 text-center text-gray-500">No audit events recorded.</div>
            ) : (
              <div className="divide-y divide-gray-100">
                {auditTrail.map((event) => (
                  <div key={event.id} className="p-4 hover:bg-gray-50">
                    <div className="flex items-start justify-between">
                      <div>
                        <p className="font-medium text-gray-900">{event.action}</p>
                        {event.previousStatus && event.newStatus && (
                          <p className="text-sm text-gray-500">
                            {event.previousStatus} &rarr; {event.newStatus}
                          </p>
                        )}
                        {event.errorMessage && (
                          <p className="text-sm text-red-600 mt-1">{event.errorMessage}</p>
                        )}
                      </div>
                      <div className="text-right">
                        <p className="text-sm text-gray-500">{formatDate(event.createdAt)}</p>
                        <p className="text-xs text-gray-400">{event.actorType}</p>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}
      </div>

      {/* Actions */}
      <div className="flex items-center justify-between border-t border-gray-200 pt-6">
        <div className="flex items-center gap-3">
          {canCancel && (
            <button
              type="button"
              onClick={handleCancel}
              className="px-4 py-2 text-red-600 hover:text-red-800 hover:bg-red-50 rounded-md transition-colors"
            >
              Cancel Submission
            </button>
          )}
        </div>

        <div className="flex items-center gap-3">
          {canValidate && (
            <button
              type="button"
              onClick={handleValidate}
              disabled={isValidating}
              className="inline-flex items-center gap-2 px-4 py-2 border border-gray-300 rounded-md text-gray-700 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isValidating ? (
                <>
                  <svg
                    className="h-4 w-4 animate-spin"
                    fill="none"
                    viewBox="0 0 24 24"
                    aria-hidden="true"
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
                      d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
                    />
                  </svg>
                  Validating...
                </>
              ) : (
                <>
                  <svg
                    className="h-4 w-4"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    aria-hidden="true"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                  </svg>
                  Validate
                </>
              )}
            </button>
          )}

          {canRetry && (
            <button
              type="button"
              onClick={handleRetry}
              disabled={isRetrying}
              className="inline-flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isRetrying ? (
                <>
                  <svg
                    className="h-4 w-4 animate-spin"
                    fill="none"
                    viewBox="0 0 24 24"
                    aria-hidden="true"
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
                      d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
                    />
                  </svg>
                  Retrying...
                </>
              ) : (
                <>
                  <svg
                    className="h-4 w-4"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    aria-hidden="true"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                    />
                  </svg>
                  Retry Submission
                </>
              )}
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
