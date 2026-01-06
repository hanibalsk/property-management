/**
 * Types for Meters feature.
 * Meter self-readings and consumption tracking.
 */

export type MeterType = 'electricity' | 'gas' | 'water' | 'heat' | 'cold_water' | 'hot_water';

export type MeterUnit = 'kWh' | 'm3' | 'GJ' | 'MWh';

export type ReadingStatus = 'pending' | 'validated' | 'rejected' | 'corrected';

export interface Meter {
  id: string;
  organizationId: string;
  buildingId: string;
  unitId?: string;
  meterType: MeterType;
  serialNumber: string;
  unit: MeterUnit;
  location?: string;
  installationDate?: string;
  lastReadingValue?: number;
  lastReadingDate?: string;
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
  // Joined fields
  buildingName?: string;
  unitDesignation?: string;
}

export interface MeterReading {
  id: string;
  meterId: string;
  organizationId: string;
  value: number;
  readingDate: string;
  photoUrl?: string;
  status: ReadingStatus;
  submittedById: string;
  submittedAt: string;
  validatedById?: string;
  validatedAt?: string;
  rejectionReason?: string;
  correctedValue?: number;
  notes?: string;
  createdAt: string;
  updatedAt: string;
  // Joined fields
  submittedByName?: string;
  validatedByName?: string;
  meterSerialNumber?: string;
  meterType?: MeterType;
  meterUnit?: MeterUnit;
}

export interface ConsumptionDataPoint {
  date: string;
  value: number;
  label?: string;
}

export interface ConsumptionHistory {
  meterId: string;
  meterType: MeterType;
  unit: MeterUnit;
  data: ConsumptionDataPoint[];
  totalConsumption: number;
  averageConsumption: number;
  trend?: 'up' | 'down' | 'stable';
  changePercentage?: number;
}

export interface ValidationResult {
  readingId: string;
  status: 'validated' | 'rejected' | 'corrected';
  correctedValue?: number;
  rejectionReason?: string;
  notes?: string;
}

export interface ReadingFormData {
  meterId: string;
  value: number;
  readingDate: string;
  photo?: File;
  notes?: string;
}

export interface EditReadingFormData extends ReadingFormData {
  readingId: string;
  reason?: string;
  previousValue?: number;
}

export interface ReadingChartData {
  meterId: string;
  meterName: string;
  meterType: MeterType;
  unit: MeterUnit;
  dataPoints: {
    date: string;
    value: number;
    consumption?: number;
  }[];
  color?: string;
}

export interface ComparisonParams {
  meterIds: string[];
  startDate: string;
  endDate: string;
  groupBy?: 'day' | 'week' | 'month';
}

export type ExportFormat = 'csv' | 'excel';

export interface ExportOptions {
  format: ExportFormat;
  startDate: string;
  endDate: string;
  meterIds?: string[];
  includeConsumption?: boolean;
}
