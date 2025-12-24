/**
 * Deep link handling for QR codes and external links.
 *
 * Epic 49 - Story 49.3: QR Code Scanning
 */
import { Linking } from 'react-native';

/**
 * Deep link route configuration.
 */
export interface DeepLinkRoute {
  /** Screen name in the app */
  screen: string;
  /** Path pattern (e.g., 'faults/:id') */
  pattern: string;
  /** Parameter names */
  params?: string[];
  /** Whether authentication is required */
  requiresAuth: boolean;
}

/**
 * Registered deep link routes.
 */
const DEEP_LINK_ROUTES: DeepLinkRoute[] = [
  { screen: 'Dashboard', pattern: 'dashboard', requiresAuth: true },
  { screen: 'Faults', pattern: 'faults', requiresAuth: true },
  { screen: 'Faults', pattern: 'faults/:id', params: ['id'], requiresAuth: true },
  { screen: 'ReportFault', pattern: 'fault/report', requiresAuth: true },
  { screen: 'Announcements', pattern: 'announcements', requiresAuth: true },
  { screen: 'Announcements', pattern: 'announcements/:id', params: ['id'], requiresAuth: true },
  { screen: 'Voting', pattern: 'voting', requiresAuth: true },
  { screen: 'Voting', pattern: 'voting/:id', params: ['id'], requiresAuth: true },
  { screen: 'Documents', pattern: 'documents', requiresAuth: true },
  { screen: 'Documents', pattern: 'documents/:id', params: ['id'], requiresAuth: true },
  { screen: 'Settings', pattern: 'settings', requiresAuth: true },
  { screen: 'WidgetSettings', pattern: 'settings/widgets', requiresAuth: true },
];

/**
 * Parsed deep link result.
 */
export interface ParsedDeepLink {
  /** Whether the link was successfully parsed */
  success: boolean;
  /** Target screen */
  screen?: string;
  /** Route parameters */
  params?: Record<string, string>;
  /** Query parameters */
  query?: Record<string, string>;
  /** Whether auth is required */
  requiresAuth?: boolean;
  /** Error message if parsing failed */
  error?: string;
}

/**
 * Parse a deep link URL.
 */
export function parseDeepLink(url: string): ParsedDeepLink {
  try {
    if (!url.startsWith('ppt://')) {
      return { success: false, error: 'Not a PPT deep link' };
    }

    const parsed = new URL(url);
    const pathParts = parsed.pathname.replace(/^\//, '').split('/').filter(Boolean);
    const path = pathParts.join('/');

    // Find matching route
    for (const route of DEEP_LINK_ROUTES) {
      const match = matchRoute(path, route.pattern, route.params);
      if (match) {
        // Extract query parameters
        const query: Record<string, string> = {};
        parsed.searchParams.forEach((value, key) => {
          query[key] = value;
        });

        return {
          success: true,
          screen: route.screen,
          params: match,
          query: Object.keys(query).length > 0 ? query : undefined,
          requiresAuth: route.requiresAuth,
        };
      }
    }

    return { success: false, error: `Unknown route: ${path}` };
  } catch {
    return { success: false, error: 'Invalid URL format' };
  }
}

/**
 * Match a path against a route pattern.
 */
function matchRoute(
  path: string,
  pattern: string,
  paramNames?: string[]
): Record<string, string> | null {
  const pathParts = path.split('/');
  const patternParts = pattern.split('/');

  if (pathParts.length !== patternParts.length) {
    return null;
  }

  const params: Record<string, string> = {};
  let paramIndex = 0;

  for (let i = 0; i < patternParts.length; i++) {
    const patternPart = patternParts[i];
    const pathPart = pathParts[i];

    if (patternPart.startsWith(':')) {
      // Parameter
      const paramName = paramNames?.[paramIndex] ?? patternPart.substring(1);
      params[paramName] = pathPart;
      paramIndex++;
    } else if (patternPart !== pathPart) {
      return null;
    }
  }

  return params;
}

/**
 * Create a deep link URL.
 */
export function createDeepLink(
  screen: string,
  params?: Record<string, string>,
  query?: Record<string, string>
): string {
  // Find route for screen
  const route = DEEP_LINK_ROUTES.find((r) => r.screen === screen);

  if (!route) {
    // Default to simple path
    let url = `ppt://${screen.toLowerCase()}`;
    if (query && Object.keys(query).length > 0) {
      url += `?${new URLSearchParams(query).toString()}`;
    }
    return url;
  }

  // Build path with parameters
  let path = route.pattern;
  if (params && route.params) {
    for (const paramName of route.params) {
      if (params[paramName]) {
        path = path.replace(`:${paramName}`, params[paramName]);
      }
    }
  }

  let url = `ppt://${path}`;
  if (query && Object.keys(query).length > 0) {
    url += `?${new URLSearchParams(query).toString()}`;
  }

  return url;
}

/**
 * Handle an incoming deep link.
 */
export type DeepLinkHandler = (link: ParsedDeepLink) => void;

/**
 * Deep link listener manager.
 */
export class DeepLinkManager {
  private handlers: Set<DeepLinkHandler> = new Set();
  private pendingLink: ParsedDeepLink | null = null;
  private isAuthenticated = false;

  /**
   * Initialize deep link handling.
   */
  async initialize(): Promise<void> {
    // Handle initial URL (app opened via deep link)
    const initialUrl = await Linking.getInitialURL();
    if (initialUrl) {
      this.handleUrl(initialUrl);
    }

    // Listen for incoming deep links
    Linking.addEventListener('url', (event) => {
      this.handleUrl(event.url);
    });
  }

  /**
   * Set authentication state.
   */
  setAuthenticated(isAuthenticated: boolean): void {
    this.isAuthenticated = isAuthenticated;

    // Process pending link if now authenticated
    if (isAuthenticated && this.pendingLink) {
      this.dispatchLink(this.pendingLink);
      this.pendingLink = null;
    }
  }

  /**
   * Register a deep link handler.
   */
  addHandler(handler: DeepLinkHandler): () => void {
    this.handlers.add(handler);
    return () => this.handlers.delete(handler);
  }

  /**
   * Handle a URL.
   */
  private handleUrl(url: string): void {
    const parsed = parseDeepLink(url);

    if (!parsed.success) {
      console.warn('Failed to parse deep link:', parsed.error);
      return;
    }

    if (parsed.requiresAuth && !this.isAuthenticated) {
      // Queue for after authentication
      this.pendingLink = parsed;
      return;
    }

    this.dispatchLink(parsed);
  }

  /**
   * Dispatch a link to handlers.
   */
  private dispatchLink(link: ParsedDeepLink): void {
    for (const handler of this.handlers) {
      handler(link);
    }
  }

  /**
   * Get pending link (for showing after login).
   */
  getPendingLink(): ParsedDeepLink | null {
    return this.pendingLink;
  }

  /**
   * Clear pending link.
   */
  clearPendingLink(): void {
    this.pendingLink = null;
  }
}

/**
 * Singleton instance.
 */
export const deepLinkManager = new DeepLinkManager();
