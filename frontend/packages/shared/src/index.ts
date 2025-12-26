/**
 * Shared utilities and hooks for Property Management apps.
 */

// Re-export API client
export * from '@ppt/api-client';

// Accessibility
export * from './accessibility';

// Shared types
export interface TenantContext {
  tenantId: string;
  tenantName: string;
  role: string;
}

export interface User {
  id: string;
  email: string;
  displayName: string;
  avatarUrl?: string;
}

// Shared constants
export const API_VERSION = 'v1';

// Shared utilities
export function formatCurrency(amount: number, currency: string): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency,
  }).format(amount / 100);
}

export function formatDate(date: string | Date): string {
  return new Intl.DateTimeFormat('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  }).format(new Date(date));
}
