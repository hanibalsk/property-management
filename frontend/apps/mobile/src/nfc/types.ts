/**
 * NFC building access type definitions.
 *
 * Epic 49 - Story 49.4: NFC Building Access
 */

/**
 * NFC credential for building access.
 */
export interface NFCCredential {
  /** Unique credential ID */
  id: string;
  /** Building ID this credential grants access to */
  buildingId: string;
  /** Building name for display */
  buildingName: string;
  /** User ID */
  userId: string;
  /** Access level */
  accessLevel: AccessLevel;
  /** Access points this credential works for */
  accessPoints: AccessPoint[];
  /** Credential valid from */
  validFrom: string;
  /** Credential valid until */
  validUntil: string;
  /** Credential status */
  status: CredentialStatus;
  /** Last used timestamp */
  lastUsed?: string;
  /** Total usage count */
  usageCount: number;
  /** Encrypted credential data for NFC transmission */
  encryptedData: string;
}

/**
 * Access level for building entry.
 */
export type AccessLevel =
  | 'owner'
  | 'resident'
  | 'tenant'
  | 'guest'
  | 'service'
  | 'delivery'
  | 'emergency';

/**
 * Credential status.
 */
export type CredentialStatus =
  | 'active'
  | 'suspended'
  | 'expired'
  | 'revoked'
  | 'pending_activation';

/**
 * Access point definition.
 */
export interface AccessPoint {
  /** Access point ID */
  id: string;
  /** Access point name */
  name: string;
  /** Access point type */
  type: AccessPointType;
  /** Building section/zone */
  zone?: string;
  /** Floor number */
  floor?: number;
  /** Time restrictions */
  timeRestrictions?: TimeRestriction[];
}

/**
 * Access point types.
 */
export type AccessPointType =
  | 'main_entrance'
  | 'parking_gate'
  | 'elevator'
  | 'floor_door'
  | 'amenity'
  | 'garage'
  | 'mailroom'
  | 'emergency_exit';

/**
 * Time-based access restriction.
 */
export interface TimeRestriction {
  /** Days of week (0=Sunday, 6=Saturday) */
  days: number[];
  /** Start time (HH:MM) */
  startTime: string;
  /** End time (HH:MM) */
  endTime: string;
}

/**
 * NFC access attempt result.
 */
export interface AccessAttemptResult {
  /** Whether access was granted */
  granted: boolean;
  /** Timestamp of attempt */
  timestamp: string;
  /** Access point used */
  accessPointId: string;
  /** Access point name */
  accessPointName: string;
  /** Denial reason if not granted */
  denialReason?: AccessDenialReason;
  /** Message to display */
  message: string;
}

/**
 * Reasons for access denial.
 */
export type AccessDenialReason =
  | 'credential_expired'
  | 'credential_suspended'
  | 'credential_revoked'
  | 'access_point_not_allowed'
  | 'time_restriction'
  | 'building_lockdown'
  | 'invalid_credential'
  | 'network_error'
  | 'hardware_error';

/**
 * NFC hardware state.
 */
export type NFCHardwareState =
  | 'unsupported'
  | 'disabled'
  | 'ready'
  | 'reading'
  | 'transmitting'
  | 'error';

/**
 * NFC tap event.
 */
export interface NFCTapEvent {
  /** Event type */
  type: 'read' | 'write';
  /** Credential ID used */
  credentialId: string;
  /** Access point detected */
  accessPointId?: string;
  /** Timestamp */
  timestamp: string;
}

/**
 * Access log entry.
 */
export interface AccessLogEntry {
  /** Log entry ID */
  id: string;
  /** Credential used */
  credentialId: string;
  /** Building ID */
  buildingId: string;
  /** Access point */
  accessPointId: string;
  /** Access point name */
  accessPointName: string;
  /** Result */
  result: 'granted' | 'denied';
  /** Denial reason if applicable */
  denialReason?: AccessDenialReason;
  /** Timestamp */
  timestamp: string;
}

/**
 * Apple Wallet pass data (iOS).
 */
export interface WalletPass {
  /** Pass type identifier */
  passTypeIdentifier: string;
  /** Serial number */
  serialNumber: string;
  /** Description */
  description: string;
  /** Organization name */
  organizationName: string;
  /** Logo image URL */
  logoUrl?: string;
  /** Pass style (generic, event, etc.) */
  style: 'generic';
  /** Barcode/NFC data */
  nfcData: string;
}

/**
 * Guest access invitation.
 */
export interface GuestAccessInvitation {
  /** Invitation ID */
  id: string;
  /** Inviter user ID */
  inviterId: string;
  /** Guest name */
  guestName: string;
  /** Guest email */
  guestEmail: string;
  /** Building ID */
  buildingId: string;
  /** Access points granted */
  accessPoints: string[];
  /** Valid from */
  validFrom: string;
  /** Valid until */
  validUntil: string;
  /** Maximum entry count */
  maxEntries?: number;
  /** Current entry count */
  entryCount: number;
  /** Invitation status */
  status: 'pending' | 'accepted' | 'expired' | 'cancelled';
  /** Deep link for guest to claim */
  claimLink: string;
}
