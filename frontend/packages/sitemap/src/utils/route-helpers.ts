import type { FrontendRoute, MobileScreen, RouteParam } from '../types';

/**
 * Build a URL from a route definition and parameters
 */
export function buildUrl(
  route: FrontendRoute | MobileScreen,
  params: Record<string, string> = {},
  queryParams: Record<string, string | number | boolean> = {}
): string {
  // Mobile screens don't have paths
  if ('screenName' in route) {
    return route.screenName;
  }

  let url = route.path;

  // Replace path parameters
  for (const [key, value] of Object.entries(params)) {
    url = url.replace(`:${key}`, encodeURIComponent(value));
    url = url.replace(`[${key}]`, encodeURIComponent(value)); // Next.js style
  }

  // Add query parameters
  const queryString = Object.entries(queryParams)
    .filter(([, value]) => value !== undefined && value !== null && value !== '')
    .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(String(value))}`)
    .join('&');

  if (queryString) {
    url += `?${queryString}`;
  }

  return url;
}

/**
 * Extract parameters from a URL path
 */
export function extractParams(
  route: FrontendRoute,
  actualPath: string
): Record<string, string> | null {
  const params: Record<string, string> = {};
  const routeParts = route.path.split('/');
  const actualParts = actualPath.split('?')[0].split('/');

  if (routeParts.length !== actualParts.length) {
    return null;
  }

  for (let i = 0; i < routeParts.length; i++) {
    const routePart = routeParts[i];
    const actualPart = actualParts[i];

    if (routePart.startsWith(':')) {
      // URL param (Express style)
      const paramName = routePart.slice(1);
      params[paramName] = decodeURIComponent(actualPart);
    } else if (routePart.startsWith('[') && routePart.endsWith(']')) {
      // URL param (Next.js style)
      const paramName = routePart.slice(1, -1);
      params[paramName] = decodeURIComponent(actualPart);
    } else if (routePart !== actualPart) {
      return null;
    }
  }

  return params;
}

/**
 * Check if a path matches a route pattern
 */
export function matchesRoute(route: FrontendRoute, path: string): boolean {
  return extractParams(route, path) !== null;
}

/**
 * Validate route parameters against their definitions
 */
export function validateParams(
  routeParams: RouteParam[] | undefined,
  params: Record<string, string>
): { valid: boolean; errors: string[] } {
  const errors: string[] = [];

  if (!routeParams) {
    return { valid: true, errors: [] };
  }

  for (const paramDef of routeParams) {
    const value = params[paramDef.name];

    if (paramDef.required && (value === undefined || value === '')) {
      errors.push(`Missing required parameter: ${paramDef.name}`);
      continue;
    }

    if (value !== undefined && value !== '') {
      switch (paramDef.type) {
        case 'uuid':
          if (!/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(value)) {
            errors.push(`Invalid UUID for parameter: ${paramDef.name}`);
          }
          break;
        case 'number':
          if (isNaN(Number(value))) {
            errors.push(`Invalid number for parameter: ${paramDef.name}`);
          }
          break;
      }
    }
  }

  return { valid: errors.length === 0, errors };
}

/**
 * Get the full path including parent routes
 */
export function getFullPath(
  route: FrontendRoute,
  allRoutes: FrontendRoute[]
): string {
  if (!route.parentId) {
    return route.path;
  }

  const parent = allRoutes.find((r) => r.id === route.parentId);
  if (!parent) {
    return route.path;
  }

  // The child path should already be full, just return it
  return route.path;
}

/**
 * Get all child routes of a parent route
 */
export function getChildRoutes(
  parentId: string,
  allRoutes: FrontendRoute[]
): FrontendRoute[] {
  return allRoutes.filter((r) => r.parentId === parentId);
}

/**
 * Get the route hierarchy (path from root to this route)
 */
export function getRouteHierarchy(
  route: FrontendRoute,
  allRoutes: FrontendRoute[]
): FrontendRoute[] {
  const hierarchy: FrontendRoute[] = [route];

  let current = route;
  while (current.parentId) {
    const parent = allRoutes.find((r) => r.id === current.parentId);
    if (parent) {
      hierarchy.unshift(parent);
      current = parent;
    } else {
      break;
    }
  }

  return hierarchy;
}
