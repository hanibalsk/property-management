/**
 * ComplianceDashboardPage - Main compliance dashboard page.
 * Epic 41: Government Portal UI (Story 41.3)
 */

import type { GovernmentPortalStats, RegulatorySubmission } from '@ppt/api-client';
import { useState } from 'react';
import { ComplianceDashboardWidget } from '../components/ComplianceDashboardWidget';
import { DeadlineCountdown } from '../components/DeadlineCountdown';
import { SubmissionStatusBadge } from '../components/SubmissionStatusBadge';

interface ComplianceDashboardPageProps {
  stats: GovernmentPortalStats;
  pendingSubmissions: RegulatorySubmission[];
  isLoading?: boolean;
  onNavigateToSubmission: (id: string) => void;
  onNavigateToNewSubmission: () => void;
  onBatchSubmit: (submissionIds: string[]) => Promise<void>;
}

export function ComplianceDashboardPage({
  stats,
  pendingSubmissions,
  isLoading = false,
  onNavigateToSubmission,
  onNavigateToNewSubmission,
  onBatchSubmit,
}: ComplianceDashboardPageProps) {
  const [selectedSubmissions, setSelectedSubmissions] = useState<string[]>([]);
  const [isBatchSubmitting, setIsBatchSubmitting] = useState(false);
  const [batchError, setBatchError] = useState<string | null>(null);

  const handleSelectSubmission = (id: string) => {
    setSelectedSubmissions((prev) =>
      prev.includes(id) ? prev.filter((s) => s !== id) : [...prev, id]
    );
  };

  const handleSelectAll = () => {
    const validatedIds = pendingSubmissions
      .filter((s) => s.status === 'validated')
      .map((s) => s.id);
    if (selectedSubmissions.length === validatedIds.length) {
      setSelectedSubmissions([]);
    } else {
      setSelectedSubmissions(validatedIds);
    }
  };

  const handleBatchSubmit = async () => {
    if (selectedSubmissions.length === 0) return;

    setIsBatchSubmitting(true);
    setBatchError(null);

    try {
      await onBatchSubmit(selectedSubmissions);
      setSelectedSubmissions([]);
    } catch (err) {
      setBatchError(err instanceof Error ? err.message : 'Batch submission failed');
    } finally {
      setIsBatchSubmitting(false);
    }
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('en-GB', {
      day: '2-digit',
      month: 'short',
      year: 'numeric',
    });
  };

  const validatedSubmissions = pendingSubmissions.filter((s) => s.status === 'validated');

  if (isLoading) {
    return (
      <div className="animate-pulse max-w-6xl mx-auto p-6">
        <div className="h-8 w-64 bg-gray-200 rounded mb-6" />
        <div className="grid grid-cols-3 gap-4 mb-8">
          {[...Array(6)].map((_, i) => (
            <div key={i} className="h-24 bg-gray-100 rounded-lg" />
          ))}
        </div>
        <div className="h-64 bg-gray-100 rounded-lg" />
      </div>
    );
  }

  return (
    <div className="max-w-6xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-8 flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Compliance Dashboard</h1>
          <p className="mt-1 text-gray-500">
            Monitor government portal submissions and compliance status
          </p>
        </div>
        <button
          type="button"
          onClick={onNavigateToNewSubmission}
          className="inline-flex items-center gap-2 rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
        >
          <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
          </svg>
          New Submission
        </button>
      </div>

      {/* Statistics */}
      <section className="mb-8">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">Overview</h2>
        <ComplianceDashboardWidget stats={stats} />
      </section>

      {/* Upcoming Deadlines */}
      {stats.upcomingDueDates.length > 0 && (
        <section className="mb-8">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">Upcoming Deadlines</h2>
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            {stats.upcomingDueDates.map((deadline) => (
              <DeadlineCountdown
                key={deadline.scheduleId}
                dueDate={deadline.dueDate}
                templateName={deadline.templateName}
                daysUntilDue={deadline.daysUntilDue}
              />
            ))}
          </div>
        </section>
      )}

      {/* Pending Submissions with Batch Submit */}
      <section>
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-lg font-semibold text-gray-900">
            Pending Submissions ({pendingSubmissions.length})
          </h2>
          {validatedSubmissions.length > 0 && (
            <div className="flex items-center gap-3">
              <label className="flex items-center gap-2 text-sm text-gray-600">
                <input
                  type="checkbox"
                  checked={
                    selectedSubmissions.length === validatedSubmissions.length &&
                    validatedSubmissions.length > 0
                  }
                  onChange={handleSelectAll}
                  className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                Select all validated
              </label>
              <button
                type="button"
                onClick={handleBatchSubmit}
                disabled={selectedSubmissions.length === 0 || isBatchSubmitting}
                className="inline-flex items-center gap-2 rounded-md bg-green-600 px-4 py-2 text-sm font-medium text-white hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isBatchSubmitting ? (
                  <>
                    <svg className="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
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
                    Submitting...
                  </>
                ) : (
                  <>
                    <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                      />
                    </svg>
                    Submit Selected ({selectedSubmissions.length})
                  </>
                )}
              </button>
            </div>
          )}
        </div>

        {/* Batch Error */}
        {batchError && (
          <div className="mb-4 rounded-lg bg-red-50 border border-red-200 p-4">
            <div className="flex items-center gap-3">
              <svg
                className="h-5 w-5 text-red-500"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                />
              </svg>
              <p className="text-red-700">{batchError}</p>
            </div>
          </div>
        )}

        {pendingSubmissions.length === 0 ? (
          <div className="rounded-lg border-2 border-dashed border-gray-300 p-12 text-center">
            <svg
              className="mx-auto h-12 w-12 text-gray-400"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            <h3 className="mt-4 text-lg font-medium text-gray-900">All caught up!</h3>
            <p className="mt-1 text-gray-500">No pending submissions at this time.</p>
          </div>
        ) : (
          <div className="overflow-hidden rounded-lg border border-gray-200">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="w-12 px-4 py-3" />
                  <th className="px-4 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                    Reference
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                    Type
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                    Period
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                    Status
                  </th>
                  <th className="px-4 py-3 text-right text-xs font-medium uppercase tracking-wider text-gray-500">
                    Actions
                  </th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-200 bg-white">
                {pendingSubmissions.map((submission) => {
                  const isValidated = submission.status === 'validated';
                  const isSelected = selectedSubmissions.includes(submission.id);

                  return (
                    <tr
                      key={submission.id}
                      className={isSelected ? 'bg-blue-50' : 'hover:bg-gray-50'}
                    >
                      <td className="px-4 py-3">
                        {isValidated && (
                          <input
                            type="checkbox"
                            checked={isSelected}
                            onChange={() => handleSelectSubmission(submission.id)}
                            className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                          />
                        )}
                      </td>
                      <td className="px-4 py-3">
                        <button
                          type="button"
                          onClick={() => onNavigateToSubmission(submission.id)}
                          className="font-medium text-blue-600 hover:text-blue-800"
                        >
                          {submission.submissionReference}
                        </button>
                      </td>
                      <td className="px-4 py-3 text-sm text-gray-900">{submission.reportType}</td>
                      <td className="px-4 py-3 text-sm text-gray-500">
                        {formatDate(submission.reportPeriodStart)} -{' '}
                        {formatDate(submission.reportPeriodEnd)}
                      </td>
                      <td className="px-4 py-3">
                        <SubmissionStatusBadge status={submission.status} />
                      </td>
                      <td className="px-4 py-3 text-right">
                        <button
                          type="button"
                          onClick={() => onNavigateToSubmission(submission.id)}
                          className="text-sm text-gray-600 hover:text-gray-900"
                        >
                          View
                        </button>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        )}
      </section>
    </div>
  );
}
