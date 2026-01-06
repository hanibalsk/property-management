/**
 * Subscription feature types.
 * These types mirror the backend subscriptions.rs API models.
 */

// Plan types
export type PlanInterval = 'monthly' | 'yearly';
export type PlanStatus = 'active' | 'inactive' | 'deprecated';

export interface PlanFeature {
  key: string;
  name: string;
  included: boolean;
  limit?: number;
}

export interface Plan {
  id: string;
  name: string;
  description: string;
  price: number;
  currency: string;
  interval: PlanInterval;
  features: PlanFeature[];
  maxUsers?: number;
  maxBuildings?: number;
  maxUnits?: number;
  status: PlanStatus;
  trialDays?: number;
  createdAt: string;
  updatedAt: string;
}

// Subscription types
export type SubscriptionStatus =
  | 'active'
  | 'trialing'
  | 'past_due'
  | 'cancelled'
  | 'expired'
  | 'suspended';

export interface Subscription {
  id: string;
  organizationId: string;
  planId: string;
  plan?: Plan;
  status: SubscriptionStatus;
  currentPeriodStart: string;
  currentPeriodEnd: string;
  cancelAtPeriodEnd: boolean;
  trialStart?: string;
  trialEnd?: string;
  cancelledAt?: string;
  createdAt: string;
  updatedAt: string;
}

// Invoice types
export type InvoiceStatus = 'draft' | 'open' | 'paid' | 'void' | 'uncollectible';

export interface InvoiceLineItem {
  id: string;
  description: string;
  quantity: number;
  unitPrice: number;
  amount: number;
}

export interface Invoice {
  id: string;
  subscriptionId: string;
  invoiceNumber: string;
  status: InvoiceStatus;
  currency: string;
  subtotal: number;
  tax: number;
  total: number;
  amountPaid: number;
  amountDue: number;
  lineItems: InvoiceLineItem[];
  periodStart: string;
  periodEnd: string;
  dueDate: string;
  paidAt?: string;
  hostedInvoiceUrl?: string;
  pdfUrl?: string;
  createdAt: string;
}

// Payment Method types
export type PaymentMethodType = 'card' | 'bank_account' | 'sepa_debit';

export interface CardDetails {
  brand: string;
  last4: string;
  expMonth: number;
  expYear: number;
}

export interface BankAccountDetails {
  bankName: string;
  last4: string;
  accountType: 'checking' | 'savings';
}

export interface PaymentMethod {
  id: string;
  type: PaymentMethodType;
  isDefault: boolean;
  card?: CardDetails;
  bankAccount?: BankAccountDetails;
  billingName?: string;
  billingEmail?: string;
  createdAt: string;
}

// Coupon types
export type CouponDuration = 'once' | 'repeating' | 'forever';

export interface Coupon {
  id: string;
  code: string;
  name: string;
  percentOff?: number;
  amountOff?: number;
  currency?: string;
  duration: CouponDuration;
  durationInMonths?: number;
  maxRedemptions?: number;
  timesRedeemed: number;
  validUntil?: string;
  isActive: boolean;
  createdAt: string;
}

// Usage types
export type UsageMetricType = 'users' | 'buildings' | 'units' | 'storage' | 'api_calls';

export interface UsageRecord {
  id: string;
  subscriptionId: string;
  metricType: UsageMetricType;
  quantity: number;
  timestamp: string;
}

export interface UsageSummary {
  metricType: UsageMetricType;
  currentUsage: number;
  limit?: number;
  periodStart: string;
  periodEnd: string;
}

// Statistics types
export interface SubscriptionStatistics {
  totalSubscriptions: number;
  activeSubscriptions: number;
  trialSubscriptions: number;
  cancelledSubscriptions: number;
  mrr: number; // Monthly Recurring Revenue
  arr: number; // Annual Recurring Revenue
  currency: string;
  churnRate: number;
  averageRevenuePerUser: number;
}

// Discount Code types
export type DiscountType = 'percentage' | 'fixed_amount';

export interface DiscountCode {
  id: string;
  code: string;
  discountType: DiscountType;
  discountValue: number;
  currency?: string;
  description?: string;
  minPurchaseAmount?: number;
  maxUses?: number;
  currentUses: number;
  validFrom?: string;
  validUntil?: string;
  isActive: boolean;
  applicablePlans?: string[];
  createdAt: string;
}

export interface AppliedDiscount {
  code: string;
  discountType: DiscountType;
  discountValue: number;
  originalPrice: number;
  discountedPrice: number;
  savings: number;
  currency: string;
}

// Trial Status types
export type TrialStatusType = 'not_started' | 'active' | 'expired' | 'converted';

export interface TrialStatus {
  status: TrialStatusType;
  trialDays: number;
  daysRemaining?: number;
  trialStartDate?: string;
  trialEndDate?: string;
  canExtend: boolean;
  extensionDays?: number;
  requiresCreditCard: boolean;
}

export interface TrialEligibility {
  eligible: boolean;
  reason?: string;
  trialDays: number;
  requiresCreditCard: boolean;
  planId: string;
}
