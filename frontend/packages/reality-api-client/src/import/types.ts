/**
 * Import Types
 *
 * TypeScript types for property import functionality (Epic 46).
 */

// CSV Import types
export interface CsvImportPreview {
  totalRows: number;
  validRows: number;
  invalidRows: number;
  headers: string[];
  sampleData: Record<string, string>[];
  errors: CsvValidationError[];
  columnMapping?: ColumnMapping;
}

export interface CsvValidationError {
  row: number;
  column: string;
  value: string;
  message: string;
  severity: 'error' | 'warning';
}

export interface ColumnMapping {
  title: string;
  description?: string;
  propertyType: string;
  transactionType: string;
  price: string;
  currency?: string;
  address?: string;
  city?: string;
  postalCode?: string;
  rooms?: string;
  bathrooms?: string;
  size?: string;
  yearBuilt?: string;
  features?: string;
  photos?: string;
}

export interface CsvImportRequest {
  file: File;
  mapping: ColumnMapping;
  skipInvalid: boolean;
}

export interface CsvImportResult {
  success: boolean;
  importId: string;
  totalProcessed: number;
  successCount: number;
  failedCount: number;
  skippedCount: number;
  errors: CsvValidationError[];
  createdListingIds: string[];
}

// CRM Connection types
export type CrmProvider = 'salesforce' | 'hubspot' | 'pipedrive' | 'zoho' | 'custom';

export interface CrmConnection {
  id: string;
  agencyId: string;
  provider: CrmProvider;
  name: string;
  status: CrmConnectionStatus;
  lastSyncAt?: string;
  nextSyncAt?: string;
  syncFrequency: SyncFrequency;
  fieldMapping: CrmFieldMapping;
  createdAt: string;
  updatedAt: string;
}

export type CrmConnectionStatus = 'connected' | 'disconnected' | 'error' | 'syncing';

export interface CrmFieldMapping {
  [localField: string]: string;
}

export interface CrmCredentials {
  apiKey?: string;
  apiSecret?: string;
  accessToken?: string;
  refreshToken?: string;
  instanceUrl?: string;
}

export interface CreateCrmConnectionRequest {
  provider: CrmProvider;
  name: string;
  credentials: CrmCredentials;
  fieldMapping: CrmFieldMapping;
  syncFrequency: SyncFrequency;
}

export interface UpdateCrmConnectionRequest {
  name?: string;
  fieldMapping?: CrmFieldMapping;
  syncFrequency?: SyncFrequency;
}

export interface CrmConnectionTestResult {
  success: boolean;
  message: string;
  availableFields?: string[];
}

// Sync Schedule types
export type SyncFrequency = 'manual' | 'hourly' | 'daily' | 'weekly';

export interface SyncSchedule {
  id: string;
  connectionId: string;
  frequency: SyncFrequency;
  preferredTime?: string;
  preferredDay?: number;
  enabled: boolean;
  lastRunAt?: string;
  nextRunAt?: string;
  createdAt: string;
  updatedAt: string;
}

export interface SyncHistoryItem {
  id: string;
  connectionId: string;
  startedAt: string;
  completedAt?: string;
  status: SyncStatus;
  recordsProcessed: number;
  recordsCreated: number;
  recordsUpdated: number;
  recordsFailed: number;
  errors?: string[];
}

export type SyncStatus = 'running' | 'completed' | 'failed' | 'cancelled';

export interface UpdateSyncScheduleRequest {
  frequency: SyncFrequency;
  preferredTime?: string;
  preferredDay?: number;
  enabled: boolean;
}

// Feed Import types
export type FeedFormat = 'xml' | 'rss' | 'atom' | 'json';

export interface FeedSource {
  id: string;
  agencyId: string;
  name: string;
  url: string;
  format: FeedFormat;
  status: FeedStatus;
  lastFetchAt?: string;
  nextFetchAt?: string;
  syncFrequency: SyncFrequency;
  fieldMapping: FeedFieldMapping;
  totalListings: number;
  createdAt: string;
  updatedAt: string;
}

export type FeedStatus = 'active' | 'paused' | 'error';

export interface FeedFieldMapping {
  [localField: string]: string;
}

export interface FeedPreview {
  success: boolean;
  format: FeedFormat;
  totalItems: number;
  sampleItems: Record<string, unknown>[];
  availableFields: string[];
  errors?: string[];
}

export interface CreateFeedSourceRequest {
  name: string;
  url: string;
  format?: FeedFormat;
  fieldMapping: FeedFieldMapping;
  syncFrequency: SyncFrequency;
}

export interface UpdateFeedSourceRequest {
  name?: string;
  url?: string;
  fieldMapping?: FeedFieldMapping;
  syncFrequency?: SyncFrequency;
  status?: FeedStatus;
}

export interface FeedImportResult {
  success: boolean;
  totalProcessed: number;
  created: number;
  updated: number;
  skipped: number;
  errors: string[];
}
