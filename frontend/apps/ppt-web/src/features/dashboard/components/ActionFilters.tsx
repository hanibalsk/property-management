/**
 * Filter controls for the action queue.
 * Allows filtering by type, priority, and search.
 *
 * @module features/dashboard/components/ActionFilters
 */

import { useCallback, useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { ActionPriority, ActionQueueFilters, ActionType } from '../hooks/useActionQueue';

interface ActionFiltersProps {
  filters: ActionQueueFilters;
  onFilterChange: (filters: ActionQueueFilters) => void;
  stats: {
    total: number;
    urgent: number;
    high: number;
    medium: number;
    low: number;
  };
  userRole: 'manager' | 'resident';
}

const managerActionTypes: ActionType[] = [
  'fault_pending',
  'fault_escalated',
  'approval_pending',
  'vote_active',
  'message_unread',
];

const residentActionTypes: ActionType[] = [
  'vote_active',
  'meter_due',
  'person_months_due',
  'announcement_unread',
];

const priorities: ActionPriority[] = ['urgent', 'high', 'medium', 'low'];

const priorityColors: Record<ActionPriority, string> = {
  urgent: 'bg-red-100 text-red-700 border-red-300 hover:bg-red-200',
  high: 'bg-orange-100 text-orange-700 border-orange-300 hover:bg-orange-200',
  medium: 'bg-yellow-100 text-yellow-700 border-yellow-300 hover:bg-yellow-200',
  low: 'bg-gray-100 text-gray-700 border-gray-300 hover:bg-gray-200',
};

const priorityActiveColors: Record<ActionPriority, string> = {
  urgent: 'bg-red-600 text-white border-red-600',
  high: 'bg-orange-500 text-white border-orange-500',
  medium: 'bg-yellow-500 text-white border-yellow-500',
  low: 'bg-gray-500 text-white border-gray-500',
};

export function ActionFilters({ filters, onFilterChange, stats, userRole }: ActionFiltersProps) {
  const { t } = useTranslation();
  const [searchValue, setSearchValue] = useState(filters.search ?? '');

  const actionTypes = userRole === 'manager' ? managerActionTypes : residentActionTypes;

  const handleTypeToggle = useCallback(
    (type: ActionType) => {
      const currentTypes = filters.types ?? [];
      const newTypes = currentTypes.includes(type)
        ? currentTypes.filter((t) => t !== type)
        : [...currentTypes, type];

      onFilterChange({
        ...filters,
        types: newTypes.length > 0 ? newTypes : undefined,
      });
    },
    [filters, onFilterChange]
  );

  const handlePriorityToggle = useCallback(
    (priority: ActionPriority) => {
      const currentPriorities = filters.priorities ?? [];
      const newPriorities = currentPriorities.includes(priority)
        ? currentPriorities.filter((p) => p !== priority)
        : [...currentPriorities, priority];

      onFilterChange({
        ...filters,
        priorities: newPriorities.length > 0 ? newPriorities : undefined,
      });
    },
    [filters, onFilterChange]
  );

  const handleSearchSubmit = useCallback(() => {
    onFilterChange({
      ...filters,
      search: searchValue.trim() || undefined,
    });
  }, [filters, searchValue, onFilterChange]);

  const handleClearFilters = useCallback(() => {
    setSearchValue('');
    onFilterChange({});
  }, [onFilterChange]);

  const hasActiveFilters =
    (filters.types?.length ?? 0) > 0 || (filters.priorities?.length ?? 0) > 0 || !!filters.search;

  return (
    <div className="bg-white rounded-lg border border-gray-200 p-4 space-y-4">
      {/* Search */}
      <div>
        <label htmlFor="action-search" className="block text-sm font-medium text-gray-700 mb-1">
          {t('dashboard.searchActions')}
        </label>
        <div className="flex gap-2">
          <input
            id="action-search"
            type="text"
            value={searchValue}
            onChange={(e) => setSearchValue(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === 'Enter') {
                handleSearchSubmit();
              }
            }}
            placeholder={t('dashboard.searchPlaceholder')}
            className="flex-1 px-3 py-2 border border-gray-300 rounded-md text-sm focus:ring-blue-500 focus:border-blue-500"
          />
          <button
            type="button"
            onClick={handleSearchSubmit}
            className="px-4 py-2 bg-blue-600 text-white text-sm font-medium rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
          >
            {t('common.search')}
          </button>
        </div>
      </div>

      {/* Priority filters */}
      <div>
        <span className="block text-sm font-medium text-gray-700 mb-2">
          {t('dashboard.filterByPriority')}
        </span>
        <div className="flex flex-wrap gap-2">
          {priorities.map((priority) => {
            const isActive = filters.priorities?.includes(priority);
            const count = stats[priority];
            return (
              <button
                key={priority}
                type="button"
                onClick={() => handlePriorityToggle(priority)}
                className={`
                  px-3 py-1.5 text-sm font-medium rounded-md border
                  transition-colors duration-150
                  focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500
                  ${isActive ? priorityActiveColors[priority] : priorityColors[priority]}
                `}
                aria-pressed={isActive}
              >
                {t(`dashboard.priority.${priority}`)}
                {count > 0 && (
                  <span className="ml-1.5 px-1.5 py-0.5 text-xs rounded-full bg-white/20">
                    {count}
                  </span>
                )}
              </button>
            );
          })}
        </div>
      </div>

      {/* Type filters */}
      <div>
        <span className="block text-sm font-medium text-gray-700 mb-2">
          {t('dashboard.filterByType')}
        </span>
        <div className="flex flex-wrap gap-2">
          {actionTypes.map((type) => {
            const isActive = filters.types?.includes(type);
            return (
              <button
                key={type}
                type="button"
                onClick={() => handleTypeToggle(type)}
                className={`
                  px-3 py-1.5 text-sm font-medium rounded-md border
                  transition-colors duration-150
                  focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500
                  ${
                    isActive
                      ? 'bg-blue-600 text-white border-blue-600'
                      : 'bg-white text-gray-700 border-gray-300 hover:bg-gray-50'
                  }
                `}
                aria-pressed={isActive}
              >
                {t(`dashboard.actionType.${type}`)}
              </button>
            );
          })}
        </div>
      </div>

      {/* Clear filters */}
      {hasActiveFilters && (
        <div className="pt-2 border-t border-gray-200">
          <button
            type="button"
            onClick={handleClearFilters}
            className="text-sm text-blue-600 hover:text-blue-800 font-medium"
          >
            {t('dashboard.clearFilters')}
          </button>
        </div>
      )}
    </div>
  );
}
