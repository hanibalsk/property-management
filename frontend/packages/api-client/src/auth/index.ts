/**
 * Authentication Module
 *
 * Provides secure token management for API clients.
 */

export {
  setTokenProvider,
  clearTokenProvider,
  getToken,
  hasTokenProvider,
  type TokenProvider,
} from './token-provider';
