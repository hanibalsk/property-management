/**
 * useSyncNotifications - Hook for managing sync progress notifications (Epic 123 - Story 123.2)
 *
 * Provides state management for sync progress toast and notifications.
 */
import { useCallback, useState } from 'react';

export interface SyncProgressState {
  /** Whether sync progress toast is visible */
  isVisible: boolean;
  /** Current progress percentage (0-100) */
  progress: number;
  /** Total items being synced */
  total: number;
  /** Items synced so far */
  current: number;
  /** Number of failed items */
  failed: number;
  /** Whether sync is complete */
  isComplete: boolean;
}

export interface UseSyncNotificationsReturn {
  /** Current sync progress state */
  syncProgress: SyncProgressState;
  /** Start showing sync progress */
  startSync: (total: number) => void;
  /** Update sync progress */
  updateProgress: (current: number, failed?: number) => void;
  /** Mark sync as complete */
  completeSync: (failed?: number) => void;
  /** Dismiss the sync toast */
  dismissSync: () => void;
  /** Show a quick sync success notification */
  showSyncSuccess: (itemName?: string) => void;
  /** Show a quick sync failure notification */
  showSyncFailure: (itemName?: string) => void;
}

const initialState: SyncProgressState = {
  isVisible: false,
  progress: 0,
  total: 0,
  current: 0,
  failed: 0,
  isComplete: false,
};

export function useSyncNotifications(): UseSyncNotificationsReturn {
  const [syncProgress, setSyncProgress] = useState<SyncProgressState>(initialState);

  const startSync = useCallback((total: number) => {
    setSyncProgress({
      isVisible: true,
      progress: 0,
      total,
      current: 0,
      failed: 0,
      isComplete: false,
    });
  }, []);

  const updateProgress = useCallback((current: number, failed = 0) => {
    setSyncProgress((prev) => ({
      ...prev,
      current,
      failed,
      progress: prev.total > 0 ? Math.round((current / prev.total) * 100) : 0,
    }));
  }, []);

  const completeSync = useCallback((failed = 0) => {
    setSyncProgress((prev) => ({
      ...prev,
      current: prev.total,
      failed,
      progress: 100,
      isComplete: true,
    }));
  }, []);

  const dismissSync = useCallback(() => {
    setSyncProgress(initialState);
  }, []);

  const showSyncSuccess = useCallback((_itemName?: string) => {
    setSyncProgress({
      isVisible: true,
      progress: 100,
      total: 1,
      current: 1,
      failed: 0,
      isComplete: true,
    });

    // Auto-dismiss handled by the toast component
  }, []);

  const showSyncFailure = useCallback((_itemName?: string) => {
    setSyncProgress({
      isVisible: true,
      progress: 100,
      total: 1,
      current: 0,
      failed: 1,
      isComplete: true,
    });
  }, []);

  return {
    syncProgress,
    startSync,
    updateProgress,
    completeSync,
    dismissSync,
    showSyncSuccess,
    showSyncFailure,
  };
}
