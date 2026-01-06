/**
 * Types for Short-Term Rentals feature.
 * Epic 18: Short-Term Rental Integration (UC-29, UC-30)
 */

export type BookingStatus =
  | 'pending'
  | 'confirmed'
  | 'checked_in'
  | 'checked_out'
  | 'cancelled'
  | 'no_show';

export type BookingSource = 'airbnb' | 'booking' | 'direct' | 'other';

export type PlatformType = 'airbnb' | 'booking';

export type ConnectionStatus = 'connected' | 'disconnected' | 'pending' | 'error';

export type CalendarEventType = 'booking' | 'block' | 'maintenance';

export type GuestRegistrationStatus = 'pending' | 'registered' | 'failed';

export interface RentalBooking {
  id: string;
  unitId: string;
  unitName?: string;
  buildingId: string;
  buildingName?: string;
  guestName: string;
  guestEmail?: string;
  guestPhone?: string;
  checkIn: string;
  checkOut: string;
  status: BookingStatus;
  source: BookingSource;
  platformBookingId?: string;
  totalPrice?: number;
  currency?: string;
  guestCount: number;
  notes?: string;
  createdAt: string;
  updatedAt: string;
}

export interface RentalGuest {
  id: string;
  bookingId: string;
  firstName: string;
  lastName: string;
  email?: string;
  phone?: string;
  dateOfBirth?: string;
  nationality?: string;
  documentType?: string;
  documentNumber?: string;
  documentExpiry?: string;
  address?: string;
  city?: string;
  country?: string;
  registrationStatus: GuestRegistrationStatus;
  registeredAt?: string;
  isPrimary: boolean;
  createdAt: string;
}

export interface CalendarEvent {
  id: string;
  unitId: string;
  type: CalendarEventType;
  title: string;
  startDate: string;
  endDate: string;
  bookingId?: string;
  guestName?: string;
  source?: BookingSource;
  status?: BookingStatus;
  color?: string;
  notes?: string;
}

export interface PlatformConnection {
  id: string;
  unitId: string;
  unitName?: string;
  platform: PlatformType;
  status: ConnectionStatus;
  platformPropertyId?: string;
  lastSyncAt?: string;
  syncErrorMessage?: string;
  isAutoSyncEnabled: boolean;
  createdAt: string;
}

export interface RentalStatistics {
  totalBookings: number;
  activeBookings: number;
  upcomingBookings: number;
  totalRevenue: number;
  occupancyRate: number;
  averageStayDuration: number;
  pendingGuestRegistrations: number;
  currency: string;
}

export interface BookingWithGuests extends RentalBooking {
  guests: RentalGuest[];
}

export interface CreateBookingRequest {
  unitId: string;
  guestName: string;
  guestEmail?: string;
  guestPhone?: string;
  checkIn: string;
  checkOut: string;
  source: BookingSource;
  totalPrice?: number;
  currency?: string;
  guestCount: number;
  notes?: string;
}

export interface CreateGuestRequest {
  bookingId: string;
  firstName: string;
  lastName: string;
  email?: string;
  phone?: string;
  dateOfBirth?: string;
  nationality?: string;
  documentType?: string;
  documentNumber?: string;
  documentExpiry?: string;
  address?: string;
  city?: string;
  country?: string;
  isPrimary: boolean;
}

export interface UpdateGuestRequest {
  firstName?: string;
  lastName?: string;
  email?: string;
  phone?: string;
  dateOfBirth?: string;
  nationality?: string;
  documentType?: string;
  documentNumber?: string;
  documentExpiry?: string;
  address?: string;
  city?: string;
  country?: string;
}

export interface CreatePlatformConnectionRequest {
  unitId: string;
  platform: PlatformType;
  platformPropertyId?: string;
  isAutoSyncEnabled: boolean;
}

export interface CalendarBlockRequest {
  unitId: string;
  startDate: string;
  endDate: string;
  reason?: string;
}

export interface BookingListParams {
  unitId?: string;
  buildingId?: string;
  platform?: BookingSource;
  status?: BookingStatus;
  fromDate?: string;
  toDate?: string;
  page?: number;
  limit?: number;
}

/**
 * Tax Report Types - UC-29 Tax Export Functionality
 */

export type TaxReportType = 'annual_summary' | 'monthly_breakdown' | 'per_booking';

export type TaxExportFormat = 'pdf' | 'csv' | 'excel';

export type TaxJurisdictionCountry = 'SK' | 'CZ' | 'DE' | 'AT' | 'HU' | 'PL';

export interface TaxJurisdiction {
  country: TaxJurisdictionCountry;
  name: string;
  defaultTaxRate: number;
  requiresGuestRegistration: boolean;
  localTaxPerNight?: number;
  localTaxCurrency?: string;
}

export interface TaxReportParams {
  year: number;
  reportType: TaxReportType;
  jurisdiction: TaxJurisdictionCountry;
  unitIds?: string[];
  buildingIds?: string[];
  includeAllProperties: boolean;
}

export interface MonthlyTaxBreakdown {
  month: number;
  monthName: string;
  income: number;
  bookingsCount: number;
  nightsOccupied: number;
  occupancyRate: number;
  expenses: number;
  netProfit: number;
}

export interface BookingTaxDetail {
  bookingId: string;
  guestName: string;
  unitName: string;
  checkIn: string;
  checkOut: string;
  nights: number;
  income: number;
  source: BookingSource;
}

export interface ExpenseCategory {
  category: string;
  amount: number;
  percentage: number;
}

export interface TaxReportData {
  year: number;
  jurisdiction: TaxJurisdiction;
  reportType: TaxReportType;
  generatedAt: string;
  currency: string;
  summary: {
    totalIncome: number;
    totalBookings: number;
    totalNightsOccupied: number;
    averageOccupancyRate: number;
    totalExpenses: number;
    netProfit: number;
    estimatedTax: number;
    effectiveTaxRate: number;
  };
  monthlyBreakdown?: MonthlyTaxBreakdown[];
  bookingDetails?: BookingTaxDetail[];
  expensesByCategory: ExpenseCategory[];
  propertiesCovered: {
    buildingId: string;
    buildingName: string;
    unitId: string;
    unitName: string;
  }[];
}

export interface TaxSummary {
  year: number;
  ytdIncome: number;
  ytdExpenses: number;
  ytdNetProfit: number;
  estimatedTax: number;
  taxRate: number;
  previousYearIncome: number;
  previousYearTax: number;
  incomeChangePercent: number;
  currency: string;
}
