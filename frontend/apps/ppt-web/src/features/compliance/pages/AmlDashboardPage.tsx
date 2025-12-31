/**
 * AML Dashboard Page (Epic 67, Story 67.1).
 *
 * Dashboard for AML risk assessments and compliance monitoring.
 */

import type React from 'react';
import { useCallback, useEffect, useState } from 'react';
import { AmlRiskAssessmentCard } from '../components/AmlRiskAssessmentCard';
import type { AmlRiskAssessment } from '../components/AmlRiskAssessmentCard';

interface AmlThresholds {
  transaction_threshold_eur: number;
  transaction_threshold_cents: number;
  cumulative_threshold_eur: number;
  review_threshold_score: number;
}

interface CountryRisk {
  country_code: string;
  country_name: string;
  risk_rating: string;
  is_sanctioned: boolean;
  fatf_status?: string;
}

export const AmlDashboardPage: React.FC = () => {
  const [assessments, setAssessments] = useState<AmlRiskAssessment[]>([]);
  const [thresholds, setThresholds] = useState<AmlThresholds | null>(null);
  const [countryRisks, setCountryRisks] = useState<CountryRisk[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Filters
  const [statusFilter, setStatusFilter] = useState<string>('');
  const [riskLevelFilter, setRiskLevelFilter] = useState<string>('');
  const [flaggedOnly, setFlaggedOnly] = useState(false);

  // Load data
  useEffect(() => {
    const loadData = async () => {
      try {
        setIsLoading(true);
        setError(null);

        // In production, these would be API calls
        // For now, we use sample data
        await new Promise((resolve) => setTimeout(resolve, 500));

        setThresholds({
          transaction_threshold_eur: 10000,
          transaction_threshold_cents: 1000000,
          cumulative_threshold_eur: 15000,
          review_threshold_score: 50,
        });

        setCountryRisks([
          {
            country_code: 'SK',
            country_name: 'Slovakia',
            risk_rating: 'low',
            is_sanctioned: false,
          },
          {
            country_code: 'CZ',
            country_name: 'Czech Republic',
            risk_rating: 'low',
            is_sanctioned: false,
          },
          {
            country_code: 'RU',
            country_name: 'Russia',
            risk_rating: 'high',
            is_sanctioned: true,
            fatf_status: 'FATF Blacklist',
          },
        ]);

        // Sample assessments
        setAssessments([]);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load AML data');
        // Error is shown to user via setError, logging handled by error boundary
      } finally {
        setIsLoading(false);
      }
    };

    loadData();
    // Note: Filters are not in deps because filtering is done client-side after data load
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const handleInitiateEdd = useCallback((_assessmentId: string) => {
    // TODO: Navigate to EDD initiation
  }, []);

  const handleReview = useCallback((_assessmentId: string) => {
    // TODO: Open review modal
  }, []);

  const filteredAssessments = assessments.filter((a) => {
    if (statusFilter && a.status !== statusFilter) return false;
    if (riskLevelFilter && a.risk_level !== riskLevelFilter) return false;
    if (flaggedOnly && !a.flagged_for_review) return false;
    return true;
  });

  return (
    <div className="aml-dashboard-page">
      <div className="aml-dashboard-header">
        <h1>AML Compliance Dashboard</h1>
        <p>Monitor anti-money laundering risk assessments and compliance status.</p>
      </div>

      {error && (
        <div className="aml-dashboard-error" role="alert">
          {error}
        </div>
      )}

      {/* Thresholds Info */}
      {thresholds && (
        <div className="aml-thresholds-section">
          <h2>AML Thresholds</h2>
          <div className="aml-thresholds-grid">
            <div className="aml-threshold-card">
              <div className="aml-threshold-value">
                {thresholds.transaction_threshold_eur.toLocaleString()} EUR
              </div>
              <div className="aml-threshold-label">Transaction Threshold</div>
            </div>
            <div className="aml-threshold-card">
              <div className="aml-threshold-value">
                {thresholds.cumulative_threshold_eur.toLocaleString()} EUR
              </div>
              <div className="aml-threshold-label">Cumulative Threshold</div>
            </div>
            <div className="aml-threshold-card">
              <div className="aml-threshold-value">{thresholds.review_threshold_score}</div>
              <div className="aml-threshold-label">Review Score Threshold</div>
            </div>
          </div>
        </div>
      )}

      {/* Country Risks */}
      <div className="aml-country-risks-section">
        <h2>Country Risk Database</h2>
        <table className="aml-country-risks-table">
          <thead>
            <tr>
              <th>Country</th>
              <th>Risk Rating</th>
              <th>Sanctioned</th>
              <th>FATF Status</th>
            </tr>
          </thead>
          <tbody>
            {countryRisks.map((country) => (
              <tr key={country.country_code} className={`risk-${country.risk_rating}`}>
                <td>
                  {country.country_code} - {country.country_name}
                </td>
                <td>
                  <span className={`risk-badge ${country.risk_rating}`}>
                    {country.risk_rating.toUpperCase()}
                  </span>
                </td>
                <td>{country.is_sanctioned ? 'Yes' : 'No'}</td>
                <td>{country.fatf_status || '-'}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Filters */}
      <div className="aml-filters-section">
        <h2>Risk Assessments</h2>
        <div className="aml-filters">
          <div className="aml-filter">
            <label htmlFor="statusFilter">Status</label>
            <select
              id="statusFilter"
              value={statusFilter}
              onChange={(e) => setStatusFilter(e.target.value)}
            >
              <option value="">All Statuses</option>
              <option value="pending">Pending</option>
              <option value="in_progress">In Progress</option>
              <option value="completed">Completed</option>
              <option value="requires_review">Requires Review</option>
              <option value="approved">Approved</option>
              <option value="rejected">Rejected</option>
            </select>
          </div>
          <div className="aml-filter">
            <label htmlFor="riskLevelFilter">Risk Level</label>
            <select
              id="riskLevelFilter"
              value={riskLevelFilter}
              onChange={(e) => setRiskLevelFilter(e.target.value)}
            >
              <option value="">All Levels</option>
              <option value="low">Low</option>
              <option value="medium">Medium</option>
              <option value="high">High</option>
              <option value="critical">Critical</option>
            </select>
          </div>
          <div className="aml-filter checkbox">
            <label htmlFor="flaggedOnly">
              <input
                type="checkbox"
                id="flaggedOnly"
                checked={flaggedOnly}
                onChange={(e) => setFlaggedOnly(e.target.checked)}
              />
              Flagged for Review Only
            </label>
          </div>
        </div>
      </div>

      {/* Assessments List */}
      {isLoading ? (
        <div className="aml-loading">Loading AML data...</div>
      ) : filteredAssessments.length > 0 ? (
        <div className="aml-assessments-list">
          {filteredAssessments.map((assessment) => (
            <AmlRiskAssessmentCard
              key={assessment.id}
              assessment={assessment}
              onInitiateEdd={handleInitiateEdd}
              onReview={handleReview}
              showActions={true}
            />
          ))}
        </div>
      ) : (
        <div className="aml-empty-state">
          <p>No AML risk assessments found matching the criteria.</p>
          <p>Assessments are automatically created when transactions exceed the threshold.</p>
        </div>
      )}
    </div>
  );
};

AmlDashboardPage.displayName = 'AmlDashboardPage';
