/**
 * Critical Notifications Types (Epic 8A, Story 8A.2)
 */

/** Critical notification response */
export interface CriticalNotificationResponse {
  id: string;
  title: string;
  message: string;
  createdBy: string;
  createdAt: string;
  isAcknowledged: boolean;
  acknowledgedAt?: string | null;
}

/** Request to create a critical notification */
export interface CreateCriticalNotificationRequest {
  title: string;
  message: string;
}

/** Response after creating a critical notification */
export interface CreateCriticalNotificationResponse {
  id: string;
  title: string;
  message: string;
  createdAt: string;
}

/** Response after acknowledging a notification */
export interface AcknowledgeCriticalNotificationResponse {
  notificationId: string;
  acknowledgedAt: string;
}

/** Unacknowledged notifications response */
export interface UnacknowledgedNotificationsResponse {
  notifications: CriticalNotificationResponse[];
  count: number;
}

/** Statistics for a critical notification (admin view) */
export interface CriticalNotificationStats {
  notificationId: string;
  totalUsers: number;
  acknowledgedCount: number;
  pendingCount: number;
}
