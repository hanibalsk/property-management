/**
 * Onboarding feature types.
 * Epic 10B: User Onboarding (Story 10B.6)
 */

/**
 * Tour status for filtering.
 */
export type TourStatus = 'all' | 'completed' | 'in_progress' | 'not_started' | 'skipped';

/**
 * Single tour step definition.
 */
export interface TourStepData {
  /** Unique step identifier */
  id: string;
  /** Step title */
  title: string;
  /** Step content/description */
  content: string;
  /** Optional CSS selector for UI target element */
  target?: string;
}

/**
 * Onboarding tour definition.
 */
export interface OnboardingTour {
  /** Database UUID */
  id: string;
  /** Unique tour identifier */
  tourId: string;
  /** Tour name */
  name: string;
  /** Tour description */
  description?: string;
  /** Array of tour steps */
  steps: TourStepData[];
  /** Roles this tour is targeted for */
  targetRoles?: string[];
  /** Whether tour is active */
  isActive: boolean;
  /** Tour creation timestamp */
  createdAt: string;
  /** Tour last update timestamp */
  updatedAt: string;
}

/**
 * User progress on a specific tour.
 */
export interface UserOnboardingProgress {
  /** Progress record UUID */
  id: string;
  /** User UUID */
  userId: string;
  /** Tour identifier */
  tourId: string;
  /** Array of completed step IDs */
  completedSteps: string[];
  /** Current step ID */
  currentStep?: string;
  /** Whether tour is fully completed */
  isCompleted: boolean;
  /** Whether user skipped the tour */
  isSkipped: boolean;
  /** When user started the tour */
  startedAt: string;
  /** When user completed the tour */
  completedAt?: string;
  /** Record creation timestamp */
  createdAt: string;
  /** Record update timestamp */
  updatedAt: string;
}

/**
 * Tour with user progress combined.
 */
export interface TourWithProgress {
  /** Tour definition */
  tour: OnboardingTour;
  /** User progress (null if not started) */
  progress: UserOnboardingProgress | null;
}

/**
 * Overall onboarding status response.
 */
export interface OnboardingStatus {
  /** Whether user has any incomplete tours */
  needsOnboarding: boolean;
  /** List of tours with their progress */
  tours: TourWithProgress[];
}

/**
 * Calculate completion percentage for a tour.
 */
export function calculateTourProgress(
  tour: OnboardingTour,
  progress: UserOnboardingProgress | null
): number {
  if (!progress) return 0;
  if (progress.isCompleted) return 100;
  if (progress.isSkipped) return 0;

  const totalSteps = tour.steps.length;
  if (totalSteps === 0) return 100;

  const completedCount = progress.completedSteps.length;
  return Math.round((completedCount / totalSteps) * 100);
}

/**
 * Get the current step index for a tour.
 */
export function getCurrentStepIndex(
  tour: OnboardingTour,
  progress: UserOnboardingProgress | null
): number {
  if (!progress || !progress.currentStep) return 0;

  const index = tour.steps.findIndex((step) => step.id === progress.currentStep);
  return index >= 0 ? index : 0;
}

/**
 * Determine the display status of a tour.
 */
export function getTourDisplayStatus(
  progress: UserOnboardingProgress | null
): 'not_started' | 'in_progress' | 'completed' | 'skipped' {
  if (!progress) return 'not_started';
  if (progress.isCompleted) return 'completed';
  if (progress.isSkipped) return 'skipped';
  if (progress.completedSteps.length > 0) return 'in_progress';
  return 'not_started';
}

// =============================================================================
// UC-42: Help Center & Support Types
// =============================================================================

/**
 * FAQ category for filtering.
 */
export type FAQCategory = 'all' | 'account' | 'billing' | 'features' | 'technical' | 'general';

/**
 * FAQ item definition.
 */
export interface FAQItem {
  /** Unique FAQ identifier */
  id: string;
  /** FAQ question */
  question: string;
  /** FAQ answer (supports markdown) */
  answer: string;
  /** Category for filtering */
  category: FAQCategory;
  /** Whether this is a popular/featured FAQ */
  isFeatured?: boolean;
  /** Order for display */
  order?: number;
  /** Creation timestamp */
  createdAt: string;
  /** Last update timestamp */
  updatedAt: string;
}

/**
 * Feedback type options.
 */
export type FeedbackType = 'suggestion' | 'complaint' | 'compliment' | 'bug_report' | 'other';

/**
 * Feedback submission data.
 */
export interface FeedbackSubmission {
  /** Unique feedback identifier */
  id: string;
  /** User who submitted the feedback */
  userId: string;
  /** Type of feedback */
  type: FeedbackType;
  /** Subject line */
  subject: string;
  /** Detailed description */
  description: string;
  /** Rating (1-5 stars) */
  rating?: number;
  /** Optional screenshot attachment URLs */
  attachments?: string[];
  /** Submission timestamp */
  createdAt: string;
  /** Status of the feedback */
  status: 'submitted' | 'under_review' | 'resolved' | 'closed';
}

/**
 * Form data for submitting feedback.
 */
export interface FeedbackFormData {
  type: FeedbackType;
  subject: string;
  description: string;
  rating?: number;
  attachments?: File[];
}

/**
 * Support ticket status.
 */
export type SupportTicketStatus =
  | 'open'
  | 'in_progress'
  | 'waiting_response'
  | 'resolved'
  | 'closed';

/**
 * Support ticket priority.
 */
export type SupportTicketPriority = 'low' | 'medium' | 'high' | 'urgent';

/**
 * Support ticket definition.
 */
export interface SupportTicket {
  /** Unique ticket identifier */
  id: string;
  /** Ticket reference number */
  ticketNumber: string;
  /** User who created the ticket */
  userId: string;
  /** Ticket subject */
  subject: string;
  /** Issue description */
  description: string;
  /** Steps to reproduce (for bug reports) */
  stepsToReproduce?: string;
  /** Expected behavior */
  expectedBehavior?: string;
  /** Actual behavior */
  actualBehavior?: string;
  /** System info (browser, OS) */
  systemInfo?: SystemInfo;
  /** Ticket status */
  status: SupportTicketStatus;
  /** Ticket priority */
  priority: SupportTicketPriority;
  /** Assigned support agent */
  assignedTo?: string;
  /** Ticket messages/responses */
  messages: SupportTicketMessage[];
  /** Attachment URLs */
  attachments?: string[];
  /** Creation timestamp */
  createdAt: string;
  /** Last update timestamp */
  updatedAt: string;
  /** Resolution timestamp */
  resolvedAt?: string;
}

/**
 * Support ticket message.
 */
export interface SupportTicketMessage {
  /** Message identifier */
  id: string;
  /** Ticket identifier */
  ticketId: string;
  /** Author user ID */
  authorId: string;
  /** Author name */
  authorName: string;
  /** Whether author is support staff */
  isStaff: boolean;
  /** Message content */
  content: string;
  /** Message timestamp */
  createdAt: string;
}

/**
 * System information for bug reports.
 */
export interface SystemInfo {
  /** Browser name and version */
  browser: string;
  /** Operating system */
  os: string;
  /** Screen resolution */
  screenResolution?: string;
  /** User agent string */
  userAgent?: string;
  /** App version */
  appVersion?: string;
}

/**
 * Bug report form data.
 */
export interface BugReportFormData {
  subject: string;
  description: string;
  stepsToReproduce: string;
  expectedBehavior: string;
  actualBehavior: string;
  priority: SupportTicketPriority;
  attachments?: File[];
}

/**
 * Video tutorial category.
 */
export type VideoTutorialCategory = 'all' | 'getting_started' | 'features' | 'tips' | 'advanced';

/**
 * Video tutorial definition.
 */
export interface VideoTutorial {
  /** Unique video identifier */
  id: string;
  /** Video title */
  title: string;
  /** Video description */
  description: string;
  /** Video URL (YouTube, Vimeo, etc.) */
  videoUrl: string;
  /** Thumbnail image URL */
  thumbnailUrl: string;
  /** Video duration in seconds */
  duration: number;
  /** Category for filtering */
  category: VideoTutorialCategory;
  /** Tags for searching */
  tags?: string[];
  /** Whether this is a featured video */
  isFeatured?: boolean;
  /** View count */
  viewCount?: number;
  /** Creation timestamp */
  createdAt: string;
  /** Last update timestamp */
  updatedAt: string;
}

/**
 * Help topic for quick links.
 */
export interface HelpTopic {
  /** Topic identifier */
  id: string;
  /** Topic title */
  title: string;
  /** Topic description */
  description: string;
  /** Icon name */
  icon: string;
  /** Link to the topic page */
  link: string;
  /** Whether this is a popular topic */
  isPopular?: boolean;
}

/**
 * Announcement for help center.
 */
export interface HelpAnnouncement {
  /** Announcement identifier */
  id: string;
  /** Announcement title */
  title: string;
  /** Announcement content */
  content: string;
  /** Announcement type */
  type: 'info' | 'warning' | 'success' | 'update';
  /** Link for more details */
  link?: string;
  /** Publication date */
  publishedAt: string;
  /** Expiration date */
  expiresAt?: string;
}

/**
 * Format duration in seconds to human readable string.
 */
export function formatVideoDuration(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;
  return `${mins}:${secs.toString().padStart(2, '0')}`;
}

/**
 * Get system info for bug reports.
 */
export function getSystemInfo(): SystemInfo {
  const ua = navigator.userAgent;
  let browser = 'Unknown';
  let os = 'Unknown';

  // Detect browser
  if (ua.includes('Firefox')) {
    browser = 'Firefox';
  } else if (ua.includes('Chrome')) {
    browser = 'Chrome';
  } else if (ua.includes('Safari')) {
    browser = 'Safari';
  } else if (ua.includes('Edge')) {
    browser = 'Edge';
  }

  // Detect OS
  if (ua.includes('Windows')) {
    os = 'Windows';
  } else if (ua.includes('Mac')) {
    os = 'macOS';
  } else if (ua.includes('Linux')) {
    os = 'Linux';
  } else if (ua.includes('Android')) {
    os = 'Android';
  } else if (ua.includes('iOS')) {
    os = 'iOS';
  }

  return {
    browser,
    os,
    screenResolution: `${window.screen.width}x${window.screen.height}`,
    userAgent: ua,
  };
}
