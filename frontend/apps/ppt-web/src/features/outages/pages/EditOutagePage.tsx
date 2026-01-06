/**
 * EditOutagePage - Form wrapper for editing an existing outage.
 * UC-12: Utility Outages
 */

import { useTranslation } from 'react-i18next';
import { OutageForm, type OutageFormData } from '../components';
import type { OutageDetail } from './ViewOutagePage';

interface Building {
  id: string;
  name: string;
  address: string;
}

interface EditOutagePageProps {
  outage: OutageDetail;
  buildings: Building[];
  isLoading?: boolean;
  onSubmit: (data: OutageFormData) => void;
  onCancel: () => void;
}

export function EditOutagePage({
  outage,
  buildings,
  isLoading,
  onSubmit,
  onCancel,
}: EditOutagePageProps) {
  const { t } = useTranslation();

  // Transform outage to form data
  const initialData: Partial<OutageFormData> = {
    title: outage.title,
    description: outage.description,
    commodity: outage.commodity,
    severity: outage.severity,
    buildingIds: outage.buildingIds,
    scheduledStart: outage.scheduledStart
      ? new Date(outage.scheduledStart).toISOString().slice(0, 16)
      : '',
    scheduledEnd: outage.scheduledEnd
      ? new Date(outage.scheduledEnd).toISOString().slice(0, 16)
      : '',
  };

  return (
    <div className="max-w-2xl mx-auto px-4 py-8">
      {/* Back button */}
      <button
        type="button"
        onClick={onCancel}
        className="mb-4 text-blue-600 hover:text-blue-800 flex items-center gap-1"
      >
        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
        </svg>
        {t('outages.backToDetail')}
      </button>

      <div className="bg-white rounded-lg shadow">
        <div className="p-6 border-b">
          <h1 className="text-2xl font-bold text-gray-900">{t('outages.editOutage')}</h1>
          <p className="mt-1 text-sm text-gray-500">
            {t('outages.editing')}: {outage.title}
          </p>
        </div>

        <div className="p-6">
          <OutageForm
            initialData={initialData}
            buildings={buildings}
            isLoading={isLoading}
            onSubmit={onSubmit}
            onCancel={onCancel}
          />
        </div>
      </div>
    </div>
  );
}
