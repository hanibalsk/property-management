/**
 * DSA Transparency Reports Page (Epic 67, Story 67.3).
 *
 * Page for generating and viewing DSA transparency reports.
 */

import type React from 'react';
import { useCallback, useEffect, useState } from 'react';
import { DsaTransparencyReportCard } from '../components/DsaTransparencyReportCard';
import type {
  DsaReportStatus,
  DsaTransparencyReport,
} from '../components/DsaTransparencyReportCard';

interface DsaMetrics {
  current_period_start: string;
  current_period_end: string;
  moderation_actions_this_period: number;
  pending_cases: number;
  avg_resolution_time_hours: number;
  sla_compliance_rate: number;
}

export const DsaReportsPage: React.FC = () => {
  const [reports, setReports] = useState<DsaTransparencyReport[]>([]);
  const [metrics, setMetrics] = useState<DsaMetrics | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isGenerating, setIsGenerating] = useState(false);

  // Report generation form
  const [showGenerateForm, setShowGenerateForm] = useState(false);
  const [periodStart, setPeriodStart] = useState('');
  const [periodEnd, setPeriodEnd] = useState('');

  // Load data
  useEffect(() => {
    const loadData = async () => {
      try {
        setIsLoading(true);
        setError(null);

        // In production, these would be API calls
        await new Promise((resolve) => setTimeout(resolve, 500));

        const now = new Date();
        const thirtyDaysAgo = new Date(now.getTime() - 30 * 24 * 60 * 60 * 1000);

        setMetrics({
          current_period_start: thirtyDaysAgo.toISOString(),
          current_period_end: now.toISOString(),
          moderation_actions_this_period: 0,
          pending_cases: 0,
          avg_resolution_time_hours: 0,
          sla_compliance_rate: 100,
        });

        setReports([]);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load DSA reports');
        console.error('Failed to load DSA reports:', err);
      } finally {
        setIsLoading(false);
      }
    };

    loadData();
  }, []);

  const handleGenerateReport = useCallback(async () => {
    if (!periodStart || !periodEnd) {
      setError('Please select both start and end dates');
      return;
    }

    try {
      setIsGenerating(true);
      setError(null);

      // In production, this would be an API call
      await new Promise((resolve) => setTimeout(resolve, 1000));

      const newReport: DsaTransparencyReport = {
        id: crypto.randomUUID(),
        period_start: periodStart,
        period_end: periodEnd,
        status: 'generated',
        summary: {
          total_moderation_actions: 0,
          content_removed: 0,
          content_restricted: 0,
          warnings_issued: 0,
          user_reports_received: 0,
          user_reports_resolved: 0,
          avg_resolution_time_hours: undefined,
          automated_decisions: 0,
          automated_decisions_overturned: 0,
          appeals_received: 0,
          appeals_upheld: 0,
          appeals_rejected: 0,
        },
        content_type_breakdown: [],
        violation_type_breakdown: [],
        download_url: undefined,
        generated_at: new Date().toISOString(),
        published_at: undefined,
      };

      setReports((prev) => [newReport, ...prev]);
      setShowGenerateForm(false);
      setPeriodStart('');
      setPeriodEnd('');
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to generate report');
      console.error('Failed to generate report:', err);
    } finally {
      setIsGenerating(false);
    }
  }, [periodStart, periodEnd]);

  const handlePublish = useCallback((reportId: string) => {
    console.log('Publish report:', reportId);
    setReports((prev) =>
      prev.map((r) =>
        r.id === reportId
          ? { ...r, status: 'published' as DsaReportStatus, published_at: new Date().toISOString() }
          : r
      )
    );
  }, []);

  const handleDownload = useCallback((reportId: string) => {
    console.log('Download report:', reportId);
    // Trigger PDF download
  }, []);

  const formatDate = (dateStr: string): string => {
    return new Date(dateStr).toLocaleDateString('en-GB', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  };

  return (
    <div className="dsa-reports-page">
      <div className="dsa-reports-header">
        <h1>DSA Transparency Reports</h1>
        <p>Generate and publish transparency reports for Digital Services Act compliance.</p>
      </div>

      {error && (
        <div className="dsa-reports-error" role="alert">
          {error}
        </div>
      )}

      {/* Current Metrics */}
      {metrics && (
        <div className="dsa-current-metrics">
          <h2>Current Period Metrics</h2>
          <p className="dsa-period-info">
            Period: {formatDate(metrics.current_period_start)} -{' '}
            {formatDate(metrics.current_period_end)}
          </p>
          <div className="dsa-metrics-grid">
            <div className="dsa-metric-card">
              <div className="dsa-metric-value">{metrics.moderation_actions_this_period}</div>
              <div className="dsa-metric-label">Moderation Actions</div>
            </div>
            <div className="dsa-metric-card">
              <div className="dsa-metric-value">{metrics.pending_cases}</div>
              <div className="dsa-metric-label">Pending Cases</div>
            </div>
            <div className="dsa-metric-card">
              <div className="dsa-metric-value">
                {metrics.avg_resolution_time_hours.toFixed(1)}h
              </div>
              <div className="dsa-metric-label">Avg Resolution Time</div>
            </div>
            <div className="dsa-metric-card">
              <div className="dsa-metric-value">{metrics.sla_compliance_rate.toFixed(1)}%</div>
              <div className="dsa-metric-label">SLA Compliance</div>
            </div>
          </div>
        </div>
      )}

      {/* Generate Report */}
      <div className="dsa-generate-section">
        {!showGenerateForm ? (
          <button
            type="button"
            className="dsa-generate-button"
            onClick={() => setShowGenerateForm(true)}
          >
            Generate New Report
          </button>
        ) : (
          <div className="dsa-generate-form">
            <h3>Generate Transparency Report</h3>
            <div className="dsa-form-row">
              <div className="dsa-form-field">
                <label htmlFor="periodStart">Period Start</label>
                <input
                  type="date"
                  id="periodStart"
                  value={periodStart}
                  onChange={(e) => setPeriodStart(e.target.value)}
                />
              </div>
              <div className="dsa-form-field">
                <label htmlFor="periodEnd">Period End</label>
                <input
                  type="date"
                  id="periodEnd"
                  value={periodEnd}
                  onChange={(e) => setPeriodEnd(e.target.value)}
                />
              </div>
            </div>
            <div className="dsa-form-actions">
              <button
                type="button"
                className="dsa-form-cancel"
                onClick={() => setShowGenerateForm(false)}
                disabled={isGenerating}
              >
                Cancel
              </button>
              <button
                type="button"
                className="dsa-form-submit"
                onClick={handleGenerateReport}
                disabled={isGenerating}
              >
                {isGenerating ? 'Generating...' : 'Generate Report'}
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Reports List */}
      <div className="dsa-reports-list-section">
        <h2>Previous Reports</h2>
        {isLoading ? (
          <div className="dsa-loading">Loading reports...</div>
        ) : reports.length > 0 ? (
          <div className="dsa-reports-list">
            {reports.map((report) => (
              <DsaTransparencyReportCard
                key={report.id}
                report={report}
                onPublish={handlePublish}
                onDownload={handleDownload}
                showActions={true}
              />
            ))}
          </div>
        ) : (
          <div className="dsa-empty-state">
            <p>No transparency reports have been generated yet.</p>
            <p>Generate your first report to comply with DSA requirements.</p>
          </div>
        )}
      </div>

      {/* DSA Requirements Info */}
      <div className="dsa-requirements-info">
        <h2>DSA Reporting Requirements</h2>
        <ul>
          <li>
            <strong>Content Moderation Actions:</strong> Report all content removal, restriction,
            and warning actions taken.
          </li>
          <li>
            <strong>User Reports:</strong> Track and report on user-submitted content reports and
            their resolutions.
          </li>
          <li>
            <strong>Automated Decisions:</strong> Document automated content moderation decisions
            and human review overturns.
          </li>
          <li>
            <strong>Appeals:</strong> Report on appeals received and their outcomes.
          </li>
          <li>
            <strong>Timeliness:</strong> Reports should be published at least annually for platforms
            with significant EU user bases.
          </li>
        </ul>
      </div>
    </div>
  );
};

DsaReportsPage.displayName = 'DsaReportsPage';
