/**
 * Feedback and bug report manager.
 *
 * Epic 50 - Story 50.4: Feedback & Bug Reports
 */
import AsyncStorage from '@react-native-async-storage/async-storage';
import { Platform } from 'react-native';

import type { AppContext, DeviceInfo, FeedbackSubmission, FeedbackType } from './types';

const FEEDBACK_DRAFTS_KEY = '@ppt/feedback_drafts';
const PENDING_FEEDBACK_KEY = '@ppt/pending_feedback';

/**
 * Manages feedback and bug reports.
 */
export class FeedbackManager {
  private apiBaseUrl: string;
  private authToken: string | null = null;
  private appVersion: string;
  private buildNumber: string;
  private sessionStartTime: number = Date.now();
  private currentScreen = 'Dashboard';
  private userId?: string;
  private buildingId?: string;

  constructor(apiBaseUrl: string, appVersion: string, buildNumber: string) {
    this.apiBaseUrl = apiBaseUrl;
    this.appVersion = appVersion;
    this.buildNumber = buildNumber;
  }

  /**
   * Set authentication token.
   */
  setAuthToken(token: string | null): void {
    this.authToken = token;
  }

  /**
   * Update current context.
   */
  updateContext(context: { screen?: string; userId?: string; buildingId?: string }): void {
    if (context.screen) this.currentScreen = context.screen;
    if (context.userId !== undefined) this.userId = context.userId;
    if (context.buildingId !== undefined) this.buildingId = context.buildingId;
  }

  /**
   * Get device information.
   */
  getDeviceInfo(): DeviceInfo {
    // Note: For more detailed device info, consider using react-native-device-info
    // which provides actual device model, manufacturer, etc.
    return {
      platform: Platform.OS as 'ios' | 'android',
      osVersion: Platform.Version?.toString() ?? 'unknown',
      // Generic placeholder - use react-native-device-info for actual model
      deviceModel: Platform.OS === 'ios' ? 'iOS Device' : 'Android Device',
      appVersion: this.appVersion,
      buildNumber: this.buildNumber,
      // Use NativeModules or react-native-localize for proper locale detection
      locale: Intl.DateTimeFormat().resolvedOptions().locale ?? 'en-US',
      timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
    };
  }

  /**
   * Get current app context.
   */
  getAppContext(): AppContext {
    return {
      currentScreen: this.currentScreen,
      userId: this.userId,
      buildingId: this.buildingId,
      timestamp: new Date().toISOString(),
      sessionDuration: Math.floor((Date.now() - this.sessionStartTime) / 1000),
    };
  }

  /**
   * Create a feedback submission.
   */
  createFeedback(
    type: FeedbackType,
    title: string,
    description: string,
    email?: string,
    screenshot?: string,
    includeDeviceInfo = true
  ): FeedbackSubmission {
    return {
      type,
      title,
      description,
      email,
      screenshot,
      // Only include device info if the user opted in
      deviceInfo: includeDeviceInfo ? this.getDeviceInfo() : undefined,
      appContext: includeDeviceInfo ? this.getAppContext() : undefined,
    };
  }

  /**
   * Submit feedback.
   */
  async submitFeedback(feedback: FeedbackSubmission): Promise<{ success: boolean; id?: string }> {
    try {
      const response = await this.apiRequest<{ id: string }>('/api/v1/feedback', 'POST', feedback);

      return { success: true, id: response.id };
    } catch (error) {
      // Log error in development for debugging
      if (__DEV__) {
        console.warn('Failed to submit feedback:', error);
      }
      // Store for later submission
      await this.storePendingFeedback(feedback);
      return { success: false };
    }
  }

  /**
   * Save feedback draft.
   */
  async saveDraft(feedback: Partial<FeedbackSubmission>): Promise<void> {
    const drafts = await this.getDrafts();
    drafts.push({
      ...feedback,
      savedAt: new Date().toISOString(),
    });
    await AsyncStorage.setItem(FEEDBACK_DRAFTS_KEY, JSON.stringify(drafts));
  }

  /**
   * Get saved drafts.
   */
  async getDrafts(): Promise<Array<Partial<FeedbackSubmission> & { savedAt: string }>> {
    const stored = await AsyncStorage.getItem(FEEDBACK_DRAFTS_KEY);
    return stored ? JSON.parse(stored) : [];
  }

  /**
   * Delete a draft.
   */
  async deleteDraft(index: number): Promise<void> {
    const drafts = await this.getDrafts();
    drafts.splice(index, 1);
    await AsyncStorage.setItem(FEEDBACK_DRAFTS_KEY, JSON.stringify(drafts));
  }

  /**
   * Clear all drafts.
   */
  async clearDrafts(): Promise<void> {
    await AsyncStorage.removeItem(FEEDBACK_DRAFTS_KEY);
  }

  /**
   * Retry pending feedback submissions.
   */
  async retryPendingFeedback(): Promise<number> {
    const pending = await this.getPendingFeedback();
    let successCount = 0;
    const remaining: FeedbackSubmission[] = [];

    for (const feedback of pending) {
      try {
        await this.apiRequest('/api/v1/feedback', 'POST', feedback);
        successCount++;
      } catch (error) {
        // Log retry failures in development
        if (__DEV__) {
          console.warn('Failed to retry feedback submission:', error);
        }
        remaining.push(feedback);
      }
    }

    await AsyncStorage.setItem(PENDING_FEEDBACK_KEY, JSON.stringify(remaining));
    return successCount;
  }

  /**
   * Get pending feedback count.
   */
  async getPendingFeedbackCount(): Promise<number> {
    const pending = await this.getPendingFeedback();
    return pending.length;
  }

  /**
   * Get feedback type options.
   */
  getFeedbackTypes(): Array<{ type: FeedbackType; label: string; icon: string }> {
    return [
      { type: 'bug', label: 'Bug Report', icon: '🐛' },
      { type: 'feature', label: 'Feature Request', icon: '💡' },
      { type: 'improvement', label: 'Improvement', icon: '✨' },
      { type: 'question', label: 'Question', icon: '❓' },
      { type: 'other', label: 'Other', icon: '📝' },
    ];
  }

  /**
   * Generate diagnostic report.
   */
  generateDiagnosticReport(): string {
    const deviceInfo = this.getDeviceInfo();
    const appContext = this.getAppContext();

    return `
=== Device Information ===
Platform: ${deviceInfo.platform}
OS Version: ${deviceInfo.osVersion}
Device Model: ${deviceInfo.deviceModel}
App Version: ${deviceInfo.appVersion}
Build Number: ${deviceInfo.buildNumber}
Locale: ${deviceInfo.locale}
Timezone: ${deviceInfo.timezone}

=== App Context ===
Current Screen: ${appContext.currentScreen}
Session Duration: ${Math.floor(appContext.sessionDuration / 60)}m ${appContext.sessionDuration % 60}s
Timestamp: ${appContext.timestamp}
    `.trim();
  }

  // Private methods

  private async getPendingFeedback(): Promise<FeedbackSubmission[]> {
    const stored = await AsyncStorage.getItem(PENDING_FEEDBACK_KEY);
    return stored ? JSON.parse(stored) : [];
  }

  private async storePendingFeedback(feedback: FeedbackSubmission): Promise<void> {
    const pending = await this.getPendingFeedback();
    pending.push(feedback);
    await AsyncStorage.setItem(PENDING_FEEDBACK_KEY, JSON.stringify(pending));
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

/**
 * Global singleton instance of FeedbackManager.
 * Should be configured at app startup with proper base URL and version info.
 */
export const feedbackManager = new FeedbackManager(
  'https://api.ppt.example.com', // Will be configured at runtime
  '1.0.0',
  '1'
);
