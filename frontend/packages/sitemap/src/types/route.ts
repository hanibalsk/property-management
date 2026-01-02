import type { AuthRequirement } from './auth';

/**
 * Represents an application (frontend or backend)
 */
export type Application =
  | 'ppt-web'
  | 'reality-web'
  | 'mobile'
  | 'api-server'
  | 'reality-server';

/**
 * Frontend application types
 */
export type FrontendApp = 'ppt-web' | 'reality-web' | 'mobile';

/**
 * Backend server types
 */
export type BackendServer = 'api-server' | 'reality-server';

/**
 * Route parameter definition
 */
export interface RouteParam {
  name: string;
  type: 'string' | 'number' | 'uuid';
  description?: string;
  required: boolean;
  example?: string;
}

/**
 * Query parameter definition
 */
export interface QueryParam extends RouteParam {
  defaultValue?: string | number | boolean;
}

/**
 * Frontend route definition
 */
export interface FrontendRoute {
  /** Unique identifier for the route */
  id: string;
  /** Application this route belongs to */
  app: 'ppt-web' | 'reality-web';
  /** Path pattern (e.g., '/documents/:documentId') */
  path: string;
  /** Human-readable name */
  name: string;
  /** Brief description */
  description?: string;
  /** Route parameters */
  params?: RouteParam[];
  /** Query parameters */
  queryParams?: QueryParam[];
  /** Authentication requirements */
  auth: AuthRequirement;
  /** Linked API endpoints (operation IDs) */
  apiEndpoints?: string[];
  /** React component name */
  component?: string;
  /** Parent route ID for nested routes */
  parentId?: string;
  /** Feature/Epic this route belongs to */
  feature?: string;
  /** Tags for categorization */
  tags?: string[];
}

/**
 * Mobile screen definition
 */
export interface MobileScreen {
  /** Unique identifier for the screen */
  id: string;
  /** Application (always 'mobile') */
  app: 'mobile';
  /** Screen name (used in navigation) */
  screenName: string;
  /** Human-readable name */
  name: string;
  /** Brief description */
  description?: string;
  /** Navigation tab this screen belongs to */
  tab?: string;
  /** Tab icon */
  tabIcon?: string;
  /** Authentication requirements */
  auth: AuthRequirement;
  /** Linked API endpoints (operation IDs) */
  apiEndpoints?: string[];
  /** React Native component name */
  component?: string;
  /** Navigation stack this screen belongs to */
  stack?: string;
  /** Feature/Epic this screen belongs to */
  feature?: string;
  /** Tags for categorization */
  tags?: string[];
}

/**
 * Union type for any route or screen
 */
export type Route = FrontendRoute | MobileScreen;
