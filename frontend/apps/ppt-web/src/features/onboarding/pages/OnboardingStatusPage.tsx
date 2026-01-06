/**
 * OnboardingStatusPage - overview of all tours and progress.
 * Epic 10B: User Onboarding (Story 10B.6)
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { OnboardingProgress } from '../components/OnboardingProgress';
import { TourCard } from '../components/TourCard';
import type { TourStatus, TourWithProgress } from '../types';
import { getTourDisplayStatus } from '../types';

export interface OnboardingStatusPageProps {
  tours: TourWithProgress[];
  isLoading?: boolean;
  onNavigateToTour: (tourId: string) => void;
  onStartTour: (tourId: string) => void;
  onResetTour: (tourId: string) => void;
}

export function OnboardingStatusPage({
  tours,
  isLoading,
  onNavigateToTour,
  onStartTour,
  onResetTour,
}: OnboardingStatusPageProps) {
  const { t } = useTranslation();
  const [filter, setFilter] = useState<TourStatus>('all');

  const filteredTours = tours.filter(({ progress }) => {
    if (filter === 'all') return true;
    const status = getTourDisplayStatus(progress);
    return status === filter;
  });

  const counts = {
    all: tours.length,
    not_started: tours.filter(({ progress }) => getTourDisplayStatus(progress) === 'not_started')
      .length,
    in_progress: tours.filter(({ progress }) => getTourDisplayStatus(progress) === 'in_progress')
      .length,
    completed: tours.filter(({ progress }) => getTourDisplayStatus(progress) === 'completed')
      .length,
    skipped: tours.filter(({ progress }) => getTourDisplayStatus(progress) === 'skipped').length,
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-gray-900">{t('onboarding.title')}</h1>
        <p className="mt-1 text-sm text-gray-500">{t('onboarding.subtitle')}</p>
      </div>

      {/* Overall progress */}
      <div className="mb-6">
        <OnboardingProgress tours={tours} showDetails />
      </div>

      {/* Filter tabs */}
      <div className="flex flex-wrap gap-2 mb-6">
        <FilterButton
          label={t('onboarding.filter.all')}
          count={counts.all}
          isActive={filter === 'all'}
          onClick={() => setFilter('all')}
        />
        <FilterButton
          label={t('onboarding.filter.notStarted')}
          count={counts.not_started}
          isActive={filter === 'not_started'}
          onClick={() => setFilter('not_started')}
          color="gray"
        />
        <FilterButton
          label={t('onboarding.filter.inProgress')}
          count={counts.in_progress}
          isActive={filter === 'in_progress'}
          onClick={() => setFilter('in_progress')}
          color="blue"
        />
        <FilterButton
          label={t('onboarding.filter.completed')}
          count={counts.completed}
          isActive={filter === 'completed'}
          onClick={() => setFilter('completed')}
          color="green"
        />
        <FilterButton
          label={t('onboarding.filter.skipped')}
          count={counts.skipped}
          isActive={filter === 'skipped'}
          onClick={() => setFilter('skipped')}
          color="yellow"
        />
      </div>

      {/* Empty state */}
      {filteredTours.length === 0 && (
        <div className="text-center py-12 bg-white rounded-lg shadow">
          <svg
            className="mx-auto h-12 w-12 text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <title>No tours</title>
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={1.5}
              d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <p className="mt-4 text-gray-500">{t('onboarding.noTours')}</p>
          {filter !== 'all' && (
            <button
              type="button"
              onClick={() => setFilter('all')}
              className="mt-2 text-sm text-blue-600 hover:text-blue-800"
            >
              {t('onboarding.showAllTours')}
            </button>
          )}
        </div>
      )}

      {/* Tours grid */}
      {filteredTours.length > 0 && (
        <div className="grid gap-4 md:grid-cols-2">
          {filteredTours.map(({ tour, progress }) => (
            <TourCard
              key={tour.tourId}
              tour={tour}
              progress={progress}
              onStart={onStartTour}
              onResume={onNavigateToTour}
              onView={onNavigateToTour}
              onReset={onResetTour}
            />
          ))}
        </div>
      )}
    </div>
  );
}

interface FilterButtonProps {
  label: string;
  count: number;
  isActive: boolean;
  onClick: () => void;
  color?: 'gray' | 'blue' | 'green' | 'yellow';
}

function FilterButton({ label, count, isActive, onClick, color = 'gray' }: FilterButtonProps) {
  const activeColors = {
    gray: 'bg-gray-900 text-white',
    blue: 'bg-blue-600 text-white',
    green: 'bg-green-600 text-white',
    yellow: 'bg-yellow-500 text-white',
  };

  return (
    <button
      type="button"
      onClick={onClick}
      className={`px-3 py-1.5 text-sm font-medium rounded-md transition-colors ${
        isActive
          ? activeColors[color]
          : 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50'
      }`}
    >
      {label} ({count})
    </button>
  );
}
