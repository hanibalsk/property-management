/**
 * Authentication Context for ppt-web.
 *
 * Provides authentication state and methods throughout the application.
 * Handles login, logout, token refresh, and session management.
 *
 * @see Story 79.2 - Authentication Flow Implementation
 */

import type React from 'react';
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
} from 'react';

// ============================================================================
// Types
// ============================================================================

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

/** Authentication state */
export interface AuthState {
  /** Currently authenticated user, null if not authenticated */
  user: AuthUser | null;
  /** Whether the user is authenticated */
  isAuthenticated: boolean;
  /** Whether authentication state is being loaded/checked */
  isLoading: boolean;
}

/** Login credentials */
export interface LoginCredentials {
  email: string;
  password: string;
}

/** Login response from the API */
interface LoginResponse {
  accessToken: string;
  refreshToken: string;
  user: AuthUser;
}

/** Token refresh response from the API */
interface RefreshResponse {
  accessToken: string;
  refreshToken: string;
}

/** Authentication context value */
export interface AuthContextValue extends AuthState {
  /** Log in with email and password */
  login: (credentials: LoginCredentials) => Promise<void>;
  /** Log out the current user */
  logout: () => Promise<void>;
  /** Refresh the access token */
  refreshToken: () => Promise<string | null>;
  /** Get the current access token */
  getAccessToken: () => string | null;
}

/** Auth provider error types */
export type AuthErrorCode =
  | 'INVALID_CREDENTIALS'
  | 'ACCOUNT_LOCKED'
  | 'ACCOUNT_DISABLED'
  | 'SESSION_EXPIRED'
  | 'NETWORK_ERROR'
  | 'UNKNOWN_ERROR';

export class AuthError extends Error {
  constructor(
    message: string,
    public readonly code: AuthErrorCode
  ) {
    super(message);
    this.name = 'AuthError';
  }
}

// ============================================================================
// Constants
// ============================================================================

const API_BASE_URL = '/api/v1';
const ACCESS_TOKEN_KEY = 'ppt_access_token';
const REFRESH_TOKEN_KEY = 'ppt_refresh_token';
const USER_KEY = 'ppt_user';

// ============================================================================
// Context
// ============================================================================

const AuthContext = createContext<AuthContextValue | null>(null);

// ============================================================================
// Token Storage (localStorage for MVP, httpOnly cookies later)
// ============================================================================

const tokenStorage = {
  getAccessToken: (): string | null => {
    try {
      return localStorage.getItem(ACCESS_TOKEN_KEY);
    } catch {
      return null;
    }
  },

  setAccessToken: (token: string): void => {
    try {
      localStorage.setItem(ACCESS_TOKEN_KEY, token);
    } catch {
      // Storage unavailable
    }
  },

  getRefreshToken: (): string | null => {
    try {
      return localStorage.getItem(REFRESH_TOKEN_KEY);
    } catch {
      return null;
    }
  },

  setRefreshToken: (token: string): void => {
    try {
      localStorage.setItem(REFRESH_TOKEN_KEY, token);
    } catch {
      // Storage unavailable
    }
  },

  getUser: (): AuthUser | null => {
    try {
      const userJson = localStorage.getItem(USER_KEY);
      return userJson ? JSON.parse(userJson) : null;
    } catch {
      return null;
    }
  },

  setUser: (user: AuthUser): void => {
    try {
      localStorage.setItem(USER_KEY, JSON.stringify(user));
    } catch {
      // Storage unavailable
    }
  },

  clear: (): void => {
    try {
      localStorage.removeItem(ACCESS_TOKEN_KEY);
      localStorage.removeItem(REFRESH_TOKEN_KEY);
      localStorage.removeItem(USER_KEY);
    } catch {
      // Storage unavailable
    }
  },
};

// ============================================================================
// API Helper
// ============================================================================

async function authFetch<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
  const response = await fetch(`${API_BASE_URL}${endpoint}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options.headers,
    },
  });

  if (!response.ok) {
    const errorData = await response.json().catch(() => ({}));
    const message = errorData.message || 'Authentication request failed';

    // Map HTTP status codes to error codes
    let code: AuthErrorCode = 'UNKNOWN_ERROR';
    if (response.status === 401) {
      code = errorData.code === 'ACCOUNT_LOCKED' ? 'ACCOUNT_LOCKED' : 'INVALID_CREDENTIALS';
    } else if (response.status === 403) {
      code = 'ACCOUNT_DISABLED';
    }

    throw new AuthError(message, code);
  }

  return response.json();
}

// ============================================================================
// Provider Component
// ============================================================================

interface AuthProviderProps {
  children: React.ReactNode;
}

/**
 * Authentication Provider Component.
 *
 * Wraps the application to provide authentication context.
 * Handles token storage, refresh, and session management.
 */
export function AuthProvider({ children }: AuthProviderProps) {
  const [user, setUser] = useState<AuthUser | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  // Track if a token refresh is in progress to prevent concurrent refreshes
  const isRefreshing = useRef(false);
  const refreshPromise = useRef<Promise<string | null> | null>(null);

  // Derived state
  const isAuthenticated = user !== null;

  /**
   * Initialize auth state from storage on mount.
   * Note: We intentionally only run this on mount to prevent loops.
   */
  useEffect(() => {
    const initializeAuth = async () => {
      try {
        const storedUser = tokenStorage.getUser();
        const accessToken = tokenStorage.getAccessToken();
        const refreshTokenValue = tokenStorage.getRefreshToken();

        if (storedUser && accessToken) {
          // Validate the token is not expired (basic check)
          // In production, we might verify with the server
          setUser(storedUser);
        } else if (refreshTokenValue) {
          // Try to refresh the token - inline implementation to avoid dependency
          try {
            const response = await authFetch<RefreshResponse>('/auth/refresh', {
              method: 'POST',
              body: JSON.stringify({ refreshToken: refreshTokenValue }),
            });
            tokenStorage.setAccessToken(response.accessToken);
            tokenStorage.setRefreshToken(response.refreshToken);
          } catch {
            tokenStorage.clear();
          }
        }
      } catch {
        // Clear any invalid stored data
        tokenStorage.clear();
      } finally {
        setIsLoading(false);
      }
    };

    initializeAuth();
  }, []);

  /**
   * Internal token refresh implementation.
   */
  const refreshTokenInternal = useCallback(async (): Promise<string | null> => {
    const refreshTokenValue = tokenStorage.getRefreshToken();
    if (!refreshTokenValue) {
      return null;
    }

    try {
      const response = await authFetch<RefreshResponse>('/auth/refresh', {
        method: 'POST',
        body: JSON.stringify({ refreshToken: refreshTokenValue }),
      });

      tokenStorage.setAccessToken(response.accessToken);
      tokenStorage.setRefreshToken(response.refreshToken);

      return response.accessToken;
    } catch (error) {
      // Refresh failed, clear auth state
      tokenStorage.clear();
      setUser(null);
      throw error;
    }
  }, []);

  /**
   * Refresh the access token with request queuing.
   * Prevents multiple concurrent refresh requests.
   */
  const refreshToken = useCallback(async (): Promise<string | null> => {
    // If already refreshing, return the existing promise
    if (isRefreshing.current && refreshPromise.current) {
      return refreshPromise.current;
    }

    isRefreshing.current = true;
    refreshPromise.current = refreshTokenInternal().finally(() => {
      isRefreshing.current = false;
      refreshPromise.current = null;
    });

    return refreshPromise.current;
  }, [refreshTokenInternal]);

  /**
   * Get the current access token.
   */
  const getAccessToken = useCallback((): string | null => {
    return tokenStorage.getAccessToken();
  }, []);

  /**
   * Log in with email and password.
   */
  const login = useCallback(async (credentials: LoginCredentials): Promise<void> => {
    setIsLoading(true);

    try {
      const response = await authFetch<LoginResponse>('/auth/login', {
        method: 'POST',
        body: JSON.stringify(credentials),
      });

      // Store tokens and user
      tokenStorage.setAccessToken(response.accessToken);
      tokenStorage.setRefreshToken(response.refreshToken);
      tokenStorage.setUser(response.user);

      setUser(response.user);
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Log out the current user.
   */
  const logout = useCallback(async (): Promise<void> => {
    const refreshTokenValue = tokenStorage.getRefreshToken();
    const accessToken = tokenStorage.getAccessToken();

    // Clear local state first for immediate UI feedback
    tokenStorage.clear();
    setUser(null);

    // Attempt to invalidate the refresh token on the server
    if (refreshTokenValue && accessToken) {
      try {
        await authFetch('/auth/logout', {
          method: 'POST',
          headers: {
            Authorization: `Bearer ${accessToken}`,
          },
          body: JSON.stringify({ refreshToken: refreshTokenValue }),
        });
      } catch {
        // Ignore errors - we've already cleared local state
      }
    }
  }, []);

  // Memoize the context value to prevent unnecessary re-renders
  const contextValue = useMemo<AuthContextValue>(
    () => ({
      user,
      isAuthenticated,
      isLoading,
      login,
      logout,
      refreshToken,
      getAccessToken,
    }),
    [user, isAuthenticated, isLoading, login, logout, refreshToken, getAccessToken]
  );

  return <AuthContext.Provider value={contextValue}>{children}</AuthContext.Provider>;
}

AuthProvider.displayName = 'AuthProvider';

// ============================================================================
// Hook
// ============================================================================

/**
 * Hook to access authentication context.
 *
 * @throws Error if used outside of AuthProvider
 * @returns The authentication context value
 *
 * @example
 * ```tsx
 * function MyComponent() {
 *   const { user, isAuthenticated, login, logout } = useAuth();
 *
 *   if (!isAuthenticated) {
 *     return <LoginForm onSubmit={login} />;
 *   }
 *
 *   return <div>Welcome, {user.firstName}!</div>;
 * }
 * ```
 */
export function useAuth(): AuthContextValue {
  const context = useContext(AuthContext);

  if (!context) {
    throw new Error(
      'useAuth must be used within an AuthProvider. ' +
        'Ensure your component is wrapped in <AuthProvider>.'
    );
  }

  return context;
}
