/**
 * ChatInput Component Tests
 * Epic 127: AI Chatbot Interface
 */

/// <reference types="vitest/globals" />
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { ChatInput } from './ChatInput';

// Mock i18next
vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => {
      const translations: Record<string, string> = {
        'aiChat.inputPlaceholder': 'Type your message...',
        'aiChat.messageInput': 'Message input',
        'aiChat.pressEnter': 'Press Enter to send',
        'aiChat.send': 'Send message',
      };
      return translations[key] || key;
    },
  }),
}));

describe('ChatInput', () => {
  it('renders textarea with placeholder', () => {
    render(<ChatInput onSend={vi.fn()} />);

    expect(screen.getByPlaceholderText('Type your message...')).toBeInTheDocument();
  });

  it('renders send button', () => {
    render(<ChatInput onSend={vi.fn()} />);

    expect(screen.getByLabelText('Send message')).toBeInTheDocument();
  });

  it('disables send button when input is empty', () => {
    render(<ChatInput onSend={vi.fn()} />);

    expect(screen.getByLabelText('Send message')).toBeDisabled();
  });

  it('enables send button when input has text', async () => {
    const user = userEvent.setup();
    render(<ChatInput onSend={vi.fn()} />);

    await user.type(screen.getByLabelText('Message input'), 'Hello');

    expect(screen.getByLabelText('Send message')).not.toBeDisabled();
  });

  it('calls onSend when send button is clicked', async () => {
    const user = userEvent.setup();
    const onSend = vi.fn();
    render(<ChatInput onSend={onSend} />);

    await user.type(screen.getByLabelText('Message input'), 'Hello');
    await user.click(screen.getByLabelText('Send message'));

    expect(onSend).toHaveBeenCalledWith('Hello');
  });

  it('calls onSend when Enter is pressed', async () => {
    const user = userEvent.setup();
    const onSend = vi.fn();
    render(<ChatInput onSend={onSend} />);

    await user.type(screen.getByLabelText('Message input'), 'Hello{Enter}');

    expect(onSend).toHaveBeenCalledWith('Hello');
  });

  it('does not call onSend when Shift+Enter is pressed', async () => {
    const user = userEvent.setup();
    const onSend = vi.fn();
    render(<ChatInput onSend={onSend} />);

    await user.type(screen.getByLabelText('Message input'), 'Hello{Shift>}{Enter}{/Shift}');

    expect(onSend).not.toHaveBeenCalled();
  });

  it('clears input after sending', async () => {
    const user = userEvent.setup();
    render(<ChatInput onSend={vi.fn()} />);

    const input = screen.getByLabelText('Message input');
    await user.type(input, 'Hello{Enter}');

    expect(input).toHaveValue('');
  });

  it('trims whitespace from message', async () => {
    const user = userEvent.setup();
    const onSend = vi.fn();
    render(<ChatInput onSend={onSend} />);

    await user.type(screen.getByLabelText('Message input'), '  Hello  ');
    await user.click(screen.getByLabelText('Send message'));

    expect(onSend).toHaveBeenCalledWith('Hello');
  });

  it('does not send empty messages after trimming', async () => {
    const user = userEvent.setup();
    const onSend = vi.fn();
    render(<ChatInput onSend={onSend} />);

    await user.type(screen.getByLabelText('Message input'), '   ');
    await user.click(screen.getByLabelText('Send message'));

    expect(onSend).not.toHaveBeenCalled();
  });

  it('disables input when disabled prop is true', () => {
    render(<ChatInput onSend={vi.fn()} disabled={true} />);

    expect(screen.getByLabelText('Message input')).toBeDisabled();
    expect(screen.getByLabelText('Send message')).toBeDisabled();
  });

  it('shows loading spinner when disabled', () => {
    render(<ChatInput onSend={vi.fn()} disabled={true} />);

    const button = screen.getByLabelText('Send message');
    expect(button.querySelector('.animate-spin')).toBeInTheDocument();
  });

  it('uses custom placeholder when provided', () => {
    render(<ChatInput onSend={vi.fn()} placeholder="Ask me anything..." />);

    expect(screen.getByPlaceholderText('Ask me anything...')).toBeInTheDocument();
  });
});
