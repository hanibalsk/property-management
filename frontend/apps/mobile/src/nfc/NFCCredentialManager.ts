/**
 * NFC credential management for building access.
 *
 * Epic 49 - Story 49.4: NFC Building Access
 */
import AsyncStorage from '@react-native-async-storage/async-storage';

import type {
  AccessDenialReason,
  AccessLogEntry,
  GuestAccessInvitation,
  NFCCredential,
} from './types';

// WARNING: NFC credentials are stored in AsyncStorage for offline access.
// In production, consider using react-native-keychain or expo-secure-store
// for sensitive credential data. AsyncStorage is not encrypted on most devices.
// See security review: docs/security/credential-storage.md
const CREDENTIALS_KEY = '@ppt/nfc_credentials';
const ACCESS_LOG_KEY = '@ppt/access_log';
const MAX_LOG_ENTRIES = 100;

/**
 * Manages NFC credentials for building access.
 */
export class NFCCredentialManager {
  private apiBaseUrl: string;
  private authToken: string | null = null;
  private credentials: NFCCredential[] = [];

  constructor(apiBaseUrl: string) {
    this.apiBaseUrl = apiBaseUrl;
  }

  /**
   * Set authentication token.
   */
  setAuthToken(token: string | null): void {
    this.authToken = token;
  }

  /**
   * Initialize and load stored credentials.
   */
  async initialize(): Promise<void> {
    await this.loadStoredCredentials();
  }

  /**
   * Fetch credentials from server.
   */
  async fetchCredentials(): Promise<NFCCredential[]> {
    try {
      const response = await this.apiRequest<{ credentials: NFCCredential[] }>(
        '/api/v1/access/credentials'
      );

      this.credentials = response.credentials;
      await this.storeCredentials();

      return this.credentials;
    } catch (error) {
      // Fall back to cached credentials
      console.warn('Failed to fetch credentials, using cached:', error);
      return this.credentials;
    }
  }

  /**
   * Get credential for a specific building.
   */
  getCredentialForBuilding(buildingId: string): NFCCredential | undefined {
    return this.credentials.find((c) => c.buildingId === buildingId && c.status === 'active');
  }

  /**
   * Get all active credentials.
   */
  getActiveCredentials(): NFCCredential[] {
    const nowMs = Date.now();
    return this.credentials.filter(
      (c) => c.status === 'active' && new Date(c.validUntil).getTime() > nowMs
    );
  }

  /**
   * Get credential by ID.
   */
  getCredentialById(id: string): NFCCredential | undefined {
    return this.credentials.find((c) => c.id === id);
  }

  /**
   * Refresh a credential (get new encrypted data).
   */
  async refreshCredential(credentialId: string): Promise<NFCCredential> {
    const response = await this.apiRequest<{ credential: NFCCredential }>(
      `/api/v1/access/credentials/${credentialId}/refresh`,
      'POST'
    );

    // Update local copy
    const index = this.credentials.findIndex((c) => c.id === credentialId);
    if (index >= 0) {
      this.credentials[index] = response.credential;
      await this.storeCredentials();
    }

    return response.credential;
  }

  /**
   * Log an access attempt.
   */
  async logAccessAttempt(
    credentialId: string,
    accessPointId: string,
    accessPointName: string,
    result: 'granted' | 'denied',
    denialReason?: AccessDenialReason
  ): Promise<void> {
    const entry: AccessLogEntry = {
      id: `log-${Date.now()}-${Math.random().toString(36).substring(2, 9)}`,
      credentialId,
      buildingId: this.getCredentialById(credentialId)?.buildingId ?? '',
      accessPointId,
      accessPointName,
      result,
      denialReason: denialReason as AccessLogEntry['denialReason'] | undefined,
      timestamp: new Date().toISOString(),
    };

    // Store locally
    const log = await this.getAccessLog();
    log.unshift(entry);
    if (log.length > MAX_LOG_ENTRIES) {
      log.pop();
    }
    await AsyncStorage.setItem(ACCESS_LOG_KEY, JSON.stringify(log));

    // Sync to server
    try {
      await this.apiRequest('/api/v1/access/log', 'POST', entry);
    } catch {
      // Will sync later
    }
  }

  /**
   * Get access log.
   */
  async getAccessLog(): Promise<AccessLogEntry[]> {
    const stored = await AsyncStorage.getItem(ACCESS_LOG_KEY);
    return stored ? JSON.parse(stored) : [];
  }

  /**
   * Get recent access attempts.
   */
  async getRecentAccessAttempts(limit = 10): Promise<AccessLogEntry[]> {
    const log = await this.getAccessLog();
    return log.slice(0, limit);
  }

  /**
   * Create a guest access invitation.
   */
  async createGuestInvitation(
    buildingId: string,
    guestName: string,
    guestEmail: string,
    accessPoints: string[],
    validFrom: string,
    validUntil: string,
    maxEntries?: number
  ): Promise<GuestAccessInvitation> {
    const response = await this.apiRequest<{ invitation: GuestAccessInvitation }>(
      '/api/v1/access/invitations',
      'POST',
      {
        buildingId,
        guestName,
        guestEmail,
        accessPoints,
        validFrom,
        validUntil,
        maxEntries,
      }
    );

    return response.invitation;
  }

  /**
   * Get guest invitations for a building.
   */
  async getGuestInvitations(buildingId: string): Promise<GuestAccessInvitation[]> {
    const response = await this.apiRequest<{ invitations: GuestAccessInvitation[] }>(
      `/api/v1/access/invitations?building_id=${buildingId}`
    );

    return response.invitations;
  }

  /**
   * Cancel a guest invitation.
   */
  async cancelGuestInvitation(invitationId: string): Promise<void> {
    await this.apiRequest(`/api/v1/access/invitations/${invitationId}/cancel`, 'POST');
  }

  /**
   * Report lost phone / emergency revoke all credentials.
   */
  async emergencyRevokeAll(): Promise<void> {
    await this.apiRequest('/api/v1/access/credentials/revoke-all', 'POST');
    this.credentials = [];
    await this.storeCredentials();
  }

  /**
   * Check if credentials need refresh.
   */
  credentialsNeedRefresh(): boolean {
    const refreshThreshold = 24 * 60 * 60 * 1000; // 24 hours
    const now = Date.now();

    for (const credential of this.credentials) {
      if (credential.status !== 'active') continue;

      const validUntil = new Date(credential.validUntil).getTime();
      if (validUntil - now < refreshThreshold) {
        return true;
      }
    }

    return false;
  }

  /**
   * Update credential usage.
   */
  async updateCredentialUsage(credentialId: string): Promise<void> {
    const credential = this.credentials.find((c) => c.id === credentialId);
    if (credential) {
      credential.lastUsed = new Date().toISOString();
      credential.usageCount += 1;
      await this.storeCredentials();
    }
  }

  // Private methods

  private async loadStoredCredentials(): Promise<void> {
    const stored = await AsyncStorage.getItem(CREDENTIALS_KEY);
    if (stored) {
      this.credentials = JSON.parse(stored);
    }
  }

  private async storeCredentials(): Promise<void> {
    await AsyncStorage.setItem(CREDENTIALS_KEY, JSON.stringify(this.credentials));
  }

  private async apiRequest<T>(
    endpoint: string,
    method: 'GET' | 'POST' = 'GET',
    body?: unknown
  ): Promise<T> {
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
    };

    if (this.authToken) {
      headers.Authorization = `Bearer ${this.authToken}`;
    }

    const response = await fetch(`${this.apiBaseUrl}${endpoint}`, {
      method,
      headers,
      body: body ? JSON.stringify(body) : undefined,
    });

    if (!response.ok) {
      throw new Error(`API request failed: ${response.status}`);
    }

    return response.json();
  }
}
