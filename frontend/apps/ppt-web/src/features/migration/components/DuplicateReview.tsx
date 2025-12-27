/**
 * Duplicate Review Component (Story 66.4).
 *
 * Displays detected duplicates for user review and resolution.
 */

import { useState } from 'react';

export interface FieldDifference {
  field: string;
  importValue?: string;
  existingValue?: string;
}

export interface DuplicateRecord {
  importRow: number;
  existingId: string;
  matchedFields: string[];
  confidence: number;
  differences: FieldDifference[];
}

type DuplicateResolution = 'skip' | 'update' | 'create_new';

interface DuplicateReviewProps {
  duplicates: DuplicateRecord[];
  onResolve: (resolutions: Map<number, DuplicateResolution>) => void;
  onCancel: () => void;
  isLoading?: boolean;
}

export function DuplicateReview({
  duplicates,
  onResolve,
  onCancel,
  isLoading = false,
}: DuplicateReviewProps) {
  const [resolutions, setResolutions] = useState<Map<number, DuplicateResolution>>(() => {
    // Default resolution based on confidence
    const map = new Map<number, DuplicateResolution>();
    for (const dup of duplicates) {
      map.set(dup.importRow, dup.confidence >= 90 ? 'skip' : 'create_new');
    }
    return map;
  });
  const [expandedRow, setExpandedRow] = useState<number | null>(null);

  const handleResolutionChange = (importRow: number, resolution: DuplicateResolution) => {
    setResolutions((prev) => {
      const next = new Map(prev);
      next.set(importRow, resolution);
      return next;
    });
  };

  const handleApplyToAll = (resolution: DuplicateResolution) => {
    setResolutions((prev) => {
      const next = new Map(prev);
      for (const dup of duplicates) {
        next.set(dup.importRow, resolution);
      }
      return next;
    });
  };

  const handleSubmit = () => {
    onResolve(resolutions);
  };

  const getConfidenceColor = (confidence: number): string => {
    if (confidence >= 90) return 'text-red-600 bg-red-50';
    if (confidence >= 70) return 'text-yellow-600 bg-yellow-50';
    return 'text-green-600 bg-green-50';
  };

  const getConfidenceLabel = (confidence: number): string => {
    if (confidence >= 90) return 'High Match';
    if (confidence >= 70) return 'Likely Match';
    return 'Possible Match';
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="border-b border-gray-200 pb-4">
        <h2 className="text-lg font-medium text-gray-900">Duplicate Review</h2>
        <p className="mt-1 text-sm text-gray-500">
          {duplicates.length} potential duplicate{duplicates.length !== 1 ? 's' : ''} detected.
          Choose how to handle each one.
        </p>
      </div>

      {/* Bulk Actions */}
      <div className="flex items-center gap-4 rounded-lg bg-gray-50 p-3">
        <span className="text-sm text-gray-600">Apply to all:</span>
        <button
          type="button"
          onClick={() => handleApplyToAll('skip')}
          className="rounded-md bg-gray-200 px-3 py-1 text-sm text-gray-700 hover:bg-gray-300"
        >
          Skip All
        </button>
        <button
          type="button"
          onClick={() => handleApplyToAll('update')}
          className="rounded-md bg-blue-100 px-3 py-1 text-sm text-blue-700 hover:bg-blue-200"
        >
          Update All
        </button>
        <button
          type="button"
          onClick={() => handleApplyToAll('create_new')}
          className="rounded-md bg-green-100 px-3 py-1 text-sm text-green-700 hover:bg-green-200"
        >
          Create All New
        </button>
      </div>

      {/* Duplicates List */}
      <div className="space-y-3">
        {duplicates.map((duplicate) => {
          const isExpanded = expandedRow === duplicate.importRow;
          const resolution = resolutions.get(duplicate.importRow) ?? 'skip';

          return (
            <div
              key={duplicate.importRow}
              className="rounded-lg border border-gray-200 bg-white"
            >
              {/* Summary Row */}
              <div className="flex items-center justify-between p-4">
                <div className="flex items-center gap-4">
                  <button
                    type="button"
                    onClick={() => setExpandedRow(isExpanded ? null : duplicate.importRow)}
                    className="rounded p-1 text-gray-400 hover:bg-gray-100 hover:text-gray-600"
                  >
                    <svg
                      className={`h-5 w-5 transition-transform ${isExpanded ? 'rotate-90' : ''}`}
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M9 5l7 7-7 7"
                      />
                    </svg>
                  </button>

                  <div>
                    <p className="text-sm font-medium text-gray-900">
                      Row {duplicate.importRow}
                    </p>
                    <p className="text-xs text-gray-500">
                      Matches: {duplicate.matchedFields.join(', ')}
                    </p>
                  </div>

                  <span
                    className={`rounded-full px-2 py-0.5 text-xs font-medium ${getConfidenceColor(duplicate.confidence)}`}
                  >
                    {duplicate.confidence}% - {getConfidenceLabel(duplicate.confidence)}
                  </span>
                </div>

                {/* Resolution Selector */}
                <div className="flex items-center gap-2">
                  <select
                    value={resolution}
                    onChange={(e) =>
                      handleResolutionChange(duplicate.importRow, e.target.value as DuplicateResolution)
                    }
                    className="rounded-md border-gray-300 text-sm shadow-sm focus:border-blue-500 focus:ring-blue-500"
                  >
                    <option value="skip">Skip (don't import)</option>
                    <option value="update">Update existing record</option>
                    <option value="create_new">Create as new record</option>
                  </select>
                </div>
              </div>

              {/* Expanded Details */}
              {isExpanded && (
                <div className="border-t border-gray-200 bg-gray-50 p-4">
                  <p className="text-xs font-medium uppercase text-gray-500">Field Comparison</p>
                  <div className="mt-3 overflow-hidden rounded-lg border border-gray-200 bg-white">
                    <table className="min-w-full divide-y divide-gray-200">
                      <thead className="bg-gray-50">
                        <tr>
                          <th className="px-4 py-2 text-left text-xs font-medium uppercase text-gray-500">
                            Field
                          </th>
                          <th className="px-4 py-2 text-left text-xs font-medium uppercase text-gray-500">
                            Import Value
                          </th>
                          <th className="px-4 py-2 text-left text-xs font-medium uppercase text-gray-500">
                            Existing Value
                          </th>
                          <th className="px-4 py-2 text-left text-xs font-medium uppercase text-gray-500">
                            Status
                          </th>
                        </tr>
                      </thead>
                      <tbody className="divide-y divide-gray-200">
                        {duplicate.matchedFields.map((field) => (
                          <tr key={field} className="bg-green-50">
                            <td className="px-4 py-2 text-sm font-medium text-gray-900">
                              {field}
                            </td>
                            <td className="px-4 py-2 text-sm text-gray-600" colSpan={2}>
                              (Values match)
                            </td>
                            <td className="px-4 py-2">
                              <span className="inline-flex items-center rounded-full bg-green-100 px-2 py-0.5 text-xs font-medium text-green-800">
                                Match
                              </span>
                            </td>
                          </tr>
                        ))}
                        {duplicate.differences.map((diff) => (
                          <tr key={diff.field}>
                            <td className="px-4 py-2 text-sm font-medium text-gray-900">
                              {diff.field}
                            </td>
                            <td className="px-4 py-2 text-sm text-gray-600">
                              {diff.importValue ?? '-'}
                            </td>
                            <td className="px-4 py-2 text-sm text-gray-600">
                              {diff.existingValue ?? '-'}
                            </td>
                            <td className="px-4 py-2">
                              <span className="inline-flex items-center rounded-full bg-yellow-100 px-2 py-0.5 text-xs font-medium text-yellow-800">
                                Different
                              </span>
                            </td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>

                  <p className="mt-3 text-xs text-gray-500">
                    Existing record ID: <code className="rounded bg-gray-200 px-1">{duplicate.existingId}</code>
                  </p>
                </div>
              )}
            </div>
          );
        })}
      </div>

      {/* Summary */}
      <div className="rounded-lg bg-gray-50 p-4">
        <h3 className="text-sm font-medium text-gray-900">Resolution Summary</h3>
        <div className="mt-2 flex gap-6 text-sm">
          <span className="text-gray-600">
            Skip:{' '}
            <span className="font-medium text-gray-900">
              {Array.from(resolutions.values()).filter((r) => r === 'skip').length}
            </span>
          </span>
          <span className="text-blue-600">
            Update:{' '}
            <span className="font-medium">
              {Array.from(resolutions.values()).filter((r) => r === 'update').length}
            </span>
          </span>
          <span className="text-green-600">
            Create new:{' '}
            <span className="font-medium">
              {Array.from(resolutions.values()).filter((r) => r === 'create_new').length}
            </span>
          </span>
        </div>
      </div>

      {/* Actions */}
      <div className="flex justify-end gap-3 border-t border-gray-200 pt-4">
        <button
          type="button"
          onClick={onCancel}
          disabled={isLoading}
          className="rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
        >
          Cancel
        </button>
        <button
          type="button"
          onClick={handleSubmit}
          disabled={isLoading}
          className="inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
        >
          {isLoading ? 'Applying...' : 'Apply Resolutions'}
        </button>
      </div>
    </div>
  );
}
