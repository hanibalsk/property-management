/**
 * AiSuggestionBadge component tests (Epic 126, Story 126.2).
 */

import { fireEvent, render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { AiSuggestionBadge } from './AiSuggestionBadge';

// Mock translations
vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => {
      const translations: Record<string, string> = {
        'faults.ai.suggestion': 'AI Suggestion',
        'faults.ai.accepted': 'Suggestion Accepted',
        'faults.ai.suggestionRegion': 'AI-suggested category and priority',
        'faults.ai.confidenceLabel': 'Confidence',
        'faults.ai.confidenceProgress': 'AI confidence level',
        'faults.ai.confidence.high': 'High confidence',
        'faults.ai.confidence.medium': 'Medium confidence',
        'faults.ai.confidence.low': 'Low confidence',
        'faults.ai.acceptSuggestion': 'Accept Suggestion',
        'faults.ai.modifySuggestion': 'Modify',
        'faults.ai.lowConfidenceNote':
          'The AI is less certain about this suggestion. Please review and adjust if needed.',
        'faults.category.plumbing': 'Plumbing',
        'faults.category.electrical': 'Electrical',
        'faults.category.other': 'Other',
        'faults.priorityMedium': 'Medium',
        'faults.priorityHigh': 'High',
        'faults.priorityUrgent': 'Urgent',
        'common.expand': 'Expand',
        'common.collapse': 'Collapse',
      };
      return translations[key] || key;
    },
  }),
}));

describe('AiSuggestionBadge', () => {
  const defaultProps = {
    category: 'plumbing' as const,
    confidence: 0.85,
    onAccept: vi.fn(),
    onModify: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders loading state', () => {
    render(<AiSuggestionBadge {...defaultProps} isLoading />);

    // Check for skeleton loading elements
    const skeleton = document.querySelector('.animate-pulse');
    expect(skeleton).toBeInTheDocument();
  });

  it('displays AI suggestion title', () => {
    render(<AiSuggestionBadge {...defaultProps} />);

    expect(screen.getByText('AI Suggestion')).toBeInTheDocument();
  });

  it('displays accepted state', () => {
    render(<AiSuggestionBadge {...defaultProps} isAccepted />);

    expect(screen.getByText('Suggestion Accepted')).toBeInTheDocument();
  });

  it('displays suggested category', () => {
    render(<AiSuggestionBadge {...defaultProps} />);

    expect(screen.getByText('Plumbing')).toBeInTheDocument();
  });

  it('displays suggested priority when provided', () => {
    render(<AiSuggestionBadge {...defaultProps} priority="high" />);

    expect(screen.getByText('Plumbing')).toBeInTheDocument();
    expect(screen.getByText('High')).toBeInTheDocument();
  });

  it('shows high confidence indicator for confidence >= 0.8', () => {
    render(<AiSuggestionBadge {...defaultProps} confidence={0.85} />);

    expect(screen.getByText('85% High confidence')).toBeInTheDocument();
  });

  it('shows medium confidence indicator for confidence >= 0.5', () => {
    render(<AiSuggestionBadge {...defaultProps} confidence={0.65} />);

    expect(screen.getByText('65% Medium confidence')).toBeInTheDocument();
  });

  it('shows low confidence indicator for confidence < 0.5', () => {
    render(<AiSuggestionBadge {...defaultProps} confidence={0.35} />);

    expect(screen.getByText('35% Low confidence')).toBeInTheDocument();
  });

  it('shows low confidence note for non-high confidence', () => {
    render(<AiSuggestionBadge {...defaultProps} confidence={0.45} />);

    expect(
      screen.getByText(
        'The AI is less certain about this suggestion. Please review and adjust if needed.'
      )
    ).toBeInTheDocument();
  });

  it('does not show low confidence note for high confidence', () => {
    render(<AiSuggestionBadge {...defaultProps} confidence={0.9} />);

    expect(
      screen.queryByText(
        'The AI is less certain about this suggestion. Please review and adjust if needed.'
      )
    ).not.toBeInTheDocument();
  });

  it('calls onAccept when accept button is clicked', () => {
    const onAccept = vi.fn();
    render(<AiSuggestionBadge {...defaultProps} onAccept={onAccept} priority="medium" />);

    const acceptButton = screen.getByText('Accept Suggestion');
    fireEvent.click(acceptButton);

    expect(onAccept).toHaveBeenCalledWith('plumbing', 'medium');
  });

  it('calls onModify when modify button is clicked', () => {
    const onModify = vi.fn();
    render(<AiSuggestionBadge {...defaultProps} onModify={onModify} />);

    const modifyButton = screen.getByText('Modify');
    fireEvent.click(modifyButton);

    expect(onModify).toHaveBeenCalled();
  });

  it('hides action buttons when accepted', () => {
    render(<AiSuggestionBadge {...defaultProps} isAccepted />);

    expect(screen.queryByText('Accept Suggestion')).not.toBeInTheDocument();
    expect(screen.queryByText('Modify')).not.toBeInTheDocument();
  });

  it('displays confidence progress bar', () => {
    render(<AiSuggestionBadge {...defaultProps} confidence={0.75} />);

    const progressBar = screen.getByRole('progressbar');
    expect(progressBar).toBeInTheDocument();
    expect(progressBar).toHaveAttribute('aria-valuenow', '75');
    expect(progressBar).toHaveAttribute('aria-valuemin', '0');
    expect(progressBar).toHaveAttribute('aria-valuemax', '100');
  });

  it('has proper ARIA region for accessibility', () => {
    render(<AiSuggestionBadge {...defaultProps} />);

    const region = screen.getByRole('region', { name: 'AI-suggested category and priority' });
    expect(region).toBeInTheDocument();
  });

  it('applies green styling when accepted', () => {
    const { container } = render(<AiSuggestionBadge {...defaultProps} isAccepted />);

    const badge = container.querySelector('.bg-green-50');
    expect(badge).toBeInTheDocument();
  });

  it('applies confidence-based color for high confidence', () => {
    const { container } = render(<AiSuggestionBadge {...defaultProps} confidence={0.9} />);

    const confidenceBadge = container.querySelector('.bg-green-100');
    expect(confidenceBadge).toBeInTheDocument();
  });

  it('applies confidence-based color for medium confidence', () => {
    const { container } = render(<AiSuggestionBadge {...defaultProps} confidence={0.6} />);

    const confidenceBadge = container.querySelector('.bg-yellow-100');
    expect(confidenceBadge).toBeInTheDocument();
  });

  it('applies confidence-based color for low confidence', () => {
    const { container } = render(<AiSuggestionBadge {...defaultProps} confidence={0.3} />);

    const confidenceBadge = container.querySelector('.bg-red-100');
    expect(confidenceBadge).toBeInTheDocument();
  });
});
