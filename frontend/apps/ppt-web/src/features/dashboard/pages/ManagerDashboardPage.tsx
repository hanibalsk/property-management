/**
 * Manager dashboard page with action-first UX pattern.
 * Shows a prioritized queue of items needing manager attention.
 *
 * @module features/dashboard/pages/ManagerDashboardPage
 */

import { useTranslation } from 'react-i18next';
import { ActionQueue } from '../components/ActionQueue';
import type { ActionButton, ActionItem } from '../hooks/useActionQueue';

interface ManagerDashboardPageProps {
  onItemAction?: (itemId: string, action: ActionButton['action'], item: ActionItem) => void;
}

export function ManagerDashboardPage({ onItemAction }: ManagerDashboardPageProps) {
  const { t } = useTranslation();

  return (
    <div className="max-w-5xl mx-auto px-4 py-6">
      {/* Page Header */}
      <header className="mb-8">
        <h1 className="text-2xl font-bold text-gray-900 mb-2">{t('dashboard.managerDashboard')}</h1>
        <p className="text-gray-600">{t('dashboard.managerWelcome')}</p>
      </header>

      {/* Quick Stats Summary */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
        <QuickStat label={t('dashboard.stats.pendingFaults')} value="3" trend="up" color="red" />
        <QuickStat
          label={t('dashboard.stats.pendingApprovals')}
          value="2"
          trend="neutral"
          color="orange"
        />
        <QuickStat label={t('dashboard.stats.activeVotes')} value="1" trend="down" color="blue" />
        <QuickStat label={t('dashboard.stats.unreadMessages')} value="5" trend="up" color="gray" />
      </div>

      {/* Action Queue */}
      <ActionQueue userRole="manager" onItemAction={onItemAction} />
    </div>
  );
}

interface QuickStatProps {
  label: string;
  value: string;
  trend: 'up' | 'down' | 'neutral';
  color: 'red' | 'orange' | 'blue' | 'green' | 'gray';
}

const colorClasses: Record<QuickStatProps['color'], string> = {
  red: 'bg-red-50 border-red-200 text-red-800',
  orange: 'bg-orange-50 border-orange-200 text-orange-800',
  blue: 'bg-blue-50 border-blue-200 text-blue-800',
  green: 'bg-green-50 border-green-200 text-green-800',
  gray: 'bg-gray-50 border-gray-200 text-gray-800',
};

const trendIcons: Record<QuickStatProps['trend'], string> = {
  up: '↑',
  down: '↓',
  neutral: '→',
};

function QuickStat({ label, value, trend, color }: QuickStatProps) {
  return (
    <div className={`rounded-lg border p-4 ${colorClasses[color]}`}>
      <div className="flex items-center justify-between">
        <span className="text-3xl font-bold">{value}</span>
        <span className="text-lg">{trendIcons[trend]}</span>
      </div>
      <p className="text-sm mt-1 opacity-80">{label}</p>
    </div>
  );
}
