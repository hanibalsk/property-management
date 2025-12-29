/**
 * Authentication Token Provider
 *
 * Provides a secure way to access authentication tokens for API requests.
 * This module avoids direct localStorage access from API modules, enabling:
 *
 * 1. Integration with React's AuthContext for client-side token management
 * 2. Future migration to httpOnly cookies for enhanced XSS protection
 * 3. Centralized token management across all API modules
 *
 * SECURITY NOTE:
 * - Direct localStorage access is XSS-vulnerable and should be avoided
 * - This provider abstracts token storage, allowing secure implementations
 * - For production, consider httpOnly cookies set by the backend
 *
 * @see AuthContext in ppt-web for the primary token source
 */

/**
 * Token provider function type.
 * Returns the current access token or null if not authenticated.
 */
export type TokenProvider = () => string | null;

/**
 * Global token provider instance.
 * Must be initialized by the application before making authenticated API calls.
 *
 * @example
 * ```tsx
 * // In your app initialization (e.g., App.tsx):
 * import { setTokenProvider } from '@ppt/api-client';
 * import { useAuth } from './contexts/AuthContext';
 *
 * function AppWithAuth() {
 *   const { getAccessToken } = useAuth();
 *
 *   useEffect(() => {
 *     setTokenProvider(getAccessToken);
 *   }, [getAccessToken]);
 *
 *   return <App />;
 * }
 * ```
 */
let globalTokenProvider: TokenProvider | null = null;

/**
 * Set the global token provider function.
 * This should be called once during app initialization with a function
 * that returns the current access token from AuthContext.
 *
 * @param provider - Function that returns the current access token
 */
export function setTokenProvider(provider: TokenProvider): void {
  globalTokenProvider = provider;
}

/**
 * Clear the global token provider.
 * Call this on logout or when the auth context is unmounted.
 */
export function clearTokenProvider(): void {
  globalTokenProvider = null;
}

/**
 * Get the current access token using the configured provider.
 * Returns null if no provider is configured or if the user is not authenticated.
 *
 * @returns The current access token or null
 */
export function getToken(): string | null {
  if (!globalTokenProvider) {
    // No provider configured - this is expected during initial app load
    // or in non-authenticated contexts
    return null;
  }
  return globalTokenProvider();
}

/**
 * Check if a token provider is configured.
 * Useful for debugging and conditional logic.
 *
 * @returns True if a token provider has been set
 */
export function hasTokenProvider(): boolean {
  return globalTokenProvider !== null;
}
