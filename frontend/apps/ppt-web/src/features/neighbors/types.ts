/**
 * Neighbors Feature Types
 *
 * TypeScript types for the neighbors feature.
 */

/**
 * Visibility level for profile fields.
 */
export type VisibilityLevel = 'public' | 'building' | 'neighbors' | 'private';

/**
 * View of a neighbor in the building directory.
 * Respects privacy settings - only shows fields the user has made visible.
 */
export interface NeighborView {
  id: string;
  firstName?: string;
  lastName?: string;
  displayName?: string;
  unitNumber?: string;
  floor?: number;
  email?: string;
  phone?: string;
  avatarUrl?: string;
  moveInDate?: string;
  bio?: string;
  isOwner?: boolean;
}

/**
 * User privacy settings for profile visibility.
 */
export interface PrivacySettings {
  showName: VisibilityLevel;
  showEmail: VisibilityLevel;
  showPhone: VisibilityLevel;
  showUnit: VisibilityLevel;
  showAvatar: VisibilityLevel;
  showBio: VisibilityLevel;
  showMoveInDate: VisibilityLevel;
  listedInDirectory: boolean;
}

/**
 * Default privacy settings for new users.
 */
export const DEFAULT_PRIVACY_SETTINGS: PrivacySettings = {
  showName: 'building',
  showEmail: 'private',
  showPhone: 'private',
  showUnit: 'building',
  showAvatar: 'building',
  showBio: 'building',
  showMoveInDate: 'private',
  listedInDirectory: true,
};

/**
 * Options for visibility level select fields.
 */
export const VISIBILITY_OPTIONS: { value: VisibilityLevel; labelKey: string }[] = [
  { value: 'public', labelKey: 'neighbors.privacy.visibilityPublic' },
  { value: 'building', labelKey: 'neighbors.privacy.visibilityBuilding' },
  { value: 'neighbors', labelKey: 'neighbors.privacy.visibilityNeighbors' },
  { value: 'private', labelKey: 'neighbors.privacy.visibilityPrivate' },
];

/**
 * Status of an invitation.
 */
export type InvitationStatus = 'pending' | 'sent' | 'accepted' | 'expired' | 'cancelled';

/**
 * Role of the invited neighbor.
 */
export type NeighborRole = 'owner' | 'tenant' | 'family_member';

/**
 * Invitation to join a building as a neighbor.
 */
export interface Invitation {
  id: string;
  email: string;
  firstName?: string;
  lastName?: string;
  unitId: string;
  unitNumber?: string;
  role: NeighborRole;
  status: InvitationStatus;
  personalMessage?: string;
  sentAt: string;
  expiresAt: string;
  createdAt: string;
  acceptedAt?: string;
  cancelledAt?: string;
  invitedBy: {
    id: string;
    name: string;
  };
}

/**
 * Form data for inviting a new neighbor.
 */
export interface InviteNeighborFormData {
  email: string;
  firstName?: string;
  lastName?: string;
  unitId: string;
  role: NeighborRole;
  personalMessage?: string;
}

/**
 * Unit option for dropdown selection.
 */
export interface UnitOption {
  id: string;
  number: string;
  floor?: number;
}

/**
 * Role options for the invitation form.
 */
export const NEIGHBOR_ROLE_OPTIONS: { value: NeighborRole; labelKey: string }[] = [
  { value: 'owner', labelKey: 'neighbors.invitation.roleOwner' },
  { value: 'tenant', labelKey: 'neighbors.invitation.roleTenant' },
  { value: 'family_member', labelKey: 'neighbors.invitation.roleFamilyMember' },
];
