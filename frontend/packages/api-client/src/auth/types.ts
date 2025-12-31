/**
 * Authentication Types
 *
 * Type definitions for the Authentication API (UC-14).
 */

/** User information returned from authentication */
export interface AuthUser {
  id: string;
  email: string;
  firstName?: string;
  lastName?: string;
  role?: string;
  organizationId?: string;
  organizationName?: string;
}

/** Login request credentials */
export interface LoginRequest {
  email: string;
  password: string;
}

/** Login response */
export interface LoginResponse {
  accessToken: string;
  refreshToken: string;
  user: AuthUser;
}

/** Token refresh request */
export interface RefreshTokenRequest {
  refreshToken: string;
}

/** Token refresh response */
export interface RefreshTokenResponse {
  accessToken: string;
  refreshToken: string;
}

/** Logout request */
export interface LogoutRequest {
  refreshToken: string;
}

/** Registration request */
export interface RegisterRequest {
  email: string;
  password: string;
  firstName: string;
  lastName: string;
  organizationId?: string;
}

/** Registration response */
export interface RegisterResponse {
  id: string;
  email: string;
  firstName: string;
  lastName: string;
  message: string;
}

/** Password reset request */
export interface RequestPasswordResetRequest {
  email: string;
}

/** Password reset confirmation */
export interface ResetPasswordRequest {
  token: string;
  newPassword: string;
}

/** Change password request (authenticated) */
export interface ChangePasswordRequest {
  currentPassword: string;
  newPassword: string;
}

/** Auth error codes */
export type AuthErrorCode =
  | 'INVALID_CREDENTIALS'
  | 'ACCOUNT_LOCKED'
  | 'ACCOUNT_DISABLED'
  | 'SESSION_EXPIRED'
  | 'TOKEN_INVALID'
  | 'TOKEN_EXPIRED'
  | 'EMAIL_NOT_VERIFIED'
  | 'EMAIL_ALREADY_EXISTS'
  | 'WEAK_PASSWORD'
  | 'NETWORK_ERROR'
  | 'UNKNOWN_ERROR';

/** Auth error response from API */
export interface AuthErrorResponse {
  code: AuthErrorCode;
  message: string;
  details?: Record<string, unknown>;
}
