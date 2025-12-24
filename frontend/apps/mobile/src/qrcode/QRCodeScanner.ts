/**
 * QR Code scanning functionality.
 *
 * Epic 49 - Story 49.3: QR Code Scanning
 */
import AsyncStorage from '@react-native-async-storage/async-storage';
import { Vibration } from 'react-native';

import type {
  BuildingAccessQRData,
  ContactQRData,
  DeepLinkQRData,
  DocumentQRData,
  ExternalURLQRData,
  FaultReportQRData,
  ParsedQRCode,
  QRCodeData,
  QRCodeType,
  ScanHistoryEntry,
  VoteQRData,
} from './types';

const SCAN_HISTORY_KEY = '@ppt/qr_scan_history';
const MAX_HISTORY_ENTRIES = 50;

/**
 * Parse a QR code content string.
 */
export function parseQRCode(content: string): ParsedQRCode {
  const trimmed = content.trim();

  // Try to parse as PPT deep link
  if (trimmed.startsWith('ppt://')) {
    return parseDeepLink(trimmed);
  }

  // Try to parse as PPT JSON payload
  if (trimmed.startsWith('{') && trimmed.includes('"ppt":')) {
    return parsePPTPayload(trimmed);
  }

  // Try to parse as vCard
  if (trimmed.startsWith('BEGIN:VCARD')) {
    return parseVCard(trimmed);
  }

  // Check if it's a URL
  if (trimmed.startsWith('http://') || trimmed.startsWith('https://')) {
    // Validate URL format to prevent malformed URLs
    try {
      new URL(trimmed);
      return {
        type: 'external_url',
        raw: trimmed,
        data: { type: 'external_url', url: trimmed },
        isValid: true,
      };
    } catch {
      return {
        type: 'unknown',
        raw: trimmed,
        data: { type: 'unknown', content: trimmed },
        isValid: false,
        error: 'Invalid URL format',
      };
    }
  }

  // Unknown content
  return {
    type: 'unknown',
    raw: trimmed,
    data: { type: 'unknown', content: trimmed },
    isValid: false,
    error: 'Unrecognized QR code format',
  };
}

/**
 * Parse a PPT deep link.
 */
function parseDeepLink(url: string): ParsedQRCode {
  try {
    const parsed = new URL(url);
    const path = parsed.pathname.replace(/^\//, '');
    const segments = path.split('/');
    const params: Record<string, string> = {};

    // Extract query parameters
    parsed.searchParams.forEach((value, key) => {
      params[key] = value;
    });

    // Determine screen from path
    const screen = segments[0] || 'dashboard';

    // Extract path parameters
    if (segments.length > 1) {
      params.id = segments[1];
    }

    const data: DeepLinkQRData = {
      type: 'deep_link',
      screen,
      params: Object.keys(params).length > 0 ? params : undefined,
    };

    return {
      type: 'deep_link',
      raw: url,
      data,
      isValid: true,
    };
  } catch {
    return {
      type: 'unknown',
      raw: url,
      data: { type: 'unknown', content: url },
      isValid: false,
      error: 'Invalid deep link format',
    };
  }
}

/**
 * Parse a PPT JSON payload.
 */
function parsePPTPayload(json: string): ParsedQRCode {
  try {
    const payload = JSON.parse(json);

    if (!payload.ppt || !payload.type) {
      throw new Error('Invalid PPT payload');
    }

    const type = payload.type as QRCodeType;
    let data: QRCodeData;

    switch (type) {
      case 'building_access':
        data = {
          type: 'building_access',
          buildingId: payload.buildingId,
          accessPoint: payload.accessPoint,
          validUntil: payload.validUntil,
          accessLevel: payload.accessLevel ?? 'resident',
        } as BuildingAccessQRData;
        break;

      case 'document':
        data = {
          type: 'document',
          documentId: payload.documentId,
          documentType: payload.documentType,
          buildingId: payload.buildingId,
        } as DocumentQRData;
        break;

      case 'fault_report':
        data = {
          type: 'fault_report',
          buildingId: payload.buildingId,
          category: payload.category,
          location: payload.location,
          equipmentId: payload.equipmentId,
        } as FaultReportQRData;
        break;

      case 'vote':
        data = {
          type: 'vote',
          voteId: payload.voteId,
          buildingId: payload.buildingId,
        } as VoteQRData;
        break;

      default:
        data = { type: 'unknown', content: json };
    }

    return {
      type,
      raw: json,
      data,
      isValid: true,
    };
  } catch {
    return {
      type: 'unknown',
      raw: json,
      data: { type: 'unknown', content: json },
      isValid: false,
      error: 'Invalid JSON payload',
    };
  }
}

/**
 * Parse a vCard.
 */
function parseVCard(vcard: string): ParsedQRCode {
  const lines = vcard.split(/\r?\n/);
  const contact: ContactQRData = {
    type: 'contact',
    name: '',
  };

  // Check for required vCard structure
  const hasBegin = lines.some((line) => line.trim() === 'BEGIN:VCARD');
  const hasEnd = lines.some((line) => line.trim() === 'END:VCARD');

  for (const line of lines) {
    if (line.startsWith('FN:')) {
      contact.name = line.substring(3);
    } else if (line.startsWith('TEL:') || line.startsWith('TEL;')) {
      contact.phone = line.split(':').pop()?.trim();
    } else if (line.startsWith('EMAIL:') || line.startsWith('EMAIL;')) {
      contact.email = line.split(':').pop()?.trim();
    } else if (line.startsWith('TITLE:')) {
      contact.role = line.substring(6);
    }
  }

  // vCard requires BEGIN, END, and at least FN (formatted name) per RFC 6350
  const isValid = hasBegin && hasEnd && contact.name.length > 0;

  return {
    type: 'contact',
    raw: vcard,
    data: contact,
    isValid,
    error: isValid ? undefined : 'Missing required vCard fields (BEGIN, END, or FN)',
  };
}

/**
 * Get action for a parsed QR code.
 */
export function getQRCodeAction(qrCode: ParsedQRCode): {
  action: string;
  navigation?: string;
  params?: Record<string, string>;
  confirmation: string;
} {
  switch (qrCode.type) {
    case 'deep_link': {
      const data = qrCode.data as DeepLinkQRData;
      return {
        action: 'navigate',
        navigation: data.screen,
        params: data.params,
        confirmation: `Opening ${data.screen}`,
      };
    }

    case 'building_access': {
      const data = qrCode.data as BuildingAccessQRData;
      return {
        action: 'access',
        confirmation: `Building access: ${data.accessPoint}`,
      };
    }

    case 'document': {
      const data = qrCode.data as DocumentQRData;
      return {
        action: 'navigate',
        navigation: 'Documents',
        params: { documentId: data.documentId },
        confirmation: 'Opening document',
      };
    }

    case 'fault_report': {
      const data = qrCode.data as FaultReportQRData;
      return {
        action: 'navigate',
        navigation: 'ReportFault',
        params: {
          buildingId: data.buildingId,
          category: data.category ?? '',
          location: data.location ?? '',
        },
        confirmation: 'Opening fault report form',
      };
    }

    case 'vote': {
      const data = qrCode.data as VoteQRData;
      return {
        action: 'navigate',
        navigation: 'Voting',
        params: { voteId: data.voteId },
        confirmation: 'Opening vote',
      };
    }

    case 'contact': {
      const data = qrCode.data as ContactQRData;
      return {
        action: 'save_contact',
        confirmation: `Save contact: ${data.name}`,
      };
    }

    case 'external_url': {
      const data = qrCode.data as ExternalURLQRData;
      return {
        action: 'open_url',
        confirmation: `Open URL: ${data.url}`,
      };
    }

    default:
      return {
        action: 'unknown',
        confirmation: 'Unknown QR code',
      };
  }
}

/**
 * Provide haptic feedback for scan result.
 */
export function provideScanFeedback(success: boolean): void {
  if (success) {
    Vibration.vibrate([0, 50, 30, 50]);
  } else {
    Vibration.vibrate([0, 100, 50, 100]);
  }
}

/**
 * Save a scan to history.
 */
export async function saveScanToHistory(qrCode: ParsedQRCode, action: string): Promise<void> {
  const history = await getScanHistory();

  const entry: ScanHistoryEntry = {
    id: `scan-${Date.now()}-${Math.random().toString(36).substring(2, 9)}`,
    qrCode,
    scannedAt: new Date().toISOString(),
    action,
  };

  // Add to beginning, limit size
  history.unshift(entry);
  if (history.length > MAX_HISTORY_ENTRIES) {
    history.pop();
  }

  await AsyncStorage.setItem(SCAN_HISTORY_KEY, JSON.stringify(history));
}

/**
 * Get scan history.
 */
export async function getScanHistory(): Promise<ScanHistoryEntry[]> {
  const stored = await AsyncStorage.getItem(SCAN_HISTORY_KEY);
  return stored ? JSON.parse(stored) : [];
}

/**
 * Clear scan history.
 */
export async function clearScanHistory(): Promise<void> {
  await AsyncStorage.removeItem(SCAN_HISTORY_KEY);
}

/**
 * Get recent scans count.
 */
export async function getRecentScansCount(hours = 24): Promise<number> {
  const history = await getScanHistory();
  const cutoff = new Date(Date.now() - hours * 60 * 60 * 1000).toISOString();
  return history.filter((entry) => entry.scannedAt > cutoff).length;
}
