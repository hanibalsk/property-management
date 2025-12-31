/**
 * Authentication Module
 *
 * Provides authentication API and secure token management for API clients.
 */

// Token provider for secure token management
export {
  setTokenProvider,
  clearTokenProvider,
  getToken,
  hasTokenProvider,
  type TokenProvider,
} from './token-provider';

// Auth API client and types
export { createAuthApi, AuthError, type AuthApi } from './api';
export type {
  AuthUser,
  LoginRequest,
  LoginResponse,
  RefreshTokenRequest,
  RefreshTokenResponse,
  LogoutRequest,
  RegisterRequest,
  RegisterResponse,
  RequestPasswordResetRequest,
  ResetPasswordRequest,
  ChangePasswordRequest,
  AuthErrorCode,
  AuthErrorResponse,
} from './types';
