/**
 * QR Code generation utilities.
 *
 * Epic 49 - Story 49.3: QR Code Scanning
 */
import type {
  BuildingAccessQRData,
  DocumentQRData,
  FaultReportQRData,
  QRCodeGenerationOptions,
  QRDeepLinkTarget,
  VoteQRData,
} from './types';

/**
 * Default QR code generation options.
 */
export const DEFAULT_QR_OPTIONS: QRCodeGenerationOptions = {
  size: 256,
  errorCorrectionLevel: 'M',
  color: '#000000',
  backgroundColor: '#FFFFFF',
  includeLogo: false,
  margin: 4,
};

/**
 * Generate a deep link QR code content.
 */
export function generateDeepLinkQR(target: QRDeepLinkTarget): string {
  let url = `ppt://${target.screen}`;

  if (target.params && Object.keys(target.params).length > 0) {
    const searchParams = new URLSearchParams(target.params);
    url += `?${searchParams.toString()}`;
  }

  return url;
}

/**
 * Generate a building access QR code content.
 */
export function generateBuildingAccessQR(data: Omit<BuildingAccessQRData, 'type'>): string {
  const payload = {
    ppt: '1.0',
    type: 'building_access',
    ...data,
    generatedAt: new Date().toISOString(),
  };

  return JSON.stringify(payload);
}

/**
 * Generate a document QR code content.
 */
export function generateDocumentQR(data: Omit<DocumentQRData, 'type'>): string {
  const payload = {
    ppt: '1.0',
    type: 'document',
    ...data,
  };

  return JSON.stringify(payload);
}

/**
 * Generate a fault report QR code content.
 */
export function generateFaultReportQR(data: Omit<FaultReportQRData, 'type'>): string {
  const payload = {
    ppt: '1.0',
    type: 'fault_report',
    ...data,
  };

  return JSON.stringify(payload);
}

/**
 * Generate a vote QR code content.
 */
export function generateVoteQR(data: Omit<VoteQRData, 'type'>): string {
  const payload = {
    ppt: '1.0',
    type: 'vote',
    ...data,
  };

  return JSON.stringify(payload);
}

/**
 * Generate a vCard QR code content.
 */
export function generateContactQR(contact: {
  name: string;
  phone?: string;
  email?: string;
  organization?: string;
  role?: string;
}): string {
  const lines: string[] = ['BEGIN:VCARD', 'VERSION:3.0', `FN:${contact.name}`];

  if (contact.phone) {
    lines.push(`TEL:${contact.phone}`);
  }

  if (contact.email) {
    lines.push(`EMAIL:${contact.email}`);
  }

  if (contact.organization) {
    lines.push(`ORG:${contact.organization}`);
  }

  if (contact.role) {
    lines.push(`TITLE:${contact.role}`);
  }

  lines.push('END:VCARD');

  return lines.join('\r\n');
}

/**
 * Get shareable content for a QR code.
 */
export function getQRShareContent(
  content: string,
  description?: string
): { title: string; message: string; url?: string } {
  if (content.startsWith('ppt://')) {
    return {
      title: description ?? 'PPT Link',
      message: description ?? 'Check out this link from Property Management',
      url: content,
    };
  }

  if (content.startsWith('http')) {
    return {
      title: description ?? 'Link',
      message: description ?? 'Check out this link',
      url: content,
    };
  }

  return {
    title: description ?? 'QR Code',
    message: content,
  };
}

/**
 * Pre-defined QR code templates for common use cases.
 */
export const QR_TEMPLATES = {
  /**
   * Dashboard link.
   */
  dashboard: (): string => generateDeepLinkQR({ screen: 'dashboard' }),

  /**
   * Fault report for specific location.
   */
  faultReport: (buildingId: string, location?: string): string =>
    generateFaultReportQR({ buildingId, location }),

  /**
   * Specific document.
   */
  document: (documentId: string, documentType: string): string =>
    generateDocumentQR({ documentId, documentType }),

  /**
   * Voting link.
   */
  vote: (voteId: string, buildingId: string): string => generateVoteQR({ voteId, buildingId }),

  /**
   * Announcements screen.
   */
  announcements: (): string => generateDeepLinkQR({ screen: 'announcements' }),

  /**
   * Building-specific access.
   */
  buildingAccess: (
    buildingId: string,
    accessPoint: string,
    accessLevel: 'resident' | 'guest' | 'service' = 'resident',
    validMinutes?: number
  ): string =>
    generateBuildingAccessQR({
      buildingId,
      accessPoint,
      accessLevel,
      validUntil: validMinutes
        ? new Date(Date.now() + validMinutes * 60 * 1000).toISOString()
        : undefined,
    }),
};

/**
 * Validate QR code content length.
 */
export function validateQRContentLength(content: string): {
  valid: boolean;
  length: number;
  maxLength: number;
  recommendedLevel: 'L' | 'M' | 'Q' | 'H';
} {
  const length = content.length;

  // Approximate max lengths for each error correction level (alphanumeric mode)
  const maxLengths = {
    L: 4296,
    M: 3391,
    Q: 2420,
    H: 1852,
  };

  let recommendedLevel: 'L' | 'M' | 'Q' | 'H' = 'M';
  let valid = true;

  if (length > maxLengths.L) {
    valid = false;
  } else if (length > maxLengths.M) {
    recommendedLevel = 'L';
  } else if (length <= maxLengths.H) {
    recommendedLevel = 'H';
  } else if (length <= maxLengths.Q) {
    recommendedLevel = 'Q';
  }

  return {
    valid,
    length,
    maxLength: maxLengths[recommendedLevel],
    recommendedLevel,
  };
}
