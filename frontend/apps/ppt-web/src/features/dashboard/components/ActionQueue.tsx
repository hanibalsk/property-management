/**
 * Main action queue component showing prioritized items needing attention.
 * Aggregates items from multiple sources and provides inline actions.
 *
 * @module features/dashboard/components/ActionQueue
 */

import { useCallback, useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useNavigate, useSearchParams } from 'react-router-dom';
import type {
  ActionButton,
  ActionItem as ActionItemType,
  ActionQueueFilters,
} from '../hooks/useActionQueue';
import { useActionQueue } from '../hooks/useActionQueue';
import { ActionFilters } from './ActionFilters';
import { ActionItem } from './ActionItem';

interface ActionQueueProps {
  userRole: 'manager' | 'resident';
  onItemAction?: (itemId: string, action: ActionButton['action'], item: ActionItemType) => void;
}

export function ActionQueue({ userRole, onItemAction }: ActionQueueProps) {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [searchParams, setSearchParams] = useSearchParams();
  const [filters, setFilters] = useState<ActionQueueFilters>({});
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [showFilters, setShowFilters] = useState(false);
  const [highlightedItemId, setHighlightedItemId] = useState<string | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const itemRefs = useRef<Map<string, HTMLDivElement>>(new Map());

  const { items, stats, isLoading, isError, error, handleAction, isExecuting, refetch } =
    useActionQueue(userRole, filters);

  // Handle deep link from notification - select and highlight the specified item
  useEffect(() => {
    const itemId = searchParams.get('itemId');
    if (itemId && items.length > 0) {
      const index = items.findIndex((item) => item.id === itemId);
      if (index !== -1) {
        setSelectedIndex(index);
        setHighlightedItemId(itemId);

        // Scroll the item into view
        const itemElement = itemRefs.current.get(itemId);
        if (itemElement) {
          itemElement.scrollIntoView({ behavior: 'smooth', block: 'center' });
          itemElement.focus();
        }

        // Clear the query param after handling (prevents re-highlighting on refetch)
        const newParams = new URLSearchParams(searchParams);
        newParams.delete('itemId');
        setSearchParams(newParams, { replace: true });

        // Clear highlight after animation
        setTimeout(() => {
          setHighlightedItemId(null);
        }, 2000);
      }
    }
  }, [searchParams, items, setSearchParams]);

  // Handle action execution with navigation
  const handleItemAction = useCallback(
    (itemId: string, action: ActionButton['action']) => {
      const item = items.find((i) => i.id === itemId);
      if (!item) return;

      // Call external handler if provided
      if (onItemAction) {
        onItemAction(itemId, action, item);
      }

      // Navigate to detail pages for 'view' action
      if (action === 'view') {
        const routeMap: Record<string, string> = {
          fault: `/faults/${item.entityId}`,
          budget_approval: `/approvals/${item.entityId}`,
          vote: `/votes/${item.entityId}`,
          message: `/messages/${item.entityId}`,
          meter_reading: '/meters/reading',
          person_months: '/person-months',
          announcement: `/announcements/${item.entityId}`,
        };
        const route = routeMap[item.entityType];
        if (route) {
          navigate(route);
          return;
        }
      }

      // Execute inline action
      handleAction(itemId, action);
    },
    [items, handleAction, navigate, onItemAction]
  );

  // Keyboard navigation
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Only handle if queue container is focused or contains focus
      if (!containerRef.current?.contains(document.activeElement)) {
        return;
      }

      switch (e.key) {
        case 'j':
        case 'ArrowDown':
          e.preventDefault();
          setSelectedIndex((prev) => Math.min(prev + 1, items.length - 1));
          break;
        case 'k':
        case 'ArrowUp':
          e.preventDefault();
          setSelectedIndex((prev) => Math.max(prev - 1, 0));
          break;
        case 'Enter':
          e.preventDefault();
          if (items[selectedIndex]) {
            handleItemAction(items[selectedIndex].id, 'view');
          }
          break;
        case 'a':
          e.preventDefault();
          if (items[selectedIndex]?.actions.some((a) => a.action === 'approve')) {
            handleItemAction(items[selectedIndex].id, 'approve');
          }
          break;
        case 'r':
          e.preventDefault();
          if (items[selectedIndex]?.actions.some((a) => a.action === 'reject')) {
            handleItemAction(items[selectedIndex].id, 'reject');
          }
          break;
        case 'Escape':
          e.preventDefault();
          setShowFilters(false);
          break;
        case '?':
          e.preventDefault();
          // TODO: Show keyboard shortcuts help modal
          break;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [items, selectedIndex, handleItemAction]);

  // Reset selected index when items change
  useEffect(() => {
    if (selectedIndex >= items.length) {
      setSelectedIndex(Math.max(0, items.length - 1));
    }
  }, [items.length, selectedIndex]);

  if (isError) {
    return (
      <div className="bg-red-50 border border-red-200 rounded-lg p-6 text-center">
        <h3 className="text-lg font-semibold text-red-800 mb-2">
          {t('dashboard.errorLoadingQueue')}
        </h3>
        <p className="text-red-600 mb-4">
          {error instanceof Error ? error.message : t('errors.unknown')}
        </p>
        <button
          type="button"
          onClick={() => refetch()}
          className="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2"
        >
          {t('common.tryAgain')}
        </button>
      </div>
    );
  }

  return (
    <div ref={containerRef} className="space-y-4" tabIndex={-1}>
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-xl font-semibold text-gray-900">
            {t(`dashboard.${userRole}ActionQueue`)}
          </h2>
          <p className="text-sm text-gray-500">
            {t('dashboard.itemsNeedingAttention', { count: stats.total })}
          </p>
        </div>

        <div className="flex items-center gap-3">
          {/* Quick stats */}
          <div className="hidden md:flex items-center gap-2">
            {stats.urgent > 0 && (
              <span className="px-2 py-1 text-xs font-medium rounded-full bg-red-100 text-red-800">
                {stats.urgent} {t('dashboard.priority.urgent')}
              </span>
            )}
            {stats.high > 0 && (
              <span className="px-2 py-1 text-xs font-medium rounded-full bg-orange-100 text-orange-800">
                {stats.high} {t('dashboard.priority.high')}
              </span>
            )}
          </div>

          {/* Filter toggle */}
          <button
            type="button"
            onClick={() => setShowFilters(!showFilters)}
            className={`
              px-3 py-2 text-sm font-medium rounded-md border
              transition-colors duration-150
              focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
              ${
                showFilters
                  ? 'bg-blue-600 text-white border-blue-600'
                  : 'bg-white text-gray-700 border-gray-300 hover:bg-gray-50'
              }
            `}
            aria-expanded={showFilters}
          >
            <span className="mr-1">🔍</span>
            {t('dashboard.filters')}
          </button>

          {/* Refresh button */}
          <button
            type="button"
            onClick={() => refetch()}
            disabled={isLoading}
            className="px-3 py-2 text-sm font-medium rounded-md border border-gray-300 bg-white text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
            aria-label={t('common.refresh')}
          >
            <span className={isLoading ? 'animate-spin inline-block' : ''}>🔄</span>
          </button>
        </div>
      </div>

      {/* Filters */}
      {showFilters && (
        <ActionFilters
          filters={filters}
          onFilterChange={setFilters}
          stats={stats}
          userRole={userRole}
        />
      )}

      {/* Loading state */}
      {isLoading && items.length === 0 && (
        <div className="flex items-center justify-center py-12">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
        </div>
      )}

      {/* Empty state */}
      {!isLoading && items.length === 0 && (
        <div className="bg-green-50 border border-green-200 rounded-lg p-8 text-center">
          <span className="text-4xl mb-4 block" aria-hidden="true">
            ✅
          </span>
          <h3 className="text-lg font-semibold text-green-800 mb-2">
            {t('dashboard.allCaughtUp')}
          </h3>
          <p className="text-green-600">{t('dashboard.noItemsNeedingAttention')}</p>
        </div>
      )}

      {/* Action items */}
      {items.length > 0 && (
        <div className="space-y-3" role="list" aria-label={t('dashboard.actionQueue')}>
          {items.map((item, index) => (
            <ActionItem
              key={item.id}
              ref={(el) => {
                if (el) {
                  itemRefs.current.set(item.id, el);
                } else {
                  itemRefs.current.delete(item.id);
                }
              }}
              item={item}
              isSelected={index === selectedIndex}
              isHighlighted={item.id === highlightedItemId}
              isExecuting={isExecuting}
              onAction={handleItemAction}
              onSelect={() => setSelectedIndex(index)}
            />
          ))}
        </div>
      )}

      {/* Keyboard shortcuts hint */}
      <div className="text-center text-xs text-gray-400 mt-4">
        <span className="hidden md:inline">
          {t('dashboard.keyboardHint')}{' '}
          <kbd className="px-1.5 py-0.5 bg-gray-100 border border-gray-300 rounded text-gray-600">
            j
          </kbd>
          /
          <kbd className="px-1.5 py-0.5 bg-gray-100 border border-gray-300 rounded text-gray-600">
            k
          </kbd>{' '}
          {t('dashboard.toNavigate')},{' '}
          <kbd className="px-1.5 py-0.5 bg-gray-100 border border-gray-300 rounded text-gray-600">
            Enter
          </kbd>{' '}
          {t('dashboard.toOpen')},{' '}
          <kbd className="px-1.5 py-0.5 bg-gray-100 border border-gray-300 rounded text-gray-600">
            ?
          </kbd>{' '}
          {t('dashboard.forHelp')}
        </span>
      </div>
    </div>
  );
}
