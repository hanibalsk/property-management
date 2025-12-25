/**
 * API Client for Property Management System
 *
 * Generated from OpenAPI specification.
 * Run `pnpm generate` to regenerate after API changes.
 */

// Export generated types and client
// These will be populated after running `pnpm generate`
export * from './generated';

// Export domain-specific modules
export * from './advanced-notifications';
export * from './announcements';
export * from './critical-notifications';
export * from './documents';
export * from './government-portal';
export * from './messaging';
export * from './neighbors';
export * from './notification-preferences';
export * from './community';
export * from './financial';
export * from './forms';
export * from './workflow-automation';
export * from './reports';
export * from './facilities';
export * from './registry';
export * from './packages';

// API client configuration
export interface ApiConfig {
  baseUrl: string;
  accessToken?: string;
  tenantId?: string;
}

// Create configured API client
export function createApiClient(config: ApiConfig) {
  return {
    baseUrl: config.baseUrl,
    headers: {
      ...(config.accessToken && { Authorization: `Bearer ${config.accessToken}` }),
      ...(config.tenantId && { 'X-Tenant-ID': config.tenantId }),
    },
  };
}
