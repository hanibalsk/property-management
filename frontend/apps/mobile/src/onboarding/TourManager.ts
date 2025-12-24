/**
 * Tour manager for interactive onboarding.
 *
 * Epic 50 - Story 50.1: Interactive Onboarding Tour
 */
import AsyncStorage from '@react-native-async-storage/async-storage';

import type { OnboardingState, TourConfig, TourProgress, TourStep } from './types';

const ONBOARDING_STATE_KEY = '@ppt/onboarding_state';

/**
 * Manages onboarding tours and progress.
 */
export class TourManager {
  private state: OnboardingState = {
    hasCompletedOnboarding: false,
    tours: {},
    currentTour: null,
    helpDismissed: {},
  };

  private tours: Map<string, TourConfig> = new Map();
  private listeners: Set<(state: OnboardingState) => void> = new Set();

  constructor() {
    this.registerDefaultTours();
  }

  /**
   * Initialize and load saved state.
   */
  async initialize(): Promise<void> {
    await this.loadState();
  }

  /**
   * Register a tour configuration.
   */
  registerTour(tour: TourConfig): void {
    this.tours.set(tour.id, tour);
  }

  /**
   * Get a tour by ID.
   */
  getTour(tourId: string): TourConfig | undefined {
    return this.tours.get(tourId);
  }

  /**
   * Get all registered tours.
   */
  getAllTours(): TourConfig[] {
    return Array.from(this.tours.values());
  }

  /**
   * Start a tour.
   */
  async startTour(tourId: string): Promise<TourProgress | null> {
    const tour = this.tours.get(tourId);
    if (!tour || tour.steps.length === 0) {
      return null;
    }

    const progress: TourProgress = {
      tourId,
      currentStepIndex: 0,
      startedAt: new Date().toISOString(),
      stepsCompleted: [],
      skipped: false,
    };

    this.state.tours[tourId] = progress;
    this.state.currentTour = tourId;
    await this.saveState();
    this.notifyListeners();

    return progress;
  }

  /**
   * Get current tour step.
   */
  getCurrentStep(): TourStep | null {
    if (!this.state.currentTour) {
      return null;
    }

    const tour = this.tours.get(this.state.currentTour);
    const progress = this.state.tours[this.state.currentTour];

    if (!tour || !progress) {
      return null;
    }

    return tour.steps[progress.currentStepIndex] ?? null;
  }

  /**
   * Advance to next step.
   */
  async nextStep(): Promise<TourStep | null> {
    if (!this.state.currentTour) {
      return null;
    }

    const tour = this.tours.get(this.state.currentTour);
    const progress = this.state.tours[this.state.currentTour];

    if (!tour || !progress) {
      return null;
    }

    // Mark current step as completed
    const currentStep = tour.steps[progress.currentStepIndex];
    if (currentStep && !progress.stepsCompleted.includes(currentStep.id)) {
      progress.stepsCompleted.push(currentStep.id);
    }

    // Move to next step
    progress.currentStepIndex += 1;

    // Check if tour is complete
    if (progress.currentStepIndex >= tour.steps.length) {
      await this.completeTour();
      return null;
    }

    await this.saveState();
    this.notifyListeners();

    return tour.steps[progress.currentStepIndex];
  }

  /**
   * Go to previous step.
   */
  async previousStep(): Promise<TourStep | null> {
    if (!this.state.currentTour) {
      return null;
    }

    const tour = this.tours.get(this.state.currentTour);
    const progress = this.state.tours[this.state.currentTour];

    if (!tour || !progress || progress.currentStepIndex <= 0) {
      return null;
    }

    progress.currentStepIndex -= 1;
    await this.saveState();
    this.notifyListeners();

    return tour.steps[progress.currentStepIndex];
  }

  /**
   * Skip current tour.
   */
  async skipTour(): Promise<void> {
    if (!this.state.currentTour) {
      return;
    }

    const progress = this.state.tours[this.state.currentTour];
    if (progress) {
      progress.skipped = true;
      progress.completedAt = new Date().toISOString();
    }

    this.state.currentTour = null;
    await this.saveState();
    this.notifyListeners();
  }

  /**
   * Complete current tour.
   */
  async completeTour(): Promise<void> {
    if (!this.state.currentTour) {
      return;
    }

    const progress = this.state.tours[this.state.currentTour];
    if (progress) {
      progress.completedAt = new Date().toISOString();
    }

    // Check if all required tours are complete
    const mainTour = this.state.tours['main-onboarding'];
    if (mainTour?.completedAt && !mainTour.skipped) {
      this.state.hasCompletedOnboarding = true;
    }

    this.state.currentTour = null;
    await this.saveState();
    this.notifyListeners();
  }

  /**
   * Check if user should see onboarding.
   */
  shouldShowOnboarding(): boolean {
    return !this.state.hasCompletedOnboarding && !this.state.currentTour;
  }

  /**
   * Check if a tour is complete.
   */
  isTourComplete(tourId: string): boolean {
    const progress = this.state.tours[tourId];
    return !!progress?.completedAt;
  }

  /**
   * Get tour progress.
   */
  getTourProgress(tourId: string): TourProgress | undefined {
    return this.state.tours[tourId];
  }

  /**
   * Get current onboarding state.
   */
  getState(): OnboardingState {
    return { ...this.state };
  }

  /**
   * Dismiss help for a screen.
   */
  async dismissHelp(screenId: string): Promise<void> {
    this.state.helpDismissed[screenId] = true;
    await this.saveState();
    this.notifyListeners();
  }

  /**
   * Check if help was dismissed for a screen.
   */
  isHelpDismissed(screenId: string): boolean {
    return this.state.helpDismissed[screenId] ?? false;
  }

  /**
   * Reset onboarding state.
   */
  async reset(): Promise<void> {
    this.state = {
      hasCompletedOnboarding: false,
      tours: {},
      currentTour: null,
      helpDismissed: {},
    };
    await this.saveState();
    this.notifyListeners();
  }

  /**
   * Subscribe to state changes.
   */
  subscribe(listener: (state: OnboardingState) => void): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  // Private methods

  private async loadState(): Promise<void> {
    try {
      const stored = await AsyncStorage.getItem(ONBOARDING_STATE_KEY);
      if (stored) {
        const parsed = JSON.parse(stored);
        // Validate basic structure before assignment
        if (parsed && typeof parsed === 'object' && 'hasCompletedOnboarding' in parsed) {
          this.state = parsed;
        }
      }
    } catch (error) {
      // Handle corrupted data gracefully - reset to default state
      if (__DEV__) {
        console.warn('Failed to load onboarding state, starting fresh:', error);
      }
      // State already initialized to default values in constructor
    }
  }

  private async saveState(): Promise<void> {
    await AsyncStorage.setItem(ONBOARDING_STATE_KEY, JSON.stringify(this.state));
  }

  private notifyListeners(): void {
    for (const listener of this.listeners) {
      listener(this.getState());
    }
  }

  private registerDefaultTours(): void {
    // Main onboarding tour
    this.registerTour({
      id: 'main-onboarding',
      name: 'Welcome Tour',
      estimatedMinutes: 3,
      steps: [
        {
          id: 'welcome',
          title: 'Welcome to PPT!',
          content:
            'This is your property management app. Let us show you around to help you get started.',
          targetScreen: 'Dashboard',
          placement: 'center',
        },
        {
          id: 'dashboard',
          title: 'Your Dashboard',
          content:
            'Here you can see an overview of your buildings, recent activity, and important notifications.',
          targetScreen: 'Dashboard',
          targetElement: 'dashboard-overview',
          placement: 'bottom',
        },
        {
          id: 'buildings',
          title: 'Buildings',
          content:
            'View and manage all your buildings. Tap on a building to see details, residents, and more.',
          targetScreen: 'Dashboard',
          targetElement: 'buildings-section',
          placement: 'bottom',
          action: 'tap',
          actionHint: 'Tap on a building card',
        },
        {
          id: 'faults',
          title: 'Report Faults',
          content:
            'Need to report an issue? Use the fault reporting feature to notify management about problems.',
          targetScreen: 'Dashboard',
          targetElement: 'quick-actions',
          placement: 'top',
        },
        {
          id: 'notifications',
          title: 'Stay Updated',
          content:
            'Check notifications to stay up to date with announcements, votes, and building news.',
          targetScreen: 'Dashboard',
          targetElement: 'notifications-bell',
          placement: 'bottom',
        },
        {
          id: 'complete',
          title: "You're All Set!",
          content:
            "That's the basics! Explore the app to discover more features. You can always access help from the menu.",
          targetScreen: 'Dashboard',
          placement: 'center',
        },
      ],
    });

    // Fault reporting tour
    this.registerTour({
      id: 'fault-reporting',
      name: 'How to Report a Fault',
      estimatedMinutes: 2,
      steps: [
        {
          id: 'start',
          title: 'Reporting Issues',
          content: "Let's learn how to report problems in your building.",
          targetScreen: 'ReportFault',
          placement: 'center',
        },
        {
          id: 'category',
          title: 'Select Category',
          content: 'Choose the type of issue (e.g., plumbing, electrical, elevator).',
          targetScreen: 'ReportFault',
          targetElement: 'category-picker',
          placement: 'bottom',
          action: 'tap',
        },
        {
          id: 'description',
          title: 'Describe the Problem',
          content:
            'Provide details about the issue. The more specific, the faster it can be fixed.',
          targetScreen: 'ReportFault',
          targetElement: 'description-input',
          placement: 'top',
          action: 'input',
        },
        {
          id: 'photo',
          title: 'Add Photos',
          content: 'Optionally add photos to help technicians understand the issue.',
          targetScreen: 'ReportFault',
          targetElement: 'photo-button',
          placement: 'top',
          action: 'tap',
        },
        {
          id: 'submit',
          title: 'Submit',
          content: "Review and submit your report. You'll receive updates as it's processed.",
          targetScreen: 'ReportFault',
          targetElement: 'submit-button',
          placement: 'top',
        },
      ],
    });

    // Voting tour
    this.registerTour({
      id: 'voting',
      name: 'How to Vote',
      estimatedMinutes: 1,
      steps: [
        {
          id: 'start',
          title: 'Building Votes',
          content: 'Participate in building decisions through our voting system.',
          targetScreen: 'Voting',
          placement: 'center',
        },
        {
          id: 'active-votes',
          title: 'Active Votes',
          content: 'See all votes currently open for your buildings.',
          targetScreen: 'Voting',
          targetElement: 'vote-list',
          placement: 'bottom',
        },
        {
          id: 'cast-vote',
          title: 'Cast Your Vote',
          content: 'Read the proposal, then select your choice. Your vote is anonymous.',
          targetScreen: 'Voting',
          targetElement: 'vote-options',
          placement: 'top',
          action: 'tap',
        },
        {
          id: 'results',
          title: 'View Results',
          content: 'After voting closes, see the results and outcome.',
          targetScreen: 'Voting',
          targetElement: 'results-section',
          placement: 'bottom',
        },
      ],
    });
  }
}

// Singleton instance
export const tourManager = new TourManager();
