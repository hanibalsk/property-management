/**
 * Types for onboarding and help system.
 *
 * Epic 50 - Story 50.1-50.4: Onboarding & Help
 */

/**
 * Onboarding tour step.
 */
export interface TourStep {
  id: string;
  title: string;
  content: string;
  targetScreen: string;
  targetElement?: string;
  placement?: 'top' | 'bottom' | 'left' | 'right' | 'center';
  highlightArea?: {
    x: number;
    y: number;
    width: number;
    height: number;
  };
  action?: 'tap' | 'swipe' | 'scroll' | 'input';
  actionHint?: string;
}

/**
 * Tour configuration.
 */
export interface TourConfig {
  id: string;
  name: string;
  steps: TourStep[];
  completionReward?: string;
  estimatedMinutes: number;
}

/**
 * Tour progress.
 */
export interface TourProgress {
  tourId: string;
  currentStepIndex: number;
  startedAt: string;
  completedAt?: string;
  stepsCompleted: string[];
  skipped: boolean;
}

/**
 * Help content item.
 */
export interface HelpContent {
  id: string;
  screen: string;
  title: string;
  shortDescription: string;
  fullContent: string;
  relatedFAQs?: string[];
  relatedTutorials?: string[];
  lastUpdated: string;
}

/**
 * FAQ item.
 */
export interface FAQItem {
  id: string;
  question: string;
  answer: string;
  category: FAQCategory;
  tags: string[];
  helpful: number;
  notHelpful: number;
  relatedQuestions?: string[];
}

/**
 * FAQ category.
 */
export type FAQCategory =
  | 'getting_started'
  | 'account'
  | 'buildings'
  | 'faults'
  | 'voting'
  | 'documents'
  | 'payments'
  | 'notifications'
  | 'security'
  | 'other';

/**
 * Tutorial item.
 */
export interface Tutorial {
  id: string;
  title: string;
  description: string;
  videoUrl?: string;
  thumbnailUrl?: string;
  duration: number;
  category: FAQCategory;
  steps?: TutorialStep[];
}

/**
 * Tutorial step.
 */
export interface TutorialStep {
  id: string;
  title: string;
  content: string;
  imageUrl?: string;
}

/**
 * Feedback type.
 */
export type FeedbackType = 'bug' | 'feature' | 'improvement' | 'question' | 'other';

/**
 * Feedback submission.
 */
export interface FeedbackSubmission {
  type: FeedbackType;
  title: string;
  description: string;
  email?: string;
  screenshot?: string;
  deviceInfo: DeviceInfo;
  appContext: AppContext;
}

/**
 * Device information.
 */
export interface DeviceInfo {
  platform: 'ios' | 'android';
  osVersion: string;
  deviceModel: string;
  appVersion: string;
  buildNumber: string;
  locale: string;
  timezone: string;
}

/**
 * App context for feedback.
 */
export interface AppContext {
  currentScreen: string;
  userId?: string;
  buildingId?: string;
  timestamp: string;
  sessionDuration: number;
}

/**
 * Help center state.
 */
export interface HelpCenterState {
  searchQuery: string;
  selectedCategory: FAQCategory | null;
  viewMode: 'faq' | 'tutorials' | 'contact';
}

/**
 * Onboarding state.
 */
export interface OnboardingState {
  hasCompletedOnboarding: boolean;
  tours: Record<string, TourProgress>;
  currentTour: string | null;
  helpDismissed: Record<string, boolean>;
}
