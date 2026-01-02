import { getProtectedRoutes, getPublicRoutes, sitemap } from '../data';
import type {
  ApiEndpoint,
  FrontendRoute,
  MobileScreen,
  TestUser,
  TestUserSet,
  UserRole,
} from '../types';
import { buildUrl } from './route-helpers';

/**
 * Get all protected routes for an app
 */
export function getTestProtectedRoutes(app: 'ppt-web' | 'reality-web'): FrontendRoute[] {
  return getProtectedRoutes(app);
}

/**
 * Get all public routes for an app
 */
export function getTestPublicRoutes(app: 'ppt-web' | 'reality-web'): FrontendRoute[] {
  return getPublicRoutes(app);
}

/**
 * Get routes accessible by a specific role.
 * Note: Returns routes where either:
 * - The route doesn't require authentication (public)
 * - The route requires auth but has no role restrictions (any authenticated user)
 * - The route requires auth and the specified role is in the allowed roles list
 */
export function getRoutesForRole(app: 'ppt-web' | 'reality-web', role: UserRole): FrontendRoute[] {
  return sitemap.routes[app].filter((r) => {
    if (!r.auth.required) return true;
    if (!r.auth.roles || r.auth.roles.length === 0) return true;
    return r.auth.roles.includes(role);
  });
}

/**
 * Get routes NOT accessible by a specific role
 */
export function getRestrictedRoutesForRole(
  app: 'ppt-web' | 'reality-web',
  role: UserRole
): FrontendRoute[] {
  return sitemap.routes[app].filter((r) => {
    if (!r.auth.required) return false;
    if (!r.auth.roles || r.auth.roles.length === 0) return false;
    return !r.auth.roles.includes(role);
  });
}

/**
 * Build URL with sample parameters for testing
 */
export function buildTestUrl(
  route: FrontendRoute | MobileScreen,
  params?: Record<string, string>
): string {
  // Generate sample params if not provided
  const sampleParams: Record<string, string> = {};

  if ('params' in route && route.params) {
    for (const param of route.params) {
      if (!params?.[param.name]) {
        sampleParams[param.name] =
          param.example ||
          (param.type === 'uuid'
            ? '00000000-0000-0000-0000-000000000000'
            : param.type === 'number'
              ? '1'
              : 'test');
      }
    }
  }

  return buildUrl(route, { ...sampleParams, ...params });
}

/**
 * Get API endpoints for a route or mobile screen.
 * Maps the route/screen to the correct backend server:
 * - reality-web routes -> reality-server
 * - ppt-web routes -> api-server
 * - mobile screens -> api-server (mobile app uses api-server)
 */
export function getRouteEndpoints(route: FrontendRoute | MobileScreen): ApiEndpoint[] {
  if (!route.apiEndpoints) return [];

  // Mobile screens and ppt-web use api-server, reality-web uses reality-server
  const server = route.app === 'reality-web' ? 'reality-server' : 'api-server';
  return route.apiEndpoints
    .map((opId) => sitemap.endpoints[server].find((e) => e.operationId === opId))
    .filter((e): e is ApiEndpoint => e !== undefined);
}

/**
 * Generate test cases for route access control
 */
export function generateAccessControlTests(
  app: 'ppt-web' | 'reality-web',
  testUsers: TestUserSet
): Array<{
  route: FrontendRoute;
  user: TestUser | null;
  shouldAllow: boolean;
  reason: string;
}> {
  const testCases: Array<{
    route: FrontendRoute;
    user: TestUser | null;
    shouldAllow: boolean;
    reason: string;
  }> = [];

  for (const route of sitemap.routes[app]) {
    // Test anonymous access
    testCases.push({
      route,
      user: null,
      shouldAllow: !route.auth.required,
      reason: route.auth.required ? 'Route requires authentication' : 'Route is public',
    });

    // Test each role
    for (const [, user] of Object.entries(testUsers)) {
      if (user === null) continue;

      const shouldAllow =
        !route.auth.required ||
        !route.auth.roles ||
        route.auth.roles.length === 0 ||
        route.auth.roles.includes(user.role);

      let reason: string;
      if (!route.auth.required) {
        reason = 'Route is public';
      } else if (!route.auth.roles || route.auth.roles.length === 0) {
        reason = 'Route requires any authenticated user';
      } else if (route.auth.roles.includes(user.role)) {
        reason = `Role ${user.role} is in allowed roles`;
      } else {
        reason = `Role ${user.role} is not in allowed roles: ${route.auth.roles.join(', ')}`;
      }

      testCases.push({ route, user, shouldAllow, reason });
    }
  }

  return testCases;
}

/**
 * Get all routes with a specific tag
 */
export function getRoutesByTestTag(app: 'ppt-web' | 'reality-web', tag: string): FrontendRoute[] {
  return sitemap.routes[app].filter((r) => r.tags?.includes(tag));
}

/**
 * Get all endpoints with a specific tag
 */
export function getEndpointsByTestTag(
  server: 'api-server' | 'reality-server',
  tag: string
): ApiEndpoint[] {
  return sitemap.endpoints[server].filter((e) => e.tags?.includes(tag));
}

/**
 * Validate that all route API endpoints exist
 */
export function validateRouteEndpoints(app: 'ppt-web' | 'reality-web'): {
  valid: boolean;
  missing: Array<{ routeId: string; endpointId: string }>;
} {
  const missing: Array<{ routeId: string; endpointId: string }> = [];
  const server = app === 'reality-web' ? 'reality-server' : 'api-server';

  for (const route of sitemap.routes[app]) {
    if (!route.apiEndpoints) continue;

    for (const endpointId of route.apiEndpoints) {
      const exists = sitemap.endpoints[server].some((e) => e.operationId === endpointId);
      if (!exists) {
        missing.push({ routeId: route.id, endpointId });
      }
    }
  }

  return { valid: missing.length === 0, missing };
}

/**
 * Get mobile screens by tab
 */
export function getScreensByTab(tab: string): MobileScreen[] {
  return sitemap.screens.mobile.filter((s) => s.tab === tab);
}

/**
 * Get all navigation tabs for mobile
 */
export function getMobileTabs(): Array<{ name: string; icon: string; screens: MobileScreen[] }> {
  const tabs = new Map<string, { icon: string; screens: MobileScreen[] }>();

  for (const screen of sitemap.screens.mobile) {
    if (screen.tab) {
      if (!tabs.has(screen.tab)) {
        tabs.set(screen.tab, { icon: screen.tabIcon || '', screens: [] });
      }
      tabs.get(screen.tab)!.screens.push(screen);
    }
  }

  return Array.from(tabs.entries()).map(([name, data]) => ({
    name,
    icon: data.icon,
    screens: data.screens,
  }));
}

/**
 * @deprecated Use individual functions instead
 * Legacy class wrapper for backward compatibility
 */
export const SitemapTestHelper = {
  getProtectedRoutes: getTestProtectedRoutes,
  getPublicRoutes: getTestPublicRoutes,
  getRoutesForRole,
  getRestrictedRoutesForRole,
  buildTestUrl,
  getRouteEndpoints,
  generateAccessControlTests,
  getRoutesByTag: getRoutesByTestTag,
  getEndpointsByTag: getEndpointsByTestTag,
  validateRouteEndpoints,
  getScreensByTab,
  getMobileTabs,
};

/**
 * Create a test user set with sample data.
 *
 * Note: Property names use camelCase (e.g., superAdmin, organizationAdmin)
 * while UserRole values use snake_case (e.g., 'super_admin', 'organization_admin').
 * This is intentional - property names follow TypeScript conventions while
 * UserRole values match the backend API format.
 */
export function createTestUserSet(overrides: Partial<TestUserSet> = {}): TestUserSet {
  return {
    superAdmin: {
      id: 'test-super-admin',
      email: 'superadmin@test.com',
      role: 'super_admin',
    },
    organizationAdmin: {
      id: 'test-org-admin',
      email: 'orgadmin@test.com',
      role: 'organization_admin',
      organizationId: 'test-org-1',
    },
    manager: {
      id: 'test-manager',
      email: 'manager@test.com',
      role: 'manager',
      organizationId: 'test-org-1',
    },
    owner: {
      id: 'test-owner',
      email: 'owner@test.com',
      role: 'owner',
      organizationId: 'test-org-1',
    },
    tenant: {
      id: 'test-tenant',
      email: 'tenant@test.com',
      role: 'tenant',
      organizationId: 'test-org-1',
    },
    anonymous: null,
    ...overrides,
  };
}
