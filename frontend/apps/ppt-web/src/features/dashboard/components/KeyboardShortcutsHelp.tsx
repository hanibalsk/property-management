/**
 * Modal displaying available keyboard shortcuts for the action queue.
 * Triggered by pressing '?' on the dashboard.
 *
 * @module features/dashboard/components/KeyboardShortcutsHelp
 */

import { useCallback, useEffect, useRef } from 'react';
import { useTranslation } from 'react-i18next';

interface KeyboardShortcutsHelpProps {
  isOpen: boolean;
  onClose: () => void;
}

interface ShortcutItem {
  keys: string[];
  descriptionKey: string;
}

const shortcuts: ShortcutItem[] = [
  { keys: ['j', '↓'], descriptionKey: 'dashboard.shortcuts.nextItem' },
  { keys: ['k', '↑'], descriptionKey: 'dashboard.shortcuts.prevItem' },
  { keys: ['Enter'], descriptionKey: 'dashboard.shortcuts.openItem' },
  { keys: ['a'], descriptionKey: 'dashboard.shortcuts.approve' },
  { keys: ['r'], descriptionKey: 'dashboard.shortcuts.reject' },
  { keys: ['Esc'], descriptionKey: 'dashboard.shortcuts.closeFilters' },
  { keys: ['?'], descriptionKey: 'dashboard.shortcuts.showHelp' },
];

export function KeyboardShortcutsHelp({ isOpen, onClose }: KeyboardShortcutsHelpProps) {
  const { t } = useTranslation();
  const dialogRef = useRef<HTMLDivElement>(null);

  // Close on Escape key
  useEffect(() => {
    if (!isOpen) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        e.preventDefault();
        onClose();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, onClose]);

  // Focus trap
  useEffect(() => {
    if (isOpen && dialogRef.current) {
      dialogRef.current.focus();
    }
  }, [isOpen]);

  // Handle backdrop click
  const handleBackdropClick = useCallback(
    (e: React.MouseEvent) => {
      if (e.target === e.currentTarget) {
        onClose();
      }
    },
    [onClose]
  );

  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
      onClick={handleBackdropClick}
      role="presentation"
    >
      <div
        ref={dialogRef}
        className="bg-white rounded-lg shadow-xl max-w-md w-full mx-4 p-6"
        role="dialog"
        aria-modal="true"
        aria-labelledby="shortcuts-title"
        tabIndex={-1}
      >
        <div className="flex items-center justify-between mb-4">
          <h2 id="shortcuts-title" className="text-lg font-semibold text-gray-900">
            {t('dashboard.shortcuts.title')}
          </h2>
          <button
            type="button"
            onClick={onClose}
            className="text-gray-400 hover:text-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500 rounded-md p-1"
            aria-label={t('common.close')}
          >
            <span aria-hidden="true" className="text-xl">
              ✕
            </span>
          </button>
        </div>

        <div className="space-y-3">
          {shortcuts.map((shortcut, index) => (
            <div key={index} className="flex items-center justify-between">
              <span className="text-gray-700">{t(shortcut.descriptionKey)}</span>
              <div className="flex items-center gap-1">
                {shortcut.keys.map((key, keyIndex) => (
                  <span key={keyIndex} className="flex items-center">
                    {keyIndex > 0 && <span className="text-gray-400 mx-1">/</span>}
                    <kbd className="px-2 py-1 bg-gray-100 border border-gray-300 rounded text-sm font-mono text-gray-700">
                      {key}
                    </kbd>
                  </span>
                ))}
              </div>
            </div>
          ))}
        </div>

        <div className="mt-6 pt-4 border-t border-gray-200">
          <p className="text-xs text-gray-500 text-center">{t('dashboard.shortcuts.hint')}</p>
        </div>
      </div>
    </div>
  );
}
