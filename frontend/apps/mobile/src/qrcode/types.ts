/**
 * QR Code types and interfaces.
 *
 * Epic 49 - Story 49.3: QR Code Scanning
 */

/**
 * QR code content types that the app can handle.
 */
export type QRCodeType =
  | 'deep_link'
  | 'building_access'
  | 'document'
  | 'fault_report'
  | 'vote'
  | 'contact'
  | 'external_url'
  | 'unknown';

/**
 * Parsed QR code result.
 */
export interface ParsedQRCode {
  /** Detected content type */
  type: QRCodeType;
  /** Raw content */
  raw: string;
  /** Parsed data */
  data: QRCodeData;
  /** Whether this is a valid PPT QR code */
  isValid: boolean;
  /** Error message if invalid */
  error?: string;
}

/**
 * QR code data payload (varies by type).
 */
export type QRCodeData =
  | DeepLinkQRData
  | BuildingAccessQRData
  | DocumentQRData
  | FaultReportQRData
  | VoteQRData
  | ContactQRData
  | ExternalURLQRData
  | UnknownQRData;

/**
 * Deep link QR data.
 */
export interface DeepLinkQRData {
  type: 'deep_link';
  screen: string;
  params?: Record<string, string>;
}

/**
 * Building access QR data.
 */
export interface BuildingAccessQRData {
  type: 'building_access';
  buildingId: string;
  accessPoint: string;
  validUntil?: string;
  accessLevel: 'resident' | 'guest' | 'service';
}

/**
 * Document QR data.
 */
export interface DocumentQRData {
  type: 'document';
  documentId: string;
  documentType: string;
  buildingId?: string;
}

/**
 * Fault report QR data (prefilled form).
 */
export interface FaultReportQRData {
  type: 'fault_report';
  buildingId: string;
  category?: string;
  location?: string;
  equipmentId?: string;
}

/**
 * Vote QR data.
 */
export interface VoteQRData {
  type: 'vote';
  voteId: string;
  buildingId: string;
}

/**
 * Contact/vCard QR data.
 */
export interface ContactQRData {
  type: 'contact';
  name: string;
  phone?: string;
  email?: string;
  role?: string;
}

/**
 * External URL QR data.
 */
export interface ExternalURLQRData {
  type: 'external_url';
  url: string;
}

/**
 * Unknown QR data.
 */
export interface UnknownQRData {
  type: 'unknown';
  content: string;
}

/**
 * QR scanner state.
 */
export type QRScannerState =
  | 'inactive'
  | 'initializing'
  | 'scanning'
  | 'processing'
  | 'success'
  | 'error';

/**
 * QR code generation options.
 */
export interface QRCodeGenerationOptions {
  /** Size in pixels */
  size: number;
  /** Error correction level */
  errorCorrectionLevel: 'L' | 'M' | 'Q' | 'H';
  /** Foreground color */
  color: string;
  /** Background color */
  backgroundColor: string;
  /** Include logo */
  includeLogo: boolean;
  /** Margin (quiet zone) */
  margin: number;
}

/**
 * Deep link target for QR generation.
 */
export interface QRDeepLinkTarget {
  /** Target screen */
  screen: string;
  /** Screen parameters */
  params?: Record<string, string>;
  /** Expiry time for generated QR */
  expiresIn?: number;
  /** Description for share */
  description?: string;
}

/**
 * Scan history entry.
 */
export interface ScanHistoryEntry {
  /** Unique ID */
  id: string;
  /** Parsed QR data */
  qrCode: ParsedQRCode;
  /** Scan timestamp */
  scannedAt: string;
  /** Action taken */
  action: string;
}
