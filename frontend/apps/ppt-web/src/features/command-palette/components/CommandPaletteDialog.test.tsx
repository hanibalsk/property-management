/**
 * CommandPaletteDialog component tests
 * Epic 129: Command Palette
 */

/// <reference types="vitest/globals" />
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { CommandPaletteProvider, useRegisterCommands } from '../hooks';
import type { Command } from '../types';
import { CommandPaletteDialog } from './CommandPaletteDialog';

// Mock useTranslation
vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string, params?: Record<string, unknown>) => {
      const translations: Record<string, string> = {
        'commandPalette.title': 'Command Palette',
        'commandPalette.placeholder': 'Type a command or search...',
        'commandPalette.searchLabel': 'Search commands',
        'commandPalette.resultsLabel': 'Command results',
        'commandPalette.noResults': 'No commands found',
        'commandPalette.navigate': 'to navigate',
        'commandPalette.select': 'to select',
        'commandPalette.shortcutHint': `Press ${params?.shortcut || '⌘K'} to open`,
        'commandPalette.categories.recent': 'Recent',
        'commandPalette.categories.navigation': 'Navigation',
        'commandPalette.categories.create': 'Create',
        'commandPalette.categories.action': 'Actions',
        'commandPalette.categories.search': 'Search',
        'commandPalette.categories.settings': 'Settings',
      };
      return translations[key] || key;
    },
  }),
}));

const mockCommands: Command[] = [
  {
    id: 'go-dashboard',
    label: 'Go to Dashboard',
    description: 'Navigate to main dashboard',
    category: 'navigation',
    keywords: ['home', 'main'],
    icon: '🏠',
    action: vi.fn(),
  },
  {
    id: 'go-buildings',
    label: 'Go to Buildings',
    description: 'View buildings',
    category: 'navigation',
    keywords: ['property', 'building'],
    icon: '🏢',
    action: vi.fn(),
  },
  {
    id: 'create-fault',
    label: 'Create Fault Report',
    description: 'Report a new issue',
    category: 'create',
    keywords: ['report', 'issue'],
    icon: '🔧',
    action: vi.fn(),
  },
  {
    id: 'toggle-dark',
    label: 'Toggle Dark Mode',
    description: 'Switch theme',
    category: 'settings',
    keywords: ['theme', 'dark', 'light'],
    icon: '🌙',
    action: vi.fn(),
  },
];

function TestWrapper({ children }: { children: React.ReactNode }) {
  return <CommandPaletteProvider>{children}</CommandPaletteProvider>;
}

function CommandPaletteWithCommands() {
  useRegisterCommands(mockCommands);
  return <CommandPaletteDialog />;
}

describe('CommandPaletteDialog', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Clear localStorage if available
    if (typeof localStorage !== 'undefined' && localStorage.clear) {
      localStorage.clear();
    }
  });

  describe('opening and closing', () => {
    it('opens with Cmd+K keyboard shortcut', async () => {
      render(
        <TestWrapper>
          <CommandPaletteWithCommands />
        </TestWrapper>
      );

      // Initially closed
      expect(screen.queryByRole('dialog')).not.toBeInTheDocument();

      // Press Cmd+K
      fireEvent.keyDown(document, { key: 'k', metaKey: true });

      await waitFor(() => {
        expect(screen.getByRole('dialog')).toBeInTheDocument();
      });
    });

    it('opens with Ctrl+K keyboard shortcut', async () => {
      render(
        <TestWrapper>
          <CommandPaletteWithCommands />
        </TestWrapper>
      );

      fireEvent.keyDown(document, { key: 'k', ctrlKey: true });

      await waitFor(() => {
        expect(screen.getByRole('dialog')).toBeInTheDocument();
      });
    });

    it('closes with Escape key', async () => {
      render(
        <TestWrapper>
          <CommandPaletteWithCommands />
        </TestWrapper>
      );

      // Open
      fireEvent.keyDown(document, { key: 'k', metaKey: true });
      await waitFor(() => {
        expect(screen.getByRole('dialog')).toBeInTheDocument();
      });

      // Close with Escape
      fireEvent.keyDown(document, { key: 'Escape' });
      await waitFor(() => {
        expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
      });
    });

    it('closes when clicking backdrop', async () => {
      render(
        <TestWrapper>
          <CommandPaletteWithCommands />
        </TestWrapper>
      );

      fireEvent.keyDown(document, { key: 'k', metaKey: true });
      await waitFor(() => {
        expect(screen.getByRole('dialog')).toBeInTheDocument();
      });

      // Click backdrop (the fixed container)
      const backdrop = document.querySelector('.fixed.inset-0');
      if (backdrop) {
        fireEvent.click(backdrop);
      }

      await waitFor(() => {
        expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
      });
    });
  });

  describe('search functionality', () => {
    it('displays all commands when no query', async () => {
      render(
        <TestWrapper>
          <CommandPaletteWithCommands />
        </TestWrapper>
      );

      fireEvent.keyDown(document, { key: 'k', metaKey: true });

      await waitFor(() => {
        expect(screen.getByText('Go to Dashboard')).toBeInTheDocument();
        expect(screen.getByText('Go to Buildings')).toBeInTheDocument();
        expect(screen.getByText('Create Fault Report')).toBeInTheDocument();
        expect(screen.getByText('Toggle Dark Mode')).toBeInTheDocument();
      });
    });

    it('filters commands based on search query', async () => {
      render(
        <TestWrapper>
          <CommandPaletteWithCommands />
        </TestWrapper>
      );

      fireEvent.keyDown(document, { key: 'k', metaKey: true });
      await waitFor(() => {
        expect(screen.getByRole('dialog')).toBeInTheDocument();
      });

      const input = screen.getByRole('combobox');
      fireEvent.change(input, { target: { value: 'dashboard' } });

      await waitFor(() => {
        // Use function matcher since text is split by highlight marks
        expect(
          screen.getByText((_, element) => element?.textContent === 'Go to Dashboard')
        ).toBeInTheDocument();
        expect(screen.queryByText('Go to Buildings')).not.toBeInTheDocument();
        expect(screen.queryByText('Create Fault Report')).not.toBeInTheDocument();
      });
    });

    it('searches by keywords', async () => {
      render(
        <TestWrapper>
          <CommandPaletteWithCommands />
        </TestWrapper>
      );

      fireEvent.keyDown(document, { key: 'k', metaKey: true });
      await waitFor(() => {
        expect(screen.getByRole('dialog')).toBeInTheDocument();
      });

      const input = screen.getByRole('combobox');
      fireEvent.change(input, { target: { value: 'theme' } });

      await waitFor(() => {
        expect(screen.getByText('Toggle Dark Mode')).toBeInTheDocument();
        expect(screen.queryByText('Go to Dashboard')).not.toBeInTheDocument();
      });
    });

    it('shows no results message when no matches', async () => {
      render(
        <TestWrapper>
          <CommandPaletteWithCommands />
        </TestWrapper>
      );

      fireEvent.keyDown(document, { key: 'k', metaKey: true });
      await waitFor(() => {
        expect(screen.getByRole('dialog')).toBeInTheDocument();
      });

      const input = screen.getByRole('combobox');
      fireEvent.change(input, { target: { value: 'xyznonexistent' } });

      await waitFor(() => {
        expect(screen.getByText('No commands found')).toBeInTheDocument();
      });
    });
  });

  describe('keyboard navigation', () => {
    it('navigates with arrow keys', async () => {
      render(
        <TestWrapper>
          <CommandPaletteWithCommands />
        </TestWrapper>
      );

      fireEvent.keyDown(document, { key: 'k', metaKey: true });
      await waitFor(() => {
        expect(screen.getByRole('dialog')).toBeInTheDocument();
      });

      const input = screen.getByRole('combobox');

      // First item should be selected by default
      expect(screen.getByRole('option', { selected: true })).toBeInTheDocument();

      // Navigate down
      fireEvent.keyDown(input, { key: 'ArrowDown' });
      fireEvent.keyDown(input, { key: 'ArrowDown' });

      // Navigate up
      fireEvent.keyDown(input, { key: 'ArrowUp' });

      // Should still have a selected option
      expect(screen.getByRole('option', { selected: true })).toBeInTheDocument();
    });

    it('executes command on Enter', async () => {
      render(
        <TestWrapper>
          <CommandPaletteWithCommands />
        </TestWrapper>
      );

      fireEvent.keyDown(document, { key: 'k', metaKey: true });
      await waitFor(() => {
        expect(screen.getByRole('dialog')).toBeInTheDocument();
      });

      const input = screen.getByRole('combobox');
      fireEvent.keyDown(input, { key: 'Enter' });

      // Command should be executed and dialog closed
      await waitFor(() => {
        expect(mockCommands[0].action).toHaveBeenCalled();
        expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
      });
    });
  });

  describe('command execution', () => {
    it('executes command on click', async () => {
      render(
        <TestWrapper>
          <CommandPaletteWithCommands />
        </TestWrapper>
      );

      fireEvent.keyDown(document, { key: 'k', metaKey: true });
      await waitFor(() => {
        expect(screen.getByRole('dialog')).toBeInTheDocument();
      });

      fireEvent.click(screen.getByText('Create Fault Report'));

      await waitFor(() => {
        expect(mockCommands[2].action).toHaveBeenCalled();
        expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
      });
    });
  });

  describe('categories', () => {
    it('groups commands by category', async () => {
      render(
        <TestWrapper>
          <CommandPaletteWithCommands />
        </TestWrapper>
      );

      fireEvent.keyDown(document, { key: 'k', metaKey: true });

      await waitFor(() => {
        expect(screen.getByText('Navigation')).toBeInTheDocument();
        expect(screen.getByText('Create')).toBeInTheDocument();
        expect(screen.getByText('Settings')).toBeInTheDocument();
      });
    });
  });

  describe('accessibility', () => {
    it('has proper ARIA attributes', async () => {
      render(
        <TestWrapper>
          <CommandPaletteWithCommands />
        </TestWrapper>
      );

      fireEvent.keyDown(document, { key: 'k', metaKey: true });

      await waitFor(() => {
        const dialog = screen.getByRole('dialog');
        expect(dialog).toHaveAttribute('aria-modal', 'true');
        expect(dialog).toHaveAttribute('aria-label', 'Command Palette');

        const input = screen.getByRole('combobox');
        expect(input).toHaveAttribute('aria-expanded', 'true');
        expect(input).toHaveAttribute('aria-haspopup', 'listbox');
      });
    });

    it('has listbox with options', async () => {
      render(
        <TestWrapper>
          <CommandPaletteWithCommands />
        </TestWrapper>
      );

      fireEvent.keyDown(document, { key: 'k', metaKey: true });

      await waitFor(() => {
        expect(screen.getByRole('listbox')).toBeInTheDocument();
        expect(screen.getAllByRole('option').length).toBeGreaterThan(0);
      });
    });
  });
});
