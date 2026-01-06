/**
 * SubmitReadingPage - page to submit a new meter reading.
 * Meters feature: Self-readings and consumption tracking.
 */

import { useTranslation } from 'react-i18next';
import { ReadingForm } from '../components/ReadingForm';
import type { Meter, ReadingFormData } from '../types';

interface SubmitReadingPageProps {
  meter: Meter;
  isSubmitting?: boolean;
  isLoading?: boolean;
  onSubmit: (data: ReadingFormData) => void;
  onCancel: () => void;
}

export function SubmitReadingPage({
  meter,
  isSubmitting,
  isLoading,
  onSubmit,
  onCancel,
}: SubmitReadingPageProps) {
  const { t } = useTranslation();

  if (isLoading) {
    return (
      <div className="flex justify-center py-12">
        <div className="animate-spin rounded-full h-10 w-10 border-b-2 border-blue-600" />
      </div>
    );
  }

  return (
    <div className="max-w-2xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-6">
        <button
          type="button"
          onClick={onCancel}
          className="text-sm text-blue-600 hover:text-blue-800 flex items-center gap-1 mb-4"
        >
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M15 19l-7-7 7-7"
            />
          </svg>
          {t('meters.backToMeter')}
        </button>
        <h1 className="text-2xl font-bold text-gray-900">{t('meters.submitReading')}</h1>
        <p className="text-gray-600 mt-1">
          {t(`meters.types.${meter.meterType}`)} - {meter.serialNumber}
        </p>
      </div>

      {/* Form */}
      <div className="bg-white rounded-lg shadow p-6">
        <ReadingForm
          meter={meter}
          isSubmitting={isSubmitting}
          onSubmit={onSubmit}
          onCancel={onCancel}
        />
      </div>

      {/* Tips */}
      <div className="mt-6 bg-blue-50 rounded-lg p-4 border border-blue-100">
        <h3 className="text-sm font-medium text-blue-900 mb-2">{t('meters.tips.title')}</h3>
        <ul className="text-sm text-blue-800 list-disc list-inside space-y-1">
          <li>{t('meters.tips.readCarefully')}</li>
          <li>{t('meters.tips.takePhoto')}</li>
          <li>{t('meters.tips.reportIssues')}</li>
        </ul>
      </div>
    </div>
  );
}
