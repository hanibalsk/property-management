/**
 * DelegationActivityItem - displays a single activity/event in delegation history.
 * Epic 3: Ownership Management (Story 3.4) - UC-28 Delegation History
 */

import { useTranslation } from 'react-i18next';
import type { DelegationActivity, DelegationActivityType } from '../types';

interface DelegationActivityItemProps {
  activity: DelegationActivity;
  /** Whether to show the full date or relative time */
  showFullDate?: boolean;
  /** Whether this is the first item (affects styling) - reserved for future use */
  isFirst?: boolean;
  /** Whether this is the last item (affects styling) */
  isLast?: boolean;
}

const activityIcons: Record<DelegationActivityType, string> = {
  created: 'M12 4v16m8-8H4',
  accepted: 'M5 13l4 4L19 7',
  declined: 'M6 18L18 6M6 6l12 12',
  revoked:
    'M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636',
  expired: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z',
  modified:
    'M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z',
};

const activityColors: Record<DelegationActivityType, string> = {
  created: 'bg-blue-100 text-blue-600',
  accepted: 'bg-green-100 text-green-600',
  declined: 'bg-red-100 text-red-600',
  revoked: 'bg-orange-100 text-orange-600',
  expired: 'bg-gray-100 text-gray-600',
  modified: 'bg-purple-100 text-purple-600',
};

const lineColors: Record<DelegationActivityType, string> = {
  created: 'bg-blue-300',
  accepted: 'bg-green-300',
  declined: 'bg-red-300',
  revoked: 'bg-orange-300',
  expired: 'bg-gray-300',
  modified: 'bg-purple-300',
};

export function DelegationActivityItem({
  activity,
  showFullDate = false,
  isFirst: _isFirst = false,
  isLast = false,
}: DelegationActivityItemProps) {
  // _isFirst is reserved for future styling enhancements (e.g., special first-item styling)
  void _isFirst;
  const { t } = useTranslation();

  const activityLabels: Record<DelegationActivityType, string> = {
    created: t('delegation.history.activityCreated'),
    accepted: t('delegation.history.activityAccepted'),
    declined: t('delegation.history.activityDeclined'),
    revoked: t('delegation.history.activityRevoked'),
    expired: t('delegation.history.activityExpired'),
    modified: t('delegation.history.activityModified'),
  };

  const formatDate = (dateString: string): string => {
    const date = new Date(dateString);
    if (showFullDate) {
      return date.toLocaleString();
    }
    // Show relative time for recent activities
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));
    const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
    const diffMinutes = Math.floor(diffMs / (1000 * 60));

    if (diffMinutes < 60) {
      return t('delegation.history.minutesAgo', { count: diffMinutes });
    }
    if (diffHours < 24) {
      return t('delegation.history.hoursAgo', { count: diffHours });
    }
    if (diffDays < 7) {
      return t('delegation.history.daysAgo', { count: diffDays });
    }
    return date.toLocaleDateString();
  };

  return (
    <div className="relative flex gap-4">
      {/* Timeline line */}
      {!isLast && (
        <div
          className={`absolute left-5 top-10 w-0.5 h-full -translate-x-1/2 ${lineColors[activity.activityType]}`}
        />
      )}

      {/* Icon */}
      <div
        className={`relative z-10 flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-full ${activityColors[activity.activityType]}`}
      >
        <svg
          className="h-5 w-5"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
          aria-hidden="true"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d={activityIcons[activity.activityType]}
          />
        </svg>
      </div>

      {/* Content */}
      <div className={`flex-1 ${isLast ? '' : 'pb-6'}`}>
        <div className="flex items-start justify-between">
          <div>
            <p className="text-sm font-medium text-gray-900">
              {activityLabels[activity.activityType]}
            </p>
            <p className="text-sm text-gray-500">
              {t('delegation.history.byUser', { name: activity.performedByName })}
            </p>
          </div>
          <time
            className="text-xs text-gray-400"
            dateTime={activity.performedAt}
            title={new Date(activity.performedAt).toLocaleString()}
          >
            {formatDate(activity.performedAt)}
          </time>
        </div>

        {/* Status change indicator */}
        {activity.previousStatus && activity.newStatus && (
          <div className="mt-2 flex items-center gap-2 text-xs">
            <span className="text-gray-400">{t('delegation.history.statusChange')}:</span>
            <span className="px-2 py-0.5 bg-gray-100 text-gray-600 rounded">
              {t(
                `delegation.status${activity.previousStatus.charAt(0).toUpperCase()}${activity.previousStatus.slice(1)}`
              )}
            </span>
            <span className="text-gray-400">-&gt;</span>
            <span className="px-2 py-0.5 bg-gray-100 text-gray-600 rounded">
              {t(
                `delegation.status${activity.newStatus.charAt(0).toUpperCase()}${activity.newStatus.slice(1)}`
              )}
            </span>
          </div>
        )}

        {/* Notes */}
        {activity.notes && (
          <div className="mt-2 rounded-md bg-gray-50 p-3">
            <p className="text-sm text-gray-600">{activity.notes}</p>
          </div>
        )}
      </div>
    </div>
  );
}
