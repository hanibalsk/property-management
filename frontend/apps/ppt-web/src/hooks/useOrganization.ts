/**
 * Organization context hook for ppt-web.
 *
 * Provides the current organization ID from auth context.
 * Throws an error if used outside of an authenticated context.
 */

import { createContext, useContext } from 'react';

export interface OrganizationContextValue {
  organizationId: string;
  organizationName?: string;
}

export const OrganizationContext = createContext<OrganizationContextValue | null>(null);

/**
 * Hook to access the current organization context.
 *
 * @throws Error if used outside of OrganizationProvider
 * @returns The current organization context with organizationId
 */
export function useOrganization(): OrganizationContextValue {
  const context = useContext(OrganizationContext);

  if (!context) {
    throw new Error(
      'useOrganization must be used within an OrganizationProvider. ' +
        'Ensure the component is wrapped in an authenticated context.'
    );
  }

  return context;
}
