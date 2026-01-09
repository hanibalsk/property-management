/**
 * Individual action item card in the action queue.
 * Displays action details and inline action buttons.
 *
 * @module features/dashboard/components/ActionItem
 */

import { useTranslation } from 'react-i18next';
import type { ActionButton, ActionItem as ActionItemType } from '../hooks/useActionQueue';

interface ActionItemProps {
  item: ActionItemType;
  isSelected?: boolean;
  isExecuting?: boolean;
  onAction: (itemId: string, action: ActionButton['action']) => void;
  onSelect?: (itemId: string) => void;
}

const priorityColors: Record<ActionItemType['priority'], string> = {
  urgent: 'bg-red-100 text-red-800 border-red-300',
  high: 'bg-orange-100 text-orange-800 border-orange-300',
  medium: 'bg-yellow-100 text-yellow-800 border-yellow-300',
  low: 'bg-gray-100 text-gray-800 border-gray-300',
};

const priorityBadgeColors: Record<ActionItemType['priority'], string> = {
  urgent: 'bg-red-600 text-white',
  high: 'bg-orange-500 text-white',
  medium: 'bg-yellow-500 text-white',
  low: 'bg-gray-400 text-white',
};

const typeIcons: Record<ActionItemType['type'], string> = {
  fault_pending: '🔧',
  fault_escalated: '⚠️',
  approval_pending: '📋',
  vote_active: '🗳️',
  message_unread: '✉️',
  meter_due: '📊',
  person_months_due: '👥',
  announcement_unread: '📢',
};

const buttonVariants: Record<ActionButton['variant'], string> = {
  primary: 'bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500',
  secondary: 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50 focus:ring-gray-500',
  danger: 'bg-red-600 text-white hover:bg-red-700 focus:ring-red-500',
};

export function ActionItem({
  item,
  isSelected = false,
  isExecuting = false,
  onAction,
  onSelect,
}: ActionItemProps) {
  const { t } = useTranslation();

  const formatTimeAgo = (dateString: string) => {
    const date = new Date(dateString);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / (1000 * 60));
    const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
    const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

    if (diffMins < 60) {
      return t('dashboard.minutesAgo', { count: diffMins });
    }
    if (diffHours < 24) {
      return t('dashboard.hoursAgo', { count: diffHours });
    }
    return t('dashboard.daysAgo', { count: diffDays });
  };

  const formatDueDate = (dateString: string) => {
    const date = new Date(dateString);
    const now = new Date();
    const diffMs = date.getTime() - now.getTime();
    const diffDays = Math.ceil(diffMs / (1000 * 60 * 60 * 24));

    if (diffDays < 0) {
      return t('dashboard.overdue', { days: Math.abs(diffDays) });
    }
    if (diffDays === 0) {
      return t('dashboard.dueToday');
    }
    if (diffDays === 1) {
      return t('dashboard.dueTomorrow');
    }
    return t('dashboard.dueInDays', { days: diffDays });
  };

  return (
    <div
      className={`
        rounded-lg border p-4 transition-all duration-200
        ${priorityColors[item.priority]}
        ${isSelected ? 'ring-2 ring-blue-500 shadow-lg' : 'shadow-sm hover:shadow-md'}
        ${isExecuting ? 'opacity-50 pointer-events-none' : ''}
      `}
      onClick={() => onSelect?.(item.id)}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          onSelect?.(item.id);
        }
      }}
      role="button"
      tabIndex={0}
      aria-selected={isSelected}
      aria-busy={isExecuting}
    >
      <div className="flex items-start justify-between gap-4">
        <div className="flex items-start gap-3 flex-1 min-w-0">
          {/* Icon */}
          <span className="text-2xl flex-shrink-0" aria-hidden="true">
            {typeIcons[item.type]}
          </span>

          {/* Content */}
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2 mb-1">
              <h3 className="font-semibold text-gray-900 truncate">{item.title}</h3>
              <span
                className={`
                  px-2 py-0.5 text-xs font-medium rounded-full flex-shrink-0
                  ${priorityBadgeColors[item.priority]}
                `}
              >
                {t(`dashboard.priority.${item.priority}`)}
              </span>
            </div>

            <p className="text-sm text-gray-600 line-clamp-2">{item.description}</p>

            <div className="flex items-center gap-4 mt-2 text-xs text-gray-500">
              <span>{formatTimeAgo(item.createdAt)}</span>
              {item.dueDate && (
                <span
                  className={
                    item.dueDate < new Date().toISOString() ? 'text-red-600 font-medium' : ''
                  }
                >
                  {formatDueDate(item.dueDate)}
                </span>
              )}
            </div>
          </div>
        </div>

        {/* Inline Actions */}
        <div className="flex items-center gap-2 flex-shrink-0">
          {item.actions.map((action) => (
            <button
              key={action.id}
              type="button"
              onClick={(e) => {
                e.stopPropagation();
                onAction(item.id, action.action);
              }}
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
      </div>
    </div>
  );
}
