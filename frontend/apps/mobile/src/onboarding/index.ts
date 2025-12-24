/**
 * Onboarding and help system module.
 *
 * Epic 50 - Stories 50.1-50.4: Onboarding & Help
 */

// Types
export type {
  AppContext,
  DeviceInfo,
  FAQCategory,
  FAQItem,
  FeedbackSubmission,
  FeedbackType,
  HelpCenterState,
  HelpContent,
  OnboardingState,
  TourConfig,
  TourProgress,
  TourStep,
  Tutorial,
  TutorialStep,
} from './types';

// Tour Manager
export { TourManager, tourManager } from './TourManager';

// Help Center
export { HelpCenter, helpCenter } from './HelpCenter';

// Feedback Manager
export { FeedbackManager, feedbackManager } from './FeedbackManager';
