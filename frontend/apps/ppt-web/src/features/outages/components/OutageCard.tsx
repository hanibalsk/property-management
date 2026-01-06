/**
 * OutageCard component displays a summary of an outage.
 * UC-12: Utility Outages
 */

import { useTranslation } from 'react-i18next';

export type OutageStatus = 'planned' | 'ongoing' | 'resolved' | 'cancelled';
export type OutageCommodity = 'electricity' | 'gas' | 'water' | 'heating' | 'internet' | 'other';
export type OutageSeverity = 'low' | 'medium' | 'high' | 'critical';

export interface OutageSummary {
  id: string;
  title: string;
  commodity: OutageCommodity;
  severity: OutageSeverity;
  status: OutageStatus;
  buildingCount: number;
  scheduledStart: string;
  scheduledEnd?: string;
  createdAt: string;
}

interface OutageCardProps {
  outage: OutageSummary;
  onView?: (id: string) => void;
  onEdit?: (id: string) => void;
}

const statusColors: Record<OutageStatus, string> = {
  planned: 'bg-blue-100 text-blue-800',
  ongoing: 'bg-red-100 text-red-800',
  resolved: 'bg-green-100 text-green-800',
  cancelled: 'bg-gray-100 text-gray-800',
};

const severityColors: Record<OutageSeverity, string> = {
  low: 'text-gray-500',
  medium: 'text-blue-500',
  high: 'text-orange-500',
  critical: 'text-red-600 font-bold',
};

const commodityIcons: Record<OutageCommodity, string> = {
  electricity: '⚡',
  gas: '🔥',
  water: '💧',
  heating: '🌡️',
  internet: '🌐',
  other: '⚠️',
};

export function OutageCard({ outage, onView, onEdit }: OutageCardProps) {
  const { t } = useTranslation();
  const canEdit = outage.status === 'planned' || outage.status === 'ongoing';

  return (
    <div className="bg-white rounded-lg shadow p-4 hover:shadow-md transition-shadow">
      <div className="flex items-start justify-between">
        <div className="flex items-start gap-3">
          <span className="text-2xl" role="img" aria-label={outage.commodity}>
            {commodityIcons[outage.commodity]}
          </span>
          <div>
            <h3 className="font-semibold text-gray-900">{outage.title}</h3>
            <div className="mt-1 flex items-center gap-3 text-sm text-gray-500">
              <span className={severityColors[outage.severity]}>
                {t(`outages.severity.${outage.severity}`)}
              </span>
              <span>•</span>
              <span>
                {outage.buildingCount}{' '}
                {outage.buildingCount === 1 ? t('outages.building') : t('outages.buildings')}
              </span>
            </div>
          </div>
        </div>
        <span className={`px-2 py-1 text-xs font-medium rounded ${statusColors[outage.status]}`}>
          {t(`outages.status.${outage.status}`)}
        </span>
      </div>

      <div className="mt-3 text-sm text-gray-600">
        <div className="flex items-center gap-2">
          <span>{t('outages.scheduled')}:</span>
          <span>{new Date(outage.scheduledStart).toLocaleString()}</span>
          {outage.scheduledEnd && (
            <>
              <span>-</span>
              <span>{new Date(outage.scheduledEnd).toLocaleString()}</span>
            </>
          )}
        </div>
      </div>

      <div className="mt-4 flex items-center gap-2">
        {onView && (
          <button
            type="button"
            onClick={() => onView(outage.id)}
            className="px-3 py-1.5 text-sm font-medium text-blue-600 hover:text-blue-800 hover:bg-blue-50 rounded"
          >
            {t('common.view')}
          </button>
        )}
        {canEdit && onEdit && (
          <button
            type="button"
            onClick={() => onEdit(outage.id)}
            className="px-3 py-1.5 text-sm font-medium text-gray-600 hover:text-gray-800 hover:bg-gray-50 rounded"
          >
            {t('common.edit')}
          </button>
        )}
      </div>
    </div>
  );
}
