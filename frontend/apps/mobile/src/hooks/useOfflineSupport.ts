import AsyncStorage from '@react-native-async-storage/async-storage';
import NetInfo, { type NetInfoState } from '@react-native-community/netinfo';
import { useCallback, useEffect, useState } from 'react';

// Storage keys
const CACHE_PREFIX = 'ppt_cache_';
const QUEUE_KEY = 'ppt_offline_queue';
const LAST_SYNC_KEY = 'ppt_last_sync';

export interface CacheOptions {
  expiresIn?: number; // milliseconds
  key: string;
}

export type SyncItemStatus = 'pending' | 'syncing' | 'synced' | 'failed';

export interface QueuedAction {
  id: string;
  type: 'CREATE' | 'UPDATE' | 'DELETE';
  endpoint: string;
  method: 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE';
  body?: unknown;
  timestamp: number;
  retries: number;
  syncStatus?: SyncItemStatus;
}

export interface SyncProgress {
  total: number;
  current: number;
  failed: number;
  isComplete: boolean;
}

export type SyncProgressCallback = (progress: SyncProgress) => void;

export interface OfflineState {
  isConnected: boolean;
  isInternetReachable: boolean | null;
  connectionType: string | null;
  queuedActionsCount: number;
  lastSyncTime: Date | null;
}

export interface UseOfflineSupportReturn extends OfflineState {
  // Caching
  cacheData: <T>(key: string, data: T, expiresIn?: number) => Promise<void>;
  getCachedData: <T>(key: string) => Promise<T | null>;
  clearCache: (key?: string) => Promise<void>;
  // Offline queue
  addToQueue: (action: Omit<QueuedAction, 'id' | 'timestamp' | 'retries'>) => Promise<void>;
  getQueuedActions: () => Promise<QueuedAction[]>;
  processQueue: (onProgress?: SyncProgressCallback) => Promise<{ success: number; failed: number }>;
  clearQueue: () => Promise<void>;
  // Sync
  syncData: (onProgress?: SyncProgressCallback) => Promise<void>;
  isSyncing: boolean;
  // Sync progress (for UI binding)
  syncProgress: SyncProgress | null;
}

export function useOfflineSupport(): UseOfflineSupportReturn {
  const [isConnected, setIsConnected] = useState(true);
  const [isInternetReachable, setIsInternetReachable] = useState<boolean | null>(null);
  const [connectionType, setConnectionType] = useState<string | null>(null);
  const [queuedActionsCount, setQueuedActionsCount] = useState(0);
  const [lastSyncTime, setLastSyncTime] = useState<Date | null>(null);
  const [isSyncing, setIsSyncing] = useState(false);
  const [syncProgress, setSyncProgress] = useState<SyncProgress | null>(null);

  // Monitor network status
  useEffect(() => {
    const unsubscribe = NetInfo.addEventListener((state: NetInfoState) => {
      setIsConnected(state.isConnected ?? false);
      setIsInternetReachable(state.isInternetReachable);
      setConnectionType(state.type);

      // Auto-sync when coming back online
      if (state.isConnected && state.isInternetReachable) {
        processQueue();
      }
    });

    // Initial network check
    NetInfo.fetch().then((state: NetInfoState) => {
      setIsConnected(state.isConnected ?? false);
      setIsInternetReachable(state.isInternetReachable);
      setConnectionType(state.type);
    });

    // Load last sync time
    loadLastSyncTime();

    // Load queue count
    loadQueueCount();

    return () => unsubscribe();
  }, []);

  const loadLastSyncTime = async () => {
    try {
      const timestamp = await AsyncStorage.getItem(LAST_SYNC_KEY);
      if (timestamp) {
        setLastSyncTime(new Date(Number.parseInt(timestamp, 10)));
      }
    } catch (error) {
      console.error('Failed to load last sync time:', error);
    }
  };

  const loadQueueCount = async () => {
    try {
      const queue = await getQueuedActions();
      setQueuedActionsCount(queue.length);
    } catch (error) {
      console.error('Failed to load queue count:', error);
    }
  };

  // Cache data locally
  const cacheData = useCallback(
    async <T>(key: string, data: T, expiresIn?: number): Promise<void> => {
      try {
        const cacheEntry = {
          data,
          timestamp: Date.now(),
          expiresAt: expiresIn ? Date.now() + expiresIn : null,
        };
        await AsyncStorage.setItem(`${CACHE_PREFIX}${key}`, JSON.stringify(cacheEntry));
      } catch (error) {
        console.error('Failed to cache data:', error);
      }
    },
    []
  );

  // Get cached data
  const getCachedData = useCallback(async <T>(key: string): Promise<T | null> => {
    try {
      const cached = await AsyncStorage.getItem(`${CACHE_PREFIX}${key}`);
      if (!cached) return null;

      const { data, expiresAt } = JSON.parse(cached);

      // Check if expired
      if (expiresAt && Date.now() > expiresAt) {
        await AsyncStorage.removeItem(`${CACHE_PREFIX}${key}`);
        return null;
      }

      return data as T;
    } catch (error) {
      console.error('Failed to get cached data:', error);
      return null;
    }
  }, []);

  // Clear cache
  const clearCache = useCallback(async (key?: string): Promise<void> => {
    try {
      if (key) {
        await AsyncStorage.removeItem(`${CACHE_PREFIX}${key}`);
      } else {
        // Clear all cache entries
        const keys = await AsyncStorage.getAllKeys();
        const cacheKeys = keys.filter((k: string) => k.startsWith(CACHE_PREFIX));
        await AsyncStorage.multiRemove(cacheKeys);
      }
    } catch (error) {
      console.error('Failed to clear cache:', error);
    }
  }, []);

  // Add action to offline queue
  const addToQueue = useCallback(
    async (action: Omit<QueuedAction, 'id' | 'timestamp' | 'retries'>): Promise<void> => {
      try {
        const queue = await getQueuedActions();

        const newAction: QueuedAction = {
          ...action,
          id: `${Date.now()}-${Math.random().toString(36).substring(2)}`,
          timestamp: Date.now(),
          retries: 0,
        };

        queue.push(newAction);
        await AsyncStorage.setItem(QUEUE_KEY, JSON.stringify(queue));
        setQueuedActionsCount(queue.length);
      } catch (error) {
        console.error('Failed to add to queue:', error);
      }
    },
    []
  );

  // Get all queued actions
  const getQueuedActions = useCallback(async (): Promise<QueuedAction[]> => {
    try {
      const queue = await AsyncStorage.getItem(QUEUE_KEY);
      return queue ? JSON.parse(queue) : [];
    } catch (error) {
      console.error('Failed to get queued actions:', error);
      return [];
    }
  }, []);

  // Process offline queue when back online
  const processQueue = useCallback(
    async (onProgress?: SyncProgressCallback): Promise<{ success: number; failed: number }> => {
      if (!isConnected || !isInternetReachable) {
        return { success: 0, failed: 0 };
      }

      setIsSyncing(true);
      let success = 0;
      let failed = 0;

      try {
        const queue = await getQueuedActions();
        const total = queue.length;

        if (total === 0) {
          return { success: 0, failed: 0 };
        }

        // Initialize progress
        const initialProgress: SyncProgress = { total, current: 0, failed: 0, isComplete: false };
        setSyncProgress(initialProgress);
        onProgress?.(initialProgress);

        const remainingActions: QueuedAction[] = [];

        for (let i = 0; i < queue.length; i++) {
          const action = queue[i];
          try {
            // Execute the queued action
            // In a real app, this would make the actual API call
            await executeQueuedAction(action);
            success++;
          } catch (_error) {
            // Increment retry count
            action.retries++;

            // Keep in queue if under max retries
            if (action.retries < 3) {
              remainingActions.push(action);
            } else {
              failed++;
              console.error('Action failed after max retries:', action);
            }
          }

          // Update progress after each item
          const currentProgress: SyncProgress = {
            total,
            current: i + 1,
            failed,
            isComplete: false,
          };
          setSyncProgress(currentProgress);
          onProgress?.(currentProgress);
        }

        // Update queue with remaining actions
        await AsyncStorage.setItem(QUEUE_KEY, JSON.stringify(remainingActions));
        setQueuedActionsCount(remainingActions.length);

        // Update last sync time
        const now = Date.now();
        await AsyncStorage.setItem(LAST_SYNC_KEY, now.toString());
        setLastSyncTime(new Date(now));

        // Final progress update
        const finalProgress: SyncProgress = { total, current: total, failed, isComplete: true };
        setSyncProgress(finalProgress);
        onProgress?.(finalProgress);

        return { success, failed };
      } catch (error) {
        console.error('Failed to process queue:', error);
        return { success, failed };
      } finally {
        setIsSyncing(false);
      }
    },
    [isConnected, isInternetReachable, getQueuedActions]
  );

  // Execute a single queued action
  const executeQueuedAction = async (action: QueuedAction): Promise<void> => {
    // This would make the actual API call
    // For now, simulate success
    console.log('Executing queued action:', action);
    await new Promise((resolve) => setTimeout(resolve, 100));
  };

  // Clear the offline queue
  const clearQueue = useCallback(async (): Promise<void> => {
    try {
      await AsyncStorage.removeItem(QUEUE_KEY);
      setQueuedActionsCount(0);
    } catch (error) {
      console.error('Failed to clear queue:', error);
    }
  }, []);

  // Sync all data
  const syncData = useCallback(
    async (onProgress?: SyncProgressCallback): Promise<void> => {
      if (!isConnected || !isInternetReachable) {
        return;
      }

      setIsSyncing(true);

      try {
        // Process offline queue first
        await processQueue(onProgress);

        // Then fetch fresh data and cache it
        // This would call your API endpoints and cache the responses
        // await cacheData('announcements', await api.getAnnouncements(), 5 * 60 * 1000);
        // await cacheData('faults', await api.getFaults(), 5 * 60 * 1000);
        // await cacheData('votes', await api.getVotes(), 5 * 60 * 1000);

        console.log('Data sync completed');
      } catch (error) {
        console.error('Failed to sync data:', error);
      } finally {
        setIsSyncing(false);
      }
    },
    [isConnected, isInternetReachable, processQueue]
  );

  return {
    isConnected,
    isInternetReachable,
    connectionType,
    queuedActionsCount,
    lastSyncTime,
    cacheData,
    getCachedData,
    clearCache,
    addToQueue,
    getQueuedActions,
    processQueue,
    clearQueue,
    syncData,
    isSyncing,
    syncProgress,
  };
}
