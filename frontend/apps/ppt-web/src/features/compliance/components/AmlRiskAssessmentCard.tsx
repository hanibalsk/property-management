/**
 * AML Risk Assessment Card Component (Epic 67, Story 67.1).
 *
 * Displays AML risk assessment results with risk score, level, and factors.
 */

import type React from 'react';

export interface RiskFactor {
  factor_type: string;
  description: string;
  weight: number;
  mitigated: boolean;
}

export type AmlRiskLevel = 'low' | 'medium' | 'high' | 'critical';
export type AmlAssessmentStatus =
  | 'pending'
  | 'in_progress'
  | 'completed'
  | 'requires_review'
  | 'approved'
  | 'rejected';

export interface AmlRiskAssessment {
  id: string;
  party_id: string;
  party_type: string;
  transaction_id?: string;
  transaction_amount_cents?: number;
  currency?: string;
  risk_score: number;
  risk_level: AmlRiskLevel;
  status: AmlAssessmentStatus;
  risk_factors: RiskFactor[];
  country_code?: string;
  country_risk?: string;
  id_verified: boolean;
  source_of_funds_documented: boolean;
  pep_check_completed: boolean;
  is_pep?: boolean;
  sanctions_check_completed: boolean;
  sanctions_match?: boolean;
  flagged_for_review: boolean;
  review_reason?: string;
  recommendations: string[];
  created_at: string;
  assessed_at?: string;
}

export interface AmlRiskAssessmentCardProps {
  assessment: AmlRiskAssessment;
  onInitiateEdd?: (assessmentId: string) => void;
  onReview?: (assessmentId: string) => void;
  showActions?: boolean;
}

const getRiskLevelColor = (level: AmlRiskLevel): string => {
  switch (level) {
    case 'low':
      return 'aml-risk-low';
    case 'medium':
      return 'aml-risk-medium';
    case 'high':
      return 'aml-risk-high';
    case 'critical':
      return 'aml-risk-critical';
    default:
      return '';
  }
};

const getStatusLabel = (status: AmlAssessmentStatus): string => {
  switch (status) {
    case 'pending':
      return 'Pending';
    case 'in_progress':
      return 'In Progress';
    case 'completed':
      return 'Completed';
    case 'requires_review':
      return 'Requires Review';
    case 'approved':
      return 'Approved';
    case 'rejected':
      return 'Rejected';
    default:
      return status;
  }
};

const formatAmount = (cents: number, currency?: string): string => {
  const amount = cents / 100;
  return new Intl.NumberFormat('en-EU', {
    style: 'currency',
    currency: currency || 'EUR',
  }).format(amount);
};

export const AmlRiskAssessmentCard: React.FC<AmlRiskAssessmentCardProps> = ({
  assessment,
  onInitiateEdd,
  onReview,
  showActions = true,
}) => {
  const riskLevelClass = getRiskLevelColor(assessment.risk_level);

  return (
    <div className="aml-risk-assessment-card">
      <div className="aml-risk-assessment-header">
        <div className="aml-risk-assessment-title">
          <h3>AML Risk Assessment</h3>
          <span className={`aml-status-badge ${assessment.status}`}>
            {getStatusLabel(assessment.status)}
          </span>
        </div>
        <div className="aml-risk-assessment-meta">
          <span>Party: {assessment.party_type}</span>
          {assessment.country_code && <span>Country: {assessment.country_code}</span>}
        </div>
      </div>

      <div className="aml-risk-score-section">
        <div className={`aml-risk-score-indicator ${riskLevelClass}`}>
          <div className="aml-risk-score-value">{assessment.risk_score}</div>
          <div className="aml-risk-score-label">{assessment.risk_level.toUpperCase()}</div>
        </div>
        <div className="aml-risk-score-bar">
          <div
            className={`aml-risk-score-fill ${riskLevelClass}`}
            style={{ width: `${Math.min(assessment.risk_score, 100)}%` }}
          />
        </div>
      </div>

      {assessment.transaction_amount_cents && (
        <div className="aml-transaction-info">
          <span className="aml-transaction-label">Transaction Amount:</span>
          <span className="aml-transaction-value">
            {formatAmount(assessment.transaction_amount_cents, assessment.currency)}
          </span>
        </div>
      )}

      <div className="aml-verification-status">
        <h4>Verification Status</h4>
        <div className="aml-verification-grid">
          <div
            className={`aml-verification-item ${assessment.id_verified ? 'verified' : 'pending'}`}
          >
            <span className="aml-verification-icon">{assessment.id_verified ? 'Y' : 'N'}</span>
            <span className="aml-verification-label">ID Verified</span>
          </div>
          <div
            className={`aml-verification-item ${assessment.source_of_funds_documented ? 'verified' : 'pending'}`}
          >
            <span className="aml-verification-icon">
              {assessment.source_of_funds_documented ? 'Y' : 'N'}
            </span>
            <span className="aml-verification-label">Source of Funds</span>
          </div>
          <div
            className={`aml-verification-item ${assessment.pep_check_completed ? 'verified' : 'pending'}`}
          >
            <span className="aml-verification-icon">
              {assessment.pep_check_completed ? 'Y' : 'N'}
            </span>
            <span className="aml-verification-label">PEP Check</span>
          </div>
          <div
            className={`aml-verification-item ${assessment.sanctions_check_completed ? 'verified' : 'pending'}`}
          >
            <span className="aml-verification-icon">
              {assessment.sanctions_check_completed ? 'Y' : 'N'}
            </span>
            <span className="aml-verification-label">Sanctions Check</span>
          </div>
        </div>
      </div>

      {assessment.risk_factors.length > 0 && (
        <div className="aml-risk-factors">
          <h4>Risk Factors</h4>
          <ul className="aml-risk-factors-list">
            {assessment.risk_factors.map((factor, index) => (
              <li key={index} className={`aml-risk-factor ${factor.mitigated ? 'mitigated' : ''}`}>
                <span className="aml-risk-factor-type">{factor.factor_type}</span>
                <span className="aml-risk-factor-description">{factor.description}</span>
                <span className="aml-risk-factor-weight">Weight: {factor.weight}</span>
              </li>
            ))}
          </ul>
        </div>
      )}

      {assessment.recommendations.length > 0 && (
        <div className="aml-recommendations">
          <h4>Recommendations</h4>
          <ul className="aml-recommendations-list">
            {assessment.recommendations.map((rec, index) => (
              <li key={index}>{rec}</li>
            ))}
          </ul>
        </div>
      )}

      {assessment.flagged_for_review && (
        <div className="aml-review-alert">
          <strong>Flagged for Review</strong>
          {assessment.review_reason && <p>{assessment.review_reason}</p>}
        </div>
      )}

      {showActions && (
        <div className="aml-risk-assessment-actions">
          {assessment.flagged_for_review && onInitiateEdd && (
            <button
              type="button"
              className="aml-action-button primary"
              onClick={() => onInitiateEdd(assessment.id)}
            >
              Initiate EDD
            </button>
          )}
          {assessment.status === 'requires_review' && onReview && (
            <button
              type="button"
              className="aml-action-button secondary"
              onClick={() => onReview(assessment.id)}
            >
              Review Assessment
            </button>
          )}
        </div>
      )}
    </div>
  );
};

AmlRiskAssessmentCard.displayName = 'AmlRiskAssessmentCard';
