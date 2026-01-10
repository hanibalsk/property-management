/**
 * Command Palette Dialog Component
 * Epic 129: Command Palette
 *
 * A keyboard-accessible quick action interface.
 * Activated with Cmd/Ctrl + K.
 */

import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useCommandPalette } from '../hooks';
import type { Command, CommandCategory, CommandGroup } from '../types';

const CATEGORY_ORDER: CommandCategory[] = [
  'recent',
  'navigation',
  'create',
  'action',
  'search',
  'settings',
];

const CATEGORY_ICONS: Record<CommandCategory, string> = {
  recent: '🕐',
  navigation: '📍',
  create: '➕',
  action: '⚡',
  search: '🔍',
  settings: '⚙️',
};

function highlightMatch(text: string, query: string): React.ReactNode {
  if (!query) return text;
  const lowerText = text.toLowerCase();
  const lowerQuery = query.toLowerCase();
  const index = lowerText.indexOf(lowerQuery);
  if (index === -1) return text;

  return (
    <>
      {text.slice(0, index)}
      <mark className="bg-yellow-200 text-gray-900">{text.slice(index, index + query.length)}</mark>
      {text.slice(index + query.length)}
    </>
  );
}

export function CommandPaletteDialog() {
  const { t } = useTranslation();
  const { isOpen, close, commands, recentCommands, addToRecent } = useCommandPalette();
  const [query, setQuery] = useState('');
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);
  const listRef = useRef<HTMLDivElement>(null);

  // Filter commands based on query
  const filteredCommands = useMemo(() => {
    if (!query.trim()) {
      return commands.filter((c) => !c.disabled);
    }
    const lowerQuery = query.toLowerCase();
    return commands.filter((cmd) => {
      if (cmd.disabled) return false;
      return (
        cmd.label.toLowerCase().includes(lowerQuery) ||
        cmd.description?.toLowerCase().includes(lowerQuery) ||
        cmd.keywords.some((k) => k.toLowerCase().includes(lowerQuery))
      );
    });
  }, [commands, query]);

  // Group commands by category
  const groupedCommands = useMemo((): CommandGroup[] => {
    const groups: CommandGroup[] = [];

    // Add recent commands group if no query and has recent
    if (!query.trim() && recentCommands.length > 0) {
      groups.push({
        category: 'recent',
        label: t('commandPalette.categories.recent'),
        commands: recentCommands,
      });
    }

    // Group remaining commands
    const commandsByCategory = new Map<CommandCategory, Command[]>();
    for (const cmd of filteredCommands) {
      const existing = commandsByCategory.get(cmd.category) || [];
      commandsByCategory.set(cmd.category, [...existing, cmd]);
    }

    for (const category of CATEGORY_ORDER) {
      if (category === 'recent') continue; // Already handled
      const cmds = commandsByCategory.get(category);
      if (cmds && cmds.length > 0) {
        groups.push({
          category,
          label: t(`commandPalette.categories.${category}`),
          commands: cmds,
        });
      }
    }

    return groups;
  }, [filteredCommands, recentCommands, query, t]);

  // Flat list of all visible commands for keyboard navigation
  const flatCommands = useMemo(() => {
    return groupedCommands.flatMap((g) => g.commands);
  }, [groupedCommands]);

  // Reset selection when filter changes
  // biome-ignore lint/correctness/useExhaustiveDependencies: query is intentionally used to trigger effect
  useEffect(() => {
    setSelectedIndex(0);
  }, [query]);

  // Focus input when dialog opens
  useEffect(() => {
    if (isOpen) {
      setQuery('');
      setSelectedIndex(0);
      setTimeout(() => inputRef.current?.focus(), 0);
    }
  }, [isOpen]);

  // Scroll selected item into view
  useEffect(() => {
    if (listRef.current && flatCommands.length > 0) {
      const selectedItem = listRef.current.querySelector(`[data-index="${selectedIndex}"]`);
      if (selectedItem && typeof selectedItem.scrollIntoView === 'function') {
        selectedItem.scrollIntoView({ block: 'nearest' });
      }
    }
  }, [selectedIndex, flatCommands.length]);

  const executeCommand = useCallback(
    (command: Command) => {
      addToRecent(command.id);
      close();
      command.action();
    },
    [addToRecent, close]
  );

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      switch (e.key) {
        case 'ArrowDown':
          e.preventDefault();
          setSelectedIndex((prev) => Math.min(prev + 1, flatCommands.length - 1));
          break;
        case 'ArrowUp':
          e.preventDefault();
          setSelectedIndex((prev) => Math.max(prev - 1, 0));
          break;
        case 'Enter':
          e.preventDefault();
          if (flatCommands[selectedIndex]) {
            executeCommand(flatCommands[selectedIndex]);
          }
          break;
        case 'Escape':
          e.preventDefault();
          close();
          break;
      }
    },
    [flatCommands, selectedIndex, executeCommand, close]
  );

  if (!isOpen) return null;

  let globalIndex = -1;

  return (
    <div
      className="fixed inset-0 z-50 flex items-start justify-center pt-[15vh]"
      onClick={close}
      onKeyDown={(e) => e.key === 'Escape' && close()}
    >
      {/* Backdrop */}
      <div className="absolute inset-0 bg-black/50 backdrop-blur-sm" aria-hidden="true" />

      {/* Dialog */}
      <div
        className="relative w-full max-w-xl bg-white rounded-xl shadow-2xl overflow-hidden"
        onClick={(e) => e.stopPropagation()}
        role="dialog"
        aria-modal="true"
        aria-label={t('commandPalette.title')}
      >
        {/* Search input */}
        <div className="flex items-center px-4 border-b border-gray-200">
          <svg
            className="w-5 h-5 text-gray-400 flex-shrink-0"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
          <input
            ref={inputRef}
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder={t('commandPalette.placeholder')}
            className="flex-1 px-3 py-4 text-gray-900 placeholder-gray-500 bg-transparent border-none focus:outline-none focus:ring-0"
            aria-label={t('commandPalette.searchLabel')}
            aria-activedescendant={
              flatCommands[selectedIndex] ? `cmd-${flatCommands[selectedIndex].id}` : undefined
            }
            aria-controls="command-list"
            role="combobox"
            aria-expanded="true"
            aria-haspopup="listbox"
          />
          <kbd className="hidden sm:inline-flex items-center px-2 py-1 text-xs font-medium text-gray-500 bg-gray-100 rounded">
            esc
          </kbd>
        </div>

        {/* Command list */}
        <div
          ref={listRef}
          id="command-list"
          role="listbox"
          tabIndex={-1}
          className="max-h-[50vh] overflow-y-auto py-2"
          aria-label={t('commandPalette.resultsLabel')}
        >
          {groupedCommands.length === 0 ? (
            <div className="px-4 py-8 text-center text-gray-500">
              {t('commandPalette.noResults')}
            </div>
          ) : (
            groupedCommands.map((group) => (
              <div key={group.category} className="mb-2">
                <div className="px-4 py-1.5 text-xs font-semibold text-gray-500 uppercase tracking-wider flex items-center gap-2">
                  <span>{CATEGORY_ICONS[group.category]}</span>
                  {group.label}
                </div>
                {group.commands.map((cmd) => {
                  globalIndex++;
                  const isSelected = globalIndex === selectedIndex;
                  const currentIndex = globalIndex;

                  return (
                    <button
                      key={cmd.id}
                      id={`cmd-${cmd.id}`}
                      data-index={currentIndex}
                      type="button"
                      role="option"
                      aria-selected={isSelected}
                      onClick={() => executeCommand(cmd)}
                      onMouseEnter={() => setSelectedIndex(currentIndex)}
                      className={`w-full px-4 py-2 flex items-center gap-3 text-left transition-colors ${
                        isSelected ? 'bg-blue-50 text-blue-900' : 'text-gray-700 hover:bg-gray-50'
                      }`}
                    >
                      {cmd.icon && (
                        <span className="text-lg flex-shrink-0" aria-hidden="true">
                          {cmd.icon}
                        </span>
                      )}
                      <div className="flex-1 min-w-0">
                        <div className="font-medium truncate">
                          {highlightMatch(cmd.label, query)}
                        </div>
                        {cmd.description && (
                          <div className="text-sm text-gray-500 truncate">
                            {highlightMatch(cmd.description, query)}
                          </div>
                        )}
                      </div>
                      {cmd.shortcut && (
                        <kbd className="hidden sm:inline-flex items-center px-2 py-0.5 text-xs font-medium text-gray-500 bg-gray-100 rounded flex-shrink-0">
                          {cmd.shortcut}
                        </kbd>
                      )}
                    </button>
                  );
                })}
              </div>
            ))
          )}
        </div>

        {/* Footer */}
        <div className="px-4 py-2 bg-gray-50 border-t border-gray-200 flex items-center justify-between text-xs text-gray-500">
          <div className="flex items-center gap-4">
            <span className="flex items-center gap-1">
              <kbd className="px-1.5 py-0.5 bg-gray-200 rounded">↑</kbd>
              <kbd className="px-1.5 py-0.5 bg-gray-200 rounded">↓</kbd>
              {t('commandPalette.navigate')}
            </span>
            <span className="flex items-center gap-1">
              <kbd className="px-1.5 py-0.5 bg-gray-200 rounded">↵</kbd>
              {t('commandPalette.select')}
            </span>
          </div>
          <span className="hidden sm:block">
            {t('commandPalette.shortcutHint', { shortcut: '⌘K' })}
          </span>
        </div>
      </div>
    </div>
  );
}
