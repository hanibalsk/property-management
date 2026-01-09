/**
 * Inline action buttons component for quick actions without navigation.
 * Provides approve/reject/complete actions with confirmation.
 *
 * @module features/dashboard/components/InlineActions
 */

import { useCallback, useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { ActionButton } from '../hooks/useActionQueue';

interface InlineActionsProps {
  actions: ActionButton[];
  itemId: string;
  isExecuting?: boolean;
  onAction: (itemId: string, action: ActionButton['action']) => void;
  requireConfirmation?: ActionButton['action'][];
}

const buttonVariants: Record<ActionButton['variant'], string> = {
  primary: 'bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500',
  secondary: 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50 focus:ring-gray-500',
  danger: 'bg-red-600 text-white hover:bg-red-700 focus:ring-red-500',
};

export function InlineActions({
  actions,
  itemId,
  isExecuting = false,
  onAction,
  requireConfirmation = ['reject', 'escalate'],
}: InlineActionsProps) {
  const { t } = useTranslation();
  const [confirmingAction, setConfirmingAction] = useState<ActionButton['action'] | null>(null);

  const handleClick = useCallback(
    (action: ActionButton['action']) => {
      if (requireConfirmation.includes(action)) {
        setConfirmingAction(action);
      } else {
        onAction(itemId, action);
      }
    },
    [itemId, onAction, requireConfirmation]
  );

  const handleConfirm = useCallback(() => {
    if (confirmingAction) {
      onAction(itemId, confirmingAction);
      setConfirmingAction(null);
    }
  }, [itemId, confirmingAction, onAction]);

  const handleCancel = useCallback(() => {
    setConfirmingAction(null);
  }, []);

  // Show confirmation dialog
  if (confirmingAction) {
    return (
      <div className="flex items-center gap-2 bg-gray-50 rounded-md p-2">
        <span className="text-sm text-gray-600">
          {t('dashboard.confirmAction', {
            action: t(`dashboard.action.${confirmingAction}`),
          })}
        </span>
        <button
          type="button"
          onClick={handleConfirm}
          disabled={isExecuting}
          className="px-3 py-1 text-sm font-medium rounded-md bg-red-600 text-white hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 disabled:opacity-50"
        >
          {t('common.confirm')}
        </button>
        <button
          type="button"
          onClick={handleCancel}
          disabled={isExecuting}
          className="px-3 py-1 text-sm font-medium rounded-md bg-white text-gray-700 border border-gray-300 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 disabled:opacity-50"
        >
          {t('common.cancel')}
        </button>
      </div>
    );
  }

  return (
    <div className="flex items-center gap-2">
      {actions.map((action) => (
        <button
          key={action.id}
          type="button"
          onClick={() => handleClick(action.action)}
          disabled={isExecuting}
          className={`
            px-3 py-1.5 text-sm font-medium rounded-md
            focus:outline-none focus:ring-2 focus:ring-offset-2
            transition-colors duration-150
            disabled:opacity-50 disabled:cursor-not-allowed
            ${buttonVariants[action.variant]}
          `}
        >
          {action.label}
        </button>
      ))}
    </div>
  );
}
