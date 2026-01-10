/**
 * Resident dashboard page with action-first UX pattern.
 * Shows a prioritized queue of items needing resident attention.
 * Designed for 60-second task completion.
 *
 * @module features/dashboard/pages/ResidentDashboardPage
 */

import { useTranslation } from 'react-i18next';
import { ActionQueue } from '../components/ActionQueue';
import type { ActionButton, ActionItem } from '../hooks/useActionQueue';

interface ResidentDashboardPageProps {
  onItemAction?: (itemId: string, action: ActionButton['action'], item: ActionItem) => void;
}

export function ResidentDashboardPage({ onItemAction }: ResidentDashboardPageProps) {
  const { t } = useTranslation();

  return (
    <div className="max-w-4xl mx-auto px-4 py-6">
      {/* Page Header */}
      <header className="mb-6">
        <h1 className="text-2xl font-bold text-gray-900 mb-2">
          {t('dashboard.residentDashboard')}
        </h1>
        <p className="text-gray-600">{t('dashboard.residentWelcome')}</p>
      </header>

      {/* Quick Tasks Banner */}
      <div className="bg-gradient-to-r from-blue-500 to-blue-600 rounded-lg p-4 mb-6 text-white">
        <div className="flex items-center gap-3">
          <span className="text-3xl" aria-hidden="true">
            ⚡
          </span>
          <div>
            <h2 className="font-semibold">{t('dashboard.quickTasks')}</h2>
            <p className="text-sm opacity-90">{t('dashboard.completeIn60Seconds')}</p>
          </div>
        </div>
      </div>

      {/* Action Queue */}
      <ActionQueue userRole="resident" onItemAction={onItemAction} />

      {/* Recent Activity (Optional) */}
      <section className="mt-8 pt-6 border-t border-gray-200">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">
          {t('dashboard.recentActivity')}
        </h2>
        <div className="space-y-3">
          <ActivityItem
            icon="✅"
            text={t('dashboard.activity.meterReadingSubmitted')}
            time="2 hours ago"
          />
          <ActivityItem icon="🗳️" text={t('dashboard.activity.voteSubmitted')} time="Yesterday" />
          <ActivityItem icon="🔧" text={t('dashboard.activity.faultReported')} time="3 days ago" />
        </div>
      </section>
    </div>
  );
}

interface ActivityItemProps {
  icon: string;
  text: string;
  time: string;
}

function ActivityItem({ icon, text, time }: ActivityItemProps) {
  return (
    <div className="flex items-center gap-3 p-3 bg-gray-50 rounded-lg">
      <span className="text-xl" aria-hidden="true">
        {icon}
      </span>
      <div className="flex-1">
        <p className="text-sm text-gray-700">{text}</p>
        <p className="text-xs text-gray-500">{time}</p>
      </div>
    </div>
  );
}
