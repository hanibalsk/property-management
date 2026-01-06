/**
 * CreateCriticalNotificationPage - Form wrapper for creating critical notifications.
 * Epic 8A, Story 8A.2
 */

import type { CreateCriticalNotificationRequest } from '@ppt/api-client';
import { useTranslation } from 'react-i18next';
import { CreateNotificationForm } from '../components';

interface CreateCriticalNotificationPageProps {
  isLoading?: boolean;
  error?: string | null;
  onSubmit: (data: CreateCriticalNotificationRequest) => Promise<void>;
  onCancel: () => void;
}

export function CreateCriticalNotificationPage({
  isLoading,
  error,
  onSubmit,
  onCancel,
}: CreateCriticalNotificationPageProps) {
  const { t } = useTranslation();

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
        {t('criticalNotifications.backToList')}
      </button>

      <div className="bg-white rounded-lg shadow">
        <div className="p-6 border-b">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-red-100 rounded-lg">
              <svg
                className="w-6 h-6 text-red-600"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                />
              </svg>
            </div>
            <div>
              <h1 className="text-2xl font-bold text-gray-900">
                {t('criticalNotifications.createNew')}
              </h1>
              <p className="mt-1 text-sm text-gray-500">
                {t('criticalNotifications.createDescription')}
              </p>
            </div>
          </div>
        </div>

        <div className="p-6">
          <CreateNotificationForm
            onSubmit={onSubmit}
            onCancel={onCancel}
            isSubmitting={isLoading}
            error={error}
          />
        </div>
      </div>
    </div>
  );
}
