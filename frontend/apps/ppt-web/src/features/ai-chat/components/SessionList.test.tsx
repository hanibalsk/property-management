/**
 * SessionList Component Tests
 * Epic 127: AI Chatbot Interface
 */

import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import type { ChatSessionSummary } from '../types';
import { SessionList } from './SessionList';

// Mock i18next
vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string, params?: Record<string, unknown>) => {
      const translations: Record<string, string> = {
        'aiChat.newConversation': 'New Conversation',
        'aiChat.noConversations': 'No conversations yet',
        'aiChat.startConversationHint': 'Start a new conversation to get help',
        'aiChat.untitledConversation': 'New chat',
        'aiChat.messageCount': `${params?.count} messages`,
        'aiChat.deleteConversation': 'Delete conversation',
      };
      return translations[key] || key;
    },
  }),
}));

const mockSessions: ChatSessionSummary[] = [
  {
    id: 'session-1',
    title: 'Question about faults',
    messageCount: 5,
    lastMessageAt: new Date().toISOString(),
    createdAt: new Date(Date.now() - 3600000).toISOString(),
  },
  {
    id: 'session-2',
    title: null as unknown as undefined,
    messageCount: 2,
    lastMessageAt: new Date(Date.now() - 86400000).toISOString(),
    createdAt: new Date(Date.now() - 172800000).toISOString(),
  },
  {
    id: 'session-3',
    title: 'Voting help',
    messageCount: 10,
    lastMessageAt: new Date(Date.now() - 604800000).toISOString(),
    createdAt: new Date(Date.now() - 604800000).toISOString(),
  },
];

describe('SessionList', () => {
  it('renders new conversation button', () => {
    render(
      <SessionList
        sessions={[]}
        currentSessionId={null}
        onSelectSession={vi.fn()}
        onNewSession={vi.fn()}
        onDeleteSession={vi.fn()}
      />
    );

    expect(screen.getByRole('button', { name: /new conversation/i })).toBeInTheDocument();
  });

  it('calls onNewSession when new conversation button is clicked', async () => {
    const user = userEvent.setup();
    const onNewSession = vi.fn();
    render(
      <SessionList
        sessions={[]}
        currentSessionId={null}
        onSelectSession={vi.fn()}
        onNewSession={onNewSession}
        onDeleteSession={vi.fn()}
      />
    );

    await user.click(screen.getByRole('button', { name: /new conversation/i }));

    expect(onNewSession).toHaveBeenCalled();
  });

  it('displays empty state when no sessions', () => {
    render(
      <SessionList
        sessions={[]}
        currentSessionId={null}
        onSelectSession={vi.fn()}
        onNewSession={vi.fn()}
        onDeleteSession={vi.fn()}
      />
    );

    expect(screen.getByText('No conversations yet')).toBeInTheDocument();
    expect(screen.getByText('Start a new conversation to get help')).toBeInTheDocument();
  });

  it('displays loading skeleton when loading', () => {
    render(
      <SessionList
        sessions={[]}
        currentSessionId={null}
        onSelectSession={vi.fn()}
        onNewSession={vi.fn()}
        onDeleteSession={vi.fn()}
        isLoading={true}
      />
    );

    // Should show skeleton items (elements with animate-pulse class)
    const skeletons = document.querySelectorAll('.animate-pulse');
    expect(skeletons.length).toBeGreaterThan(0);
  });

  it('renders session titles', () => {
    render(
      <SessionList
        sessions={mockSessions}
        currentSessionId={null}
        onSelectSession={vi.fn()}
        onNewSession={vi.fn()}
        onDeleteSession={vi.fn()}
      />
    );

    expect(screen.getByText('Question about faults')).toBeInTheDocument();
    expect(screen.getByText('Voting help')).toBeInTheDocument();
  });

  it('shows "New chat" for untitled sessions', () => {
    render(
      <SessionList
        sessions={mockSessions}
        currentSessionId={null}
        onSelectSession={vi.fn()}
        onNewSession={vi.fn()}
        onDeleteSession={vi.fn()}
      />
    );

    expect(screen.getByText('New chat')).toBeInTheDocument();
  });

  it('displays message count for each session', () => {
    render(
      <SessionList
        sessions={mockSessions}
        currentSessionId={null}
        onSelectSession={vi.fn()}
        onNewSession={vi.fn()}
        onDeleteSession={vi.fn()}
      />
    );

    expect(screen.getByText('5 messages')).toBeInTheDocument();
    expect(screen.getByText('2 messages')).toBeInTheDocument();
    expect(screen.getByText('10 messages')).toBeInTheDocument();
  });

  it('highlights current session', () => {
    render(
      <SessionList
        sessions={mockSessions}
        currentSessionId="session-1"
        onSelectSession={vi.fn()}
        onNewSession={vi.fn()}
        onDeleteSession={vi.fn()}
      />
    );

    // The highlighted session's list item should have the bg-blue-50 class
    const listItems = screen.getAllByRole('listitem');
    const currentItem = listItems.find((item) => item.classList.contains('bg-blue-50'));
    expect(currentItem).toBeInTheDocument();
    // Verify the button inside has aria-current="true"
    const currentButton = currentItem?.querySelector('[aria-current="true"]');
    expect(currentButton).toBeInTheDocument();
  });

  it('calls onSelectSession when session is clicked', async () => {
    const user = userEvent.setup();
    const onSelectSession = vi.fn();
    render(
      <SessionList
        sessions={mockSessions}
        currentSessionId={null}
        onSelectSession={onSelectSession}
        onNewSession={vi.fn()}
        onDeleteSession={vi.fn()}
      />
    );

    await user.click(screen.getByText('Question about faults'));

    expect(onSelectSession).toHaveBeenCalledWith('session-1');
  });

  it('calls onDeleteSession when delete button is clicked', async () => {
    const user = userEvent.setup();
    const onDeleteSession = vi.fn();
    render(
      <SessionList
        sessions={mockSessions}
        currentSessionId={null}
        onSelectSession={vi.fn()}
        onNewSession={vi.fn()}
        onDeleteSession={onDeleteSession}
      />
    );

    const deleteButtons = screen.getAllByLabelText('Delete conversation');
    await user.click(deleteButtons[0]);

    expect(onDeleteSession).toHaveBeenCalledWith('session-1');
  });

  it('does not call onSelectSession when delete button is clicked', async () => {
    const user = userEvent.setup();
    const onSelectSession = vi.fn();
    const onDeleteSession = vi.fn();
    render(
      <SessionList
        sessions={mockSessions}
        currentSessionId={null}
        onSelectSession={onSelectSession}
        onNewSession={vi.fn()}
        onDeleteSession={onDeleteSession}
      />
    );

    const deleteButtons = screen.getAllByLabelText('Delete conversation');
    await user.click(deleteButtons[0]);

    expect(onDeleteSession).toHaveBeenCalled();
    expect(onSelectSession).not.toHaveBeenCalled();
  });
});
