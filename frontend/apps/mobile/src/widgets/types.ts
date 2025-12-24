/**
 * Widget type definitions.
 *
 * Epic 49 - Story 49.1: Home Screen Widgets
 */

/**
 * Available widget sizes for home screen.
 */
export type WidgetSize = 'small' | 'medium' | 'large';

/**
 * Widget configuration that can be customized by the user.
 */
export interface WidgetConfig {
  /** Unique widget instance ID */
  id: string;
  /** Widget type identifier */
  type: WidgetType;
  /** Widget display size */
  size: WidgetSize;
  /** User-selected building ID (if applicable) */
  buildingId?: string;
  /** Refresh interval in minutes */
  refreshInterval: number;
  /** Last data update timestamp */
  lastUpdated?: string;
}

/**
 * Widget types available for home screen.
 */
export type WidgetType =
  | 'notifications_count'
  | 'latest_announcement'
  | 'pending_votes'
  | 'active_faults'
  | 'upcoming_meetings'
  | 'quick_actions';

/**
 * Notification count widget data.
 */
export interface NotificationsWidgetData {
  type: 'notifications_count';
  unreadCount: number;
  categories: {
    announcements: number;
    faults: number;
    votes: number;
    documents: number;
    messages: number;
  };
}

/**
 * Latest announcement widget data.
 */
export interface AnnouncementWidgetData {
  type: 'latest_announcement';
  id: string;
  title: string;
  preview: string;
  createdAt: string;
  isUrgent: boolean;
  buildingName: string;
}

/**
 * Pending votes widget data.
 */
export interface PendingVotesWidgetData {
  type: 'pending_votes';
  pendingCount: number;
  votes: Array<{
    id: string;
    title: string;
    endDate: string;
    buildingName: string;
  }>;
}

/**
 * Active faults widget data.
 */
export interface ActiveFaultsWidgetData {
  type: 'active_faults';
  activeCount: number;
  recentFaults: Array<{
    id: string;
    title: string;
    status: 'open' | 'in_progress' | 'awaiting_parts';
    reportedAt: string;
  }>;
}

/**
 * Upcoming meetings widget data.
 */
export interface UpcomingMeetingsWidgetData {
  type: 'upcoming_meetings';
  upcomingCount: number;
  meetings: Array<{
    id: string;
    title: string;
    date: string;
    location: string;
  }>;
}

/**
 * Quick actions widget data.
 */
export interface QuickActionsWidgetData {
  type: 'quick_actions';
  actions: Array<{
    id: string;
    label: string;
    icon: string;
    deepLink: string;
  }>;
}

/**
 * Union type for all widget data types.
 */
export type WidgetData =
  | NotificationsWidgetData
  | AnnouncementWidgetData
  | PendingVotesWidgetData
  | ActiveFaultsWidgetData
  | UpcomingMeetingsWidgetData
  | QuickActionsWidgetData;

/**
 * Widget update request.
 */
export interface WidgetUpdateRequest {
  widgetId: string;
  type: WidgetType;
  buildingId?: string;
}

/**
 * Widget deep link targets.
 */
export type WidgetDeepLink =
  | { screen: 'Dashboard'; query?: Record<string, string> }
  | { screen: 'Faults'; faultId?: string; query?: Record<string, string> }
  | { screen: 'Announcements'; announcementId?: string; query?: Record<string, string> }
  | { screen: 'Voting'; voteId?: string; query?: Record<string, string> }
  | { screen: 'Documents'; query?: Record<string, string> }
  | { screen: 'ReportFault'; query?: Record<string, string> };
