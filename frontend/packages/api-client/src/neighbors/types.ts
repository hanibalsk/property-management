/**
 * Neighbor Types
 *
 * TypeScript types for neighbor information (Epic 6, Story 6.6).
 */

// ============================================================================
// Core Types
// ============================================================================

/** Profile visibility options */
export type ProfileVisibility = 'visible' | 'hidden' | 'contacts_only';

/** Privacy-aware view of a neighbor */
export interface NeighborView {
  userId: string;
  /** Display name - full name if visible, "Resident of Unit X" if hidden */
  displayName: string;
  /** Unit identifier (e.g., "Apt 4B") */
  unitLabel: string;
  /** Whether the profile is visible (vs hidden/anonymous) */
  isVisible: boolean;
  /** Contact email (only if show_contact_info is true) */
  email: string | null;
  /** Contact phone (only if show_contact_info is true) */
  phone: string | null;
  /** Resident type (owner, tenant, etc.) */
  residentType: string;
}

/** User's privacy settings */
export interface PrivacySettings {
  profileVisibility: ProfileVisibility;
  showContactInfo: boolean;
}

// ============================================================================
// Request Types
// ============================================================================

/** Request to update privacy settings */
export interface UpdatePrivacySettingsRequest {
  profileVisibility?: ProfileVisibility;
  showContactInfo?: boolean;
}

// ============================================================================
// Response Types
// ============================================================================

/** Response for neighbor list */
export interface NeighborsResponse {
  neighbors: NeighborView[];
  count: number;
  total: number;
}

/** Response for privacy settings */
export interface PrivacySettingsResponse {
  settings: PrivacySettings;
}
