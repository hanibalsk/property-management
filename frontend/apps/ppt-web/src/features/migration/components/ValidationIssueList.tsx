/**
 * Validation Issue List Component (Story 66.4).
 *
 * Displays a detailed, filterable list of validation issues.
 */

import { useMemo, useState } from 'react';
import type { ValidationIssue, ValidationSeverity } from './ImportPreview';

interface ValidationIssueListProps {
  issues: ValidationIssue[];
  totalCount: number;
  onLoadMore?: () => void;
  hasMore?: boolean;
  isLoading?: boolean;
}

const SEVERITY_LABELS: Record<ValidationSeverity, string> = {
  error: 'Error',
  warning: 'Warning',
  info: 'Info',
};

const SEVERITY_COLORS: Record<ValidationSeverity, { bg: string; text: string; border: string }> = {
  error: {
    bg: 'bg-red-50',
    text: 'text-red-800',
    border: 'border-red-200',
  },
  warning: {
    bg: 'bg-yellow-50',
    text: 'text-yellow-800',
    border: 'border-yellow-200',
  },
  info: {
    bg: 'bg-blue-50',
    text: 'text-blue-800',
    border: 'border-blue-200',
  },
};

export function ValidationIssueList({
  issues,
  totalCount,
  onLoadMore,
  hasMore = false,
  isLoading = false,
}: ValidationIssueListProps) {
  const [severityFilter, setSeverityFilter] = useState<ValidationSeverity | 'all'>('all');
  const [columnFilter, setColumnFilter] = useState<string>('all');
  const [searchQuery, setSearchQuery] = useState('');

  // Get unique columns for filter
  const uniqueColumns = useMemo(() => {
    const columns = new Set<string>();
    for (const issue of issues) {
      if (issue.column) {
        columns.add(issue.column);
      }
    }
    return Array.from(columns).sort();
  }, [issues]);

  // Filter issues
  const filteredIssues = useMemo(() => {
    return issues.filter((issue) => {
      if (severityFilter !== 'all' && issue.severity !== severityFilter) {
        return false;
      }
      if (columnFilter !== 'all' && issue.column !== columnFilter) {
        return false;
      }
      if (searchQuery) {
        const query = searchQuery.toLowerCase();
        const matchesMessage = issue.message.toLowerCase().includes(query);
        const matchesCode = issue.code.toLowerCase().includes(query);
        const matchesValue = issue.originalValue?.toLowerCase().includes(query);
        if (!matchesMessage && !matchesCode && !matchesValue) {
          return false;
        }
      }
      return true;
    });
  }, [issues, severityFilter, columnFilter, searchQuery]);

  // Count by severity
  const countBySeverity = useMemo(() => {
    const counts = { error: 0, warning: 0, info: 0 };
    for (const issue of issues) {
      counts[issue.severity]++;
    }
    return counts;
  }, [issues]);

  return (
    <div className="space-y-4">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-medium text-gray-900">
          Validation Issues ({totalCount.toLocaleString()})
        </h3>
        <div className="flex items-center gap-4 text-sm">
          <span className="flex items-center gap-1 text-red-600">
            <span className="h-2 w-2 rounded-full bg-red-500" />
            {countBySeverity.error} errors
          </span>
          <span className="flex items-center gap-1 text-yellow-600">
            <span className="h-2 w-2 rounded-full bg-yellow-500" />
            {countBySeverity.warning} warnings
          </span>
          <span className="flex items-center gap-1 text-blue-600">
            <span className="h-2 w-2 rounded-full bg-blue-500" />
            {countBySeverity.info} info
          </span>
        </div>
      </div>

      {/* Filters */}
      <div className="flex flex-wrap items-center gap-3">
        <div>
          <label htmlFor="severity-filter" className="sr-only">
            Filter by severity
          </label>
          <select
            id="severity-filter"
            value={severityFilter}
            onChange={(e) => setSeverityFilter(e.target.value as ValidationSeverity | 'all')}
            className="rounded-md border-gray-300 text-sm shadow-sm focus:border-blue-500 focus:ring-blue-500"
          >
            <option value="all">All Severities</option>
            <option value="error">Errors Only</option>
            <option value="warning">Warnings Only</option>
            <option value="info">Info Only</option>
          </select>
        </div>

        {uniqueColumns.length > 0 && (
          <div>
            <label htmlFor="column-filter" className="sr-only">
              Filter by column
            </label>
            <select
              id="column-filter"
              value={columnFilter}
              onChange={(e) => setColumnFilter(e.target.value)}
              className="rounded-md border-gray-300 text-sm shadow-sm focus:border-blue-500 focus:ring-blue-500"
            >
              <option value="all">All Columns</option>
              {uniqueColumns.map((column) => (
                <option key={column} value={column}>
                  {column}
                </option>
              ))}
            </select>
          </div>
        )}

        <div className="flex-1">
          <label htmlFor="search-issues" className="sr-only">
            Search issues
          </label>
          <input
            type="text"
            id="search-issues"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search issues..."
            className="w-full max-w-xs rounded-md border-gray-300 text-sm shadow-sm focus:border-blue-500 focus:ring-blue-500"
          />
        </div>
      </div>

      {/* Issue List */}
      {filteredIssues.length === 0 ? (
        <div className="py-8 text-center text-sm text-gray-500">
          {issues.length === 0 ? 'No validation issues.' : 'No issues match your filters.'}
        </div>
      ) : (
        <div className="space-y-2">
          {filteredIssues.map((issue, index) => (
            <IssueCard key={index} issue={issue} />
          ))}
        </div>
      )}

      {/* Load More */}
      {hasMore && (
        <div className="text-center">
          <button
            type="button"
            onClick={onLoadMore}
            disabled={isLoading}
            className="inline-flex items-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
          >
            {isLoading ? 'Loading...' : 'Load More Issues'}
          </button>
        </div>
      )}

      {/* Showing count */}
      <p className="text-center text-xs text-gray-500">
        Showing {filteredIssues.length} of {totalCount} issues
      </p>
    </div>
  );
}

interface IssueCardProps {
  issue: ValidationIssue;
}

function IssueCard({ issue }: IssueCardProps) {
  const colors = SEVERITY_COLORS[issue.severity];

  return (
    <div className={`rounded-lg border p-4 ${colors.bg} ${colors.border}`}>
      <div className="flex items-start justify-between gap-4">
        <div className="flex-1">
          <div className="flex items-center gap-2">
            <span
              className={`inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${colors.text}`}
            >
              {SEVERITY_LABELS[issue.severity]}
            </span>
            <span className="text-xs text-gray-500">{issue.code}</span>
          </div>

          <p className={`mt-1 text-sm font-medium ${colors.text}`}>{issue.message}</p>

          <div className="mt-2 flex flex-wrap gap-3 text-xs text-gray-600">
            {issue.rowNumber && (
              <span className="flex items-center gap-1">
                <svg className="h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M4 6h16M4 10h16M4 14h16M4 18h16"
                  />
                </svg>
                Row {issue.rowNumber}
              </span>
            )}
            {issue.column && (
              <span className="flex items-center gap-1">
                <svg className="h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M9 17V7m0 10a2 2 0 01-2 2H5a2 2 0 01-2-2V7a2 2 0 012-2h2a2 2 0 012 2m0 10a2 2 0 002 2h2a2 2 0 002-2M9 7a2 2 0 012-2h2a2 2 0 012 2m0 10V7m0 10a2 2 0 002 2h2a2 2 0 002-2V7a2 2 0 00-2-2h-2a2 2 0 00-2 2"
                  />
                </svg>
                {issue.column}
              </span>
            )}
          </div>

          {/* Values */}
          {(issue.originalValue || issue.suggestedValue) && (
            <div className="mt-3 space-y-1">
              {issue.originalValue && (
                <div className="flex items-center gap-2 text-xs">
                  <span className="text-gray-500">Original:</span>
                  <code className="rounded bg-white/60 px-1.5 py-0.5 font-mono">
                    {issue.originalValue}
                  </code>
                </div>
              )}
              {issue.suggestedValue && (
                <div className="flex items-center gap-2 text-xs">
                  <span className="text-gray-500">Suggested:</span>
                  <code className="rounded bg-green-100 px-1.5 py-0.5 font-mono text-green-800">
                    {issue.suggestedValue}
                  </code>
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
