/**
 * User roles from the property management system
 */
export type UserRole =
  | 'super_admin'
  | 'organization_admin'
  | 'manager'
  | 'technical_manager'
  | 'owner'
  | 'owner_delegate'
  | 'tenant'
  | 'resident'
  | 'property_manager'
  | 'guest'
  | 'real_estate_agent'
  | 'portal_user'
  | 'anonymous';

/**
 * Authentication requirement for a route or endpoint
 */
export interface AuthRequirement {
  /** Whether authentication is required */
  required: boolean;
  /** Allowed roles (empty means any authenticated user) */
  roles?: UserRole[];
  /** Required scopes (OAuth) */
  scopes?: string[];
  /** Whether multi-factor authentication is required */
  mfaRequired?: boolean;
  /** Tenant context requirements */
  tenantContext?: {
    required: boolean;
    headerName?: string;
  };
  /** Special permissions needed */
  permissions?: string[];
}

/**
 * Test user configuration for auth testing
 */
export interface TestUser {
  id: string;
  email: string;
  role: UserRole;
  organizationId?: string;
  permissions?: string[];
  /** JWT access token (for test purposes) */
  accessToken?: string;
}

/**
 * Predefined test users for different roles
 */
export interface TestUserSet {
  superAdmin: TestUser;
  organizationAdmin: TestUser;
  manager: TestUser;
  owner: TestUser;
  tenant: TestUser;
  anonymous: null;
}
