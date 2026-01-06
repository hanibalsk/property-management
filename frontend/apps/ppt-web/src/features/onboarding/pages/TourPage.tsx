/**
 * TourPage - interactive tour walkthrough page.
 * Epic 10B: User Onboarding (Story 10B.6)
 */

import { useTranslation } from 'react-i18next';
import { TourStep, TourStepListItem } from '../components/TourStep';
import type { OnboardingTour, UserOnboardingProgress } from '../types';
import { calculateTourProgress, getCurrentStepIndex, getTourDisplayStatus } from '../types';

export interface TourPageProps {
  tour: OnboardingTour;
  progress: UserOnboardingProgress | null;
  isLoading?: boolean;
  onCompleteStep: (stepId: string) => void;
  onCompleteTour: () => void;
  onSkipTour: () => void;
  onResetTour: () => void;
  onNavigateBack: () => void;
}

export function TourPage({
  tour,
  progress,
  isLoading,
  onCompleteStep,
  onCompleteTour,
  onSkipTour,
  onResetTour,
  onNavigateBack,
}: TourPageProps) {
  const { t } = useTranslation();

  const displayStatus = getTourDisplayStatus(progress);
  const progressPercent = calculateTourProgress(tour, progress);
  const currentStepIndex = getCurrentStepIndex(tour, progress);
  const completedStepIds = progress?.completedSteps ?? [];

  const isCompleted = displayStatus === 'completed';
  const isSkipped = displayStatus === 'skipped';

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
      </div>
    );
  }

  const handleStepComplete = (stepIndex: number) => {
    const step = tour.steps[stepIndex];
    if (step) {
      onCompleteStep(step.id);
    }
  };

  const handlePreviousStep = () => {
    // Previous step handling would be controlled by parent
  };

  const handleFinish = () => {
    const lastStep = tour.steps[tour.steps.length - 1];
    if (lastStep && !completedStepIds.includes(lastStep.id)) {
      onCompleteStep(lastStep.id);
    }
    onCompleteTour();
  };

  return (
    <div className="max-w-4xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-6">
        <button
          type="button"
          onClick={onNavigateBack}
          className="flex items-center text-sm text-gray-600 hover:text-gray-800 mb-4"
        >
          <svg className="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
            <title>Back</title>
            <path
              fillRule="evenodd"
              d="M9.707 16.707a1 1 0 01-1.414 0l-6-6a1 1 0 010-1.414l6-6a1 1 0 011.414 1.414L5.414 9H17a1 1 0 110 2H5.414l4.293 4.293a1 1 0 010 1.414z"
              clipRule="evenodd"
            />
          </svg>
          {t('onboarding.backToOnboarding')}
        </button>

        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold text-gray-900">{tour.name}</h1>
            {tour.description && <p className="mt-1 text-sm text-gray-500">{tour.description}</p>}
          </div>

          {/* Status badge */}
          {(isCompleted || isSkipped) && (
            <span
              className={`px-3 py-1 text-sm font-medium rounded-full ${
                isCompleted ? 'bg-green-100 text-green-800' : 'bg-yellow-100 text-yellow-800'
              }`}
            >
              {t(`onboarding.status.${displayStatus}`)}
            </span>
          )}
        </div>
      </div>

      {/* Progress bar */}
      <div className="mb-8 bg-white rounded-lg shadow p-4">
        <div className="flex items-center justify-between text-sm text-gray-600 mb-2">
          <span>
            {t('onboarding.stepsCompleted', {
              completed: completedStepIds.length,
              total: tour.steps.length,
            })}
          </span>
          <span className="font-medium">{progressPercent}%</span>
        </div>
        <div className="w-full bg-gray-200 rounded-full h-2.5">
          <div
            className={`h-2.5 rounded-full transition-all duration-300 ${
              isCompleted ? 'bg-green-500' : isSkipped ? 'bg-yellow-500' : 'bg-blue-500'
            }`}
            style={{ width: `${progressPercent}%` }}
          />
        </div>
      </div>

      {/* Completed state */}
      {isCompleted && (
        <div className="mb-8 bg-green-50 border border-green-200 rounded-lg p-6 text-center">
          <svg
            className="mx-auto h-12 w-12 text-green-500 mb-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <title>Completed</title>
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <h2 className="text-lg font-semibold text-green-900 mb-2">
            {t('onboarding.tourCompleted')}
          </h2>
          <p className="text-sm text-green-700 mb-4">{t('onboarding.tourCompletedDescription')}</p>
          <button
            type="button"
            onClick={onResetTour}
            className="text-sm text-green-600 hover:text-green-800 font-medium"
          >
            {t('onboarding.restartTour')}
          </button>
        </div>
      )}

      {/* Skipped state */}
      {isSkipped && (
        <div className="mb-8 bg-yellow-50 border border-yellow-200 rounded-lg p-6 text-center">
          <svg
            className="mx-auto h-12 w-12 text-yellow-500 mb-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <title>Skipped</title>
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
            />
          </svg>
          <h2 className="text-lg font-semibold text-yellow-900 mb-2">
            {t('onboarding.tourSkipped')}
          </h2>
          <p className="text-sm text-yellow-700 mb-4">{t('onboarding.tourSkippedDescription')}</p>
          <button
            type="button"
            onClick={onResetTour}
            className="text-sm text-yellow-600 hover:text-yellow-800 font-medium"
          >
            {t('onboarding.restartTour')}
          </button>
        </div>
      )}

      {/* Layout with sidebar and main content */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Steps list sidebar */}
        <div className="lg:col-span-1">
          <div className="bg-white rounded-lg shadow p-4">
            <h3 className="text-sm font-semibold text-gray-700 mb-3">{t('onboarding.steps')}</h3>
            <div className="space-y-2">
              {tour.steps.map((step, index) => (
                <TourStepListItem
                  key={step.id}
                  step={step}
                  stepNumber={index + 1}
                  isCompleted={completedStepIds.includes(step.id)}
                  isCurrent={index === currentStepIndex && !isCompleted && !isSkipped}
                />
              ))}
            </div>
          </div>
        </div>

        {/* Main content area */}
        <div className="lg:col-span-2 space-y-4">
          {!isCompleted &&
            !isSkipped &&
            tour.steps.map((step, index) => {
              const isStepCompleted = completedStepIds.includes(step.id);
              const isCurrent = index === currentStepIndex;
              const isFirst = index === 0;
              const isLast = index === tour.steps.length - 1;

              // Only show current step and completed steps
              if (!isStepCompleted && !isCurrent) return null;

              return (
                <TourStep
                  key={step.id}
                  step={step}
                  stepNumber={index + 1}
                  totalSteps={tour.steps.length}
                  isCompleted={isStepCompleted}
                  isCurrent={isCurrent}
                  isFirst={isFirst}
                  isLast={isLast}
                  onComplete={() => (isLast ? handleFinish() : handleStepComplete(index))}
                  onBack={isFirst ? undefined : handlePreviousStep}
                />
              );
            })}

          {/* Show all steps in completed/skipped state */}
          {(isCompleted || isSkipped) &&
            tour.steps.map((step, index) => (
              <TourStep
                key={step.id}
                step={step}
                stepNumber={index + 1}
                totalSteps={tour.steps.length}
                isCompleted={completedStepIds.includes(step.id)}
                isCurrent={false}
              />
            ))}
        </div>
      </div>

      {/* Action buttons */}
      {!isCompleted && !isSkipped && (
        <div className="mt-8 flex items-center justify-between border-t pt-6">
          <button
            type="button"
            onClick={onSkipTour}
            className="text-sm text-gray-500 hover:text-gray-700"
          >
            {t('onboarding.skipTour')}
          </button>

          {completedStepIds.length === tour.steps.length && (
            <button
              type="button"
              onClick={onCompleteTour}
              className="px-4 py-2 text-sm font-medium text-white bg-green-600 rounded-md hover:bg-green-700"
            >
              {t('onboarding.finishTour')}
            </button>
          )}
        </div>
      )}
    </div>
  );
}
