/**
 * WelcomeModal component - welcome modal for new users.
 * Epic 10B: User Onboarding (Story 10B.6)
 */

import { useTranslation } from 'react-i18next';
import type { TourWithProgress } from '../types';

interface WelcomeModalProps {
  isOpen: boolean;
  tours: TourWithProgress[];
  userName?: string;
  onStartTour: (tourId: string) => void;
  onSkipAll: () => void;
  onClose: () => void;
}

export function WelcomeModal({
  isOpen,
  tours,
  userName,
  onStartTour,
  onSkipAll,
  onClose,
}: WelcomeModalProps) {
  const { t } = useTranslation();

  if (!isOpen) {
    return null;
  }

  const pendingTours = tours.filter(
    ({ progress }) => !progress?.isCompleted && !progress?.isSkipped
  );

  const firstTour = pendingTours[0];

  return (
    <div className="fixed inset-0 z-50 overflow-y-auto">
      {/* Backdrop */}
      <div className="fixed inset-0 bg-black bg-opacity-50 transition-opacity" onClick={onClose} />

      {/* Modal */}
      <div className="flex min-h-full items-center justify-center p-4">
        <div className="relative bg-white rounded-lg shadow-xl max-w-lg w-full p-6 transform transition-all">
          {/* Close button */}
          <button
            type="button"
            onClick={onClose}
            className="absolute top-4 right-4 text-gray-400 hover:text-gray-600"
          >
            <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <title>Close</title>
              <path
                fillRule="evenodd"
                d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                clipRule="evenodd"
              />
            </svg>
          </button>

          {/* Welcome icon */}
          <div className="mx-auto flex items-center justify-center h-16 w-16 rounded-full bg-blue-100 mb-4">
            <svg
              className="h-8 w-8 text-blue-600"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <title>Welcome</title>
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"
              />
            </svg>
          </div>

          {/* Title */}
          <h2 className="text-2xl font-bold text-center text-gray-900 mb-2">
            {userName
              ? t('onboarding.welcomeUser', { name: userName })
              : t('onboarding.welcomeTitle')}
          </h2>

          {/* Description */}
          <p className="text-center text-gray-600 mb-6">{t('onboarding.welcomeDescription')}</p>

          {/* Available tours */}
          {pendingTours.length > 0 && (
            <div className="mb-6">
              <h3 className="text-sm font-semibold text-gray-700 mb-3">
                {t('onboarding.availableTours')}
              </h3>
              <div className="space-y-2">
                {pendingTours.map(({ tour }) => (
                  <div
                    key={tour.tourId}
                    className="flex items-center justify-between p-3 bg-gray-50 rounded-lg"
                  >
                    <div>
                      <p className="font-medium text-gray-900">{tour.name}</p>
                      {tour.description && (
                        <p className="text-xs text-gray-500">{tour.description}</p>
                      )}
                      <p className="text-xs text-gray-400 mt-1">
                        {t('onboarding.stepsCount', { count: tour.steps.length })}
                      </p>
                    </div>
                    <button
                      type="button"
                      onClick={() => onStartTour(tour.tourId)}
                      className="px-3 py-1.5 text-sm font-medium text-blue-600 hover:text-blue-800"
                    >
                      {t('onboarding.startTour')}
                    </button>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Actions */}
          <div className="flex flex-col gap-3">
            {firstTour && (
              <button
                type="button"
                onClick={() => onStartTour(firstTour.tour.tourId)}
                className="w-full px-4 py-2.5 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
              >
                {t('onboarding.startFirstTour')}
              </button>
            )}
            <div className="flex gap-3">
              <button
                type="button"
                onClick={onSkipAll}
                className="flex-1 px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
              >
                {t('onboarding.skipAll')}
              </button>
              <button
                type="button"
                onClick={onClose}
                className="flex-1 px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
              >
                {t('onboarding.remindLater')}
              </button>
            </div>
          </div>

          {/* Note */}
          <p className="mt-4 text-xs text-center text-gray-400">{t('onboarding.canAccessLater')}</p>
        </div>
      </div>
    </div>
  );
}
