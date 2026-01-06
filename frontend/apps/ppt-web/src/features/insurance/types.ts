/**
 * Insurance feature types.
 * Defines TypeScript interfaces for insurance policies and claims.
 */

export type PolicyType =
  | 'building'
  | 'liability'
  | 'property'
  | 'flood'
  | 'earthquake'
  | 'umbrella'
  | 'directors_officers'
  | 'workers_comp'
  | 'other';

export type PolicyStatus = 'active' | 'expired' | 'cancelled' | 'pending';

export type ClaimStatus =
  | 'submitted'
  | 'under_review'
  | 'approved'
  | 'denied'
  | 'settled'
  | 'closed';

export type ClaimType =
  | 'property_damage'
  | 'liability'
  | 'theft'
  | 'water_damage'
  | 'fire_damage'
  | 'natural_disaster'
  | 'personal_injury'
  | 'other';

export interface InsurancePolicy {
  id: string;
  organizationId: string;
  buildingId?: string;
  policyNumber: string;
  policyType: PolicyType;
  provider: string;
  providerContact?: string;
  providerPhone?: string;
  providerEmail?: string;
  coverageAmount: number;
  deductible: number;
  premium: number;
  premiumFrequency: 'monthly' | 'quarterly' | 'annually';
  currency: string;
  startDate: string;
  endDate: string;
  status: PolicyStatus;
  description?: string;
  coverageDetails?: string;
  documentIds?: string[];
  createdAt: string;
  updatedAt: string;
  // Joined fields
  buildingName?: string;
  buildingAddress?: string;
}

export interface InsuranceClaim {
  id: string;
  organizationId: string;
  policyId: string;
  buildingId?: string;
  unitId?: string;
  claimNumber?: string;
  claimType: ClaimType;
  status: ClaimStatus;
  title: string;
  description: string;
  incidentDate: string;
  reportedDate: string;
  claimAmount?: number;
  approvedAmount?: number;
  currency: string;
  filedBy: string;
  assignedAdjuster?: string;
  adjusterContact?: string;
  notes?: string;
  documentIds?: string[];
  createdAt: string;
  updatedAt: string;
  // Joined fields
  policyNumber?: string;
  policyProvider?: string;
  buildingName?: string;
  unitDesignation?: string;
  filedByName?: string;
}

export interface InsuranceReminder {
  id: string;
  policyId: string;
  reminderDate: string;
  reminderType: 'renewal' | 'payment' | 'review' | 'custom';
  message: string;
  isCompleted: boolean;
  completedAt?: string;
  createdAt: string;
}

export interface InsuranceStatistics {
  totalPolicies: number;
  activePolicies: number;
  expiringPolicies: number;
  totalCoverage: number;
  totalPremiums: number;
  totalClaims: number;
  pendingClaims: number;
  approvedClaimsAmount: number;
  currency: string;
}

export interface PolicyFormData {
  buildingId?: string;
  policyNumber: string;
  policyType: PolicyType;
  provider: string;
  providerContact?: string;
  providerPhone?: string;
  providerEmail?: string;
  coverageAmount: number;
  deductible: number;
  premium: number;
  premiumFrequency: 'monthly' | 'quarterly' | 'annually';
  currency: string;
  startDate: string;
  endDate: string;
  description?: string;
  coverageDetails?: string;
}

export interface ClaimFormData {
  policyId: string;
  buildingId?: string;
  unitId?: string;
  claimType: ClaimType;
  title: string;
  description: string;
  incidentDate: string;
  claimAmount?: number;
}

// Quote comparison types (UC-35)

export type CoverageType = 'building' | 'contents' | 'liability' | 'comprehensive';

export type PropertyType = 'apartment' | 'house' | 'commercial' | 'mixed_use';

export interface QuoteProvider {
  id: string;
  name: string;
  logoUrl?: string;
  rating?: number;
  reviewCount?: number;
}

export interface InsuranceQuote {
  id: string;
  provider: QuoteProvider;
  coverageType: CoverageType;
  coverageAmount: number;
  deductible: number;
  premiumMonthly: number;
  premiumAnnual: number;
  currency: string;
  features: string[];
  exclusions: string[];
  coverageLimits: {
    label: string;
    value: string;
  }[];
  isBestValue?: boolean;
  isLowestPrice?: boolean;
  validUntil: string;
  createdAt: string;
}

export interface QuoteRequest {
  coverageType: CoverageType;
  coverageAmount: number;
  propertyType: PropertyType;
  propertySize: number; // in square meters/feet
  startDate: string;
  buildingId?: string;
}

export interface QuoteComparison {
  id: string;
  organizationId: string;
  request: QuoteRequest;
  quotes: InsuranceQuote[];
  savedAt?: string;
  createdAt: string;
}
