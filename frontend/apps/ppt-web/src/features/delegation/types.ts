/**
 * Delegation feature types.
 * Epic 3: Ownership Management (Story 3.4) - UC-28 Delegation History
 */

import type { DelegationScope, DelegationStatus } from './components/DelegationCard';

/**
 * Types of activities/events that can occur on a delegation.
 */
export type DelegationActivityType =
  | 'created'
  | 'accepted'
  | 'declined'
  | 'revoked'
  | 'expired'
  | 'modified';

/**
 * Represents a single activity/event in a delegation's history.
 */
export interface DelegationActivity {
  id: string;
  delegationId: string;
  activityType: DelegationActivityType;
  performedByUserId: string;
  performedByName: string;
  performedAt: string;
  notes?: string;
  /** Previous status before the activity (for status changes) */
  previousStatus?: DelegationStatus;
  /** New status after the activity */
  newStatus?: DelegationStatus;
  /** Additional metadata about the activity */
  metadata?: Record<string, unknown>;
}

/**
 * Filter options for delegation history.
 */
export interface DelegationHistoryFilter {
  /** Start of date range */
  startDate?: string;
  /** End of date range */
  endDate?: string;
  /** Filter by specific activity types */
  activityTypes?: DelegationActivityType[];
  /** Filter by delegation ID */
  delegationId?: string;
  /** Filter by user who performed the action */
  performedByUserId?: string;
}

/**
 * Delegation with full activity history.
 */
export interface DelegationWithHistory {
  id: string;
  ownerUserId: string;
  delegateUserId: string;
  unitId?: string;
  scopes: DelegationScope[];
  status: DelegationStatus;
  startDate: string;
  endDate?: string;
  acceptedAt?: string;
  revokedAt?: string;
  revokedReason?: string;
  createdAt: string;
  // Display fields
  ownerName: string;
  ownerEmail: string;
  delegateName: string;
  delegateEmail: string;
  unitDesignation?: string;
  // Activity history
  activities: DelegationActivity[];
}

/**
 * Summary statistics for delegation history.
 */
export interface DelegationHistorySummary {
  totalDelegations: number;
  activeDelegations: number;
  revokedDelegations: number;
  expiredDelegations: number;
  declinedDelegations: number;
  totalActivities: number;
  recentActivities: DelegationActivity[];
}

// Re-export types from DelegationCard for convenience
export type { DelegationScope, DelegationStatus } from './components/DelegationCard';
