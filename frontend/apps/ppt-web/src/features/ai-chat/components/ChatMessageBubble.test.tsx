/**
 * ChatMessageBubble Component Tests
 * Epic 127: AI Chatbot Interface
 */

/// <reference types="vitest/globals" />
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { ChatMessage } from '../types';
import { ChatMessageBubble } from './ChatMessageBubble';

// Mock i18next
vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string, params?: Record<string, unknown>) => {
      const translations: Record<string, string> = {
        'aiChat.messageFrom': `Message from ${params?.role}`,
        'aiChat.confidence.high': 'High confidence',
        'aiChat.confidence.medium': 'Medium confidence',
        'aiChat.confidence.low': 'Low confidence',
        'aiChat.escalated': 'This question may need human assistance',
        'aiChat.sources': `${params?.count} sources`,
        'aiChat.wasHelpful': 'Was this helpful?',
        'aiChat.helpful': 'Mark as helpful',
        'aiChat.notHelpful': 'Mark as not helpful',
        'aiChat.responseTime': `Response time: ${params?.ms}ms`,
      };
      return translations[key] || key;
    },
  }),
}));

const mockUserMessage: ChatMessage = {
  id: 'msg-1',
  sessionId: 'session-1',
  role: 'user',
  content: 'How do I report a fault?',
  sources: [],
  escalated: false,
  createdAt: '2026-01-10T10:00:00Z',
};

const mockAssistantMessage: ChatMessage = {
  id: 'msg-2',
  sessionId: 'session-1',
  role: 'assistant',
  content: 'To report a fault, go to the Faults section in the navigation menu.',
  confidence: 0.85,
  sources: [
    {
      sourceType: 'document',
      sourceId: 'doc-1',
      title: 'User Guide',
      snippet: 'Faults can be reported through...',
      relevanceScore: 0.9,
    },
  ],
  escalated: false,
  latencyMs: 450,
  createdAt: '2026-01-10T10:00:05Z',
};

const mockEscalatedMessage: ChatMessage = {
  id: 'msg-3',
  sessionId: 'session-1',
  role: 'assistant',
  content: "I'm not sure about this question.",
  confidence: 0.3,
  sources: [],
  escalated: true,
  escalationReason: 'Low confidence - suggest human review',
  createdAt: '2026-01-10T10:01:00Z',
};

describe('ChatMessageBubble', () => {
  it('renders user message correctly', () => {
    render(<ChatMessageBubble message={mockUserMessage} />);

    expect(screen.getByText('How do I report a fault?')).toBeInTheDocument();
    // User messages should have blue background
    const article = screen.getByRole('article');
    expect(article.querySelector('.bg-blue-600')).toBeInTheDocument();
  });

  it('renders assistant message correctly', () => {
    render(<ChatMessageBubble message={mockAssistantMessage} />);

    expect(
      screen.getByText('To report a fault, go to the Faults section in the navigation menu.')
    ).toBeInTheDocument();
    // Assistant messages should have gray background
    const article = screen.getByRole('article');
    expect(article.querySelector('.bg-gray-100')).toBeInTheDocument();
  });

  it('displays confidence indicator for assistant messages', () => {
    render(<ChatMessageBubble message={mockAssistantMessage} />);

    expect(screen.getByText('High confidence')).toBeInTheDocument();
    expect(screen.getByText('85%')).toBeInTheDocument();
  });

  it('displays sources toggle when sources exist', async () => {
    const user = userEvent.setup();
    render(<ChatMessageBubble message={mockAssistantMessage} />);

    const sourcesButton = screen.getByRole('button', { name: /source/i });
    expect(sourcesButton).toBeInTheDocument();

    // Initially sources are hidden
    expect(screen.queryByText('User Guide')).not.toBeInTheDocument();

    // Click to show sources
    await user.click(sourcesButton);
    expect(screen.getByText('User Guide')).toBeInTheDocument();
    expect(screen.getByText('Faults can be reported through...')).toBeInTheDocument();
  });

  it('displays escalation warning for escalated messages', () => {
    render(<ChatMessageBubble message={mockEscalatedMessage} />);

    const alert = screen.getByRole('alert');
    expect(alert).toHaveTextContent('Low confidence - suggest human review');
  });

  it('displays response time for assistant messages', () => {
    render(<ChatMessageBubble message={mockAssistantMessage} />);

    expect(screen.getByText('Response time: 450ms')).toBeInTheDocument();
  });

  it('shows feedback buttons for assistant messages', () => {
    const onFeedback = vi.fn();
    render(
      <ChatMessageBubble
        message={mockAssistantMessage}
        onFeedback={onFeedback}
        showFeedback={true}
      />
    );

    expect(screen.getByText('Was this helpful?')).toBeInTheDocument();
    expect(screen.getByLabelText('Mark as helpful')).toBeInTheDocument();
    expect(screen.getByLabelText('Mark as not helpful')).toBeInTheDocument();
  });

  it('calls onFeedback when helpful button is clicked', async () => {
    const user = userEvent.setup();
    const onFeedback = vi.fn();
    render(
      <ChatMessageBubble
        message={mockAssistantMessage}
        onFeedback={onFeedback}
        showFeedback={true}
      />
    );

    await user.click(screen.getByLabelText('Mark as helpful'));
    expect(onFeedback).toHaveBeenCalledWith('msg-2', true);
  });

  it('calls onFeedback when not helpful button is clicked', async () => {
    const user = userEvent.setup();
    const onFeedback = vi.fn();
    render(
      <ChatMessageBubble
        message={mockAssistantMessage}
        onFeedback={onFeedback}
        showFeedback={true}
      />
    );

    await user.click(screen.getByLabelText('Mark as not helpful'));
    expect(onFeedback).toHaveBeenCalledWith('msg-2', false);
  });

  it('disables feedback buttons after feedback is given', async () => {
    const user = userEvent.setup();
    const onFeedback = vi.fn();
    render(
      <ChatMessageBubble
        message={mockAssistantMessage}
        onFeedback={onFeedback}
        showFeedback={true}
      />
    );

    await user.click(screen.getByLabelText('Mark as helpful'));

    // Both buttons should now be disabled
    expect(screen.getByLabelText('Mark as helpful')).toBeDisabled();
    expect(screen.getByLabelText('Mark as not helpful')).toBeDisabled();

    // Clicking again should not trigger callback
    await user.click(screen.getByLabelText('Mark as not helpful'));
    expect(onFeedback).toHaveBeenCalledTimes(1);
  });

  it('does not show feedback buttons for user messages', () => {
    const onFeedback = vi.fn();
    render(
      <ChatMessageBubble message={mockUserMessage} onFeedback={onFeedback} showFeedback={true} />
    );

    expect(screen.queryByText('Was this helpful?')).not.toBeInTheDocument();
  });

  it('hides feedback buttons when showFeedback is false', () => {
    render(<ChatMessageBubble message={mockAssistantMessage} showFeedback={false} />);

    expect(screen.queryByText('Was this helpful?')).not.toBeInTheDocument();
  });
});
