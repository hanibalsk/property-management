/**
 * Person-months feature type definitions.
 */

/**
 * Export format options for person-month data.
 */
export type ExportFormat = 'csv' | 'excel';

/**
 * Single unit entry for bulk person-month entry.
 */
export interface BulkEntryUnit {
  unitId: string;
  unitDesignation: string;
  personCount: number;
  currentPersonCount?: number;
}

/**
 * Form data for bulk person-month entry.
 */
export interface BulkEntryFormData {
  year: number;
  month: number;
  entries: BulkEntryUnit[];
}

/**
 * Validation error for bulk entry.
 */
export interface BulkEntryValidationError {
  unitId: string;
  message: string;
}

/**
 * Result of bulk entry submission for a single unit.
 */
export interface BulkEntryResult {
  unitId: string;
  unitDesignation: string;
  success: boolean;
  error?: string;
}

/**
 * Summary of bulk entry submission results.
 */
export interface BulkEntrySummary {
  total: number;
  successful: number;
  failed: number;
  results: BulkEntryResult[];
}

/**
 * Export data structure for person-months.
 */
export interface PersonMonthExportData {
  buildingName: string;
  year: number;
  month: number;
  units: {
    unitDesignation: string;
    personCount: number;
    updatedAt?: string;
  }[];
  totalPersons: number;
  exportedAt: string;
}

/**
 * Export data structure for unit person-month history.
 */
export interface UnitPersonMonthExportData {
  buildingName: string;
  unitDesignation: string;
  year: number;
  entries: {
    month: number;
    personCount: number;
    updatedAt?: string;
  }[];
  totalPersonMonths: number;
  averagePersons: number;
  exportedAt: string;
}
