/**
 * OnboardingProgress component - displays overall onboarding progress indicator.
 * Epic 10B: User Onboarding (Story 10B.6)
 */

import { useTranslation } from 'react-i18next';
import type { TourWithProgress } from '../types';
import { calculateTourProgress, getTourDisplayStatus } from '../types';

interface OnboardingProgressProps {
  tours: TourWithProgress[];
  showDetails?: boolean;
}

export function OnboardingProgress({ tours, showDetails = false }: OnboardingProgressProps) {
  const { t } = useTranslation();

  const completedTours = tours.filter(
    ({ progress }) => progress?.isCompleted || progress?.isSkipped
  ).length;
  const totalTours = tours.length;
  const overallPercent = totalTours > 0 ? Math.round((completedTours / totalTours) * 100) : 0;

  const inProgressTours = tours.filter(
    ({ progress }) =>
      progress && !progress.isCompleted && !progress.isSkipped && progress.completedSteps.length > 0
  );

  return (
    <div className="bg-white rounded-lg shadow p-4">
      <div className="flex items-center justify-between mb-2">
        <h3 className="text-sm font-semibold text-gray-700">{t('onboarding.overallProgress')}</h3>
        <span className="text-sm font-medium text-gray-900">{overallPercent}%</span>
      </div>

      {/* Overall progress bar */}
      <div className="w-full bg-gray-200 rounded-full h-3 mb-3">
        <div
          className="h-3 rounded-full bg-gradient-to-r from-blue-500 to-green-500 transition-all duration-500"
          style={{ width: `${overallPercent}%` }}
        />
      </div>

      {/* Stats */}
      <div className="flex items-center justify-between text-xs text-gray-500">
        <span>
          {t('onboarding.toursCompleted', { completed: completedTours, total: totalTours })}
        </span>
        {inProgressTours.length > 0 && (
          <span className="text-blue-600">
            {t('onboarding.toursInProgress', { count: inProgressTours.length })}
          </span>
        )}
      </div>

      {/* Detailed breakdown */}
      {showDetails && tours.length > 0 && (
        <div className="mt-4 pt-4 border-t border-gray-200 space-y-2">
          {tours.map(({ tour, progress }) => {
            const status = getTourDisplayStatus(progress);
            const percent = calculateTourProgress(tour, progress);

            return (
              <div key={tour.tourId} className="flex items-center gap-2">
                <div
                  className={`w-2 h-2 rounded-full ${
                    status === 'completed'
                      ? 'bg-green-500'
                      : status === 'in_progress'
                        ? 'bg-blue-500'
                        : status === 'skipped'
                          ? 'bg-yellow-500'
                          : 'bg-gray-300'
                  }`}
                />
                <span className="flex-1 text-xs text-gray-600 truncate">{tour.name}</span>
                <span className="text-xs text-gray-400">{percent}%</span>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}

/**
 * Compact inline progress indicator.
 */
interface OnboardingProgressInlineProps {
  tours: TourWithProgress[];
  onClick?: () => void;
}

export function OnboardingProgressInline({ tours, onClick }: OnboardingProgressInlineProps) {
  const { t } = useTranslation();

  const incompleteTours = tours.filter(
    ({ progress }) => !progress?.isCompleted && !progress?.isSkipped
  );

  if (incompleteTours.length === 0) {
    return null;
  }

  return (
    <button
      type="button"
      onClick={onClick}
      className="flex items-center gap-2 px-3 py-1.5 text-sm bg-blue-50 text-blue-700 rounded-full hover:bg-blue-100 transition-colors"
    >
      <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <title>Onboarding</title>
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={2}
          d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
        />
      </svg>
      <span>{t('onboarding.toursPending', { count: incompleteTours.length })}</span>
    </button>
  );
}
