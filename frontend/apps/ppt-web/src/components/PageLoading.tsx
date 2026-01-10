/**
 * Page Loading Component
 * Epic 130: Performance Optimization
 *
 * Displays a loading spinner while lazy-loaded pages are being fetched.
 */

import { useTranslation } from 'react-i18next';

export function PageLoading() {
  const { t } = useTranslation();

  return (
    <div
      className="flex items-center justify-center min-h-[50vh]"
      role="status"
      aria-live="polite"
      aria-label={t('common.loading')}
    >
      <div className="flex flex-col items-center gap-4">
        <div
          className="w-10 h-10 border-4 border-blue-600 border-t-transparent rounded-full animate-spin"
          aria-hidden="true"
        />
        <span className="text-gray-600 font-medium">{t('common.loading')}</span>
      </div>
    </div>
  );
}
