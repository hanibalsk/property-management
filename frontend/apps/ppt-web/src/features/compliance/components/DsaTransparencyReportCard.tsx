/**
 * DSA Transparency Report Card Component (Epic 67, Story 67.3).
 *
 * Displays DSA transparency report summary and statistics.
 */

import type React from 'react';

export type DsaReportStatus = 'draft' | 'generated' | 'published' | 'archived';

export interface ContentTypeCount {
  content_type: string;
  count: number;
}

export interface ViolationTypeCount {
  violation_type: string;
  count: number;
}

export interface DsaReportSummary {
  total_moderation_actions: number;
  content_removed: number;
  content_restricted: number;
  warnings_issued: number;
  user_reports_received: number;
  user_reports_resolved: number;
  avg_resolution_time_hours?: number;
  automated_decisions: number;
  automated_decisions_overturned: number;
  appeals_received: number;
  appeals_upheld: number;
  appeals_rejected: number;
}

export interface DsaTransparencyReport {
  id: string;
  period_start: string;
  period_end: string;
  status: DsaReportStatus;
  summary: DsaReportSummary;
  content_type_breakdown: ContentTypeCount[];
  violation_type_breakdown: ViolationTypeCount[];
  download_url?: string;
  generated_at?: string;
  published_at?: string;
}

export interface DsaTransparencyReportCardProps {
  report: DsaTransparencyReport;
  onPublish?: (reportId: string) => void;
  onDownload?: (reportId: string) => void;
  showActions?: boolean;
}

const getStatusLabel = (status: DsaReportStatus): string => {
  switch (status) {
    case 'draft':
      return 'Draft';
    case 'generated':
      return 'Generated';
    case 'published':
      return 'Published';
    case 'archived':
      return 'Archived';
    default:
      return status;
  }
};

const formatDate = (dateStr: string): string => {
  return new Date(dateStr).toLocaleDateString('en-GB', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  });
};

const formatViolationType = (type: string): string => {
  return type
    .split('_')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
};

export const DsaTransparencyReportCard: React.FC<DsaTransparencyReportCardProps> = ({
  report,
  onPublish,
  onDownload,
  showActions = true,
}) => {
  const { summary } = report;

  const resolutionRate =
    summary.user_reports_received > 0
      ? ((summary.user_reports_resolved / summary.user_reports_received) * 100).toFixed(1)
      : '0.0';

  const appealUpheldRate =
    summary.appeals_received > 0
      ? ((summary.appeals_upheld / summary.appeals_received) * 100).toFixed(1)
      : '0.0';

  return (
    <div className="dsa-report-card">
      <div className="dsa-report-header">
        <div className="dsa-report-title">
          <h3>DSA Transparency Report</h3>
          <span className={`dsa-status-badge ${report.status}`}>{getStatusLabel(report.status)}</span>
        </div>
        <div className="dsa-report-period">
          <span>
            Period: {formatDate(report.period_start)} - {formatDate(report.period_end)}
          </span>
        </div>
      </div>

      {/* Summary Statistics */}
      <div className="dsa-summary-section">
        <h4>Summary Statistics</h4>
        <div className="dsa-stats-grid">
          <div className="dsa-stat-card">
            <div className="dsa-stat-value">{summary.total_moderation_actions}</div>
            <div className="dsa-stat-label">Total Moderation Actions</div>
          </div>
          <div className="dsa-stat-card">
            <div className="dsa-stat-value">{summary.user_reports_received}</div>
            <div className="dsa-stat-label">User Reports Received</div>
          </div>
          <div className="dsa-stat-card">
            <div className="dsa-stat-value">{resolutionRate}%</div>
            <div className="dsa-stat-label">Resolution Rate</div>
          </div>
          <div className="dsa-stat-card">
            <div className="dsa-stat-value">
              {summary.avg_resolution_time_hours?.toFixed(1) || 'N/A'}h
            </div>
            <div className="dsa-stat-label">Avg Resolution Time</div>
          </div>
        </div>
      </div>

      {/* Action Breakdown */}
      <div className="dsa-actions-section">
        <h4>Moderation Actions</h4>
        <div className="dsa-actions-grid">
          <div className="dsa-action-item removed">
            <span className="dsa-action-count">{summary.content_removed}</span>
            <span className="dsa-action-label">Content Removed</span>
          </div>
          <div className="dsa-action-item restricted">
            <span className="dsa-action-count">{summary.content_restricted}</span>
            <span className="dsa-action-label">Content Restricted</span>
          </div>
          <div className="dsa-action-item warned">
            <span className="dsa-action-count">{summary.warnings_issued}</span>
            <span className="dsa-action-label">Warnings Issued</span>
          </div>
        </div>
      </div>

      {/* Automated Decisions */}
      <div className="dsa-automated-section">
        <h4>Automated Decisions</h4>
        <div className="dsa-automated-stats">
          <div className="dsa-automated-item">
            <span className="dsa-automated-value">{summary.automated_decisions}</span>
            <span className="dsa-automated-label">Total Automated Decisions</span>
          </div>
          <div className="dsa-automated-item">
            <span className="dsa-automated-value">{summary.automated_decisions_overturned}</span>
            <span className="dsa-automated-label">Overturned by Human Review</span>
          </div>
        </div>
      </div>

      {/* Appeals */}
      <div className="dsa-appeals-section">
        <h4>Appeals</h4>
        <div className="dsa-appeals-stats">
          <div className="dsa-appeals-item">
            <span className="dsa-appeals-value">{summary.appeals_received}</span>
            <span className="dsa-appeals-label">Appeals Received</span>
          </div>
          <div className="dsa-appeals-item upheld">
            <span className="dsa-appeals-value">{summary.appeals_upheld}</span>
            <span className="dsa-appeals-label">Appeals Upheld</span>
          </div>
          <div className="dsa-appeals-item rejected">
            <span className="dsa-appeals-value">{summary.appeals_rejected}</span>
            <span className="dsa-appeals-label">Appeals Rejected</span>
          </div>
          <div className="dsa-appeals-item rate">
            <span className="dsa-appeals-value">{appealUpheldRate}%</span>
            <span className="dsa-appeals-label">Upheld Rate</span>
          </div>
        </div>
      </div>

      {/* Violation Type Breakdown */}
      {report.violation_type_breakdown.length > 0 && (
        <div className="dsa-violation-section">
          <h4>By Violation Type</h4>
          <ul className="dsa-violation-list">
            {report.violation_type_breakdown.map((item, index) => (
              <li key={index} className="dsa-violation-item">
                <span className="dsa-violation-type">{formatViolationType(item.violation_type)}</span>
                <span className="dsa-violation-count">{item.count}</span>
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Content Type Breakdown */}
      {report.content_type_breakdown.length > 0 && (
        <div className="dsa-content-type-section">
          <h4>By Content Type</h4>
          <ul className="dsa-content-type-list">
            {report.content_type_breakdown.map((item, index) => (
              <li key={index} className="dsa-content-type-item">
                <span className="dsa-content-type">{formatViolationType(item.content_type)}</span>
                <span className="dsa-content-count">{item.count}</span>
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Actions */}
      {showActions && (
        <div className="dsa-report-actions">
          {report.status === 'generated' && onPublish && (
            <button
              type="button"
              className="dsa-action-button primary"
              onClick={() => onPublish(report.id)}
            >
              Publish Report
            </button>
          )}
          {report.download_url && onDownload && (
            <button
              type="button"
              className="dsa-action-button secondary"
              onClick={() => onDownload(report.id)}
            >
              Download PDF
            </button>
          )}
        </div>
      )}

      {/* Publication Info */}
      {report.published_at && (
        <div className="dsa-published-info">
          <span>Published on {formatDate(report.published_at)}</span>
        </div>
      )}
    </div>
  );
};

DsaTransparencyReportCard.displayName = 'DsaTransparencyReportCard';
