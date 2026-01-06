/**
 * TourStep component - displays an individual step with completion state.
 * Epic 10B: User Onboarding (Story 10B.6)
 */

import { useTranslation } from 'react-i18next';
import type { TourStepData } from '../types';

interface TourStepProps {
  step: TourStepData;
  stepNumber: number;
  totalSteps: number;
  isCompleted: boolean;
  isCurrent: boolean;
  onComplete?: () => void;
  onBack?: () => void;
  isFirst?: boolean;
  isLast?: boolean;
}

export function TourStep({
  step,
  stepNumber,
  totalSteps,
  isCompleted,
  isCurrent,
  onComplete,
  onBack,
  isFirst = false,
  isLast = false,
}: TourStepProps) {
  const { t } = useTranslation();

  return (
    <div
      className={`p-4 rounded-lg border-2 transition-all ${
        isCurrent
          ? 'border-blue-500 bg-blue-50'
          : isCompleted
            ? 'border-green-300 bg-green-50'
            : 'border-gray-200 bg-white'
      }`}
    >
      {/* Step header */}
      <div className="flex items-center gap-3 mb-3">
        <div
          className={`flex items-center justify-center w-8 h-8 rounded-full text-sm font-semibold ${
            isCompleted
              ? 'bg-green-500 text-white'
              : isCurrent
                ? 'bg-blue-500 text-white'
                : 'bg-gray-300 text-gray-600'
          }`}
        >
          {isCompleted ? (
            <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <title>Completed</title>
              <path
                fillRule="evenodd"
                d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                clipRule="evenodd"
              />
            </svg>
          ) : (
            stepNumber
          )}
        </div>
        <div className="flex-1">
          <h4 className="font-semibold text-gray-900">{step.title}</h4>
          <p className="text-xs text-gray-500">
            {t('onboarding.stepOf', { current: stepNumber, total: totalSteps })}
          </p>
        </div>
        {isCompleted && (
          <span className="px-2 py-0.5 text-xs font-medium bg-green-100 text-green-800 rounded">
            {t('onboarding.status.completed')}
          </span>
        )}
      </div>

      {/* Step content */}
      <div className="ml-11">
        <p className="text-gray-600 text-sm leading-relaxed">{step.content}</p>

        {/* Target hint */}
        {step.target && isCurrent && (
          <div className="mt-3 p-2 bg-yellow-50 border border-yellow-200 rounded text-xs text-yellow-700">
            <span className="font-medium">{t('onboarding.targetHint')}:</span> {step.target}
          </div>
        )}

        {/* Actions */}
        {isCurrent && (
          <div className="mt-4 flex items-center gap-3">
            {!isFirst && onBack && (
              <button
                type="button"
                onClick={onBack}
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
              >
                {t('common.back')}
              </button>
            )}
            {onComplete && (
              <button
                type="button"
                onClick={onComplete}
                className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
              >
                {isLast ? t('onboarding.finishTour') : t('onboarding.completeStep')}
              </button>
            )}
          </div>
        )}
      </div>
    </div>
  );
}

/**
 * TourStepListItem - Compact list item version of a tour step.
 */
interface TourStepListItemProps {
  step: TourStepData;
  stepNumber: number;
  isCompleted: boolean;
  isCurrent: boolean;
  onClick?: () => void;
}

export function TourStepListItem({
  step,
  stepNumber,
  isCompleted,
  isCurrent,
  onClick,
}: TourStepListItemProps) {
  return (
    <button
      type="button"
      onClick={onClick}
      className={`w-full flex items-center gap-3 p-3 rounded-lg text-left transition-colors ${
        isCurrent
          ? 'bg-blue-50 border border-blue-200'
          : isCompleted
            ? 'bg-green-50 hover:bg-green-100'
            : 'bg-gray-50 hover:bg-gray-100'
      }`}
    >
      <div
        className={`flex items-center justify-center w-6 h-6 rounded-full text-xs font-semibold ${
          isCompleted
            ? 'bg-green-500 text-white'
            : isCurrent
              ? 'bg-blue-500 text-white'
              : 'bg-gray-300 text-gray-600'
        }`}
      >
        {isCompleted ? (
          <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
            <title>Completed</title>
            <path
              fillRule="evenodd"
              d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
              clipRule="evenodd"
            />
          </svg>
        ) : (
          stepNumber
        )}
      </div>
      <div className="flex-1 min-w-0">
        <p
          className={`text-sm font-medium truncate ${
            isCurrent ? 'text-blue-900' : isCompleted ? 'text-green-900' : 'text-gray-700'
          }`}
        >
          {step.title}
        </p>
      </div>
      {isCurrent && (
        <svg className="w-4 h-4 text-blue-500" fill="currentColor" viewBox="0 0 20 20">
          <title>Current</title>
          <path
            fillRule="evenodd"
            d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
            clipRule="evenodd"
          />
        </svg>
      )}
    </button>
  );
}
